use crate::permission::PermissionSet;
use crate::types::ChannelId;
use crate::user::User;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::time::Instant;

/// Session information for a user, including their current channel and connection status.
#[derive(Debug)]
pub struct Session {
    pub id: String,              // Unique session ID for the user.
    pub user: User,              // The user associated with this session.
    pub socket_addr: SocketAddr, // The socket address of the user's connection.
    pub connected_at: Instant,   // Timestamp when the session was created.
    pub last_active: Instant,    // Last time the user was active in this session.
    pub state: SessionState,     // Current state of the session.
    pub current_channel: Option<ChannelId>,
    pub subscribed_channels: HashSet<ChannelId>, // Channels the user is subscribed to.
    pub permission: PermissionSet,               // Computed from roles at connection time.
    pub auth_token: String,                      // JWT token for authentication.
    pub client_version: String,                  // Version of the client software.
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    Authenticating,
    Active,
    Away, // User set themselves away (deafened/muted).
    Disconnecting,
}

impl Session {
    // update the users activity time.
    pub fn update_activity(&mut self) {
        let now = Instant::now();
        self.last_active = now;
    }

    // Has the user been idle longer than the given duration in seconds
    pub fn is_idle(&self, duration: u64) -> bool {
        // Get idle duration by subtracting last_active from now
        let dur = Instant::now().duration_since(self.last_active);

        dur.as_secs() >= duration
    }
}
