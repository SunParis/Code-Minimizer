//! Structural complexity scoring.
//!
//! Scores are compared lexicographically. Runtime duration is intentionally not
//! part of the score because the current design only accepts result-observable
//! bugs such as crashes, wrong results, and A/B output differences.

use serde::{Deserialize, Serialize};

use super::{NodeKind, NodeRole, ProgramIndex};

/// Lexicographic structural complexity score.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ComplexityScore {
    /// Number of top-level declarations.
    pub top_level_decls: usize,
    /// Number of declarations considered reachable from entries.
    pub reachable_decls: usize,
    /// Number of declarations not known to be reachable.
    pub unreachable_decls: usize,
    /// Number of function-like declarations.
    pub functions: usize,
    /// Number of type declarations.
    pub type_decls: usize,
    /// Number of field-like declarations.
    pub fields: usize,
    /// Number of parameters.
    pub parameters: usize,
    /// Number of imports.
    pub imports: usize,
    /// Number of executable statements.
    pub statements: usize,
    /// Number of control-flow nodes.
    pub control_flow_nodes: usize,
    /// Number of loops.
    pub loops: usize,
    /// Number of recursion edges.
    pub recursion_edges: usize,
    /// Maximum nesting depth.
    pub max_nesting_depth: usize,
    /// Maximum block sibling-list length.
    pub max_block_len: usize,
    /// Number of expression nodes.
    pub expressions: usize,
    /// Number of call expressions.
    pub call_exprs: usize,
    /// Number of assignment-like expressions.
    pub assignments: usize,
    /// Total bytes covered by literal nodes.
    pub literal_bytes: usize,
    /// Approximate numeric literal magnitude.
    pub literal_magnitude: u128,
    /// Number of collection literal items.
    pub collection_items: usize,
    /// Number of output operations.
    pub output_operations: usize,
    /// Number of dependency graph edges.
    pub dependency_edges: usize,
    /// Total AST/CST node count.
    pub ast_nodes: usize,
    /// Source length in bytes.
    pub source_bytes: usize,
    /// Static runtime-cost total used by runtime-cost reduction.
    pub runtime_cost_total: u64,
}

impl ComplexityScore {
    /// Computes a conservative score from an index and source text.
    pub fn compute(index: &ProgramIndex, source: &str) -> Self {
        let mut score = Self {
            ast_nodes: index.nodes.len(),
            source_bytes: source.len(),
            dependency_edges: index.dependency_graph.edges.len(),
            ..Self::default()
        };

        for node in &index.nodes {
            score.max_nesting_depth = score.max_nesting_depth.max(node.depth as usize);
            match node.kind {
                NodeKind::Import => score.imports += 1,
                NodeKind::TypeDecl => {
                    score.top_level_decls += usize::from(node.depth <= 1);
                    score.type_decls += 1;
                    score.reachable_decls += usize::from(matches!(node.role, NodeRole::EntryPoint));
                    score.unreachable_decls +=
                        usize::from(!matches!(node.role, NodeRole::EntryPoint));
                }
                NodeKind::FunctionDecl => {
                    score.top_level_decls += usize::from(node.depth <= 2);
                    score.functions += 1;
                    score.reachable_decls += usize::from(matches!(node.role, NodeRole::EntryPoint));
                    score.unreachable_decls +=
                        usize::from(!matches!(node.role, NodeRole::EntryPoint));
                }
                NodeKind::Declaration => score.fields += 1,
                NodeKind::Statement => score.statements += 1,
                NodeKind::ControlFlow => {
                    score.statements += 1;
                    score.control_flow_nodes += 1;
                }
                NodeKind::Loop => {
                    score.statements += 1;
                    score.control_flow_nodes += 1;
                    score.loops += 1;
                    score.runtime_cost_total = score.runtime_cost_total.saturating_add(
                        10 + node.range.len() as u64 / 20 + u64::from(node.depth) * 3,
                    );
                }
                NodeKind::CallExpr => {
                    score.expressions += 1;
                    score.call_exprs += 1;
                    score.runtime_cost_total = score.runtime_cost_total.saturating_add(15);
                    if matches!(node.role, NodeRole::OutputOperation) {
                        score.output_operations += 1;
                    }
                }
                NodeKind::Expression => {
                    score.expressions += 1;
                    if node.language_kind.contains("assignment") {
                        score.assignments += 1;
                    }
                }
                NodeKind::Literal => {
                    score.expressions += 1;
                    score.literal_bytes += node.range.len();
                    score.runtime_cost_total = score
                        .runtime_cost_total
                        .saturating_add(node.range.len() as u64);
                    score.literal_magnitude =
                        score
                            .literal_magnitude
                            .saturating_add(estimate_literal_magnitude(node_text(
                                source,
                                node.range.start,
                                node.range.end,
                            )));
                    if matches!(
                        node.language_kind.as_str(),
                        "array" | "object" | "array_initializer" | "object_creation_expression"
                    ) {
                        score.collection_items += node.children.len();
                    }
                }
                NodeKind::Trivia | NodeKind::Block | NodeKind::Program | NodeKind::Other => {}
            }

            if matches!(node.role, NodeRole::OutputOperation)
                && !matches!(node.kind, NodeKind::CallExpr)
            {
                score.output_operations += 1;
            }
        }

        for list in &index.sibling_lists {
            score.max_block_len = score.max_block_len.max(list.items.len());
        }

        score
    }

