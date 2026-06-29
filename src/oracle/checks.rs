//! Oracle validation and reason formatting.
//!
//! These helpers define what counts as a completed build/run and how the oracle
//! explains lost diffs or exit-status policy failures.

use crate::{
    config::{DiffMode, PreserveExit, TrialSide},
    output_diff::OutputDiff,
    runner::{CommandOutcome, ExitStatusSummary, InvocationOutcome},
};

/// Rejects builds that timed out, truncated output, or exited unsuccessfully.
pub(super) fn validate_build_completed(outcome: &CommandOutcome) -> anyhow::Result<()> {
    if outcome.timed_out {
        anyhow::bail!("build command timed out");
    }
    if outcome.output_truncated() {
        anyhow::bail!("build command output exceeded the capture limit");
    }
    if !outcome.status.is_success() {
        anyhow::bail!("build command exited with status {:?}", outcome.status);
    }
    Ok(())
}

/// Rejects run outcomes that timed out.
pub(super) fn validate_run_completed(outcome: &InvocationOutcome) -> Result<(), String> {
    if outcome.run.timed_out {
        return Err("run command timed out".to_owned());
    }
    Ok(())
}

/// Rejects any side whose build or run output exceeded the hard cap.
pub(super) fn validate_no_truncation(outcome: &InvocationOutcome) -> Result<(), String> {
    if let Some(build) = &outcome.build {
        if build.output_truncated() {
            return Err("build output exceeded the capture limit".to_owned());
        }
    }
    if outcome.run.output_truncated() {
        return Err("run output exceeded the capture limit".to_owned());
    }
    Ok(())
}

/// Explains an exit-preservation rejection with baseline and candidate details.
pub(super) fn exit_status_change_reason(
    side: TrialSide,
    policy: PreserveExit,
    baseline: &ExitStatusSummary,
    current: &ExitStatusSummary,
) -> String {
    format!(
        "Side {} run exit status changed under preserve-exit {:?}: baseline={:?}, candidate={:?}",
        side.as_label(),
        policy,
        baseline,
        current
    )
}

/// Explains a lost A/B difference with the configured mode and observed diff state.
pub(super) fn diff_lost_reason(mode: DiffMode, diff: &OutputDiff) -> String {
    format!(
        "A/B output difference was lost for diff-mode {:?}: observed stdout differs={}, stderr differs={}, exit differs={}",
        mode, diff.stdout_differs, diff.stderr_differs, diff.exit_differs
    )
}

/// Compares trial exit status against the baseline according to policy.
pub(super) fn exit_status_matches(
    policy: PreserveExit,
    baseline: &ExitStatusSummary,
    current: &ExitStatusSummary,
) -> bool {
    match policy {
        PreserveExit::None => true,
        PreserveExit::SameClass => baseline.class() == current.class(),
        PreserveExit::Exact => baseline == current,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_status_change_reason_includes_policy_and_statuses() {
        let reason = exit_status_change_reason(
            TrialSide::A,
            PreserveExit::SameClass,
            &ExitStatusSummary::Code(0),
            &ExitStatusSummary::Code(1),
        );

        assert!(reason.contains("Side A"));
        assert!(reason.contains("SameClass"));
        assert!(reason.contains("baseline=Code(0)"));
        assert!(reason.contains("candidate=Code(1)"));
    }

    #[test]
    fn diff_lost_reason_includes_mode_and_observed_channels() {
        let reason = diff_lost_reason(
            DiffMode::Stdout,
            &OutputDiff {
                stdout_differs: false,
                stderr_differs: true,
                exit_differs: false,
            },
        );

        assert!(reason.contains("Stdout"));
        assert!(reason.contains("stdout differs=false"));
        assert!(reason.contains("stderr differs=true"));
    }
}
