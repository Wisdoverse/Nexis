//! WebRTC signaling message models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Signaling message kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalType {
    Offer,
    Answer,
    IceCandidate,
}

/// Generic signaling payload envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalMessage {
    pub id: Uuid,
    pub room_id: Uuid,
    pub from_participant_id: Uuid,
    pub to_participant_id: Option<Uuid>,
    pub signal_type: SignalType,
    pub payload: Value,
    pub sent_at: DateTime<Utc>,
}
