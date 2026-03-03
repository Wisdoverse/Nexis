//! Task domain models.

use crate::workflow::{DefaultTaskWorkflow, TaskWorkflow, TransitionError, TransitionResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Task lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Created,
    Assigned,
    InProgress,
    Blocked,
    Completed,
    Cancelled,
}

/// Task priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Origin of task creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskSource {
    Manual,
    MeetingActionItem,
    DocComment,
    AiGenerated,
}

/// Task entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub assigned_to: Option<Uuid>,
    pub block_reason: Option<String>,
    pub priority: TaskPriority,
    pub source: TaskSource,
    pub due_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn assign_to(&mut self, user_id: Uuid) -> Result<TransitionResult, TransitionError> {
        let result = self.transition_to(TaskStatus::Assigned)?;
        self.assigned_to = Some(user_id);
        self.block_reason = None;
        Ok(result)
    }

    pub fn start(&mut self) -> Result<TransitionResult, TransitionError> {
        let result = self.transition_to(TaskStatus::InProgress)?;
        self.block_reason = None;
        Ok(result)
    }

    pub fn complete(&mut self) -> Result<TransitionResult, TransitionError> {
        let result = self.transition_to(TaskStatus::Completed)?;
        self.block_reason = None;
        Ok(result)
    }

    pub fn block(
        &mut self,
        reason: impl Into<String>,
    ) -> Result<TransitionResult, TransitionError> {
        let reason = reason.into();
        if reason.trim().is_empty() {
            return Err(TransitionError::InvalidBlockReason);
        }

        let result = self.transition_to(TaskStatus::Blocked)?;
        self.block_reason = Some(reason);
        Ok(result)
    }

    pub fn cancel(&mut self) -> Result<TransitionResult, TransitionError> {
        let result = self.transition_to(TaskStatus::Cancelled)?;
        self.block_reason = None;
        Ok(result)
    }

    fn transition_to(&mut self, to: TaskStatus) -> Result<TransitionResult, TransitionError> {
        let workflow = DefaultTaskWorkflow;
        let result = workflow.transition(self.status, to)?;
        self.status = to;
        self.updated_at = Utc::now();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::{Task, TaskPriority, TaskSource, TaskStatus};
    use crate::workflow::TransitionError;
    use chrono::Utc;
    use uuid::Uuid;

    fn make_task(status: TaskStatus) -> Task {
        let now = Utc::now();
        Task {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            title: "Task".to_string(),
            description: None,
            status,
            assigned_to: None,
            block_reason: None,
            priority: TaskPriority::Medium,
            source: TaskSource::Manual,
            due_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn assign_start_complete_happy_path() {
        let mut task = make_task(TaskStatus::Created);
        let user_id = Uuid::new_v4();

        task.assign_to(user_id).expect("assign should succeed");
        assert_eq!(task.status, TaskStatus::Assigned);
        assert_eq!(task.assigned_to, Some(user_id));

        task.start().expect("start should succeed");
        assert_eq!(task.status, TaskStatus::InProgress);

        task.complete().expect("complete should succeed");
        assert_eq!(task.status, TaskStatus::Completed);
    }

    #[test]
    fn block_requires_reason() {
        let mut task = make_task(TaskStatus::InProgress);
        let err = task.block("   ").expect_err("blank reason should fail");
        assert_eq!(err, TransitionError::InvalidBlockReason);
    }

    #[test]
    fn cancel_is_allowed_from_assigned() {
        let mut task = make_task(TaskStatus::Assigned);
        task.cancel().expect("cancel should succeed");
        assert_eq!(task.status, TaskStatus::Cancelled);
    }
}
