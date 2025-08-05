//! Role management for Fleet Net.
//!
//! This module provides role-based access control with Discord integration.
//! Roles can be mapped from Discord roles and have priority-based resolution.

use serde::{Deserialize, Serialize};

/// Represents a role in the Fleet Net system with associated permissions.
///
/// Roles provide a way to group permissions and assign them to users.
/// Each role can be mapped to multiple Discord roles, allowing seamless
/// integration with Discord's role system.
///
/// # Priority System
///
/// Roles have a priority value where lower numbers indicate higher priority.
/// When a user has multiple roles, the highest priority role's permissions
/// take precedence for channel-specific overrides.
///
/// # Examples
///
/// ```
/// use fleet_net_common::role::Role;
/// use fleet_net_common::permission::permissions;
///
/// let admin_role = Role::new("admin".to_string(), "Administrator".to_string())
///     .with_permissions(permissions::ADMINISTRATOR)
///     .with_priority(1)
///     .with_discord_roles(vec!["discord_admin_id".to_string()]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// Unique identifier for this role.
    pub id: String,

    /// Human-readable name for the role.
    pub name: String,

    /// Bitmask of permissions granted by this role.
    /// Uses the same permission constants as PermissionSet.
    pub permissions: u64,

    /// List of Discord role IDs that map to this Fleet Net role.
    /// Users with any of these Discord roles will be granted this role.
    pub discord_role_ids: Vec<String>,

    /// Priority for role resolution.
    /// Lower values have higher priority (1 is highest priority).
    /// Used when determining which role's channel overrides apply.
    pub priority: u32,
}

impl Role {
    /// Creates a new Role with the given ID and name.
    ///
    /// The role starts with no permissions, no Discord role mappings,
    /// and priority 0 (which should be updated using the builder methods).
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the role
    /// * `name` - Human-readable name for the role
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::role::Role;
    ///
    /// let role = Role::new("moderator".to_string(), "Moderator".to_string());
    /// assert_eq!(role.id, "moderator");
    /// assert_eq!(role.permissions, 0);
    /// ```
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            permissions: 0,
            discord_role_ids: Vec::new(),
            priority: 0,
        }
    }

    /// Sets the permissions for this role (builder pattern).
    ///
    /// # Arguments
    ///
    /// * `permissions` - Bitmask of permissions to grant
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::role::Role;
    /// use fleet_net_common::permission::permissions;
    ///
    /// let role = Role::new("mod".to_string(), "Moderator".to_string())
    ///     .with_permissions(permissions::KICK_USERS | permissions::MUTE_USERS);
    /// ```
    pub fn with_permissions(mut self, permissions: u64) -> Self {
        self.permissions = permissions;
        self
    }

    /// Sets the Discord role IDs that map to this role (builder pattern).
    ///
    /// # Arguments
    ///
    /// * `role_ids` - List of Discord role IDs
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::role::Role;
    ///
    /// let role = Role::new("vip".to_string(), "VIP Member".to_string())
    ///     .with_discord_roles(vec![
    ///         "discord_vip_id".to_string(),
    ///         "discord_premium_id".to_string()
    ///     ]);
    /// ```
    pub fn with_discord_roles(mut self, role_ids: Vec<String>) -> Self {
        self.discord_role_ids = role_ids;
        self
    }

    /// Sets the priority for this role (builder pattern).
    ///
    /// Lower values indicate higher priority. For example:
    /// - Priority 1: Highest priority (e.g., Admin)
    /// - Priority 5: Medium priority (e.g., Moderator)
    /// - Priority 10: Lower priority (e.g., Member)
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority value (lower = higher priority)
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::role::Role;
    ///
    /// let admin = Role::new("admin".to_string(), "Admin".to_string())
    ///     .with_priority(1);  // Highest priority
    /// ```
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority as u32;
        self
    }

    /// Adds a Discord role ID to this role's mappings.
    ///
    /// Duplicate role IDs are automatically prevented.
    ///
    /// # Arguments
    ///
    /// * `role_id` - Discord role ID to add
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::role::Role;
    ///
    /// let mut role = Role::new("member".to_string(), "Member".to_string());
    /// role.add_discord_role("discord_member_id".to_string());
    /// role.add_discord_role("discord_member_id".to_string()); // Duplicate, won't be added
    /// assert_eq!(role.discord_role_ids.len(), 1);
    /// ```
    pub fn add_discord_role(&mut self, role_id: String) {
        // Only add if not already present to prevent duplicates
        if !self.discord_role_ids.contains(&role_id) {
            self.discord_role_ids.push(role_id);
        }
    }

    /// Removes a Discord role ID from this role's mappings.
    ///
    /// # Arguments
    ///
    /// * `role_id` - Discord role ID to remove
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::role::Role;
    ///
    /// let mut role = Role::new("member".to_string(), "Member".to_string())
    ///     .with_discord_roles(vec!["role1".to_string(), "role2".to_string()]);
    ///
    /// role.remove_discord_role("role1");
    /// assert_eq!(role.discord_role_ids.len(), 1);
    /// assert!(!role.discord_role_ids.contains(&"role1".to_string()));
    /// ```
    pub fn remove_discord_role(&mut self, role_id: &str) {
        // Retain all role IDs except the one to remove
        self.discord_role_ids.retain(|id| id != role_id);
    }

    /// Checks if this role matches any of the provided Discord role IDs.
    ///
    /// This is used to determine if a user with certain Discord roles
    /// should be granted this Fleet Net role.
    ///
    /// # Arguments
    ///
    /// * `role_ids` - List of Discord role IDs to check against
    ///
    /// # Returns
    ///
    /// `true` if any Discord role ID matches this role's mappings
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::role::Role;
    ///
    /// let role = Role::new("vip".to_string(), "VIP".to_string())
    ///     .with_discord_roles(vec!["vip_id".to_string(), "premium_id".to_string()]);
    ///
    /// assert!(role.matches_discord_roles(&["vip_id".to_string(), "other_id".to_string()]));
    /// assert!(!role.matches_discord_roles(&["other_id".to_string()]));
    /// ```
    pub fn matches_discord_roles(&self, role_ids: &[String]) -> bool {
        // Check if any of the Discord role IDs match
        self.discord_role_ids.iter().any(|id| role_ids.contains(id))
    }
}

