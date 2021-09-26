use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
use std::path::{Path, PathBuf};

pub fn read_file(file_path: &str) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
