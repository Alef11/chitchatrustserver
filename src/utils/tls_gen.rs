use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, SanType};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::logger::logger::LogExpect;
use crate::log;

pub fn generate_localhost_certs() {
    let cert_path = Path::new("certs/certificate.pem");
    let key_path = Path::new("certs/private.pem");

    if cert_path.exists() && key_path.exists() {
        log!("TLS certificate and key already exist.");
        return;
    }

    log!("Generating self-signed TLS certificate for localhost...");

    fs::create_dir_all("certs").log_expect("Failed to create certs directory", file!());

    let mut params = CertificateParams::new(vec!["localhost".into()]);
    params.distinguished_name = DistinguishedName::new();
    params
        .distinguished_name
        .push(DnType::CommonName, "localhost");
    params
        .subject_alt_names
        .push(SanType::DnsName("localhost".into()));

    let cert = Certificate::from_params(params).log_expect("Failed to generate certificate", file!());

    let mut cert_file = File::create(cert_path).log_expect("Failed to create certificate file", file!());
    cert_file
        .write_all(cert.serialize_pem().unwrap().as_bytes())
        .log_expect("Failed to write certificate", file!());

    let mut key_file = File::create(key_path).log_expect("Failed to create private key file", file!());
    key_file
        .write_all(cert.serialize_private_key_pem().as_bytes())
        .log_expect("Failed to write private key", file!());

    log!("Certificate and key generated at: certs/");
}
