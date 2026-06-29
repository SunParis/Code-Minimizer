//! Language-neutral candidate generation for reducer stages.
//!
//! This module derives candidates from snapshot analysis: loop-bound shrinks,
//! call-site neutralization, block/body transforms, statement chunks, and
//! dead/output cleanup. Every edit remains advisory and is validated by the
//! parser, build commands, and oracle before acceptance.

use crate::{
    edit::{Edit, TextRange},
    ir::{CallSiteContext, NodeId, NodeKind, NodeRole, ProgramSnapshot, SiblingListKind},
    reducer::{
        candidate::{Candidate, StageKind, TransformKind},
        group::{CandidateGroup, CandidateGroupKind, GroupStrategy},
    },
};

/// Builds stage-specific groups from the current snapshot.
pub fn generate_phase_groups(stage: StageKind, snapshot: &ProgramSnapshot) -> Vec<CandidateGroup> {
    match stage {
        StageKind::RuntimeCostReduction => runtime_cost_groups(snapshot),
        StageKind::AggressiveFunctionElimination => function_groups(snapshot),
        StageKind::AggressiveBlockElimination => block_groups(snapshot),
        StageKind::StatementAndSiblingReduction => statement_groups(snapshot),
        StageKind::DeadDeclarationAndOutputCleanup => cleanup_groups(snapshot),
        StageKind::ExpressionLiteralTypeCleanup => expression_literal_groups(snapshot),
        StageKind::BaselineAndIndex
        | StageKind::FinalOneMinimalSweep
        | StageKind::BlankLineCleanup => Vec::new(),
    }
}

fn runtime_cost_groups(snapshot: &ProgramSnapshot) -> Vec<CandidateGroup> {
    let mut loops = snapshot.analysis.loops.clone();
    loops.sort_by(|left, right| right.score.cmp(&left.score));
    let limit = loops.len().div_ceil(2).max(1);

    loops
        .into_iter()
        .take(limit)
        .filter_map(|loop_summary| {
            let node = &snapshot.index.nodes[loop_summary.node.0];
            let text = node_text(snapshot, node.id);
            let candidates = loop_bound_candidates(snapshot, node.id, text);
            (!candidates.is_empty()).then(|| {
                CandidateGroup::new(
                    format!("phase:loop:{}", node.range.start),
                    snapshot.version,
                    StageKind::RuntimeCostReduction,
                    CandidateGroupKind::ControlPathSet,
                    "Shrink high-cost loop bound",
                    candidates,
                    GroupStrategy::Alternatives,
                    320 + loop_summary.score.min(500) as i32,
                )
            })
        })
        .collect()
}

fn loop_bound_candidates(snapshot: &ProgramSnapshot, node: NodeId, text: &str) -> Vec<Candidate> {
    let Some((number_range, value)) = largest_integer_literal(snapshot, node, text) else {
        return Vec::new();
    };
    if value <= 1 {
        return Vec::new();
    }

    let mut values = vec![
        value.div_ceil(2),
        value.div_ceil(4),
        value.div_ceil(8),
        16,
        8,
        3,
        2,
        1,
    ];
    values.retain(|candidate| *candidate < value);
    values.sort_unstable_by(|left, right| right.cmp(left));
    values.dedup();

    values
        .into_iter()
        .take(5)
        .map(|new_value| {
            let mut candidate = Candidate::new(
                StageKind::RuntimeCostReduction,
                TransformKind::ShrinkLoopBound,
                format!("phase:loop-bound:{}:{new_value}", number_range.start),
                "Shrink loop bound",
                Edit::Replace {
                    range: number_range.clone(),
                    replacement: new_value.to_string(),
                },
                260,
                new_value.to_string().len() as isize - number_range.len() as isize,
            );
            candidate.target = Some(node);
            candidate
        })
        .collect()
}

