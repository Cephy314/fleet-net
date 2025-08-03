#[derive(Debug, Clone)]
pub struct PermissionSet {
    permissions: u64, // Bitmask representing the user's permissions.
}

impl PermissionSet {
    pub fn new() -> Self {
        Self { permissions: 0 }
    }

    pub fn from_bits(permissions: u64) -> Self {
        Self { permissions }
    }

    pub fn add(&mut self, permission: u64) {
        self.permissions |= permission;
    }

    pub fn remove(&mut self, permission: u64) {
        self.permissions &= !permission;
    }

    pub fn has(&self, permission: u64) -> bool {
        // Administrator permission overrides all others.
        if self.permissions & permissions::ADMINISTRATOR != 0 {
            return true;
        }

        self.permissions & permission != 0
    }

    pub fn has_all(&self, permissions: &[u64]) -> bool {
        permissions.iter().all(|&p| self.has(p))
    }

    pub fn has_any(&self, permissions: &[u64]) -> bool {
        permissions.iter().any(|&p| self.has(p))
    }
}

impl Default for PermissionSet {
    fn default() -> Self {
        Self::new()
    }
}

pub mod permissions {
    pub const CONNECT: u64 = 1 << 0; // Can connect to the server.
    pub const SPEAK: u64 = 1 << 1; // Can send
    pub const LISTEN: u64 = 1 << 2; // Can listen to audio.
    pub const MOVE_USERS: u64 = 1 << 3; // Can move users between channels.
    pub const MUTE_USERS: u64 = 1 << 4; // Can mute users.
    pub const KICK_USERS: u64 = 1 << 5; // Can kick users from the server.
    pub const BAN_USERS: u64 = 1 << 6; // Can ban users
    pub const MANAGE_CHANNELS: u64 = 1 << 7; // Can create, delete, or modify channels.
    pub const MANAGE_ROLES: u64 = 1 << 8; // Can create, delete, or modify roles.
    pub const ADMINISTRATOR: u64 = 1 << 63; // Full administrative permissions.
}
