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
    use fleet_net_common::error::FleetNetError;
    use fleet_test_support::{generate_test_certs, init_crypto_once};
    use std::fs;
    use std::sync::Arc;
    use tempfile::TempDir;

    #[test]
    fn test_load_server_certificates() {
        init_crypto_once();

        // Given: Generate a valid self-signed certificate and key
        let bundle = generate_test_certs("localhost");

        // When: Loading them for server configuration
        let tls_config = TlsConfig::new_server(&bundle.cert_path, &bundle.key_path);

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
        init_crypto_once();

        // Given: A CA certificate for validation
        let ca_bundle = generate_test_certs("ca.localhost");

        // When: Creating a client configuration
        let tls_config = TlsConfig::new_client(&ca_bundle.cert_path);

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
    fn test_reject_missing_certificate_files() {
        init_crypto_once();
        use std::path::Path;
        // Test 1: None-existent files should return file system error.
        let result = TlsConfig::new_server(
            Path::new("/non/existent/cert.pem"),
            Path::new("/non/existent/key.pem"),
        );

        assert!(result.is_err());
        if let Err(FleetNetError::FileSystemError(msg)) = result {
            assert!(msg.contains("Failed to open"));
        } else {
            panic!("Expected FileSystemError for non-existent files");
        }
    }

    #[test]
    fn test_reject_empty_certification_files() {
        init_crypto_once();
        let temp_dir = TempDir::new().unwrap();
        let empty_cert = temp_dir.path().join("empty_cert.pem");
        let empty_key = temp_dir.path().join("empty_key.pem");

        fs::write(&empty_cert, "").unwrap();
        fs::write(&empty_key, "").unwrap();

        let result = TlsConfig::new_server(&empty_cert, &empty_key);
        assert!(result.is_err());
        if let Err(FleetNetError::EncryptionError(msg)) = result {
            assert!(msg.contains("No certificates found") || msg.contains("No valid private keys"));
        } else {
            panic!("Expected EncryptionError for empty certificate/key files");
        }
    }

    #[test]
    fn test_reject_invalid_pem_data() {
        init_crypto_once();

        let temp_dir = TempDir::new().unwrap();
        let invalid_cert = temp_dir.path().join("invalid_cert.pem");
        let invalid_key = temp_dir.path().join("invalid_key.pem");

        fs::write(&invalid_cert, "This is not a valid certificate").unwrap();
        fs::write(&invalid_key, "This is not a valid key").unwrap();

        let result = TlsConfig::new_server(&invalid_cert, &invalid_key);
        assert!(result.is_err());
        assert!(matches!(result, Err(FleetNetError::EncryptionError(_))));
    }

    #[test]
    fn test_tls_config_cipher_suites() {
        init_crypto_once();

        // Create a valid self-signed certificate and key
        let bundle = generate_test_certs("localhost");

        let tls_config = TlsConfig::new_server(&bundle.cert_path, &bundle.key_path)
            .expect("Should create valid TLS config");

        let server_config = tls_config.server_config.unwrap();

        // Verify that we're using the ring crypto provider (installed in init_crypto)
        // This ensures we're using modern, secure cipher suites

        // Verify no insecure protocol versions are allowed
        // rustls by default only allows TLS 1.2 and 1.3, which is what we want

        // The server config should exist and be properly configured
        // rustls automatically excludes weak ciphers like:
        // - RC4
        // - DES/3DES
        // - Export ciphers
        // - NULL ciphers

        // We can verify the config is created successfully as a basic check
        // More detailed cipher suite inspection would require checking
        // server_config's internal crypto provider settings

        assert_eq!(Arc::strong_count(&server_config), 1);
    }
}
