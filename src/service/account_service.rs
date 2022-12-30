use crate::dto::account::{Account};
use crate::utils::string::capitalize; 
use crate::utils::io::{get_user_input, confirm_action, write_json_data, get_json_data, build_account};
use crate::utils::messages::{print_account, print_user_input};

pub fn list_sites() {
    let accounts = get_json_data();
    println!("List of accounts:");
    println!("==================");

    for account in accounts {
        println!(" - {}", capitalize(&account.site));
    }

    println!("==================");
}

pub fn get_one_account() {
    println!("🌎 Please enter the site:");
    let site = get_user_input("site".to_string(), false);

    let account = find_by_site(&site);

    if account.is_some() {
        print_account(&account.unwrap());
    } else {
        println!("❗️ Account not found");
    }
}

pub fn get_all_accounts() -> Vec<Account> {
    get_json_data()
}

pub fn find_by_site(site: &str) -> Option<Account> {
    let accounts = get_all_accounts();
    accounts.iter().find(|&account| account.site == site).cloned()
}

pub fn delete_account() {
    let mut accounts = get_all_accounts();
    println!("🌎 Please enter the site:");
    let site = get_user_input("site".to_string(), false);

    let account_optional = find_by_site(&site);

    if account_optional.is_none() {
        println!("❗️ No account found for {}", site);
        return;
    }

    let account = account_optional.unwrap();

    print_account(&account);

    if confirm_action("Do you want to delete this account? (y/n)") {
        accounts.retain(|a| a.site != site);

        write_json_data(&accounts);

        println!("✅ Account deleted!");
    }
}

pub fn edit_account() {
    let mut accounts = get_all_accounts();

    println!("🌎 Please enter the site :");
    let site = get_user_input("site".to_string(), false);

    let account_optional = accounts
        .iter()
        .find(|a| a.site == site);

    if account_optional.is_none() {
        println!("❗️ No account found for {}", site);
        return;
    }

    let account = account_optional.unwrap();

    print_account(&account);

    if confirm_action("Do you want to edit this account? (y/n)") {
        loop {
            let account = build_account();

            print_user_input(&account);
            println!(); 

            if confirm_action("Do you want to save this account? (y/n)") {
                accounts.retain(|a| a.site != site);
                accounts.push(account);
    
                write_json_data(&accounts);
    
                println!("✅ Account saved!");
                break;
            } else {
                println!("Re-enter your account info:");
            }
        }
    }
}

pub fn save_account() {
    let mut accounts = get_all_accounts();

    loop {
        let account = build_account();

        print_user_input(&account);

        if confirm_action("Do you want to save this account? (y/n)") {
            accounts.push(account);

            write_json_data(&accounts);

            println!("✅ Account saved!");
            break;
        } else {
            println!("Re-enter your account info:");
        }
    }
}