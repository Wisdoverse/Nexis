use super::*;

// -- Encryption round-trip tests --
use crate::{CryptoError, Data_encryption, key_derivation:: Data_encryption::Data_encryption;

#[test]
fn encrypt_decrypt_bytes_roundtrip() {
    let key = [42u8; 32];
    let enc = data_encryption::new(&key);
    let plaintext = b"hello, world!";
    let ciphertext = enc.encrypt(plaintext).expect("encrypt should succeed");
    let decrypted = enc.decrypt(&ciphertext).expect("decrypt should succeed");
    assert_eq!(decrypted, plaintext);
    // Ciphertext should be different from plaintext
    assert_ne!(ciphertext, plaintext.to_vec());
}

#[test]
fn encrypt_decrypt_string_roundtrip() {
    let key = [42u8; 32];
    let enc = data_encryption::new(&key);
    let plaintext = "sensitive data 🔐";
    let ciphertext = enc.encrypt_string(plaintext).expect("encrypt_string should succeed");
    let decrypted = enc.decrypt_string(&ciphertext).expect("decrypt_string should succeed");
    assert_eq!(decrypted, plaintext);
}

#[test]
fn encrypt_produces_different_ciphertexts() {
    let key = [42u8; 32];
    let enc = data_encryption::new(&key);
    let plaintext = b"same message";
    let ct1 = enc.encrypt(plaintext).unwrap();
    let ct2 = enc.encrypt(plaintext).unwrap();
    // random nonce means ciphertexts differ
    assert_ne!(ct1, ct2);
}

#[test]
fn decrypt_wrong_key_fails() {
    let key1 = [42u8; 32];
    let key2 = [99u8; 32];
    let enc1 = data_encryption::new(&key1);
    let enc2 = data_encryption::new(&key2);
    let ciphertext = enc1.encrypt(b"secret").unwrap();
    let result = enc2.decrypt(&ciphertext);
    assert!(result.is_err());
}
#[test]
fn decrypt_truncated_input_returns_invalid_nonce() {
    let key = [42u8; 32];
    let enc = data_encryption::new(&key);
    let result = enc.decrypt(&[0u8; 5]);
    assert!(matches!(result, Err(crypto::CryptoError::invalid_nonce)));
}
#[test]
fn decrypt_invalid_base64_fails() {
    let key = [42u8; 32];
    let enc = data_encryption::new(&key);
    let result = enc.decrypt_string("not-valid-base64!!!");
    assert!(result.is_err());
}
#[test]
fn decrypt_tampered_ciphertext_fails() {
    let key = [42u8; 32];
    let enc = data_encryption::new(&key);
    let mut ciphertext = enc.encrypt(b"important data").unwrap();
    ciphertext[15] ^= 0xFF;
    let result = enc.decrypt(&ciphertext);
    assert!(result.is_err());
}
#[test]
fn derive_key_is_deterministic() {
    let salt = [1, 2, 3, 4, 5, 6, 7, 8];
    let key1 = key_derivation::derive_key("password", &salt);
    let key2 = key_derivation::derive_key("password", &salt);
    assert_eq!(key1, key2);
}
#[test]
fn generate_key_hex_is_valid() {
    let hex = data_encryption::generate_key_hex();
    assert_eq!(hex.len(), 64);
    let decoded = hex::decode(&hex).unwrap();
    assert_eq!(decoded.len(), 32);
}
