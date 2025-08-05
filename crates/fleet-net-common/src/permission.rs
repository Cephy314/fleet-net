//! Permission system for Fleet Net.
//!
//! This module provides a flexible permission system based on bitflags,
//! allowing fine-grained control over user capabilities within the system.
//!
//! # Architecture
//!
//! The permission system uses a 64-bit bitmask where each bit represents
//! a specific permission. The ADMINISTRATOR permission (bit 63) acts as
//! a special override that grants all permissions.

/// A set of permissions represented as a bitmask.
///
/// PermissionSet provides methods to check, add, and remove permissions
/// efficiently using bitwise operations. The ADMINISTRATOR permission
/// acts as a master override.
///
/// # Examples
///
/// ```
/// use fleet_net_common::permission::{PermissionSet, permissions};
///
/// let mut perms = PermissionSet::new();
/// perms.add(permissions::SPEAK);
/// perms.add(permissions::LISTEN);
///
/// assert!(perms.has(permissions::SPEAK));
/// assert!(!perms.has(permissions::BAN_USERS));
/// ```
#[derive(Debug, Clone)]
pub struct PermissionSet {
    /// Bitmask representing the user's permissions.
    /// Each bit corresponds to a specific permission defined in the permissions module.
    permissions: u64,
}

impl PermissionSet {
    /// Creates a new empty PermissionSet with no permissions.
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::permission::PermissionSet;
    ///
    /// let perms = PermissionSet::new();
    /// assert_eq!(perms.has_any(&[]), false);
    /// ```
    pub fn new() -> Self {
        Self { permissions: 0 }
    }

    /// Creates a PermissionSet from a raw bitmask.
    ///
    /// This is useful for deserializing permissions or creating
    /// a set with multiple permissions at once.
    ///
    /// # Arguments
    ///
    /// * `permissions` - A bitmask where each bit represents a permission
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::permission::{PermissionSet, permissions};
    ///
    /// let perms = PermissionSet::from_bits(permissions::SPEAK | permissions::LISTEN);
    /// assert!(perms.has(permissions::SPEAK));
    /// assert!(perms.has(permissions::LISTEN));
    /// ```
    pub fn from_bits(permissions: u64) -> Self {
        Self { permissions }
    }

    /// Adds a permission to the set.
    ///
    /// Multiple permissions can be added by OR-ing them together.
    ///
    /// # Arguments
    ///
    /// * `permission` - The permission bit(s) to add
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::permission::{PermissionSet, permissions};
    ///
    /// let mut perms = PermissionSet::new();
    /// perms.add(permissions::CONNECT);
    /// perms.add(permissions::SPEAK | permissions::LISTEN);
    ///
    /// assert!(perms.has(permissions::CONNECT));
    /// assert!(perms.has(permissions::SPEAK));
    /// assert!(perms.has(permissions::LISTEN));
    /// ```
    pub fn add(&mut self, permission: u64) {
        // Use bitwise OR to add permission bits
        self.permissions |= permission;
    }

    /// Removes a permission from the set.
    ///
    /// Multiple permissions can be removed by OR-ing them together.
    ///
    /// # Arguments
    ///
    /// * `permission` - The permission bit(s) to remove
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::permission::{PermissionSet, permissions};
    ///
    /// let mut perms = PermissionSet::from_bits(
    ///     permissions::CONNECT | permissions::SPEAK | permissions::LISTEN
    /// );
    ///
    /// perms.remove(permissions::SPEAK);
    /// assert!(perms.has(permissions::CONNECT));
    /// assert!(!perms.has(permissions::SPEAK));
    /// assert!(perms.has(permissions::LISTEN));
    /// ```
    pub fn remove(&mut self, permission: u64) {
        // Use bitwise AND with negation to remove permission bits
        self.permissions &= !permission;
    }

