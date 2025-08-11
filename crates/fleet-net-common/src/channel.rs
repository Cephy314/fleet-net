//! Channel management for Fleet Net.
//!
//! This module provides channel structures and permission management,
//! supporting voice channels, radio channels, and organizational categories.
//!
//! # Permission System
//!
//! Channels use a sophisticated permission system that:
//! - Supports role-based permission overrides
//! - Inherits permissions from parent channels
//! - Uses priority-based role resolution
//! - Allows partial permission overrides (only override specific permissions)

use crate::error::FleetNetError;
use crate::types::ChannelId;
use crate::Role;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

/// Represents a channel in the Fleet Net system.
///
/// Channels are the primary organizational unit for voice communication.
/// They can be nested under category channels and have role-specific
/// permission overrides.
///
/// # Channel Hierarchy
///
/// - Categories can contain voice/radio channels
/// - Voice/radio channels can inherit permissions from parent categories
/// - Permission inheritance is recursive up the channel tree
///
/// # Examples
///
/// ```
/// use fleet_net_common::channel::{Channel, ChannelType};
/// use std::collections::HashMap;
///
/// let channel = Channel {
///     id: 1,
///     name: "General".to_string(),
///     description: Some("Main voice channel".to_string()),
///     channel_type: ChannelType::Voice,
///     role_permissions: HashMap::new(),
///     position: 0,
///     parent_id: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// Unique identifier for the channel.
    pub id: ChannelId,

    /// Display name of the channel.
    pub name: String,

    /// Optional description for the channel.
    /// Useful for explaining channel purpose or rules.
    pub description: Option<String>,

    /// Type of channel (Voice, Radio, or Category).
    pub channel_type: ChannelType,

    /// Role-specific permission overrides for this channel.
    /// Key is the role ID, value is the permission override.
    pub role_permissions: HashMap<String, ChannelPermissions>,

    /// Position in the channel list for ordering.
    /// Lower numbers appear first.
    pub position: u32,

    /// Parent channel ID for nested channels.
    /// Voice/Radio channels can be nested under Categories.
    pub parent_id: Option<ChannelId>,
}

impl Channel {
    pub fn validate(&self) -> Result<(), FleetNetError> {
        if self.name.is_empty() {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "Channel name cannot be empty",
            )));
        }
        if self.name.len() > 100 {
            return Err(FleetNetError::ValidationError(Cow::Borrowed(
                "Channel name cannot exceed 100 characters",
            )));
        }
        if let Some(desc) = &self.description {
            if desc.len() > 500 {
                return Err(FleetNetError::ValidationError(Cow::Borrowed(
                    "Channel description cannot exceed 500 characters",
                )));
            }
        }

        if let Some(parent_id) = &self.parent_id {
            if *parent_id == self.id {
                return Err(FleetNetError::ValidationError(Cow::Borrowed(
                    "Channel cannot be its own parent",
                )));
            }
        }
        Ok(())
    }
}

/// Types of channels supported by Fleet Net.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ChannelType {
    /// Standard voice channel for real-time communication.
    /// Users can only be in one voice channel at a time.
    Voice,

    /// Radio channel for military simulation.
    /// Users can subscribe to multiple radio channels.
    Radio,

    /// Category for organizing other channels.
    /// Cannot be joined directly but can contain permissions.
    Category,
}

/// Permission overrides for a specific role in a channel.
///
/// This struct uses allow/deny bitmasks to enable fine-grained
/// permission control. Permissions can be explicitly allowed,
/// explicitly denied, or left unspecified (inherit).
///
/// # Permission Resolution
///
/// 1. Explicit deny always wins
/// 2. Explicit allow overrides inherited permissions
/// 3. Unspecified permissions inherit from parent/role
///
/// # Examples
///
/// ```
/// use fleet_net_common::channel::ChannelPermissions;
/// use fleet_net_common::permission::permissions;
///
/// // Allow speaking but deny moving users
/// let perms = ChannelPermissions {
///     allow: permissions::SPEAK,
///     deny: permissions::MOVE_USERS,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChannelPermissions {
    /// Bitmask of explicitly allowed permissions.
    /// These permissions are granted regardless of role permissions.
    pub allow: u64,

    /// Bitmask of explicitly denied permissions.
    /// These permissions are denied regardless of role permissions.
    pub deny: u64,
}

