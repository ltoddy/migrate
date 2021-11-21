use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub repository: PathBuf,
}

impl Config {
    pub fn new(repository: PathBuf) -> Self {
        Config { repository }
    }

    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    fn load() -> Result<Self> {
        let filename = PathBuf::from("migrate.toml");
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        let mut content = String::with_capacity(128);
        reader.read_to_string(&mut content)?;

        let config = toml::from_str::<Config>(&content)?;
        Ok(config)
    }

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
