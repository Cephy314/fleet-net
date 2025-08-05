//! User management for Fleet Net.
//!
//! This module provides user representation with Discord integration,
//! supporting both Discord-authenticated and standalone users.

use crate::types::UserId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Represents a user in the Fleet Net system.
///
/// Users can be authenticated through Discord OAuth or created locally.
/// The system tracks both Discord guild roles and local Fleet Net roles
/// for flexible permission management.
///
/// # Role System
///
/// - `guild_roles`: Direct roles from Discord guild (e.g., "Member", "VIP")
/// - `local_roles`: Fleet Net roles mapped from Discord roles (e.g., "admin", "moderator")
///
/// # Examples
///
/// ```
/// use fleet_net_common::user::{User, DiscordUser};
///
/// // Create a user with Discord authentication
/// let discord_user = DiscordUser {
///     id: "123456789".to_string(),
///     username: "JohnDoe".to_string(),
///     discriminator: None,
///     avatar: Some("avatar_hash".to_string()),
/// };
///
/// let user = User::new_with_discord(42, discord_user);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user ID assigned by the server.
    /// This is the primary identifier within Fleet Net.
    pub id: UserId,

    /// Optional Discord user information.
    /// Present when user authenticated via Discord OAuth.
    pub discord_user: Option<DiscordUser>,

    /// Roles from the Discord guild.
    /// These are the raw role names from Discord.
    pub guild_roles: Vec<String>,

    /// Server-specific roles mapped from Discord.
    /// These are Fleet Net roles computed from guild_roles.
    pub local_roles: HashSet<String>,

    /// User creation timestamp.
    /// Records when the user first connected to this Fleet Net server.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Last time the user was active.
    /// Updated when user connects or performs actions.
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

/// Discord user information obtained through OAuth.
///
/// This struct contains the subset of Discord user data
/// relevant to Fleet Net operations.
///
/// # Discriminator Changes
///
/// Discord is phasing out discriminators for new usernames.
/// The discriminator field is optional to support both:
/// - Legacy usernames: "User#1234" (has discriminator)
/// - New usernames: "user" (no discriminator)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordUser {
    /// Discord snowflake ID.
    /// This is Discord's unique identifier for the user.
    pub id: String,

    /// Discord username.
    /// The display name shown in Discord.
    pub username: String,

    /// Discord discriminator (e.g., "1234").
    /// Optional for compatibility with new Discord username system.
    pub discriminator: Option<String>,

    /// Avatar hash from Discord.
    /// Can be used to construct avatar URLs.
    pub avatar: Option<String>,
}

impl User {
    /// Creates a new User with the given Id and default values
    pub fn new(id: UserId) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            discord_user: None,
            guild_roles: vec![],
            local_roles: HashSet::new(),
            created_at: now,
            last_seen: now,
        }
    }

    /// Creates a new User with Discord authentication information.
    ///
    /// The user starts with empty role lists that should be populated
    /// after verifying Discord guild membership.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique Fleet Net user ID
    /// * `discord_user` - Discord user information from OAuth
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::user::{User, DiscordUser};
    ///
    /// let discord_user = DiscordUser {
    ///     id: "123456789".to_string(),
    ///     username: "JohnDoe".to_string(),
    ///     discriminator: Some("1234".to_string()),
    ///     avatar: None,
    /// };
    ///
    /// let user = User::new_with_discord(42, discord_user);
    /// assert!(user.discord_user.is_some());
    /// assert_eq!(user.id, 42);
    /// ```
    pub fn new_with_discord(id: UserId, discord_user: DiscordUser) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            discord_user: Some(discord_user),
            guild_roles: vec![],
            local_roles: HashSet::new(),
            created_at: now,
            last_seen: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(1);

        assert_eq!(user.id, 1);
        assert!(user.discord_user.is_none());
        assert!(user.guild_roles.is_empty());
        assert!(user.local_roles.is_empty());
    }

    #[test]
    fn test_user_with_discord() {
        let discord_user = DiscordUser {
            id: "123456789".to_string(),
            username: "TestUser".to_string(),
            discriminator: Some("1234".to_string()),
            avatar: Some("AvatarHash".to_string()),
        };

        let user = User::new_with_discord(42, discord_user.clone());

        assert_eq!(user.id, 42);
        assert!(user.discord_user.is_some());

        let discord = user.discord_user.as_ref().unwrap();
        assert_eq!(discord.id, "123456789");
        assert_eq!(discord.username, "TestUser");
        assert_eq!(discord.discriminator, Some("1234".to_string()));
        assert_eq!(discord.avatar, Some("AvatarHash".to_string()));
    }

    #[test]
    fn test_user_serialization() {
        let mut local_roles = HashSet::new();
        local_roles.insert("admin".to_string());
        local_roles.insert("moderator".to_string());

        let mut guild_roles = ["member".to_string(), "vip".to_string()];

        let mut user = User::new_with_discord(
            100,
            DiscordUser {
                id: "987654321".to_string(),
                username: "SampleUser".to_string(),
                discriminator: None,
                avatar: None,
            },
        );

        user.local_roles = local_roles.clone();
        user.guild_roles = guild_roles.to_vec();

        // Test Serialization
        let serialized = serde_json::to_string(&user).expect("Failed to serialize");
        let deserialized: User = serde_json::from_str(&serialized).expect("Failed to deserialize");

        assert_eq!(deserialized.id, user.id);
        assert_eq!(deserialized.guild_roles, user.guild_roles);
        assert_eq!(deserialized.local_roles, user.local_roles);

        // Check Discord user
        let original_discord = user.discord_user.as_ref().unwrap();
        let deserialized_discord = deserialized.discord_user.as_ref().unwrap();
        assert_eq!(deserialized_discord.id, original_discord.id);
        assert_eq!(deserialized_discord.username, original_discord.username);
    }
}
