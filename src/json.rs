extern crate serde_json;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    pub id: String,
    pub roles: Vec<String>,
}

fn read_file(path: &PathBuf) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_json(contents: &String) -> Result<Vec<Account>, serde_json::Error> {
    serde_json::from_str::<Vec<Account>>(&contents)
}

pub fn read_config(path: &PathBuf) -> Result<Vec<Account>, io::Error> {
    let contents = read_file(&path)?;
    let res = parse_json(&contents)?;
    Ok(res)
}
