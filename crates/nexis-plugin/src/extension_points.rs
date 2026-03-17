//! Extension point traits for specialized plugin capabilities.
//!
//! These are optional interfaces that plugins can implement to hook into
//! specific subsystems beyond the core event lifecycle.

use async_trait::async_trait;

use crate::error::PluginError;
use crate::plugin::{Member, Message, Response};

/// A slash command parsed from user input.
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub sender_id: String,
    pub room_id: String,
}

/// Message filter extension point.
///
/// Implement to perform content moderation, sensitive word filtering,
/// spam detection, etc. Filters run in registration order; if any filter
/// returns an error, the message is rejected.
#[async_trait]
pub trait MessageFilter: Send + Sync {
    /// Filter or transform a message.
    /// Return `Ok(())` to allow, `Err` to reject.
    async fn filter(&self, message: &mut Message) -> Result<(), PluginError>;
}

/// Command handler extension point.
///
/// Implement to add new slash commands to the platform.
#[async_trait]
pub trait CommandHandler: Send + Sync {
    /// The command name (without the `/` prefix).
    fn command_name(&self) -> &str;

    /// Handle the command. Return `Some(Response)` to send a reply.
    async fn handle(&self, cmd: &Command) -> Result<Option<Response>, PluginError>;
}

/// Notification channel extension point.
///
/// Implement to deliver notifications to external services
/// (Webhook, Email, Slack, etc.).
#[async_trait]
pub trait NotificationChannel: Send + Sync {
    /// Channel identifier (e.g. "webhook", "email", "slack").
    fn channel_type(&self) -> &str;

    /// Send a notification to the external channel.
    async fn send(
        &self,
        title: &str,
        body: &str,
        target: &str,
    ) -> Result<(), PluginError>;
}

/// Key-value pair for storage.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StorageEntry {
    pub key: String,
    pub value: serde_json::Value,
    pub ttl_seconds: Option<u64>,
}

/// Storage adapter extension point.
///
/// Implement to plug in different storage backends (S3, MongoDB, Redis, etc.).
#[async_trait]
pub trait StorageAdapter: Send + Sync {
    /// Adapter identifier (e.g. "s3", "mongodb", "local_fs").
    fn adapter_type(&self) -> &str;

    /// Get a value by key.
    async fn get(&self, key: &str) -> Result<Option<serde_json::Value>, PluginError>;

    /// Set a key-value pair.
    async fn set(&self, entry: StorageEntry) -> Result<(), PluginError>;

    /// Delete a key.
    async fn delete(&self, key: &str) -> Result<bool, PluginError>;
}