    /// Checks if the set contains a specific permission.
    ///
    /// The ADMINISTRATOR permission acts as a master override,
    /// granting all permissions when present.
    ///
    /// # Arguments
    ///
    /// * `permission` - The permission to check for
    ///
    /// # Returns
    ///
    /// `true` if the permission is present or if ADMINISTRATOR is set
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::permission::{PermissionSet, permissions};
    ///
    /// let mut perms = PermissionSet::new();
    /// perms.add(permissions::SPEAK);
    /// assert!(perms.has(permissions::SPEAK));
    /// assert!(!perms.has(permissions::BAN_USERS));
    ///
    /// // Administrator overrides all
    /// perms.add(permissions::ADMINISTRATOR);
    /// assert!(perms.has(permissions::BAN_USERS));
    /// ```
    pub fn has(&self, permission: u64) -> bool {
        // Administrator permission overrides all others.
        if self.permissions & permissions::ADMINISTRATOR != 0 {
            return true;
        }

        // Check if the specific permission bit is set
        self.permissions & permission != 0
    }

    /// Checks if the set contains all of the specified permissions.
    ///
    /// # Arguments
    ///
    /// * `permissions` - A slice of permissions to check for
    ///
    /// # Returns
    ///
    /// `true` if all permissions are present (or ADMINISTRATOR is set)
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::permission::{PermissionSet, permissions};
    ///
    /// let mut perms = PermissionSet::new();
    /// perms.add(permissions::CONNECT);
    /// perms.add(permissions::SPEAK);
    /// perms.add(permissions::LISTEN);
    ///
    /// assert!(perms.has_all(&[permissions::CONNECT, permissions::SPEAK]));
    /// assert!(!perms.has_all(&[permissions::CONNECT, permissions::BAN_USERS]));
    /// ```
    pub fn has_all(&self, permissions: &[u64]) -> bool {
        // Check that every permission in the slice is present
        permissions.iter().all(|&p| self.has(p))
    }

    /// Checks if the set contains any of the specified permissions.
    ///
    /// # Arguments
    ///
    /// * `permissions` - A slice of permissions to check for
    ///
    /// # Returns
    ///
    /// `true` if at least one permission is present (or ADMINISTRATOR is set)
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::permission::{PermissionSet, permissions};
    ///
    /// let mut perms = PermissionSet::new();
    /// perms.add(permissions::SPEAK);
    ///
    /// assert!(perms.has_any(&[permissions::SPEAK, permissions::BAN_USERS]));
    /// assert!(!perms.has_any(&[permissions::MOVE_USERS, permissions::BAN_USERS]));
    /// ```
    pub fn has_any(&self, permissions: &[u64]) -> bool {
        // Check if any permission in the slice is present
        permissions.iter().any(|&p| self.has(p))
    }
}

impl Default for PermissionSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Permission bit constants for the Fleet Net system.
///
/// Each permission is represented as a single bit in a 64-bit integer,
/// allowing for efficient permission checking and combination.
///
/// # Permission Hierarchy
///
/// - ADMINISTRATOR (bit 63) - Overrides all other permissions
/// - Management permissions - Control server structure (channels, roles)
/// - Moderation permissions - Control users (kick, ban, mute)
/// - Basic permissions - Core functionality (connect, speak, listen)
pub mod permissions {
    /// Allows connecting to the server.
    /// This is the most basic permission required for any interaction.
    pub const CONNECT: u64 = 1 << 0;

    /// Allows transmitting audio in voice channels.
    /// Users without this permission can still listen but cannot speak.
    pub const SPEAK: u64 = 1 << 1;

    /// Allows receiving audio in voice channels.
    /// Users without this permission cannot hear others.
    pub const LISTEN: u64 = 1 << 2;

    /// Allows moving other users between voice channels.
    /// This is a moderation permission typically granted to staff.
    pub const MOVE_USERS: u64 = 1 << 3;

