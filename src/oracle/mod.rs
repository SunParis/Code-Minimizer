//! Interestingness oracle for A/B command comparisons.
//!
//! The oracle is the only component that decides whether a candidate source is
//! still worth keeping. It validates builds, runs A and B in isolated
//! directories, compares stdout/stderr according to configuration, and applies
//! the selected exit-status preservation policy.

mod checks;
mod execute;
mod normalize;
#[cfg(test)]
mod tests;
mod types;

use std::path::Path;

use crate::{
    config::{DiffMode, ReduceConfig, TrialSide},
    error::CodeMinimizerError,
    output_diff::OutputDiff,
    runner::{CommandRunner, InvocationOutcome},
    workspace::{SessionWorkspace, TrialLayout},
};

use checks::{
    diff_lost_reason, exit_status_change_reason, exit_status_matches, validate_no_truncation,
    validate_run_completed,
};
use execute::empty_failed_run;
use normalize::output_normalizer_for_layout;
use types::with_confirmation_reason;
pub use types::{Baseline, OracleDecision};

/// Oracle runner bound to immutable session configuration.
#[derive(Clone, Debug)]
pub struct Oracle {
    pub(super) config: ReduceConfig,
    pub(super) runner: CommandRunner,
    pub(super) file_name: String,
    pub(super) stem: String,
}

impl Oracle {
    /// Creates an oracle for one reducer session.
    pub fn new(config: ReduceConfig) -> anyhow::Result<Self> {
        let file_name = config.input_file_name()?;
        let stem = config.input_stem()?;
        let runner = CommandRunner::new(config.timeout, config.max_output_bytes);

        Ok(Self {
            config,
            runner,
            file_name,
            stem,
        })
    }

    /// Executes and validates the baseline source before reduction begins.
    pub fn establish_baseline(
        &self,
        workspace: &SessionWorkspace,
        source: &str,
    ) -> anyhow::Result<Baseline> {
        let mut accepted = None;
        for confirmation in 1..=self.config.confirm_runs {
            let layout = workspace.prepare_trial(
                &format!("baseline-confirmation-{confirmation}"),
                &self.file_name,
                source,
            )?;
            let a = self.execute_side(&layout, TrialSide::A).map_err(|error| {
                CodeMinimizerError::BaselineNotInteresting(format!(
                    "Side A failed before comparison on confirmation {confirmation}: {error}"
                ))
            })?;
            let b = self.execute_side(&layout, TrialSide::B).map_err(|error| {
                CodeMinimizerError::BaselineNotInteresting(format!(
                    "Side B failed before comparison on confirmation {confirmation}: {error}"
                ))
            })?;

            validate_no_truncation(&a).map_err(|reason| {
                CodeMinimizerError::BaselineNotInteresting(format!(
                    "Side A {reason} on confirmation {confirmation}"
                ))
            })?;
            validate_no_truncation(&b).map_err(|reason| {
                CodeMinimizerError::BaselineNotInteresting(format!(
                    "Side B {reason} on confirmation {confirmation}"
                ))
            })?;
            validate_run_completed(&a).map_err(|reason| {
                CodeMinimizerError::BaselineNotInteresting(format!(
                    "Side A {reason} on confirmation {confirmation}"
                ))
            })?;
            validate_run_completed(&b).map_err(|reason| {
                CodeMinimizerError::BaselineNotInteresting(format!(
                    "Side B {reason} on confirmation {confirmation}"
                ))
            })?;

            let normalizer = output_normalizer_for_layout(&layout)?;
            let diff = OutputDiff::compare_normalized(&a, &b, &normalizer);
            if !diff.satisfies(self.config.diff_mode) {
                return Err(CodeMinimizerError::BaselineNotInteresting(format!(
                    "A/B output does not differ according to diff mode {:?} on confirmation {confirmation}",
                    self.config.diff_mode
                ))
                .into());
            }

            accepted = Some(Baseline { a, b, diff });
        }

        accepted.ok_or_else(|| {
            CodeMinimizerError::BaselineNotInteresting(
                "No baseline confirmation was executed".to_owned(),
            )
            .into()
        })
    }

