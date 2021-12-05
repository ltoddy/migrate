use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct MySQLConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    db: String,
}

impl MySQLConfig {
    pub fn new(host: String, port: u16, username: String, password: String, db: String) -> Self {
        MySQLConfig { host, port, username, password, db }
    }

    pub fn get_url(&self) -> String {
        format!(
            "mysql://{username}:{password}@{host}:{port}/{db}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
            db = self.db,
        )
    }
}

impl Default for MySQLConfig {
    fn default() -> Self {
        MySQLConfig {
            host: "127.0.0.1".to_string(),
            port: 3306,
            username: "root".to_string(),
            password: "".to_string(),
            db: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub repository: PathBuf,
    pub mysql: MySQLConfig,
}

impl Config {
    pub fn new(repository: PathBuf, mysql: MySQLConfig) -> Self {
        Config { repository, mysql }
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
        log::info!("saving migrate config");
        std::fs::write(filename, content)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { repository: PathBuf::from("migrations"), mysql: MySQLConfig::default() }
    }
}
