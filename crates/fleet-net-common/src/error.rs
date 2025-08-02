use thiserror::Error;

#[derive(Error, Debug)]
pub enum FleetNetError {
    #[error("Network error: {0}")] // Represents a general network error.
    NetworkError(String),

    #[error("Audio error: {0}")]
    // Represents an error related to audio processing or transmission.
    AudioError(String),

    #[error("Packet error: {0}")] // Represents an error related to packet processing.
    PacketError(String),

    #[error("Authentication error: {0}")] // Represents an error related to authentication.
    AuthError(String),

    #[error("Permission error: {0}")] // Represents an error related to permissions.
    PermissionError(String),
}
