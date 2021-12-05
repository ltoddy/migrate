use std::fs::{self};

use mysql::prelude::*;
use mysql::{Opts, Pool};

use crate::config::Config;
use crate::error::Result;
use crate::options::ExecOption;

pub fn exec(opt: ExecOption) -> Result<()> {
    let config = Config::load_or_default();

    let url = config.mysql.get_url();
    let opts = Opts::from_url(&url).expect("wrong mysql config");
    let pool = Pool::new(opts).expect("can't connect mysql");
    let mut conn = pool.get_conn().expect("connect mysql timeout");

    for entry in fs::read_dir(config.repository)? {
        let entry = entry?;
        let path = entry.path();

        match path.extension() {
            Some(extension) if extension == "sql" => {
                let filename = format!("{}", path.display());
                log::info!("find sql file: {}", filename);

                let content = fs::read_to_string(path)?;
                let sqls = content.split(';').collect::<Vec<_>>();
                for sql in sqls {
                    let sql = sql.trim();
                    if sql.is_empty() {
                        continue;
                    }
                    if let Err(e) = conn.query_drop(sql) {
                        log::error!("{}", e);
                        log::error!("\n{}", sql);
                        if !opt.ignore {
                            break;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    Ok(())
}
