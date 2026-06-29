//! Trial and acceptance records.
//!
//! The reducer records enough information to explain why candidates were
//! accepted, rejected, skipped by cache, or skipped because they were not
//! structurally simpler.

use serde::{Deserialize, Serialize};

use crate::ir::{ComplexityScore, SnapshotId, SourceHash, StructureHash, score::ScoreDelta};

use super::{
    candidate::{CandidateId, StageKind},
    group::CandidateGroupId,
};

/// Monotonic trial id.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TrialId(pub usize);

/// Stable rejection reason category.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum RejectionReason {
    /// Edit could not be applied.
    RangeError,
    /// Parser rejected the edited source.
    ParseFailure,
    /// Edited source was not structurally simpler.
    NotSimpler,
    /// Build command failed.
    BuildFailure,
    /// Run command failed.
    RunFailure,
    /// A command timed out.
    Timeout,
    /// Output exceeded capture limits.
    OutputTruncated,
    /// Required A/B diff disappeared.
    DiffLost,
    /// Exit preservation policy failed.
    ExitPolicyMismatch,
    /// Command could not be launched or reducer internal command handling failed.
    CommandError,
    /// Rejected result came from the cache.
    CacheHitRejected,
    /// Rejected by the oracle with a free-form reason.
    OracleRejected(String),
}

/// Outcome of a reducer trial.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TrialOutcome {
    /// Accepted and produced a new snapshot.
    Accepted { new_snapshot: SnapshotId },
    /// Rejected after validation.
    Rejected,
    /// Skipped because the oracle cache already had a result.
    SkippedByCache,
    /// Skipped before oracle validation because the score did not decrease.
    SkippedAsNotSimpler,
}

/// Full record for one candidate or chunk attempt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TrialRecord {
    /// Trial id.
    pub id: TrialId,
    /// Parent snapshot version.
    pub parent_snapshot: SnapshotId,
    /// Candidate id or synthetic chunk id.
    pub candidate: CandidateId,
    /// Owning group when available.
    pub group: Option<CandidateGroupId>,
    /// Candidate source hash.
    pub source_hash: SourceHash,
    /// Parsed structure hash when parsing succeeded.
    pub structure_hash: Option<StructureHash>,
    /// Score before the edit.
    pub score_before: ComplexityScore,
    /// Score after the edit when available.
    pub score_after: Option<ComplexityScore>,
    /// Trial outcome.
    pub outcome: TrialOutcome,
    /// Rejection reason when rejected.
    pub rejection_reason: Option<RejectionReason>,
}

/// Record for one accepted candidate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AcceptRecord {
    /// Trial id.
    pub trial: TrialId,
    /// Stage that accepted the candidate.
    pub stage: StageKind,
    /// Group id when available.
    pub group: Option<CandidateGroupId>,
    /// Candidate id or synthetic chunk id.
    pub candidate: CandidateId,
    /// Previous snapshot version.
    pub old_snapshot: SnapshotId,
    /// New snapshot version.
    pub new_snapshot: SnapshotId,
    /// Score delta.
    pub score_delta: ScoreDelta,
    /// English description.
    pub description: String,
}

/// Aggregated rejection counters for reports and future adaptive scheduling.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RejectionStats {
    /// Number of parse failures.
    pub parse_failures: usize,
    /// Number of not-simpler skips.
    pub not_simpler: usize,
    /// Number of oracle rejections.
    pub oracle_rejections: usize,
    /// Number of invalid range/edit failures.
    pub range_errors: usize,
}

impl RejectionStats {
    /// Adds one categorized rejection.
    pub fn record(&mut self, reason: &RejectionReason) {
        match reason {
            RejectionReason::ParseFailure => self.parse_failures += 1,
            RejectionReason::NotSimpler => self.not_simpler += 1,
            RejectionReason::RangeError => self.range_errors += 1,
            _ => self.oracle_rejections += 1,
        }
    }
}
