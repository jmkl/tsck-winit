#![allow(unused)]
use crate::{AppConfigHandler, event::TS_PATH, log_info, ts_struct};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
};
use ts_rs::TS;

ts_struct! {
    path = TS_PATH,
    pub struct SmartObjectItem {
            pub id:i32,
        pub name: String,
        pub thumb: String,

    }
}
ts_struct! {
    path = TS_PATH,
    pub struct SmartObjects {
        smartobject_dir:PathBuf,
        pub files: Vec<SmartObjectItem>,
        root_path:String,
        http_server_port:u16
    }
}
ts_struct! {
    path = TS_PATH,
    pub struct PaginationItems {
        pub page: usize,
        pub total_page: usize,
        pub current_items: Vec<SmartObjectItem>,
    }
}

impl SmartObjects {
    pub fn new() -> Self {
        let (root_folder, http_server_port) = {
            let config = AppConfigHandler::get();
            let root_folder = config.get(|c| c.store_root.clone());
            let http_server_port = config.get(|c| c.http_server_port);
            (root_folder, http_server_port)
        };
        let smartobject_dir = PathBuf::from(&root_folder).join("smartobject");
        let files = list_dir(&smartobject_dir).unwrap();

        Self {
            smartobject_dir,
            root_path: root_folder,
            http_server_port: http_server_port,
            files: files
                .iter()
                .map(|a| SmartObjectItem {
                    id: 0,
                    name: a.clone(),
                    thumb: format!("{}.png", a.to_string().trim_end_matches(".psb")),
                })
                .collect(),
        }
    }
    pub fn add_file(&mut self, new_item: &SmartObjectItem) {
        self.files.push(new_item.clone());
    }
    pub fn remove_file(&mut self, file_name: &str) {
        if let Some(pos) = self.files.iter().position(|item| item.name == file_name) {
            log_info!("remove_file at:", &pos.to_string());
            self.files.remove(pos);
        }
    }
    pub fn get_all_files(&self) -> &[SmartObjectItem] {
        &self.files
    }

    pub fn update_files(&mut self) -> Vec<SmartObjectItem> {
        self.files = list_dir(&self.smartobject_dir)
            .unwrap()
            .iter()
            .map(|a| SmartObjectItem {
                id: 0,
                name: a.clone(),
                thumb: format!("{}.png", a.to_string().trim_end_matches(".psb")),
            })
            .collect();
        self.files.clone()
    }
    pub fn filter(&self, filter: &str) -> Vec<SmartObjectItem> {
        self.files
            .iter()
            .filter(|a| a.name.contains(filter))
            .cloned()
            .collect()
    }
    pub fn filter_chunk(&self, filter: &str, page: usize, per_page: usize) -> PaginationItems {
        let result: Vec<SmartObjectItem> = self
            .files
            .iter()
            .filter(|a| a.name.contains(filter))
            .cloned()
            .collect();
        let total_page = result.len().div_ceil(per_page);
        let start = page * per_page;
        let end = (start + per_page).min(result.len());
        let current_items = result[start..end].to_vec();
        PaginationItems {
            page,
            total_page,
            current_items,
        }
    }

    pub fn to_psb(&self, filename: &str) -> Option<String> {
        let path = Path::new(filename);
        let stem = path.file_stem()?.to_string_lossy().into_owned();
        Some(format!("{}.psb", stem))
    }

    pub fn delete_psb(&mut self, file_name: &str) -> anyhow::Result<()> {
        let file_name = Path::new(file_name)
            .file_stem()
            .expect("Not found")
            .to_string_lossy()
            .into_owned();
        let psd = Path::new(&self.smartobject_dir).join(format!("{file_name}.psb"));
        let thumb = Path::new(&self.smartobject_dir)
            .join("thumbs")
            .join(format!("{file_name}.png"));
        println!(
            "{:?}=>{}\n{:?}=>{}",
            psd,
            psd.exists(),
            thumb,
            thumb.exists()
        );
        fs::remove_file(psd)?;
        fs::remove_file(thumb)?;
        Ok(())
    }

    pub fn convert_psd_to_png(&self, psd_file: &str) -> Option<String> {
        let parent_dir = Path::new(psd_file).parent().unwrap().to_path_buf();
        let file = Path::new(psd_file).file_stem().unwrap().to_str().unwrap();
        let png_path = normalize_path(
            Path::join(&parent_dir, "thumbs")
                .join(format!("{}.png", file))
                .as_path(),
        );
        let psd_str = format!("\"{psd_file}\"");
        let png_str = format!("\"{}\"", png_path.to_str().unwrap());

        let output = Command::new("pwsh")
            .args([
                "-NoProfile",
                "-C",
                "magick",
                &psd_str,
                "-scale",
                "120",
                "-delete",
                "1--1",
                &png_str,
            ])
            .creation_flags(0x08000000)
            .output()
            .expect("Failed");
        if output.status.success() {
            Some(format!("{}.png", file))
        } else {
            None
        }
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            comp => components.push(comp),
        }
    }

    components.iter().collect()
}
fn list_dir(path: &PathBuf) -> std::io::Result<Vec<String>> {
    let mut items = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            items.push(name.to_string());
        }
    }

    Ok(items)
}

#[cfg(test)]
mod test_repo {
    use crate::photoshop::repo_smartobject::SmartObjects;

    #[test]
    fn test_repo_smartobject() {
        let so = SmartObjects::new();
        // println!("{:#?}", so.files);
    }
}
