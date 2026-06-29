//! Structural analyses used by reducer stage scheduling.
//!
//! These summaries are intentionally conservative. They provide enough shape
//! information to rank candidates such as loop-bound shrinks, function
//! elimination, block deletion, and output-only cleanup. They are rebuilt for
//! every accepted snapshot and never replace parse/build/oracle validation.

use serde::{Deserialize, Serialize};

use super::{NodeId, NodeKind, NodeRole, ProgramIndex};

/// Snapshot-local function summary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionSummary {
    /// Function-like node.
    pub node: NodeId,
    /// Best-effort source-level name.
    pub name: Option<String>,
    /// Whether this function appears to be an entry point.
    pub entry: bool,
    /// Whether this function should not be deleted by ordinary function stages.
    pub protected: bool,
    /// Bytes covered by the function node.
    pub body_bytes: usize,
    /// Statement count inside this function.
    pub statements: usize,
    /// Loop count inside this function.
    pub loops: usize,
    /// Call count inside this function.
    pub calls: usize,
    /// Call-site nodes syntactically inside this function.
    pub call_sites: Vec<NodeId>,
}

/// Snapshot-local call-site summary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CallSiteSummary {
    /// Call expression node.
    pub node: NodeId,
    /// Enclosing function when known.
    pub enclosing_function: Option<NodeId>,
    /// Best-effort called name.
    pub callee_name: Option<String>,
    /// Syntactic context used by call-site replacement.
    pub context: CallSiteContext,
}

/// Coarse call-site context.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum CallSiteContext {
    /// The call is the main expression in an expression statement.
    Statement,
    /// The call occurs in an assignment right-hand side.
    Assignment,
    /// The call occurs inside a declaration initializer.
    DeclarationInitializer,
    /// The call occurs inside a control-flow condition.
    Condition,
    /// The call occurs in a return expression.
    Return,
    /// The call occurs in a nested expression or argument.
    NestedExpression,
    /// The context was not recognized.
    Unknown,
}

/// Snapshot-local loop summary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LoopSummary {
    /// Loop node.
    pub node: NodeId,
    /// Parser-specific loop kind.
    pub kind: String,
    /// Bytes covered by the loop body or whole loop when body is not isolated.
    pub body_bytes: usize,
    /// Nested loop count.
    pub nested_loops: usize,
    /// Call count inside the loop.
    pub calls: usize,
    /// Output operation count inside the loop.
    pub outputs: usize,
    /// Block nesting depth.
    pub depth: u32,
    /// Best-effort static trip count.
    pub estimated_trip_count: Option<u64>,
    /// Priority score for runtime-cost reduction.
    pub score: u64,
}

/// Snapshot-local block or construct summary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockSummary {
    /// Block node.
    pub node: NodeId,
    /// Optional enclosing owner construct.
    pub owner: Option<NodeId>,
    /// Bytes covered by the owner or block.
    pub owner_bytes: usize,
    /// Bytes covered by the block itself.
    pub block_bytes: usize,
    /// Statements inside this block.
    pub statements: usize,
    /// Nested blocks inside this block.
    pub nested_blocks: usize,
    /// Loops inside this block.
    pub loops: usize,
    /// Calls inside this block.
    pub calls: usize,
    /// Priority score for block elimination.
    pub score: u64,
}

/// Snapshot-local declaration/use summary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DefUseSummary {
    /// Declaration node.
    pub declaration: NodeId,
    /// Best-effort declared name.
    pub name: Option<String>,
    /// Number of references outside the declaration.
    pub references: usize,
    /// Number of references that appear only in output contexts.
    pub output_references: usize,
    /// Whether the declaration initializer may have side effects.
    pub initializer_may_have_side_effects: bool,
}

/// Static runtime-cost estimate used by stage objective checks.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct RuntimeCostEstimate {
    /// Estimated loop cost.
    pub loop_cost: u64,
    /// Estimated recursive or dense call cost.
    pub recursive_call_cost: u64,
    /// Large literal and collection cost.
    pub large_literal_cost: u64,
    /// Call density cost.
    pub call_density_cost: u64,
    /// Allocation-like construct cost.
    pub allocation_cost: u64,
    /// User-visible output operation cost.
    pub io_noise_cost: u64,
}

