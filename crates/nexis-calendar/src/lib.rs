//! Nexis Calendar - calendar event, conflict, and reminder models.

pub mod conflict;
pub mod error;
pub mod event;
pub mod reminder;

pub use conflict::{Conflict, ConflictSeverity};
pub use error::{CalendarError, CalendarResult};
pub use event::{CalendarEvent, EventAttendee, ResponseStatus};
pub use reminder::{Reminder, ReminderRule};

/// Prelude for common imports.
pub mod prelude {
    pub use crate::conflict::{Conflict, ConflictSeverity};
    pub use crate::error::{CalendarError, CalendarResult};
    pub use crate::event::{CalendarEvent, EventAttendee, ResponseStatus};
    pub use crate::reminder::{Reminder, ReminderRule};
}
