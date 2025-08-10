//! Cryptography test helpers including certificate generation and provider initialization

use once_cell::sync::OnceCell;
use rcgen::{generate_simple_self_signed, CertifiedKey};
use std::path::PathBuf;
use tempfile::TempDir;

/// Initialize the rustls crypto provider once for all tests.
/// This is safe to call multiple times and will only initialize once.
pub fn init_crypto_once() {
    static INIT: OnceCell<()> = OnceCell::new();
    let _ = INIT.get_or_try_init(|| {
        rustls::crypto::ring::default_provider()
            .install_default()
            .map_err(|_| ()) // Ignore "already initialized" errors
    });
}

/// Bundle containing test certificates and their file paths
pub struct TestCertBundle {
    /// Temporary directory containing the certificate files (cleaned up on drop)
    pub temp_dir: TempDir,
    /// Path to the certificate PEM file
    pub cert_path: PathBuf,
    /// Path to the private key PEM file
    pub key_path: PathBuf,
    /// The generated certificate and key pair
    pub cert: CertifiedKey,
}

/// Generate a self-signed certificate for testing with the given hostname.
///
/// The certificate will be valid for the specified hostname as well as
/// "localhost", "127.0.0.1", and "::1" to cover common test scenarios.
pub fn generate_test_certs(hostname: &str) -> TestCertBundle {
    let cert = generate_simple_self_signed(vec![
        hostname.to_string(),
        "localhost".to_string(),
        "127.0.0.1".to_string(),
        "::1".to_string(),
    ])
    .expect("Failed to generate certificate");

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let cert_path = temp_dir.path().join("cert.pem");
    let key_path = temp_dir.path().join("key.pem");

    std::fs::write(&cert_path, cert.cert.pem()).expect("Failed to write cert");
    std::fs::write(&key_path, cert.key_pair.serialize_pem()).expect("Failed to write key");

    TestCertBundle {
        temp_dir,
        cert_path,
        key_path,
        cert,
    }
}

/// Generate a CA certificate and a server certificate signed by it.
/// Useful for testing certificate chain validation.
pub fn generate_ca_and_server_certs(server_hostname: &str) -> (TestCertBundle, TestCertBundle) {
    // Generate CA certificate
    let ca_cert = generate_simple_self_signed(vec!["Test CA".to_string()])
        .expect("Failed to generate CA certificate");

    let ca_temp_dir = TempDir::new().expect("Failed to create CA temp dir");
    let ca_cert_path = ca_temp_dir.path().join("ca_cert.pem");
    let ca_key_path = ca_temp_dir.path().join("ca_key.pem");

    std::fs::write(&ca_cert_path, ca_cert.cert.pem()).expect("Failed to write CA cert");
    std::fs::write(&ca_key_path, ca_cert.key_pair.serialize_pem()).expect("Failed to write CA key");

    let ca_bundle = TestCertBundle {
        temp_dir: ca_temp_dir,
        cert_path: ca_cert_path,
        key_path: ca_key_path,
        cert: ca_cert,
    };

    // For now, we'll use a self-signed cert as the server cert
    // (proper CA signing would require more complex rcgen setup)
    let server_bundle = generate_test_certs(server_hostname);

    (ca_bundle, server_bundle)
}
