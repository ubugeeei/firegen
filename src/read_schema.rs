use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
use std::path::{Path, PathBuf};

pub fn read_multi_schema(target_dirs: Vec<&str>) -> String {
    let mut merged_schema_strings = String::new();

    for dir in target_dirs.iter() {
        let mut paths = vec![];
        get_paths(dir, paths.as_mut()).ok();

        for path in paths.iter() {
            let mut is_file = false;
            for s in path.to_str().unwrap().chars() {
                if s == '.' {
                    is_file = true;
                }
            }
            if is_file {
                let result = read_schema(path.as_os_str().to_str().unwrap()).unwrap();
                merged_schema_strings.push_str("\n\n");
                merged_schema_strings.push_str(&result);
            }
        }
    }

    merged_schema_strings
}

fn read_schema(file_path: &str) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_paths<P: AsRef<Path>>(path: P, paths: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            get_paths(entry.path(), paths)?;
        }
        paths.push(entry.path());
    }
    Ok(())
}