    /// Allows server-muting other users.
    /// Muted users cannot transmit audio regardless of their SPEAK permission.
    pub const MUTE_USERS: u64 = 1 << 4;

    /// Allows removing users from the server temporarily.
    /// Kicked users can rejoin unless also banned.
    pub const KICK_USERS: u64 = 1 << 5;

    /// Allows permanently banning users from the server.
    /// Banned users cannot rejoin until unbanned.
    pub const BAN_USERS: u64 = 1 << 6;

    /// Allows creating, modifying, and deleting channels.
    /// This includes changing channel permissions and properties.
    pub const MANAGE_CHANNELS: u64 = 1 << 7;

    /// Allows creating, modifying, and deleting roles.
    /// This includes changing role permissions and assignments.
    pub const MANAGE_ROLES: u64 = 1 << 8;

    /// Master permission that grants all capabilities.
    /// Users with this permission bypass all permission checks.
    pub const ADMINISTRATOR: u64 = 1 << 63;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_set_new() {
        let perms = PermissionSet::new();
        assert_eq!(perms.permissions, 0);

        // Should not have any permissions initially
        assert!(!perms.has(permissions::CONNECT));
        assert!(!perms.has(permissions::SPEAK));
    }

    #[test]
    fn test_permission_set_from_bits() {
        let perms = PermissionSet::from_bits(permissions::CONNECT | permissions::SPEAK);

        assert!(perms.has(permissions::CONNECT));
        assert!(perms.has(permissions::SPEAK));
        assert!(!perms.has(permissions::LISTEN));
    }

    #[test]
    fn test_add_remove_permissions() {
        let mut perms = PermissionSet::new();

        // Add permissions
        perms.add(permissions::CONNECT);
        perms.add(permissions::SPEAK);
        perms.add(permissions::LISTEN);

        assert!(perms.has(permissions::CONNECT));
        assert!(perms.has(permissions::SPEAK));
        assert!(perms.has(permissions::LISTEN));

        // Remove a permission
        perms.remove(permissions::SPEAK);
        assert!(perms.has(permissions::CONNECT));
        assert!(!perms.has(permissions::SPEAK));
        assert!(perms.has(permissions::LISTEN));
    }

    #[test]
    fn test_administrator_overrides_all() {
        let mut perms = PermissionSet::new();
        perms.add(permissions::ADMINISTRATOR);

        // Administrator should have all permissions
        assert!(perms.has(permissions::CONNECT));
        assert!(perms.has(permissions::SPEAK));
        assert!(perms.has(permissions::LISTEN));
        assert!(perms.has(permissions::MANAGE_CHANNELS));
        assert!(perms.has(permissions::BAN_USERS));
    }

    #[test]
    fn test_has_all_permissions() {
        let mut perms = PermissionSet::new();
        perms.add(permissions::CONNECT);
        perms.add(permissions::SPEAK);
        perms.add(permissions::LISTEN);

        // Should have all specified permissions
        assert!(perms.has_all(&[permissions::CONNECT, permissions::SPEAK]));
        assert!(perms.has_all(&[
            permissions::CONNECT,
            permissions::SPEAK,
            permissions::LISTEN
        ]));

        // Should not have all if missing one
        assert!(!perms.has_all(&[
            permissions::CONNECT,
            permissions::SPEAK,
            permissions::MOVE_USERS
        ]));
    }

    #[test]
    fn test_has_any_permissions() {
        let mut perms = PermissionSet::new();
        perms.add(permissions::CONNECT);
        perms.add(permissions::SPEAK);

        // Should have at least one permission
        assert!(perms.has_any(&[permissions::CONNECT, permissions::MOVE_USERS]));
        assert!(perms.has_any(&[permissions::LISTEN, permissions::SPEAK]));

        // Should not have any if none match
        assert!(!perms.has_any(&[
            permissions::LISTEN,
            permissions::MOVE_USERS,
            permissions::BAN_USERS
        ]));
    }
}
