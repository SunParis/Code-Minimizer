//! Mutable engine state for one reduction session.
//!
//! This state is separate from `ReducerEngine` so the algorithm-facing context
//! can mutate only the session data that is supposed to change: the current
//! snapshot, trial counters, cache, and final diff.

use std::collections::HashSet;

use crate::{
    cache::{CacheKey, TrialCache},
    ir::{ProgramSnapshot, SnapshotId},
    oracle::Baseline,
    output_diff::OutputDiff,
};

/// Mutable state owned by the engine while reducing.
pub(super) struct EngineState {
    /// Current accepted snapshot. All generated candidates must target this snapshot.
    pub(super) current: ProgramSnapshot,
    /// Snapshot id assigned to the next accepted candidate before it becomes current.
    pub(super) next_snapshot: SnapshotId,
    /// Baseline A/B observation that every candidate must preserve.
    pub(super) baseline: Baseline,
    /// Oracle-result cache keyed by source text and oracle configuration.
    pub(super) cache: TrialCache,
    /// Source cache keys that have already been accepted in this session.
    pub(super) accepted_sources: HashSet<CacheKey>,
    /// Monotonic count of fresh oracle trials after baseline validation.
    pub(super) trial_id: usize,
    /// Total accepted attempts, including accepted cache hits.
    pub(super) accepted_total: usize,
    /// Total rejected/skipped attempts reported by algorithms.
    pub(super) rejected_total: usize,
    /// Latest known diff state for the current accepted source.
    pub(super) final_diff: OutputDiff,
    /// True when the configured max-trials limit stopped reduction.
    pub(super) stopped_by_trial_limit: bool,
    /// True when a configured accepted-source size target stopped reduction.
    pub(super) stopped_by_size_limit: bool,
    /// Shutdown signal that stopped reduction after child-process cleanup.
    pub(super) interrupted_by_signal: Option<i32>,
}

impl EngineState {
    /// Creates initial reducer state after baseline validation.
    pub(super) fn new(
        initial_snapshot: ProgramSnapshot,
        baseline: Baseline,
        final_diff: OutputDiff,
        oracle_fingerprint: &str,
    ) -> Self {
        let mut accepted_sources = HashSet::new();
        accepted_sources.insert(CacheKey::new(&initial_snapshot.source, oracle_fingerprint));
        let next_snapshot = initial_snapshot.version.next();

        Self {
            current: initial_snapshot,
            next_snapshot,
            baseline,
            cache: TrialCache::default(),
            accepted_sources,
            trial_id: 0,
            accepted_total: 0,
            rejected_total: 0,
            final_diff,
            stopped_by_trial_limit: false,
            stopped_by_size_limit: false,
            interrupted_by_signal: None,
        }
    }
}
