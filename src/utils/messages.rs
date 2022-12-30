use crate::dto::account::{Account};
use crate::utils::string::capitalize; 

pub fn welcome() {
    println!(r###"
   _____                             __
  / ___/____  ____  ____  ___  _____/ /_  ___  _________ _
  \__ \/ __ \/ __ \/ __ \/ _ \/ ___/ __ \/ _ \/ ___/ __ `/
 ___/ / /_/ / / / / / / /  __/ /  / /_/ /  __/ /  / /_/ /
/____/\____/_/ /_/_/ /_/\___/_/  /_.___/\___/_/   \__, /
                                                 /____/
                                by German Eichemberger
"###);
    println!("");
}

pub fn print_menu() {
    println!("━━━━━━━━━▲━━━━━━━━━");
    println!("1. List accounts");
    println!("2. Add account");
    println!("3. Edit account");
    println!("4. Delete account");
    println!("5. Get account info");
    println!("6. Exit");
    println!("━━━━━━━━━▼━━━━━━━━━");
}

pub fn print_account(account: &Account) {
    if account.url.is_some() {
        println!("🌐 {}", account.url.as_ref().unwrap());
    }
    println!("👤 {}", account.login);
    println!("🔑 {}", account.password); 
    println!("🌎 {}", account.site);
}

pub fn print_user_input(account: &Account) {
    println!();
    if account.url.is_some() {
        println!(" 🌐 Your site url is: {}", account.url.as_ref().unwrap());
    }
    println!(" 🌎 Your site is: {}", account.site);
    println!(" 👤 Your login method is: {}", account.login);
    println!(" 🔑 Your password is: {}", account.password);
    println!();
}

pub fn list_sites(accounts: &mut Vec<Account>) {
    println!("List of accounts:");

    println!("==================");
    for account in accounts {
        println!(" - {}", capitalize(&account.site));
    }
    println!("==================");
}