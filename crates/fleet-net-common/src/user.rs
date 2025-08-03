use crate::types::UserId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,                        // Unique user id assigned by the server.
    pub discord_user: Option<DiscordUser>, // Optional Discord user information.
    pub guild_roles: Vec<String>,          // Roles from the Discord guild.
    pub local_roles: HashSet<String>,      // Server-specific roles mapped from Discord.
    pub created_at: chrono::DateTime<chrono::Utc>, // User creation timestamp.
    pub last_seen: chrono::DateTime<chrono::Utc>, // Last time the user was active.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordUser {
    pub id: String,                    // Discord snowflake ID.
    pub username: String,              // Friendly Discord username.
    pub discriminator: Option<String>, // Discord discriminator (e.g., "1234"). Compatibility with older versions.
    pub avatar: Option<String>,
}
