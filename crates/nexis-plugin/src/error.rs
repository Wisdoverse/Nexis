//! Plugin error types.

use thiserror::Error;

/// Errors that can occur within the plugin system.
#[derive(Debug, Error)]
pub enum PluginError {
    /// A plugin failed to process a hook.
    #[error("plugin '{name}' failed: {reason}")]
    HookFailed { name: String, reason: String },

    /// A plugin with the same name is already registered.
    #[error("plugin '{0}' is already registered")]
    AlreadyRegistered(String),

    /// The requested plugin was not found.
    #[error("plugin '{0}' not found")]
    NotFound(String),

    /// A plugin failed to initialize.
    #[error("plugin '{name}' init failed: {reason}")]
    InitFailed { name: String, reason: String },

    /// A plugin failed during teardown.
    #[error("plugin '{name}' teardown failed: {reason}")]
    TeardownFailed { name: String, reason: String },

    /// Extension point error.
    #[error("extension point error: {0}")]
    ExtensionPoint(String),

    /// Serialization error.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
