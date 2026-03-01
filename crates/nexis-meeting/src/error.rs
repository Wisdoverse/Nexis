//! Error types for meeting operations.

use thiserror::Error;

/// Result type for meeting operations.
pub type MeetingResult<T> = Result<T, MeetingError>;

/// Meeting crate error type.
#[derive(Debug, Error)]
pub enum MeetingError {
    /// Entity could not be found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Input validation failed.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Signaling operation failed.
    #[error("Signaling error: {0}")]
    Signaling(String),

    /// Recording operation failed.
    #[error("Recording error: {0}")]
    Recording(String),

    /// JSON serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
