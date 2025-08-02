use fleet_net_common::types::{ChannelId, UserId};
use serde::{Deserialize, Serialize};

// TCP Control Messages for state management
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ControlMessage {
    // Authentication Messages
    Authenticate {
        token: String,
        client_version: String,
    },
    AuthResponse {
        success: bool,
        user_id: Option<UserId>,
        error: Option<String>,
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
        version: String,
        user_count: u32,
        channel_count: u32,
    },
    Error {
        code: String,
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
            client_version: "1.0.0".to_string(),
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
                assert_eq!(client_version, "1.0.0");
            }
            _ => panic!("Wrong message type!"),
        }
    }
}
