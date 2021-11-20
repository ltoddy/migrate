use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    repository: PathBuf,
}

impl Config {
    pub fn new(repository: PathBuf) -> Self {
        Config { repository }
    }

    // pub fn load() -> Result<Self> {
    //     let filename = PathBuf::from("migrate.toml");
    //     let mut file = File::open(filename)?;
    //
    //     let mut content = Vec::with_capacity(128);
    //     file.read_to_end(&mut content)?;
    //
    //     let config = toml::from_slice::<Config>(&content)?;
    //     Ok(config)
    // }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(self).unwrap();
        let filename = PathBuf::from("migrate.toml");
        if filename.is_file() {
            return Err(Error::PathAlreadyExist(filename));
        }
        std::fs::write(filename, content)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { repository: PathBuf::from("migrations") }
    }
}