fn function_groups(snapshot: &ProgramSnapshot) -> Vec<CandidateGroup> {
    let mut functions = snapshot.analysis.functions.clone();
    functions.sort_by(|left, right| {
        function_score(right)
            .cmp(&function_score(left))
            .then_with(|| right.body_bytes.cmp(&left.body_bytes))
    });
    let limit = ((functions.len() * 3).div_ceil(4)).max(1);

    functions
        .into_iter()
        .take(limit)
        .filter(|function| !function.entry && !function.protected)
        .filter_map(|function| {
            let mut candidates = Vec::new();
            for call_site in &snapshot.analysis.call_sites {
                if call_site.enclosing_function == Some(function.node) {
                    continue;
                }
                if call_site.callee_name.is_some()
                    && function.name.is_some()
                    && call_site.callee_name != function.name
                {
                    continue;
                }
                candidates.extend(call_site_candidates(
                    snapshot,
                    call_site.node,
                    call_site.context,
                ));
            }

            let decl_node = &snapshot.index.nodes[function.node.0];
            let mut delete = Candidate::new(
                StageKind::AggressiveFunctionElimination,
                TransformKind::DeleteFunctionDecl,
                format!("phase:function-delete:{}", decl_node.range.start),
                "Delete function declaration",
                delete_with_padding(&snapshot.source, &decl_node.range),
                180,
                -(decl_node.range.len() as isize),
            );
            delete.target = Some(function.node);
            candidates.push(delete);

            (!candidates.is_empty()).then(|| {
                CandidateGroup::new(
                    format!("phase:function:{}", decl_node.range.start),
                    snapshot.version,
                    StageKind::AggressiveFunctionElimination,
                    CandidateGroupKind::DeclarationFamily,
                    "Remove function call sites and declaration",
                    candidates,
                    GroupStrategy::WholeThenChunksThenSingles,
                    280 + function_score(&function).min(500) as i32,
                )
            })
        })
        .collect()
}

fn call_site_candidates(
    snapshot: &ProgramSnapshot,
    call_node: NodeId,
    context: CallSiteContext,
) -> Vec<Candidate> {
    let call = &snapshot.index.nodes[call_node.0];
    let mut candidates = Vec::new();

    for replacement in default_values_for_context(context) {
        let mut candidate = Candidate::new(
            StageKind::AggressiveFunctionElimination,
            TransformKind::ReplaceCallWithValue,
            format!(
                "phase:call-replace:{}:{}",
                call.range.start,
                replacement.replace('"', "quote")
            ),
            "Replace call with deterministic default value",
            Edit::Replace {
                range: call.range.clone(),
                replacement: (*replacement).to_owned(),
            },
            210,
            replacement.len() as isize - call.range.len() as isize,
        );
        candidate.target = Some(call_node);
        candidates.push(candidate);
    }

    if matches!(context, CallSiteContext::Statement) {
        if let Some(statement) = ancestor_or_self_of_kind(snapshot, call_node, NodeKind::Statement)
        {
            let statement_node = &snapshot.index.nodes[statement.0];
            let mut candidate = Candidate::new(
                StageKind::AggressiveFunctionElimination,
                TransformKind::RemoveCallSite,
                format!("phase:call-delete:{}", statement_node.range.start),
                "Remove call-site statement",
                delete_with_padding(&snapshot.source, &statement_node.range),
                230,
                -(statement_node.range.len() as isize),
            );
            candidate.target = Some(statement);
            candidates.push(candidate);
        }
    }

    candidates
}

