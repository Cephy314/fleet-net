use crate::message::ControlMessage;
use fleet_net_common::error::FleetNetError;
use std::borrow::Cow;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub struct Connection<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    stream: S,
}

impl<S> Connection<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    pub async fn write_message(&mut self, message: &ControlMessage) -> Result<(), FleetNetError> {
        // Serialize the message to JSON
        let json = serde_json::to_string(message)?;

        // Write the length of the message first
        let length = json.len() as u32;
        self.stream.write_all(&length.to_be_bytes()).await?;

        // Then write the actual message
        self.stream.write_all(json.as_bytes()).await?;

        Ok(())
    }

    pub async fn read_message(&mut self) -> Result<ControlMessage, FleetNetError> {
        // First read the length of the incoming message
        let mut length_bytes = [0u8; 4];
        self.stream.read_exact(&mut length_bytes).await?;

        // Convert bytes to u32
        let length = u32::from_be_bytes(length_bytes);

        // Read the actual message data
        let mut buffer = vec![0u8; length as usize];
        self.stream.read_exact(&mut buffer).await?;

        // Test if the length matches the buffer size
        if buffer.len() != length as usize {
            return Err(FleetNetError::PacketError(Cow::Borrowed(
                "Received message length does not match expected length",
            )));
        }

        // Deserialize the JSON message
        let message: ControlMessage = serde_json::from_slice(&buffer)?;

        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::ControlMessage;
    use fleet_test_support::connected_tcp_pair;
    use std::borrow::Cow;

    // Test connection handles message framing and deframing correctly.
    #[tokio::test]
    async fn test_connection_handles_message_framing() {
        // Set up connected streams.
        let (server_stream, client_stream) = connected_tcp_pair().await.unwrap();

        // Create connections
        let mut server_connection = Connection::new(server_stream);
        let mut client_connection = Connection::new(client_stream);

        // Server sends a message
        let message = ControlMessage::ServerInfo {
            name: "TestServer".to_string(),
            version: Cow::Borrowed("1.0.0"),
            user_count: 0,
            channel_count: 0,
        };

        // Use a task to avoid deadlock
        let server_task = tokio::spawn(async move {
            server_connection.write_message(&message).await.unwrap();
        });

        // Client reads the message
        let received = client_connection.read_message().await.unwrap();

        // Verify we got the correct message
        match received {
            ControlMessage::ServerInfo {
                name,
                version,
                user_count,
                channel_count,
            } => {
                assert_eq!(name, "TestServer");
                assert_eq!(version, Cow::Borrowed("1.0.0"));
                assert_eq!(user_count, 0);
                assert_eq!(channel_count, 0);
            }
            _ => panic!("Expected ServerInfo message"),
        }

        server_task.await.unwrap();
    }
}

#[cfg(test)]
mod tls_tests {
    use crate::connection::Connection;
    use crate::message::ControlMessage;
    use crate::tls::TlsConfig;
    use fleet_test_support::{generate_test_certs, init_crypto_once};
    use std::borrow::Cow;
    use tokio::net::{TcpListener, TcpStream};
    use tokio_rustls::{TlsAcceptor, TlsConnector};

    // Helper to create TLS acceptor from certificate bundle
    async fn create_tls_server(
        bundle: &fleet_test_support::TestCertBundle,
    ) -> (TlsAcceptor, TcpListener, std::net::SocketAddr) {
        let server_config = TlsConfig::new_server(&bundle.cert_path, &bundle.key_path)
            .expect("Failed to create server config");
        let acceptor = TlsAcceptor::from(server_config.server_config.unwrap());
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (acceptor, listener, addr)
    }

    // Helper to create TLS connector from certificate bundle
    fn create_tls_client(bundle: &fleet_test_support::TestCertBundle) -> TlsConnector {
        let client_config =
            TlsConfig::new_client(&bundle.cert_path).expect("Failed to create client config");
        TlsConnector::from(client_config.client_config.unwrap())
    }

    // Helper to attempt TLS connection
    async fn try_tls_connect(
        connector: &TlsConnector,
        addr: std::net::SocketAddr,
        hostname: &str,
    ) -> Result<tokio_rustls::client::TlsStream<TcpStream>, std::io::Error> {
        let tcp_stream = TcpStream::connect(addr).await?;
        let domain = rustls::pki_types::ServerName::try_from(hostname.to_owned())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
        connector.connect(domain, tcp_stream).await
    }

