use chrono::{Duration, Utc};
use secrecy::{ExposeSecret, SecretBox};
use zeroize::Zeroize;
use crate::cli::commands::models::VaultField;
use crate::services::vault::constants::SESSION_TTL;
use crate::services::vault::models::password_entry::{PasswordEntry, PasswordEntryDebug};
use crate::services::vault::models::vault::Vault;
use crate::services::vault::vault::VaultManager;
use crate::state::{AppState, Session};

pub fn create_vault(name: &str, secret: &SecretBox<String>) {
    VaultManager::create(&*name, &secret).expect("Failed to create vault");
}

pub fn open_vault(name: &str, secret: &SecretBox<String>, state: &mut AppState) {
    let result = VaultManager::load(&name, &secret);
    match result {
        Ok(vault) => {
            state.session = Some(
                Session {
                    vault,
                    name: String::from(name),
                    secret: SecretBox::new(Box::from(secret.expose_secret().clone())),
                    expires: Utc::now() + Duration::minutes(SESSION_TTL)
                }
            );
        }
        Err(e) => {
            println!("Failed to load vault: {}", e);
        }
    }
}

pub fn close_vault(state: &mut AppState) {
    let session = state.session.take();
    if let Some(mut session) = session {
        session.vault.zeroize();
    }
}

pub fn show_vault(service: Option<String>, expose: bool, state: &mut AppState) -> Result<(), &'static str> {
    let entries = state.session.as_mut().unwrap().vault.entries.iter();
    let filtered: Vec<&PasswordEntry> = match service.clone() {
        Some(s) => entries.filter(|entry| entry.service == s).collect(),
        None => entries.collect(),
    };
    if service.is_some() && filtered.is_empty() {
        return Err("Service not found");
    }
    for entry in filtered {
        println!("{:?}", PasswordEntryDebug { entry, expose });
    }
    Ok(())
}

pub fn delete_vault(state: &mut AppState) {
    let name = state.session.as_mut().unwrap().name.clone();
    VaultManager::delete(&name);
}

pub fn list_vaults() {
    let vaults = VaultManager::list().expect("Failed to list vaults");
    for vault in vaults.iter() {
        println!("{}", vault);
    }
}

pub fn add_to_vault(service: &str, username: &str, password: &str, state: &mut AppState) {
    let session = state.session.as_mut().unwrap();
    let new_entry = PasswordEntry {
        service: String::from(service),
        username: String::from(username),
        password: SecretBox::new(Box::from(String::from(password))),
    };
    session.vault.entries.append(&mut vec![new_entry]);
    VaultManager::save(&session.name, &session.secret, &session.vault).expect("Failed to save vault");
}

pub fn update_vault(service: &str, field: &VaultField, value: &str, state: &mut AppState) -> Result<(), &'static str> {
    let session = state.session.as_mut().unwrap();
    let mut entry = get_vault_entry(service, &mut session.vault)?;
    match field {
        VaultField::Username => entry.username = String::from(value),
        VaultField::Password => entry.password = SecretBox::new(Box::from(String::from(value))),
    }
    VaultManager::save(&session.name, &session.secret, &session.vault).expect("Failed to save vault");
    Ok(())
}

pub fn delete_from_vault(service: &str, state: &mut AppState) -> Result<(), &'static str> {
    let session = state.session.as_mut().unwrap();
    let entry = get_vault_entry(service, &mut session.vault)?;
    session.vault.entries.retain(|e| e.service != entry.service);
    Ok(())
}

pub fn in_vault(state: &AppState) -> Result<(), &'static str> {
    if state.session.is_some() {
        Ok(())
    } else {
        Err("No vault opened")
    }
}


pub fn vault_exists(name: &str) -> Result<(), &'static str> {
    if VaultManager::exists(&name) {
        Ok(())
    } else {
        Err("Vault does not exist")
    }
}

fn get_vault_entry(service: &str, vault: &mut Vault) -> Result<PasswordEntry, &'static str> {
    let entry_opt = vault.entries.iter_mut().find(|entry| entry.service == service);
    if entry_opt.is_none() {
        Err("Service not found")
    } else {
        let entry = entry_opt.unwrap();
        Ok(PasswordEntry {
            service: entry.service.clone(),
            username: entry.username.clone(),
            password: SecretBox::new(Box::from(entry.password.expose_secret().clone())),
        })
    }
}