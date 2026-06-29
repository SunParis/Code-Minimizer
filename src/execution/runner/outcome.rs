//! Serializable command outcomes used by the oracle and reports.
//!
//! These types deliberately hide platform-specific `std::process::ExitStatus`
//! values behind stable enums, so JSON reports and cache comparisons do not
//! depend on OS formatting details.

use std::{process::ExitStatus, time::Duration};

use serde::{Deserialize, Serialize};

/// Stable process status used in reports and oracle comparisons.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExitStatusSummary {
    /// The process exited with a normal integer exit code.
    Code(i32),
    /// The process was terminated by a Unix signal.
    Signal(i32),
    /// The process exceeded the configured timeout.
    TimedOut,
    /// The platform did not expose a normal status code or signal.
    Unknown,
}

impl ExitStatusSummary {
    /// Converts an OS exit status into the stable summary.
    pub fn from_exit_status(status: ExitStatus) -> Self {
        if let Some(code) = status.code() {
            return Self::Code(code);
        }

        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            if let Some(signal) = status.signal() {
                return Self::Signal(signal);
            }
        }

        Self::Unknown
    }

    /// Returns true when the command exited successfully.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Code(0))
    }

    /// Returns the broad class used by `--preserve-exit same-class`.
    pub fn class(&self) -> ExitStatusClass {
        match self {
            Self::Code(0) => ExitStatusClass::Success,
            Self::Code(_) => ExitStatusClass::NonZero,
            Self::Signal(_) => ExitStatusClass::Signal,
            Self::TimedOut => ExitStatusClass::Timeout,
            Self::Unknown => ExitStatusClass::Unknown,
        }
    }
}

/// Coarse exit-status categories for robust preservation checks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExitStatusClass {
    /// Normal successful exit.
    Success,
    /// Normal non-zero exit.
    NonZero,
    /// Signal termination.
    Signal,
    /// Timeout termination.
    Timeout,
    /// Platform-specific unknown status.
    Unknown,
}

/// Captured outcome of one shell command.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommandOutcome {
    /// Stable exit status summary.
    pub status: ExitStatusSummary,
    /// Captured stdout bytes, truncated to the configured cap.
    pub stdout: Vec<u8>,
    /// Captured stderr bytes, truncated to the configured cap.
    pub stderr: Vec<u8>,
    /// Whether the process exceeded its timeout and was killed.
    pub timed_out: bool,
    /// Wall-clock duration spent waiting for the process.
    pub duration: Duration,
    /// Whether stdout exceeded the configured capture cap.
    pub stdout_truncated: bool,
    /// Whether stderr exceeded the configured capture cap.
    pub stderr_truncated: bool,
}

impl CommandOutcome {
    /// Returns true when either output stream exceeded the hard cap.
    pub fn output_truncated(&self) -> bool {
        self.stdout_truncated || self.stderr_truncated
    }
}

/// Full build/run outcome for one side of a trial.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InvocationOutcome {
    /// Optional build command outcome.
    pub build: Option<CommandOutcome>,
    /// Run command outcome.
    pub run: CommandOutcome,
}
