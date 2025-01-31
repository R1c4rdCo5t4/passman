use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use secrecy::{ExposeSecret, SecretBox};
use passman::domain::vault::vault::Vault;
use passman::domain::vault::vault_file::VaultFile;
use passman::repository::vault::vault_crypto::VaultCrypto;
use passman::repository::vault::vault_manager_trait::VaultManagerTrait;

#[cfg(test)]
pub struct MockVaultManager {
    storage: Arc<Mutex<HashMap<String, (VaultFile, SecretBox<String>)>>>
}

#[cfg(test)]
#[cfg(test)]
impl MockVaultManager {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[cfg(test)]
impl VaultManagerTrait for MockVaultManager {
    fn create(&self, name: &str, password: &SecretBox<String>) -> Result<(), String> {
        let vault = Vault { entries: vec![] };
        self.save(name, password, &vault)
    }

    fn save(&self, name: &str, password: &SecretBox<String>, vault: &Vault) -> Result<(), String> {
        let (salt, nonce, ciphertext) = VaultCrypto::encrypt(vault, password);
        let mut vaults = self.storage.lock().unwrap();
        let secret = SecretBox::new(Box::new(password.expose_secret().to_string()));
        vaults.insert(
            name.to_string(),
            (VaultFile { salt, nonce, ciphertext }, secret)
        );
        Ok(())
    }

    fn load(&self, name: &str, password: &SecretBox<String>) -> Result<Vault, String> {
        let vaults = self.storage.lock().unwrap();
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
        let vaults = self.storage.lock().unwrap();
        Ok(vaults.keys().cloned().collect())
    }

    fn delete(&self, name: &str) -> Result<(), String> {
        let mut vaults = self.storage.lock().unwrap();
        vaults.remove(name);
        Ok(())
    }

    fn exists(&self, name: &str) -> Result<bool, String> {
        let vaults = self.storage.lock().unwrap();
        Ok(vaults.contains_key(name))
    }
}