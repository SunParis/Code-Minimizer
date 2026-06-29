//! Program index assembled from parsed source.
//!
//! The initial index is intentionally lightweight. It flattens the parse tree
//! into snapshot-local nodes, records explicit sibling lists, and leaves symbol
//! information available for language adapters to enrich over time.

use serde::{Deserialize, Serialize};

use super::{
    CallEdge, DependencyGraph, NodeId, NodeSummary, ReferenceSummary, ScopeSummary, SiblingList,
    SymbolSummary,
};

/// Language-neutral index for one parsed source snapshot.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgramIndex {
    /// Flattened nodes indexed by `NodeId.0`.
    pub nodes: Vec<NodeSummary>,
    /// Root node id.
    pub root: NodeId,
    /// Scope summaries.
    pub scopes: Vec<ScopeSummary>,
    /// Symbol summaries.
    pub symbols: Vec<SymbolSummary>,
    /// Reference summaries.
    pub references: Vec<ReferenceSummary>,
    /// Conservative call edges.
    pub call_edges: Vec<CallEdge>,
    /// Entry point node ids.
    pub entry_points: Vec<NodeId>,
    /// Explicit sibling lists used by chunk plans.
    pub sibling_lists: Vec<SiblingList>,
    /// Advisory dependency graph.
    pub dependency_graph: DependencyGraph,
}

impl Default for ProgramIndex {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            root: NodeId(0),
            scopes: Vec::new(),
            symbols: Vec::new(),
            references: Vec::new(),
            call_edges: Vec::new(),
            entry_points: Vec::new(),
            sibling_lists: Vec::new(),
            dependency_graph: DependencyGraph::default(),
        }
    }
}
