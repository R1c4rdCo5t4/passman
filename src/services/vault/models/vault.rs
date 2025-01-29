use serde::{Deserialize, Serialize};
use zeroize::Zeroize;
use crate::services::vault::models::password_entry::PasswordEntry;

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<PasswordEntry>,
}

impl Zeroize for Vault {
    fn zeroize(&mut self) {
        self.entries.iter_mut().for_each(|entry| {
            entry.zeroize();
        });
    }
}