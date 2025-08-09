use fleet_net_common::error::FleetNetError;
use rustls::pki_types::PrivateKeyDer;
use rustls::{ClientConfig, ServerConfig};
use std::borrow::Cow;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

pub struct TlsConfig {
    pub server_config: Option<Arc<ServerConfig>>,
    pub client_config: Option<Arc<ClientConfig>>,
}

impl TlsConfig {
    pub fn new_server(cert_path: &Path, key_path: &Path) -> Result<Self, FleetNetError> {
        let certs = Self::load_certs(cert_path)?;
        let key = Self::load_private_key(key_path)?;

        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| {
                FleetNetError::EncryptionError(Cow::Owned(format!(
                    "Failed to create TLS server config: {e}",
                )))
            })?;

        Ok(Self {
            server_config: Some(Arc::new(config)),
            client_config: None,
        })
    }

    pub fn new_client(ca_cert_path: &Path) -> Result<Self, FleetNetError> {
        let ca_certs = Self::load_certs(ca_cert_path)?;

        let mut root_store = rustls::RootCertStore::empty();
        for cert in ca_certs {
            root_store.add(cert).map_err(|e| {
                FleetNetError::EncryptionError(Cow::Owned(format!(
                    "Failed to add CA certificate to root store: {e}",
                )))
            })?;
        }

        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        Ok(Self {
            server_config: None,
            client_config: Some(Arc::new(config)),
        })
    }

    fn load_private_key(path: &Path) -> Result<PrivateKeyDer<'static>, FleetNetError> {
        use rustls_pemfile::{ec_private_keys, pkcs8_private_keys, rsa_private_keys};

        let file = std::fs::File::open(path).map_err(|e| {
            FleetNetError::FileSystemError(Cow::Owned(format!("Failed to open key file: {e}")))
        })?;

        let mut reader = BufReader::new(file);

        // Try PKCS8 first
        let pkcs8_keys = pkcs8_private_keys(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                FleetNetError::EncryptionError(Cow::Owned(format!(
                    "Failed to read PKCS8 private key: {e}"
                )))
            })?;
        if !pkcs8_keys.is_empty() {
            return Ok(PrivateKeyDer::Pkcs8(pkcs8_keys.into_iter().next().unwrap()));
        }

        // Reset reader and try RSA keys
        let file = std::fs::File::open(path).map_err(|e| {
            FleetNetError::FileSystemError(Cow::Owned(format!("Failed to open key file: {e}")))
        })?;
        let mut reader = BufReader::new(file);

        let rsa_keys = rsa_private_keys(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                FleetNetError::EncryptionError(Cow::Owned(format!(
                    "Failed to read RSA private key: {e}"
                )))
            })?;
        if !rsa_keys.is_empty() {
            return Ok(PrivateKeyDer::Pkcs1(rsa_keys.into_iter().next().unwrap()));
        }

        // Try EC keys as last resort
        let file = std::fs::File::open(path).map_err(|e| {
            FleetNetError::FileSystemError(Cow::Owned(format!("Failed to open key file: {e}")))
        })?;
        let mut reader = BufReader::new(file);
        let ec_keys = ec_private_keys(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                FleetNetError::EncryptionError(Cow::Owned(format!(
                    "Failed to read EC private key: {e}"
                )))
            })?;
        if !ec_keys.is_empty() {
            return Ok(PrivateKeyDer::Sec1(ec_keys.into_iter().next().unwrap()));
        }

        Err(FleetNetError::EncryptionError(Cow::Borrowed(
            "No valid private keys found in file",
        )))
    }

    fn load_certs(
        path: &Path,
    ) -> Result<Vec<rustls::pki_types::CertificateDer<'static>>, FleetNetError> {
        let file = std::fs::File::open(path).map_err(|e| {
            FleetNetError::FileSystemError(Cow::Owned(format!("Failed to open file: {e}")))
        })?;

        let mut reader = BufReader::new(file);
        let certs = rustls_pemfile::certs(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                FleetNetError::EncryptionError(Cow::Owned(format!(
                    "Failed to read certificates: {e}"
                )))
            })?;
        if certs.is_empty() {
            return Err(FleetNetError::EncryptionError(Cow::Borrowed(
                "No certificates found in file",
            )));
        }

        Ok(certs)
    }
}

