//! Serializable oracle observations and decisions.
//!
//! These types are shared by the reducer state, JSON reports, and tests. They
//! intentionally contain outcomes and reason text, not reducer scheduling
//! details.

use serde::{Deserialize, Serialize};

use crate::{output_diff::OutputDiff, runner::InvocationOutcome};

/// Baseline A/B outcomes and their output diff.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Baseline {
    /// Invocation outcome for side A.
    pub a: InvocationOutcome,
    /// Invocation outcome for side B.
    pub b: InvocationOutcome,
    /// Baseline stdout/stderr diff state.
    pub diff: OutputDiff,
}

/// Verdict returned by the oracle for one candidate source.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleDecision {
    /// Whether the candidate remains interesting.
    pub accepted: bool,
    /// Stable rejection reason when `accepted` is false.
    pub reason: Option<String>,
    /// Diff state when both commands reached the comparison step.
    pub diff: Option<OutputDiff>,
    /// Side A outcome when available.
    pub a: Option<InvocationOutcome>,
    /// Side B outcome when available.
    pub b: Option<InvocationOutcome>,
}

impl OracleDecision {
    /// Creates an accepted decision with full command outcomes.
    pub(super) fn accepted(a: InvocationOutcome, b: InvocationOutcome, diff: OutputDiff) -> Self {
        Self {
            accepted: true,
            reason: None,
            diff: Some(diff),
            a: Some(a),
            b: Some(b),
        }
    }

    /// Creates a rejected decision with a stable English reason.
    pub(super) fn rejected(reason: impl Into<String>) -> Self {
        Self {
            accepted: false,
            reason: Some(reason.into()),
            diff: None,
            a: None,
            b: None,
        }
    }

    /// Creates a rejected decision that still records command outcomes and diff.
    pub(super) fn rejected_with_outcomes(
        reason: impl Into<String>,
        a: InvocationOutcome,
        b: InvocationOutcome,
        diff: OutputDiff,
    ) -> Self {
        Self {
            accepted: false,
            reason: Some(reason.into()),
            diff: Some(diff),
            a: Some(a),
            b: Some(b),
        }
    }
}

/// Annotates a candidate rejection with the confirmation number that failed.
pub(super) fn with_confirmation_reason(
    mut decision: OracleDecision,
    confirmation: usize,
) -> OracleDecision {
    if confirmation > 1 {
        if let Some(reason) = decision.reason.take() {
            decision.reason = Some(format!("{reason} on confirmation {confirmation}"));
        }
    }
    decision
}
