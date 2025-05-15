use std::{fs, path::Path};

pub fn generate_certs_directory() {
    let certs_dir = "certs";
    if !Path::new(certs_dir).exists() {
        fs::create_dir(certs_dir).expect("Failed to create certs directory");
    }
}