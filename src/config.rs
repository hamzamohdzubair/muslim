use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub prayers: Prayers,
}

#[derive(Debug, Deserialize)]
pub struct Prayers {
    pub fajr: u16,    // HHMM format: 0530 = 5:30 AM
    pub dhuhr: u16,   // 1245 = 12:45 PM
    pub asr: u16,     // 1615 = 4:15 PM
    pub maghrib: u16, // 1830 = 6:30 PM
    pub isha: u16,    // 2000 = 8:00 PM
}

const DEFAULT_CONFIG: &str = r#"[prayers]
fajr = 530
dhuhr = 1245
asr = 1615
maghrib = 1830
isha = 2000
"#;

/// Get the config directory path (~/.config/muslim)
pub fn get_config_dir() -> Result<PathBuf> {
    ProjectDirs::from("", "", "muslim")
        .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
        .context("Failed to determine config directory")
}

/// Get the main config file path (~/.config/muslim/main.toml)
pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("main.toml"))
}

/// Load config from file
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    let contents = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

    let config: Config = toml::from_str(&contents)
        .context("Failed to parse config file")?;

    Ok(config)
}

/// Create default config file with dummy times
pub fn create_default_config() -> Result<()> {
    let config_dir = get_config_dir()?;
    let config_path = config_dir.join("main.toml");

    // Create config directory if it doesn't exist
    fs::create_dir_all(&config_dir)
        .with_context(|| format!("Failed to create config directory: {}", config_dir.display()))?;

    // Write default config
    fs::write(&config_path, DEFAULT_CONFIG)
        .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

    println!("Created config file at: {}", config_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_default_config() {
        let config: Config = toml::from_str(DEFAULT_CONFIG).unwrap();
        assert_eq!(config.prayers.fajr, 530);
        assert_eq!(config.prayers.dhuhr, 1245);
        assert_eq!(config.prayers.asr, 1615);
        assert_eq!(config.prayers.maghrib, 1830);
        assert_eq!(config.prayers.isha, 2000);
    }

    #[test]
    fn test_leading_zero_times() {
        // Test that we handle times with hours < 10
        let config_str = r#"
[prayers]
fajr = 530
dhuhr = 1245
asr = 1615
maghrib = 1830
isha = 2000
"#;
        let config: Config = toml::from_str(config_str).unwrap();
        assert_eq!(config.prayers.fajr, 530);
    }
}
