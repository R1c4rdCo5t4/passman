use std::fs;
use secrecy::SecretBox;
use crate::cli::commands::enums::{Command, Vault};
use crate::cli::stdin::{read_line_with};
use crate::cli::stdout::clear_console;
use crate::services::vault::vault::{Vault, VaultManager};

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
    match command {
        Vault::New(name) => {
            loop {
                let password = read_line_with("Choose master password for vault: ");
                let confirm_password = read_line_with("Confirm master password: ");
                if password == confirm_password {
                    let secret = SecretBox::new(Box::from(String::from(password)));
                    VaultManager::create(&*name, &secret).expect("Failed to create vault");
                    break;
                }
                println!("Passwords do not match, please try again.");
            }
        }
        Vault::Open(name) => {
            let password = read_line_with("Enter master password for vault: ");
            let secret = SecretBox::new(Box::from(String::from(password)));
            let result = VaultManager::load(&name, &secret);
            match result {
                Ok(vault) => {
                    println!("{:?}", &vault.entries);

                }
                Err(e) => {
                    println!("Failed to load vault: {}", e);
                }
            }
        }
        Vault::Close => {}
        Vault::List => {}
        Vault::Show(_, _) => {}
        Vault::Add(_, _, _) => {}
        Vault::Update(_, _, _) => {}
        Vault::Delete(_) => {}
        Vault::Copy(_) => {}
        Vault::Search(_) => {}
        Vault::Destroy => {}
    }
}

fn generate_pwd() {
    println!("Generating password");
    todo!()
}

fn analyze_pwd(password: String) {
    println!("Analyzing password: {}", password);
    todo!()
}
