//! Error types for document collaboration operations.

use thiserror::Error;

/// Result type for document operations.
pub type DocResult<T> = Result<T, DocError>;

/// Doc crate error type.
#[derive(Debug, Error)]
pub enum DocError {
    /// Entity could not be found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Input validation failed.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// CRDT operation failed.
    #[error("CRDT error: {0}")]
    Crdt(String),

    /// Snapshot operation failed.
    #[error("Snapshot error: {0}")]
    Snapshot(String),

    /// JSON serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
