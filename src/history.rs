use crate::util;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct History {
    pub account: String,
    pub role: String,
}

pub fn read(path: &PathBuf) -> Result<Option<History>> {
    let contents = util::read_file(&path).unwrap_or(String::from(""));
    let lines: Vec<&str> = contents.split("\n").collect();
    if lines.len() >= 2 {
        Ok(Some(History {
            account: String::from(lines[0]),
            role: String::from(lines[1]),
        }))
    } else {
        Ok(None)
    }
}

pub fn save(path: &PathBuf, history: &History) -> Result<()> {
    let contents = format!("{}\n{}", history.account, history.role);
    util::write_file(path, &contents)
}
