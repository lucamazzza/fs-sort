use std::{path::Path, process::exit};

use config::{ConfigFile, FilePath, CONFIG_FILE_PATH};
use notify::{RecursiveMode, Result, Watcher};
use utils::path_exists;

mod algo;
mod config;
mod filevalidator;
mod utils;

fn main() -> Result<()> {
    let config_file_path: &mut FilePath = &mut (CONFIG_FILE_PATH.to_string() as FilePath);
    if !path_exists(config_file_path) {
        println!("Config file not found at {:?}", CONFIG_FILE_PATH);
        exit(1)
    }
    let config: ConfigFile = config::parse_toml_file(config_file_path).unwrap();
    let mut watcher = notify::recommended_watcher(|res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })?;
    loop {
        watcher.watch(Path::new("/Users/lucamazza"), RecursiveMode::Recursive)?;
    }
}
