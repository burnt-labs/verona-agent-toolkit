//! AES-GCM encryption for credential storage.
//!
//! This module provides encrypted file-based credential storage that works
//! across all platforms without requiring OS keychain interaction.
//!
//! # Key Derivation Strategy
//!
//! The encryption key is derived from (in order of priority):
//! 1. `VERONA_CI_ENCRYPTION_KEY` environment variable (for CI/CD only)
//! 2. Machine ID via `machine-uid` crate (for local development, default)
//!
//! **Note**: Local development does NOT need `VERONA_CI_ENCRYPTION_KEY`.
//! The machine ID derivation is the default and recommended for local use.
//!
//! # Security Model
//!
//! - Uses AES-256-GCM for authenticated encryption
//! - Random 96-bit nonce for each encryption operation
//! - Key is never stored on disk
//!
//! # CI/CD Usage
//!
//! For automated testing in CI/CD environments, set `VERONA_CI_ENCRYPTION_KEY` to a fixed
//! 32-byte hex string. This is only needed in CI/CD where machine ID may be unstable.
//!
//! ```bash
//! export VERONA_CI_ENCRYPTION_KEY=0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
//! cargo test
//! ```

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;

use super::env_compat::env_var_with_legacy;

/// Environment variable name for CI/CD encryption key
pub const ENV_KEY_NAME: &str = "VERONA_CI_ENCRYPTION_KEY";

const LEGACY_ENV_KEY_NAME: &str = "XION_CI_ENCRYPTION_KEY";

/// Key length in bytes (AES-256)
const KEY_LEN: usize = 32;

/// Nonce length in bytes (96-bit for GCM)
const NONCE_LEN: usize = 12;

const NEW_SALT: &[u8] = b"verona-toolkit-credentials-v1:";
const LEGACY_SALT: &[u8] = b"xion-toolkit-credentials-v1:";

fn env_encryption_key_hex() -> Option<String> {
    env_var_with_legacy(ENV_KEY_NAME, LEGACY_ENV_KEY_NAME)
}

/// Get or derive the encryption key.
///
/// Priority:
/// 1. `VERONA_CI_ENCRYPTION_KEY` environment variable (hex-encoded 32 bytes), with `XION_CI_ENCRYPTION_KEY` fallback
/// 2. Machine ID derivation with the Verona salt prefix
pub fn get_encryption_key() -> Result<[u8; KEY_LEN]> {
    if let Some(key_hex) = env_encryption_key_hex() {
        let key_bytes = hex::decode(&key_hex)
            .with_context(|| format!("Failed to decode {} as hex", ENV_KEY_NAME))?;

        if key_bytes.len() != KEY_LEN {
            return Err(anyhow!(
                "{} must be exactly {} bytes ({} hex characters), got {} bytes",
                ENV_KEY_NAME,
                KEY_LEN,
                KEY_LEN * 2,
                key_bytes.len()
            ));
        }

        let mut key = [0u8; KEY_LEN];
        key.copy_from_slice(&key_bytes);
        return Ok(key);
    }

    derive_key_from_salt(NEW_SALT)
}

fn derive_key_from_salt(salt: &[u8]) -> Result<[u8; KEY_LEN]> {
    let machine_id = machine_uid::get().map_err(|e| {
        anyhow::anyhow!(
            "Failed to get machine ID: {}. Set {} environment variable for CI/CD",
            e,
            ENV_KEY_NAME
        )
    })?;

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(machine_id.as_bytes());
    let hash = hasher.finalize();

    let mut key = [0u8; KEY_LEN];
    key.copy_from_slice(&hash);
    Ok(key)
}

fn legacy_machine_encryption_key() -> Result<[u8; KEY_LEN]> {
    derive_key_from_salt(LEGACY_SALT)
}

fn decrypt_with_key(ciphertext_b64: &str, key: &[u8; KEY_LEN]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).context("Failed to create cipher from key")?;

    let data = BASE64
        .decode(ciphertext_b64)
        .context("Failed to decode base64 ciphertext")?;

    if data.len() < NONCE_LEN {
        return Err(anyhow!("Ciphertext too short"));
    }

    let nonce = Nonce::from_slice(&data[..NONCE_LEN]);
    let ciphertext = &data[NONCE_LEN..];

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("Decryption failed: {}", e))
}

/// Encrypt data using AES-256-GCM.
///
/// Returns base64-encoded ciphertext with prepended nonce.
/// Format: base64(nonce || ciphertext || tag)
pub fn encrypt(plaintext: &[u8]) -> Result<String> {
    let key = get_encryption_key()?;
    let cipher = Aes256Gcm::new_from_slice(&key).context("Failed to create cipher from key")?;

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| anyhow!("Encryption failed: {}", e))?;

    // Prepend nonce to ciphertext
    let mut result = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(&result))
}

/// Decrypt data encrypted with `encrypt`.
///
/// Expects base64-encoded ciphertext with prepended nonce.
/// When using machine-derived keys, retries with the legacy salt prefix.
pub fn decrypt(ciphertext_b64: &str) -> Result<Vec<u8>> {
    let primary_key = get_encryption_key()?;
    match decrypt_with_key(ciphertext_b64, &primary_key) {
        Ok(plaintext) => Ok(plaintext),
        Err(primary_err) => {
            if env_encryption_key_hex().is_some() {
                return Err(primary_err);
            }
            let legacy_key = legacy_machine_encryption_key()?;
            decrypt_with_key(ciphertext_b64, &legacy_key).map_err(|_| primary_err)
        }
    }
}

