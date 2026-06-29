//! Snapshot-based reduction engine.
//!
//! The engine is language-independent. It repeatedly asks the adapter for
//! stage-specific candidate groups, applies edit plans against the current
//! snapshot, rejects non-simplifying edits before expensive oracle work, and
//! accepts only candidates that both preserve the configured result difference
//! and reduce structural complexity.

use std::fs;

use anyhow::Context;

use crate::{
    config::{ReduceConfig, TrialSide},
    ir::{ProgramSnapshot, SnapshotId},
    lang::{LanguageAdapter, adapter_for},
    logging,
    oracle::Oracle,
    reducer::{
        algorithms::algorithm_for,
        candidate::StageKind,
        group::{CandidateGroup, ChunkAttempt},
        planning::{
            grouping::{group_failure_budget, regroup_candidates_by_region, target_for_candidate},
            ordering::normalize_groups,
            phase::generate_phase_groups,
        },
        shared_stages::blank_lines::run_final_blank_line_cleanup,
    },
    report::{ReductionReport, ReductionSummary, StageReport},
    runner::received_signal,
    workspace::SessionWorkspace,
};

pub use super::{
    attempt::TrialAttempt,
    context::ReductionContext,
    objective::{SimplificationObjective, candidate_satisfies_stage_objective},
};
use super::{
    reporting::write_json_report, state::EngineState, validation::reject_invalid_snapshot,
};

/// Coordinates parsing, grouping, oracle checks, acceptance, rollback, and reports.
pub struct ReducerEngine {
    pub(super) config: ReduceConfig,
    pub(super) adapter: Box<dyn LanguageAdapter>,
}

impl ReducerEngine {
    /// Creates a reducer engine for the provided configuration.
    pub fn new(config: ReduceConfig) -> anyhow::Result<Self> {
        let adapter = adapter_for(&config.language)?;
        Ok(Self { config, adapter })
    }

    /// Creates an engine with a custom adapter for reducer unit tests.
    #[cfg(test)]
    pub(crate) fn new_with_adapter_for_tests(
        config: ReduceConfig,
        adapter: Box<dyn LanguageAdapter>,
    ) -> Self {
        Self { config, adapter }
    }

