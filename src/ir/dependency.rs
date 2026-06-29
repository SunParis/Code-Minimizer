//! Advisory dependency graph used for grouping and ordering candidates.
//!
//! Dependency edges are not proof obligations. They let the reducer try more
//! useful coordinated edits, while parse/build/oracle validation still decides
//! whether a candidate is actually acceptable.

use serde::{Deserialize, Serialize};

use super::{NodeId, SymbolId};

/// Confidence attached to a static analysis result.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Confidence {
    /// Exact according to the adapter's local syntax model.
    Exact,
    /// Conservative approximation that may include extra edges.
    Conservative,
    /// Heuristic edge derived from names or local structure.
    Heuristic,
    /// Unknown confidence.
    Unknown,
}

/// A node or symbol participating in a dependency edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum DependencyKey {
    /// Node dependency endpoint.
    Node(NodeId),
    /// Symbol dependency endpoint.
    Symbol(SymbolId),
}

/// Dependency category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum DependencyKind {
    /// Value reference to a definition.
    Reference,
    /// Call-site dependency.
    Call,
    /// Type-use dependency.
    TypeUse,
    /// Import dependency.
    ImportUse,
    /// Initializer expression dependency.
    InitializerUse,
    /// Signature dependency such as parameters, throws, or override shape.
    SignatureUse,
    /// Parent owns child.
    Containment,
    /// Sibling order may matter.
    SiblingOrder,
}

/// One directed dependency edge.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Source endpoint.
    pub from: DependencyKey,
    /// Required endpoint.
    pub to: DependencyKey,
    /// Edge kind.
    pub kind: DependencyKind,
    /// Static confidence.
    pub confidence: Confidence,
}

/// Advisory dependency graph with forward and reverse edges.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Edges from dependent to dependency.
    pub edges: Vec<DependencyEdge>,
    /// Reversed edges from dependency to dependent.
    pub reverse_edges: Vec<DependencyEdge>,
}

/// One conservative call edge.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CallEdge {
    /// Caller node.
    pub caller: NodeId,
    /// Call-site node.
    pub call_site: NodeId,
    /// Callee symbol if known.
    pub callee: Option<SymbolId>,
    /// Resolution confidence.
    pub confidence: Confidence,
}
