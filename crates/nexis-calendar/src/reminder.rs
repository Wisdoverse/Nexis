//! Reminder and scheduling rule models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Reminder rule for an event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReminderRule {
    pub minutes_before: u32,
    pub repeat_count: u8,
}

/// Reminder instance for event delivery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reminder {
    pub id: Uuid,
    pub event_id: Uuid,
    pub rule: ReminderRule,
    pub next_trigger_at: DateTime<Utc>,
    pub last_triggered_at: Option<DateTime<Utc>>,
}
