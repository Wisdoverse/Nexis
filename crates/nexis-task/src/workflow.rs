//! Task workflow state machine and transitions.

use crate::task::TaskStatus;
use thiserror::Error;

/// Side effects emitted by workflow transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionSideEffect {
    NotifyAssignment,
    NotifyBlocked,
    NotifyCompleted,
    NotifyCancelled,
}

/// Result of applying a state transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionResult {
    pub from: TaskStatus,
    pub to: TaskStatus,
    pub side_effects: Vec<TransitionSideEffect>,
}

/// Error returned for invalid transitions.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TransitionError {
    #[error("invalid task transition: {from:?} -> {to:?}")]
    InvalidTransition { from: TaskStatus, to: TaskStatus },
    #[error("block reason cannot be empty")]
    InvalidBlockReason,
}

/// Task workflow contract.
pub trait TaskWorkflow {
    fn transition(
        &self,
        from: TaskStatus,
        to: TaskStatus,
    ) -> Result<TransitionResult, TransitionError>;
}

/// Default workflow implementation for task lifecycle transitions.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultTaskWorkflow;

impl TaskWorkflow for DefaultTaskWorkflow {
    fn transition(
        &self,
        from: TaskStatus,
        to: TaskStatus,
    ) -> Result<TransitionResult, TransitionError> {
        if !is_allowed_transition(from, to) {
            return Err(TransitionError::InvalidTransition { from, to });
        }

        let mut side_effects = Vec::new();
        match to {
            TaskStatus::Assigned => side_effects.push(TransitionSideEffect::NotifyAssignment),
            TaskStatus::Blocked => side_effects.push(TransitionSideEffect::NotifyBlocked),
            TaskStatus::Completed => side_effects.push(TransitionSideEffect::NotifyCompleted),
            TaskStatus::Cancelled => side_effects.push(TransitionSideEffect::NotifyCancelled),
            TaskStatus::Created | TaskStatus::InProgress => {}
        }

        Ok(TransitionResult {
            from,
            to,
            side_effects,
        })
    }
}

fn is_allowed_transition(from: TaskStatus, to: TaskStatus) -> bool {
    matches!(
        (from, to),
        (TaskStatus::Created, TaskStatus::Assigned)
            | (TaskStatus::Created, TaskStatus::Cancelled)
            | (TaskStatus::Assigned, TaskStatus::InProgress)
            | (TaskStatus::Assigned, TaskStatus::Cancelled)
            | (TaskStatus::InProgress, TaskStatus::Completed)
            | (TaskStatus::InProgress, TaskStatus::Blocked)
            | (TaskStatus::InProgress, TaskStatus::Cancelled)
            | (TaskStatus::Blocked, TaskStatus::InProgress)
            | (TaskStatus::Blocked, TaskStatus::Cancelled)
    )
}

#[cfg(test)]
mod tests {
    use super::{DefaultTaskWorkflow, TaskWorkflow, TransitionError, TransitionSideEffect};
    use crate::task::TaskStatus;

    #[test]
    fn allows_happy_path_transitions() {
        let workflow = DefaultTaskWorkflow;

        let first = workflow
            .transition(TaskStatus::Created, TaskStatus::Assigned)
            .expect("created -> assigned should be valid");
        assert_eq!(
            first.side_effects,
            vec![TransitionSideEffect::NotifyAssignment]
        );

        workflow
            .transition(TaskStatus::Assigned, TaskStatus::InProgress)
            .expect("assigned -> in_progress should be valid");

        let last = workflow
            .transition(TaskStatus::InProgress, TaskStatus::Completed)
            .expect("in_progress -> completed should be valid");
        assert_eq!(
            last.side_effects,
            vec![TransitionSideEffect::NotifyCompleted]
        );
    }

    #[test]
    fn rejects_invalid_transition() {
        let workflow = DefaultTaskWorkflow;
        let err = workflow
            .transition(TaskStatus::Created, TaskStatus::Completed)
            .expect_err("created -> completed should be invalid");

        assert_eq!(
            err,
            TransitionError::InvalidTransition {
                from: TaskStatus::Created,
                to: TaskStatus::Completed,
            }
        );
    }
}
