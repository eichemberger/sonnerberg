use serde_json;

use crate::dto::account::{Account};
use crate::utils::config::FILE_PATH;
use std::fs::read_to_string;
use std::fs::write;

pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn confirm_action(mesage: &str) -> bool{
    println!("{}", mesage);
    let confirmation = read_input();

    confirmation.to_uppercase() == "Y" 
}

pub fn build_account() -> Account {
    println!("🌎 Please enter the site:");
    let site = get_user_input("site".to_string(), false);

    println!("👤 Please enter your login method (email/user):");
    let login = get_user_input("login method".to_string(), false);
    
    println!("🔑 Please enter your password:");
    let password = get_user_input("password".to_string(), false);

    println!("🌐 Please enter the site url (optional):");
    let url = get_user_input("url".to_string(), true);

    let url = if url == "" {
        None
    } else {
        Some(url)
    };
    
    Account {
        login,
        password,
        site,
        url
    }
}

pub fn get_user_input(input_type: String, optional: bool) -> String {
    loop {
        let input = read_input();
        if input == "" && !optional {
            println!("Please enter a valid {}:", input_type);
        } else {
            return input;
        }
    }    
}

pub fn get_json_data() -> Vec<Account> {
    let raw_data = read_to_string(FILE_PATH).expect("Unable to read file");
    let data: Vec<Account> = serde_json::from_str(&raw_data).unwrap();

    data
}

pub fn write_json_data(accounts: &Vec<Account>) {
    let json = serde_json::to_string_pretty(&accounts).unwrap();
    write(FILE_PATH, json).expect("Unable to write file");
}