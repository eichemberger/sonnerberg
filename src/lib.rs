pub mod utils;
pub mod dto;
pub mod service;
pub mod commands;

use crate::utils::io::{read_input};
use crate::dto::menu_option::MenuOption;
use crate::service::account_service::{delete_account, edit_account, get_one_account, save_account, list_sites};
use crate::utils::messages::{print_menu, welcome};

pub fn run_app() {
    welcome();
    println!("What do you want to do?");
    print_menu(); 
    loop {
        println!();

        let option = get_selected_option();
        if option == MenuOption::EXIT {
            break;
        }
    }
}

fn get_selected_option() -> i32 {
    let option = read_input().parse().unwrap_or(-1);
    match option {
        MenuOption::ADD => save_account(),
        MenuOption::DELETE => delete_account(),
        MenuOption::LIST => list_sites(),
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

