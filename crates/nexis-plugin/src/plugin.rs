//! Core Plugin trait definition.

use async_trait::async_trait;

use crate::error::PluginError;

// Re-export domain types for convenience
pub use nexis_core::Message;
pub use nexis_protocol::MemberId;

/// Simplified member info passed to plugin hooks.
#[derive(Debug, Clone)]
pub struct Member {
    pub id: MemberId,
    pub display_name: String,
    pub room_id: String,
}

/// Response returned by command handlers.
#[derive(Debug, Clone)]
pub struct Response {
    pub content: String,
    pub is_markdown: bool,
}

/// The core Plugin trait.
///
/// All Nexis plugins implement this trait. Each method provides a hook into
/// the platform's event lifecycle. Default implementations are no-ops so
/// plugins only need to implement the hooks they care about.
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Unique plugin identifier.
    fn name(&self) -> &str;

    /// Semantic version (e.g. "1.0.0").
    fn version(&self) -> &str;

    /// Called when a new message arrives. Plugins can mutate or drop messages.
    fn on_message(&self, _message: &mut Message) -> Result<(), PluginError> {
        Ok(())
    }

    /// Called when a member joins a room.
    fn on_member_join(&self, _member: &Member) -> Result<(), PluginError> {
        Ok(())
    }

    /// Called when a member leaves a room.
    fn on_member_leave(&self, _member: &Member) -> Result<(), PluginError> {
        Ok(())
    }

    /// Called when a slash command is invoked. Return `Some(Response)` to reply.
    fn on_command(&self, _cmd: &crate::Command) -> Result<Option<Response>, PluginError> {
        Ok(None)
    }

    /// Optional initialization hook called once after registration.
    async fn on_init(&self) -> Result<(), PluginError> {
        Ok(())
    }

    /// Optional teardown hook called once before unregistration.
    async fn on_teardown(&self) -> Result<(), PluginError> {
        Ok(())
    }
}
