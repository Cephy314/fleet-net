//! Session management for Fleet Net users.
//!
//! This module handles user sessions, tracking connection state,
//! channel subscriptions, and user activity.

use crate::permission::PermissionSet;
use crate::types::ChannelId;
use crate::user::User;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::SocketAddr;
use std::time::Instant;

/// Represents an active user session in the Fleet Net system.
///
/// A session tracks all the runtime state for a connected user,
/// including their current channel, subscribed channels, permissions,
/// and connection metadata.
///
/// # Lifecycle
///
/// 1. Session created when user connects (Authenticating state)
/// 2. After successful auth, transitions to Active state
/// 3. User can set themselves Away (self-muted/deafened)
/// 4. Session ends when user disconnects (Disconnecting state)
///
/// # Examples
///
/// ```no_run
/// use fleet_net_common::session::{Session, SessionState};
/// use fleet_net_common::user::User;
/// use fleet_net_common::permission::PermissionSet;
/// use std::net::SocketAddr;
/// use std::time::Instant;
///
/// let session = Session {
///     id: "session_123".to_string(),
///     user: User::new(42),
///     socket_addr: "127.0.0.1:8080".parse().unwrap(),
///     connected_at: Instant::now(),
///     last_active: Instant::now(),
///     state: SessionState::Active,
///     current_channel: None,
///     subscribed_channels: Default::default(),
///     permission: PermissionSet::new(),
///     auth_token: "jwt_token".to_string(),
///     client_version: "1.0.0".to_string(),
/// };
/// ```
#[derive(Debug)]
pub struct Session {
    /// Unique session identifier (typically a UUID).
    pub id: String,

    /// The user associated with this session.
    pub user: User,

    /// The socket address of the user's connection.
    /// Used for network communication and logging.
    pub socket_addr: SocketAddr,

    /// Timestamp when the session was created.
    /// Used for session duration tracking.
    pub connected_at: Instant,

    /// Last time the user was active in this session.
    /// Updated on any user action (speaking, channel change, etc.).
    pub last_active: Instant,

    /// Current state of the session.
    pub state: SessionState,

    /// The channel the user is currently connected to.
    /// None if the user is in the lobby or not in a voice channel.
    pub current_channel: Option<ChannelId>,

    /// Channels the user is subscribed to for receiving audio.
    /// In radio mode, users can subscribe to multiple channels.
    pub subscribed_channels: HashSet<ChannelId>,

    /// Computed permissions for this session.
    /// Calculated from user roles at connection time.
    pub permission: PermissionSet,

    /// JWT token for authentication.
    /// Used to validate API requests from this session.
    pub auth_token: String,

    /// Version of the client software.
    /// Used for compatibility checks and feature gating.
    pub client_version: String,
}

/// Represents the current state of a user session.
///
/// Sessions transition through these states during their lifecycle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SessionState {
    /// Initial state when a connection is established but not yet authenticated.
    Authenticating,

    /// Normal active state - user is connected and can interact.
    Active,

    /// User has set themselves away (self-muted and self-deafened).
    /// Common when users need to step away temporarily.
    Away,

    /// Session is in the process of disconnecting.
    /// Used to clean up resources before removal.
    Disconnecting,
}

impl Session {
    /// Updates the user's last activity timestamp to the current time.
    ///
    /// This should be called whenever the user performs any action,
    /// such as speaking, changing channels, or sending messages.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fleet_net_common::session::Session;
    /// # let mut session: Session = todo!();
    /// // User speaks in channel
    /// session.update_activity();
    ///
    /// // User changes channel
    /// session.update_activity();
    /// ```
    pub fn update_activity(&mut self) {
        let now = Instant::now();
        self.last_active = now;
    }

    /// Checks if the user has been idle for longer than the specified duration.
    ///
    /// Idle time is calculated from the last_active timestamp.
    ///
    /// # Arguments
    ///
    /// * `duration` - Idle threshold in seconds
    ///
    /// # Returns
    ///
    /// `true` if the user has been idle for at least `duration` seconds
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fleet_net_common::session::Session;
    /// # let session: Session = todo!();
    /// // Check if user has been idle for 5 minutes
    /// if session.is_idle(300) {
    ///     println!("User has been idle for 5+ minutes");
    /// }
    /// ```
    pub fn is_idle(&self, duration: u64) -> bool {
        // Get idle duration by subtracting last_active from now
        let dur = Instant::now().duration_since(self.last_active);

        dur.as_secs() >= duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::User;
    use std::net::{IpAddr, Ipv4Addr};

    fn create_test_session() -> Session {
        Session {
            id: "test_session_123".to_string(),
            user: User::new(1),
            socket_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            connected_at: Instant::now(),
            last_active: Instant::now(),
            state: SessionState::Active,
            current_channel: None,
            subscribed_channels: HashSet::new(),
            permission: PermissionSet::new(),
            auth_token: "test_token".to_string(),
            client_version: "1.0.0".to_string(),
        }
    }

    #[test]
    fn test_session_creation() {
        let session = create_test_session();

        assert_eq!(session.id, "test_session_123");
        assert_eq!(session.user.id, 1);
        assert_eq!(session.state, SessionState::Active);
        assert!(session.current_channel.is_none());
        assert!(session.subscribed_channels.is_empty());
        assert_eq!(session.client_version, "1.0.0");
    }

    #[test]
    fn test_update_activity() {
        let mut session = create_test_session();
        let initial_activity = session.last_active;

        // Simulate some time passing
        std::thread::sleep(std::time::Duration::from_millis(100));

        session.update_activity();

        assert!(session.last_active > initial_activity);
    }

    #[test]
    fn test_is_idle() {
        let mut session = create_test_session();

        // Should not be idle immediately.
        assert!(!session.is_idle(1));

        // update last_active to simulate activity
        session.last_active = Instant::now() - std::time::Duration::from_secs(10);

        // Should  be idle for duration less than 10 seconds
        assert!(session.is_idle(5));
        assert!(session.is_idle(10));

        // Should not be idle for duration greater than 10 seconds
        assert!(!session.is_idle(15));
    }
}