impl ChannelPermissions {
    /// Computes the final permissions from allow/deny masks.
    ///
    /// Denied permissions are removed from allowed permissions.
    /// This is used when only this channel's overrides matter.
    ///
    /// # Returns
    ///
    /// The effective permissions after applying denies to allows.
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::channel::ChannelPermissions;
    ///
    /// let perms = ChannelPermissions {
    ///     allow: 0b111,  // Allow first 3 permissions
    ///     deny:  0b010,  // Deny second permission
    /// };
    ///
    /// assert_eq!(perms.compute_final_permissions(), 0b101);
    /// ```
    pub fn compute_final_permissions(&self) -> u64 {
        // Only the allowed permissions, minus any denied ones
        self.allow & !self.deny
    }
}

impl Channel {
    /// Computes the effective permissions for a user in this channel.
    ///
    /// This method implements a sophisticated permission resolution system:
    /// 1. Checks role-specific overrides in priority order
    /// 2. Only applies permissions that haven't been set by higher priority roles
    /// 3. Recursively inherits from parent channels
    /// 4. Falls back to base role permissions
    ///
    /// # Arguments
    ///
    /// * `user_roles` - User's roles sorted by priority (highest first)
    /// * `get_parent_channel` - Callback to retrieve parent channels
    ///
    /// # Returns
    ///
    /// The final computed permission bitmask for the user.
    ///
    /// # Algorithm Details
    ///
    /// The algorithm tracks which permissions have been explicitly set
    /// to avoid lower priority roles overriding higher priority decisions.
    /// This allows partial overrides where a role only affects specific
    /// permissions without touching others.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fleet_net_common::channel::Channel;
    /// # use fleet_net_common::role::Role;
    /// # let channel: Channel = todo!();
    /// # let roles: Vec<Role> = todo!();
    /// let permissions = channel.compute_user_permissions(
    ///     &roles,
    ///     |parent_id| None  // No parent channels
    /// );
    /// ```
    pub fn compute_user_permissions(
        &self,
        user_roles: &[Role],
        get_parent_channel: impl Fn(ChannelId) -> Option<Channel>,
    ) -> u64 {
        let mut final_permissions = 0u64;
        let mut checked_permissions = 0u64;

        // Process each role in priority order (highest priority first)
        for role in user_roles {
            // Check if this channel has specific permissions for this role
            if let Some(channel_perms) = self.role_permissions.get(&role.id) {
                // Apply allows that haven't been set yet by higher priority roles
                let new_allows = channel_perms.allow & !checked_permissions;
                final_permissions |= new_allows;
                checked_permissions |= new_allows;

                // Apply denies that haven't been set yet
                let new_denies = channel_perms.deny & !checked_permissions;
                final_permissions &= !new_denies;
                checked_permissions |= new_denies;
            }
        }

        // Inherit permissions from parent channel for any unset bits
        if let Some(parent_id) = self.parent_id {
            if let Some(parent) = get_parent_channel(parent_id) {
                let parent_perms = parent.compute_user_permissions(user_roles, get_parent_channel);
                // Only use parent permissions for bits we haven't set
                final_permissions |= parent_perms & !checked_permissions;
                // Update checked_permissions to include parent's contributions
                checked_permissions |= parent_perms;
            }
        }

        // For any still unset permissions, use the highest priority role's base permissions
        if let Some(role) = user_roles.first() {
            final_permissions |= role.permissions & !checked_permissions;
        }

        final_permissions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permissions;

    fn create_test_channel(id: u16) -> Channel {
        Channel {
            id,
            name: "Test Channel".to_string(),
            description: Some("A test channel".to_string()),
            channel_type: ChannelType::Voice,
            role_permissions: HashMap::new(),
            position: 0,
            parent_id: None,
        }
    }

    #[test]
    fn test_compute_final_permissions_deny_overrides_allow() {
        let perms = ChannelPermissions {
            allow: permissions::SPEAK | permissions::LISTEN,
            deny: permissions::SPEAK,
        };

        let final_perms = perms.compute_final_permissions();

        assert_eq!(final_perms & permissions::SPEAK, 0); // Speak should be denied
        assert_ne!(final_perms & permissions::LISTEN, 0); // Listen should still be allowed
    }

    #[test]
    fn test_compute_user_permissions_uses_first_matching_role() {
        let mut channel = create_test_channel(1);

        // Setup: Admin can speak, Member cannot
        channel.role_permissions.insert(
            "admin".to_string(),
            ChannelPermissions {
                allow: permissions::SPEAK,
                deny: 0,
            },
        );
        channel.role_permissions.insert(
            "member".to_string(),
            ChannelPermissions {
                allow: 0,
                deny: permissions::SPEAK,
            },
        );

        // User with admin role (higher priority) should be able to speak.
        let admin_role = Role::new("admin".to_string(), "Admin".to_string())
            .with_permissions(0)
            .with_priority(1);
        let member_role = Role::new("member".to_string(), "Member".to_string())
            .with_permissions(permissions::SPEAK)
            .with_priority(10);

        let roles = vec![admin_role, member_role];
        let perms = channel.compute_user_permissions(&roles, |_| None);

        assert_ne!(perms & permissions::SPEAK, 0); // Admin should have permission to speak
    }

    #[test]
    fn test_compute_user_permissions_inherits_from_parents() {
        let mut parent = create_test_channel(1);

        let mut child = create_test_channel(2);
        child.parent_id = Some(parent.id);

        // Add grandparent setup
        let mut grandparent = create_test_channel(0);
        grandparent.role_permissions.insert(
            "member".to_string(),
            ChannelPermissions {
                allow: permissions::LISTEN | permissions::SPEAK,
                deny: 0,
            },
        );

        // Make parent's parent_id = Some(0)
        parent.parent_id = Some(0);

        let member_role = Role::new("member".to_string(), "Member".to_string()).with_permissions(0);

        let roles = [member_role];
        let perms = child.compute_user_permissions(&roles, |id| match id {
            0 => Some(grandparent.clone()),
            1 => Some(parent.clone()),
            _ => None,
        });

        // Should inherit SPEAK and LISTEN from parent
        assert_ne!(perms & permissions::SPEAK, 0);
        assert_ne!(perms & permissions::LISTEN, 0);
    }

    #[test]
    fn test_compute_user_permissions_falls_back_to_role_base() {
        let channel = create_test_channel(1);

        let role = Role::new("member".to_string(), "Member".to_string())
            .with_permissions(permissions::SPEAK | permissions::CONNECT); // Only has LISTEN permission

        let roles = [role];
        let perms = channel.compute_user_permissions(&roles, |_| None);

        assert_eq!(perms, permissions::SPEAK | permissions::CONNECT); // Should return base role permissions
    }

    #[test]
    fn test_compute_user_permissions_handles_no_roles() {
        let channel = create_test_channel(1);

        // let role = Role::new("member".to_string(), "Member".to_string())
        //     .with_permissions(permissions::SPEAK | permissions::CONNECT); // Only has LISTEN permission

        let roles: Vec<Role> = vec![];
        let perms = channel.compute_user_permissions(&roles, |_| None);

        assert_eq!(perms, 0); // No Roles = No Permissions
    }

    #[test]
    fn test_banned_role_overrides_member_role() {
        let mut channel = create_test_channel(1);

        // Member can speak
        channel.role_permissions.insert(
            "member".to_string(),
            ChannelPermissions {
                allow: permissions::SPEAK | permissions::LISTEN,
                deny: 0,
            },
        );

        channel.role_permissions.insert(
            "banned".to_string(),
            ChannelPermissions {
                allow: 0,
                deny: permissions::SPEAK, // Banned role denies speaking and listening
            },
        );

        // Roles sorted by priority - banned has higher priority than member.

        let banned_role = Role::new("banned".to_string(), "Banned".to_string())
            .with_permissions(0)
            .with_priority(10);

        let member_role = Role::new("member".to_string(), "Member".to_string())
            .with_permissions(permissions::SPEAK)
            .with_priority(5);

        let roles = [banned_role, member_role];

        let perms = channel.compute_user_permissions(&roles, |_| None);

        // Banned role should override member role, so no permissions should be granted
        assert_eq!(perms & permissions::SPEAK, 0); // Speak should be denied
        assert_ne!(perms & permissions::LISTEN, 0); // Listen should be allowed
    }

    #[test]
    fn test_admin_role_overrides_banned_role() {
        let mut channel = create_test_channel(1);

        // Setup channel permissions for admin and banned roles.
        channel.role_permissions.insert(
            "admin".to_string(),
            ChannelPermissions {
                allow: permissions::SPEAK | permissions::LISTEN | permissions::CONNECT,
                deny: 0,
            },
        );
        channel.role_permissions.insert(
            "banned".to_string(),
            ChannelPermissions {
                allow: permissions::CONNECT,
                deny: permissions::SPEAK | permissions::LISTEN, // Banned role denies
            },
        );

        // Admin has higher priority than banned.
        let admin_role = Role::new("admin".to_string(), "Admin".to_string())
            .with_permissions(0)
            .with_priority(10);

        let banned_role = Role::new("banned".to_string(), "Banned".to_string())
            .with_permissions(0)
            .with_priority(5);

        // User has both admin and banned roles.
        let roles = [admin_role, banned_role];
        let perms = channel.compute_user_permissions(&roles, |_| None);

        assert_ne!(perms & permissions::SPEAK, 0);
        assert_ne!(perms & permissions::LISTEN, 0);
        assert_ne!(perms & permissions::CONNECT, 0); // Admin should have all permissions, even if banned.
    }
}
