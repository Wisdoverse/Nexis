//! Nexis Plugin System
//!
//! Extensible plugin architecture supporting message filters, command handlers,
//! notification channels, and storage adapters.

pub mod error;
pub mod extension_points;
pub mod manager;
pub mod plugin;

pub use error::PluginError;
pub use extension_points::{
    Command, CommandHandler, MessageFilter, NotificationChannel, StorageAdapter,
};
pub use manager::PluginManager;
pub use plugin::Plugin;
