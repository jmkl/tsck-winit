use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
static PARENT: OnceLock<PathBuf> = OnceLock::new();

pub struct Dir;

impl Dir {
    #[inline]
    pub fn store_parent() -> Result<&'static Path> {
        let result = PARENT
            .get_or_init(|| {
                eprintln!("INIT DOTFILES PARENT DIR");
                if let Some(base) = std::env::var("USERPROFILE")
                    .or_else(|_| std::env::var("HOME"))
                    .context("HOME or USERPROFILE environment variable not set")
                    .ok()
                {
                    let dir = Path::new(&base).join(".config");
                    std::fs::create_dir_all(&dir)
                        .context("Failed to create .config directory")
                        .ok();
                    return dir;
                }
                panic!("Nooo");
            })
            .as_path();
        Ok(result)
    }

    #[inline]
    pub fn store_parent_str() -> Result<&'static str> {
        Self::store_parent()?
            .to_str()
            .context("Config path contains invalid UTF-8")
    }

    pub fn store_path(app_name: &str) -> Result<PathBuf> {
        let parent = Self::store_parent()?;
        let dir = parent.join(app_name);
        if !dir.exists() {
            std::fs::create_dir_all(&dir)
                .with_context(|| format!("Failed to create config directory for '{}'", app_name))?;
        }

        Ok(dir)
    }

    #[inline]
    pub fn store_path_str(app_name: &str) -> Result<String> {
        let path = Self::store_path(app_name)?;
        path.to_str()
            .context("Config path contains invalid UTF-8")
            .map(str::to_string)
    }

    #[inline]
    pub fn store_file(app_name: &str, filename: &str) -> Result<PathBuf> {
        Ok(Self::store_path(app_name)?.join(filename))
    }
}

#[cfg(test)]
mod dir_examples {
    use super::*;

    #[test]
    fn example_basic_usage() -> Result<()> {
        // Most efficient: Get as &Path
        let parent: &Path = Dir::store_parent()?;
        println!("Config parent: {}", parent.display());

        // Zero-copy string (after first call)
        let parent_str: &str = Dir::store_parent_str()?;
        println!("Config parent str: {}", parent_str);

        // App-specific directory
        let app_path = Dir::store_path("myapp")?;
        println!("App config: {}", app_path.display());

        // Full file path
        let settings = Dir::store_file("myapp", "settings.json")?;
        println!("Settings file: {}", settings.display());

        Ok(())
    }

    #[test]
    fn example_efficient_patterns() -> Result<()> {
        // ✅ BEST: Use &Path directly (no allocations)
        let parent = Dir::store_parent()?;
        let file = parent.join("myapp").join("data.db");

        // ✅ GOOD: Get PathBuf once, use as &Path many times
        let app_path = Dir::store_path("myapp")?;
        let settings = app_path.join("settings.json");
        let cache = app_path.join("cache.dat");

        // ❌ AVOID: Converting to String unnecessarily
        // let _path_str = Dir::config_path_str("myapp")?;  // Only if you need String!

        Ok(())
    }

    #[test]
    fn example_with_lazy_static() {
        use std::sync::LazyLock;

        // Cache app path as static if used frequently
        static APP_PATH: LazyLock<PathBuf> =
            LazyLock::new(|| Dir::store_path("myapp").expect("Failed to get app path"));

        // Use as &Path anywhere
        let settings = APP_PATH.join("settings.json");
        let cache = APP_PATH.join("cache");

        println!("Settings: {}", settings.display());
    }

    #[test]
    fn example_error_handling() {
        match Dir::store_parent() {
            Ok(path) => println!("Config: {}", path.display()),
            Err(e) => eprintln!("Error: {:#}", e),
        }
    }
}