    /// Returns true when `self` is strictly simpler than `other`.
    pub fn is_strictly_less_than(&self, other: &Self) -> bool {
        self.comparison_key() < other.comparison_key()
    }

    /// Returns the reducer acceptance key.
    ///
    /// This key intentionally places output operations and executable structure
    /// before advisory reachability counters. A conservative reachability index
    /// may change after an unrelated edit, but that must not block a candidate
    /// that clearly removes output statements, AST nodes, and source bytes.
    fn comparison_key(&self) -> ComplexityComparisonKey {
        ComplexityComparisonKey {
            output_operations: self.output_operations,
            top_level_decls: self.top_level_decls,
            functions: self.functions,
            type_decls: self.type_decls,
            imports: self.imports,
            statements: self.statements,
            control_flow_nodes: self.control_flow_nodes,
            loops: self.loops,
            max_nesting_depth: self.max_nesting_depth,
            max_block_len: self.max_block_len,
            expressions: self.expressions,
            call_exprs: self.call_exprs,
            assignments: self.assignments,
            literal_bytes: self.literal_bytes,
            literal_magnitude: self.literal_magnitude,
            collection_items: self.collection_items,
            fields: self.fields,
            parameters: self.parameters,
            unreachable_decls: self.unreachable_decls,
            dependency_edges: self.dependency_edges,
            ast_nodes: self.ast_nodes,
            source_bytes: self.source_bytes,
        }
    }
}

/// Private lexicographic score key used for candidate acceptance.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ComplexityComparisonKey {
    output_operations: usize,
    top_level_decls: usize,
    functions: usize,
    type_decls: usize,
    imports: usize,
    statements: usize,
    control_flow_nodes: usize,
    loops: usize,
    max_nesting_depth: usize,
    max_block_len: usize,
    expressions: usize,
    call_exprs: usize,
    assignments: usize,
    literal_bytes: usize,
    literal_magnitude: u128,
    collection_items: usize,
    fields: usize,
    parameters: usize,
    unreachable_decls: usize,
    dependency_edges: usize,
    ast_nodes: usize,
    source_bytes: usize,
}

/// Estimated score delta used for candidate ordering and reports.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScoreDelta {
    /// Estimated source byte delta.
    pub source_bytes: isize,
}

impl ScoreDelta {
    /// Builds a score delta from an estimated source-size delta.
    pub fn from_size_delta(source_bytes: isize) -> Self {
        Self { source_bytes }
    }
}

/// Safely returns source text for an AST range.
fn node_text(source: &str, start: usize, end: usize) -> &str {
    source.get(start..end).unwrap_or("")
}

/// Extracts a small approximate magnitude from numeric literal text.
fn estimate_literal_magnitude(text: &str) -> u128 {
    let digits = text
        .bytes()
        .filter(|byte| byte.is_ascii_digit())
        .take(38)
        .fold(0_u128, |value, byte| {
            value
                .saturating_mul(10)
                .saturating_add(u128::from(byte - b'0'))
        });
    digits
}
