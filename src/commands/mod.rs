use std::process::ExitCode;
use std::env;
use crate::service::account_service::{list_sites, find_by_site};
use crate::utils::messages::print_account;

const GET: &str ="get";
const LIST: &str = "ls";
const PASSWORD_OPTION : &str = "-p";

pub fn execute_command() -> ExitCode  {
    let raw_args: Vec<String> = env::args().collect();
    let args = raw_args.iter().skip(1).collect::<Vec<&String>>();

    let command = &args[0];

    if command.chars().next().unwrap() == '-' {
        println!("the first argument must be a command");
        return ExitCode::FAILURE;
    }

    match command.as_str() {
        LIST => list_sites(),
        GET => return run_get(args),
        _ => {
            println!("Please enter a valid command");
            return ExitCode::FAILURE; 
        }
    }

    return ExitCode::SUCCESS;
}

fn run_get(args: Vec<&String>) -> ExitCode {
    let site_optional = args.iter()
    .skip(1)
    .filter(|a| a.chars().next().unwrap() != '-')
    .next();

    if site_optional.is_none() {
        println!("❗ The get command requires a site name");
        return ExitCode::FAILURE;
    }

    let site = site_optional
        .unwrap()
        .to_lowercase();

    let account_optional = find_by_site(&site);

    if account_optional.is_none() {
        println!("❗️ No account found for {}", site);
        return ExitCode::FAILURE;
    }

    let account = account_optional.unwrap();

    let options = args.iter()
        .skip(1)
        .filter(|a| a.chars().next().unwrap() == '-')
        .next();

    if options.is_none() {
        print_account(&account);
        return ExitCode::SUCCESS;
    }

    match options.unwrap().as_str() {
        PASSWORD_OPTION => println!("🔑 {}", account.password),
        _ => {
            println!(" ❗ Invalid option");
            return ExitCode::from(101);
        },
    }

    ExitCode::SUCCESS
}