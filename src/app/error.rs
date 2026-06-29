//! Error types used across the reducer.
//!
//! Most public APIs return [`anyhow::Result`] so callers receive context-rich
//! messages, but this module provides stable domain errors for validation
//! paths that tests and CLI code need to distinguish.

use std::path::PathBuf;

use thiserror::Error;

/// Domain-level failures that are expected during configuration or reduction.
#[derive(Debug, Error)]
pub enum CodeMinimizerError {
    /// The requested source language does not have a registered adapter.
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    /// A command template references a placeholder that the runner does not know.
    #[error("Unknown command placeholder: {0}")]
    UnknownPlaceholder(String),

    /// The user supplied a command template that cannot be executed.
    #[error("Invalid command template: {0}")]
    InvalidCommandTemplate(String),

    /// A text edit has invalid byte ranges for the current source string.
    #[error("Invalid text edit: {0}")]
    InvalidEdit(String),

    /// The source cannot be represented as a valid parsed program.
    #[error("Parse failed: {0}")]
    ParseFailed(String),

    /// The baseline A/B run is not interesting, so reduction must not start.
    #[error("Baseline is not interesting: {0}")]
    BaselineNotInteresting(String),

    /// The reducer reached a configured limit before exhausting all candidates.
    #[error("Reduction limit reached: {0}")]
    LimitReached(String),

    /// A required input file was not found.
    #[error("Input file not found: {}", .0.display())]
    MissingInput(PathBuf),
}
