use std::path::PathBuf;
use directories::ProjectDirs;
use std::fs::{self};
use secrecy::SecretBox;
use crate::services::vault::models::vault::Vault;
use crate::services::vault::models::vault_file::VaultFile;
use super::crypto::{self};

pub struct VaultManager;

impl VaultManager {

    pub fn create(name: &str, password: &SecretBox<String>) -> Result<(), String> {
        let vault = Vault { entries: Vec::new() };
        Self::save(&name, &password, &vault)
    }

    pub fn save(name: &str, password: &SecretBox<String>, vault: &Vault) -> Result<(), String> {
        let (salt, nonce, ciphertext) = crypto::encrypt_vault(&vault, &password);
        let vault_file = VaultFile { salt, nonce, ciphertext };
        let data = serde_json::to_vec(&vault_file).map_err(|e| format!("Serialization failed: {}", e))?;
        let path = Self::get_path(Option::from(name));
        fs::create_dir_all(path.parent().unwrap()).map_err(|e| format!("Directory creation failed: {}", e))?;
        fs::write(path, data).map_err(|e| format!("Failed to write vault file: {}", e))
    }

    pub fn load(name: &str, password: &SecretBox<String>) -> Result<Vault, String> {
        let path = Self::get_path(Option::from(name));
        let data = fs::read(path).expect("Failed to read vault file");
        let vault_file: VaultFile = serde_json::from_slice(&data).expect("Deserialization failed");
        let VaultFile { ciphertext, salt, nonce } = vault_file;
        crypto::decrypt_vault(&password, &ciphertext, &salt, &nonce)
    }

    pub fn list() -> Result<Vec<String>, String> {
        let path = Self::get_path(None);
        let files = fs::read_dir(&path)
            .map_err(|e| format!("Failed to read vault directory: {}", e))?;
        files
            .map(|entry| {
                entry
                    .map_err(|e| format!("Failed to process entry: {}", e))
                    .map(|file| file.file_name().to_string_lossy().to_string())
            })
            .collect()
    }

    pub fn delete(name: &str) {
        let path = Self::get_path(Option::from(name));
        fs::remove_file(path).expect("Failed to delete vault file");
    }

    pub fn exists(name: &str) -> bool {
        let path = Self::get_path(Option::from(name));
        path.exists()
    }

    fn get_path(name: Option<&str>) -> PathBuf {
        let dirs = ProjectDirs::from("com", "passman", "Passman")
            .expect("Failed to find project directory");
        let mut path = dirs.data_dir().to_path_buf();
        if name.is_some(){
            path.push(format!("{}.vault", name.unwrap()));
        }
        path
    }
}