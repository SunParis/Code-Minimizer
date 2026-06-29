//! Language-neutral intermediate representation used by the reducer.
//!
//! The IR is deliberately conservative. It gives the reducer enough structure
//! to order candidates, group sibling edits, compute complexity, and rebuild a
//! fresh snapshot after every accepted edit. It does not try to prove semantic
//! equivalence; the oracle remains the final authority.

pub mod analysis;
pub mod dependency;
pub mod index;
pub mod node;
pub mod score;
pub mod sibling;
pub mod snapshot;
pub mod symbol;

pub use analysis::{
    BlockSummary, CallSiteContext, CallSiteSummary, DefUseSummary, FunctionSummary, LoopSummary,
    ProgramAnalysis, RuntimeCostEstimate,
};
pub use dependency::{
    CallEdge, Confidence, DependencyEdge, DependencyGraph, DependencyKey, DependencyKind,
};
pub use index::ProgramIndex;
pub use node::{NodeFlags, NodeId, NodeKind, NodeRole, NodeSummary};
pub use score::{ComplexityScore, ScoreDelta};
pub use sibling::{SiblingList, SiblingListId, SiblingListKind};
pub use snapshot::{ProgramSnapshot, SnapshotId, SourceHash, StructureHash};
pub use symbol::{
    ReferenceId, ReferenceKind, ReferenceSummary, ScopeId, ScopeKind, ScopeSummary, SymbolFlags,
    SymbolId, SymbolKind, SymbolSummary, Visibility,
};
