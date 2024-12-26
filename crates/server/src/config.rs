use anyhow::Context;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub log_level: LevelFilter,
    pub otp_secret: String,
    pub token_secret: String,
    pub db_url: Url,
}

impl Config {
    pub fn from_yaml(path: impl AsRef<Path>) -> Result<Config, anyhow::Error> {
        let file = File::open(path).context("failed to open config file")?;
        let config = serde_yaml::from_reader(file).context("failed to parse config file")?;
        Ok(config)
    }
}
