use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VaultFile {
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}