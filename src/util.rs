use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(path: &Path, contents: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