    #[tokio::test]
    async fn test_tls_connection_establishes_secure_channel() {
        init_crypto_once();
        // Given: A server with valid TLS certificates
        let bundle = generate_test_certs("localhost");

        // Create server and client
        let (acceptor, listener, addr) = create_tls_server(&bundle).await;
        let connector = create_tls_client(&bundle);

        // Server task - accept and wrap with TLS
        let server_task = tokio::spawn(async move {
            let (tcp_stream, _) = listener.accept().await.unwrap();
            let tls_stream = acceptor.accept(tcp_stream).await.unwrap();

            // Create connection with TLS stream and send message
            let mut conn = Connection::new(tls_stream);
            let msg = ControlMessage::ServerInfo {
                name: "TLSTestServer".to_string(),
                version: Cow::Borrowed("1.0.0"),
                user_count: 42,
                channel_count: 5,
            };
            conn.write_message(&msg).await.unwrap();
        });

        // Client connect with TLS
        let tls_stream = try_tls_connect(&connector, addr, "localhost")
            .await
            .expect("Failed to establish TLS connection");

        let mut client_conn = Connection::new(tls_stream);
        let received = client_conn.read_message().await.unwrap();

        // Then Verify the message was transmitted correctly over TLS
        match received {
            ControlMessage::ServerInfo {
                name,
                version,
                user_count,
                channel_count,
            } => {
                assert_eq!(name, "TLSTestServer");
                assert_eq!(version, Cow::Borrowed("1.0.0"));
                assert_eq!(user_count, 42);
                assert_eq!(channel_count, 5);
            }
            _ => panic!("Expected ServerInfo message"),
        }

        server_task.await.unwrap();
    }

    #[tokio::test]
    async fn test_tls_rejects_untrusted_certificate() {
        init_crypto_once();

        // Generate different certificates.
        let server_bundle = generate_test_certs("localhost");
        let untrusted_bundle = generate_test_certs("untrusted-ca");

        // Setup server with its certificates.
        let (acceptor, listener, addr) = create_tls_server(&server_bundle).await;

        // Setup client with untrusted CA
        let connector = create_tls_client(&untrusted_bundle);

        // Server accepts connection (byt handshake will fail)
        let server_task = tokio::spawn(async move {
            let (tcp_stream, _) = listener.accept().await.unwrap();
            let _ = acceptor.accept(tcp_stream).await; // Ignore result
        });

        // Client connection should fail
        let result = try_tls_connect(&connector, addr, "localhost").await;
        assert!(
            result.is_err(),
            "Connection should be rejected due to untrusted certificate"
        );

        let error_msg = format!("{:?}", result.err().unwrap());
        assert!(
            error_msg.contains("CertificateUnknown")
                || error_msg.contains("InvalidCertificate")
                || error_msg.contains("UnknownIssuer"),
            "Expected certificate validation error, got: {error_msg}"
        );

        server_task.await.unwrap();
    }

    #[tokio::test]
    async fn test_tls_accepts_trusted_certificate() {
        init_crypto_once();

        // Use same certificate for server and client trust
        let bundle = generate_test_certs("localhost");

        // Setup server and client with same cert
        let (acceptor, listener, addr) = create_tls_server(&bundle).await;
        let connector = create_tls_client(&bundle);

        // Server task
        let server_task = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let tls_stream = acceptor.accept(stream).await.unwrap();

            let mut conn = Connection::new(tls_stream);
            let msg = ControlMessage::ServerInfo {
                name: "TrustedServer".to_string(),
                version: Cow::Borrowed("1.0.0"),
                user_count: 1,
                channel_count: 1,
            };
            conn.write_message(&msg).await.unwrap();
        });

        // Client connects successfully
        let tls_stream = try_tls_connect(&connector, addr, "localhost")
            .await
            .expect("Connection should succeed with trusted certificate");

        let mut client_conn = Connection::new(tls_stream);
        let received = client_conn.read_message().await.unwrap();

        match received {
            ControlMessage::ServerInfo { name, .. } => {
                assert_eq!(name, "TrustedServer");
            }
            _ => panic!("Expected ServerInfo message"),
        }

        server_task.await.unwrap();
    }
}
