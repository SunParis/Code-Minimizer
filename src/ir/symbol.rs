//! Conservative scope, symbol, and reference summaries.
//!
//! The first implementation records enough metadata for reports and future
//! dependency-aware expansions. The reducer treats this data as advisory: an
//! imprecise symbol graph may change ordering, but it must never replace oracle
//! validation.

use serde::{Deserialize, Serialize};

use super::node::NodeId;

/// Snapshot-local scope identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ScopeId(pub usize);

/// Snapshot-local symbol identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct SymbolId(pub usize);

/// Snapshot-local reference identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ReferenceId(pub usize);

/// Coarse scope category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ScopeKind {
    /// Whole source file.
    Global,
    /// Class, interface, enum, record, or object-like type scope.
    Type,
    /// Function, method, constructor, or closure scope.
    Function,
    /// Block-local scope.
    Block,
}

/// Coarse symbol category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    /// Function, method, constructor, or callable value.
    Function,
    /// Class, interface, enum, record, or JavaScript class.
    Type,
    /// Field or object member.
    Field,
    /// Local or global variable.
    Variable,
    /// Function or method parameter.
    Parameter,
    /// Import binding.
    Import,
    /// Unknown declaration-like symbol.
    Unknown,
}

/// Coarse reference category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ReferenceKind {
    /// Name reference in an expression or statement.
    Value,
    /// Type reference.
    Type,
    /// Function or method call reference.
    Call,
    /// Import use reference.
    ImportUse,
    /// Unknown reference category.
    Unknown,
}

/// Visibility summary used by declaration ordering.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Visibility {
    /// Publicly visible declaration.
    Public,
    /// Private declaration.
    Private,
    /// Package/module/internal visibility.
    Internal,
    /// Unknown or language-specific visibility.
    Unknown,
}

/// Small symbol flag set.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SymbolFlags {
    /// True when this symbol should be protected by default.
    pub protected: bool,
    /// True when this symbol appears to be an entry point.
    pub entry_point: bool,
}

/// One lexical or structural scope.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScopeSummary {
    /// Scope id.
    pub id: ScopeId,
    /// Scope category.
    pub kind: ScopeKind,
    /// Node that owns the scope.
    pub owner: NodeId,
    /// Optional parent scope.
    pub parent: Option<ScopeId>,
    /// Symbols declared directly in this scope.
    pub declared_symbols: Vec<SymbolId>,
}

/// One declared symbol.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SymbolSummary {
    /// Symbol id.
    pub id: SymbolId,
    /// Source-level symbol name.
    pub name: String,
    /// Symbol category.
    pub kind: SymbolKind,
    /// Declaration node id.
    pub declaration: NodeId,
    /// Declaring scope.
    pub scope: ScopeId,
    /// Visibility summary.
    pub visibility: Visibility,
    /// References resolved to this symbol when known.
    pub references: Vec<ReferenceId>,
    /// Additional flags.
    pub flags: SymbolFlags,
}

/// One source-level reference.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReferenceSummary {
    /// Reference id.
    pub id: ReferenceId,
    /// Referenced name text.
    pub name: String,
    /// Reference category.
    pub kind: ReferenceKind,
    /// Node that contains the reference.
    pub node: NodeId,
    /// Scope where the reference occurs.
    pub scope: ScopeId,
    /// Resolved symbol when the conservative resolver can identify one.
    pub resolved: Option<SymbolId>,
}
