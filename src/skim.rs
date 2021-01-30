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
            return Ok(options.iter().position(|e| e == &item.output()));
        }
    }
    Ok(None)
}

fn get_account_names(accounts: &Vec<Account>) -> Vec<String> {
    accounts.clone().into_iter().map(|e| e.name).collect()
}

fn sort_with_preselect(list: &Vec<String>, preselect: &Option<String>) -> Vec<String> {
    if let Some(element) = preselect {
        if let Some(pos) = list.iter().position(|e| e == element) {
            let mut list = list.to_vec();
            list.remove(pos);
            list.insert(0, element.to_string());
            return list;
        }
    }
    list.clone()
}

pub fn select_account(
    mut accounts: Vec<Account>,
    preselect: &Option<String>,
) -> Result<Option<Account>> {
    let account_names = get_account_names(&accounts);
    let mut account_names = sort_with_preselect(&account_names, preselect);
    get_selection(&String::from("Accounts:"), &account_names).map(|o| {
        o.map(|a| {
            let account_name = account_names.remove(a);
            let pos = accounts
                .iter()
                .position(|e| e.name == account_name)
                .unwrap();
            accounts.remove(pos)
        })
    })
}

pub fn select_role(roles: Vec<Role>, preselect: &Option<String>) -> Result<Option<Role>> {
    let mut roles = sort_with_preselect(&roles, preselect);
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
