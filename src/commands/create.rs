use std::fs;

use configit::Loader;

use crate::cli::CreateArgs;
use crate::config::Config;
use crate::Result;

pub fn create(args: CreateArgs) -> Result<()> {
    let CreateArgs { message } = args;

    let config = Config::load(Config::FILENAME).unwrap_or_default();
    let now = now();
    let message = message.replace(" ", "_");
    let filename = vec![now, message].join("_");
    let mut path = config.repository.join(filename);
    path.set_extension("sql");
    fs::File::create(&path)?;
    log::info!("create file: {}", path.display());

    Ok(())
}

fn now() -> String {
    const LAYOUT: &str = "%Y%m%d%H%M%S";
    let now = chrono::Local::now();
    now.format(LAYOUT).to_string()
}
