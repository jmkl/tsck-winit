#![allow(unused)]
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use ts_rs::TS;

use crate::{event::TS_PATH, store::database::Model};

pub struct TextureScanner;

impl TextureScanner {
    pub fn scan(root_dir: &Path) -> Result<Vec<Texture>> {
        // let root = root_dir.canonicalize()?; // resolve once
        let root = root_dir.to_path_buf();
        let mut entries = Vec::with_capacity(1024); // heuristic
        Self::walk(&root, &root, &mut entries)?;
        Ok(entries)
    }

    fn walk(root: &Path, dir: &Path, out: &mut Vec<Texture>) -> Result<()> {
        if dir.file_name().is_some_and(|n| n == ".thumbnail") {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let ft = entry.file_type()?;
            let path = entry.path();

            if ft.is_dir() {
                Self::walk(root, &path, out)?;
            } else if ft.is_file() {
                let filename = entry.file_name();
                let folder = dir
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let thumbnail = Self::thumbnail_name(&filename.to_string_lossy().into_owned());
                out.push(Texture {
                    id: None,
                    filename: Some(filename.to_string_lossy().into_owned()),
                    abs_path: Some(path.to_string_lossy().into_owned()),
                    category: Some(folder),
                    thumbnail: Some(thumbnail),
                    favorite: Some(0),
                });
            }
        }
        Ok(())
    }

    #[inline]
    pub fn thumbnail_name(filename: &str) -> String {
        match Path::new(filename).extension().and_then(|e| e.to_str()) {
            Some(ext) if ext.eq_ignore_ascii_case("eps") => {
                let mut p = PathBuf::from(filename);
                p.set_extension("jpg");
                p.to_string_lossy().into_owned()
            }
            _ => filename.to_owned(),
        }
    }

    #[inline]
    pub fn thumbnail_path(root_dir: &Path, filename: &str) -> String {
        let name = Self::thumbnail_name(filename);
        root_dir
            .join(".thumbnail")
            .join(name)
            .to_string_lossy()
            .into_owned()
    }
}
#[derive(Serialize, Debug, Deserialize, TS)]
#[ts(export,export_to=TS_PATH)]
pub struct Texture {
    #[ts(type = "number")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub abs_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub favorite: Option<i32>,
}
impl Model for Texture {
    fn table_name() -> &'static str {
        "textures"
    }

    fn create_table_sql() -> &'static str {
        "CREATE TABLE IF NOT EXISTS textures (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        category TEXT,
        filename TEXT,
        abs_path TEXT,
        thumbnail TEXT,
        favorite INTEGER
    )"
    }

    fn from_row(row: &rusqlite::Row) -> Result<Self> {
        Ok(Texture {
            id: row.get("id").ok(),
            category: row.get("category").ok(),
            filename: row.get("filename").ok(),
            abs_path: row.get("abs_path").ok(),
            thumbnail: row.get("thumbnail").ok(),
            favorite: row.get("favorite").ok(),
        })
    }
}

#[cfg(test)]
mod texture_test {
    use crate::{
        store::{
            database::DbStore,
            textures::{Texture, TextureScanner},
        },
        transaction,
    };
    use anyhow;
    #[test]
    fn walk_dir() -> anyhow::Result<()> {
        let result = TextureScanner::scan(std::path::Path::new(
            r"I:\_GOOGLE DRIVE\GOOGLE DRIVE RK\THUMBNAIL\_ROOT\texture",
        ))?;
        println!("{:?}", result.get(0));
        Ok(())
    }
    #[test]
    fn create_table() -> anyhow::Result<()> {
        let texture_path =
            std::path::Path::new(r"I:\_GOOGLE DRIVE\GOOGLE DRIVE RK\THUMBNAIL\_ROOT\texture");
        let mut db = DbStore::new()?;
        db.write(|db| -> anyhow::Result<()> {
            db.create_table::<Texture>()?;
            match TextureScanner::scan(&texture_path) {
                Ok(entries) => {
                    if let Err(err) = transaction!(db, "textures", &entries) {
                        println!("{err}");
                    }
                }
                Err(_) => {}
            }
            Ok(())
        })
    }
}
