use fleet_net_common::error::FleetNetError;
use fleet_net_protocol::connection::Connection;
use fleet_net_protocol::message::ControlMessage;
use fleet_net_protocol::tls::TlsConfig;
use std::borrow::Cow;
use std::path::Path;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_rustls::{rustls, TlsConnector};

pub struct ServerConnection {
    connection: Option<Arc<Mutex<Connection<tokio_rustls::client::TlsStream<TcpStream>>>>>,
}

impl ServerConnection {
    pub fn new() -> Self {
        Self { connection: None }
    }

    pub async fn connect(
        &mut self,
        server_addr: &str,
        cert_path: Option<&str>,
    ) -> Result<String, FleetNetError> {
        // For development, we'll use test certificates.
        let cert = if let Some(path) = cert_path {
            Path::new(path)
        } else {
            return Err(FleetNetError::FileSystemError(Cow::Borrowed(
                "Certificate path is required",
            )));
        };

        // Create TLS configuration
        let tls_config = TlsConfig::new_client(&cert).map_err(|e| {
            FleetNetError::EncryptionError(Cow::Owned(format!("TLS config error: {e}")))
        })?;

        let connector = TlsConnector::from(tls_config.client_config.ok_or(
            FleetNetError::EncryptionError(Cow::Borrowed("Failed to create client TLS config")),
        )?);

        // Connect to server
        let tcp_stream = TcpStream::connect(server_addr).await.map_err(|e| {
            FleetNetError::NetworkError(Cow::Owned(format!("TCP connection error: {e}")))
        })?;

        // Perform TLS handshakke
        let domain =
            rustls::pki_types::ServerName::try_from("localhost".to_owned()).map_err(|e| {
                FleetNetError::EncryptionError(Cow::Owned(format!("Invalid domain: {e}")))
            })?;

        let tls_stream = connector.connect(domain, tcp_stream).await.map_err(|e| {
            FleetNetError::EncryptionError(Cow::Owned(format!("TLS handshake failed: {e}")))
        })?;

        // Create Connection wrapper
        let conn = Connection::new(tls_stream);
        self.connection = Some(Arc::new(Mutex::new(conn)));

        Ok("Connected successfully".to_string())
    }

    pub async fn read_server_info(
        &self,
    ) -> Result<fleet_net_common::types::ServerInfo, FleetNetError> {
        let conn = self
            .connection
            .as_ref()
            .ok_or(FleetNetError::NetworkError(Cow::Borrowed("Not connected")))?;

        let mut conn = conn.lock().await;

        match conn.read_message().await {
            Ok(ControlMessage::ServerInfo(info)) => Ok(info),
            Ok(msg) => Err(FleetNetError::NetworkError(Cow::Owned(format!(
                "Unexpected message: {msg:?}"
            )))),
            Err(e) => Err(FleetNetError::NetworkError(Cow::Owned(format!(
                "Failed to read message: {e}"
            )))),
        }
    }
}
