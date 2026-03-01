//! Nexis Task - task lifecycle, assignment, and reporting models.

pub mod assignment;
pub mod error;
pub mod report;
pub mod task;

pub use assignment::{Assignee, Assignment};
pub use error::{TaskError, TaskResult};
pub use report::{ReportPeriod, TaskReport};
pub use task::{Task, TaskPriority, TaskSource, TaskStatus};

/// Prelude for common imports.
pub mod prelude {
    pub use crate::assignment::{Assignee, Assignment};
    pub use crate::error::{TaskError, TaskResult};
    pub use crate::report::{ReportPeriod, TaskReport};
    pub use crate::task::{Task, TaskPriority, TaskSource, TaskStatus};
}
