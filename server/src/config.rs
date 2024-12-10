use anyhow::anyhow;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize, Clone, Debug)]
pub struct OclusConfig {
    pub bind_addr: String,
}

impl OclusConfig {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let file = File::open(&path).map_err(|err| {
            anyhow!(
                "Failed to open config file at {} - {}",
                path.as_ref().display(),
                err
            )
        })?;

        let config = serde_yaml::from_reader(file)
            .map_err(|err| anyhow!("Failed to parse yaml configuration - {}", err))?;

        Ok(config)
    }
}
