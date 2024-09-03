use std::{path::Path, process::exit};

use config::{ConfigFile, FilePath, CONFIG_FILE_PATH};
use notify::{Config, RecommendedWatcher, RecursiveMode, Result, Watcher, Event};
use futures::{channel::mpsc::{channel, Receiver}, SinkExt, StreamExt};
use utils::path_exists;

mod algo;
mod config;
mod filevalidator;
mod utils;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;
    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => println!("changed: {:?}", event),
            Err(e) => println!("error: {:?}", e),
        }
    }
    Ok(())
}

fn main() {
    let config_file_path: &mut FilePath = &mut ("/Users/lucamazza/.config/fssort.toml".to_string() as FilePath);
    if !path_exists(config_file_path) {
        println!("Config file not found at {:?}", CONFIG_FILE_PATH);
        exit(1)
    }
    //let config: ConfigFile = config::parse_toml_file(config_file_path).unwrap();
    futures::executor::block_on(async {
        if let Err(e) = async_watch("/Users/lucamazza").await {
            println!("error: {:?}", e)
        }
    });
}
