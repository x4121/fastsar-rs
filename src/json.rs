extern crate serde_json;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Account {
    pub name: String,
    pub id: String,
    pub roles: Vec<String>,
}

fn read_file(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_json(contents: &String) -> Result<Vec<Account>> {
    let acc: Vec<Account> = serde_json::from_str(&contents)?;
    Ok(acc)
}

pub fn read_config(path: &PathBuf) -> Result<Vec<Account>> {
    let contents = read_file(&path)?;
    let res = parse_json(&contents)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_empty() {
        assert_eq!(parse_json(&String::from("[]")).unwrap(), Vec::new());
    }

    #[test]
    fn read_minimal() {
        let minimal = String::from(
            r#"[{
            "name": "foo",
            "id": "123456789",
            "roles": ["admin"]
        }]"#,
        );
        let expected = Account {
            name: String::from("foo"),
            id: String::from("123456789"),
            roles: vec![String::from("admin")],
        };
        assert_eq!(parse_json(&minimal).unwrap(), vec![expected]);
    }
}
