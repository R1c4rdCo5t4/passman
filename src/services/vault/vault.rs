use std::path::PathBuf;
use directories::ProjectDirs;
use std::fs::{self};
use std::io::prelude::*;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use secrecy::SecretBox;
use super::models::{Vault, VaultFile};
use super::crypto::{self, derive_key};

pub struct VaultManager;

impl VaultManager {
    pub fn get_path(name: &str) -> PathBuf {
        let dirs = ProjectDirs::from("com", "passman", "Passman")
            .expect("Failed to find project directory");
        let mut path = dirs.data_dir().to_path_buf();
        path.push(format!("{}.vault", name));
        path
    }

    pub fn create(name: &str, password: &SecretBox<String>) -> Result<(), String> {
        let vault = Vault { entries: Vec::new() };
        Self::save(&name, &password, &vault)
    }

    pub fn save(name: &str, password: &SecretBox<String>, vault: &Vault) -> Result<(), String> {
        let (salt, nonce, ciphertext) = crypto::encrypt_vault(&vault, &password);

        let vault_file = VaultFile { salt, nonce, ciphertext };
        let data = serde_json::to_vec(&vault_file)
            .map_err(|e| format!("Serialization failed: {}", e))?;

        let path = Self::get_path(name);
        fs::create_dir_all(path.parent().unwrap())
            .map_err(|e| format!("Directory creation failed: {}", e))?;

        fs::write(path, data)
            .map_err(|e| format!("File write failed: {}", e))
    }

    pub fn load(name: &str, password: &SecretBox<String>) -> Result<Vault, String> {
        let path = Self::get_path(name);
        let data = fs::read(path)
            .map_err(|e| format!("File read failed: {}", e))?;

        let vault_file: VaultFile = serde_json::from_slice(&data)
            .map_err(|e| format!("Deserialization failed: {}", e))?;

        let salt_bytes = STANDARD.decode(&vault_file.salt)
            .map_err(|e| format!("Salt decoding failed: {}", e))?;

        let key = derive_key(password, &salt_bytes);
        crypto::decrypt_vault(
            &vault_file.salt,
            &vault_file.nonce,
            &vault_file.ciphertext,
            &key
        )
    }
}