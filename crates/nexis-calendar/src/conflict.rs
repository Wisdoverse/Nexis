//! Conflict detection models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Severity of a calendar conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
}

/// Conflict detected between events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conflict {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub member_id: Uuid,
    pub event_a_id: Uuid,
    pub event_b_id: Uuid,
    pub severity: ConflictSeverity,
    pub detected_at: DateTime<Utc>,
}
