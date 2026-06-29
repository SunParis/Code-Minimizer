//! Algorithm-facing reducer context.
//!
//! Concrete algorithms should not reach into the engine internals directly.
//! This context exposes the stable services they need: current snapshot access,
//! candidate generation, trial execution, stage reporting, and accepted-source
//! commits.

use crate::{
    config::ReduceConfig,
    ir::{ProgramSnapshot, SnapshotId},
    oracle::Oracle,
    report::{ReductionReport, StageReport},
    runner::received_signal,
    workspace::SessionWorkspace,
};

use super::{
    attempt::TrialAttempt, engine::ReducerEngine, objective::SimplificationObjective,
    state::EngineState, validation::reject_invalid_snapshot,
};
use crate::reducer::{
    candidate::{Candidate, StageKind},
    group::CandidateGroup,
};

/// Shared reducer services available to concrete reduction algorithms.
pub struct ReductionContext<'a> {
    /// Owning engine, used only as a gateway to immutable config, adapter, and helper methods.
    engine: &'a ReducerEngine,
    /// Session workspace that owns accepted/current trial files.
    workspace: &'a SessionWorkspace,
    /// Oracle that executes A/B commands and decides interestingness.
    oracle: &'a Oracle,
    /// Mutable reduction state shared by all algorithm passes.
    state: &'a mut EngineState,
    /// Session report being accumulated until final output confirmation.
    report: &'a mut ReductionReport,
}

impl<'a> ReductionContext<'a> {
    /// Creates the context used for one algorithm run.
    pub(super) fn new(
        engine: &'a ReducerEngine,
        workspace: &'a SessionWorkspace,
        oracle: &'a Oracle,
        state: &'a mut EngineState,
        report: &'a mut ReductionReport,
    ) -> Self {
        Self {
            engine,
            workspace,
            oracle,
            state,
            report,
        }
    }

    /// Returns immutable session configuration.
    pub fn config(&self) -> &ReduceConfig {
        &self.engine.config
    }

    /// Returns the current accepted snapshot.
    pub fn current(&self) -> &ProgramSnapshot {
        &self.state.current
    }

    /// Replaces the current snapshot after an algorithm accepts a returned trial.
    pub fn set_current(&mut self, snapshot: ProgramSnapshot) -> anyhow::Result<()> {
        // Algorithms receive accepted snapshots from `try_group` or
        // `try_candidates`. This method is the only place where they should make
        // that snapshot current, because it also mirrors the source into the
        // workspace's `accepted/` file for crash inspection and future diffs.
        self.state.current = snapshot;
        self.workspace
            .write_accepted(&self.state.current.file_name, &self.state.current.source)?;
        Ok(())
    }

    /// Returns true when no further oracle trials should be attempted.
    pub fn trial_limit_reached(&mut self) -> bool {
        // The engine stores raw counters and configured targets; the context
        // translates them into an algorithm-facing stop signal and records which
        // configured limit was responsible for termination.
        if let Some(signal) = self.state.interrupted_by_signal {
            self.record_signal_interrupt(signal);
            return true;
        }

        if let Some(signal) = received_signal() {
            self.record_signal_interrupt(signal);
            return true;
        }

        if self.engine.trial_limit_reached(self.state.trial_id) {
            self.state.stopped_by_trial_limit = true;
            return true;
        }

        if self
            .engine
            .size_limit_reached(self.state.current.source.len(), self.report.original_size)
        {
            if !self.state.stopped_by_size_limit {
                log_size_limit_reached(
                    self.state.current.source.len(),
                    self.report.original_size,
                    self.engine.config.limits.stop_size.bytes,
                    self.engine.config.limits.stop_size.percent,
                );
            }
            self.state.stopped_by_size_limit = true;
            return true;
        }

        false
    }

    /// Returns whether any configured reducer bound stopped the algorithm.
    pub fn stopped_by_limit(&self) -> bool {
        self.state.stopped_by_trial_limit
            || self.state.stopped_by_size_limit
            || self.state.interrupted_by_signal.is_some()
    }

    /// Records a shutdown signal exactly once in state and the in-progress report.
    fn record_signal_interrupt(&mut self, signal: i32) {
        if self.state.interrupted_by_signal.is_none() {
            crate::logging::info(format_args!(
                "shutdown signal {signal} observed; stopping reduction and writing current accepted source"
            ));
        }
        self.state.interrupted_by_signal = Some(signal);
        self.report.interrupted_by_signal = Some(signal);
    }

