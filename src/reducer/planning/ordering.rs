//! Candidate ordering and deduplication.
//!
//! Adapters may generate overlapping or duplicate candidates through different
//! tree walks. The reducer normalizes each candidate list before attempting chunks.

use std::collections::HashSet;

use crate::{
    edit::Edit,
    reducer::{candidate::Candidate, group::CandidateGroup},
};

/// Removes duplicate edits and orders high-value candidates first.
pub fn normalize_candidates(mut candidates: Vec<Candidate>) -> Vec<Candidate> {
    let mut seen = HashSet::<Edit>::new();
    candidates.retain(|candidate| seen.insert(candidate.edit.clone()));

    candidates.sort_by(|left, right| {
        right
            .priority
            .cmp(&left.priority)
            .then_with(|| left.estimated_size_delta.cmp(&right.estimated_size_delta))
            .then_with(|| left.id.0.cmp(&right.id.0))
    });

    candidates
}

/// Removes duplicate candidate edits inside each group and orders groups by priority.
pub fn normalize_groups(mut groups: Vec<CandidateGroup>) -> Vec<CandidateGroup> {
    for group in &mut groups {
        group.candidates = normalize_candidates(std::mem::take(&mut group.candidates));
        group.chunk_plan =
            crate::reducer::group::build_chunk_plan(&group.candidates, group.strategy);
    }

    groups.retain(|group| !group.candidates.is_empty());
    groups.sort_by(|left, right| {
        right
            .priority
            .cmp(&left.priority)
            .then_with(|| right.candidates.len().cmp(&left.candidates.len()))
            .then_with(|| left.id.0.cmp(&right.id.0))
    });
    groups
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        edit::{Edit, TextRange},
        reducer::candidate::{StageKind, TransformKind},
    };

    #[test]
    fn normalize_candidates_deduplicates_same_edit() {
        let edit = Edit::Delete(TextRange::new(0, 1).unwrap());
        let candidates = vec![
            Candidate::new(
                StageKind::ExpressionLiteralTypeCleanup,
                TransformKind::Cleanup,
                "a",
                "delete",
                edit.clone(),
                1,
                -1,
            ),
            Candidate::new(
                StageKind::ExpressionLiteralTypeCleanup,
                TransformKind::Cleanup,
                "b",
                "delete",
                edit,
                1,
                -1,
            ),
        ];
        assert_eq!(normalize_candidates(candidates).len(), 1);
    }
}
