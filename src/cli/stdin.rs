use std::io;
use std::io::Write;
use rpassword::read_password;
use crate::cli::stdout::print_prefix;

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read cli");
    input.trim().to_string()
}

pub fn read_line_hidden() -> String {
    read_password().expect("Failed to read password")
}

pub fn read_line_with(content: &str) -> String {
    print!("{}", content);
    io::stdout().flush().expect("Failed to flush stdout");
    read_line()
}

pub fn read_line_with_prefix(vault: Option<&str>) -> String {
    loop {
        print_prefix(vault);
        let line = read_line();
        if line.len() > 0 {
            return line;
        }
    }
}

pub fn read_line_hidden_with(content: &str) -> String {
    print!("{}", content);
    io::stdout().flush().expect("Failed to flush stdout");
    read_line_hidden()
}
