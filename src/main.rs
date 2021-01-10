use crate::arguments::Arguments;
use json::Account;
use rusoto_sts::Credentials;
use shell::Shell;
use std::env;
use structopt::StructOpt;

mod arguments;
mod aws;
mod json;
mod shell;
mod skim;

#[tokio::main]
async fn main() {
    let arguments: Arguments = Arguments::from_args();
    let shell = shell::get_shell(&arguments.shell);
    println!("{:#?}", arguments);
    let account: Option<Account> = select_account(&arguments);
    let role: Option<String> = match &account {
        Some(a) => select_role(&a),
        _ => None,
    };
    let credentials = match (account, role) {
        (Some(a), Some(r)) => {
            let id = a.id;
            match aws::assume_role(&id, &r).await {
                Ok(credentials) => Some(credentials),
                _ => None,
            }
        }
        _ => None,
    };

    match credentials {
        Some(c) => print_credentials(&shell, &c),
        _ => (),
    };
}

fn select_account(arguments: &Arguments) -> Option<Account> {
    let mut accounts: Vec<Account> = json::read_config(&arguments.get_config_path()).unwrap();
    match accounts.len() {
        0 => None,
        1 => Some(accounts.remove(0)),
        _ => skim::select_account(accounts),
    }
}

fn select_role(account: &Account) -> Option<String> {
    let mut roles: Vec<String> = account.clone().roles;
    match roles.len() {
        0 => None,
        1 => Some(roles.remove(0)),
        _ => skim::select_role(roles),
    }
}

fn set_credentials(credentials: &Credentials) {
    env::set_var(aws::ACCESS_KEY_ID, &credentials.access_key_id);
    env::set_var(aws::SECRET_ACCESS_KEY, &credentials.secret_access_key);
    env::set_var(aws::SESSION_TOKEN, &credentials.session_token);
}

fn print_credentials(shell: &Shell, credentials: &Credentials) {
    println!(
        "{}",
        shell::export_string(&shell, aws::ACCESS_KEY_ID, &credentials.access_key_id)
    );
    println!(
        "{}",
        shell::export_string(
            &shell,
            aws::SECRET_ACCESS_KEY,
            &credentials.secret_access_key
        )
    );
    println!(
        "{}",
        shell::export_string(&shell, aws::SESSION_TOKEN, &credentials.session_token)
    );
}
