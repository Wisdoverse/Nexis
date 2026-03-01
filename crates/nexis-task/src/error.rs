//! Error types for task operations.

use thiserror::Error;

/// Result type for task operations.
pub type TaskResult<T> = Result<T, TaskError>;

/// Task crate error type.
#[derive(Debug, Error)]
pub enum TaskError {
    /// Entity could not be found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Input validation failed.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Assignment operation failed.
    #[error("Assignment error: {0}")]
    Assignment(String),

    /// Report operation failed.
    #[error("Report error: {0}")]
    Report(String),

    /// JSON serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
