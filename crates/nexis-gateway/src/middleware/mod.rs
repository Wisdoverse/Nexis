//! Middleware components for Nexis Gateway.
//!
//! This module provides middleware for:
//! - `rate_limit`: Token-bucket rate limiting (429 + Retry-After)
//! - `tenant`: Multi-tenant context resolution

pub mod rate_limit;

pub use rate_limit::{
    register_metrics, rate_limit_middleware, RateLimitConfig, RateLimitResponse, RateLimiter,
    TokenBucket,
};

#[cfg(feature = "multi-tenant")]
pub mod tenant;

#[cfg(feature = "multi-tenant")]
pub use tenant::{
    InMemoryTenantStore, MiddlewareTenantContext, ResolvedTenant, ResolutionStrategy,
    TenantLookup, TenantResolutionConfig, TenantResolutionError, TenantResolver, TenantSource,
};
