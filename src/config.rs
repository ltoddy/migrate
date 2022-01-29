use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
    pub const FILENAME: &'static str = "migrate.toml";

    pub fn new(repository: PathBuf, mysql: MySQLConfig) -> Self {
        Config { repository, mysql }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { repository: PathBuf::from("migrations"), mysql: MySQLConfig::default() }
    }
}
