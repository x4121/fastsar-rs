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

pub fn read_config(path: &PathBuf) -> Result<Vec<Account>, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let res: Vec<Account> = serde_json::from_str(&mut contents)?;
    Ok(res)
}
