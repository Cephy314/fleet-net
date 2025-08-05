use crate::hmac::{generate_hmac, validate_hmac, HmacKey};
use fleet_net_common::error::FleetNetError;
use fleet_net_common::types::{ChannelId, UserId};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// Message frame with HMAC for integrity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FramedMessage {
    pub payload: Vec<u8>,
    pub hmac: Vec<u8>, // HMAC-SHA256
}

impl FramedMessage {
    // Create a new framed message with HMAC
    pub fn new(message: &ControlMessage, key: &HmacKey) -> Self {
        let payload = serde_json::to_vec(message).expect("Failed to serialize message");
        let hmac = generate_hmac(key, &payload);

        Self { payload, hmac }
    }

    /// Validate the HMAC and deserialize the payload
    pub fn validate_and_decode(&self, key: &HmacKey) -> Result<ControlMessage, FleetNetError> {
        // Validate HMAC first
        if !validate_hmac(key, &self.payload, &self.hmac) {
            return Err(FleetNetError::PacketError(Cow::Borrowed(
                "Invalid HMAC, message integrity check failed",
            )));
        }

        // Deserialize the message
        serde_json::from_slice(&self.payload)
            .map_err(|_| FleetNetError::PacketError(Cow::Borrowed("Failed to deserialize message")))
    }
}

// TCP Control Messages for state management
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ControlMessage {
    // Authentication Messages
    Authenticate {
        token: String,
        client_version: Cow<'static, str>,
    },
    AuthResponse {
        success: bool,
        user_id: Option<UserId>,
        error: Option<Cow<'static, str>>,
    },
    JoinChannel {
        channel_id: ChannelId,
    },
    LeaveChannel {
        channel_id: ChannelId,
    },
    ChannelJoined {
        channel_id: ChannelId,
        users: Vec<UserId>,
    },
    ChannelLeft {
        channel_id: ChannelId,
    },
    UserJoined {
        user_id: UserId,
        username: String,
        channel_id: Option<ChannelId>,
    },
    UserLeft {
        user_id: UserId,
    },
    UserChangedChannel {
        user_id: UserId,
        from_channel: Option<ChannelId>,
        to_channel: Option<ChannelId>,
    },
    // Server State
    ServerInfo {
        name: String,
        version: Cow<'static, str>,
        user_count: u32,
        channel_count: u32,
    },
    Error {
        code: Cow<'static, str>,
        message: String,
    },

    Ping,
    Pong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = ControlMessage::Authenticate {
            token: "discord_token_123".to_string(),
            client_version: Cow::Borrowed("1.0.0"),
        };

        // Serialize to JSON
        //let json = serde_json::to_string(&msg).unwrap();
        let json = serde_json::to_string(&msg).unwrap();

        // Deserialize back to ControlMessage
        let parsed: ControlMessage = serde_json::from_str(&json).unwrap();

        // Verify the parsed message matches the original
        match parsed {
            ControlMessage::Authenticate {
                token,
                client_version,
            } => {
                assert_eq!(token, "discord_token_123");
                assert_eq!(client_version, Cow::Borrowed("1.0.0"));
            }
            _ => panic!("Wrong message type!"),
        }
    }

    #[test]
    fn test_message_with_hmac() {
        // Create a test message.
        let msg = ControlMessage::JoinChannel { channel_id: 42 };

        // Create a session key
        let key = HmacKey::from_bytes(b"test_session_key_32_bytes_long!!");
        let message_bytes = serde_json::to_vec(&msg).unwrap();
        let hmac = generate_hmac(&key, &message_bytes);

        let framed = FramedMessage {
            payload: message_bytes.clone(),
            hmac: hmac.clone(),
        };

        // Validate HMAC
        assert!(validate_hmac(&key, &framed.payload, &framed.hmac));

        // Parse the payload back to ControlMessage
        let parsed: ControlMessage = serde_json::from_slice(&framed.payload).unwrap();
        match parsed {
            ControlMessage::JoinChannel { channel_id } => {
                assert_eq!(channel_id, 42);
            }
            _ => todo!(),
        }
    }
}
