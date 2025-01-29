use std::fs;
use arboard::Clipboard;
use secrecy::{ExposeSecret, SecretBox};
use crate::cli::commands::models::{Command, VaultCommand, VaultField};
use crate::cli::stdin::{read_line_hidden_with, read_line_with};
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
            let password = read_line_hidden_with("Choose master password for vault: ");
            let confirm_password = read_line_hidden_with("Confirm master password: ");
            if password != confirm_password {
                return Err("Passwords don't match");
            }
            let secret = SecretBox::new(Box::from(String::from(password)));
            create_vault(&name, &secret);
        }
        VaultCommand::Open(name) => {
            vault_exists(&name)?;
            let password = read_line_hidden_with("Enter master password for vault: ");
            let secret = SecretBox::new(Box::from(String::from(password)));
            open_vault(&name, &secret, state);
        }
        VaultCommand::Close => close_vault(state),
        VaultCommand::List => list_vaults(),
        VaultCommand::Show(service, expose) => {
            in_vault(state)?;
            show_vault(service, expose, state)
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
        VaultCommand::Copy(service, field) => {
            in_vault(state)?;
            let entry_opt = state.session.as_mut().unwrap().vault.entries.iter().find(|entry| entry.service == service);
            if entry_opt.is_none() {
                return Err("Service not found");
            }
            let entry = entry_opt.unwrap();
            let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
            let text = match field {
                VaultField::Username => entry.username.clone(),
                VaultField::Password => entry.password.expose_secret().clone(),
            };
            clipboard.set_text(text).expect("Failed to copy to clipboard");
            return Ok(Some(format!("Copied {} to clipboard", field.to_string().to_lowercase())))
        }
        VaultCommand::Panic => {
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
