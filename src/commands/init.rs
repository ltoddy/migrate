use std::fs;

use crate::config::{Config, MySQLConfig};
use crate::error::Result;
use crate::options::InitOption;
use crate::repository::Repository;

pub fn init(opt: InitOption) -> Result<()> {
    let directory = opt.repository;
    if !directory.exists() {
        fs::create_dir_all(&directory)?;
        log::info!("create {}", directory.display());
    }

    Repository::initialize(directory.join("manage.db"));

    let mysql = MySQLConfig::new(
        opt.mysql_host,
        opt.mysql_port,
        opt.mysql_username,
        opt.mysql_password,
        opt.mysql_db,
    );
    Config::new(directory, mysql).save()?;

    Ok(())
}
