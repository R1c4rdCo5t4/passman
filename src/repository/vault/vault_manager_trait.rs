use secrecy::SecretBox;
use crate::domain::vault::vault::Vault;

pub trait VaultManagerTrait {
    fn create(&self, name: &str, password: &SecretBox<String>) -> Result<(), String>;
    fn save(&self, name: &str, password: &SecretBox<String>, vault: &Vault) -> Result<(), String>;
    fn load(&self, name: &str, password: &SecretBox<String>) -> Result<Vault, String>;
    fn list(&self) -> Result<Vec<String>, String>;
    fn delete(&self, name: &str) -> Result<(), String>;
    fn exists(&self, name: &str) -> Result<bool, String>;
}