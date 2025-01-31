use std::{fs, thread};
use secrecy::{ExposeSecret, SecretBox};
use crate::cli::io::{read_line_hidden_with, read_line_with, clear_clipboard, clear_console, copy_to_clipboard, confirmation_prompt, confirmation_prompt_with};
use crate::domain::app::error::AppError;
use crate::domain::app::state::AppState;
use crate::domain::cli::commands::{Command, VaultCommand};
use crate::domain::cli::field::Field;
use crate::domain::cli::password_params::PasswordParams;
use crate::repository::vault::vault_manager::VaultManager;
use crate::services::vault_service::VaultService;
use crate::utils::constants::CLIPBOARD_TTL;
use crate::utils::validation::{validate_arg, validate_password, validate_password_strength};

const HELP_FILE_PATH: &str = "HELP.txt";
type CommandResult = Result<Option<String>, AppError>;

pub fn execute_cmd(
    cmd: Command,
    vault_service: &VaultService<VaultManager>,
    state: &mut AppState
) -> CommandResult {
    match cmd {
        Command::Exit => exit(),
        Command::Help(cmd) => help(cmd),
        Command::Clear => clear(),
        Command::Analyze(pwd) => analyze_password(pwd),
        Command::Generate(params, copy) => generate_password(params, copy),
        Command::Vault(cmd) => vault_cmd(cmd, vault_service, state),
        Command::Panic => panic(vault_service, state),
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

fn panic(vault: &VaultService<VaultManager>, state: &mut AppState) -> CommandResult {
    if state.session.is_some() { // if in vault
        vault.close(state);
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
                Err(AppError::Other("No help available for provided command".to_string()))
            } else {
                Ok(Some(lines.join("\n")))
            }
        }
        None => {
            Ok(Some(help_text))
        }
    }
}

fn vault_cmd(
    command: VaultCommand,
    vault: &VaultService<VaultManager>,
    state: &mut AppState
) -> CommandResult {
    match command {
        VaultCommand::New(name) => {
            match vault.exists(&name) {
                Ok(_) => return Err(AppError::Other("Vault already exists".to_string())),
                Err(_) => {}
            }
            let password = read_line_hidden_with("Choose master password for vault: ");
            validate_arg(&password, "password")?;
            let confirm_password = read_line_hidden_with("Confirm master password: ");
            validate_arg(&confirm_password, "confirm-password")?;
            if password != confirm_password {
                return Err(AppError::Other("Passwords don't match".to_string()));
            }
            validate_password_strength(&password)?;
            let secret = SecretBox::new(Box::from(String::from(password)));
            vault.create(&name, &secret);
        }
        VaultCommand::Open(name) => {
            vault.exists(&name)?;
            let password = read_line_hidden_with("Enter master password for vault: ");
            validate_password(&password)?;
            let secret = SecretBox::new(Box::from(String::from(password)));
            vault.open(&name, &secret, state);
        }
        VaultCommand::Close => vault.close(state),
        VaultCommand::List => vault.list(),
        VaultCommand::Show(service, expose) => {
            vault.is_accessible(state)?;
            if service.is_none() && expose {
                if !confirmation_prompt_with("This will expose all credentials in the vault. Do you want to continue?")? {
                    return Ok(None);
                }
            }
            vault.show(service, expose, state)?
        }
        VaultCommand::Add(service) => {
            vault.is_accessible(state)?;
            let duplicate_entry = state.session.as_mut().unwrap().vault.entries.iter().find(|entry| entry.service == service);
            if duplicate_entry.is_some() {
                if !confirmation_prompt_with("Service already exists. Do you want to update it?")? {
                    return Ok(None);
                } else {
                    vault.delete_entry(&service, state)?;
                }
            }
            let username = read_line_with("Username: ");
            validate_arg(&username, "username")?;
            let password = read_line_hidden_with("Password: ");
            validate_password(&password)?;
            vault.add_entry(&service, &username, &password, state);
        }
        VaultCommand::Update(service, field, value) => {
            vault.is_accessible(state)?;
            if confirmation_prompt()? {
                vault.update_entry(&service, &field, &value, state)?;
            }
        }
        VaultCommand::Delete(service) => {
            vault.is_accessible(state)?;
            if confirmation_prompt()? {
                vault.delete_entry(&service, state)?;
            }
        }
        VaultCommand::Copy(service, field) => {
            vault.is_accessible(state)?;
            let entry_opt = state.session.as_mut().unwrap().vault.entries.iter().find(|entry| entry.service == service);
            if entry_opt.is_none() {
                return Err(AppError::Other("Service not found".to_string()));
            }
            let entry = entry_opt.unwrap();
            let text = match field {
                Field::Username => entry.username.clone(),
                Field::Password => entry.password.expose_secret().clone(),
            };
            copy_to_clipboard(text);

            // launch auto-clear clipboard thread
            thread::spawn(move || {
                thread::sleep(CLIPBOARD_TTL.to_std().unwrap());
                clear_clipboard();
            });

            return Ok(Some(format!("Copied {} to clipboard", field.to_string().to_lowercase())))
        }
        VaultCommand::Destroy => {
            vault.is_accessible(state)?;
            if confirmation_prompt()? {
                vault.delete(state);
                vault.close(state);
            }
        }
    }
    Ok(None)
}