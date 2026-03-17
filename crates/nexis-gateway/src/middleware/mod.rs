//! Middleware components for Nexis Gateway.
//!
//! This module provides middleware for:
//! - `tenant`: Multi-tenant context resolution

#[cfg(feature = "multi-tenant")]
pub mod tenant;

#[cfg(feature = "multi-tenant")]
pub use tenant::{
    InMemoryTenantStore, MiddlewareTenantContext, ResolvedTenant, ResolutionStrategy,
    TenantLookup, TenantResolutionConfig, TenantResolutionError, TenantResolver, TenantSource,
};
