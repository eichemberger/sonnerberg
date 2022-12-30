use std::env; 

use sonnerberg::commands::execute_command; 
use sonnerberg::{run_app};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        run_app();
    } else {
        execute_command(); 
    }

}
