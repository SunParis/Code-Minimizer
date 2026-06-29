//! Candidate edit and group-shaping helpers.
//!
//! Candidate generation can produce broad groups that cover several structural
//! regions. This module normalizes those groups into smaller regions and owns
//! low-level edit extraction helpers used by the engine.

use crate::{
    edit::{Edit, TextRange},
    ir::{NodeId, ProgramIndex},
};

use super::{
    candidate::{Candidate, StageKind, TransformKind},
    group::{CandidateGroup, CandidateGroupKind, GroupStrategy},
};

/// Builds the low-level edit for a chunk of candidates.
pub(super) fn edit_for_candidates(candidates: &[Candidate]) -> Edit {
    if candidates.len() == 1 {
        candidates[0].plan.as_edit()
    } else {
        Edit::Multi(
            candidates
                .iter()
                .flat_map(|candidate| candidate.plan.edits.clone())
                .collect(),
        )
    }
}

/// Finds the smallest indexed node covered by a candidate edit.
pub(super) fn target_for_candidate(candidate: &Candidate, index: &ProgramIndex) -> Option<NodeId> {
    let range = candidate_edit_range(candidate)?;
    index
        .nodes
        .iter()
        .filter(|node| node.range.start <= range.start && node.range.end >= range.end)
        .min_by_key(|node| node.range.len())
        .map(|node| node.id)
}

/// Returns the outer range touched by a candidate when it is a simple edit.
fn candidate_edit_range(candidate: &Candidate) -> Option<TextRange> {
    let mut ranges = Vec::new();
    collect_edit_ranges(&candidate.plan.as_edit(), &mut ranges);
    let start = ranges.iter().map(|range| range.start).min()?;
    let end = ranges.iter().map(|range| range.end).max()?;
    Some(TextRange { start, end })
}

/// Collects all low-level ranges in an edit tree.
fn collect_edit_ranges(edit: &Edit, ranges: &mut Vec<TextRange>) {
    match edit {
        Edit::Delete(range) => ranges.push(range.clone()),
        Edit::Replace { range, .. } => ranges.push(range.clone()),
        Edit::Multi(edits) => {
            for edit in edits {
                collect_edit_ranges(edit, ranges);
            }
        }
    }
}

/// Splits large adapter groups into smaller structure-region groups.
pub(super) fn regroup_candidates_by_region(
    stage: StageKind,
    groups: Vec<CandidateGroup>,
    index: &ProgramIndex,
) -> Vec<CandidateGroup> {
    let mut regrouped = Vec::new();
    for group in groups {
        if group.candidates.len() <= 1
            || matches!(stage, StageKind::DeadDeclarationAndOutputCleanup)
        {
            regrouped.push(group);
            continue;
        }

        let mut buckets = Vec::<(GroupBucketKey, Vec<Candidate>)>::new();
        for candidate in group.candidates {
            let region = candidate
                .target
                .and_then(|target| reduction_region_for_target(target, index));
            let key = GroupBucketKey {
                region,
                transform: candidate.transform,
                intent: format!("{:?}", candidate.plan.intent),
            };
            if let Some((_, bucket)) = buckets.iter_mut().find(|(id, _)| *id == key) {
                bucket.push(candidate);
            } else {
                buckets.push((key, vec![candidate]));
            }
        }

        for (bucket_index, (key, candidates)) in buckets.into_iter().enumerate() {
            if candidates.is_empty() {
                continue;
            }
            let strategy = if candidates.len() == 1 {
                GroupStrategy::SinglesOnly
            } else {
                group.strategy
            };
            let mut split = CandidateGroup::new(
                format!("{}:region:{bucket_index}", group.id.0),
                group.snapshot,
                group.stage,
                group_kind_for_split(stage, group.kind),
                format!("{} region {bucket_index}", group.description),
                candidates,
                strategy,
                group.priority,
            );
            split.region = key.region;
            regrouped.push(split);
        }
    }
    regrouped
}

/// Key used to split broad adapter groups into non-overlapping reducer groups.
#[derive(Clone, Debug, Eq, PartialEq)]
struct GroupBucketKey {
    region: Option<NodeId>,
    transform: TransformKind,
    intent: String,
}

/// Returns the enclosing parent used as a reduction region.
fn reduction_region_for_target(target: NodeId, index: &ProgramIndex) -> Option<NodeId> {
    let mut current = Some(target);
    while let Some(id) = current {
        let node = &index.nodes[id.0];
        if matches!(
            node.kind,
            crate::ir::NodeKind::Block
                | crate::ir::NodeKind::FunctionDecl
                | crate::ir::NodeKind::TypeDecl
                | crate::ir::NodeKind::Program
        ) {
            return Some(id);
        }
        current = node.parent;
    }
    None
}

/// Preserves useful group categories after splitting.
fn group_kind_for_split(stage: StageKind, fallback: CandidateGroupKind) -> CandidateGroupKind {
    match stage {
        StageKind::AggressiveFunctionElimination | StageKind::DeadDeclarationAndOutputCleanup => {
            CandidateGroupKind::DeclarationFamily
        }
        StageKind::RuntimeCostReduction | StageKind::AggressiveBlockElimination => {
            CandidateGroupKind::ControlPathSet
        }
        StageKind::ExpressionLiteralTypeCleanup => CandidateGroupKind::LiteralShrinkSet,
        _ => fallback,
    }
}

/// Bounds how long one structurally difficult group can monopolize a stage.
pub(super) fn group_failure_budget(group: &CandidateGroup) -> usize {
    match group.stage {
        StageKind::RuntimeCostReduction => 1,
        StageKind::AggressiveFunctionElimination => 16,
        StageKind::AggressiveBlockElimination => 12,
        StageKind::StatementAndSiblingReduction => 20,
        StageKind::DeadDeclarationAndOutputCleanup => 16,
        StageKind::ExpressionLiteralTypeCleanup => 12,
        StageKind::BaselineAndIndex
        | StageKind::FinalOneMinimalSweep
        | StageKind::BlankLineCleanup => 12,
    }
}

#[cfg(test)]
mod tests {
    use crate::edit::TextRange;

    use super::*;

    #[test]
    fn edit_for_multiple_candidates_builds_multi_edit() {
        let a = Candidate::new(
            StageKind::StatementAndSiblingReduction,
            TransformKind::DeleteStatementChunk,
            "a",
            "Delete a",
            Edit::Delete(TextRange { start: 0, end: 1 }),
            1,
            -1,
        );
        let b = Candidate::new(
            StageKind::StatementAndSiblingReduction,
            TransformKind::DeleteStatementChunk,
            "b",
            "Delete b",
            Edit::Delete(TextRange { start: 2, end: 3 }),
            1,
            -1,
        );

        assert!(matches!(edit_for_candidates(&[a, b]), Edit::Multi(_)));
    }
}