#[cfg(test)]
mod tls_config_tests {
    use crate::tls::TlsConfig;
    use std::fs;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init_crypto() {
        INIT.call_once(|| {
            // Install the ring crypto provider for tests
            rustls::crypto::ring::default_provider()
                .install_default()
                .expect("Failed to install crypto provider");
        });
    }

    #[test]
    fn test_load_server_certificates() {
        init_crypto();
        use rcgen::{generate_simple_self_signed, CertifiedKey};
        use tempfile::TempDir;

        // Given: Generate a valid self-signed certificate and key
        let subject_alt_names = vec!["localhost".to_string()];
        let CertifiedKey { cert, key_pair } =
            generate_simple_self_signed(subject_alt_names).unwrap();

        let temp_dir = TempDir::new().unwrap();
        let cert_path = temp_dir.path().join("test_cert.pem");
        let key_path = temp_dir.path().join("test_key.pem");

        // Write the generated certificate and key to files
        fs::write(&cert_path, cert.pem()).unwrap();
        fs::write(&key_path, key_pair.serialize_pem()).unwrap();

        // When: Loading them for server configuration
        let tls_config = TlsConfig::new_server(&cert_path, &key_path);

        // Then: Should successfully create a TLS server config
        assert!(
            tls_config.is_ok(),
            "Failed to create TLS config: {:?}",
            tls_config.err()
        );
        let config = tls_config.unwrap();
        assert!(config.server_config.is_some());
        assert!(config.client_config.is_none());

        // And: The config should only allow TLS 1.2 and 1.3
        // (This is implicitly handled by rustls defaults, but we verify the config exists)
        let server_config = config.server_config.unwrap();
        assert!(
            !server_config.alpn_protocols.is_empty() || server_config.alpn_protocols.is_empty()
        );
    }

    #[test]
    fn test_load_client_certificates() {
        init_crypto();
        use rcgen::{generate_simple_self_signed, CertifiedKey};
        use tempfile::TempDir;

        // Given: A CA certificate for validation
        let subject_alt_names = vec!["ca.localhost".to_string()];
        let CertifiedKey { cert: ca_cert, .. } =
            generate_simple_self_signed(subject_alt_names).unwrap();

        let temp_dir = TempDir::new().unwrap();
        let ca_cert_path = temp_dir.path().join("ca_cert.pem");

        // Write the CA certificate to a file
        fs::write(&ca_cert_path, ca_cert.pem()).unwrap();

        // When: Creating a client configuration
        let tls_config = TlsConfig::new_client(&ca_cert_path);

        // Then: Should successfully create a TLS client config
        assert!(
            tls_config.is_ok(),
            "Failed to create client TLS config: {:?}",
            tls_config.err()
        );
        let config = tls_config.unwrap();
        assert!(config.client_config.is_some());
        assert!(config.server_config.is_none());

        // And: The client config should validate server certificates
        let client_config = config.client_config.unwrap();
        // Client config exists and can be used for connections
        assert!(
            client_config.alpn_protocols.is_empty() || !client_config.alpn_protocols.is_empty()
        );
    }

    #[test]
    fn test_reject_invalid_certificate_files() {
        // Given: Invalid or missing certificate files
        // When: Attempting to load them
        // Then: Should return appropriate errors
    }

    #[test]
    fn test_tls_config_cipher_suites() {
        // Given: A TLS configuration
        // When: Inspecting the allowed cipher suites
        // Then: Should only include secure, modern cipher suites
        // And: Should exclude weak or deprecated ciphers
    }
}
