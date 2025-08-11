use fleet_net_common::types::ChannelId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Radio {
    pub id: u8,
    pub radio_type: RadioTypes,
    pub channel_id: ChannelId,
    pub volume: f32,
    pub pan_lr: f32,
    pub is_dimmed: bool,
    pub is_muted: bool,
    pub has_priority: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum RadioTypes {
    Hf = 0,
    Uhf = 1,
    Vhf = 2,
    Satellite = 3,
    Quantum = 4,
}

// Mapped to RadioTypes for a radio to know how to process the audio.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RadioEffect {
    pub low_cut: f32,    // Low cut frequency
    pub high_cut: f32,   // High cut frequency
    pub distortion: f32, // Apply distortion effect
    pub decay: f32,      // Simulate decay with random noise interruption.
}
