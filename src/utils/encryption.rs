use sha2::{Digest, Sha256};

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
