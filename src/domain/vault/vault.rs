use serde::{Deserialize, Serialize};
use zeroize::Zeroize;
use crate::domain::vault::password_entry::PasswordEntry;

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<PasswordEntry>,
}

impl Zeroize for Vault {
    fn zeroize(&mut self) {
        self.entries.iter_mut().for_each(|entry| {
            entry.zeroize();
        });
        self.entries.clear();
        self.entries.shrink_to_fit();
    }
}

impl Drop for Vault {
    fn drop(&mut self) {
        self.zeroize();
    }
}