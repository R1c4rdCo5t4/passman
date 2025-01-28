use chrono::{Duration, Utc};
use secrecy::{ExposeSecret, SecretBox};
use zeroize::Zeroize;
use crate::services::vault::constants::SESSION_TTL;
use crate::services::vault::models::PasswordEntry;
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

pub fn show_vault(state: &mut AppState) {
    for entry in state.session.as_mut().unwrap().vault.entries.iter() {
        println!("{:?}", entry);
    }
}