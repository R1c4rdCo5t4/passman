use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct VaultFile {
    pub(crate) salt: String,
    pub(crate) nonce: String,
    pub(crate) ciphertext: String,
}