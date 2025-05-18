use std::{fs, path::Path};
use crate::logger::logger::log_message;

pub fn generate_certs_directory() {
    log_message("Checking for certs directory", file!());
    let certs_dir = "certs";
    if !Path::new(certs_dir).exists() {
        log_message("Certs directory does not exist, creating it", file!());
        fs::create_dir(certs_dir).expect("Failed to create certs directory");
    }
}