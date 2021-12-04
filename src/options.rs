use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Options {
    #[structopt(name = "init", help = "create an empty repository at the specified path")]
    Init(InitOption),
    #[structopt(name = "create")]
    Create(CreateOption),
    #[structopt(name = "exec")]
    Exec(ExecOption),
}

#[derive(StructOpt, Debug)]
pub struct InitOption {
    #[structopt(default_value = "migrations")]
    pub repository: PathBuf,

    #[structopt(long = "mysql-host", default_value = "127.0.0.1")]
    pub mysql_host: String,
    #[structopt(long = "mysql-port", default_value = "3306")]
    pub mysql_port: u16,
    #[structopt(long = "mysql-username", default_value = "root")]
    pub mysql_username: String,
    #[structopt(long = "mysql-password", default_value = "")]
    pub mysql_password: String,
    #[structopt(long = "mysql-db", default_value = "")]
    pub mysql_db: String,
}

#[derive(StructOpt, Debug)]
pub struct CreateOption {
    #[structopt(short = "m", long = "message", default_value = "")]
    pub message: String,
}

#[derive(StructOpt, Debug)]
pub struct ExecOption {
    #[structopt(short = "f", long = "force")]
    pub force: bool,
    #[structopt(short = "i", long = "ignore")]
    pub ignore: bool,
}
