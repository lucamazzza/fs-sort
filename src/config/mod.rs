use serde::{Deserialize, Serialize};
use core::str;
use std::fs;

use crate::{utils, utils::path_exists};

pub const CONFIG_FILE_PATH: &str = "~/.config/fssort.toml";

pub type FilePath = String;

pub trait Config {
    fn get_basedir(&self) -> FilePath;
    fn get_file_types(&self) -> FileTypes;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    basedir: FilePath,
    file_types: FileTypes,
    security: Security,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileTypes {
    folderized: Vec<String>,
    ignored: Vec<String>,
    excluded: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Security {
    journaled: Option<bool>,
    journal_location: Option<String>,
    backup: Option<bool>,
    backup_location: Option<String>,
}

impl Config for ConfigFile {
    fn get_basedir(&self) -> FilePath {
        self.basedir.clone()
    }
    fn get_file_types(&self) -> FileTypes {
        self.file_types.clone()
    }
}

fn ser(config: ConfigFile) -> String {
    toml::to_string(&config).unwrap()
}

fn des(config: String) -> ConfigFile {
    toml::from_str(config.as_str().as_ref()).unwrap()
}
fn read_config_file(path: &mut FilePath) -> Result<String, &'static str> {
    if !path_exists(path) {
        return Err("File not found");
    }
    let file_content = fs::read_to_string(path).expect("Unable to read configuration file");
    Ok(file_content)
}

pub fn parse_toml_file(path: &mut FilePath) -> Result<ConfigFile, &'static str> {
    let file_content = read_config_file(path).unwrap();
    if file_content.is_empty() {
        return Err("Config file is empty");
    }
    let config: ConfigFile = des(file_content);
    Ok(config)
}

pub fn write_config_file(config: ConfigFile) -> Result<(), &'static str> {
    let config_file_path: &mut FilePath = &mut (CONFIG_FILE_PATH.to_string() as FilePath);
    Ok(utils::write_file(config_file_path, ser(config))?) 
}
