use std::fs;

use crate::config::Config;
use crate::error::Result;
use crate::options::CreateOption;

pub fn create(opt: CreateOption) -> Result<()> {
    let CreateOption { message } = opt;

    let config = Config::load_or_default();
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
