#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;
use crate::arguments::Arguments;
use crate::config::{Account, Role};
use crate::shell::Shell;
use anyhow::Result;
use history::History;
use rusoto_sts::Credentials;
use simplelog::{Config, TermLogger};
use std::io::Write;
use std::process;
use structopt::StructOpt;
use subprocess::Exec;
use termcolor::{ColorSpec, StandardStream, WriteColor};

mod arguments;
mod aws;
mod config;
mod history;
mod shell;
mod skim;
mod util;

#[tokio::main]
async fn main() {
    let arguments: Arguments = Arguments::from_args();

    let _ = TermLogger::init(
        arguments.get_log_level(),
        Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    );

    debug!("{:#?}", arguments);
    let shell = shell::get_shell(&arguments.shell);
    debug!("Shell: {:?}", &shell);
    let history = history::read(&arguments.get_history_path());
    debug!("History: {:?}", &history);
    let region = match aws::get_region(arguments.region.as_deref()) {
        Ok(region) => region,
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    };
    debug!("Region: {:?}", &region);

    let (account_id, role) = match (&arguments.account, &arguments.role) {
        (Some(account_id), Some(role)) => (account_id.to_string(), role.to_string()),
        _ => {
            let account = match select_account(&arguments, history.as_ref()) {
                Ok(Some(account)) => account,
                Ok(None) => process::exit(0),
                Err(err) => {
                    error!("{}", err);
                    process::exit(1);
                }
            };
            debug!("Account: {:?}", &account.id);
            let role = match select_role(&account, &arguments, history.as_ref()) {
                Ok(Some(role)) => role,
                Ok(None) => process::exit(0),
                Err(err) => {
                    error!("{}", err);
                    process::exit(1);
                }
            };
            debug!("Role: {:?}", &role);
            (account.id, role)
        }
    };

    let credentials = match aws::assume_role(&account_id, &role, region, &arguments).await {
        Ok(credentials) => credentials,
        Err(err) => {
            error!("{}", err);
            process::exit(1);
        }
    };

    let status = match &arguments.exec {
        Some(exec) => set_credentials_and_exec(&credentials, exec),
        None => print_credentials(&shell, &credentials),
    };
    if status.is_err() {
        process::exit(1);
    }
    let history = History { account_id, role };
    if let Err(err) = history::write(&arguments.get_history_path(), &history) {
        error!("{}", err);
        process::exit(1);
    }
}

fn select_account(arguments: &Arguments, history: Option<&History>) -> Result<Option<Account>> {
    let mut accounts = config::read(&arguments.get_config_path())?;
    match &arguments.account {
        Some(account_id) => {
            let account = accounts.iter().find(|a| &a.id == account_id);
            match account {
                Some(account) => Ok(Some(account.clone())),
                None => bail!("Account {} not found in config.", account_id),
            }
        }
        _ => match accounts.len() {
            0 => bail!("Config file is empty."),
            1 => Ok(Some(accounts.remove(0))),
            _ => Ok(skim::select_account(
                accounts,
                history.map(|h| h.account_id.as_str()),
            )),
        },
    }
}

fn select_role(
    account: &Account,
    arguments: &Arguments,
    history: Option<&History>,
) -> Result<Option<Role>> {
    let mut roles = account.clone().roles;
    match &arguments.role {
        Some(role) if roles.iter().any(|r| r == role) => Ok(Some(role.to_string())),
        Some(role) => bail!("Role {} not found in config.", role),
        _ => match roles.len() {
            0 => bail!("Account {} has no assigned roles.", account.id),
            1 => Ok(Some(roles.remove(0))),
            _ => Ok(skim::select_role(roles, history.map(|h| h.role.as_str()))),
        },
    }
}

fn set_credentials_and_exec(credentials: &Credentials, exec: &str) -> Result<()> {
    set_credentials(credentials)?;
    let _ = Exec::shell(exec).join()?;
    Ok(())
}

fn set_credentials(credentials: &Credentials) -> Result<()> {
    if let Err(err) = shell::set_var(aws::ACCESS_KEY_ID, &credentials.access_key_id) {
        error!("Could not set env '{}': {}", aws::ACCESS_KEY_ID, err);
        return Err(err);
    };
    if let Err(err) = shell::set_var(aws::SECRET_ACCESS_KEY, &credentials.secret_access_key) {
        error!("Could not set env '{}': {}", aws::SECRET_ACCESS_KEY, err);
        return Err(err);
    };
    if let Err(err) = shell::set_var(aws::SESSION_TOKEN, &credentials.session_token) {
        error!("Could not set env '{}': {}", aws::SESSION_TOKEN, err);
        return Err(err);
    };
    Ok(())
}

fn print_credentials(shell: &Shell, credentials: &Credentials) -> Result<()> {
    match shell::export_string(shell, aws::ACCESS_KEY_ID, &credentials.access_key_id) {
        Ok(set_env) => println!("{}", set_env),
        Err(err) => {
            error!("Could not set env '{}': {}", aws::ACCESS_KEY_ID, err);
            return Err(err);
        }
    }
    match shell::export_string(
        shell,
        aws::SECRET_ACCESS_KEY,
        &credentials.secret_access_key,
    ) {
        Ok(set_env) => println!("{}", set_env),
        Err(err) => {
            error!("Could not set env '{}': {}", aws::SECRET_ACCESS_KEY, err);
            return Err(err);
        }
    }
    match shell::export_string(shell, aws::SESSION_TOKEN, &credentials.session_token) {
        Ok(set_env) => println!("{}", set_env),
        Err(err) => {
            error!("Could not set env '{}': {}", aws::SESSION_TOKEN, err);
            return Err(err);
        }
    }

    let mut stderr = StandardStream::stderr(termcolor::ColorChoice::Always);
    stderr.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Green)))?;
    writeln!(
        &mut stderr,
        "Session valid until {}.",
        credentials.expiration
    )?;
    Ok(())
}
