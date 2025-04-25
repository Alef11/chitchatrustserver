use chitchatrustserver::utils::encryption::{check_password, encrypt};

#[test]
fn test_to_sha256() {
    let input = "hello world";
    let expected_hash = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
    let result = encrypt(input);
    assert_eq!(result, expected_hash);
}

#[test]
fn test_check_password() {
    let input = "hello world";
    let hash = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
    let result = check_password(input, hash);
    assert!(result);
}

#[test]
fn test_check_password_fail() {
    let input = "hello world!"; // Incorrect password
    let hash = "a";
    let result = check_password(input, hash);
    assert!(!result); // Should fail}
}

#[test]
fn test_sha256_fail() {
    let input = "hello world";
    let expected_hash = "opfa";
    let result = encrypt(input);
    assert_ne!(result, expected_hash);
}