fn block_groups(snapshot: &ProgramSnapshot) -> Vec<CandidateGroup> {
    let mut blocks = snapshot.analysis.blocks.clone();
    blocks.sort_by(|left, right| right.score.cmp(&left.score));

    blocks
        .into_iter()
        .filter_map(|block| {
            let block_node = &snapshot.index.nodes[block.node.0];
            let mut candidates = Vec::new();

            if let Some(owner) = block.owner {
                let owner_node = &snapshot.index.nodes[owner.0];
                let mut delete_owner = Candidate::new(
                    StageKind::AggressiveBlockElimination,
                    TransformKind::DeleteWholeConstruct,
                    format!("phase:construct-delete:{}", owner_node.range.start),
                    "Delete whole owner construct",
                    delete_with_padding(&snapshot.source, &owner_node.range),
                    250,
                    -(owner_node.range.len() as isize),
                );
                delete_owner.target = Some(owner);
                candidates.push(delete_owner);
            }

            let replacement = minimal_block_replacement(snapshot, block.node);
            let mut empty = Candidate::new(
                StageKind::AggressiveBlockElimination,
                TransformKind::EmptyBlockBody,
                format!("phase:block-empty:{}", block_node.range.start),
                "Replace block with minimal body",
                Edit::Replace {
                    range: block_node.range.clone(),
                    replacement,
                },
                220,
                2_isize - block_node.range.len() as isize,
            );
            empty.target = Some(block.node);
            candidates.push(empty);

            if let Some(unwrapped) = unwrap_block_candidate(snapshot, block.node) {
                candidates.push(unwrapped);
            }

            (!candidates.is_empty()).then(|| {
                CandidateGroup::new(
                    format!("phase:block:{}", block_node.range.start),
                    snapshot.version,
                    StageKind::AggressiveBlockElimination,
                    CandidateGroupKind::ControlPathSet,
                    "Delete, empty, or unwrap block",
                    candidates,
                    GroupStrategy::Alternatives,
                    260 + block.score.min(500) as i32,
                )
            })
        })
        .collect()
}

fn statement_groups(snapshot: &ProgramSnapshot) -> Vec<CandidateGroup> {
    let mut groups = Vec::new();
    for list in &snapshot.index.sibling_lists {
        if list.kind != SiblingListKind::Statements || list.items.is_empty() {
            continue;
        }
        let mut candidates = Vec::new();
        for item in &list.items {
            let node = &snapshot.index.nodes[item.0];
            let mut candidate = Candidate::new(
                StageKind::StatementAndSiblingReduction,
                TransformKind::DeleteStatementChunk,
                format!("phase:statement-delete:{}", node.range.start),
                "Delete statement",
                delete_with_padding(&snapshot.source, &node.range),
                statement_priority(node.kind.clone(), node.role),
                -(node.range.len() as isize),
            );
            candidate.target = Some(*item);
            candidates.push(candidate);
        }

        candidates.extend(statement_window_candidates(snapshot, &list.items));

        groups.push(CandidateGroup::new(
            format!("phase:statements:{}", list.id.0),
            snapshot.version,
            StageKind::StatementAndSiblingReduction,
            CandidateGroupKind::SiblingList,
            "Reduce statement sibling list",
            candidates,
            GroupStrategy::WholeThenChunksThenSingles,
            220 + list.items.len() as i32,
        ));
    }
    groups
}

fn cleanup_groups(snapshot: &ProgramSnapshot) -> Vec<CandidateGroup> {
    let mut candidates = Vec::new();

    for def_use in &snapshot.analysis.def_uses {
        let node = &snapshot.index.nodes[def_use.declaration.0];
        if def_use.references == 0 || def_use.references == def_use.output_references {
            let transform = if def_use.output_references > 0 {
                TransformKind::RemoveOutputOnlyVariable
            } else {
                TransformKind::DeleteDeadDeclaration
            };
            let mut candidate = Candidate::new(
                StageKind::DeadDeclarationAndOutputCleanup,
                transform,
                format!("phase:dead-decl:{}", node.range.start),
                "Delete dead declaration",
                delete_with_padding(&snapshot.source, &node.range),
                230,
                -(node.range.len() as isize),
            );
            candidate.target = Some(def_use.declaration);
            candidates.push(candidate);
        }
    }

    for node in &snapshot.index.nodes {
        if matches!(node.role, NodeRole::OutputOperation) {
            let target =
                ancestor_or_self_of_kind(snapshot, node.id, NodeKind::Statement).unwrap_or(node.id);
            let target_node = &snapshot.index.nodes[target.0];
            let mut candidate = Candidate::new(
                StageKind::DeadDeclarationAndOutputCleanup,
                TransformKind::RemoveOutputOnlyVariable,
                format!("phase:output-delete:{}", target_node.range.start),
                "Delete output statement",
                delete_with_padding(&snapshot.source, &target_node.range),
                300,
                -(target_node.range.len() as isize),
            );
            candidate.target = Some(target);
            candidates.push(candidate);
        }
    }

    if candidates.is_empty() {
        Vec::new()
    } else {
        vec![CandidateGroup::new(
            "phase:cleanup",
            snapshot.version,
            StageKind::DeadDeclarationAndOutputCleanup,
            CandidateGroupKind::OutputNoise,
            "Clean up dead declarations and output-only code",
            candidates,
            GroupStrategy::WholeThenChunksThenSingles,
            240,
        )]
    }
}

