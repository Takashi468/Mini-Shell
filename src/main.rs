mod check_os;
mod windows;
mod print_input;

use windows::commands_base::commands_base;
use std::env;
use std::io::Write;

fn main() {
    let os = check_os::detect_os();
    println!("OS: {}", os);
    // windows::commands_base::commands_base().expect("Failed to execute commards_base");
    loop {
        let mut input = String::new();
        print!("Enter Commands base:");
        std::io::stdout().flush().expect("Failed to flush stdout");

        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        if !input.trim().is_empty() {
            commands_base(&input).expect("Failed to execute commards_base");
        } else {
            println!("No command provided");
            continue;
        }
    }
}
