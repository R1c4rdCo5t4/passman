use std::collections::HashMap;
use std::sync::{Mutex};
use lazy_static::lazy_static;
use secrecy::{ExposeSecret, SecretBox};
use passman::domain::vault::vault::Vault;
use passman::domain::vault::vault_file::VaultFile;
use passman::repository::vault::vault_crypto::VaultCrypto;
use passman::repository::vault::vault_manager_trait::VaultManagerTrait;

#[cfg(test)]
lazy_static! {
    static ref MOCK_STORAGE: Mutex<HashMap<String, (VaultFile, SecretBox<String>)>> = Mutex::new(HashMap::new());
}

#[cfg(test)]
pub struct MockVaultManager;

#[cfg(test)]
impl VaultManagerTrait for MockVaultManager {
    fn create(&self, name: &str, password: &SecretBox<String>) -> Result<(), String> {
        let vault = Vault { entries: vec![] };
        self.save(name, password, &vault)
    }

    fn save(&self, name: &str, password: &SecretBox<String>, vault: &Vault) -> Result<(), String> {
        let (salt, nonce, ciphertext) = VaultCrypto::encrypt(vault, password);
        let mut vaults = MOCK_STORAGE.lock().unwrap();
        let secret = SecretBox::new(Box::new(password.expose_secret().to_string()));
        vaults.insert(
            name.to_string(),
            (VaultFile { salt, nonce, ciphertext }, secret)
        );
        Ok(())
    }

    fn load(&self, name: &str, password: &SecretBox<String>) -> Result<Vault, String> {
        let vaults = MOCK_STORAGE.lock().unwrap();
        let (vault_file, stored_pass) = vaults.get(name)
            .ok_or("Vault not found")?;

        if password.expose_secret() != stored_pass.expose_secret() {
            return Err("Invalid password".into());
        }

        VaultCrypto::decrypt(
            password,
            &vault_file.ciphertext,
            &vault_file.salt,
            &vault_file.nonce
        )
    }

    fn list(&self) -> Result<Vec<String>, String> {
        let vaults = MOCK_STORAGE.lock().unwrap();
        Ok(vaults.keys().cloned().collect())
    }

    fn delete(&self, name: &str) -> Result<(), String> {
        let mut vaults = MOCK_STORAGE.lock().unwrap();
        vaults.remove(name);
        Ok(())
    }

    fn exists(&self, name: &str) -> Result<bool, String> {
        let vaults = MOCK_STORAGE.lock().unwrap();
        Ok(vaults.contains_key(name))
    }
}