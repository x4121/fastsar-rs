use std::env;

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