    /// Adds one stage report to the session report.
    pub fn push_stage_report(&mut self, report: StageReport) {
        // Algorithms decide when a logical stage/pass is complete. The engine
        // later serializes this accumulated vector into the JSON report.
        self.report.stages.push(report);
    }

    /// Records one rejected or skipped attempt in the session totals.
    pub fn record_rejected_attempt(&mut self) {
        // Direct users of `try_candidates` must call this for rejected outcomes
        // because only `try_group` can update totals automatically. Keeping this
        // explicit prevents hidden counting from double-counting structured
        // group attempts.
        self.state.rejected_total += 1;
    }

    /// Creates a stage report initialized from the current snapshot.
    pub fn stage_report(&self, round: usize, stage: StageKind) -> StageReport {
        // Reports capture before/after size and static runtime cost. The caller
        // fills in generated/trial/accepted/rejected counts and final after
        // values when the pass ends.
        StageReport::new(
            round,
            stage,
            self.state.current.source.len(),
            self.state.current.score.runtime_cost_total,
        )
    }

    /// Ensures the current accepted source is still parseable.
    pub fn reject_invalid_current(&self, context: &str) -> anyhow::Result<()> {
        // Accepted snapshots should always be parseable. Checking this at pass
        // boundaries catches bugs in normalization or algorithm state handling
        // before more stale candidates are generated.
        reject_invalid_snapshot(&self.state.current, context)
    }

    /// Generates normalized candidate groups for the active snapshot.
    pub fn generate_groups(
        &self,
        stage: StageKind,
        singles_only: bool,
    ) -> anyhow::Result<Vec<CandidateGroup>> {
        // This is the only candidate-generation entry point exposed to
        // algorithms. It merges adapter groups with language-neutral groups,
        // retargets candidates to the current snapshot, optionally rewrites
        // schedules for single-candidate sweeps, and normalizes ordering.
        self.engine
            .generate_groups_for_current(stage, self.state, singles_only)
    }

    /// Tries one candidate group using the shared deterministic chunk schedule.
    pub fn try_group(
        &mut self,
        group: &CandidateGroup,
        report: &mut StageReport,
        singles_only: bool,
    ) -> anyhow::Result<Option<ProgramSnapshot>> {
        // Structured algorithms can use this higher-level helper when they want
        // the group's chunk schedule, failure budget, and rejection accounting
        // to be applied exactly as defined by the shared reducer policy.
        self.engine.try_group(
            group,
            self.workspace,
            self.oracle,
            self.state,
            report,
            singles_only,
        )
    }

    /// Tries one explicit candidate set and returns the detailed attempt outcome.
    pub fn try_candidates(
        &mut self,
        candidates: &[Candidate],
        group: &CandidateGroup,
        report: &mut StageReport,
        attempt_description: &str,
        objective: SimplificationObjective,
    ) -> anyhow::Result<TrialAttempt> {
        // Adaptive algorithms use this lower-level helper when they have already
        // selected an exact candidate set. It still applies edits, reparses,
        // checks the simplification objective, consults the cache, and runs the
        // oracle. The caller is responsible for updating pass-local rejection
        // totals via `record_rejected_attempt` when the result is not accepted.
        self.engine.try_candidates(
            candidates,
            group,
            self.workspace,
            self.oracle,
            self.state,
            report,
            attempt_description,
            objective,
        )
    }

    /// Builds a fresh snapshot through the selected language adapter.
    pub fn build_snapshot(
        &self,
        version: SnapshotId,
        source: String,
        file_name: &str,
    ) -> anyhow::Result<ProgramSnapshot> {
        // Most algorithms should not need to build snapshots directly. This is
        // available for future algorithms that synthesize source outside normal
        // candidates while still using the configured language adapter.
        self.engine.build_snapshot(version, source, file_name)
    }
}

/// Logs size-limit termination without exposing engine internals to algorithms.
fn log_size_limit_reached(
    current_size: usize,
    original_size: usize,
    target_bytes: Option<usize>,
    target_percent: Option<u32>,
) {
    let byte_part = target_bytes
        .map(|bytes| format!("{bytes} bytes"))
        .unwrap_or_else(|| "disabled".to_owned());
    let percent_part = target_percent
        .map(|percent| format!("{percent}%"))
        .unwrap_or_else(|| "disabled".to_owned());
    crate::logging::info(format_args!(
        "size stop target reached: current={} bytes, original={} bytes, target_size={}, target_percent={}",
        current_size, original_size, byte_part, percent_part
    ));
}