fn expression_literal_groups(snapshot: &ProgramSnapshot) -> Vec<CandidateGroup> {
    let mut candidates = Vec::new();
    for node in &snapshot.index.nodes {
        if matches!(node.kind, NodeKind::Literal) {
            for replacement in literal_replacements(node_text(snapshot, node.id)) {
                let mut candidate = Candidate::new(
                    StageKind::ExpressionLiteralTypeCleanup,
                    TransformKind::ShrinkLiteral,
                    format!("phase:literal:{}:{replacement}", node.range.start),
                    "Shrink literal",
                    Edit::Replace {
                        range: node.range.clone(),
                        replacement: (*replacement).to_owned(),
                    },
                    160,
                    replacement.len() as isize - node.range.len() as isize,
                );
                candidate.target = Some(node.id);
                candidates.push(candidate);
            }
        }
        if matches!(node.kind, NodeKind::Trivia) {
            let mut candidate = Candidate::new(
                StageKind::ExpressionLiteralTypeCleanup,
                TransformKind::Cleanup,
                format!("phase:trivia-delete:{}", node.range.start),
                "Delete trivia",
                delete_with_padding(&snapshot.source, &node.range),
                120,
                -(node.range.len() as isize),
            );
            candidate.target = Some(node.id);
            candidates.push(candidate);
        }
    }

    if candidates.is_empty() {
        Vec::new()
    } else {
        vec![CandidateGroup::new(
            "phase:expression-literal-cleanup",
            snapshot.version,
            StageKind::ExpressionLiteralTypeCleanup,
            CandidateGroupKind::LiteralShrinkSet,
            "Clean expressions, literals, and trivia",
            candidates,
            GroupStrategy::Alternatives,
            160,
        )]
    }
}

fn statement_window_candidates(snapshot: &ProgramSnapshot, items: &[NodeId]) -> Vec<Candidate> {
    if items.len() < 8 {
        return Vec::new();
    }
    let mut candidates = Vec::new();
    for window_size in [16_usize, 8, 4, 2] {
        if window_size > items.len() {
            continue;
        }
        let step = (window_size / 2).max(1);
        let mut start = 0;
        while start + window_size <= items.len() {
            let window = &items[start..start + window_size];
            let edits = window
                .iter()
                .map(|id| {
                    let node = &snapshot.index.nodes[id.0];
                    delete_with_padding(&snapshot.source, &node.range)
                })
                .collect::<Vec<_>>();
            let removed = edits.iter().map(Edit::removed_bytes).sum::<usize>();
            let mut candidate = Candidate::new(
                StageKind::StatementAndSiblingReduction,
                TransformKind::DeleteStatementChunk,
                format!(
                    "phase:statement-window:{}:{}",
                    snapshot.index.nodes[window[0].0].range.start, window_size
                ),
                "Delete statement window",
                Edit::Multi(edits),
                240 + window_size as i32,
                -(removed as isize),
            );
            candidate.target = Some(window[0]);
            candidates.push(candidate);
            start += step;
        }
    }
    candidates
}

