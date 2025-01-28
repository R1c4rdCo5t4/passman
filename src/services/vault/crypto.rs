use aes_gcm_siv::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    Aes256GcmSiv
};
use argon2::{self, password_hash::{PasswordHasher, SaltString}, Argon2, Params};
use rand::Rng;
use base64::{Engine as _, engine::general_purpose};
use secrecy::{SecretBox, ExposeSecret};
use crate::services::vault::constants::{NONCE_LENGTH, SALT_LENGTH};
use crate::services::vault::models::Vault;


pub fn derive_key(password: &SecretBox<String>, salt: &[u8]) -> SecretBox<[u8; 32]> {
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
        general_purpose::STANDARD.encode(salt),
        general_purpose::STANDARD.encode(nonce),
        general_purpose::STANDARD.encode(ciphertext)
    )
}

pub fn decrypt_vault(
    ciphertext: &str,
    nonce: &str,
    key: &SecretBox<[u8; 32]>
) -> Result<Vault, String> {
    let nonce_bytes = general_purpose::STANDARD.decode(nonce)
        .map_err(|e| format!("Nonce decoding failed: {}", e))?;
    let ciphertext_bytes = general_purpose::STANDARD.decode(ciphertext)
        .map_err(|e| format!("Ciphertext decoding failed: {}", e))?;

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(key.expose_secret()));
    let decrypted_data = cipher.decrypt(
        GenericArray::from_slice(&nonce_bytes),
        ciphertext_bytes.as_ref()
    ).map_err(|e| format!("Decryption failed: {}", e))?;

    serde_json::from_slice(&decrypted_data)
        .map_err(|e| format!("Deserialization failed: {}", e))
}
