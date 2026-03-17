//! API Handlers for Nexis Gateway

pub mod gdpr;

pub use gdpr::{delete_data, export_data, purge_expired_deletions};
