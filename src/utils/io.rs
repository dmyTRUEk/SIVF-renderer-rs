//! This file contains Input/Output Utils

use std::io::{stdin, stdout, Write};



pub fn read_user_input() -> String {
    let mut user_input: String = String::new();
    stdin().read_line(&mut user_input).unwrap();
    user_input
}



// #[allow(dead_code)]
pub fn wait_for_enter() {
    let mut line: String = String::new();
    stdin().read_line(&mut line).unwrap();
}



pub fn flush() {
    stdout().flush().unwrap();
}

