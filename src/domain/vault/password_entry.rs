use secrecy::{ExposeSecret, SecretBox};
use std::fmt;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub name: String,
    pub username: String,
    #[serde(with = "secret_serde")]
    pub password: SecretBox<String>,
}

impl Zeroize for PasswordEntry {
    fn zeroize(&mut self) {
        self.name.zeroize();
        self.username.zeroize();

        let mut empty = String::new();
        empty.zeroize(); // prevent old password from being left in memory
        self.password = SecretBox::new(Box::new(empty));
    }
}

impl Drop for PasswordEntry {
    fn drop(&mut self) {
        self.zeroize();
    }
}

pub struct PasswordEntryDebug<'a> {
    pub entry: &'a PasswordEntry,
    pub expose: bool,
}

impl PasswordEntry {
    pub fn expose(&self) -> PasswordEntryDebug {
        PasswordEntryDebug { entry: self, expose: true }
    }
}

impl fmt::Display for PasswordEntryDebug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let password_str = if self.expose {
            self.entry.password.expose_secret().to_string()
        } else {
            "<hidden>".to_string()
        };
        write!(
            f,
            "{}\n  {} {}\n  {} {}\n",
            self.entry.name.bold().bright_white(),
            "Username:".italic(),
            self.entry.username.white(),
            "Password:".italic(),
            password_str.white(),
        )
    }
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
