extern crate serde_json;
use crate::util;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct History {
    pub account_id: String,
    pub role: String,
}

fn parse_json(contents: &str) -> Option<History> {
    serde_json::from_str(contents).ok()
}

pub fn read(path: &Path) -> Option<History> {
    let contents = util::read_file(path).ok();
    contents.map(|c| parse_json(&c)).flatten()
}

pub fn write(path: &Path, history: &History) -> Result<()> {
    let contents = serde_json::to_string(&history)?;
    util::write_file(path, &contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_empty() {
        assert_eq!(parse_json(&String::from("")), None);
        assert_eq!(parse_json(&String::from("{}")), None);
    }

    #[test]
    fn read_invalid() {
        assert_eq!(parse_json(&String::from("{")), None);
        let old = String::from(
            r#"some-account-name
            some-role-name"#,
        );
        assert_eq!(parse_json(&old), None);
    }

    #[test]
    fn read_valid() {
        let valid = String::from(
            r#"{
            "account_id": "123123123123",
            "role": "some-role-name"
            }"#,
        );
        let expected = History {
            account_id: String::from("123123123123"),
            role: String::from("some-role-name"),
        };
        assert_eq!(parse_json(&valid), Some(expected));
    }
}
