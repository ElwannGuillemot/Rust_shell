//
// EPITECH PROJECT, 2023
// my_sh
// File description:
// input
//

use std::io::{self, BufRead};

pub fn read_command() -> String {
    let stdin: io::Stdin = io::stdin();
    let mut buffer: String = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
