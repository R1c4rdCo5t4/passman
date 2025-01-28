use std::fs;
use crate::cli::commands::enums::{Command, Vault};
use crate::cli::stdout::clear_console;

pub fn execute_cmd(cmd: Command) {
    match cmd {
        Command::Exit => exit(),
        Command::Help(cmd) => help(cmd),
        Command::Clear => clear(),
        Command::Analyze(pwd) => analyze_pwd(pwd),
        Command::Generate => generate_pwd(),
        Command::Vault(cmd) => vault_cmd(cmd),
    }
}

fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}

fn clear() {
    clear_console();
}

fn help(cmd: Option<String>) {
    let help_text = fs::read_to_string("HELP.txt")
        .expect("Failed to read help file");

    match cmd {
        Some(command) => {
            let lines: Vec<&str> = help_text.lines()
                .filter(|line| {
                    if let Some((cmd, _desc)) = line.split_once('>') {
                        cmd.trim().to_lowercase().contains(&command.to_lowercase())
                    } else {
                        false
                    }
                })
                .collect();

            if lines.is_empty() {
                println!("No help available for command: {}", command);
            } else {
                println!("{}", lines.join("\n"));
            }
        }
        None => {
            println!("{}", help_text);
        }
    }
}


fn vault_cmd(command: Vault) {
    println!("Executing vault command: {:?}", command);
    todo!()
}

fn generate_pwd() {
    println!("Generating password");
    todo!()
}

fn analyze_pwd(password: String) {
    println!("Analyzing password: {}", password);
    todo!()
}
