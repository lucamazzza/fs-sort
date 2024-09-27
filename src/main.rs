use std::{env, path::Path, process::exit};

use config::{ConfigFile, FilePath, CONFIG_FILE_PATH};
use notify::{Config, RecommendedWatcher, RecursiveMode, Result, Watcher, Event};
use futures::{channel::mpsc::{channel, Receiver}, SinkExt, StreamExt};
use utils::path_exists;

mod algo;
mod cli;
mod config;
mod filevalidator;
mod utils;
mod cnst;

fn help() {
    let help_string = r#"
    fs-sort version 0.0.1
    (C) 2024-2025 Luca Mazza. 
    Released under MIT License.

    Parameters:
     * init                  initializes the config file to the default one;
     * reindex               reindexes all files managed by fs-sort (reorders them);
     * ...
    "#;
    println!("{}", help_string);
}

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

fn one_arg(arg0: &str) {
    match arg0 {
        // "init" => config::init(),
        // "reindex" => algo::reindex(),
        "help" => help(),
        _ => {
            println!("Invalid parameter '{:?}'", arg0);
            help();
        },
    }
}

fn two_arg(arg0: &str, arg1: &str) {
    match (arg0, arg1) {
        // ("arg0", "arg1") => help(),
        _ => {
            println!("Invalid parameters '{:?} {:?}'", arg0, arg1);
            help();
        },
    }
}

fn execute_args(args: Vec<String>) {
    match args.len() {
        1 => one_arg(args.get(0).unwrap().as_str()),
        2 => two_arg(args.get(0).unwrap().as_str(), args.get(1).unwrap().as_str()),
        _ => {
            println!("Command fssort does not accept more than {:?} parameters", cnst::MAX_CLI_ARGS);
        },
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let config_file_path: &mut FilePath = &mut ("/Users/lucamazza/.config/fssort.toml".to_string() as FilePath);
    if !path_exists(config_file_path) {
        println!("Config file not found at {:?}", CONFIG_FILE_PATH);
        exit(1)
    }
    // let config: ConfigFile = config::parse_toml_file(config_file_path).unwrap();
    execute_args(args);
    futures::executor::block_on(async {
        if let Err(e) = async_watch("/Users/lucamazza").await {
            println!("error: {:?}", e)
        }
    });
}
