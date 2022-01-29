use clap::Parser;
use log::LevelFilter::Trace;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::cli::{Cli, SubCommands};
use crate::error::Result;

pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod repository;

pub fn main() {
    TermLogger::init(Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    if let Err(e) = run() {
        log::error!("{}", e);
    }
}

pub fn run() -> Result<()> {
    let cli: Cli = Cli::parse();

    match cli.subcommand {
        SubCommands::Init(args) => commands::init(args)?,
        SubCommands::Create(args) => commands::create(args)?,
        SubCommands::Exec(args) => commands::exec(args)?,
    }
    Ok(())
}
