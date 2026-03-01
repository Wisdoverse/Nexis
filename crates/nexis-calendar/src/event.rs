//! Calendar event domain models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Attendee response status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    NeedsAction,
    Accepted,
    Declined,
    Tentative,
}

/// Event attendee metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventAttendee {
    pub member_id: Uuid,
    pub display_name: String,
    pub response_status: ResponseStatus,
    pub optional: bool,
}

/// Calendar event entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub attendees: Vec<EventAttendee>,
    pub source_type: Option<String>,
    pub source_ref_id: Option<Uuid>,
}
