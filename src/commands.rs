use std::fs::{self, File};

use chrono::Local;

use crate::config::{Config, MySQLConfig};
use crate::error::{Error, Result};
use crate::options::{CreateOption, InitOption};
use crate::repository::Repository;

pub fn exec_init(opt: InitOption) -> Result<()> {
    let directory = opt.repository;

    match directory.is_dir() {
        true => return Err(Error::PathAlreadyExist(directory)),
        false => fs::create_dir_all(&directory)?,
    }
    Repository::create(directory.join("manage.db"));

    let mysql =
        MySQLConfig::new(opt.mysql_host, opt.mysql_port, opt.mysql_username, opt.mysql_password);
    Config::new(directory, mysql).save()?;

    Ok(())
}

pub fn exec_create(opt: CreateOption) -> Result<()> {
    let CreateOption { message } = opt;

    let config = Config::load_or_default();

    let now = Local::now();
    let filename =
        format!("{}{}.sql", now.format("%Y%m%d%H%M%S"), message.to_lowercase().replace(" ", "_"));
    let path = config.repository.join(filename);
    File::create(path)?;

    Ok(())
}
