//! Error types for Fleet Net operations.
//!
//! This module defines the common error types used throughout the Fleet Net system.
//! All errors implement the standard `Error` trait and provide human-readable messages.

use std::borrow::Cow;
use thiserror::Error;

/// Central error type for all Fleet Net operations.
///
/// This enum encompasses all possible errors that can occur within the Fleet Net system,
/// providing a unified error handling approach across client and server components.
///
/// # Examples
///
/// ```
/// use std::borrow::Cow;
/// use fleet_net_common::error::FleetNetError;
///
/// fn process_audio() -> Result<(), FleetNetError> {
///     // For static error messages (no allocation):
///   return Err(FleetNetError::AudioError(Cow::Borrowed("Failed to initialize audio device")));
///
///   // For dynamic error messages (when you need format!):
///   Err(FleetNetError::AudioError(Cow::Owned("Device not found!".to_string())))
/// }
/// ```
#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum FleetNetError {
    /// Network-related errors including connection failures and timeouts.
    ///
    /// This variant covers:
    /// - TCP/UDP socket errors
    /// - Connection timeouts
    /// - Network unreachability
    /// - Protocol violations
    #[error("Network error: {0}")]
    NetworkError(Cow<'static, str>),

    /// Audio processing and transmission errors.
    ///
    /// This variant covers:
    /// - Audio device initialization failures
    /// - Codec errors (Opus encoding/decoding)
    /// - Buffer underruns/overruns
    /// - Invalid audio format parameters
    #[error("Audio error: {0}")]
    AudioError(Cow<'static, str>),

    /// Packet processing and validation errors.
    ///
    /// This variant covers:
    /// - Malformed packet structures
    /// - Invalid packet headers
    /// - HMAC verification failures
    /// - Sequence number violations
    #[error("Packet error: {0}")]
    PacketError(Cow<'static, str>),

    #[error("JSON error: {0}")]
    JsonError(Cow<'static, str>),

    /// Authentication and authorization failures.
    ///
    /// This variant covers:
    /// - Invalid credentials
    /// - Expired JWT tokens
    /// - Discord OAuth failures
    /// - Session validation errors
    #[error("Authentication error: {0}")]
    AuthError(Cow<'static, str>),

    /// Permission and access control violations.
    ///
    /// This variant covers:
    /// - Insufficient permissions for channel operations
    /// - Role-based access denials
    /// - Administrative action restrictions
    /// - Channel subscription denials
    #[error("Permission error: {0}")]
    PermissionError(Cow<'static, str>),

    #[error("File systemerror: {0}")]
    FileSystemError(Cow<'static, str>),

    #[error("Encryption error: {0}")]
    EncryptionError(Cow<'static, str>),
}

impl From<serde_json::Error> for FleetNetError {
    fn from(err: serde_json::Error) -> Self {
        FleetNetError::JsonError(Cow::Owned(err.to_string()))
    }
}

impl From<std::io::Error> for FleetNetError {
    fn from(err: std::io::Error) -> Self {
        FleetNetError::NetworkError(Cow::Owned(err.to_string()))
    }
}
