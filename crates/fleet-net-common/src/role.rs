use serde::{Deserialize, Serialize};

// Role mapping from Discord roles to local permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub permissions: u64,              // Bitmask of permissions
    pub discord_role_ids: Vec<String>, // List of Discord role IDs that map to this local role
    pub priority: u32, // Priority for role resolution, lower values have higher priority
}

impl Role {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            permissions: 0,
            discord_role_ids: Vec::new(),
            priority: 0,
        }
    }

    pub fn with_permissions(mut self, permissions: u64) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_discord_roles(mut self, role_ids: Vec<String>) -> Self {
        self.discord_role_ids = role_ids;
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority as u32;
        self
    }

    pub fn add_discord_role(&mut self, role_id: String) {
        if !self.discord_role_ids.contains(&role_id) {
            self.discord_role_ids.push(role_id);
        }
    }

    pub fn remove_discord_role(&mut self, role_id: &str) {
        self.discord_role_ids.retain(|id| id != role_id);
    }

    pub fn matches_discord_roles(&self, role_ids: &[String]) -> bool {
        // Check if any of the Discord role IDs match
        self.discord_role_ids.iter().any(|id| role_ids.contains(id))
    }
}

// Helper function to compute permissions from multiple roles
#[deprecated]
pub fn compute_permissions(roles: &[Role], user_discord_roles: &[String]) -> u64 {
    let mut applicable_roles: Vec<&Role> = roles
        .iter()
        .filter(|role| role.matches_discord_roles(user_discord_roles))
        .collect();

    // Sort roles by priority (lower value means higher priority)
    applicable_roles.sort_by_key(|role| role.priority);

    // Combine permissions from all applicable roles
    applicable_roles
        .iter()
        .fold(0u64, |acc, role| acc | role.permissions)
}
