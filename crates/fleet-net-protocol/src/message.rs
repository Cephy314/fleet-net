use crate::hmac::{generate_hmac, validate_hmac, HmacKey};
use fleet_net_common::error::FleetNetError;
use fleet_net_common::types::{
    AuthRequest, ErrorResponse, JoinChannelRequest, ServerInfo, ServerState, UserChannelChange,
    UserStateChange, VoiceChannelState,
};
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
    Authenticate(AuthRequest),
    // Server State
    ServerInfo(ServerInfo),
    ServerState(ServerState),
    VoiceChannelState(VoiceChannelState),

    // User Actions
    UserStateChange(UserStateChange),
    UserChannelChange(UserChannelChange),
    JoinChannelRequest(JoinChannelRequest),

    // Error Message
    ErrorResponse(ErrorResponse),
    Ping,
    Pong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg =
            ControlMessage::Authenticate(AuthRequest::new("".to_string(), Default::default()));

        // Serialize to JSON
        //let json = serde_json::to_string(&msg).unwrap();
        let json = serde_json::to_string(&msg).unwrap();

        // Deserialize back to ControlMessage
        let parsed: ControlMessage = serde_json::from_str(&json).unwrap();

        // Verify the parsed message matches the original
        match parsed {
            ControlMessage::Authenticate(info) => {
                assert_eq!(info.token, "discord_token_123");
                assert_eq!(info.client_version, Cow::Borrowed("1.0.0"));
            }
            _ => panic!("Wrong message type!"),
        }
    }

    #[test]
    fn test_message_with_hmac() {
        // Create a test message.
        let msg = ControlMessage::JoinChannelRequest(JoinChannelRequest::new(42));

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
            ControlMessage::JoinChannelRequest(info) => {
                assert_eq!(info.channel_id, 42);
            }
            _ => todo!(),
        }
    }
}
