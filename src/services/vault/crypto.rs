use crate::services::vault::constants::{NONCE_LENGTH, SALT_LENGTH};
use crate::services::vault::models::Vault;
use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256GcmSiv
};
use argon2::{self, password_hash::{PasswordHasher, SaltString}, Argon2, Params};
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use rand::Rng;
use secrecy::{ExposeSecret, SecretBox};

pub fn encrypt_vault(vault: &Vault, password: &SecretBox<String>) -> (String, String, String) {
    let data = serde_json::to_vec(vault).expect("Serialization failed");
    let mut rng = rand::rng();
    let salt: [u8; SALT_LENGTH] = rng.random();
    let nonce: [u8; NONCE_LENGTH] = rng.random();
    let key = derive_key(password, &salt);

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(key.expose_secret()));
    let ciphertext = cipher.encrypt(GenericArray::from_slice(&nonce), data.as_ref())
        .expect("Encryption failed");

    (
        STANDARD.encode(salt),
        STANDARD.encode(nonce),
        STANDARD.encode(ciphertext)
    )
}

pub fn decrypt_vault(
    password: &SecretBox<String>,
    ciphertext: &str,
    salt: &str,
    nonce: &str,
) -> Result<Vault, String> {
    let ciphertext_bytes = STANDARD.decode(ciphertext)
        .map_err(|e| format!("Ciphertext decoding failed: {}", e))?;
    let salt_bytes = STANDARD.decode(salt)
        .map_err(|e| format!("Salt decoding failed: {}", e))?;
    let nonce_bytes = STANDARD.decode(nonce)
        .map_err(|e| format!("Nonce decoding failed: {}", e))?;

    let key = derive_key(password, &salt_bytes);
    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(key.expose_secret()));
    let decrypted_data = cipher.decrypt(
        GenericArray::from_slice(&nonce_bytes),
        ciphertext_bytes.as_ref()
    ).map_err(|_| "Wrong vault password")?;

    serde_json::from_slice(&decrypted_data)
        .map_err(|e| format!("Deserialization failed: {}", e))
}

fn derive_key(password: &SecretBox<String>, salt: &[u8]) -> SecretBox<[u8; 32]> {
    let params = Params::new(32, 3, 1, None).expect("Invalid Argon2 parameters");
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let salt_str = SaltString::encode_b64(salt).expect("Failed to convert salt");
    let password_hash = argon2
        .hash_password(password.expose_secret().as_bytes(), &salt_str)
        .expect("Password hashing failed");

    let hash_bytes = password_hash.hash.expect("Failed to extract hash bytes");
    let mut key = [0u8; 32];
    key.copy_from_slice(hash_bytes.as_bytes());
    SecretBox::new(Box::from(key))
}

