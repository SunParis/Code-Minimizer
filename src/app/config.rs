//! Configuration values shared by the CLI, oracle, runner, and reducer.
//!
//! Configuration is deliberately immutable once reduction begins. This keeps
//! cache keys and report metadata stable across all trials in one session.

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::error::CodeMinimizerError;

/// Controls which stdout/stderr result must continue to differ between command A and B.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DiffMode {
    /// Accept a trial when stdout differs or stderr differs.
    AnyChannel,
    /// Accept a trial only when stdout differs.
    Stdout,
    /// Accept a trial only when stderr differs.
    Stderr,
    /// Accept a trial only when stdout and stderr both differ.
    Both,
}

impl DiffMode {
    /// Parses a stable CLI spelling into a diff mode.
    pub fn parse(value: &str) -> anyhow::Result<Self> {
        match value {
            "any-channel" => Ok(Self::AnyChannel),
            "stdout" => Ok(Self::Stdout),
            "stderr" => Ok(Self::Stderr),
            "both" => Ok(Self::Both),
            _ => Err(anyhow::anyhow!(
                "Invalid diff mode '{value}'. Expected one of: any-channel, stdout, stderr, both"
            )),
        }
    }
}

/// Controls how strictly command exit status classes are preserved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PreserveExit {
    /// Do not compare trial exit statuses to baseline exit statuses.
    None,
    /// Preserve success versus non-zero versus signal-style exit classes.
    SameClass,
    /// Preserve exact exit status summaries.
    Exact,
}

impl PreserveExit {
    /// Parses a stable CLI spelling into an exit preservation policy.
    pub fn parse(value: &str) -> anyhow::Result<Self> {
        match value {
            "none" => Ok(Self::None),
            "same-class" => Ok(Self::SameClass),
            "exact" => Ok(Self::Exact),
            _ => Err(anyhow::anyhow!(
                "Invalid exit preservation policy '{value}'. Expected one of: none, same-class, exact"
            )),
        }
    }
}

/// Selects the candidate scheduling algorithm used by the reducer.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReductionAlgorithm {
    /// Deterministic staged reduction with chunking and a final single-candidate sweep.
    ///
    /// This is the stable default because it follows a predictable phase order,
    /// prefers larger coordinated edits before singles, and ends with a
    /// practical one-minimal sweep.
    #[default]
    Structured,
    /// Weighted random point deletion with local probability updates and cleanup sweeps.
    ///
    /// This experimental scheduler samples deletable statement points by
    /// adaptive weights, then performs deterministic cleanup, conservative
    /// identifier renaming, and the engine's shared final cleanup.
    WeightedRandom,
}

impl ReductionAlgorithm {
    /// Parses a stable CLI spelling into an algorithm selector.
    pub fn parse(value: &str) -> anyhow::Result<Self> {
        // Keep CLI spellings explicit instead of deriving from enum names. That
        // lets logs, JSON reports, and docs stay stable even if Rust variant
        // names change later.
        match value {
            "structured" => Ok(Self::Structured),
            "weighted-random" => Ok(Self::WeightedRandom),
            _ => Err(anyhow::anyhow!(
                "Invalid reduction algorithm '{value}'. Expected one of: structured, weighted-random"
            )),
        }
    }

    /// Returns the stable CLI/report spelling for this algorithm.
    pub fn as_str(self) -> &'static str {
        // `as_str` is used for logs and JSON reports, so it must stay aligned
        // with `parse`.
        match self {
            Self::Structured => "structured",
            Self::WeightedRandom => "weighted-random",
        }
    }
}

/// Build command configuration for A/B executions.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BuildConfig {
    /// No build command is needed before running the source.
    None,
    /// Both A and B use the same build template in their isolated side directory.
    Shared(String),
    /// A and B use separate build templates in their isolated side directories.
    PerSide {
        a: Option<String>,
        b: Option<String>,
    },
}

impl BuildConfig {
    /// Returns the build command template for one side of a trial.
    pub fn command_for_side(&self, side: TrialSide) -> Option<&str> {
        match self {
            Self::None => None,
            Self::Shared(command) => Some(command.as_str()),
            Self::PerSide { a, b } => match side {
                TrialSide::A => a.as_deref(),
                TrialSide::B => b.as_deref(),
            },
        }
    }

