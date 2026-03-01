//! Nexis Doc - real-time document collaboration and CRDT operation models.

pub mod comment;
pub mod crdt;
pub mod document;
pub mod error;
pub mod snapshot;

pub use comment::{Comment, CommentAnchor, CommentThread};
pub use crdt::{CRDTOperation, OpType, OperationLog};
pub use document::{DocMetadata, DocVersion, Document};
pub use error::{DocError, DocResult};
pub use snapshot::{DocSnapshot, SnapshotMeta};

/// Prelude for common imports.
pub mod prelude {
    pub use crate::comment::{Comment, CommentAnchor, CommentThread};
    pub use crate::crdt::{CRDTOperation, OpType, OperationLog};
    pub use crate::document::{DocMetadata, DocVersion, Document};
    pub use crate::error::{DocError, DocResult};
    pub use crate::snapshot::{DocSnapshot, SnapshotMeta};
}
