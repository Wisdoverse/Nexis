//! Nexis Gateway - Control Plane
//!
//! This crate implements the Control Plane for Nexis, handling:
//! - WebSocket connections
//! - Message routing
//! - Authentication and authorization
//! - Connection management
//! - Message indexing and semantic search
//! - Metrics and monitoring
//! - Multi-tenant isolation (with `multi-tenant` feature)

pub mod auth;
pub mod collaboration;
pub mod connection;
pub mod db;
pub mod handlers;
pub mod indexing;
pub mod metrics;
#[cfg(feature = "multi-tenant")]
pub mod middleware;
pub mod observability;
pub mod router;
pub mod search;
pub mod server;

#[cfg(feature = "otel")]
pub mod tracing;

#[allow(unused_imports)]
pub use auth::{AuthError, AuthenticatedUser, Claims, JwtConfig};
pub use indexing::{IndexingService, MessageIndexer};
pub use metrics::{export as export_metrics, init_metrics};
pub use router::build_routes;
pub use search::{SearchRequest, SearchResponse, SearchService, SemanticSearchService};

#[cfg(feature = "multi-tenant")]
pub use auth::{TenantContext, TenantError, TenantExtractor};

#[cfg(feature = "multi-tenant")]
pub use middleware::{
    InMemoryTenantStore, MiddlewareTenantContext, ResolvedTenant, ResolutionStrategy, TenantLookup,
    TenantResolutionConfig, TenantResolutionError, TenantResolver, TenantSource,
};

// Re-export multi-tenant models when feature is enabled
#[cfg(feature = "multi-tenant")]
pub use db::models::{
    CreateMember, CreateTenant, CreateWorkspace, Member, MemberType, Plan, Tenant, TenantLimits,
    UpdateMember, UpdateTenant, UpdateWorkspace, Workspace,
};

/// Gateway version
pub const GATEWAY_VERSION: &str = env!("CARGO_PKG_VERSION");
