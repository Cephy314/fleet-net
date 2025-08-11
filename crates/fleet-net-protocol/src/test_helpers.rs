//! Test helper functions for protocol messages
//!
//! This module is only available when the `test-helpers` feature is enabled.

use crate::message::ControlMessage;
use fleet_net_common::types::{AuthRequest, JoinChannelRequest, ServerInfo, UserChannelChange};
use std::borrow::Cow;

/// Assert that a message is a ServerInfo with the expected name.
pub fn assert_is_server_info(msg: &ControlMessage, expected_name: &str) {
    match msg {
        ControlMessage::ServerInfo(info) => {
            assert_eq!(info.name, expected_name, "ServerInfo name mismatch");
        }
        other => panic!("Expected ServerInfo, got {other:?}"),
    }
}

/// Assert that a message is a ServerInfo and return its fields.
pub fn assert_server_info(msg: &ControlMessage) -> (&str, &str, u16, u16) {
    match msg {
        ControlMessage::ServerInfo(info) => (
            info.name.as_str(),
            info.version.as_ref(),
            info.user_count,
            info.channel_count,
        ),
        other => panic!("Expected ServerInfo, got {other:?}"),
    }
}

// Note: AuthResponse helper will be added when AuthResponse is implemented

/// Assert that a message is an Error with the expected message.
pub fn assert_is_error(msg: &ControlMessage, expected_error: &str) {
    match msg {
        ControlMessage::ErrorResponse(error) => {
            assert_eq!(
                error.message.as_str(),
                expected_error,
                "Error message mismatch"
            );
        }
        other => panic!("Expected ErrorResponse, got {other:?}"),
    }
}

/// Create a test Authenticate message.
pub fn create_test_authenticate(token: &str, client_version: &'static str) -> ControlMessage {
    ControlMessage::Authenticate(AuthRequest::new(
        token.to_string(),
        Cow::Borrowed(client_version),
    ))
}

/// Create a test ServerInfo message.
pub fn create_test_server_info(name: &str, version: &'static str) -> ControlMessage {
    ControlMessage::ServerInfo(ServerInfo::new(
        name.to_string(),
        Cow::Borrowed(version),
        0,
        0,
    ))
}

/// Create a test JoinChannelRequest message.
pub fn create_test_join_channel(channel_id: u16) -> ControlMessage {
    ControlMessage::JoinChannelRequest(JoinChannelRequest::new(channel_id))
}

/// Create a test UserChannelChange message.
pub fn create_test_channel_change(
    user_id: u16,
    from_channel: Option<u16>,
    to_channel: Option<u16>,
) -> ControlMessage {
    ControlMessage::UserChannelChange(UserChannelChange::new(user_id, from_channel, to_channel))
}

/// Assert that a message matches a specific type using the matches! macro.
pub fn assert_message_type<F>(msg: &ControlMessage, type_name: &str, check: F)
where
    F: FnOnce(&ControlMessage) -> bool,
{
    assert!(
        check(msg),
        "Expected message type {}, got {:?}",
        type_name,
        msg
    );
}

/// Example usage of assert_message_type for ServerInfo.
pub fn assert_is_server_info_with_user_count(msg: &ControlMessage, expected_count: u16) {
    assert_message_type(
        msg,
        "ServerInfo",
        |m| matches!(m, ControlMessage::ServerInfo(info) if info.user_count == expected_count),
    );
}
