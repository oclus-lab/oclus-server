use log::LevelFilter;
use serde::Deserialize;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub log_level: LevelFilter,
    pub bind_addr: String,
    pub db_url: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("failed to open config file: {0}")]
    File(#[from] io::Error),

    #[error("invalid config file: {0}")]
    Serde(#[from] serde_yaml::Error),
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
        let file = File::open(path)?;

        let config: Config = serde_yaml::from_reader(file)?;
        Ok(config)
    }
}
