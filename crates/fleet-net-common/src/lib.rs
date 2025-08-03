mod audio;
mod channel;
pub mod error;
pub mod logging;
mod permission;
mod role;
mod session;
pub mod types;
mod user;

// Re-export common types and error handling
pub use audio::UserAudioState;
pub use permission::{permissions, PermissionSet};
pub use role::Role;
pub use session::{Session, SessionState};
pub use user::{DiscordUser, User};
