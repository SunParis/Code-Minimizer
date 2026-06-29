//! Sibling-list summaries for chunking and final single-candidate sweeps.
//!
//! The reducer uses explicit sibling lists instead of guessing local list
//! boundaries from raw text. Lists are rebuilt with each snapshot and therefore
//! carry only snapshot-local node ids.

use serde::{Deserialize, Serialize};

use super::NodeId;

/// Snapshot-local sibling-list identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct SiblingListId(pub usize);

/// Category of a sibling list.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SiblingListKind {
    /// Top-level import declarations.
    Imports,
    /// Top-level declarations.
    TopLevel,
    /// Class or object members.
    Members,
    /// Executable block statements.
    Statements,
    /// Array/object/list literal items.
    LiteralItems,
    /// Generic child list when the adapter cannot classify it more precisely.
    Other,
}

/// Explicit sibling list in source order.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SiblingList {
    /// List id.
    pub id: SiblingListId,
    /// Parent node that owns the list.
    pub parent: NodeId,
    /// List category.
    pub kind: SiblingListKind,
    /// Item node ids in source order.
    pub items: Vec<NodeId>,
}
