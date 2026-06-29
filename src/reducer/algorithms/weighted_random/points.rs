//! Conversion from generated reducer candidates into weighted points.
//!
//! The weighted loop needs one center byte offset and one candidate per point.
//! This module filters generated candidates down to simple statement-like
//! deletions, deduplicates overlapping generator output, and builds temporary
//! groups for shared logging/reporting.

use std::collections::HashSet;

use crate::{
    edit::{Edit, TextRange},
    ir::SnapshotId,
    reducer::{
        candidate::{Candidate, StageKind, TransformKind},
        engine::ReductionContext,
        group::{CandidateGroup, CandidateGroupKind, GroupStrategy},
        ordering::normalize_candidates,
    },
};

/// One weighted deletion point in the current snapshot.
#[derive(Clone)]
pub(super) struct Point {
    /// Candidate whose edit is tested when this point is sampled.
    pub(super) candidate: Candidate,
    /// Byte center of the candidate edit, used for distance-decayed updates.
    pub(super) center: usize,
    /// Current sampling weight. Higher values make the point more likely.
    pub(super) weight: f64,
}

/// Collects the current statement deletion points from normal candidate generation.
pub(super) fn collect_points(context: &ReductionContext<'_>) -> anyhow::Result<Vec<Point>> {
    // Candidates from adapters and language-neutral generation can overlap.
    // `seen` deduplicates by the final edit span so one source range becomes one
    // random point regardless of how many generators discovered it.
    let mut seen = HashSet::new();

    // Use normal statement-reduction generation with normal stages preserved.
    // The random scheduler chooses singles itself; it does not need the engine
    // to rewrite stages into the final-sweep stage.
    let mut candidates = context
        .generate_groups(StageKind::StatementAndSiblingReduction, false)?
        .into_iter()
        .flat_map(|group| group.candidates)
        .filter(is_point_candidate)
        .collect::<Vec<_>>();
    candidates = normalize_candidates(candidates);

    let mut points = Vec::new();
    for candidate in candidates {
        // The weighted algorithm is point-based, so candidates touching
        // multiple disconnected ranges are skipped here. Later cleanup sweeps
        // can still try coordinated edits one at a time.
        let Some(range) = edit_range(&candidate.plan.as_edit()) else {
            continue;
        };
        if range.is_empty() || !seen.insert((range.start, range.end)) {
            continue;
        }
        let center = (range.start + range.end) / 2;
        points.push(Point {
            candidate,
            center,
            // Every fresh snapshot starts with uniform weights. Local learning
            // then happens inside the current snapshot only.
            weight: 1.0,
        });
    }
    points.sort_by_key(|point| point.center);
    Ok(points)
}

/// Returns true for simple statement-like deletion points.
fn is_point_candidate(candidate: &Candidate) -> bool {
    // Restrict the random loop to direct deletions of statements or output
    // statements. Replacements, literals, declarations, and coordinated edits
    // are handled later by deterministic sweeps where their risk is easier to
    // understand from logs.
    matches!(
        candidate.transform,
        TransformKind::DeleteStatementChunk | TransformKind::RemoveOutputOnlyVariable
    ) && matches!(candidate.plan.as_edit(), Edit::Delete(_))
}

/// Builds a temporary single-candidate group for shared trial logging/reporting.
pub(super) fn point_group(snapshot: SnapshotId, candidate: &Candidate) -> CandidateGroup {
    // This group is not fed through chunk planning. It exists so
    // `try_candidates` can use the same logging shape and report metadata for a
    // random point as it does for deterministic groups.
    CandidateGroup::new(
        format!("weighted-random:point:{}", candidate.id.0),
        snapshot,
        StageKind::StatementAndSiblingReduction,
        CandidateGroupKind::SiblingList,
        "Weighted random statement deletion point",
        vec![candidate.clone()],
        GroupStrategy::SinglesOnly,
        candidate.priority,
    )
}

/// Returns the outer range touched by an edit when it can be represented as one span.
fn edit_range(edit: &Edit) -> Option<TextRange> {
    // A point needs one center position for neighborhood updates. For multi-edits
    // this returns the enclosing range, but point filtering currently keeps only
    // direct deletes. The helper is shared with future point types.
    let mut ranges = Vec::new();
    collect_ranges(edit, &mut ranges);
    let start = ranges.iter().map(|range| range.start).min()?;
    let end = ranges.iter().map(|range| range.end).max()?;
    Some(TextRange { start, end })
}

/// Collects all edit ranges recursively.
fn collect_ranges(edit: &Edit, ranges: &mut Vec<TextRange>) {
    match edit {
        Edit::Delete(range) => ranges.push(range.clone()),
        Edit::Replace { range, .. } => ranges.push(range.clone()),
        Edit::Multi(edits) => {
            for edit in edits {
                collect_ranges(edit, ranges);
            }
        }
    }
}
