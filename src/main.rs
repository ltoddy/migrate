use structopt::StructOpt;

use crate::error::Result;
use crate::options::Options;

pub mod commands;
pub mod config;
pub mod error;
pub mod options;
pub mod repository;

pub fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

pub fn run() -> Result<()> {
    let opts: Options = Options::from_args();
    match opts {
        Options::Init(opt) => commands::exec_init(opt)?,
        Options::Create(opt) => commands::exec_create(opt)?,
        Options::Exec(opt) => commands::exec_exec(opt)?,
    }
    Ok(())
}
