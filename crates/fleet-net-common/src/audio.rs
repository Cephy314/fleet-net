//! Audio state management for Fleet Net users.
//!
//! This module provides structures and utilities for managing user audio states,
//! including mute/deafen status and volume control.

use crate::types::UserId;
use serde::{Deserialize, Serialize};

/// Represents the complete audio state for a user in a voice channel.
///
/// This struct tracks both server-side and client-side audio states,
/// allowing for proper management of voice transmission and reception.
///
/// # State Hierarchy
///
/// - Server mute/deafen overrides self mute/deafen
/// - Deafen state prevents both speaking and hearing
/// - Mute state only prevents speaking
///
/// # Examples
///
/// ```
/// use fleet_net_common::audio::UserAudioState;
/// use fleet_net_common::types::UserId;
///
/// let mut audio_state = UserAudioState::new(42);
/// assert!(audio_state.can_speak());
///
/// audio_state.set_away();
/// assert!(!audio_state.can_speak());
/// assert!(!audio_state.can_hear());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAudioState {
    /// The unique identifier for the user this audio state belongs to.
    pub user_id: UserId,

    /// Server-side mute status. When true, the user cannot transmit audio
    /// regardless of their self-mute status.
    pub is_muted: bool,

    /// Server-side deafen status. When true, the user cannot transmit or
    /// receive audio regardless of their self-deafen status.
    pub is_deafened: bool,

    /// Client-side deafen status. When true, the user chooses not to
    /// receive audio from other users.
    pub is_self_deafened: bool,

    /// Client-side mute status. When true, the user chooses not to
    /// transmit audio to other users.
    pub is_self_muted: bool,

    /// Volume level for the user's audio output.
    /// Valid range is 0.0 to 2.0, where 1.0 is normal volume.
    pub volume: f32,
}

impl UserAudioState {
    /// Creates a new UserAudioState with default settings.
    ///
    /// The user starts unmuted, undeafened, and at normal volume (1.0).
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier for the user
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::audio::UserAudioState;
    ///
    /// let audio_state = UserAudioState::new(123);
    /// assert_eq!(audio_state.user_id, 123);
    /// assert_eq!(audio_state.volume, 1.0);
    /// ```
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

    /// Checks if the user is currently able to transmit audio.
    ///
    /// A user can speak only if they are not muted or deafened by any means
    /// (server-side or self-imposed).
    ///
    /// # Returns
    ///
    /// `true` if the user can transmit audio, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::audio::UserAudioState;
    ///
    /// let mut audio_state = UserAudioState::new(42);
    /// assert!(audio_state.can_speak());
    ///
    /// audio_state.is_muted = true;
    /// assert!(!audio_state.can_speak());
    /// ```
    pub fn can_speak(&self) -> bool {
        // User cannot speak if ANY mute or deafen condition is active
        !self.is_muted && !self.is_deafened && !self.is_self_muted && !self.is_self_deafened
    }

    /// Checks if the user is currently able to receive audio.
    ///
    /// A user can hear only if they are not deafened (server-side or self-imposed).
    /// Mute status does not affect the ability to hear.
    ///
    /// # Returns
    ///
    /// `true` if the user can receive audio, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::audio::UserAudioState;
    ///
    /// let mut audio_state = UserAudioState::new(42);
    /// assert!(audio_state.can_hear());
    ///
    /// audio_state.is_self_deafened = true;
    /// assert!(!audio_state.can_hear());
    /// ```
    pub fn can_hear(&self) -> bool {
        // User cannot hear if ANY deafen condition is active
        !self.is_deafened && !self.is_self_deafened
    }

    /// Sets the user to "away" state by self-muting and self-deafening.
    ///
    /// This is commonly used when a user wants to temporarily disconnect
    /// from voice activity without leaving the channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::audio::UserAudioState;
    ///
    /// let mut audio_state = UserAudioState::new(42);
    /// audio_state.set_away();
    ///
    /// assert!(audio_state.is_self_muted);
    /// assert!(audio_state.is_self_deafened);
    /// assert!(!audio_state.can_speak());
    /// assert!(!audio_state.can_hear());
    /// ```
    pub fn set_away(&mut self) {
        self.is_self_muted = true;
        self.is_self_deafened = true;
    }

    /// Sets the user's volume level with automatic clamping.
    ///
    /// Volume is clamped between 0.0 (silent) and 2.0 (200% volume).
    /// Normal volume is 1.0.
    ///
    /// # Arguments
    ///
    /// * `volume` - The desired volume level (will be clamped to valid range)
    ///
    /// # Examples
    ///
    /// ```
    /// use fleet_net_common::audio::UserAudioState;
    ///
    /// let mut audio_state = UserAudioState::new(42);
    ///
    /// audio_state.set_volume(1.5);
    /// assert_eq!(audio_state.volume, 1.5);
    ///
    /// audio_state.set_volume(3.0);
    /// assert_eq!(audio_state.volume, 2.0); // Clamped to maximum
    ///
    /// audio_state.set_volume(-0.5);
    /// assert_eq!(audio_state.volume, 0.0); // Clamped to minimum
    /// ```
    pub fn set_volume(&mut self, volume: f32) {
        // Clamp volume between silence (0.0) and maximum boost (2.0)
        self.volume = volume.clamp(0.0, 2.0);
    }
}

impl Default for UserAudioState {
    /// Creates a default UserAudioState with user_id 0.
    ///
    /// This is primarily used for testing or placeholder purposes.
    /// In production, use `UserAudioState::new()` with a valid user_id.
    fn default() -> Self {
        Self::new(0)
    }
}
