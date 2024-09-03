use serde::{Deserialize, Serialize};
use core::str;
use std::fs;

use crate::{utils, utils::path_exists};

pub const CONFIG_FILE_PATH: &str = "~/.config/fssort.toml";

pub type FilePath = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    basedir: FilePath,
    folderized_filetypes: Vec<String>,
    ignored_filetypes: Vec<String>,
    excluded_filetypes: Vec<String>,
    journaled: Option<bool>,
    journal_location: Option<String>,
    backup: Option<bool>,
    backup_location: Option<String>,
}

impl ConfigFile {
    pub fn get_basedir(&self) -> FilePath {
        self.basedir.clone()
    }

    pub fn get_folderized_filetypes(&self) -> Vec<String> {
        self.folderized_filetypes.clone()
    }

    pub fn get_ignored_filetypes(&self) -> Vec<String> {
        self.ignored_filetypes.clone()
    }

    pub fn get_excluded_filetypes(&self) -> Vec<String> {
        self.excluded_filetypes.clone()
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
