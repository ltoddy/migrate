use std::fs::{self, File};

use chrono::Local;
use mysql::prelude::*;
use mysql::{Opts, Pool};

use crate::config::{Config, MySQLConfig};
use crate::error::{Error, Result};
use crate::options::{CreateOption, ExecOption, InitOption};
use crate::repository::Repository;

pub fn exec_init(opt: InitOption) -> Result<()> {
    let directory = opt.repository;

    match directory.is_dir() {
        true => return Err(Error::PathAlreadyExist(directory)),
        false => fs::create_dir_all(&directory)?,
    }
    Repository::create(directory.join("manage.db"));

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

pub fn exec_exec(opt: ExecOption) -> Result<()> {
    let config = Config::load_or_default();

    let opts = Opts::from_url(&config.mysql.get_url()).expect("wrong mysql config");
    let pool = Pool::new(opts).expect("can't connect mysql");
    let mut conn = pool.get_conn().expect("connect mysql timeout");

    for entry in fs::read_dir(config.repository)? {
        let entry = entry?;
        let path = entry.path();

        match path.extension() {
            Some(extension) if extension == "sql" => {
                let content = fs::read_to_string(path)?;
                let sqls = content.split(';').collect::<Vec<_>>();
                for sql in sqls {
                    let sql = sql.trim();
                    match opt.ignore {
                        true => {
                            let _ = conn.query_drop(sql);
                        }
                        false => conn.query_drop(sql).expect("exec sql failed"),
                    }
                }
            }
            _ => (),
        }
    }

    Ok(())
}
