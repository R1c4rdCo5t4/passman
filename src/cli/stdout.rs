use std::io;
use std::io::Write;
use std::process::Command;
use colored::Colorize;

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