/// Computes the combined permissions for a user based on their Discord roles.
///
/// **Deprecated**: This function uses simple OR-based permission combination.
/// The new channel permission system provides more sophisticated priority-based
/// resolution. Use `Channel::compute_user_permissions` instead.
///
/// # Arguments
///
/// * `roles` - All available Fleet Net roles
/// * `user_discord_roles` - The user's Discord role IDs
///
/// # Returns
///
/// Combined permission bitmask from all matching roles
///
/// # Algorithm
///
/// 1. Find all Fleet Net roles that match the user's Discord roles
/// 2. Sort by priority (lower value = higher priority)
/// 3. Combine all permissions using bitwise OR
#[deprecated(note = "Use Channel::compute_user_permissions for proper priority-based resolution")]
pub fn compute_permissions(roles: &[Role], user_discord_roles: &[String]) -> u64 {
    let mut applicable_roles: Vec<&Role> = roles
        .iter()
        .filter(|role| role.matches_discord_roles(user_discord_roles))
        .collect();

    // Sort roles by priority (lower value means higher priority)
    applicable_roles.sort_by_key(|role| role.priority);

    // Combine permissions from all applicable roles using bitwise OR
    applicable_roles
        .iter()
        .fold(0u64, |acc, role| acc | role.permissions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_creation() {
        let role = Role::new("admin_role".to_string(), "Administrator".to_string());

        assert_eq!(role.id, "admin_role");
        assert_eq!(role.name, "Administrator");
        assert_eq!(role.permissions, 0);
        assert!(role.discord_role_ids.is_empty());
        assert_eq!(role.priority, 0);
    }

    #[test]
    fn test_role_builder_pattern() {
        let role = Role::new("mod_role".to_string(), "Moderator".to_string())
            .with_permissions(0b1111) // Some Permission bits
            .with_discord_roles(vec![
                "discord_mod_1".to_string(),
                "discord_mod_2".to_string(),
            ])
            .with_priority(10);

        assert_eq!(role.permissions, 0b1111);
        assert_eq!(role.discord_role_ids.len(), 2);
        assert_eq!(role.priority, 10);
    }

    #[test]
    fn test_add_remove_discord_role() {
        let mut role = Role::new("test_role".to_string(), "Test Role".to_string());
        role.add_discord_role("discord_role_1".to_string());
        role.add_discord_role("discord_role_2".to_string());
        // duplicate add should not change the count
        role.add_discord_role("discord_role_1".to_string());

        assert_eq!(role.discord_role_ids.len(), 2);
        assert!(role
            .discord_role_ids
            .contains(&"discord_role_1".to_string()));
        assert!(role
            .discord_role_ids
            .contains(&"discord_role_2".to_string()));

        role.remove_discord_role("discord_role_1");
        assert_eq!(role.discord_role_ids.len(), 1);
        assert!(!role
            .discord_role_ids
            .contains(&"discord_role_1".to_string()));
        assert!(role
            .discord_role_ids
            .contains(&"discord_role_2".to_string()));
    }

    #[test]
    fn test_role_matches_discord_roles() {
        let role =
            Role::new("test_role".to_string(), "Test Role".to_string()).with_discord_roles(vec![
                "discord_role_1".to_string(),
                "discord_role_2".to_string(),
            ]);

        // Should match if any role matches
        assert!(role.matches_discord_roles(&["discord_role_1".to_string()]));
        assert!(role.matches_discord_roles(&["discord_role_2".to_string()]));
        assert!(role
            .matches_discord_roles(&["discord_role_3".to_string(), "discord_role_1".to_string()]));

        // Should not match if no roles match
        assert!(!role.matches_discord_roles(&["discord_role_3".to_string()]));

        // Should not match if empty
        assert!(!role.matches_discord_roles(&[]));
    }
}
