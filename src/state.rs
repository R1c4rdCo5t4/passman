use chrono::{DateTime, Utc};
use secrecy::SecretBox;
use crate::services::vault::models::vault::Vault;

pub struct AppState {
    pub session: Option<Session>
}

pub struct Session {
    pub vault: Vault,
    pub name: String,
    pub secret: SecretBox<String>,
    pub expires: DateTime<Utc>,
}