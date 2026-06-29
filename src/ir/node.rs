//! Node identifiers and language-neutral node summaries.
//!
//! Node ids are valid only inside the `ProgramSnapshot` that created them. The
//! reducer must rebuild a snapshot after every accepted edit because byte ranges
//! and tree-sitter node identities are not stable across source changes.

use serde::{Deserialize, Serialize};

use crate::edit::TextRange;

use super::symbol::{ScopeId, SymbolId};

/// Snapshot-local node identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct NodeId(pub usize);

/// Stable language-neutral node category.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum NodeKind {
    /// Whole source file.
    Program,
    /// Import, require, package import, or module import.
    Import,
    /// Top-level or nested type declaration.
    TypeDecl,
    /// Function, method, constructor, or arrow-function declaration.
    FunctionDecl,
    /// Field, variable, parameter, or local declaration.
    Declaration,
    /// Executable block.
    Block,
    /// Executable statement.
    Statement,
    /// Control-flow construct.
    ControlFlow,
    /// Loop construct.
    Loop,
    /// Call expression.
    CallExpr,
    /// Generic expression.
    Expression,
    /// Literal or literal-like data structure.
    Literal,
    /// Comment or trivia node.
    Trivia,
    /// Unknown node kind retained for reporting and scoring.
    Other,
}

/// Reducer-facing role for a node.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum NodeRole {
    /// Node has no special role.
    Ordinary,
    /// Node is an entry point or directly owns an entry point.
    EntryPoint,
    /// Node is user-visible output such as `System.out.println` or `console.log`.
    OutputOperation,
    /// Node is likely unreachable after a terminating statement.
    Unreachable,
    /// Node is a declaration that can be considered by dead-code cleanup.
    DeclarationOwner,
}

/// Small flag set attached to a node summary.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NodeFlags {
    /// True when the parser reported an error under this node.
    pub has_error: bool,
    /// True when tree-sitter marks the node as named.
    pub named: bool,
    /// True when removing this node usually requires a legal replacement.
    pub requires_replacement: bool,
    /// True when the node should not be deleted directly by generic stages.
    pub protected: bool,
}

/// Snapshot-local summary of one source structure node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeSummary {
    /// Snapshot-local id.
    pub id: NodeId,
    /// Language-neutral kind used by reducer stages.
    pub kind: NodeKind,
    /// Byte range in the snapshot source.
    pub range: TextRange,
    /// Optional parent node id.
    pub parent: Option<NodeId>,
    /// Child node ids in source order.
    pub children: Vec<NodeId>,
    /// Depth from the root node.
    pub depth: u32,
    /// Index inside the parent's child list.
    pub sibling_index: u32,
    /// Whether the original parser node was named.
    pub named: bool,
    /// Optional lexical scope id.
    pub scope: Option<ScopeId>,
    /// Optional declared or referenced symbol id.
    pub symbol: Option<SymbolId>,
    /// Reducer-facing node role.
    pub role: NodeRole,
    /// Additional boolean flags.
    pub flags: NodeFlags,
    /// Original language-specific tree-sitter node kind.
    pub language_kind: String,
}