    /// Runs the complete reduction workflow and writes the minimized output.
    pub fn reduce(&mut self) -> anyhow::Result<ReductionSummary> {
        let original_source = fs::read_to_string(&self.config.input_path).with_context(|| {
            format!(
                "Failed to read input source '{}'",
                self.config.input_path.display()
            )
        })?;
        let original_size = original_source.len();
        let file_name = self.config.input_file_name()?;

        let workspace =
            SessionWorkspace::for_output(&self.config.output_path, self.config.keep_temp)?;
        logging::info(format_args!(
            "current trial directory: {}",
            workspace.current_trial_dir().display()
        ));
        logging::info(format_args!(
            "current side A directory: {}",
            workspace.current_side_dir(TrialSide::A).display()
        ));
        logging::info(format_args!(
            "current side B directory: {}",
            workspace.current_side_dir(TrialSide::B).display()
        ));
        workspace.write_accepted(&file_name, &original_source)?;

        let oracle = Oracle::new(self.config.clone())?;
        let initial_snapshot =
            self.build_snapshot(SnapshotId::ROOT, original_source, &file_name)?;
        reject_invalid_snapshot(&initial_snapshot, "Initial parse failed")?;

        let baseline = match oracle.establish_baseline(&workspace, &initial_snapshot.source) {
            Ok(baseline) => baseline,
            Err(error) => {
                if let Some(signal) = received_signal() {
                    let report = ReductionReport::interrupted_without_baseline(
                        self.config.input_path.clone(),
                        self.config.output_path.clone(),
                        self.adapter.language_id().to_owned(),
                        self.config.algorithm.as_str().to_owned(),
                        original_size,
                        self.config.limits.max_rounds,
                        self.config.limits.max_trials,
                        self.config.limits.stop_size.bytes,
                        self.config.limits.stop_size.percent,
                        self.config.confirm_runs,
                        self.config.timeout.as_millis(),
                        self.config.max_output_bytes,
                        signal,
                    );
                    return self.finish_interrupted_without_state(
                        workspace,
                        &file_name,
                        &initial_snapshot.source,
                        report,
                    );
                }
                return Err(error);
            }
        };
        if let Some(signal) = received_signal() {
            let mut report = ReductionReport::new(
                self.config.input_path.clone(),
                self.config.output_path.clone(),
                self.adapter.language_id().to_owned(),
                self.config.algorithm.as_str().to_owned(),
                original_size,
                baseline.diff.clone(),
                self.config.limits.max_rounds,
                self.config.limits.max_trials,
                self.config.limits.stop_size.bytes,
                self.config.limits.stop_size.percent,
                self.config.confirm_runs,
                self.config.timeout.as_millis(),
                self.config.max_output_bytes,
            );
            report.interrupted_by_signal = Some(signal);
            return self.finish_interrupted_without_state(
                workspace,
                &file_name,
                &initial_snapshot.source,
                report,
            );
        }
        logging::info(format_args!(
            "baseline: stdout differs={} stderr differs={} exit differs={}",
            baseline.diff.stdout_differs, baseline.diff.stderr_differs, baseline.diff.exit_differs
        ));

        let mut report = ReductionReport::new(
            self.config.input_path.clone(),
            self.config.output_path.clone(),
            self.adapter.language_id().to_owned(),
            self.config.algorithm.as_str().to_owned(),
            original_size,
            baseline.diff.clone(),
            self.config.limits.max_rounds,
            self.config.limits.max_trials,
            self.config.limits.stop_size.bytes,
            self.config.limits.stop_size.percent,
            self.config.confirm_runs,
            self.config.timeout.as_millis(),
            self.config.max_output_bytes,
        );

        let mut state = EngineState::new(
            initial_snapshot,
            baseline,
            report.final_diff.clone(),
            &self.config.oracle_fingerprint(),
        );

        logging::info(format_args!(
            "reduction algorithm: {}",
            self.config.algorithm.as_str()
        ));
        let mut context = ReductionContext::new(self, &workspace, &oracle, &mut state, &mut report);
        algorithm_for(self.config.algorithm).run(&mut context)?;
        if !context.trial_limit_reached() {
            run_final_blank_line_cleanup(&mut context, 1)?;
        }

        fs::write(&self.config.output_path, &state.current.source).with_context(|| {
            format!(
                "Failed to write output source '{}'",
                self.config.output_path.display()
            )
        })?;
        workspace.write_accepted(&file_name, &state.current.source)?;

        if let Some(signal) = state.interrupted_by_signal.or_else(received_signal) {
            state.interrupted_by_signal = Some(signal);
            report.interrupted_by_signal = Some(signal);
            logging::info(format_args!(
                "skipping final confirmation after shutdown signal {signal}; writing last accepted source"
            ));
            return self.finish_report(workspace, state, report);
        }

        let final_decision = match oracle.evaluate_candidate(
            &workspace,
            &state.current.source,
            state.trial_id.saturating_add(1),
            &state.baseline,
        ) {
            Ok(decision) => decision,
            Err(error) => {
                if let Some(signal) = received_signal() {
                    state.interrupted_by_signal = Some(signal);
                    report.interrupted_by_signal = Some(signal);
                    logging::info(format_args!(
                        "shutdown signal {signal} observed during final confirmation; writing last accepted source"
                    ));
                    return self.finish_report(workspace, state, report);
                }
                return Err(error);
            }
        };
        if let Some(signal) = received_signal() {
            state.interrupted_by_signal = Some(signal);
            report.interrupted_by_signal = Some(signal);
            logging::info(format_args!(
                "shutdown signal {signal} observed during final confirmation; writing last accepted source"
            ));
            return self.finish_report(workspace, state, report);
        }
        if !final_decision.accepted {
            anyhow::bail!(
                "Final confirmation failed: {}",
                final_decision
                    .reason
                    .unwrap_or_else(|| "final source was not interesting".to_owned())
            );
        }
        if let Some(diff) = final_decision.diff {
            state.final_diff = diff;
        }

        self.finish_report(workspace, state, report)
    }

    /// Finalizes output metadata and writes the optional JSON report.
    fn finish_report(
        &self,
        workspace: SessionWorkspace,
        state: EngineState,
        mut report: ReductionReport,
    ) -> anyhow::Result<ReductionSummary> {
        let kept_temp_dir = workspace.finish()?;

        report.final_size = state.current.source.len();
        report.total_trials = state.trial_id;
        report.accepted_trials = state.accepted_total;
        report.rejected_trials = state.rejected_total;
        report.cache_hits = state.cache.hits();
        report.final_diff = state.final_diff;
        report.trial_limit_reached = state.stopped_by_trial_limit;
        report.size_limit_reached = state.stopped_by_size_limit;
        report.interrupted_by_signal = state.interrupted_by_signal;
        report.kept_temp_dir = kept_temp_dir.clone();

        if let Some(path) = &self.config.json_report_path {
            write_json_report(path.clone(), &report)?;
        }

        if let Some(path) = kept_temp_dir {
            logging::info(format_args!("workspace kept: {}", path.display()));
        }

        Ok(report.summary())
    }

    /// Writes the original source and an interrupted report when baseline stopped early.
    fn finish_interrupted_without_state(
        &self,
        workspace: SessionWorkspace,
        file_name: &str,
        source: &str,
        mut report: ReductionReport,
    ) -> anyhow::Result<ReductionSummary> {
        fs::write(&self.config.output_path, source).with_context(|| {
            format!(
                "Failed to write output source '{}'",
                self.config.output_path.display()
            )
        })?;
        workspace.write_accepted(file_name, source)?;

        let kept_temp_dir = workspace.finish()?;
        report.final_size = source.len();
        report.kept_temp_dir = kept_temp_dir.clone();

        if let Some(path) = &self.config.json_report_path {
            write_json_report(path.clone(), &report)?;
        }

        if let Some(path) = kept_temp_dir {
            logging::info(format_args!("workspace kept: {}", path.display()));
        }

        Ok(report.summary())
    }

