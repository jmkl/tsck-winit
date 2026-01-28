use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Cmd {
    pub exe: String,
    pub args: Vec<String>,
}
#[derive(Deserialize)]
pub struct Config {
    pub commands: Vec<Cmd>,
}

pub fn read_config() -> anyhow::Result<Config> {
    let root = std::env::current_dir()?;
    let file = Path::new(&root).join("tsck.json");
    if file.exists() {
        let result = std::fs::read_to_string(file)?;
        let config = serde_json::from_str::<Config>(&result)?;
        return Ok(config);
    } else {
        std::fs::write(file, "{}")?;
    }
    anyhow::bail!("Config Not Found")
}
