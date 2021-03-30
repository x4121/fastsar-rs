extern crate serde_json;
use crate::util;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub type Role = String;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Account {
    pub name: String,
    pub id: String,
    pub roles: Vec<Role>,
}

fn parse_json(contents: &str) -> Result<Vec<Account>> {
    let acc: Vec<Account> = serde_json::from_str(&contents)?;
    Ok(acc)
}

pub fn read_config(path: &Path) -> Result<Vec<Account>> {
    let contents = util::read_file(&path)?;
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
