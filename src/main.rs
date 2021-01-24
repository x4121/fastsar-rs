#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
use crate::arguments::Arguments;
use crate::json::Account;
use crate::shell::Shell;
use rusoto_sts::Credentials;
use std::process;
use structopt::StructOpt;
use subprocess::Exec;

mod arguments;
mod aws;
mod json;
mod shell;
mod skim;

#[tokio::main]
async fn main() {
    let arguments: Arguments = Arguments::from_args();
    let shell = shell::get_shell(&arguments.shell);
    let region = match aws::get_region(&arguments.region) {
        Ok(region) => region,
        Err(err) => {
            eprintln!("{:?}", err);
            process::exit(1);
        }
    };

    println!("{:#?}", arguments);
    let account = select_account(&arguments);
    let role = if let Some(account) = &account {
        select_role(&account, &arguments)
    } else {
        None
    };
    println!(
        "account: {:?}, role: {:?}, region: {:?}",
        &account, &role, &region
    );
    let credentials = match (account, role) {
        (Some(account), Some(role)) => {
            match aws::assume_role(&account.id, &role, region, &arguments).await {
                Ok(credentials) => Some(credentials),
                _ => None,
            }
        }
        _ => None,
    };

    match (credentials, arguments.exec) {
        (Some(credentials), Some(exec)) => {
            set_credentials(&credentials);
            let _ = Exec::shell(&exec).join();
        }
        (Some(credentials), None) => print_credentials(&shell, &credentials),
        _ => (),
    };
}

fn select_account(arguments: &Arguments) -> Option<Account> {
    let mut accounts = json::read_config(&arguments.get_config_path()).unwrap();
    match &arguments.account {
        Some(account_id) => accounts
            .iter()
            .filter(|&a| &a.id == account_id)
            .cloned()
            .collect::<Vec<Account>>()
            .first()
            .cloned(),
        _ => match accounts.len() {
            0 => None,
            1 => Some(accounts.remove(0)),
            _ => skim::select_account(accounts),
        },
    }
}

fn select_role(account: &Account, arguments: &Arguments) -> Option<String> {
    let mut roles = account.clone().roles;
    match &arguments.role {
        Some(role) if roles.iter().any(|r| r == role) => Some(role.to_string()),
        Some(_) => None,
        _ => match roles.len() {
            0 => None,
            1 => Some(roles.remove(0)),
            _ => skim::select_role(roles),
        },
    }
}

fn set_credentials(credentials: &Credentials) {
    if let Err(err) = shell::set_var(aws::ACCESS_KEY_ID, &credentials.access_key_id) {
        eprintln!("Could not set env '{}': {}", aws::ACCESS_KEY_ID, err);
    };
    if let Err(err) = shell::set_var(aws::SECRET_ACCESS_KEY, &credentials.secret_access_key) {
        eprintln!("Could not set env '{}': {}", aws::SECRET_ACCESS_KEY, err);
    };
    if let Err(err) = shell::set_var(aws::SESSION_TOKEN, &credentials.session_token) {
        eprintln!("Could not set env '{}': {}", aws::SESSION_TOKEN, err);
    };
}

fn print_credentials(shell: &Shell, credentials: &Credentials) {
    match shell::export_string(&shell, aws::ACCESS_KEY_ID, &credentials.access_key_id) {
        Ok(set_env) => println!("{}", set_env),
        Err(err) => eprintln!("Could not set env '{}': {}", aws::ACCESS_KEY_ID, err),
    }
    match shell::export_string(
        &shell,
        aws::SECRET_ACCESS_KEY,
        &credentials.secret_access_key,
    ) {
        Ok(set_env) => println!("{}", set_env),
        Err(err) => eprintln!("Could not set env '{}': {}", aws::SECRET_ACCESS_KEY, err),
    }
    match shell::export_string(&shell, aws::SESSION_TOKEN, &credentials.session_token) {
        Ok(set_env) => println!("{}", set_env),
        Err(err) => eprintln!("Could not set env '{}': {}", aws::SESSION_TOKEN, err),
    }
}
