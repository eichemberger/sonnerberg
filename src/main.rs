use serde_json;

use std::fs::write;
use std::env; 
use sonnerberg::utils::messages::{print_menu, welcome, print_account, print_user_input, list_sites};
use sonnerberg::dto::account::{Account};
use sonnerberg::dto::menu_option::MenuOption;
use sonnerberg::utils::io::{read_input, confirm_action, build_account, get_user_input};
use sonnerberg::utils::io::{get_json_data};
use sonnerberg::commands::execute_command; 
use sonnerberg::service::account_service::{delete_account, edit_account, get_one_account, save_account};

pub fn get_selected_option(accounts: &mut Vec<Account>) -> i32 {
    let option = read_input().parse().unwrap_or(-1);
    match option {
        MenuOption::ADD => save_account(),
        MenuOption::DELETE => delete_account(),
        MenuOption::LIST => list_sites(accounts),
        MenuOption::EDIT => edit_account(),
        MenuOption::GET => get_one_account(),
        MenuOption::SHOW_MENU => print_menu(),
        MenuOption::EXIT => println!("Bye!"),
        _ => {
            println!("❗️ Please enter a valid option");
            println!();
            print_menu();
        },
    }

    option
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut data = get_json_data();

    if args.len() < 2 {

        welcome();
        println!("Loading...");
        println!();
        println!("What do you want to do?");
        print_menu(); 
        loop {
            println!();

            let option = get_selected_option(&mut data);
            if option == MenuOption::EXIT {
                break;
            }
        }

    } else {
        execute_command(); 
    }

}
