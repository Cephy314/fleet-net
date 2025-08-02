pub enum FleetNetError {
    #[error("Network error: {0}")] // Represents a general network error.
    NetworkError(String),
    #[error("Audio error: {0}")] // Represents an error related to audio processing or transmission.
    AudioError(String),
}