impl RuntimeCostEstimate {
    /// Returns the lexicographic/weighted total used for phase-level filters.
    pub fn total(&self) -> u64 {
        self.loop_cost
            .saturating_add(self.recursive_call_cost)
            .saturating_add(self.large_literal_cost)
            .saturating_add(self.call_density_cost)
            .saturating_add(self.allocation_cost)
            .saturating_add(self.io_noise_cost)
    }
}

/// All advisory analyses for a snapshot.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgramAnalysis {
    /// Function-like declarations.
    pub functions: Vec<FunctionSummary>,
    /// Call expressions.
    pub call_sites: Vec<CallSiteSummary>,
    /// Loop summaries.
    pub loops: Vec<LoopSummary>,
    /// Block summaries.
    pub blocks: Vec<BlockSummary>,
    /// Declaration/use summaries.
    pub def_uses: Vec<DefUseSummary>,
    /// Static runtime-cost estimate.
    pub runtime_cost: RuntimeCostEstimate,
}

impl ProgramAnalysis {
    /// Computes conservative summaries from the language-neutral index.
    pub fn compute(index: &ProgramIndex, source: &str) -> Self {
        let call_sites = build_call_sites(index, source);
        let functions = build_functions(index, source, &call_sites);
        let loops = build_loops(index, source);
        let blocks = build_blocks(index);
        let def_uses = build_def_uses(index, source);
        let runtime_cost = compute_runtime_cost(index, source, &loops);

        Self {
            functions,
            call_sites,
            loops,
            blocks,
            def_uses,
            runtime_cost,
        }
    }
}

fn build_functions(
    index: &ProgramIndex,
    source: &str,
    call_sites: &[CallSiteSummary],
) -> Vec<FunctionSummary> {
    index
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, NodeKind::FunctionDecl))
        .map(|node| {
            let call_site_ids = call_sites
                .iter()
                .filter(|call| range_contains_node(index, node.id, call.node))
                .map(|call| call.node)
                .collect::<Vec<_>>();
            FunctionSummary {
                node: node.id,
                name: first_identifier_text(index, source, node.id),
                entry: matches!(node.role, NodeRole::EntryPoint),
                protected: node.flags.protected,
                body_bytes: node.range.len(),
                statements: count_descendants(index, node.id, |kind| {
                    matches!(
                        kind,
                        NodeKind::Statement | NodeKind::ControlFlow | NodeKind::Loop
                    )
                }),
                loops: count_descendants(index, node.id, |kind| matches!(kind, NodeKind::Loop)),
                calls: call_site_ids.len(),
                call_sites: call_site_ids,
            }
        })
        .collect()
}

fn build_call_sites(index: &ProgramIndex, source: &str) -> Vec<CallSiteSummary> {
    index
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, NodeKind::CallExpr))
        .map(|node| CallSiteSummary {
            node: node.id,
            enclosing_function: ancestor_of_kind(index, node.id, NodeKind::FunctionDecl),
            callee_name: call_name(index, source, node.id),
            context: call_context(index, node.id),
        })
        .collect()
}

fn build_loops(index: &ProgramIndex, source: &str) -> Vec<LoopSummary> {
    index
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, NodeKind::Loop))
        .map(|node| {
            let body_bytes = node
                .children
                .iter()
                .find_map(|child| {
                    let child_node = &index.nodes[child.0];
                    matches!(child_node.kind, NodeKind::Block).then_some(child_node.range.len())
                })
                .unwrap_or_else(|| node.range.len());
            let nested_loops =
                count_descendants(index, node.id, |kind| matches!(kind, NodeKind::Loop))
                    .saturating_sub(1);
            let calls =
                count_descendants(index, node.id, |kind| matches!(kind, NodeKind::CallExpr));
            let outputs = count_descendant_roles(index, node.id, NodeRole::OutputOperation);
            let estimated_trip_count =
                estimate_trip_count(node_text(source, node.range.start, node.range.end));
            let score = loop_score(
                estimated_trip_count,
                body_bytes,
                nested_loops,
                calls,
                outputs,
                node.depth,
            );
            LoopSummary {
                node: node.id,
                kind: node.language_kind.clone(),
                body_bytes,
                nested_loops,
                calls,
                outputs,
                depth: node.depth,
                estimated_trip_count,
                score,
            }
        })
        .collect()
}

