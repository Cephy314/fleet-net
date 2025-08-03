use crate::types::UserId;
use serde::{Deserialize, Serialize};

// User state for voice/audio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAudioState {
    pub user_id: UserId,        // Unique user ID
    pub is_muted: bool,         // Whether the user is muted
    pub is_deafened: bool,      // Whether the user is deafened
    pub is_self_deafened: bool, // Whether the user has self-deafened
    pub is_self_muted: bool,    // Whether the user has self-muted
    pub volume: f32,            // Volume level for the user
}

impl UserAudioState {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            is_muted: false,
            is_deafened: false,
            is_self_deafened: false,
            is_self_muted: false,
            volume: 1.0, // Default volume level
        }
    }

    pub fn can_speak(&self) -> bool {
        !self.is_muted && !self.is_deafened && !self.is_self_muted && !self.is_self_deafened
    }

    pub fn can_hear(&self) -> bool {
        !self.is_deafened && !self.is_self_deafened
    }

    // Set away state (bot mutes and deafens)
    pub fn set_away(&mut self) {
        self.is_self_muted = true;
        self.is_self_deafened = true;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 2.0);
    }
}

impl Default for UserAudioState {
    fn default() -> Self {
        Self::new(0)
    }
}
