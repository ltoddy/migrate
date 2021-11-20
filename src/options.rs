use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Options {
    #[structopt(name = "init", help = "create an empty repository at the specified path")]
    Init(InitOption),
    #[structopt(name = "create")]
    Create(CreateOption),
}

#[derive(StructOpt, Debug)]
pub struct InitOption {
    #[structopt(default_value = "migrations")]
    pub repository: PathBuf,
}

#[derive(StructOpt, Debug)]
pub struct CreateOption {
    #[structopt(short = "m", long = "message", default_value = "")]
    pub message: String,
}
