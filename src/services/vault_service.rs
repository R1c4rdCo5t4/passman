use chrono::{Utc};
use secrecy::{ExposeSecret, SecretBox};
use zeroize::Zeroize;
use crate::domain::app::error::AppError;
use crate::domain::app::session::AppSession;
use crate::domain::app::state::AppState;
use crate::domain::cli::field::Field;
use crate::utils::constants::SESSION_TTL;
use crate::domain::vault::password_entry::{PasswordEntry, PasswordEntryDebug};
use crate::domain::vault::vault::Vault;
use crate::repository::vault::vault_manager_trait::VaultManagerTrait;

pub struct VaultService<V: VaultManagerTrait> {
    vault: V,
}

impl<V: VaultManagerTrait> VaultService<V> {

    pub fn new(vault: V) -> Self {
        Self { vault }
    }

    pub fn create(&self, name: &str, secret: &SecretBox<String>) {
        self.vault.create(name, secret).expect("Failed to create vault");
    }

    pub fn open(&self, name: &str, secret: &SecretBox<String>, state: &mut AppState) -> Result<(), AppError>{
        let result = self.vault.load(name, secret);
        match result {
            Ok(vault) => {
                state.session = Some(
                    AppSession {
                        vault,
                        name: String::from(name),
                        secret: SecretBox::new(Box::from(secret.expose_secret().clone())),
                        expires_at: Utc::now() + SESSION_TTL
                    }
                );
                Ok(())
            }
            Err(e) => Err(AppError::Other(format!("Failed to load vault: {}", e)))
        }
    }

    pub fn close(&self, state: &mut AppState) {
        let session = state.session.take();
        if let Some(mut session) = session {
            session.vault.zeroize();
        }
    }

    pub fn show(&self, entry: Option<String>, expose: bool, state: &mut AppState) -> Result<String, AppError> {
        let entries = state.session.as_mut().unwrap().vault.entries.iter();
        let filtered: Vec<&PasswordEntry> = match entry.clone() {
            Some(s) => entries.filter(|entry| entry.name == s).collect(),
            None => entries.collect(),
        };
        if entry.is_some() && filtered.is_empty() {
            return Err(AppError::Other("Service not found".to_string()));
        }
        Ok(
            filtered.iter()
                .map(|entry| PasswordEntryDebug { entry, expose }.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }

    pub fn delete(&self, state: &mut AppState) {
        let name = state.session.as_mut().unwrap().name.clone();
        self.vault.delete(&name).expect("Failed to delete vault");
    }

    pub fn list(&self) -> String {
        let vaults = self.vault.list().expect("Failed to list vaults");
        vaults.join("\n")
    }

    pub fn add_entry(&self, entry: &str, username: &str, password: &str, state: &mut AppState) {
        let session = state.session.as_mut().unwrap();
        let new_entry = PasswordEntry {
            name: String::from(entry),
            username: String::from(username),
            password: SecretBox::new(Box::from(String::from(password))),
        };
        session.vault.entries.append(&mut vec![new_entry]);
        self.vault.save(&session.name, &session.secret, &session.vault).expect("Failed to save vault");
    }

    pub fn update_entry(&self, entry: &str, field: &Field, value: &str, state: &mut AppState) -> Result<(), AppError> {
        let session = state.session.as_mut().unwrap();
        let entry = Self::get_vault_entry(entry, &mut session.vault)?;
        match field {
            Field::Username => entry.username = String::from(value),
            Field::Password => entry.password = SecretBox::new(Box::from(String::from(value))),
        }
        self.vault.save(&session.name, &session.secret, &session.vault).expect("Failed to save vault");
        Ok(())
    }

    pub fn delete_entry(&self, entry: &str, state: &mut AppState) -> Result<(), AppError> {
        let session = state.session.as_mut().unwrap();
        let entry = Self::get_vault_entry(entry, &mut session.vault)?.name.clone();
        session.vault.entries.retain(|e| e.name != entry);
        Ok(())
    }

    pub fn is_accessible(&self, state: &mut AppState) -> Result<(), AppError> {
        if let Some(session) = &state.session {
            // check session
            let now = Utc::now();
            if session.expires_at < now {
                // session expired
                self.close(state);
                Err(AppError::Other("Session expired".to_string()))
            } else {
                // extend session
                state.session.as_mut().unwrap().expires_at = now + SESSION_TTL;
                Ok(())
            }
        } else {
            Err(AppError::Other("No vault opened".to_string()))
        }
    }

    pub fn exists(&self, name: &str) -> Result<(), AppError> {
        if self.vault.exists(name).unwrap() {
            Ok(())
        } else {
            Err(AppError::Other("Vault not found".to_string()))
        }
    }

    fn get_vault_entry<'a>(entry: &str, vault: &'a mut Vault) -> Result<&'a mut PasswordEntry, AppError> {
        vault.entries
            .iter_mut()
            .find(|e| e.name == entry)
            .ok_or(AppError::Other("Service not found".to_string()))
    }
}