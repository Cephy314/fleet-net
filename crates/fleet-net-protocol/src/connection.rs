use crate::message::ControlMessage;
use fleet_net_common::error::FleetNetError;
use std::borrow::Cow;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
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
    use std::borrow::Cow;
    use tokio::net::{TcpListener, TcpStream};

    // Helper function to create a connected pair of TCP streams.
    async fn create_test_connection_pair() -> (TcpStream, TcpStream) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Connect from client side
        let client_future = TcpStream::connect(addr);

        // Accept on server side
        let server_future = async move {
            let (stream, _) = listener.accept().await.unwrap();
            stream
        };

        // Wait for both to complete.
        let (client, server) = tokio::join!(client_future, server_future);

        (server, client.unwrap())
    }

    // Test connection handles message framing and deframing correctly.
    #[tokio::test]
    async fn test_connection_handles_message_framing() {
        // Set up connected streams.
        let (server_stream, client_stream) = create_test_connection_pair().await;

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
    #[tokio::test]
    async fn test_tls_connection_establishes_secure_channel() {
        // Given: A server with valid TLS certificates
        // let cert_path = "test_certs/server.crt";
        // let key_path = "test_certs/server.key";

        // When: A client connects using TLS
        // Then: The connection should be established successfully
        // And: Messages should be encrypted in transit

        // This test verifies that:
        // 1. TLS handshake completes successfully
        // 2. Data sent through the connection is encrypted
        // 3. Both sides can exchange ControlMessages over TLS
    }

    #[tokio::test]
    async fn test_tls_connection_validates_certificates() {
        // Given: A server with an invalid/self-signed certificate
        // When: A client attempts to connect with certificate validation enabled
        // Then: The connection should be rejected

        // Given: A server with a valid certificate
        // When: A client connects with certificate validation
        // Then: The connection should succeed
    }

    #[tokio::test]
    async fn test_tls_connection_supports_client_certificates() {
        // Given: A server configured to require client certificates
        // When: A client connects without a certificate
        // Then: The connection should be rejected

        // When: A client connects with a valid certificate
        // Then: The connection should succeed
        // And: The server should be able to identify the client
    }

    #[tokio::test]
    async fn test_tls_connection_handles_protocol_versions() {
        // Given: A server configured to accept only TLS 1.2 and 1.3
        // When: A client attempts to connect with TLS 1.1
        // Then: The connection should be rejected

        // When: A client connects with TLS 1.2 or 1.3
        // Then: The connection should succeed
    }

    #[tokio::test]
    async fn test_plain_and_tls_connections_use_same_interface() {
        // Given: Two connections, one plain TCP and one TLS
        // When: Sending the same ControlMessage through both
        // Then: Both should successfully transmit the message
        // This ensures our abstraction works correctly
    }
}
