//
// EPITECH PROJECT, 2023
// my_sh
// File description:
// exec
//

use std::process::{Command, exit};
use crate::parser::ParsedCommand;
use crate::builtins::cd::change_directory;

fn check_builtins(parsed_command: &ParsedCommand) -> bool {
    let builtins_commands = vec!["exit", "cd", "env", "setenv", "unsetenv", "mkdir", "rm",
    "tree", "history", "alias"];

    if builtins_commands.contains(&parsed_command.keyword.as_str()) {
        match parsed_command.keyword.as_str() {
            "exit" => {
                exit(0);
            }
            "cd" => {
                let _ = change_directory(&parsed_command.arguments[0]);
                return true;
            }
            _ => {}
        }
    }

    false
}

pub fn execute_command(parsed_command: ParsedCommand) -> Result<(), String> {
    if check_builtins(&parsed_command) {
        return Ok(());
    }
    let mut command = Command::new(&parsed_command.keyword);
    command.args(&parsed_command.arguments);
    for option in &parsed_command.options {
        command.arg(option);
    }
    let mut child_process = command.spawn()
        .map_err(|err| format!("{}: Command not found.", err))?;
    let exit_status = child_process.wait().unwrap();
    if !exit_status.success() {
        exit(exit_status.code().unwrap_or(1));
    }
    Ok(())
}
