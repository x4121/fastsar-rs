use std::env;

#[derive(PartialEq, Debug)]
pub enum Shell {
    Fish,
    Bash,
}

fn from_str(s: &str) -> Shell {
    match s {
        "fish" => Shell::Fish,
        _ => Shell::Bash,
    }
}

pub fn get_shell(preselect: &Option<String>) -> Shell {
    match preselect {
        Some(shell) => from_str(shell.as_str()),
        _ => match env::var("SHELL") {
            Ok(shell) => from_str(shell.split("/").last().unwrap()),
            _ => from_str(""),
        },
    }
}

pub fn export_string(shell: &Shell, var: &str, val: &String) -> String {
    match shell {
        Shell::Fish => format!("set -gx {} {};", var, val),
        Shell::Bash => format!("export {}={}", var, val),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_from_string() {
        assert_eq!(Shell::Bash, from_str("bash"));
        assert_eq!(Shell::Fish, from_str("fish"));
        assert_eq!(Shell::Bash, from_str("zsh")); // TODO: replace with quickcheck
    }

    #[test]
    fn reading_from_env() {
        env::set_var("SHELL", "/usr/bin/fish");
        assert_eq!(Shell::Fish, get_shell(&None));
        env::set_var("SHELL", "/usr/bin/bash");
        assert_eq!(Shell::Bash, get_shell(&None));
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
            export_string(&Shell::Fish, "", &String::from("")),
            "set -gx  ;"
        ); // TODO: this should panic
        assert_eq!(
            export_string(&Shell::Fish, "FOO", &String::from("bar")),
            "set -gx FOO bar;"
        );
        assert_eq!(
            export_string(&Shell::Bash, "", &String::from("")),
            "export ="
        ); // TODO: this should panic
        assert_eq!(
            export_string(&Shell::Bash, "FOO", &String::from("bar")),
            "export FOO=bar"
        );
    }
}
