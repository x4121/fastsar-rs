extern crate skim;
use crate::config::{Account, Role};
use skim::prelude::*;
use std::io::Cursor;

fn get_selection(header: &str, options: &[String]) -> Option<usize> {
    let skim_options = SkimOptionsBuilder::default()
        .header(Some(header))
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

fn get_account_names(accounts: &[Account]) -> Vec<String> {
    accounts.iter().map(|e| e.name.clone()).collect()
}

fn sort_with_preselect<T: Clone>(
    list: &[T],
    preselect: Option<&str>,
    finder: &dyn Fn(&[T], &str) -> Option<usize>,
) -> Vec<T> {
    if let Some(pos) = preselect.and_then(|el| finder(list, el)) {
        let mut list = list.to_vec();
        let x = list.remove(pos);
        list.insert(0, x);
        list
    } else {
        list.to_owned()
    }
}

pub fn select_account(accounts: Vec<Account>, preselect: Option<&str>) -> Option<Account> {
    fn finder(list: &[Account], p: &str) -> Option<usize> {
        list.iter().position(|a| a.id == p)
    }
    let accounts = sort_with_preselect(&accounts, preselect, &finder);
    let account_names = get_account_names(&accounts);
    let pos = get_selection(&String::from("Accounts:"), &account_names);
    get_account_from_sorted_names(accounts, account_names, &pos)
}

fn get_account_from_sorted_names(
    mut accounts: Vec<Account>,
    mut account_names: Vec<String>,
    pos: &Option<usize>,
) -> Option<Account> {
    pos.map(|p| {
        let account_name = account_names.remove(p);
        let pos = accounts
            .iter()
            .position(|e| e.name == account_name)
            .unwrap();
        accounts.remove(pos)
    })
}

pub fn select_role(roles: Vec<Role>, preselect: Option<&str>) -> Option<Role> {
    fn finder(list: &[String], p: &str) -> Option<usize> {
        list.iter().position(|a| a == p)
    }
    let mut roles = sort_with_preselect(&roles, preselect, &finder);
    get_selection(&String::from("Roles:"), &roles).map(|r| roles.remove(r))
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

    #[test]
    fn sort_list() {
        fn finder(list: &[String], p: &str) -> Option<usize> {
            list.iter().position(|a| a == p)
        }

        let list = vec![
            String::from("foo"),
            String::from("bar"),
            String::from("baz"),
        ];
        assert_eq!(sort_with_preselect(&list, None, &finder), list);
        assert_eq!(sort_with_preselect(&list, Some("nope"), &finder), list);

        let sorted_list = vec![
            String::from("baz"),
            String::from("foo"),
            String::from("bar"),
        ];
        assert_eq!(
            sort_with_preselect(&list, Some("baz"), &finder),
            sorted_list
        );
    }

    #[test]
    fn get_from_sorted_list() {
        let accounts = vec![
            Account {
                name: String::from("foo"),
                id: String::from("1"),
                roles: vec![String::from("user")],
            },
            Account {
                name: String::from("bar"),
                id: String::from("2"),
                roles: vec![String::from("admin")],
            },
        ];
        let account_names = get_account_names(&accounts);
        assert_eq!(
            get_account_from_sorted_names(accounts.clone(), account_names.clone(), &None),
            None
        );
        assert_eq!(
            get_account_from_sorted_names(accounts.clone(), account_names.clone(), &Some(0)),
            Some(accounts[0].clone())
        );
        assert_eq!(
            get_account_from_sorted_names(accounts.clone(), account_names.clone(), &Some(1)),
            Some(accounts[1].clone())
        );

        fn finder(list: &[Account], p: &str) -> Option<usize> {
            list.iter().position(|a| a.id == p)
        }
        let sorted = sort_with_preselect(&accounts, Some("2"), &finder);
        let account_names = get_account_names(&sorted);
        assert_eq!(
            get_account_from_sorted_names(accounts.clone(), account_names.clone(), &Some(0)),
            Some(accounts[1].clone())
        );
        assert_eq!(
            get_account_from_sorted_names(accounts.clone(), account_names.clone(), &Some(1)),
            Some(accounts[0].clone())
        );
    }
}