/// Generate a random key for testing purposes.
///
/// Returns a 32-byte hex-encoded string suitable for VERONA_CI_ENCRYPTION_KEY.
#[allow(dead_code)]
pub fn generate_test_key() -> String {
    let mut key = [0u8; KEY_LEN];
    rand::rng().fill_bytes(&mut key);
    hex::encode(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    /// Helper to set up encryption key for tests
    fn setup_test_key() -> Option<String> {
        let original = env::var(ENV_KEY_NAME).ok();
        let test_key = generate_test_key();
        env::set_var(ENV_KEY_NAME, &test_key);
        original
    }

    /// Helper to restore original key
    fn restore_key(original: Option<String>) {
        if let Some(key) = original {
            env::set_var(ENV_KEY_NAME, key);
        } else {
            env::remove_var(ENV_KEY_NAME);
        }
    }

    #[test]
    #[serial(encryption_key)]
    fn test_encrypt_decrypt_roundtrip() {
        let original = setup_test_key();

        let plaintext = b"hello, world!";
        let encrypted = encrypt(plaintext).expect("Encryption failed");
        let decrypted = decrypt(&encrypted).expect("Decryption failed");
        assert_eq!(plaintext.as_slice(), decrypted.as_slice());

        restore_key(original);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_encrypt_produces_different_ciphertext() {
        let original = setup_test_key();

        let plaintext = b"hello, world!";
        let encrypted1 = encrypt(plaintext).expect("Encryption failed");
        let encrypted2 = encrypt(plaintext).expect("Encryption failed");
        // Different due to random nonce
        assert_ne!(encrypted1, encrypted2);

        restore_key(original);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_decrypt_wrong_key_fails() {
        let original_key = env::var(ENV_KEY_NAME).ok();

        env::set_var(
            ENV_KEY_NAME,
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        );
        let plaintext = b"secret data";
        let encrypted = encrypt(plaintext).expect("Encryption failed");

        // Change key
        env::set_var(
            ENV_KEY_NAME,
            "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
        );
        let result = decrypt(&encrypted);
        assert!(result.is_err(), "Decryption with wrong key should fail");

        restore_key(original_key);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_env_key_validation() {
        let original_key = env::var(ENV_KEY_NAME).ok();

        // Too short
        env::set_var(ENV_KEY_NAME, "0123456789abcdef");
        assert!(get_encryption_key().is_err());

        // Invalid hex
        env::set_var(
            ENV_KEY_NAME,
            "not-valid-hex-string-!!!!!-not-valid-hex-string!!",
        );
        assert!(get_encryption_key().is_err());

        // Correct length
        env::set_var(
            ENV_KEY_NAME,
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        );
        assert!(get_encryption_key().is_ok());

        // Restore
        restore_key(original_key);
    }

    #[test]
    fn test_generate_test_key() {
        let key1 = generate_test_key();
        let key2 = generate_test_key();
        assert_eq!(key1.len(), 64); // 32 bytes = 64 hex chars
        assert_ne!(key1, key2); // Random keys should differ
    }

    #[test]
    #[serial(encryption_key)]
    fn test_legacy_env_key_fallback() {
        let original_verona = env::var(ENV_KEY_NAME).ok();
        let original_xion = env::var(LEGACY_ENV_KEY_NAME).ok();
        env::remove_var(ENV_KEY_NAME);
        env::set_var(
            LEGACY_ENV_KEY_NAME,
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        );

        assert!(get_encryption_key().is_ok());

        if let Some(key) = original_verona {
            env::set_var(ENV_KEY_NAME, key);
        } else {
            env::remove_var(ENV_KEY_NAME);
        }
        if let Some(key) = original_xion {
            env::set_var(LEGACY_ENV_KEY_NAME, key);
        } else {
            env::remove_var(LEGACY_ENV_KEY_NAME);
        }
    }

    #[test]
    #[serial(encryption_key)]
    fn test_decrypt_legacy_salt_ciphertext() {
        let original = setup_test_key();
        env::remove_var(ENV_KEY_NAME);
        env::remove_var(LEGACY_ENV_KEY_NAME);

        let plaintext = b"legacy salt payload";
        let legacy_key = legacy_machine_encryption_key().expect("legacy key");
        let cipher =
            Aes256Gcm::new_from_slice(&legacy_key).expect("Failed to create cipher from key");
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_ref())
            .expect("Encryption failed");
        let mut blob = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        blob.extend_from_slice(&nonce_bytes);
        blob.extend_from_slice(&ciphertext);
        let encrypted = BASE64.encode(blob);

        let decrypted = decrypt(&encrypted).expect("legacy decrypt");
        assert_eq!(plaintext.as_slice(), decrypted.as_slice());

        restore_key(original);
    }

    #[test]
    #[serial(encryption_key)]
    fn test_decrypt_malformed_data() {
        let original = setup_test_key();

        let result = decrypt("not-valid-base64!!!");
        assert!(result.is_err());

        let short_data = BASE64.encode(b"short");
        let result = decrypt(&short_data);
        assert!(result.is_err());

        restore_key(original);
    }
}
