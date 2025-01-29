use std::io;
use std::io::Write;
use std::process::Command;
use arboard::Clipboard;
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

pub fn copy_to_clipboard(text: String) {
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
    clipboard.set_text(text).expect("Failed to copy to clipboard");
}

pub fn clear_clipboard() {
    copy_to_clipboard(String::new());
}
