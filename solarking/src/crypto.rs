//! ChaCha20-Poly1305 ledger encryption with versioned KDF.
//!
//! v1 envelope (legacy): `[12-byte nonce][ciphertext]` — SHA-256(passphrase) key
//! v2 envelope: `[0x02][16-byte salt][12-byte nonce][ciphertext]` — Argon2id key

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use sha2::{Digest, Sha256};
use std::env;

const NONCE_LEN: usize = 12;
const SALT_LEN: usize = 16;
const VERSION_V2: u8 = 0x02;

pub fn encryption_enabled() -> bool {
    env::var("SOLARKING_PASSPHRASE")
        .map(|s| !s.is_empty())
        .unwrap_or(false)
}

fn passphrase() -> Result<String, String> {
    env::var("SOLARKING_PASSPHRASE").map_err(|_| "no passphrase".into())
}

fn derive_key_v1(passphrase: &str) -> [u8; 32] {
    let hash = Sha256::digest(passphrase.as_bytes());
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash);
    key
}

fn derive_key_v2(passphrase: &str, salt: &[u8]) -> Result<[u8; 32], String> {
    // Modest params — interactive unlock on commodity hardware
    let params = Params::new(19_456, 2, 1, Some(32)).map_err(|e| e.to_string())?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .map_err(|e| e.to_string())?;
    Ok(key)
}

/// Encrypt with v2 Argon2id envelope.
pub fn encrypt_ledger(plaintext: &[u8]) -> Result<Vec<u8>, String> {
    let passphrase = passphrase()?;
    let mut salt = [0u8; SALT_LEN];
    getrandom::getrandom(&mut salt).map_err(|e| e.to_string())?;
    let key = derive_key_v2(&passphrase, &salt)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key).map_err(|e| e.to_string())?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    getrandom::getrandom(&mut nonce_bytes).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| e.to_string())?;

    let mut out = Vec::with_capacity(1 + SALT_LEN + NONCE_LEN + ciphertext.len());
    out.push(VERSION_V2);
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// Decrypt v2 (Argon2) or legacy v1 (SHA-256) envelopes.
pub fn decrypt_ledger(data: &[u8]) -> Result<String, String> {
    if data.is_empty() {
        return Err("ciphertext empty".into());
    }

    if data[0] == VERSION_V2 {
        decrypt_v2(data)
    } else {
        decrypt_v1(data)
    }
}

fn decrypt_v2(data: &[u8]) -> Result<String, String> {
    let min = 1 + SALT_LEN + NONCE_LEN + 16; // tag
    if data.len() < min {
        return Err("v2 ciphertext too short".into());
    }
    let passphrase = passphrase()?;
    let salt = &data[1..1 + SALT_LEN];
    let nonce_bytes = &data[1 + SALT_LEN..1 + SALT_LEN + NONCE_LEN];
    let ciphertext = &data[1 + SALT_LEN + NONCE_LEN..];

    let key = derive_key_v2(&passphrase, salt)?;
    let cipher = ChaCha20Poly1305::new_from_slice(&key).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "decryption failed — wrong passphrase?".to_string())?;
    String::from_utf8(plaintext).map_err(|e| e.to_string())
}

fn decrypt_v1(data: &[u8]) -> Result<String, String> {
    if data.len() < NONCE_LEN {
        return Err("ciphertext too short".into());
    }
    let passphrase = passphrase()?;
    let key = derive_key_v1(&passphrase);
    let cipher = ChaCha20Poly1305::new_from_slice(&key).map_err(|e| e.to_string())?;

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "decryption failed — wrong passphrase?".to_string())?;
    String::from_utf8(plaintext).map_err(|e| e.to_string())
}

/// Test helpers: serialize all tests that touch `SOLARKING_PASSPHRASE`
/// so parallel suites cannot race save/load encryption paths.
#[cfg(test)]
pub(crate) mod test_env {
    use std::env;
    use std::sync::{Mutex, MutexGuard};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    pub fn lock() -> MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Run `f` with encryption disabled (passphrase cleared), then restore prior value.
    pub fn without_passphrase<R>(f: impl FnOnce() -> R) -> R {
        let _g = lock();
        let prev = env::var("SOLARKING_PASSPHRASE").ok();
        env::remove_var("SOLARKING_PASSPHRASE");
        let result = f();
        match prev {
            Some(v) => env::set_var("SOLARKING_PASSPHRASE", v),
            None => env::remove_var("SOLARKING_PASSPHRASE"),
        }
        result
    }

    /// Run `f` with a known passphrase set, then restore prior value.
    pub fn with_passphrase<R>(passphrase: &str, f: impl FnOnce() -> R) -> R {
        let _g = lock();
        let prev = env::var("SOLARKING_PASSPHRASE").ok();
        env::set_var("SOLARKING_PASSPHRASE", passphrase);
        let result = f();
        match prev {
            Some(v) => env::set_var("SOLARKING_PASSPHRASE", v),
            None => env::remove_var("SOLARKING_PASSPHRASE"),
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_v2() {
        test_env::with_passphrase("sovereign-test-key", || {
            let pt = br#"{"harmonic_369":1}"#;
            let ct = encrypt_ledger(pt).unwrap();
            assert_eq!(ct[0], VERSION_V2);
            let out = decrypt_ledger(&ct).unwrap();
            assert_eq!(out.as_bytes(), pt);
        });
    }

    #[test]
    fn wrong_passphrase_fails() {
        let _g = test_env::lock();
        let prev = env::var("SOLARKING_PASSPHRASE").ok();
        env::set_var("SOLARKING_PASSPHRASE", "correct");
        let ct = encrypt_ledger(b"secret").unwrap();
        env::set_var("SOLARKING_PASSPHRASE", "wrong");
        assert!(decrypt_ledger(&ct).is_err());
        match prev {
            Some(v) => env::set_var("SOLARKING_PASSPHRASE", v),
            None => env::remove_var("SOLARKING_PASSPHRASE"),
        }
    }

    #[test]
    fn legacy_v1_still_decrypts() {
        test_env::with_passphrase("legacy-key", || {
            let key = derive_key_v1("legacy-key");
            let cipher = ChaCha20Poly1305::new_from_slice(&key).unwrap();
            let nonce_bytes = [1u8; 12];
            let nonce = Nonce::from_slice(&nonce_bytes);
            let ct = cipher.encrypt(nonce, b"hello-v1".as_ref()).unwrap();
            let mut data = Vec::new();
            data.extend_from_slice(&nonce_bytes);
            data.extend_from_slice(&ct);
            let out = decrypt_ledger(&data).unwrap();
            assert_eq!(out, "hello-v1");
        });
    }
}