    /// Generates, retargets, and orders groups for the current snapshot.
    pub(super) fn generate_groups_for_current(
        &self,
        stage: StageKind,
        state: &EngineState,
        singles_only: bool,
    ) -> anyhow::Result<Vec<CandidateGroup>> {
        let mut groups = self.adapter.generate_groups(
            stage,
            &state.current.parsed,
            &state.current.index,
            &state.current.score,
        )?;
        groups.extend(generate_phase_groups(stage, &state.current));
        let effective_stage = if singles_only {
            StageKind::FinalOneMinimalSweep
        } else {
            stage
        };

        for group in &mut groups {
            group.snapshot = state.current.version;
            group.stage = effective_stage;
            for candidate in &mut group.candidates {
                candidate.snapshot = state.current.version;
                candidate.stage = effective_stage;
                candidate.plan.snapshot = state.current.version;
                candidate.target = target_for_candidate(candidate, &state.current.index);
                candidate.plan.primary_target = candidate.target;
            }
        }

        let mut groups = regroup_candidates_by_region(stage, groups, &state.current.index);

        if singles_only {
            for group in &mut groups {
                group.chunk_plan.schedule = group
                    .candidates
                    .iter()
                    .map(|candidate| ChunkAttempt {
                        ids: vec![candidate.id.clone()],
                        description: "Try one final-sweep candidate".to_owned(),
                    })
                    .collect();
            }
        }

        Ok(normalize_groups(groups))
    }

    /// Tries one candidate group according to its chunk plan.
    pub(super) fn try_group(
        &self,
        group: &CandidateGroup,
        workspace: &SessionWorkspace,
        oracle: &Oracle,
        state: &mut EngineState,
        report: &mut StageReport,
        singles_only: bool,
    ) -> anyhow::Result<Option<ProgramSnapshot>> {
        let mut consecutive_failures = 0_usize;
        let failure_budget = if singles_only {
            usize::MAX
        } else {
            group_failure_budget(group)
        };

        for attempt in &group.chunk_plan.schedule {
            if self.trial_limit_reached(state.trial_id) {
                break;
            }
            if consecutive_failures >= failure_budget {
                logging::info(format_args!(
                    "phase {} ({}): skipping remaining attempts in group {} after {} consecutive failures; group={}, purpose={}",
                    group.stage.display_name(),
                    group.stage.as_str(),
                    group.id.0,
                    consecutive_failures,
                    group.description,
                    group.stage.purpose()
                ));
                break;
            }
            if singles_only && attempt.ids.len() != 1 {
                continue;
            }

            let candidates = attempt
                .ids
                .iter()
                .filter_map(|id| {
                    group
                        .candidates
                        .iter()
                        .find(|candidate| candidate.id == *id)
                        .cloned()
                })
                .collect::<Vec<_>>();
            if candidates.is_empty() {
                continue;
            }

            match self.try_candidates(
                &candidates,
                group,
                workspace,
                oracle,
                state,
                report,
                &attempt.description,
                SimplificationObjective::Stage(group.stage),
            )? {
                TrialAttempt::Accepted(snapshot) | TrialAttempt::CachedAccepted(snapshot) => {
                    report.accepted += 1;
                    return Ok(Some(snapshot));
                }
                TrialAttempt::Rejected(_)
                | TrialAttempt::CachedRejected(_)
                | TrialAttempt::InvalidEdit => {
                    report.rejected += 1;
                    state.rejected_total += 1;
                    consecutive_failures += 1;
                }
                TrialAttempt::Interrupted => {
                    return Ok(None);
                }
                TrialAttempt::NotSimpler => {
                    report.rejected += 1;
                    state.rejected_total += 1;
                    consecutive_failures += 1;
                }
            }
        }

        Ok(None)
    }

    /// Builds a snapshot through the selected language adapter.
    pub(super) fn build_snapshot(
        &self,
        version: SnapshotId,
        source: String,
        file_name: &str,
    ) -> anyhow::Result<ProgramSnapshot> {
        ProgramSnapshot::build(version, source, file_name, self.adapter.as_ref())
    }

    /// Returns true when the configured trial limit has been reached.
    pub(super) fn trial_limit_reached(&self, trial_id: usize) -> bool {
        trial_id >= self.config.limits.max_trials
    }

    /// Returns true when the accepted source has reached a configured size target.
    pub(super) fn size_limit_reached(&self, current_size: usize, original_size: usize) -> bool {
        self.config
            .limits
            .stop_size
            .reached(current_size, original_size)
    }
}
