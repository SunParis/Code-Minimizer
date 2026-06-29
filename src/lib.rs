//! Public library entry points for Code Minimizer.
//!
//! The crate is organized by engineering responsibility rather than by a flat
//! list of implementation files:
//!
//! - [`app`] owns user-facing configuration, CLI parsing, logging, reports, and
//!   error types.
//! - [`execution`] owns shell-command expansion, execution, timeout handling,
//!   output capture, and signal cleanup.
//! - [`workspace`] owns the on-disk trial layout.
//! - [`oracle`] owns interestingness decisions for A/B command outcomes.
//! - [`source`] owns language-agnostic text edits and output-diff utilities.
//! - [`reducer`], [`ir`], and [`lang`] own the core minimization model.
//!
//! The binary is intentionally thin: it parses CLI arguments, builds a
//! [`config::ReduceConfig`], and delegates the actual work to the reducer
//! engine. Keeping the core logic in the library makes the oracle, command
//! runner, edit model, and language adapters directly testable.

pub mod app;
pub mod execution;
pub mod ir;
pub mod lang;
pub mod oracle;
pub mod reducer;
pub mod source;
pub mod workspace;

/// Backward-compatible access to the trial cache while the implementation lives
/// with reducer state.
pub mod cache {
    pub use crate::reducer::cache::*;
}

/// Backward-compatible access to CLI parsing while the implementation lives in
/// the application-facing module group.
pub mod cli {
    pub use crate::app::cli::*;
}

/// Backward-compatible access to command templates while execution details live
/// under `execution`.
pub mod command_template {
    pub use crate::execution::command_template::*;
}

/// Backward-compatible access to reducer configuration.
pub mod config {
    pub use crate::app::config::*;
}

/// Backward-compatible access to source edit primitives.
pub mod edit {
    pub use crate::source::edit::*;
}

/// Backward-compatible access to crate error types.
pub mod error {
    pub use crate::app::error::*;
}

/// Backward-compatible access to timestamped logging helpers.
pub mod logging {
    pub use crate::app::logging::*;
}

/// Backward-compatible access to output-diff helpers.
pub mod output_diff {
    pub use crate::source::output_diff::*;
}

/// Backward-compatible access to JSON report types.
pub mod report {
    pub use crate::app::report::*;
}

/// Backward-compatible access to command execution types.
pub mod runner {
    pub use crate::execution::runner::*;
}

pub use config::ReduceConfig;
pub use reducer::engine::ReducerEngine;