    /// Builds a validated configuration from CLI build options.
    pub fn from_cli(
        shared: Option<String>,
        build_a: Option<String>,
        build_b: Option<String>,
    ) -> anyhow::Result<Self> {
        if shared.is_some() && (build_a.is_some() || build_b.is_some()) {
            anyhow::bail!(
                "Use either --build or --build-a/--build-b, but do not combine shared and per-side build commands"
            );
        }

        if let Some(command) = shared {
            Ok(Self::Shared(command))
        } else if build_a.is_some() || build_b.is_some() {
            Ok(Self::PerSide {
                a: build_a,
                b: build_b,
            })
        } else {
            Ok(Self::None)
        }
    }
}

/// Identifies which command side is being executed in a trial.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TrialSide {
    /// The A command path.
    A,
    /// The B command path.
    B,
}

impl TrialSide {
    /// Returns the stable directory suffix used for the side.
    pub fn as_dir_name(self) -> &'static str {
        match self {
            Self::A => "a",
            Self::B => "b",
        }
    }

    /// Returns the stable human-readable side name used in reports.
    pub fn as_label(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
        }
    }
}

/// Limits that bound reduction work for practical fuzzing workflows.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReducerLimits {
    /// Maximum fixed-point rounds over all reducer stages.
    pub max_rounds: usize,
    /// Maximum oracle trials after the baseline has been established.
    pub max_trials: usize,
}

impl Default for ReducerLimits {
    fn default() -> Self {
        Self {
            max_rounds: 8,
            max_trials: 2_000,
        }
    }
}

/// Fully validated configuration for one reduction session.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReduceConfig {
    /// Source language identifier such as `java`, `js`, or `javascript`.
    pub language: String,
    /// Original source file. The reducer never writes to this path.
    pub input_path: PathBuf,
    /// Final minimized source file.
    pub output_path: PathBuf,
    /// Command template for side A.
    pub run_a: String,
    /// Command template for side B.
    pub run_b: String,
    /// Optional build commands used before each run command.
    pub build: BuildConfig,
    /// Timeout applied to each build or run command.
    pub timeout: Duration,
    /// Maximum bytes captured from stdout and stderr for each command.
    pub max_output_bytes: usize,
    /// Number of repeated oracle confirmations required for baseline, accepted trials, and final output.
    pub confirm_runs: usize,
    /// Exit status preservation policy.
    pub preserve_exit: PreserveExit,
    /// Result diff preservation policy.
    pub diff_mode: DiffMode,
    /// Whether temporary trial directories should remain on disk.
    pub keep_temp: bool,
    /// Optional JSON report output path.
    pub json_report_path: Option<PathBuf>,
    /// Current implementation is single-threaded, but the option is preserved for API stability.
    pub jobs: usize,
    /// Practical bounds for fixed-point rounds and oracle trials.
    pub limits: ReducerLimits,
    /// Candidate scheduling algorithm.
    pub algorithm: ReductionAlgorithm,
}

