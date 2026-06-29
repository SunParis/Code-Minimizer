//! JSON-serializable reduction reports.
//!
//! Reports intentionally use stable English field names and reason strings so
//! they can be consumed by scripts, CI jobs, and bug-report tooling.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{output_diff::OutputDiff, reducer::candidate::StageKind};

/// Final high-level summary returned to the CLI.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReductionSummary {
    /// Final minimized source path.
    pub output_path: PathBuf,
    /// Original source size in bytes.
    pub original_size: usize,
    /// Final source size in bytes.
    pub final_size: usize,
    /// Total oracle trials attempted after baseline validation.
    pub total_trials: usize,
    /// Number of accepted candidate trials.
    pub accepted_trials: usize,
    /// Number of rejected candidate trials.
    pub rejected_trials: usize,
    /// Number of trial-cache hits.
    pub cache_hits: usize,
    /// Kept workspace directory when `--keep-temp` was used.
    pub kept_temp_dir: Option<PathBuf>,
}

/// Full JSON report for one reduction session.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReductionReport {
    /// Original source path.
    pub input_path: PathBuf,
    /// Final minimized source path.
    pub output_path: PathBuf,
    /// Source language identifier.
    pub language: String,
    /// Reducer algorithm used for this session.
    ///
    /// This is serialized as the CLI spelling such as `structured` or
    /// `weighted-random` so report consumers can compare runs that use different
    /// candidate schedulers.
    pub algorithm: String,
    /// Original source size in bytes.
    pub original_size: usize,
    /// Final source size in bytes.
    pub final_size: usize,
    /// Total oracle trials attempted after baseline validation.
    pub total_trials: usize,
    /// Number of accepted trials.
    pub accepted_trials: usize,
    /// Number of rejected trials.
    pub rejected_trials: usize,
    /// Number of cache hits.
    pub cache_hits: usize,
    /// Configured maximum fixed-point rounds.
    pub max_rounds: usize,
    /// Configured maximum oracle trials after baseline validation.
    pub max_trials: usize,
    /// Number of repeated oracle confirmations required for accepted results.
    pub confirm_runs: usize,
    /// Per-command timeout in milliseconds.
    pub timeout_ms: u128,
    /// Maximum captured bytes per output stream.
    pub max_output_bytes: usize,
    /// Baseline output diff state.
    pub baseline_diff: OutputDiff,
    /// Final output diff state when known.
    pub final_diff: OutputDiff,
    /// Per-stage statistics.
    pub stages: Vec<StageReport>,
    /// Whether the reducer stopped because `--max-trials` was reached.
    pub trial_limit_reached: bool,
    /// Workspace directory retained by `--keep-temp`.
    pub kept_temp_dir: Option<PathBuf>,
}

impl ReductionReport {
    /// Creates a report initialized with baseline metadata.
    pub fn new(
        input_path: PathBuf,
        output_path: PathBuf,
        language: String,
        algorithm: String,
        original_size: usize,
        baseline_diff: OutputDiff,
        max_rounds: usize,
        max_trials: usize,
        confirm_runs: usize,
        timeout_ms: u128,
        max_output_bytes: usize,
    ) -> Self {
        // The report starts with baseline metadata and zero counters. The engine
        // fills in final size, trial totals, cache hits, final diff, and kept
        // workspace path after the algorithm and final confirmation complete.
        Self {
            input_path,
            output_path,
            language,
            algorithm,
            original_size,
            final_size: original_size,
            total_trials: 0,
            accepted_trials: 0,
            rejected_trials: 0,
            cache_hits: 0,
            max_rounds,
            max_trials,
            confirm_runs,
            timeout_ms,
            max_output_bytes,
            baseline_diff: baseline_diff.clone(),
            final_diff: baseline_diff,
            stages: Vec::new(),
            trial_limit_reached: false,
            kept_temp_dir: None,
        }
    }

    /// Converts the report into the compact CLI summary.
    pub fn summary(&self) -> ReductionSummary {
        ReductionSummary {
            output_path: self.output_path.clone(),
            original_size: self.original_size,
            final_size: self.final_size,
            total_trials: self.total_trials,
            accepted_trials: self.accepted_trials,
            rejected_trials: self.rejected_trials,
            cache_hits: self.cache_hits,
            kept_temp_dir: self.kept_temp_dir.clone(),
        }
    }
}

/// Statistics for one reducer stage in one fixed-point round.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StageReport {
    /// Fixed-point round index starting at one.
    pub round: usize,
    /// Reducer stage kind.
    pub stage: StageKind,
    /// Number of candidates generated for this stage.
    pub generated_candidates: usize,
    /// Number of oracle trials attempted for this stage.
    pub trials: usize,
    /// Number of accepted trials for this stage.
    pub accepted: usize,
    /// Number of rejected trials for this stage.
    pub rejected: usize,
    /// Source size before the stage.
    pub size_before: usize,
    /// Source size after the stage.
    pub size_after: usize,
    /// Static runtime-cost total before the stage.
    pub runtime_cost_before: u64,
    /// Static runtime-cost total after the stage.
    pub runtime_cost_after: u64,
}

impl StageReport {
    /// Creates an empty stage report.
    pub fn new(
        round: usize,
        stage: StageKind,
        size_before: usize,
        runtime_cost_before: u64,
    ) -> Self {
        Self {
            round,
            stage,
            generated_candidates: 0,
            trials: 0,
            accepted: 0,
            rejected: 0,
            size_before,
            size_after: size_before,
            runtime_cost_before,
            runtime_cost_after: runtime_cost_before,
        }
    }
}
