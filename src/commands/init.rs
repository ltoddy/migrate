use std::fs;

use configit::Storage;

use crate::cli::InitArgs;
use crate::config::{Config, MySQLConfig};
use crate::repository::Repository;
use crate::Result;

pub fn init(args: InitArgs) -> Result<()> {
    let directory = args.repository;
    if !directory.exists() {
        fs::create_dir_all(&directory)?;
        log::info!("create {}", directory.display());
    }

    Repository::initialize(directory.join("manage.db"));

    let mysql = MySQLConfig::new(
        args.mysql_host,
        args.mysql_port,
        args.mysql_username,
        args.mysql_password,
        args.mysql_db,
    );
    Config::new(directory, mysql).store(Config::FILENAME)?;

    Ok(())
}