    /// Evaluates a candidate source in a fresh trial directory.
    pub fn evaluate_candidate(
        &self,
        workspace: &SessionWorkspace,
        source: &str,
        trial_id: usize,
        baseline: &Baseline,
    ) -> anyhow::Result<OracleDecision> {
        let mut accepted = None;
        for confirmation in 1..=self.config.confirm_runs {
            let trial_name = if self.config.confirm_runs == 1 {
                format!("trial-{trial_id}")
            } else {
                format!("trial-{trial_id}-confirmation-{confirmation}")
            };
            let layout = workspace.prepare_trial(&trial_name, &self.file_name, source)?;
            let decision = self.evaluate_once(&layout, baseline)?;
            workspace.cleanup_trial_outputs(&layout)?;
            if !decision.accepted {
                return Ok(with_confirmation_reason(decision, confirmation));
            }
            accepted = Some(decision);
        }

        accepted.ok_or_else(|| anyhow::anyhow!("No candidate confirmation was executed"))
    }

    /// Evaluates one candidate confirmation in an already prepared trial layout.
    fn evaluate_once(
        &self,
        layout: &TrialLayout,
        baseline: &Baseline,
    ) -> anyhow::Result<OracleDecision> {
        let a = match self.execute_side(layout, TrialSide::A) {
            Ok(outcome) => outcome,
            Err(error) => {
                return Ok(OracleDecision::rejected(format!(
                    "Side A command failed before comparison: {error}"
                )));
            }
        };

        if let Err(reason) = validate_no_truncation(&a) {
            return Ok(OracleDecision::rejected(format!("Side A {reason}")));
        }
        if let Err(reason) = validate_run_completed(&a) {
            return Ok(OracleDecision::rejected(format!("Side A {reason}")));
        }
        if !exit_status_matches(
            self.config.preserve_exit,
            &baseline.a.run.status,
            &a.run.status,
        ) {
            return Ok(OracleDecision::rejected(exit_status_change_reason(
                TrialSide::A,
                self.config.preserve_exit,
                &baseline.a.run.status,
                &a.run.status,
            )));
        }

        let b = match self.execute_side(layout, TrialSide::B) {
            Ok(outcome) => outcome,
            Err(error) => {
                return Ok(OracleDecision::rejected_with_outcomes(
                    format!("Side B command failed before comparison: {error}"),
                    a,
                    InvocationOutcome {
                        build: None,
                        run: empty_failed_run(),
                    },
                    OutputDiff {
                        stdout_differs: false,
                        stderr_differs: false,
                        exit_differs: false,
                    },
                ));
            }
        };

        if let Err(reason) = validate_no_truncation(&b) {
            return Ok(OracleDecision::rejected_with_outcomes(
                format!("Side B {reason}"),
                a,
                b,
                OutputDiff {
                    stdout_differs: false,
                    stderr_differs: false,
                    exit_differs: false,
                },
            ));
        }
        if let Err(reason) = validate_run_completed(&b) {
            return Ok(OracleDecision::rejected_with_outcomes(
                format!("Side B {reason}"),
                a,
                b,
                OutputDiff {
                    stdout_differs: false,
                    stderr_differs: false,
                    exit_differs: false,
                },
            ));
        }

        let normalizer = output_normalizer_for_layout(layout)?;
        let diff = OutputDiff::compare_normalized(&a, &b, &normalizer);
        if !diff.satisfies(self.config.diff_mode) {
            return Ok(OracleDecision::rejected_with_outcomes(
                diff_lost_reason(self.config.diff_mode, &diff),
                a,
                b,
                diff,
            ));
        }

        if !exit_status_matches(
            self.config.preserve_exit,
            &baseline.b.run.status,
            &b.run.status,
        ) {
            return Ok(OracleDecision::rejected_with_outcomes(
                exit_status_change_reason(
                    TrialSide::B,
                    self.config.preserve_exit,
                    &baseline.b.run.status,
                    &b.run.status,
                ),
                a,
                b,
                diff,
            ));
        }

        Ok(OracleDecision::accepted(a, b, diff))
    }

    /// Returns the configured diff mode.
    pub fn diff_mode(&self) -> DiffMode {
        self.config.diff_mode
    }

    /// Returns the input file name used inside trials.
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Returns the original input stem used for command templates.
    pub fn stem(&self) -> &str {
        &self.stem
    }
}

/// Returns true when a path exists and points to a regular file.
pub fn is_regular_file(path: &Path) -> bool {
    path.is_file()
}
