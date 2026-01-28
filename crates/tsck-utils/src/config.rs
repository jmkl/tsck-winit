#![allow(unused)]
use crate::generate_func_enums;
use anyhow::Result;
pub use paste;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
    string::ParseError,
};

pub struct ConfigStore<T> {
    path: PathBuf,
    data: T,
}

impl<T> ConfigStore<T> {
    fn root_dir(app_name: &str) -> Result<PathBuf> {
        let base = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME"))?;

        let dir = Path::new(&base).join(".config").join(app_name);

        fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}

impl<T> ConfigStore<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Default,
{
    pub fn new(app_name: &str, config_name: Option<&str>) -> Result<Self> {
        let dir = Self::root_dir(app_name)?;
        let path = dir.join(config_name.unwrap_or("config.json"));

        let data = if path.exists() {
            let content = fs::read_to_string(&path)?;
            serde_json::from_str(&content)?
        } else {
            T::default()
        };

        Ok(Self { path, data })
    }
}

impl<T> ConfigStore<T>
where
    T: Serialize,
{
    fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.path, content)?;
        Ok(())
    }
}

impl<T> ConfigStore<T> {
    pub fn get<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&self.data)
    }
}

impl<T> ConfigStore<T>
where
    T: Serialize,
{
    pub fn set(&mut self, f: impl FnOnce(&mut T)) -> Result<()> {
        f(&mut self.data);
        self.save()
    }
}

// "app::CYCLE",
// "app::TSOCKEENUM",
// "app::TSOCKEESWAP",
// "app::PHOTOSHOP",
// "app::LAUNCHPLUGIN",
// "app::LAUNCHPLUGIN",
// "app::TSOCKEENIR"
// "workspace::TOGGLE",
// "workspace::SKIP",

struct FrmStrError;
enum Entry {
    App,
    Workspace,
}
impl FromStr for Entry {
    type Err = FrmStrError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "app" => Ok(Self::App),
            "workspace" => Ok(Self::Workspace),
            _ => Err(FrmStrError),
        }
    }
}

enum AppFunc {
    Cycle,
    Tsockee,
    Phothoshop,
    LaunchPlugin,
}
enum WorkspaceFunc {
    Toggle,
}

fn test() {}

// enum FuncEntry {
//     App,
//     Workspace,
// }
// enum FuncFunction {
//     Tsockee,
//     LaunchPlugin,
//     Photoshop,
//     CycleApps,
//     Toggle,
// }

#[cfg(test)]
mod test_config {
    generate_func_enums!(
        KeeEntry => (
            App => (
                Tsockee,
                LaunchPlugin,
                Photoshop,
                CycleApps,
            )
            Workspace => (
                Toggle,
            )
        )
    );
    use std::{collections::HashMap, str::FromStr};

    use crate::{Func, config::ConfigStore, generate_func_enums, parse_func};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Default)]
    #[serde(default)]
    struct Keys {
        key: String,
        func: String,
    }
    #[derive(Serialize, Deserialize, Default)]
    #[serde(default)]
    pub struct TestConfig {
        monitors: Vec<(i32, i32)>,
        apps: Vec<String>,
        kees: HashMap<String, String>,
        version: String,
    }
    #[test]
    fn test_config() -> anyhow::Result<()> {
        let mut config = ConfigStore::<TestConfig>::new("test_app", Some("test_config.json"))?;
        config.set(|c| {
            c.monitors = vec![(1200, 600), (800, 480)];
            c.apps = vec!["app1".to_string(), "app2".to_string()];
            c.version = "0.0.1".to_string();
        })?;
        Ok(())
    }
    #[test]
    fn get_config() -> anyhow::Result<()> {
        let config = ConfigStore::<TestConfig>::new("test_app", Some("test_config.json"))?;
        let c: Vec<String> = config.get(|c| c.kees.iter().map(|(k, v)| v.clone()).collect());
        c.iter().for_each(|c| {
            if let Some(cmd) = parse_func(c) {
                let entry = cmd.entry;
                let func = cmd.func;
                let args = cmd.args;
                if let Ok(entry) = KeeEntry::from_str(cmd.entry) {
                    match entry {
                        KeeEntry::App => {
                            if let Ok(func) = AppFunc::from_str(cmd.func) {
                                match func {
                                    AppFunc::Tsockee => println!("::Tsockee"),
                                    AppFunc::LaunchPlugin => println!("::LaunchPlugin"),
                                    AppFunc::Photoshop => println!("::Photoshop"),
                                    AppFunc::CycleApps => println!("::CycleApps"),
                                }
                            }
                        }
                        KeeEntry::Workspace => {
                            if let Ok(func) = WorkspaceFunc::from_str(cmd.func) {
                                match func {
                                    WorkspaceFunc::Toggle => println!("::Toggle"),
                                }
                            }
                        }
                    }
                }
                // if let Ok(cmd) = Command::from_str(namespace) {
                //     match cmd {
                //         Command::WorkSpace => todo!(),
                //         Command::Action => todo!(),
                //         Command::Args => todo!(),
                //     }
                // }
            }
        });
        Ok(())
    }
}
