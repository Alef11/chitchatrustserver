use base64::{Engine, engine::general_purpose};
use rand::{TryRngCore, rngs::OsRng};
use sha2::{Digest, Sha256};
use crate::logger::logger::LogExpect;

pub fn encrypt(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    // Convert hash bytes to a hex string
    format!("{:x}", result)
}

pub fn check_password(input: &str, hash: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    // Convert hash bytes to a hex string and compare with the stored hash
    format!("{:x}", result) == hash
}

pub fn generate_token() -> String {
    let mut bytes = [0u8; 32]; // 256-bit token
    OsRng
        .try_fill_bytes(&mut bytes)
        .log_expect("Failed to generate secure random bytes", file!()); // Cryptographically secure RNG
    general_purpose::URL_SAFE_NO_PAD.encode(&bytes) // Base64 URL-safe encoding
}
