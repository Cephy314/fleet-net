//! TLS test helpers for creating test configurations and connections

use crate::crypto::TestCertBundle;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use rustls::{ClientConfig, RootCertStore, ServerConfig};
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_rustls::{TlsAcceptor, TlsConnector};

/// Create a TLS server configuration from a test certificate bundle.
pub fn server_config_from_bundle(bundle: &TestCertBundle) -> Arc<ServerConfig> {
    let certs = load_certs(&bundle.cert_path).expect("Failed to load server certs");
    let key = load_private_key(&bundle.key_path).expect("Failed to load server key");

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .expect("Failed to create server config");

    Arc::new(config)
}

/// Create a TLS client configuration that trusts the given certificate.
pub fn client_config_from_bundle(bundle: &TestCertBundle) -> Arc<ClientConfig> {
    let ca_certs = load_certs(&bundle.cert_path).expect("Failed to load CA certs");

    let mut root_store = RootCertStore::empty();
    for cert in ca_certs {
        root_store
            .add(cert)
            .expect("Failed to add cert to root store");
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    Arc::new(config)
}

/// Create a TLS acceptor from a server configuration.
pub fn create_tls_acceptor(config: Arc<ServerConfig>) -> TlsAcceptor {
    TlsAcceptor::from(config)
}

/// Create a TLS connector from a client configuration.
pub fn create_tls_connector(config: Arc<ClientConfig>) -> TlsConnector {
    TlsConnector::from(config)
}

/// Helper to connect a TLS client to a server.
pub async fn tls_connect(
    connector: &TlsConnector,
    addr: std::net::SocketAddr,
    hostname: &str,
) -> std::io::Result<tokio_rustls::client::TlsStream<TcpStream>> {
    let tcp_stream = TcpStream::connect(addr).await?;
    let domain = ServerName::try_from(hostname.to_owned())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
    connector.connect(domain, tcp_stream).await
}

// Helper functions for loading certificates and keys
fn load_certs(path: &Path) -> std::io::Result<Vec<CertificateDer<'static>>> {
    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = rustls_pemfile::certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(certs)
}

fn load_private_key(path: &Path) -> std::io::Result<PrivateKeyDer<'static>> {
    use rustls_pemfile::{ec_private_keys, pkcs8_private_keys, rsa_private_keys};

    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);

    // Try PKCS8 first
    let pkcs8_keys = pkcs8_private_keys(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    if !pkcs8_keys.is_empty() {
        return Ok(PrivateKeyDer::Pkcs8(pkcs8_keys.into_iter().next().unwrap()));
    }

    // Reset and try RSA
    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let rsa_keys = rsa_private_keys(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    if !rsa_keys.is_empty() {
        return Ok(PrivateKeyDer::Pkcs1(rsa_keys.into_iter().next().unwrap()));
    }

    // Reset and try EC
    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let ec_keys = ec_private_keys(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    if !ec_keys.is_empty() {
        return Ok(PrivateKeyDer::Sec1(ec_keys.into_iter().next().unwrap()));
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "No valid private keys found",
    ))
}
