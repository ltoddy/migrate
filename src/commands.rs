use std::fs;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::options::{CreateOption, InitOption};

pub fn exec_init(opt: InitOption) -> Result<()> {
    let directory = opt.repository;

    match directory.is_dir() {
        true => return Err(Error::PathAlreadyExist(directory)),
        false => fs::create_dir_all(&directory)?,
    }

    let config = Config::new(directory);
    config.save()?;

    Ok(())
}

pub fn exec_create(_opt: CreateOption) -> Result<()> {
    Ok(())
}
