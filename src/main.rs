#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;
use crate::arguments::Arguments;
use crate::json::{Account, Role};
use crate::shell::Shell;
use anyhow::Result;
use rusoto_sts::Credentials;
use simple_logger::SimpleLogger;
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

    SimpleLogger::new()
        .with_level(arguments.get_debug())
        .init()
        .unwrap();

    debug!("{:#?}", arguments);
    let shell = shell::get_shell(&arguments.shell);
    debug!("Shell: {:?}", &shell);
    let region = match aws::get_region(&arguments.region) {
        Ok(region) => region,
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    };
    debug!("Region: {:?}", &region);

    let account = match select_account(&arguments) {
        Ok(Some(account)) => account,
        Ok(None) => process::exit(0),
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    };
    debug!("Account: {:?}", &account.id);
    let role = match select_role(&account, &arguments) {
        Ok(Some(role)) => role,
        Ok(None) => process::exit(0),
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    };
    debug!("Role: {:?}", &role);
    let credentials = match aws::assume_role(&account.id, &role, region, &arguments).await {
        Ok(credentials) => credentials,
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    };

    match arguments.exec {
        Some(exec) => {
            set_credentials(&credentials);
            let _ = Exec::shell(&exec).join();
        }
        None => print_credentials(&shell, &credentials),
    };
}

fn select_account(arguments: &Arguments) -> Result<Option<Account>> {
    let mut accounts = json::read_config(&arguments.get_config_path())?;
    match &arguments.account {
        Some(account_id) => {
            let account = accounts
                .iter()
                .filter(|&a| &a.id == account_id)
                .cloned()
                .collect::<Vec<Account>>()
                .first()
                .cloned();
            match account {
                Some(account) => Ok(Some(account)),
                None => bail!("Account {} not found in config.", account_id),
            }
        }
        _ => match accounts.len() {
            0 => bail!("Config file is empty."),
            1 => Ok(Some(accounts.remove(0))),
            _ => skim::select_account(accounts),
        },
    }
}

fn select_role(account: &Account, arguments: &Arguments) -> Result<Option<Role>> {
    let mut roles = account.clone().roles;
    match &arguments.role {
        Some(role) if roles.iter().any(|r| r == role) => Ok(Some(role.to_string())),
        Some(role) => bail!("Role {} not found in config.", role),
        _ => match roles.len() {
            0 => bail!("Account {} has no assigned roles.", account.id),
            1 => Ok(Some(roles.remove(0))),
            _ => skim::select_role(roles),
        },
    }
}

fn set_credentials(credentials: &Credentials) {
    if let Err(err) = shell::set_var(aws::ACCESS_KEY_ID, &credentials.access_key_id) {
        error!("Could not set env '{}': {}", aws::ACCESS_KEY_ID, err);
    };
    if let Err(err) = shell::set_var(aws::SECRET_ACCESS_KEY, &credentials.secret_access_key) {
        error!("Could not set env '{}': {}", aws::SECRET_ACCESS_KEY, err);
    };
    if let Err(err) = shell::set_var(aws::SESSION_TOKEN, &credentials.session_token) {
        error!("Could not set env '{}': {}", aws::SESSION_TOKEN, err);
    };
}

fn print_credentials(shell: &Shell, credentials: &Credentials) {
    match shell::export_string(&shell, aws::ACCESS_KEY_ID, &credentials.access_key_id) {
        Ok(set_env) => println!("{}", set_env),
        Err(err) => error!("Could not set env '{}': {}", aws::ACCESS_KEY_ID, err),
    }
    match shell::export_string(
        &shell,
        aws::SECRET_ACCESS_KEY,
        &credentials.secret_access_key,
    ) {
        Ok(set_env) => println!("{}", set_env),
        Err(err) => error!("Could not set env '{}': {}", aws::SECRET_ACCESS_KEY, err),
    }
    match shell::export_string(&shell, aws::SESSION_TOKEN, &credentials.session_token) {
        Ok(set_env) => println!("{}", set_env),
        Err(err) => error!("Could not set env '{}': {}", aws::SESSION_TOKEN, err),
    }
}
