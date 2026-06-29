//! Candidate groups and chunk schedules.
//!
//! Groups let the reducer try large, structurally related edits first and then
//! fall back to smaller chunks and single candidates. The group schedule is
//! rebuilt for every fresh snapshot to avoid stale ranges.

use serde::{Deserialize, Serialize};

use crate::ir::{NodeId, SnapshotId};

use super::candidate::{Candidate, CandidateId, StageKind};

/// Stable group identifier.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CandidateGroupId(pub String);

/// Group category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum CandidateGroupKind {
    /// One atomic coordinated edit.
    Atomic,
    /// Items from the same sibling list.
    SiblingList,
    /// Declaration and helper family.
    DeclarationFamily,
    /// Output/log/debug noise.
    OutputNoise,
    /// Dependency closure deletion.
    DependencyClosure,
    /// Alternative literal shrinks.
    LiteralShrinkSet,
    /// Control-flow alternatives.
    ControlPathSet,
}

/// Group reduction strategy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum GroupStrategy {
    /// Try the whole group, chunks, then individual candidates.
    WholeThenChunksThenSingles,
    /// Try individual candidates only.
    SinglesOnly,
    /// Try alternatives one by one and accept the first simpler result.
    Alternatives,
}

/// Candidate ids included in one attempt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChunkAttempt {
    /// Candidate ids attempted together.
    pub ids: Vec<CandidateId>,
    /// English attempt description.
    pub description: String,
}

/// Chunk schedule for one group.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChunkPlan {
    /// Base candidate ids in source or priority order.
    pub base_items: Vec<CandidateId>,
    /// Attempts generated from base items.
    pub schedule: Vec<ChunkAttempt>,
    /// Whether the reducer should fall back to single candidates.
    pub fallback_to_single: bool,
}

/// A related candidate set scheduled as one reducer unit.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CandidateGroup {
    /// Stable group id.
    pub id: CandidateGroupId,
    /// Snapshot version this group belongs to.
    pub snapshot: SnapshotId,
    /// Owning stage.
    pub stage: StageKind,
    /// Region node for this group.
    pub region: Option<NodeId>,
    /// English explanation for reports and logs.
    pub description: String,
    /// Group kind.
    pub kind: CandidateGroupKind,
    /// Candidate list.
    pub candidates: Vec<Candidate>,
    /// Chunking strategy.
    pub strategy: GroupStrategy,
    /// Precomputed chunk plan.
    pub chunk_plan: ChunkPlan,
    /// Higher-priority groups run earlier.
    pub priority: i32,
}

impl CandidateGroup {
    /// Creates a group and derives a chunk plan from the candidate order.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        snapshot: SnapshotId,
        stage: StageKind,
        kind: CandidateGroupKind,
        description: impl Into<String>,
        candidates: Vec<Candidate>,
        strategy: GroupStrategy,
        priority: i32,
    ) -> Self {
        let chunk_plan = build_chunk_plan(&candidates, strategy);
        Self {
            id: CandidateGroupId(id.into()),
            snapshot,
            stage,
            region: None,
            description: description.into(),
            kind,
            candidates,
            strategy,
            chunk_plan,
            priority,
        }
    }
}

/// Builds a conservative chunk plan from a candidate list.
pub fn build_chunk_plan(candidates: &[Candidate], strategy: GroupStrategy) -> ChunkPlan {
    let base_items = candidates
        .iter()
        .map(|candidate| candidate.id.clone())
        .collect::<Vec<_>>();

    if candidates.is_empty() {
        return ChunkPlan {
            base_items,
            schedule: Vec::new(),
            fallback_to_single: true,
        };
    }

    let mut schedule = Vec::new();
    if !matches!(
        strategy,
        GroupStrategy::SinglesOnly | GroupStrategy::Alternatives
    ) && candidates.len() > 1
    {
        schedule.push(ChunkAttempt {
            ids: base_items.clone(),
            description: "Try the whole candidate group".to_owned(),
        });

        let mut parts = 2_usize;
        while parts <= candidates.len() {
            for range in crate::reducer::ddmin::chunk_ranges(candidates.len(), parts) {
                let ids = base_items[range.clone()].to_vec();
                if ids.len() > 1 {
                    schedule.push(ChunkAttempt {
                        ids,
                        description: format!("Try a {parts}-way candidate chunk"),
                    });
                }

                let complement = base_items
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| !range.contains(index))
                    .map(|(_, id)| id.clone())
                    .collect::<Vec<_>>();
                if complement.len() > 1 {
                    schedule.push(ChunkAttempt {
                        ids: complement,
                        description: format!("Try the complement of a {parts}-way chunk"),
                    });
                }
            }
            if parts == candidates.len() {
                break;
            }
            parts = (parts * 2).min(candidates.len());
        }
    }

    schedule.extend(base_items.iter().cloned().map(|id| ChunkAttempt {
        ids: vec![id],
        description: "Try one candidate".to_owned(),
    }));

    ChunkPlan {
        base_items,
        schedule,
        fallback_to_single: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_id_is_reportable() {
        let id = CandidateGroupId("group".to_owned());
        assert_eq!(id.0, "group");
    }
}
