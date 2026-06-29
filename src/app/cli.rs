//! CLI argument parsing.
//!
//! This module converts human-facing command-line options into the validated
//! reducer configuration used by the rest of the crate. All help and error text
//! is English because runtime output must be English.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::config::{
    BuildConfig, DiffMode, PreserveExit, ReduceConfig, ReducerLimits, ReductionAlgorithm,
    parse_duration,
};

/// Top-level CLI parser.
#[derive(Debug, Parser)]
#[command(name = "code-minimizer")]
#[command(
    about = "Reduce single-file compiler/runtime fuzzing cases while preserving A/B result differences."
)]
pub struct Cli {
    /// Command to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Supported top-level commands.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Reduce one source file while preserving the A/B result difference.
    Reduce(ReduceArgs),
}

/// Arguments for `code-minimizer reduce`.
#[derive(Debug, Args)]
pub struct ReduceArgs {
    /// Source language: java, js, or javascript.
    #[arg(long)]
    pub lang: String,

    /// Input source file.
    #[arg(long)]
    pub input: PathBuf,

    /// Output source file. Defaults to inserting `.min` before the extension.
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Run command template for side A.
    #[arg(long)]
    pub run_a: String,

    /// Run command template for side B.
    #[arg(long)]
    pub run_b: String,

    /// Shared build command template used for both sides.
    #[arg(long)]
    pub build: Option<String>,

    /// Build command template for side A.
    #[arg(long)]
    pub build_a: Option<String>,

    /// Build command template for side B.
    #[arg(long)]
    pub build_b: Option<String>,

    /// Per-command timeout, for example `5s` or `250ms`.
    #[arg(long, default_value = "5s")]
    pub timeout: String,

    /// Maximum captured bytes per output stream.
    #[arg(long, default_value_t = 1_048_576)]
    pub max_output_bytes: usize,

    /// Re-run baseline, accepted candidates, and final confirmation this many times.
    #[arg(long, default_value_t = 1)]
    pub confirm_runs: usize,

    /// Reserved parallelism setting. The first implementation runs trials sequentially.
    #[arg(long, default_value_t = 1)]
    pub jobs: usize,

    /// Exit status preservation policy: none, same-class, or exact.
    #[arg(long, default_value = "same-class")]
    pub preserve_exit: String,

    /// Diff preservation mode: any-channel, stdout, stderr, or both.
    #[arg(long, default_value = "any-channel")]
    pub diff_mode: String,

    /// Reduction algorithm: structured or weighted-random.
    ///
    /// The default keeps historical behavior. Selecting `weighted-random`
    /// changes only scheduling; parser/oracle/cache/workspace semantics remain
    /// shared with the structured algorithm.
    #[arg(long, default_value = "structured")]
    pub algorithm: String,

    /// Keep temporary session directories on disk.
    #[arg(long, default_value_t = false)]
    pub keep_temp: bool,

    /// Write a JSON reduction report to this path.
    #[arg(long)]
    pub json_report: Option<PathBuf>,

    /// Maximum fixed-point rounds over all reducer stages. Use 0 for no explicit limit.
    #[arg(long, default_value_t = 8)]
    pub max_rounds: usize,

    /// Maximum oracle trials after baseline validation. Use 0 for no explicit limit.
    #[arg(long, default_value_t = 2000)]
    pub max_trials: usize,
}

impl ReduceArgs {
    /// Converts CLI arguments into a validated reducer configuration.
    pub fn into_config(self) -> anyhow::Result<ReduceConfig> {
        let build = BuildConfig::from_cli(self.build, self.build_a, self.build_b)?;
        let timeout = parse_duration(&self.timeout)?;
        let preserve_exit = PreserveExit::parse(&self.preserve_exit)?;
        let diff_mode = DiffMode::parse(&self.diff_mode)?;
        // Parse the algorithm before constructing the config so CLI errors are
        // reported with the rest of the user-facing validation failures.
        let algorithm = ReductionAlgorithm::parse(&self.algorithm)?;
        let limits = ReducerLimits {
            max_rounds: if self.max_rounds == 0 {
                usize::MAX
            } else {
                self.max_rounds
            },
            max_trials: if self.max_trials == 0 {
                usize::MAX
            } else {
                self.max_trials
            },
        };

        ReduceConfig::new(
            self.lang,
            self.input,
            self.output,
            self.run_a,
            self.run_b,
            build,
            timeout,
            self.max_output_bytes,
            self.confirm_runs,
            preserve_exit,
            diff_mode,
            self.keep_temp,
            self.json_report,
            self.jobs,
            limits,
        )
        // `ReduceConfig::new` defaults to the stable structured algorithm for
        // API compatibility. The CLI then applies the user-selected scheduler.
        .map(|config| config.with_algorithm(algorithm))
    }
}
