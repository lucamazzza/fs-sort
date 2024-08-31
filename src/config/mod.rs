use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

pub const CONFIG_FILE_PATH: &str = r#"~/.config/fs-sort.toml"#;

pub type FilePath = String;

pub trait Config {
    fn get_basedir(&self) -> FilePath;
    fn get_file_types(&self) -> FileTypes;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile { // TODO: Define single structures for each section of the config
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

fn path_exists(path: &mut FilePath) -> bool {
    Path::new(path).exists()
}

fn read_config_file(path: &mut FilePath) -> Result<String, &'static str> {
    if !path_exists(path) { return Err("File not found") }
    let file_content = fs::read_to_string(path)
        .expect("Unable to read configuration file");
    Ok(file_content)
}

pub fn toml_string_as_config(path: &mut FilePath) -> Result<ConfigFile, String> {
    let file_content = read_config_file(path).unwrap();
    if file_content.is_empty() {
        return Err("Config file is empty".to_string())
    }
    let config: ConfigFile = des(file_content);
    Ok(config)
}
