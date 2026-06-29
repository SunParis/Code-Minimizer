//! Shared candidate trial execution path.
//!
//! Every algorithm eventually asks the reducer to test a set of candidates.
//! This module owns that path: apply edits, rebuild a snapshot, check the
//! simplification objective, consult/update the cache, run the oracle, and
//! return a structured `TrialAttempt`.

use anyhow::Context;

use crate::{
    cache::{CacheKey, CachedTrial},
    ir::ProgramSnapshot,
    logging,
    oracle::Oracle,
    report::StageReport,
    runner::received_signal,
    workspace::SessionWorkspace,
};

use super::{
    attempt::TrialAttempt,
    engine::ReducerEngine,
    objective::{SimplificationObjective, candidate_satisfies_objective},
    reporting::score_summary,
    state::EngineState,
    validation::reject_invalid_snapshot,
};
use crate::reducer::{
    candidate::Candidate, group::CandidateGroup, planning::grouping::edit_for_candidates,
};

impl ReducerEngine {
    /// Applies and validates one chunk or single candidate.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn try_candidates(
        &self,
        candidates: &[Candidate],
        group: &CandidateGroup,
        workspace: &SessionWorkspace,
        oracle: &Oracle,
        state: &mut EngineState,
        report: &mut StageReport,
        attempt_description: &str,
        objective: SimplificationObjective,
    ) -> anyhow::Result<TrialAttempt> {
        if let Some(signal) = received_signal() {
            state.interrupted_by_signal = Some(signal);
            return Ok(TrialAttempt::Interrupted);
        }

        let edit = edit_for_candidates(candidates);
        let candidate_source = match edit.apply(&state.current.source) {
            Ok(candidate_source) if candidate_source != state.current.source => candidate_source,
            Ok(_) => return Ok(TrialAttempt::InvalidEdit),
            Err(_) => return Ok(TrialAttempt::InvalidEdit),
        };

        let candidate_snapshot = match self.build_snapshot(
            state.next_snapshot,
            candidate_source,
            &state.current.file_name,
        ) {
            Ok(snapshot) => snapshot,
            Err(_) => return Ok(TrialAttempt::InvalidEdit),
        };
        if !candidate_snapshot.parsed.diagnostics.is_empty() {
            return Ok(TrialAttempt::InvalidEdit);
        }
        if !candidate_satisfies_objective(
            objective,
            &candidate_snapshot.score,
            &state.current.score,
        ) {
            return Ok(TrialAttempt::NotSimpler);
        }

        let key = CacheKey::new(
            &candidate_snapshot.source,
            &self.config.oracle_fingerprint(),
        );
        if state.accepted_sources.contains(&key) {
            return Ok(TrialAttempt::CachedRejected(
                "Candidate source was already accepted earlier".to_owned(),
            ));
        }

        if let Some(cached) = state.cache.get(&key) {
            return Ok(match cached {
                CachedTrial::Accepted => {
                    let snapshot = self.accept_snapshot(candidate_snapshot, state, key)?;
                    state.accepted_total += 1;
                    TrialAttempt::CachedAccepted(snapshot)
                }
                CachedTrial::Rejected(reason) => TrialAttempt::CachedRejected(reason),
            });
        }

        if let Some(signal) = received_signal() {
            state.interrupted_by_signal = Some(signal);
            return Ok(TrialAttempt::Interrupted);
        }
        state.trial_id += 1;
        report.trials += 1;
        let current_trial_id = state.trial_id;
        let decision = match oracle.evaluate_candidate(
            workspace,
            &candidate_snapshot.source,
            current_trial_id,
            &state.baseline,
        ) {
            Ok(decision) => decision,
            Err(error) => {
                if let Some(signal) = received_signal() {
                    state.interrupted_by_signal = Some(signal);
                    return Ok(TrialAttempt::Interrupted);
                }
                return Err(error);
            }
        };
        if let Some(signal) = received_signal() {
            state.interrupted_by_signal = Some(signal);
            return Ok(TrialAttempt::Interrupted);
        }

        if decision.accepted {
            if let Some(diff) = decision.diff {
                state.final_diff = diff;
            }
            state.cache.insert(key.clone(), CachedTrial::Accepted);
            let snapshot = self.accept_snapshot(candidate_snapshot, state, key)?;
            state.accepted_total += 1;
            logging::info(format_args!(
                "trial {current_trial_id}: accepted, phase={} ({}), group={}, attempt={}, size {} -> {}, score {} -> {}",
                group.stage.display_name(),
                group.stage.as_str(),
                group.id.0,
                attempt_description,
                state.current.source.len(),
                snapshot.source.len(),
                score_summary(&state.current.score),
                score_summary(&snapshot.score)
            ));
            Ok(TrialAttempt::Accepted(snapshot))
        } else {
            let reason = decision
                .reason
                .unwrap_or_else(|| "Candidate was rejected".to_owned());
            logging::info(format_args!(
                "trial {current_trial_id}: rejected, phase={} ({}), group={} [{}], attempt={}, candidates={}, reason={reason}",
                group.stage.display_name(),
                group.stage.as_str(),
                group.id.0,
                group.description,
                attempt_description,
                candidates.len()
            ));
            state
                .cache
                .insert(key, CachedTrial::Rejected(reason.clone()));
            Ok(TrialAttempt::Rejected(reason))
        }
    }

    /// Marks a candidate snapshot as accepted and advances the snapshot version.
    fn accept_snapshot(
        &self,
        mut snapshot: ProgramSnapshot,
        state: &mut EngineState,
        key: CacheKey,
    ) -> anyhow::Result<ProgramSnapshot> {
        let normalized = self
            .adapter
            .normalize_after_accept(&snapshot.source, &snapshot.file_name)
            .context("Language normalization failed after accepted edit")?;

        if normalized != snapshot.source {
            let file_name = snapshot.file_name.clone();
            snapshot = self.build_snapshot(state.next_snapshot, normalized, &file_name)?;
            reject_invalid_snapshot(&snapshot, "Normalized accepted source became unparsable")?;
        }

        state.accepted_sources.insert(key);
        state.next_snapshot = state.next_snapshot.next();
        Ok(snapshot)
    }
}
