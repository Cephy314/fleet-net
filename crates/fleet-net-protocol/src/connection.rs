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

    // Test connection can read and write control messages.

    // Test the connection rejects oversized messages.

    // Test connection handles invalid messages (malformed JSON, unknown types)

    // Test connection timeout - idle connections should be closed

    // Test error propagation - various failure modes are handled gracefully

    // Test for creating a session on connection

    // Test for handling a disconnection and cleaning up the session state

    // Test for handling a reconnection and resuming the previous session state
}
