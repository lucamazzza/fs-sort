use crate::config::{Config, ConfigFile};

pub fn get_basedir_validated(conf: ConfigFile) -> Result<String, &'static str> {
    let basedir = conf.get_basedir();
    if basedir.is_empty() {
        return Err("Base Directory not defined")
    }
    Ok(basedir)
}
