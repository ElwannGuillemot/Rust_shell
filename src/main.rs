//
// EPITECH PROJECT, 2023
// my_sh
// File description:
// main
//

mod prompt;
mod input;
mod parser;
mod exec;
mod signals;
mod builtins {
    pub mod cd;
    pub mod filesystem;
}

use prompt::display_prompt;
use input::read_command;
use parser::parse_command;
use exec::execute_command;
use std::io::{self, Write};

fn main() {
    loop {
        display_prompt();
        io::stdout().flush().unwrap();
        let command = read_command();
        let parsed_command = parse_command(&command);
        if let Ok(parsed_command) = parsed_command {
            if let Err(err) = execute_command(parsed_command) {
                println!("{}", err);
            }
        }
        // signals::handle_signals();
    }
}
