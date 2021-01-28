extern crate skim;
use crate::json::{Account, Role};
use anyhow::Result;
use skim::prelude::*;
use std::io::Cursor;

fn get_selection(header: &String, options: &Vec<String>) -> Result<Option<usize>> {
    let skim_options = SkimOptionsBuilder::default()
        .header(Some(&header))
        .build()
        .unwrap();

    let items = SkimItemReader::default().of_bufread(Cursor::new(options.join("\n")));

    if let Some(out) = Skim::run_with(&skim_options, Some(items)) {
        if let Event::EvActAccept(_) = out.final_event {
            let item = &out.selected_items[0];
            Ok(options.iter().position(|e| e == &item.output()))
        } else {
            Ok(None)
        }
    } else {
        bail!("Err")
    }
}

fn get_account_names(accounts: &Vec<Account>) -> Vec<String> {
    accounts.clone().into_iter().map(|e| e.name).collect()
}

pub fn select_account(mut accounts: Vec<Account>) -> Result<Option<Account>> {
    let account_names = get_account_names(&accounts);
    get_selection(&String::from("Accounts:"), &account_names).map(|o| o.map(|a| accounts.remove(a)))
}

pub fn select_role(mut roles: Vec<Role>) -> Result<Option<Role>> {
    get_selection(&String::from("Roles:"), &roles).map(|o| o.map(|r| roles.remove(r)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_names_from_empty_list() {
        let expected: Vec<String> = Vec::new();
        assert_eq!(get_account_names(&Vec::new()), expected);
    }

    #[test]
    fn account_names_from_accounts() {
        let accounts = vec![
            Account {
                name: String::from("foo"),
                id: String::from("123123123"),
                roles: vec![String::from("admin")],
            },
            Account {
                name: String::from("bar"),
                id: String::from("321321321"),
                roles: vec![String::from("user")],
            },
        ];
        let expected = vec![accounts[0].name.clone(), accounts[1].name.clone()];
        assert_eq!(get_account_names(&accounts), expected);
    }
}
