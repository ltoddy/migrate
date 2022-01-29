use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    Init(InitArgs),
    Create(CreateArgs),
    Exec(ExecArgs),
}

#[derive(Debug, Parser)]
pub struct InitArgs {
    #[clap(default_value = "migrations")]
    pub repository: PathBuf,

    #[clap(long = "mysql-host", default_value = "127.0.0.1")]
    pub mysql_host: String,
    #[clap(long = "mysql-port", default_value = "3306")]
    pub mysql_port: u16,
    #[clap(long = "mysql-username", default_value = "root")]
    pub mysql_username: String,
    #[clap(long = "mysql-password", default_value = "")]
    pub mysql_password: String,
    #[clap(long = "mysql-db", default_value = "")]
    pub mysql_db: String,
}

#[derive(Debug, Parser)]
pub struct CreateArgs {
    #[clap(short = 'm', long = "message", default_value = "")]
    pub message: String,
}

#[derive(Debug, Parser)]
pub struct ExecArgs {
    #[clap(short = 'f', long = "force")]
    pub force: bool,
    #[clap(short = 'i', long = "ignore")]
    pub ignore: bool,
}
