use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn read_schema(file_path: &str) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
