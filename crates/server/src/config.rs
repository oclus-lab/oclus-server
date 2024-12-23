use std::fs::File;
use std::path::Path;
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub log_level: String,
    pub otp_secret: String,
    pub token_secret: String,
    pub database_url: String,
}

impl Config {
    pub fn from_yaml(path: impl AsRef<Path>) -> Result<Config, anyhow::Error> {
        let file = File::open(path).context("failed to open config file")?;
        let config = serde_yaml::from_reader(file).context("failed to parse config file")?;
        Ok(config)
    }
}