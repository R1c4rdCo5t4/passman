use std::io;
use std::io::Write;
use std::process::Command;
use arboard::Clipboard;
use colored::Colorize;
use rpassword::read_password;

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

pub fn print_prefix(vault: Option<&str>) {
    match vault {
        Some(vault) => print!("{}", format!("{}@passman $ ", vault).bright_cyan()),
        None => print!("{}", "passman $ ".bright_cyan()),
    }
    io::stdout().flush().expect("Failed to flush stdout");
}

pub fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear console");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear console");
    }
}

pub fn copy_to_clipboard(text: String) {
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
    clipboard.set_text(text).expect("Failed to copy to clipboard");
}

pub fn clear_clipboard() {
    copy_to_clipboard(String::new());
}
