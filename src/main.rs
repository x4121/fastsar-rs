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
    let mut accounts: Vec<Account> = json::read_config(&arguments.get_config_path()).unwrap();
    let account: Option<Account> = match accounts.len() {
        0 => None,
        1 => Some(accounts.remove(0)),
        _ => skim::select_account(accounts),
    };
    account.map(|a| {
        let id = a.id;
        let mut roles: Vec<String> = a.roles;
        let role: Option<String> = match roles.len() {
            0 => None,
            1 => Some(roles.remove(0)),
            _ => skim::select_role(roles),
        };

        role.map(|r| {
            println!("{:?} - {:?}", &id, &r);
            let x: Result<Credentials, String> = aws::assume_role(&id, &r);
            match aws::assume_role(&id, &r) {
                Ok(credentials) => print_credentials(&shell, &credentials),
                _ => (),
            };
        })
    });
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
