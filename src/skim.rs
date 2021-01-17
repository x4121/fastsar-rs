extern crate skim;
use crate::json::Account;
use skim::prelude::*;
use std::io::Cursor;

pub fn get_selection(header: &String, options: &Vec<String>) -> Option<usize> {
    let skim_options = SkimOptionsBuilder::default()
        .header(Some(&header))
        .build()
        .unwrap();

    let items = SkimItemReader::default().of_bufread(Cursor::new(options.join("\n")));

    match Skim::run_with(&skim_options, Some(items)) {
        Some(out) if out.final_event != Event::EvActAbort => {
            let item = &out.selected_items[0];
            options.iter().position(|e| e == &item.output())
        }
        _ => None
    }
}

pub fn select_account(mut accounts: Vec<Account>) -> Option<Account> {
    let account_names: &Vec<String> = &accounts.clone().into_iter().map(|e| e.name).collect();
    get_selection(&String::from("Accounts:"), account_names).map(|i| accounts.remove(i))
}

pub fn select_role(mut roles: Vec<String>) -> Option<String> {
    get_selection(&String::from("Roles:"), &roles).map(|i| roles.remove(i))
}
