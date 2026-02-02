#![allow(unused)]
use crate::{event::TS_PATH, ts_struct};
use serde::{Deserialize, Serialize};
use std::path::Path;
use ts_rs::TS;

ts_struct! {path = TS_PATH,
pub struct CustomScripts{
    pub script_list:Vec<String>,

}
}
impl CustomScripts {
    pub fn new() -> Self {
        Self {
            script_list: Vec::new(),
        }
    }
    pub fn script_to_str(
        &self,
        script_root: &Path,
        script_name: &str,
    ) -> Result<String, std::io::Error> {
        // let helper = std::fs::read_to_string(Path::new(script_root).join("HELPER.js"))?;
        let str = std::fs::read_to_string(Path::new(script_root).join(script_name))?;
        Ok(format!("\n{}", str))
    }
    pub fn load_scripts(&mut self, path: &Path) -> Vec<String> {
        let parent = Path::new(path);
        if let Ok(files) = std::fs::read_dir(parent) {
            let scripts = files
                .filter_map(|e| e.ok())
                .map(|s| s.path())
                .filter(|p| p.extension().is_some_and(|e| e.to_str() == Some("js")))
                .filter_map(|p| {
                    p.file_name()
                        .and_then(|name| name.to_str().map(|s| s.to_string()))
                })
                .collect();
            self.script_list = scripts;
            self.script_list.clone()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod test_script {
    use super::*;

    #[test]
    fn test_script_load() {
        let root_dir = "I:/_GOOGLE DRIVE/GOOGLE DRIVE RK/THUMBNAIL/_ROOT/customscripts";
        let mut customscript = CustomScripts::new();
        customscript.load_scripts(Path::new(root_dir));
        customscript.script_to_str(Path::new(root_dir), "splittext.js");
    }
}
