use fleet_net_common::error::FleetNetError;
use fleet_net_protocol::connection::Connection;
use fleet_net_protocol::message::ControlMessage;
use fleet_net_protocol::tls::TlsConfig;
use std::borrow::Cow;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tracing::info;

pub struct ServerConfig {
    pub bind_address: String,
    pub tls_cert_path: Option<PathBuf>,
    pub tls_key_path: Option<PathBuf>,
}

pub struct Server {
    config: ServerConfig,
    listener: Option<TcpListener>,
    tls_acceptor: Option<TlsAcceptor>,
}

impl Server {
    pub fn new(config: ServerConfig) -> Result<Self, FleetNetError> {
        // Initialize TLS if cert and key paths are provided
        let tls_acceptor = if let (Some(cert_path), Some(key_path)) =
            (&config.tls_cert_path, &config.tls_key_path)
        {
            let tls_config = TlsConfig::new_server(cert_path, key_path)?;
            Some(TlsAcceptor::from(tls_config.server_config.unwrap()))
        } else {
            None
        };

        Ok(Self {
            config,
            listener: None,
            tls_acceptor,
        })
    }

    pub async fn start(&mut self) -> Result<SocketAddr, FleetNetError> {
        let listener = TcpListener::bind(&self.config.bind_address).await?;
        let addr = listener.local_addr()?;
        info!("Server listening on {}", addr);

        self.listener = Some(listener);
        Ok(addr)
    }

    pub async fn accept_connection(&self) -> Result<(), FleetNetError> {
        let listener = self
            .listener
            .as_ref()
            .ok_or(FleetNetError::NetworkError(Cow::Borrowed(
                "Server not started",
            )))?;
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from {}", addr);

        // Handle TLS if configured
        if let Some(acceptor) = &self.tls_acceptor {
            let tls_stream = acceptor.accept(stream).await?;
            let mut conn = Connection::new(tls_stream);

            // Send server info message
            let msg = ControlMessage::ServerInfo {
                name: "Fleet Net Server".to_string(),
                version: Cow::Borrowed("0.1.0"),
                user_count: 0,
                channel_count: 0,
            };
            conn.write_message(&msg).await?;
        }

        Ok(())
    }

    pub async fn run(&self) -> Result<(), FleetNetError> {
        let listener = self
            .listener
            .as_ref()
            .ok_or(FleetNetError::NetworkError(Cow::Borrowed(
                "Server not started",
            )))?;

        loop {
            let (stream, addr) = listener.accept().await?;
            info!("Accepted connection from {addr}");

            // CLone what we need for the spawned task.
            let acceptor = self.tls_acceptor.clone();

            // Spawn a task to handle this connection
            tokio::spawn(async move {
                if let Some(acceptor) = acceptor {
                    match acceptor.accept(stream).await {
                        Ok(tls_stream) => {
                            let mut conn = Connection::new(tls_stream);

                            // Send server info message
                            let msg = ControlMessage::ServerInfo {
                                name: "Fleet Net Server".to_string(),
                                version: Cow::Borrowed("0.1.0"),
                                user_count: 0,
                                channel_count: 0,
                            };

                            if let Err(e) = conn.write_message(&msg).await {
                                tracing::error!("Failed to send server info: {e}");
                            }
                        }
                        Err(e) => {
                            tracing::error!("TLS handshake failed: {e}");
                        }
                    }
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fleet_test_support::{generate_test_certs, init_crypto_once};
    use std::time::Duration;
    use tokio::net::TcpStream;
    use tokio_rustls::TlsConnector;
    use tracing::log::trace;

    #[tokio::test]
    async fn test_server_accepts_single_tls_connection() {
        init_crypto_once();

        // Given: Generate a self-signed certificate for testing
        let bundle = generate_test_certs("localhost");

        // Create server configuration
        let config = ServerConfig {
            bind_address: "127.0.0.1:0".to_string(), // Use port 0 for auto-assignment
            tls_cert_path: Some(bundle.cert_path.clone()),
            tls_key_path: Some(bundle.key_path.clone()),
        };

        // When: Create and start the server
        let mut server = Server::new(config).expect("Failed to create server");
        let addr = server.start().await.expect("Failed to start server");

        // Start handling connections in background
        let server_handle = tokio::spawn(async move { server.accept_connection().await });

        // Create TLS client
        let client_config =
            TlsConfig::new_client(&bundle.cert_path).expect("Failed to create client config");
        let connector = TlsConnector::from(client_config.client_config.unwrap());

        // Connect to server
        let tcp_stream = TcpStream::connect(addr)
            .await
            .expect("Failed to connect to server");

        let domain = rustls::pki_types::ServerName::try_from("localhost".to_owned())
            .expect("Invalid domain");
        let tls_stream = connector
            .connect(domain, tcp_stream)
            .await
            .expect("Failed to establish TLS connection");

        let mut conn = Connection::new(tls_stream);

        // Then: Client should receive ServerInfo message
        let msg = conn.read_message().await.expect("Failed to read message");

        match msg {
            ControlMessage::ServerInfo { name, version, .. } => {
                assert_eq!(name, "Fleet Net Server");
                assert_eq!(version, Cow::Borrowed("0.1.0"));
            }
            _ => panic!("Expected ServerInfo message, got {msg:?}"),
        }

        // Cleanup
        server_handle.abort();
    }

    #[tokio::test]
    async fn test_server_handles_multiple_concurrent_connections() {
        init_crypto_once();

        // Given: Generate a self-signed certificate for testing
        let bundle = generate_test_certs("localhost");

        // Create server configuration
        let config = ServerConfig {
            bind_address: "127.0.0.1:0".to_string(), // Use port 0 for auto-assignment
            tls_cert_path: Some(bundle.cert_path.clone()),
            tls_key_path: Some(bundle.key_path.clone()),
        };

        // Create and start server
        let mut server = Server::new(config).expect("Failed to create server");
        let addr = server.start().await.expect("Failed to start server");

        let server = std::sync::Arc::new(server);
        let server_clone = server.clone();
        let server_handle = tokio::spawn(async move { server_clone.run().await });

        tokio::time::sleep(Duration::from_millis(100)).await;

        let num_clients = 3;
        let mut client_handles = vec![];

        for i in 0..num_clients {
            //let addr = addr;
            let cert_path = bundle.cert_path.clone();

            let handle = tokio::spawn(async move {
                // Create client TLS config
                let client_config =
                    TlsConfig::new_client(&cert_path).expect("Failed to create client config");
                let connector = TlsConnector::from(client_config.client_config.unwrap());

                // connect to server
                let tcp_stream = TcpStream::connect(addr).await.expect("Failed to connect");

                let domain = rustls::pki_types::ServerName::try_from("localhost".to_owned())
                    .expect("Invalid domain");
                let tls_stream = connector
                    .connect(domain, tcp_stream)
                    .await
                    .expect("Failed to establish TLS");

                let mut conn = Connection::new(tls_stream);

                // Read ServerInfo message
                let msg = conn.read_message().await.expect("Failed to read message");

                // Verify we got ServerInfo
                match msg {
                    ControlMessage::ServerInfo { .. } => i,
                    _ => panic!("Client {i} got unexpected message"),
                }
            });

            client_handles.push(handle);
        }

        // Wait for all clients to complete
        for handle in client_handles {
            let client_num = handle.await.expect("Client task failed");
            trace!("Client {client_num} successfully connected");
        }

        // Cleanup: stop the server.as
        server_handle.abort();
    }
}
