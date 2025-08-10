//! Test helper functions for protocol messages
//!
//! This module is only available when the `test-helpers` feature is enabled.

use crate::message::ControlMessage;
use std::borrow::Cow;

/// Assert that a message is a ServerInfo with the expected name.
pub fn assert_is_server_info(msg: &ControlMessage, expected_name: &str) {
    match msg {
        ControlMessage::ServerInfo { name, .. } => {
            assert_eq!(name, expected_name, "ServerInfo name mismatch");
        }
        other => panic!("Expected ServerInfo, got {other:?}"),
    }
}

/// Assert that a message is a ServerInfo and return its fields.
pub fn assert_server_info(msg: &ControlMessage) -> (&str, &str, u32, u32) {
    match msg {
        ControlMessage::ServerInfo {
            name,
            version,
            user_count,
            channel_count,
        } => (name.as_str(), version.as_ref(), *user_count, *channel_count),
        other => panic!("Expected ServerInfo, got {other:?}"),
    }
}

/// Assert that a message is an AuthResponse with success.
pub fn assert_auth_success(msg: &ControlMessage) {
    match msg {
        ControlMessage::AuthResponse { success, .. } => {
            assert!(*success, "Expected successful auth response");
        }
        other => panic!("Expected AuthResponse, got {other:?}"),
    }
}

/// Assert that a message is an Error with the expected message.
pub fn assert_is_error(msg: &ControlMessage, expected_error: &str) {
    match msg {
        ControlMessage::Error { message, .. } => {
            assert_eq!(message.as_str(), expected_error, "Error message mismatch");
        }
        other => panic!("Expected Error, got {other:?}"),
    }
}

/// Create a test Authenticate message.
pub fn create_test_authenticate(token: &str, client_version: &'static str) -> ControlMessage {
    ControlMessage::Authenticate {
        token: token.to_string(),
        client_version: Cow::Borrowed(client_version),
    }
}

/// Create a test ServerInfo message.
pub fn create_test_server_info(name: &str, version: &'static str) -> ControlMessage {
    ControlMessage::ServerInfo {
        name: name.to_string(),
        version: Cow::Borrowed(version),
        user_count: 0,
        channel_count: 0,
    }
}
