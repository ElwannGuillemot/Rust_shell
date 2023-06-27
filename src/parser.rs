//
// EPITECH PROJECT, 2023
// my_sh
// File description:
// parser
//

pub struct ParsedCommand {
    pub keyword: String,
    pub options: Vec<String>,
    pub arguments: Vec<String>,
}

pub fn parse_command(command: &str) -> Result<ParsedCommand, String> {
    let mut parts = command.split_whitespace();
    let keyword = parts.next().ok_or_else(|| String::from("Command not found."))?;
    let mut options: Vec<String> = Vec::new();
    let mut arguments: Vec<String> = Vec::new();
    for part in parts {
        if part.starts_with('-') {
            options.push(part.to_string());
        } else {
            arguments.push(part.to_string());
        }
    }
    let parsed_command = ParsedCommand {
        keyword: keyword.to_string(),
        options,
        arguments,
    };
    Ok(parsed_command)
}