fn unwrap_block_candidate(snapshot: &ProgramSnapshot, block: NodeId) -> Option<Candidate> {
    let node = &snapshot.index.nodes[block.0];
    let text = node_text(snapshot, block).trim();
    if !text.starts_with('{') || !text.ends_with('}') {
        return None;
    }
    let inner_start = node.range.start + text.find('{')? + 1;
    let inner_end = node.range.end - text.chars().rev().position(|ch| ch == '}')? - 1;
    if inner_start > inner_end || !snapshot.source.is_char_boundary(inner_start) {
        return None;
    }
    let inner = snapshot
        .source
        .get(inner_start..inner_end)?
        .trim()
        .to_owned();
    let mut candidate = Candidate::new(
        StageKind::AggressiveBlockElimination,
        TransformKind::UnwrapBlock,
        format!("phase:block-unwrap:{}", node.range.start),
        "Unwrap block body",
        Edit::Replace {
            range: node.range.clone(),
            replacement: inner,
        },
        180,
        -(2_isize),
    );
    candidate.target = Some(block);
    Some(candidate)
}

fn minimal_block_replacement(snapshot: &ProgramSnapshot, block: NodeId) -> String {
    let Some(function) = ancestor_or_self_of_kind(snapshot, block, NodeKind::FunctionDecl) else {
        return "{}".to_owned();
    };
    let text = node_text(snapshot, function);
    if text.contains(" boolean ") || text.contains("boolean ") {
        "{ return false; }".to_owned()
    } else if text.contains(" void ") || text.contains("void ") {
        "{}".to_owned()
    } else if text.contains(" int ")
        || text.contains(" long ")
        || text.contains(" short ")
        || text.contains(" byte ")
        || text.contains(" float ")
        || text.contains(" double ")
    {
        "{ return 0; }".to_owned()
    } else {
        "{}".to_owned()
    }
}

fn default_values_for_context(context: CallSiteContext) -> &'static [&'static str] {
    match context {
        CallSiteContext::Condition => &["false", "true"],
        CallSiteContext::Statement => &[],
        CallSiteContext::Assignment
        | CallSiteContext::DeclarationInitializer
        | CallSiteContext::Return
        | CallSiteContext::NestedExpression
        | CallSiteContext::Unknown => &["0", "false", "null", "undefined", "\"\""],
    }
}

fn literal_replacements(text: &str) -> &'static [&'static str] {
    let trimmed = text.trim();
    if trimmed.starts_with('"') || trimmed.starts_with('`') {
        &["\"\""]
    } else if trimmed == "true" || trimmed == "false" {
        &["false", "true"]
    } else if trimmed.starts_with('[') {
        &["[]"]
    } else if trimmed.starts_with('{') {
        &["{}"]
    } else if trimmed.chars().any(|ch| ch.is_ascii_digit()) {
        &["0", "1", "-1"]
    } else {
        &[]
    }
}

fn largest_integer_literal(
    snapshot: &ProgramSnapshot,
    node: NodeId,
    text: &str,
) -> Option<(TextRange, u64)> {
    let node_start = snapshot.index.nodes[node.0].range.start;
    let mut best = None::<(TextRange, u64)>;
    let mut offset = 0;
    let bytes = text.as_bytes();
    while offset < bytes.len() {
        if !bytes[offset].is_ascii_digit() {
            offset += 1;
            continue;
        }
        let start = offset;
        while offset < bytes.len() && bytes[offset].is_ascii_digit() {
            offset += 1;
        }
        let value = text[start..offset].parse::<u64>().ok()?;
        let range = TextRange {
            start: node_start + start,
            end: node_start + offset,
        };
        if best.as_ref().is_none_or(|(_, old)| value > *old) {
            best = Some((range, value));
        }
    }
    best
}

fn function_score(function: &crate::ir::FunctionSummary) -> u64 {
    function.body_bytes as u64
        + function.statements as u64 * 10
        + function.loops as u64 * 80
        + function.call_sites.len() as u64 * 30
}

fn statement_priority(kind: NodeKind, role: NodeRole) -> i32 {
    if role == NodeRole::OutputOperation {
        return 280;
    }
    match kind {
        NodeKind::Declaration => 230,
        NodeKind::CallExpr => 220,
        NodeKind::Statement => 200,
        NodeKind::ControlFlow | NodeKind::Loop => 170,
        _ => 140,
    }
}

