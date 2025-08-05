//! Common functionality shared across Fleet Net components.
//!
//! This crate provides the core data structures and utilities used by both
//! the Fleet Net server and client. It includes user management, permissions,
//! sessions, channels, and audio state tracking.
//!
//! # Architecture
//!
//! The common crate is designed to be the single source of truth for:
//! - Data structures that need to be serialized between client and server
//! - Permission and role management logic
//! - Type definitions and constants
//! - Shared error types
//!
//! # Module Organization
//!
//! - `audio` - Audio state management for users
//! - `channel` - Channel structures and permission resolution
//! - `error` - Common error types
//! - `logging` - Logging configuration utilities
//! - `permission` - Permission system with bitflags
//! - `role` - Role-based access control
//! - `session` - User session management
//! - `types` - Core type aliases
//! - `user` - User representation with Discord integration
//!
//! # Examples
//!
//! ```
//! use fleet_net_common::{User, Role, PermissionSet, permissions};
//!
//! // Create a new user
//! let user = User::new(123);
//!
//! // Create a role with permissions
//! let admin_role = Role::new("admin".to_string(), "Administrator".to_string())
//!     .with_permissions(permissions::ADMINISTRATOR);
//! ```

pub mod audio;
pub mod channel;
pub mod error;
pub mod logging;
pub mod permission;
pub mod role;
pub mod session;
pub mod types;
pub mod user;

// Re-export commonly used types for convenience
pub use audio::UserAudioState;
pub use channel::{Channel, ChannelPermissions, ChannelType};
pub use permission::{permissions, PermissionSet};
pub use role::Role;
pub use session::{Session, SessionState};
pub use user::{DiscordUser, User};
