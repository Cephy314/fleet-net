//! Core type definitions for Fleet Net.
//!
//! This module contains fundamental type aliases used throughout the Fleet Net system.
//! These types provide semantic meaning and consistent sizing for network identifiers.

use crate::error::FleetNetError;
use crate::{Channel, SessionState};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Unique identifier for users in the Fleet Net system.
///
/// Using a 16-bit unsigned integer provides:
/// - Fast lookup performance in collections
/// - Compact network packet size
/// - Support for up to 65,535 concurrent users per server
///
/// # Examples
///
/// ```
/// use fleet_net_common::types::UserId;
///
/// let user_id: UserId = 42;
/// assert_eq!(user_id, 42u16);
/// ```
pub type UserId = u16;

/// Unique identifier for channels in the Fleet Net system.
///
/// Using a 16-bit unsigned integer provides:
/// - Fast lookup performance in channel collections
/// - Compact representation in network packets
/// - Support for up to 65,535 channels per server
///
/// Channels can be voice channels, radio channels, or categories.
///
/// # Examples
///
/// ```
/// use fleet_net_common::types::ChannelId;
///
/// let voice_channel: ChannelId = 1;
/// let category_channel: ChannelId = 100;
/// ```
pub type ChannelId = u16;

// ============================================================
// Shared Message Structures
// ============================================================

/// Server information.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: Cow<'static, str>,
    pub user_count: u16,
    pub channel_count: u16,
}

impl ServerInfo {
    pub fn new(
        name: String,
        version: Cow<'static, str>,
        user_count: u16,
        channel_count: u16,
    ) -> Self {
        Self {
            name,
            version,
            user_count,
            channel_count,
        }
    }
    pub fn validate(&self) -> Result<(), FleetNetError> {
        if self.name.is_empty() {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "Server Name cannot be empty",
            )));
        }
        if self.version.is_empty() {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "Server Version cannot be empty",
            )));
        }
        Ok(())
    }
}

/// Complete server state information.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerState {
    pub users: Vec<UserInfo>,
    pub channels: Vec<VoiceChannelState>,
}

impl ServerState {
    pub fn new(users: Vec<UserInfo>, channels: Vec<VoiceChannelState>) -> Self {
        Self { users, channels }
    }
    pub fn validate(&self) -> Result<(), FleetNetError> {
        // Ensure users are of type UserInfo
        for user in &self.users {
            user.validate()?;
        }
        // Ensure channels are of type VoiceChannelState
        for channel in &self.channels {
            channel.validate()?;
        }

        Ok(())
    }
}

/// Channel state information.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChannelState {
    pub channel: Channel,
    pub users: Vec<UserId>,
}

impl VoiceChannelState {
    pub fn new(channel: Channel, users: Vec<UserId>) -> Self {
        Self { channel, users }
    }
    pub fn validate(&self) -> Result<(), FleetNetError> {
        self.channel.validate()?;
        Ok(())
    }
}

/// Authentication request from client.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub token: String,
    pub client_version: Cow<'static, str>,
}

impl AuthRequest {
    pub fn new(token: String, client_version: Cow<'static, str>) -> Self {
        Self {
            token,
            client_version,
        }
    }
    pub fn validate(&self) -> Result<(), FleetNetError> {
        if self.token.is_empty() {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "Auth token cannot be empty",
            )));
        }
        if self.client_version.is_empty() {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "Client version cannot be empty",
            )));
        }
        Ok(())
    }
}

/// User information for connected users
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: UserId,
    pub username: String,
    pub avatar_url: Option<String>,
    pub channel_id: Option<ChannelId>,
}

impl UserInfo {
    pub fn new(
        user_id: UserId,
        username: String,
        avatar_url: Option<String>,
        channel_id: Option<ChannelId>,
    ) -> Self {
        Self {
            user_id,
            username,
            avatar_url,
            channel_id,
        }
    }
    pub fn validate(&self) -> Result<(), FleetNetError> {
        if self.username.is_empty() {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "Username cannot be empty",
            )));
        }
        Ok(())
    }
}

/// User state change notification
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStateChange {
    pub user_id: UserId,
    pub session_state: SessionState,
    pub is_self_muted: bool,
    pub is_self_deafened: bool,
    pub is_server_muted: bool,
    pub is_server_deafened: bool,
}

impl UserStateChange {
    pub fn new(
        user_id: UserId,
        session_state: SessionState,
        is_self_muted: bool,
        is_self_deafened: bool,
        is_server_muted: bool,
        is_server_deafened: bool,
    ) -> Self {
        Self {
            user_id,
            session_state,
            is_self_muted,
            is_self_deafened,
            is_server_muted,
            is_server_deafened,
        }
    }
    pub fn validate(&self) -> Result<(), FleetNetError> {
        if (self.user_id == 0) {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "User ID cannot be zero",
            )));
        }
        Ok(())
    }
}

/// User channel change notification
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserChannelChange {
    pub user_id: UserId,
    pub from_channel: Option<ChannelId>,
    pub to_channel: Option<ChannelId>,
}

impl UserChannelChange {
    pub fn new(
        user_id: UserId,
        from_channel: Option<ChannelId>,
        to_channel: Option<ChannelId>,
    ) -> Self {
        Self {
            user_id,
            from_channel,
            to_channel,
        }
    }
    pub fn validate(&self) -> Result<(), FleetNetError> {
        if self.user_id == 0 {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "User ID cannot be zero",
            )));
        }

        // Requires either from_channel or to_channel to be Some
        if self.from_channel.is_none() && self.to_channel.is_none() {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "Either from_channel or to_channel must be specified",
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JoinChannelRequest {
    pub channel_id: ChannelId,
}

impl JoinChannelRequest {
    pub fn new(channel_id: ChannelId) -> Self {
        Self { channel_id }
    }
    pub fn validate(&self) -> Result<(), FleetNetError> {
        if self.channel_id == 0 {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "Channel ID cannot be zero",
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: Cow<'static, str>,
    pub message: String,
}
