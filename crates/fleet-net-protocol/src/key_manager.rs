use crate::hmac::HmacKey;
use fleet_net_common::types::UserId;
use sha2::{Digest, Sha256};

pub struct ProtocolKeys {
    pub tcp_key: HmacKey,
    pub udp_key: HmacKey,
}

pub struct KeyManager;

impl KeyManager {
    pub fn generate_session_key(
        user_id: UserId,
        server_secret: &[u8],
        session_nonce: &[u8],
    ) -> HmacKey {
        let mut hasher = Sha256::new();

        // Mix all inputs together
        hasher.update(server_secret);
        hasher.update(user_id.to_be_bytes());
        hasher.update(session_nonce);

        // Get the hash result
        let result = hasher.finalize();

        // Convert to 32-byte array for HmacKey
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&result[..32]);

        HmacKey::from_bytes(&key_bytes)
    }

    pub fn derive_protocol_keys(base_key: &HmacKey) -> ProtocolKeys {
        // Derive separate keys for TCP and UDP by hashing the base key with different labels
        let mut tcp_hasher = Sha256::new();
        tcp_hasher.update(base_key.as_bytes());
        tcp_hasher.update(b"TCP_KEY_DERIVATION");
        let tcp_result = tcp_hasher.finalize();

        let mut udp_hasher = Sha256::new();
        udp_hasher.update(base_key.as_bytes());
        udp_hasher.update(b"UDP_KEY_DERIVATION");
        let udp_result = udp_hasher.finalize();

        let mut tcp_key_bytes = [0u8; 32];
        tcp_key_bytes.copy_from_slice(&tcp_result[..32]);

        let mut udp_key_bytes = [0u8; 32];
        udp_key_bytes.copy_from_slice(&udp_result[..32]);

        ProtocolKeys {
            tcp_key: HmacKey::from_bytes(&tcp_key_bytes),
            udp_key: HmacKey::from_bytes(&udp_key_bytes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hmac::extract_hmac_prefix;
    use crate::message::{ControlMessage, FramedMessage};
    use fleet_net_common::types::JoinChannelRequest;

    #[test]
    fn test_generate_session_key() {
        // Test generating a cryptographically secure random session key for a user
        let user_id: UserId = 42;
        let server_secret = b"super_secret_server_key_32b!!!!!";
        let session_nonce = b"unique_session_nonce_value";

        // Generate session key deterministically for testing
        let key = KeyManager::generate_session_key(user_id, server_secret, session_nonce);

        // Should produce a valid 32-byte key
        assert_eq!(key.as_bytes().len(), 32);

        // Same inputs should produce same key (deterministic)
        let key2 = KeyManager::generate_session_key(user_id, server_secret, session_nonce);
        assert_eq!(key.as_bytes(), key2.as_bytes());
    }

    #[test]
    fn test_derive_protocol_key() {
        // Test deriving separate keys for TCP and UDP from a base key
        let base_key = HmacKey::from_bytes(b"base_session_key_32_bytes_long!!");

        let keys = KeyManager::derive_protocol_keys(&base_key);

        // TCP and UDP keys should be different
        assert_ne!(keys.tcp_key.as_bytes(), keys.udp_key.as_bytes());

        // Both should be valid 32-byte keys
        assert_eq!(keys.tcp_key.as_bytes().len(), 32);
        assert_eq!(keys.udp_key.as_bytes().len(), 32);
    }

    #[test]
    fn test_tcp_message_flow_with_hmac() {
        // Simulate server generating a session key for a user
        let user_id: UserId = 1001;
        let server_secret = b"super_secret_server_key_32b!!!!!";
        let session_nonce = b"unique_session_nonce_value_10011";

        let session_key = KeyManager::generate_session_key(user_id, server_secret, session_nonce);

        // Derive protocol-specific keys
        let keys = KeyManager::derive_protocol_keys(&session_key);

        // Client sends a TCP control message
        let msg = ControlMessage::JoinChannelRequest(JoinChannelRequest::new(42));
        let framed = FramedMessage::new(&msg, &keys.tcp_key);

        // Server receives and validates the message
        let decoded = framed.validate_and_decode(&keys.tcp_key).unwrap();

        // Should get the original message back
        match decoded {
            ControlMessage::JoinChannelRequest(info) => {
                assert_eq!(info.channel_id, 42)
            }
            _ => panic!("Unexpected message type"),
        }
    }

    #[test]
    fn test_udp_packet_flow_with_hmac() {
        // Generate session and protocol keys
        let session_key = KeyManager::generate_session_key(
            2002,
            b"another_secret_server_key_32b!",
            b"session_nonce_2002",
        );
        let keys = KeyManager::derive_protocol_keys(&session_key);

        // Create audio packet header
        let mut header = crate::packet::PacketHeader {
            channel_id: 5,
            user_id: 10,
            sequence: 100,
            timestamp: 123456,
            signal_strength: 255,
            frame_duration: 20,
            audio_length: 128,
            hmac_prefix: 0, // Will be set after HMAC calculation
        };

        // Generate audio data
        let audio_data = vec![0xFF; 128];

        // Calculate HMAC for the packet
        let mut packet_bytes = Vec::new();
        packet_bytes.extend_from_slice(&header.channel_id.to_be_bytes());
        packet_bytes.extend_from_slice(&header.user_id.to_be_bytes());
        packet_bytes.extend_from_slice(&header.sequence.to_be_bytes());
        packet_bytes.extend_from_slice(&header.timestamp.to_be_bytes());
        packet_bytes.push(header.signal_strength);
        packet_bytes.push(header.frame_duration);
        packet_bytes.extend_from_slice(&header.audio_length.to_be_bytes());
        packet_bytes.extend_from_slice(&audio_data);

        let full_hmac = crate::hmac::generate_hmac(&keys.udp_key, &packet_bytes);
        header.hmac_prefix = extract_hmac_prefix(&full_hmac);

        // Validate on receiver side
        assert!(header.validate_hmac(&keys.udp_key, &audio_data))
    }

    #[test]
    fn test_invalid_hmac_rejected() {
        let key1 = HmacKey::from_bytes(b"valid_session_key_32_bytes_long!");
        let key2 = HmacKey::from_bytes(b"invalid_session_key_32_bytes_lon");

        // Create message with key1
        let msg = ControlMessage::Ping;
        let framed = FramedMessage::new(&msg, &key1);

        // Try to validate with key2 - should fail
        assert!(framed.validate_and_decode(&key2).is_err());

        // Validate with correct key - should succeed
        assert!(framed.validate_and_decode(&key1).is_ok());
    }
}
