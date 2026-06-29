//! One structured stage invocation.
//!
//! A stage invocation repeatedly regenerates groups for the current accepted
//! snapshot until no group is accepted. It is shared by normal fixed-point
//! passes and by the final single-candidate sweep.

use crate::reducer::{candidate::StageKind, engine::ReductionContext};

/// Runs one stage until it reaches a stage-local fixed point.
pub(super) fn run_stage(
    context: &mut ReductionContext<'_>,
    stage: StageKind,
    round: usize,
    singles_only: bool,
) -> anyhow::Result<StageResult> {
    // A stage report covers one stage invocation. If the stage accepts a
    // candidate and loops, generated candidate counts accumulate across the
    // regenerated snapshots inside this same invocation.
    let mut report = context.stage_report(round, stage);
    let mut changed = false;

    loop {
        if context.trial_limit_reached() {
            break;
        }

        context.reject_invalid_current("Accepted source became unparsable")?;

        // Candidate groups are generated from the current accepted snapshot.
        // They must never be reused after `context.set_current` accepts a new
        // snapshot because their byte ranges and node ids are snapshot-local.
        let groups = context.generate_groups(stage, singles_only)?;
        let generated = groups
            .iter()
            .map(|group| group.candidates.len())
            .sum::<usize>();
        report.generated_candidates += generated;
        if groups.is_empty() {
            break;
        }

        let mut accepted = None;
        for group in groups {
            if context.trial_limit_reached() {
                break;
            }
            accepted = context.try_group(&group, &mut report, singles_only)?;
            if accepted.is_some() {
                // Accepting one group result invalidates every remaining group
                // from this snapshot. The outer loop will regenerate them.
                break;
            }
        }

        if let Some(new_snapshot) = accepted {
            // Commit the accepted snapshot through the context so the accepted
            // source mirror and engine state remain synchronized.
            context.set_current(new_snapshot)?;
            changed = true;
            continue;
        }

        break;
    }

    report.size_after = context.current().source.len();
    report.runtime_cost_after = context.current().score.runtime_cost_total;
    Ok(StageResult { report, changed })
}

/// Result of one stage run.
pub(super) struct StageResult {
    /// Report accumulated for this stage invocation.
    pub(super) report: crate::report::StageReport,
    /// Whether the stage accepted at least one candidate.
    pub(super) changed: bool,
}
