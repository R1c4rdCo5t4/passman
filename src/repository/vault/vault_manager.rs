use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use secrecy::SecretBox;
use crate::domain::vault::vault::Vault;
use crate::domain::vault::vault_file::VaultFile;
use crate::repository::vault::vault_crypto::VaultCrypto;
use crate::repository::vault::vault_manager_trait::VaultManagerTrait;

pub struct VaultManager;

impl VaultManager {
    fn get_path(name: Option<&str>) -> PathBuf {
        let dirs = ProjectDirs::from("com", "passman", "Passman")
            .expect("Failed to find project directory");
        let mut path = dirs.data_dir().to_path_buf();
        if let Some(name) = name {
            path.push(format!("{}.vault", name));
        }
        path
    }
}

impl VaultManagerTrait for VaultManager {

    fn create(&self, name: &str, password: &SecretBox<String>) -> Result<(), String> {
        let vault = Vault { entries: Vec::new() };
        self.save(name, password, &vault)
    }

    fn save(&self, name: &str, password: &SecretBox<String>, vault: &Vault) -> Result<(), String> {
        let (salt, nonce, ciphertext) = VaultCrypto::encrypt(vault, password);
        let vault_file = VaultFile { salt, nonce, ciphertext };
        let data = serde_json::to_vec(&vault_file).map_err(|e| format!("Serialization failed: {}", e))?;
        let path = Self::get_path(Option::from(name));
        fs::create_dir_all(path.parent().unwrap()).map_err(|e| format!("Directory creation failed: {}", e))?;
        fs::write(path, data).map_err(|e| format!("Failed to write vault file: {}", e))
    }

    fn load(&self, name: &str, password: &SecretBox<String>) -> Result<Vault, String> {
        let path = Self::get_path(Option::from(name));
        let data = fs::read(path).expect("Failed to read vault file");
        let vault_file: VaultFile = serde_json::from_slice(&data).expect("Deserialization failed");
        let VaultFile { ciphertext, salt, nonce } = vault_file;
        VaultCrypto::decrypt(password, &ciphertext, &salt, &nonce)
    }

    fn list(&self) -> Result<Vec<String>, String> {
        let path = Self::get_path(None);
        let files = fs::read_dir(path)
            .map_err(|e| format!("Failed to read vault directory: {}", e))?;
        files
            .map(|entry| {
                entry
                    .map_err(|e| format!("Failed to process entry: {}", e))
                    .map(|file| file.file_name().to_string_lossy().to_string())
            })
            .collect()
    }

    fn delete(&self, name: &str) -> Result<(), String> {
        let path = Self::get_path(Option::from(name));
        match fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to delete vault".into())
        }
    }

    fn exists(&self, name: &str) -> Result<bool, String> {
        let path = Self::get_path(Option::from(name));
        Ok(path.exists())
    }
}
