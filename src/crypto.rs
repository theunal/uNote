use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

fn get_aes_key(password: &str) -> Key<Aes256Gcm> {
    let pwd_bytes = password.as_bytes();
    let mut key = [0u8; 32];
    let len = pwd_bytes.len().min(32);
    key[..len].copy_from_slice(&pwd_bytes[..len]);
    *Key::<Aes256Gcm>::from_slice(&key)
}

pub fn encrypt(plaintext: &str, password: &str) -> String {
    if password.is_empty() {
        return plaintext.to_string();
    }
    let cipher = Aes256Gcm::new(&get_aes_key(password));
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .expect("encryption failed");
    let mut combined = nonce_bytes.to_vec();
    combined.extend_from_slice(&ciphertext);
    BASE64.encode(&combined)
}

pub fn decrypt(data: &str, password: &str) -> String {
    if password.is_empty() {
        return data.to_string();
    }
    let bytes = BASE64.decode(data).expect("base64 decode failed");
    let cipher = Aes256Gcm::new(&get_aes_key(password));
    let (nonce_bytes, encrypted) = bytes.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher.decrypt(nonce, encrypted).expect("decryption failed");
    String::from_utf8(plaintext).expect("invalid UTF-8")
}
