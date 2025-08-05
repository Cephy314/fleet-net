//! Core type definitions for Fleet Net.
//!
//! This module contains fundamental type aliases used throughout the Fleet Net system.
//! These types provide semantic meaning and consistent sizing for network identifiers.

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
