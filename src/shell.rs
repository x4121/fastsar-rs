use crate::error::Error;
use std::env;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Shell {
    Fish,
    Bash,
}

impl Default for Shell {
    fn default() -> Self {
        Shell::Bash
    }
}

impl FromStr for Shell {
    type Err = ();

    fn from_str(shell: &str) -> Result<Self, Self::Err> {
        match shell {
            "fish" => Ok(Shell::Fish),
            _ => Ok(Shell::default()),
        }
    }
}

pub fn get_shell(preselect: &Option<String>) -> Shell {
    match preselect {
        Some(shell) => Shell::from_str(shell.as_str()).unwrap(),
        _ => match env::var("SHELL") {
            Ok(shell) => Shell::from_str(shell.split("/").last().unwrap()).unwrap(),
            _ => Shell::default(),
        },
    }
}

pub fn export_string(shell: &Shell, var: &str, val: &String) -> Result<String, Error> {
    if var.is_empty() || val.is_empty() {
        Err(Error::InvalidSetEnv)
    } else {
        let string = match shell {
            Shell::Fish => format!("set -gx {} {};", var, val),
            Shell::Bash => format!("export {}={}", var, val),
        };
        Ok(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_from_string() {
        assert_eq!(Shell::Bash, Shell::from_str("bash").unwrap());
        assert_eq!(Shell::Fish, Shell::from_str("fish").unwrap());
        assert_eq!(Shell::default(), Shell::from_str("zsh").unwrap()); // TODO: replace with quickcheck
    }

    #[test]
    fn reading_from_env() {
        env::set_var("SHELL", "/usr/bin/fish");
        assert_eq!(Shell::Fish, get_shell(&None));
        env::set_var("SHELL", "/usr/bin/bash");
        assert_eq!(Shell::Bash, get_shell(&None));
        env::set_var("SHELL", "/usr/bin/zsh");
        assert_eq!(Shell::default(), get_shell(&None));
    }

    #[test]
    fn prefer_preselect_over_env() {
        env::set_var("SHELL", "/usr/bin/fish");
        assert_eq!(Shell::Bash, get_shell(&Some(String::from("bash"))));
    }

    #[test]
    fn string_formatting() {
        // TODO: generalize test cases for different shells
        assert_eq!(
            export_string(&Shell::Fish, "FOO", &String::from("bar")).unwrap(),
            "set -gx FOO bar;"
        );
        assert_eq!(
            export_string(&Shell::Bash, "FOO", &String::from("bar")).unwrap(),
            "export FOO=bar"
        );
    }
    #[test]
    fn prevent_invalid_setenv() {
        assert!(export_string(&Shell::default(), "", &String::from("")).is_err());
        assert!(export_string(&Shell::default(), "FOO", &String::from("")).is_err());
        assert!(export_string(&Shell::default(), "", &String::from("bar")).is_err());
    }
}
