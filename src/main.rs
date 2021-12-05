use log::LevelFilter::Trace;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use structopt::StructOpt;

use crate::error::Result;
use crate::options::Options;

pub mod commands;
pub mod config;
pub mod error;
pub mod options;
pub mod repository;

pub fn main() {
    TermLogger::init(Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    if let Err(e) = run() {
        log::error!("{}", e);
    }
}

pub fn run() -> Result<()> {
    let opts: Options = Options::from_args();
    match opts {
        Options::Init(opt) => commands::init(opt)?,
        Options::Create(opt) => commands::create(opt)?,
        Options::Exec(opt) => commands::exec(opt)?,
    }
    Ok(())
}
