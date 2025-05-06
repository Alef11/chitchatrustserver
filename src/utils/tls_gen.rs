use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, SanType};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn generate_localhost_certs() {
    let cert_path = Path::new("certs/certificate.pem");
    let key_path = Path::new("certs/private.pem");

    if cert_path.exists() && key_path.exists() {
        println!("✅ TLS certificate and key already exist.");
        return;
    }

    println!("🔐 Generating self-signed TLS certificate for localhost...");

    fs::create_dir_all("certs").expect("Failed to create certs directory");

    let mut params = CertificateParams::new(vec!["localhost".into()]);
    params.distinguished_name = DistinguishedName::new();
    params
        .distinguished_name
        .push(DnType::CommonName, "localhost");
    params
        .subject_alt_names
        .push(SanType::DnsName("localhost".into()));

    let cert = Certificate::from_params(params).expect("Failed to generate certificate");

    let mut cert_file = File::create(cert_path).expect("Failed to create certificate file");
    cert_file
        .write_all(cert.serialize_pem().unwrap().as_bytes())
        .expect("Failed to write certificate");

    let mut key_file = File::create(key_path).expect("Failed to create private key file");
    key_file
        .write_all(cert.serialize_private_key_pem().as_bytes())
        .expect("Failed to write private key");

    println!("✅ Certificate and key generated at: certs/");
}
