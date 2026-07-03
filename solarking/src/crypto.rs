use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use sha2::{Digest, Sha256};
use std::env;

const NONCE_LEN: usize = 12;

pub fn encryption_enabled() -> bool {
    env::var("SOLARKING_PASSPHRASE")
        .map(|s| !s.is_empty())
        .unwrap_or(false)
}

fn derive_key(passphrase: &str) -> [u8; 32] {
    let hash = Sha256::digest(passphrase.as_bytes());
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash);
    key
}

pub fn encrypt_ledger(plaintext: &[u8]) -> Result<Vec<u8>, String> {
    let passphrase = env::var("SOLARKING_PASSPHRASE").map_err(|_| "no passphrase")?;
    let key = derive_key(&passphrase);
    let cipher = ChaCha20Poly1305::new_from_slice(&key).map_err(|e| e.to_string())?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    getrandom::getrandom(&mut nonce_bytes).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| e.to_string())?;

    let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

pub fn decrypt_ledger(data: &[u8]) -> Result<String, String> {
    if data.len() < NONCE_LEN {
        return Err("ciphertext too short".into());
    }
    let passphrase = env::var("SOLARKING_PASSPHRASE").map_err(|_| "no passphrase")?;
    let key = derive_key(&passphrase);
    let cipher = ChaCha20Poly1305::new_from_slice(&key).map_err(|e| e.to_string())?;

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "decryption failed — wrong passphrase?".to_string())?;
    String::from_utf8(plaintext).map_err(|e| e.to_string())
}