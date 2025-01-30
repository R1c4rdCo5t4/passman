use std::fs;
use secrecy::{ExposeSecret, SecretBox};
use crate::cli::io::{read_line_hidden_with, read_line_with, clear_clipboard, clear_console, copy_to_clipboard};
use crate::domain::app::state::AppState;
use crate::domain::cli::commands::{Command, VaultCommand};
use crate::domain::cli::field::Field;
use crate::domain::cli::password_params::PasswordParams;
use crate::services::vault::operations::{add_to_vault, close_vault, create_vault, delete_from_vault, delete_vault, in_vault, list_vaults, open_vault, show_vault, update_vault, vault_exists};

const HELP_FILE_PATH: &str = "HELP.txt";
type CommandResult = Result<Option<String>, &'static str>;

pub fn execute_cmd(cmd: Command, state: &mut AppState) -> CommandResult {
    match cmd {
        Command::Exit => exit(),
        Command::Help(cmd) => help(cmd),
        Command::Clear => clear(),
        Command::Analyze(pwd) => analyze_password(pwd),
        Command::Generate(params, copy) => generate_password(params, copy),
        Command::Vault(cmd) => vault_cmd(cmd, state),
        Command::Panic => panic(state),
    }
}

fn exit() -> CommandResult {
    clear_console();
    std::process::exit(0);
}

fn clear() -> CommandResult {
    clear_console();
    Ok(None)
}

fn panic(state: &mut AppState) -> CommandResult {
    if state.session.is_some() { // if in vault
        close_vault(state);
    }
    clear_clipboard();
    clear_console();
    exit()
}

fn generate_password(_: PasswordParams, _: bool) -> CommandResult {
    todo!()
}

fn analyze_password(_: String) -> CommandResult {
    todo!()
}


fn help(cmd: Option<String>) -> CommandResult {
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

fn vault_cmd(command: VaultCommand, state: &mut AppState) -> CommandResult {
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
            show_vault(service, expose, state)?
        }
        VaultCommand::Add(service) => {
            in_vault(state)?;
            let username = read_line_with("Username: ");
            let password = read_line_hidden_with("Password: ");
            add_to_vault(&service, &username, &password, state);
        }
        VaultCommand::Update(service, field, value) => {
            in_vault(state)?;
            if confirmation_prompt() {
                update_vault(&service, &field, &value, state)?;
            }
        }
        VaultCommand::Delete(service) => {
            in_vault(state)?;
            if confirmation_prompt() {
                delete_from_vault(&service, state)?;
            }
        }
        VaultCommand::Copy(service, field) => {
            in_vault(state)?;
            let entry_opt = state.session.as_mut().unwrap().vault.entries.iter().find(|entry| entry.service == service);
            if entry_opt.is_none() {
                return Err("Service not found");
            }
            let entry = entry_opt.unwrap();
            let text = match field {
                Field::Username => entry.username.clone(),
                Field::Password => entry.password.expose_secret().clone(),
            };
            copy_to_clipboard(text);
            return Ok(Some(format!("Copied {} to clipboard", field.to_string().to_lowercase())))
        }
        VaultCommand::Destroy => {
            in_vault(state)?;
            if confirmation_prompt() {
                delete_vault(state);
                close_vault(state);
            }
        }
    }
    Ok(None)
}

fn confirmation_prompt() -> bool {
    let input = read_line_with("Are you sure? (y/n): ").to_lowercase();
    input == "y" || input == "yes"
}