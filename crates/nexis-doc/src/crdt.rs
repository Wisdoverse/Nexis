//! CRDT operation models and operation log stub.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Type of CRDT operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpType {
    Insert,
    Delete,
    Format,
    Replace,
}

/// Single CRDT operation entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CRDTOperation {
    pub id: Uuid,
    pub doc_id: Uuid,
    pub actor_id: Uuid,
    pub op_type: OpType,
    pub payload: Value,
    pub seq: u64,
    pub created_at: DateTime<Utc>,
}

/// In-memory operation log stub for future persistence integration.
#[derive(Debug, Default, Clone)]
pub struct OperationLog {
    operations: Vec<CRDTOperation>,
}

impl OperationLog {
    /// Creates an empty operation log.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a CRDT operation to the log.
    pub fn append(&mut self, operation: CRDTOperation) {
        self.operations.push(operation);
    }

    /// Returns all logged operations.
    #[must_use]
    pub fn all(&self) -> &[CRDTOperation] {
        &self.operations
    }
}
