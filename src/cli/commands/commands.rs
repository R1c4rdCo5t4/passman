use std::fs;
use secrecy::SecretBox;
use crate::cli::commands::enums::{Command, Vault};
use crate::cli::stdin::{read_line_with};
use crate::cli::stdout::clear_console;
use crate::services::operations::vault::{add_to_vault, close_vault, create_vault, open_vault, show_vault};
use crate::state::{AppState};

type Result = std::result::Result<(), &'static str>;

pub fn execute_cmd(cmd: Command, state: &mut AppState) -> Result {
    match cmd {
        Command::Exit => exit(),
        Command::Help(cmd) => help(cmd),
        Command::Clear => clear(),
        Command::Analyze(pwd) => analyze_pwd(pwd),
        Command::Generate => generate_pwd(),
        Command::Vault(cmd) => vault_cmd(cmd, state),
    }
}

fn exit() -> Result {
    println!("Exiting...");
    std::process::exit(0);
}

fn clear() -> Result {
    clear_console();
    Ok(())
}

fn help(cmd: Option<String>) -> Result {
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
                return Err("No help available for provided command");
            } else {
                println!("{}", lines.join("\n"));
            }
        }
        None => {
            println!("{}", help_text);
        }
    }
    Ok(())
}


fn vault_cmd(command: Vault, state: &mut AppState) -> Result {
    match command {
        Vault::New(name) => {
            let password = read_line_with("Choose master password for vault: ");
            let confirm_password = read_line_with("Confirm master password: ");
            if password != confirm_password {
                return Err("Passwords don't match");
            }
            let secret = SecretBox::new(Box::from(String::from(password)));
            create_vault(&name, &secret);
        }
        Vault::Open(name) => {
            let password = read_line_with("Enter master password for vault: ");
            let secret = SecretBox::new(Box::from(String::from(password)));
            open_vault(&name, &secret, state);
        }
        Vault::Close => {
            close_vault(state)
        }
        Vault::List => {}
        Vault::Show(_, _) => {
            if state.session.is_none() {
                return Err("No vault opened");
            }
            show_vault(state)
        }
        Vault::Add(service, username, password) => {
            if state.session.is_none() {
                return Err("No vault opened");
            }
            add_to_vault(&service, &username, &password, state);
        }
        Vault::Update(_, _, _) => {
            if state.session.is_none() {
                return Err("No vault opened");
            }
        }
        Vault::Delete(_) => {
            if state.session.is_none() {
                return Err("No vault opened");
            }
        }
        Vault::Copy(_) => {
            if state.session.is_none() {
                return Err("No vault opened");
            }
        }
        Vault::Search(_) => {
            if state.session.is_none() {
                return Err("No vault opened");
            }
        }
        Vault::Destroy => {
            if state.session.is_none() {
                return Err("No vault opened");
            }
        }
    }
    Ok(())
}

fn generate_pwd() -> Result {
    todo!()
}

fn analyze_pwd(_: String) -> Result {
    todo!()
}