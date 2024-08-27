use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

type FilePath = String;

pub trait Config {
    fn ser(config: ConfigFile) -> String;
    fn des(config: String) -> ConfigFile;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile { // TODO: Define single structures for each section of the config
    basedir: FilePath,
    file_types: FileTypes,
    security: Security,
}

#[derive(Serialize, Deserialize, Debug)]
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
    fn ser(config: ConfigFile) -> String {
        toml::to_string(&config).unwrap()
    }
    fn des(config: String) -> ConfigFile {
        toml::from_str(config.as_str().as_ref()).unwrap()
    }
}

fn path_exists(path: &mut FilePath) -> bool {
    Path::new(path).exists()
}

pub fn read_config_file(path: &mut FilePath) -> Result<String, &'static str> {
    if !path_exists(path) { return Err("File not found") }
    let file_content = fs::read_to_string(path)
        .expect("Unable to read configuration file");
    Ok(file_content)
}
