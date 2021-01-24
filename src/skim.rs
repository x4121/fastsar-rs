extern crate skim;
use crate::json::Account;
use skim::prelude::*;
use std::io::Cursor;

fn get_selection(header: &String, options: &Vec<String>) -> Option<usize> {
    let skim_options = SkimOptionsBuilder::default()
        .header(Some(&header))
        .build()
        .unwrap();

    let items = SkimItemReader::default().of_bufread(Cursor::new(options.join("\n")));

    if let Some(out) = Skim::run_with(&skim_options, Some(items)) {
        if let Event::EvActAccept(_) = out.final_event {
            let item = &out.selected_items[0];
            return options.iter().position(|e| e == &item.output());
        }
    }
    None
}

fn get_account_names(accounts: &Vec<Account>) -> Vec<String> {
    accounts.clone().into_iter().map(|e| e.name).collect()
}

pub fn select_account(mut accounts: Vec<Account>) -> Option<Account> {
    let account_names = get_account_names(&accounts);
    get_selection(&String::from("Accounts:"), &account_names).map(|i| accounts.remove(i))
}

pub fn select_role(mut roles: Vec<String>) -> Option<String> {
    get_selection(&String::from("Roles:"), &roles).map(|i| roles.remove(i))
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