impl ReduceConfig {
    /// Validates and constructs a reducer configuration.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        language: String,
        input_path: PathBuf,
        output_path: Option<PathBuf>,
        run_a: String,
        run_b: String,
        build: BuildConfig,
        timeout: Duration,
        max_output_bytes: usize,
        confirm_runs: usize,
        preserve_exit: PreserveExit,
        diff_mode: DiffMode,
        keep_temp: bool,
        json_report_path: Option<PathBuf>,
        jobs: usize,
        limits: ReducerLimits,
    ) -> anyhow::Result<Self> {
        if !input_path.exists() {
            return Err(CodeMinimizerError::MissingInput(input_path).into());
        }

        if run_a.trim().is_empty() || run_b.trim().is_empty() {
            anyhow::bail!("Both --run-a and --run-b must be non-empty command templates");
        }

        if max_output_bytes == 0 {
            anyhow::bail!("--max-output-bytes must be greater than zero");
        }

        if confirm_runs == 0 {
            anyhow::bail!("--confirm-runs must be greater than zero");
        }

        if jobs == 0 {
            anyhow::bail!("--jobs must be greater than zero");
        }

        let output_path = output_path.unwrap_or_else(|| default_output_path(&input_path));

        Ok(Self {
            language,
            input_path,
            output_path,
            run_a,
            run_b,
            build,
            timeout,
            max_output_bytes,
            confirm_runs,
            preserve_exit,
            diff_mode,
            keep_temp,
            json_report_path,
            jobs,
            limits,
            algorithm: ReductionAlgorithm::default(),
        })
    }

    /// Sets the reducer algorithm after the base configuration has been validated.
    pub fn with_algorithm(mut self, algorithm: ReductionAlgorithm) -> Self {
        // `ReduceConfig::new` keeps its existing signature for API stability and
        // defaults to the structured algorithm. CLI parsing calls this method
        // after validating the user-provided selector.
        self.algorithm = algorithm;
        self
    }

    /// Returns the file name used inside trial directories.
    pub fn input_file_name(&self) -> anyhow::Result<String> {
        let name = self
            .input_path
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| anyhow::anyhow!("Input path must have a valid UTF-8 file name"))?;
        Ok(name.to_owned())
    }

    /// Returns the input stem used for command template expansion.
    pub fn input_stem(&self) -> anyhow::Result<String> {
        let stem = self
            .input_path
            .file_stem()
            .and_then(|value| value.to_str())
            .ok_or_else(|| anyhow::anyhow!("Input path must have a valid UTF-8 file stem"))?;
        Ok(stem.to_owned())
    }

    /// Returns a stable fingerprint of oracle-affecting settings for cache keys.
    pub fn oracle_fingerprint(&self) -> String {
        format!(
            "lang={};run_a={};run_b={};build={:?};timeout_ms={};max_output={};confirm_runs={};preserve={:?};diff={:?}",
            self.language,
            self.run_a,
            self.run_b,
            self.build,
            self.timeout.as_millis(),
            self.max_output_bytes,
            self.confirm_runs,
            self.preserve_exit,
            self.diff_mode
        )
    }
}

/// Computes the default output path by inserting `.min` before the extension.
pub fn default_output_path(input_path: &Path) -> PathBuf {
    let mut output = input_path.to_path_buf();
    let file_name = input_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("output");

    if let Some((stem, ext)) = file_name.rsplit_once('.') {
        output.set_file_name(format!("{stem}.min.{ext}"));
    } else {
        output.set_file_name(format!("{file_name}.min"));
    }

    output
}

/// Parses a human-oriented duration used by the CLI.
pub fn parse_duration(value: &str) -> anyhow::Result<Duration> {
    let value = value.trim();
    if value.is_empty() {
        anyhow::bail!("Duration must not be empty");
    }

    if let Some(ms) = value.strip_suffix("ms") {
        let number: u64 = ms.parse()?;
        return Ok(Duration::from_millis(number));
    }

    if let Some(seconds) = value.strip_suffix('s') {
        let number: u64 = seconds.parse()?;
        return Ok(Duration::from_secs(number));
    }

    let number: u64 = value.parse()?;
    Ok(Duration::from_secs(number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_output_path_inserts_min_before_extension() {
        let path = default_output_path(Path::new("/tmp/Test.java"));
        assert_eq!(path, PathBuf::from("/tmp/Test.min.java"));
    }

    #[test]
    fn parse_duration_accepts_seconds_and_milliseconds() {
        assert_eq!(parse_duration("5s").unwrap(), Duration::from_secs(5));
        assert_eq!(parse_duration("250ms").unwrap(), Duration::from_millis(250));
        assert_eq!(parse_duration("7").unwrap(), Duration::from_secs(7));
    }

    #[test]
    fn reduction_algorithm_parses_cli_spelling() {
        assert_eq!(
            ReductionAlgorithm::parse("structured").unwrap(),
            ReductionAlgorithm::Structured
        );
        assert_eq!(
            ReductionAlgorithm::parse("weighted-random").unwrap(),
            ReductionAlgorithm::WeightedRandom
        );
        assert!(ReductionAlgorithm::parse("random").is_err());
    }

    #[test]
    fn build_config_rejects_mixed_shared_and_per_side_commands() {
        let result = BuildConfig::from_cli(Some("make".into()), Some("make a".into()), None);
        assert!(result.is_err(), "Mixed build modes must be rejected");
    }
}
