use std::fs;
use secrecy::SecretBox;
use crate::cli::commands::models::{Command, VaultCommand};
use crate::cli::stdin::{read_line_with};
use crate::cli::stdout::clear_console;
use crate::services::vault::operations::{add_to_vault, close_vault, create_vault, in_vault, list_vaults, open_vault, show_vault, vault_exists};
use crate::state::{AppState};

const HELP_FILE_PATH: &str = "HELP.txt";
type Result = std::result::Result<Option<String>, &'static str>;

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
    std::process::exit(0);
}

fn clear() -> Result {
    clear_console();
    Ok(None)
}

fn help(cmd: Option<String>) -> Result {
    let help_text = fs::read_to_string(HELP_FILE_PATH)
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
                Err("No help available for provided command")
            } else {
                Ok(Some(lines.join("\n")))
            }
        }
        None => {
            Ok(Some(help_text))
        }
    }
}


fn vault_cmd(command: VaultCommand, state: &mut AppState) -> Result {
    match command {
        VaultCommand::New(name) => {
            let password = read_line_with("Choose master password for vault: ");
            let confirm_password = read_line_with("Confirm master password: ");
            if password != confirm_password {
                return Err("Passwords don't match");
            }
            let secret = SecretBox::new(Box::from(String::from(password)));
            create_vault(&name, &secret);
        }
        VaultCommand::Open(name) => {
            vault_exists(&name)?;
            let password = read_line_with("Enter master password for vault: ");
            let secret = SecretBox::new(Box::from(String::from(password)));
            open_vault(&name, &secret, state);
        }
        VaultCommand::Close => close_vault(state),
        VaultCommand::List => list_vaults(),
        VaultCommand::Show(_, unmask) => {
            in_vault(state)?;
            show_vault(unmask, state)
        }
        VaultCommand::Add(service, username, password) => {
            in_vault(state)?;
            add_to_vault(&service, &username, &password, state);
        }
        VaultCommand::Update(_, _, _) => {
            in_vault(state)?;
        }
        VaultCommand::Delete(_) => {
            in_vault(state)?;
        }
        VaultCommand::Copy(_) => {
            // if vec!["username", ""]
            in_vault(state)?;

        }
        VaultCommand::Search(_) => {
            in_vault(state)?;
        }
        VaultCommand::Destroy => {
            in_vault(state)?;
        }
    }
    Ok(None)
}

fn generate_pwd() -> Result {
    todo!()
}

fn analyze_pwd(_: String) -> Result {
    todo!()
}

// pub fn show_usage(cmd: &str) {
//     let help_line = read_help_file(Some(cmd.to_string())).unwrap();
//     let usage = help_line.split(">").collect::<Vec<&str>>()[1].trim().to_string();
//     println!("Usage: {}", usage)
// }