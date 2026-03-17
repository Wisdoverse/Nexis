//! Plugin manager with registration, lifecycle, and error isolation.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::error::PluginError;
use crate::extension_points::{
    Command, CommandHandler, MessageFilter, NotificationChannel, StorageAdapter,
};
use crate::plugin::{Member, Message, Plugin, Response};

/// Manages plugin registration, lifecycle, and dispatch.
pub struct PluginManager {
    plugins: RwLock<HashMap<String, Arc<dyn Plugin>>>,
}

impl PluginManager {
    /// Create a new empty plugin manager.
    pub fn new() -> Self {
        Self {
            plugins: RwLock::new(HashMap::new()),
        }
    }

    /// Register a plugin. Initializes it immediately.
    ///
    /// Returns an error if a plugin with the same name already exists
    /// or if initialization fails.
    pub async fn register(&self, plugin: impl Plugin + 'static) -> Result<(), PluginError> {
        let name = plugin.name().to_string();
        let version = plugin.version().to_string();
        let plugin = Arc::new(plugin);

        {
            let mut plugins = self.plugins.write().await;
            if plugins.contains_key(&name) {
                return Err(PluginError::AlreadyRegistered(name));
            }
            plugins.insert(name.clone(), plugin.clone());
        }

        // Initialize with error isolation
        if let Err(e) = plugin.on_init().await {
            warn!("Plugin '{}' init failed, removing: {}", name, e);
            self.plugins.write().await.remove(&name);
            return Err(PluginError::InitFailed {
                name,
                reason: e.to_string(),
            });
        }

        info!(plugin = %name, version = %version, "Plugin registered");
        Ok(())
    }

    /// Unregister a plugin. Calls teardown before removal.
    pub async fn unregister(&self, name: &str) -> Result<(), PluginError> {
        let plugin = {
            let mut plugins = self.plugins.write().await;
            plugins.remove(name).ok_or_else(|| PluginError::NotFound(name.to_string()))?
        };

        if let Err(e) = plugin.on_teardown().await {
            warn!("Plugin '{}' teardown failed: {}", name, e);
            // Still considered unregistered even if teardown fails
        }

        info!(plugin = %name, "Plugin unregistered");
        Ok(())
    }

    /// List registered plugin names.
    pub async fn list_plugins(&self) -> Vec<String> {
        self.plugins.read().await.keys().cloned().collect()
    }

    /// Dispatch a message to all plugins' `on_message` hook.
    ///
    /// Errors are logged and isolated — one failing plugin does not
    /// prevent others from running.
    pub async fn dispatch_message(&self, message: &mut Message) {
        let plugins = self.plugins.read().await;
        for (name, plugin) in plugins.iter() {
            if let Err(e) = plugin.on_message(message) {
                warn!(plugin = %name, error = %e, "Plugin on_message failed");
            }
        }
    }

    /// Dispatch a member join event to all plugins.
    pub async fn dispatch_member_join(&self, member: &Member) {
        let plugins = self.plugins.read().await;
        for (name, plugin) in plugins.iter() {
            if let Err(e) = plugin.on_member_join(member) {
                warn!(plugin = %name, error = %e, "Plugin on_member_join failed");
            }
        }
    }

    /// Dispatch a member leave event to all plugins.
    pub async fn dispatch_member_leave(&self, member: &Member) {
        let plugins = self.plugins.read().await;
        for (name, plugin) in plugins.iter() {
            if let Err(e) = plugin.on_member_leave(member) {
                warn!(plugin = %name, error = %e, "Plugin on_member_leave failed");
            }
        }
    }

    /// Dispatch a command to all plugins. Returns the first non-None response.
    pub async fn dispatch_command(&self, cmd: &Command) -> Option<Response> {
        let plugins = self.plugins.read().await;
        for (name, plugin) in plugins.iter() {
            match plugin.on_command(cmd) {
                Ok(Some(response)) => return Some(response),
                Ok(None) => continue,
                Err(e) => {
                    warn!(plugin = %name, error = %e, "Plugin on_command failed");
                }
            }
        }
        None
    }

    /// Run message filters in order. Returns an error if any filter rejects.
    pub async fn run_message_filters(
        &self,
        filters: &[&dyn MessageFilter],
        message: &mut Message,
    ) -> Result<(), PluginError> {
        for filter in filters {
            filter.filter(message).await?;
        }
        Ok(())
    }

    /// Run command handlers. Returns the first response.
    pub async fn run_command_handlers(
        &self,
        handlers: &[&dyn CommandHandler],
        cmd: &Command,
    ) -> Result<Option<Response>, PluginError> {
        for handler in handlers {
            if handler.command_name() == cmd.name {
                return handler.handle(cmd).await;
            }
        }
        Ok(None)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
