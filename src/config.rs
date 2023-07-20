use anyhow::{Context as _, Result};
use serde::Deserialize;
use std::{fs, path::PathBuf};

pub type Name = String;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub default: Name,
    pub presets: Vec<Kaomoji>,
}

#[derive(Clone, Deserialize)]
pub struct Kaomoji {
    pub name: Name,
    pub kaomoji: String,
    pub speech_bubble_left: Option<String>,
    pub speech_bubble_right: Option<String>,
}

const DEFAULT_CONFIG: &str = include_str!("../config/kaomoji-echo.toml");

pub fn config_path() -> Result<PathBuf> {
    let config_path = dirs::config_dir()
        .with_context(|| "Failed to get config directory.")?
        .join("kaomoji-config.toml");
    if !config_path.exists() {
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::write(&config_path, DEFAULT_CONFIG)?;
    }
    Ok(config_path)
}

pub fn read_config() -> Result<Config> {
    let config_path = config_path()?;
    let s = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read: `{}`.", config_path.display()))?;
    let config = toml::from_str(&s).with_context(|| {
        format!(
            "Failed to parse the toml file: `{}`.",
            config_path.display()
        )
    });
    config
}
