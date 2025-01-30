use chrono::{DateTime, Utc};
use secrecy::SecretBox;
use crate::domain::vault::vault::Vault;

pub struct AppSession {
    pub vault: Vault,
    pub name: String,
    pub secret: SecretBox<String>,
    pub expires_at: DateTime<Utc>,
}