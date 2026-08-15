use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};
use argon2::Argon2;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

const SALT_LEN: usize = 16;

fn derive_key(password: &str, salt: &[u8]) -> Result<Key<Aes256Gcm>, String> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| format!("argon2 key derivation failed: {e}"))?;
    Ok(Key::<Aes256Gcm>::from(key))
}

pub fn encrypt(plaintext: &str, password: &str) -> String {
    if password.is_empty() {
        return plaintext.to_string();
    }
    let salt: [u8; SALT_LEN] = rand::random();
    let cipher = Aes256Gcm::new(&derive_key(password, &salt).expect("argon2 key derivation failed"));
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from(nonce_bytes);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .expect("encryption failed");
    let mut combined = salt.to_vec();
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);
    BASE64.encode(&combined)
}

pub fn decrypt(data: &str, password: &str) -> Result<String, String> {
    if password.is_empty() {
        return Err("master password required".to_string());
    }
    let bytes = BASE64
        .decode(data)
        .map_err(|e| format!("base64 decode failed: {e}"))?;
    let (salt, rest) = bytes.split_at(SALT_LEN);
    let cipher = Aes256Gcm::new(&derive_key(password, salt)?);
    let (nonce_bytes, encrypted) = rest.split_at(12);
    let nonce = Nonce::try_from(nonce_bytes).map_err(|_| "invalid nonce length".to_string())?;
    let plaintext = cipher
        .decrypt(&nonce, encrypted)
        .map_err(|_| "decryption failed (wrong master password or corrupted data)".to_string())?;
    String::from_utf8(plaintext).map_err(|e| format!("invalid UTF-8: {e}"))
}
