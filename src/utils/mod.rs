use std::io::{self, Write};
use std::{fs, fs::File, path::Path};

use crate::config::FilePath;

pub fn path_exists(path: &mut FilePath) -> bool {
    Path::new(path).exists()
}

pub fn mkdir_if_inexistent(dirpath: FilePath) -> Result<(), io::Error> {
    fs::create_dir_all(dirpath)?;
    Ok(())
}

pub fn write_file(path: &mut FilePath, buf: String) -> Result<(), &'static str> {
    if !path_exists(path) {
        return Err("File not found");
    }
    if buf.is_empty() {
        return Err("Cannot write \"\" into file");
    }
    let mut file = File::create(path).unwrap();
    file.write_all(buf.as_bytes()).unwrap();
    Ok(())
}
