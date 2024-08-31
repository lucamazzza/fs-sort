use std::io;
use std::fs;

use crate::config::{Config, ConfigFile, FilePath};

pub fn get_basedir(conf: ConfigFile) -> Result<String, String> {
    let basedir = conf.get_basedir();
    if basedir.is_empty() {
        return Err("Base Directory not defined".to_string())
    }
    Ok(basedir)
}

pub fn mkdir_if_inexistent(dirpath: FilePath) -> Result<(), io::Error> {
    fs::create_dir_all(dirpath)?;
    Ok(())
}
