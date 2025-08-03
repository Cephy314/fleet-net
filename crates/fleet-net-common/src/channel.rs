use crate::types::ChannelId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: ChannelId,               // Unique identifier for the channel.
    pub name: String,                // Name of the channel.
    pub description: Option<String>, // Description of the channel.
    pub channel_type: ChannelType,   // Type of the channel (e.g., voice, radio, category).
    pub role_permissions: HashMap<String, ChannelPermissions>, // Permissions for roles in this channel.
    pub user_overrides: HashMap<String, ChannelPermissions>, // User-specific permission overrides.
    pub position: u32,                                       // Position of the channel in the list.
    pub parent_id: Option<ChannelId>, // Optional parent channel ID for nested channels.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    Voice,    // Voice channel type.
    Radio,    // Radio channel type.
    Category, // Category for organizing channels.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPermissions {
    pub allow: u64, // Bitmask of allowed permissions.
    pub deny: u64,  // Bitmask of denied permissions.
}

impl ChannelPermissions {
    pub fn compute_permissions(&self, base_permissions: u64, permissions: u64) -> bool {
        // Explicit deny overrides everything
        if self.deny & permissions != 0 {
            return false;
        }

        // Explicit allow takes precedence over base permissions
        if self.allow & permissions != 0 {
            return true;
        }

        // Fallback to base permissions
        base_permissions & permissions != 0
    }
}

impl Channel {
    pub fn can_user_transmit(
        &self,
        user_roles: &[String],
        user_permissions: u64,
        user_id: &str,
    ) -> bool {
        use crate::permission::permissions::SPEAK;

        // Check user-specific overrides first
        if let Some(override_permissions) = self.user_overrides.get(user_id) {
            return override_permissions.compute_permissions(user_permissions, SPEAK);
        }

        // Check role permissions (Highest priority role wins)
        for role_id in user_roles {
            if let Some(role_perms) = self.role_permissions.get(role_id) {
                if role_perms.deny & SPEAK != 0 {
                    return false;
                }
                if role_perms.allow & SPEAK != 0 {
                    return true;
                }
            }
        }

        // Fallback to base permissions
        user_permissions & SPEAK != 0
    }
}