fn build_blocks(index: &ProgramIndex) -> Vec<BlockSummary> {
    index
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, NodeKind::Block))
        .map(|node| {
            let owner = node.parent.and_then(|parent| {
                matches!(
                    index.nodes[parent.0].kind,
                    NodeKind::ControlFlow | NodeKind::Loop | NodeKind::FunctionDecl
                )
                .then_some(parent)
            });
            let owner_bytes = owner
                .map(|id| index.nodes[id.0].range.len())
                .unwrap_or_else(|| node.range.len());
            let statements = count_descendants(index, node.id, |kind| {
                matches!(
                    kind,
                    NodeKind::Statement | NodeKind::ControlFlow | NodeKind::Loop
                )
            });
            let nested_blocks =
                count_descendants(index, node.id, |kind| matches!(kind, NodeKind::Block))
                    .saturating_sub(1);
            let loops = count_descendants(index, node.id, |kind| matches!(kind, NodeKind::Loop));
            let calls =
                count_descendants(index, node.id, |kind| matches!(kind, NodeKind::CallExpr));
            let score = owner_bytes as u64
                + node.range.len() as u64
                + statements as u64 * 8
                + nested_blocks as u64 * 20
                + loops as u64 * 80
                + calls as u64 * 15
                + u64::from(node.depth) * 3;
            BlockSummary {
                node: node.id,
                owner,
                owner_bytes,
                block_bytes: node.range.len(),
                statements,
                nested_blocks,
                loops,
                calls,
                score,
            }
        })
        .collect()
}

fn build_def_uses(index: &ProgramIndex, source: &str) -> Vec<DefUseSummary> {
    index
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, NodeKind::Declaration | NodeKind::Import))
        .map(|node| {
            let name = first_identifier_text(index, source, node.id);
            let references = name
                .as_deref()
                .map(|name| count_identifier_occurrences(source, name).saturating_sub(1))
                .unwrap_or(0);
            DefUseSummary {
                declaration: node.id,
                name,
                references,
                output_references: 0,
                initializer_may_have_side_effects: node_text(
                    source,
                    node.range.start,
                    node.range.end,
                )
                .contains('('),
            }
        })
        .collect()
}

fn compute_runtime_cost(
    index: &ProgramIndex,
    source: &str,
    loops: &[LoopSummary],
) -> RuntimeCostEstimate {
    let loop_cost = loops
        .iter()
        .map(|loop_summary| loop_summary.score)
        .sum::<u64>();
    let call_count = index
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, NodeKind::CallExpr))
        .count() as u64;
    let literal_bytes = index
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, NodeKind::Literal))
        .map(|node| node.range.len() as u64)
        .sum::<u64>();
    let allocation_cost = source.matches("new ").count() as u64 * 20;
    let io_noise_cost = index
        .nodes
        .iter()
        .filter(|node| matches!(node.role, NodeRole::OutputOperation))
        .count() as u64
        * 10;

    RuntimeCostEstimate {
        loop_cost,
        recursive_call_cost: 0,
        large_literal_cost: literal_bytes,
        call_density_cost: call_count * 15,
        allocation_cost,
        io_noise_cost,
    }
}

fn loop_score(
    estimated_trip_count: Option<u64>,
    body_bytes: usize,
    nested_loops: usize,
    calls: usize,
    outputs: usize,
    depth: u32,
) -> u64 {
    let trip = estimated_trip_count
        .map(|count| u64::from(count.saturating_add(1).ilog2()) * 20)
        .unwrap_or(10);
    trip + (body_bytes as u64 / 20).min(100)
        + nested_loops as u64 * 80
        + calls as u64 * 15
        + outputs as u64 * 10
        + u64::from(depth) * 3
}

