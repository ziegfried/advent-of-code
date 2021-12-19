use anyhow::{Context, Result};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub session_key: String,
}

pub fn config_path() -> PathBuf {
    #[allow(deprecated)]
    env::home_dir().unwrap().join(".config").join("aoc.toml")
}

pub fn load_config() -> Option<Config> {
    match fs::read_to_string(config_path()) {
        Ok(contents) => match toml::from_str::<Config>(contents.as_str()) {
            Ok(config) => Some(config),
            Err(e) => {
                eprintln!("Warn: Unable to parse config file: {:?}", e);
                None
            }
        },
        Err(e) => {
            eprintln!("Warn: Unable to read config file: {:?}", e);
            None
        }
    }
}

pub fn ensure_config_dirs() -> Result<()> {
    fs::create_dir_all(config_path().parent().unwrap())
        .context("Unable to create .config directory")?;
    Ok(())
}

pub fn write_config(config: &Config) -> Result<()> {
    ensure_config_dirs()?;
    let serialized = toml::to_string(&config).context("Failed to serialize config to TOML")?;
    std::fs::write(config_path(), serialized).context("Failed to write config file")?;
    Ok(())
}