fn ancestor_or_self_of_kind(
    snapshot: &ProgramSnapshot,
    node: NodeId,
    kind: NodeKind,
) -> Option<NodeId> {
    let mut current = Some(node);
    while let Some(id) = current {
        if snapshot.index.nodes[id.0].kind == kind {
            return Some(id);
        }
        current = snapshot.index.nodes[id.0].parent;
    }
    None
}

fn delete_with_padding(source: &str, range: &TextRange) -> Edit {
    let mut start = range.start;
    let mut end = range.end;
    let bytes = source.as_bytes();
    while start > 0 && matches!(bytes[start - 1], b' ' | b'\t') {
        start -= 1;
    }
    while end < bytes.len() && matches!(bytes[end], b' ' | b'\t') {
        end += 1;
    }
    if end < bytes.len() && bytes[end] == b'\r' {
        end += 1;
    }
    if end < bytes.len() && bytes[end] == b'\n' {
        end += 1;
    } else if start > 0 && bytes[start - 1] == b'\n' {
        start -= 1;
    }
    Edit::Delete(TextRange { start, end })
}

fn node_text(snapshot: &ProgramSnapshot, node: NodeId) -> &str {
    let range = &snapshot.index.nodes[node.0].range;
    snapshot.source.get(range.start..range.end).unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ir::ProgramSnapshot,
        lang::{java::JavaAdapter, javascript::JavaScriptAdapter},
    };

    fn java_snapshot(source: &str) -> ProgramSnapshot {
        ProgramSnapshot::build(
            crate::ir::SnapshotId::ROOT,
            source.to_owned(),
            "Test.java",
            &JavaAdapter::new(),
        )
        .unwrap()
    }

    fn js_snapshot(source: &str) -> ProgramSnapshot {
        ProgramSnapshot::build(
            crate::ir::SnapshotId::ROOT,
            source.to_owned(),
            "case.js",
            &JavaScriptAdapter::new(),
        )
        .unwrap()
    }

    #[test]
    fn stage_generation_produces_loop_bound_shrink_candidates() {
        let snapshot = java_snapshot(
            "class Test { static void f(){ for (int i = 0; i < 1000; i++) { g(); } } static void g(){} }",
        );
        let groups = generate_phase_groups(StageKind::RuntimeCostReduction, &snapshot);

        assert!(
            groups
                .iter()
                .flat_map(|group| &group.candidates)
                .any(|candidate| candidate.transform == TransformKind::ShrinkLoopBound),
            "Runtime-cost stage should generate loop-bound shrink candidates"
        );
    }

    #[test]
    fn stage_generation_produces_call_site_replacement_candidates() {
        let snapshot = java_snapshot(
            "class Test { static int helper(){ return 1; } static void f(){ int x = helper(); } }",
        );
        let groups = generate_phase_groups(StageKind::AggressiveFunctionElimination, &snapshot);

        assert!(
            groups
                .iter()
                .flat_map(|group| &group.candidates)
                .any(|candidate| candidate.transform == TransformKind::ReplaceCallWithValue),
            "Function stage should generate call replacement candidates"
        );
    }

    #[test]
    fn stage_generation_produces_statement_window_candidates_for_long_blocks() {
        let snapshot = js_snapshot(
            "function f(){ let a=1; let b=2; let c=3; let d=4; let e=5; let f=6; let g=7; let h=8; let i=9; }",
        );
        let groups = generate_phase_groups(StageKind::StatementAndSiblingReduction, &snapshot);

        assert!(
            groups
                .iter()
                .flat_map(|group| &group.candidates)
                .any(|candidate| candidate.description == "Delete statement window"),
            "Statement stage should generate sliding-window chunks"
        );
    }

    #[test]
    fn stage_generation_produces_output_cleanup_candidates() {
        let snapshot = js_snapshot("function f(){ console.log('noise'); let keep = 1; }");
        let groups = generate_phase_groups(StageKind::DeadDeclarationAndOutputCleanup, &snapshot);

        assert!(
            groups
                .iter()
                .flat_map(|group| &group.candidates)
                .any(|candidate| candidate.transform == TransformKind::RemoveOutputOnlyVariable),
            "Cleanup stage should prioritize output cleanup"
        );
    }
}
