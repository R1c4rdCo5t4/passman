use serde::{Deserialize, Serialize};
use zeroize::Zeroize;
use secrecy::SecretBox;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub service: String,
    pub username: String,
    #[serde(with = "secret_serde")]
    pub password: SecretBox<String>,
}

impl Zeroize for PasswordEntry {
    fn zeroize(&mut self) {
        self.service.zeroize();
        self.username.zeroize();
        self.password = SecretBox::new(Box::from(String::new()));
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct VaultFile {
    pub(crate) salt: String,
    pub(crate) nonce: String,
    pub(crate) ciphertext: String,
}

mod secret_serde {
    use secrecy::{ExposeSecret, SecretBox};
    use serde::{Serializer, Deserializer, Deserialize};

    pub fn serialize<S>(secret: &SecretBox<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(secret.expose_secret())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SecretBox<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|s| SecretBox::new(Box::from(s)))
    }
}
