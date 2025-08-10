//! Network test helpers for creating connected streams and mock connections

use std::io;
use tokio::io::{duplex, DuplexStream};
use tokio::net::{TcpListener, TcpStream};

/// Create a connected pair of TCP streams for testing.
///
/// This creates a real TCP connection over localhost, useful for testing
/// actual network behavior including socket options and TCP features.
pub async fn connected_tcp_pair() -> io::Result<(TcpStream, TcpStream)> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;

    // Connect client and accept server connection concurrently
    let client = TcpStream::connect(addr);
    let server = async {
        let (stream, _) = listener.accept().await?;
        Ok::<_, io::Error>(stream)
    };

    let (client, server) = tokio::try_join!(client, server)?;
    Ok((server, client))
}

/// Create a pair of in-memory duplex streams for testing.
///
/// These streams are connected - data written to one can be read from the other.
/// This is perfect for testing protocol logic without the overhead of real networking.
///
/// # Arguments
/// * `buffer_size` - Size of the internal buffer for each direction (default 8192)
pub fn mock_connection_pair(buffer_size: usize) -> (DuplexStream, DuplexStream) {
    duplex(buffer_size)
}

/// Create a pair of in-memory duplex streams with default buffer size (8KB).
pub fn mock_connection_pair_default() -> (DuplexStream, DuplexStream) {
    mock_connection_pair(8192)
}

/// Helper to bind a TCP listener on an ephemeral port and return the address.
pub async fn bind_ephemeral() -> io::Result<(TcpListener, std::net::SocketAddr)> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    Ok((listener, addr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_connected_tcp_pair() {
        let (mut server, mut client) = connected_tcp_pair()
            .await
            .expect("Failed to create TCP pair");

        // Test bidirectional communication
        client.write_all(b"hello").await.expect("Failed to write");
        let mut buf = [0u8; 5];
        server.read_exact(&mut buf).await.expect("Failed to read");
        assert_eq!(&buf, b"hello");

        server.write_all(b"world").await.expect("Failed to write");
        client.read_exact(&mut buf).await.expect("Failed to read");
        assert_eq!(&buf, b"world");
    }

    #[tokio::test]
    async fn test_mock_connection_pair() {
        let (mut stream1, mut stream2) = mock_connection_pair_default();

        // Test bidirectional communication
        stream1.write_all(b"test").await.expect("Failed to write");
        let mut buf = [0u8; 4];
        stream2.read_exact(&mut buf).await.expect("Failed to read");
        assert_eq!(&buf, b"test");

        stream2.write_all(b"data").await.expect("Failed to write");
        stream1.read_exact(&mut buf).await.expect("Failed to read");
        assert_eq!(&buf, b"data");
    }
}