fn count_descendants(
    index: &ProgramIndex,
    node: NodeId,
    predicate: impl Fn(&NodeKind) -> bool + Copy,
) -> usize {
    let current = &index.nodes[node.0];
    usize::from(predicate(&current.kind))
        + current
            .children
            .iter()
            .map(|child| count_descendants(index, *child, predicate))
            .sum::<usize>()
}

fn count_descendant_roles(index: &ProgramIndex, node: NodeId, role: NodeRole) -> usize {
    let current = &index.nodes[node.0];
    usize::from(current.role == role)
        + current
            .children
            .iter()
            .map(|child| count_descendant_roles(index, *child, role))
            .sum::<usize>()
}

fn range_contains_node(index: &ProgramIndex, parent: NodeId, child: NodeId) -> bool {
    let parent = &index.nodes[parent.0];
    let child = &index.nodes[child.0];
    parent.range.start <= child.range.start && parent.range.end >= child.range.end
}

fn ancestor_of_kind(index: &ProgramIndex, node: NodeId, kind: NodeKind) -> Option<NodeId> {
    let mut current = index.nodes[node.0].parent;
    while let Some(id) = current {
        if index.nodes[id.0].kind == kind {
            return Some(id);
        }
        current = index.nodes[id.0].parent;
    }
    None
}

fn call_context(index: &ProgramIndex, node: NodeId) -> CallSiteContext {
    let mut current = index.nodes[node.0].parent;
    while let Some(id) = current {
        let current_node = &index.nodes[id.0];
        match current_node.language_kind.as_str() {
            "expression_statement" => return CallSiteContext::Statement,
            "assignment_expression" | "augmented_assignment_expression" => {
                return CallSiteContext::Assignment;
            }
            "local_variable_declaration" | "variable_declaration" | "lexical_declaration" => {
                return CallSiteContext::DeclarationInitializer;
            }
            "if_statement" | "while_statement" | "for_statement" | "do_statement" => {
                return CallSiteContext::Condition;
            }
            "return_statement" => return CallSiteContext::Return,
            _ => {}
        }
        if matches!(
            current_node.kind,
            NodeKind::Statement | NodeKind::ControlFlow | NodeKind::Loop
        ) {
            return CallSiteContext::NestedExpression;
        }
        current = current_node.parent;
    }
    CallSiteContext::Unknown
}

fn call_name(index: &ProgramIndex, source: &str, node: NodeId) -> Option<String> {
    first_identifier_text(index, source, node)
}

fn first_identifier_text(index: &ProgramIndex, source: &str, node: NodeId) -> Option<String> {
    let current = &index.nodes[node.0];
    if matches!(
        current.language_kind.as_str(),
        "identifier" | "type_identifier"
    ) {
        return Some(node_text(source, current.range.start, current.range.end).to_owned());
    }
    current
        .children
        .iter()
        .find_map(|child| first_identifier_text(index, source, *child))
}

fn estimate_trip_count(text: &str) -> Option<u64> {
    let numbers = text
        .split(|ch: char| !ch.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .filter_map(|part| part.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let max = numbers.iter().copied().max()?;
    let min = numbers.iter().copied().min().unwrap_or(0);
    Some(max.saturating_sub(min).max(max))
}

fn count_identifier_occurrences(source: &str, name: &str) -> usize {
    if name.is_empty() {
        return 0;
    }

    source
        .match_indices(name)
        .filter(|(start, _)| {
            let end = start + name.len();
            let before = source[..*start].chars().next_back();
            let after = source[end..].chars().next();
            !before.is_some_and(is_identifier_part) && !after.is_some_and(is_identifier_part)
        })
        .count()
}

fn is_identifier_part(ch: char) -> bool {
    ch == '_' || ch == '$' || ch.is_ascii_alphanumeric()
}

fn node_text(source: &str, start: usize, end: usize) -> &str {
    source.get(start..end).unwrap_or("")
}
