use crate::arguments::Arguments;
use json::Account;
use structopt::StructOpt;

mod arguments;
mod json;
mod skim;

fn main() {
    let arguments: Arguments = Arguments::from_args();
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
        })
    });
}
