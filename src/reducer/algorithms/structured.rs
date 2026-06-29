//! Deterministic structured reduction algorithm.
//!
//! This module contains the original reduction flow. It is deterministic: each
//! round visits the configured stage order, each stage asks the shared candidate
//! generators for groups, and each group follows its precomputed chunk plan.
//! After no normal stage accepts a candidate, the final sweep retries remaining
//! candidates one at a time for practical one-minimality.

use crate::{
    logging,
    reducer::{
        algorithms::ReducerAlgorithm, candidate::StageKind, engine::ReductionContext,
        final_sweep::final_sweep_stages,
    },
};

/// Deterministic stage-and-chunk reducer.
pub struct StructuredAlgorithm;

impl ReducerAlgorithm for StructuredAlgorithm {
    fn run(&self, context: &mut ReductionContext<'_>) -> anyhow::Result<()> {
        // First run the high-value normal stages until they reach a fixed point.
        // Then force a single-candidate sweep to catch leftovers that were not
        // accepted as part of a chunk or group.
        run_fixed_point(context)?;
        run_final_sweep(context)
    }
}

/// Runs all normal stages to a fixed point.
fn run_fixed_point(context: &mut ReductionContext<'_>) -> anyhow::Result<()> {
    // `max_rounds` bounds complete passes over the stage list. A value of `0`
    // from the CLI has already been converted to `usize::MAX` in configuration.
    for round in 1..=context.config().limits.max_rounds {
        // A round is considered productive if any stage accepted at least one
        // candidate. Byte size is intentionally not used as the fixed-point
        // signal because some accepted structural edits do not reduce bytes.
        let mut round_changed = false;

        for stage in StageKind::ordered() {
            if context.trial_limit_reached() {
                break;
            }

            let stage_result = run_stage(context, *stage, round, false)?;
            round_changed |= stage_result.changed;

            // Empty stages are omitted from the report/log to keep long runs
            // readable. A stage that generated or tried anything is preserved.
            if stage_result.report.generated_candidates > 0 || stage_result.report.trials > 0 {
                logging::info(format_args!(
                    "stage {} ({}): accepted {}/{}, size {} -> {}",
                    stage.display_name(),
                    stage.as_str(),
                    stage_result.report.accepted,
                    stage_result.report.generated_candidates,
                    stage_result.report.size_before,
                    stage_result.report.size_after
                ));
                context.push_stage_report(stage_result.report);
            }
        }

        if context.stopped_by_limit() {
            // The context marks this flag when the global trial limit is reached.
            // The engine later copies it into the final JSON report.
            break;
        }

        if fixed_point_reached(round_changed) {
            logging::info(format_args!("fixed point reached after {round} rounds"));
            break;
        }
    }

    Ok(())
}

/// Runs the final single-candidate sweep until no candidate is accepted.
fn run_final_sweep(context: &mut ReductionContext<'_>) -> anyhow::Result<()> {
    // The sweep restarts from the first source stage after every acceptance. An
    // accepted single edit invalidates all later ranges from the old snapshot.
    let mut sweep_round = 1_usize;
    loop {
        if context.trial_limit_reached() {
            break;
        }

        let mut accepted_any = false;
        for stage in final_sweep_stages() {
            // `singles_only=true` makes `ReductionContext::generate_groups`
            // rewrite chunk plans so each attempt contains exactly one
            // candidate.
            let result = run_stage(context, *stage, sweep_round, true)?;
            if result.report.generated_candidates > 0 || result.report.trials > 0 {
                logging::info(format_args!(
                    "final sweep from {} ({}): accepted {}/{}, size {} -> {}",
                    stage.display_name(),
                    stage.as_str(),
                    result.report.accepted,
                    result.report.generated_candidates,
                    result.report.size_before,
                    result.report.size_after
                ));
                context.push_stage_report(result.report);
            }
            if result.changed {
                // Stop the current sweep pass immediately; the next loop
                // iteration regenerates groups from the newly accepted snapshot.
                accepted_any = true;
                break;
            }
        }

        if !accepted_any {
            logging::info(format_args!(
                "final one-minimal sweep reached a fixed point"
            ));
            break;
        }
        sweep_round = sweep_round.saturating_add(1);
    }

    Ok(())
}

/// Runs one stage until it reaches a stage-local fixed point.
fn run_stage(
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
struct StageResult {
    /// Report accumulated for this stage invocation.
    report: crate::report::StageReport,
    /// Whether the stage accepted at least one candidate.
    changed: bool,
}

/// Returns true when a normal reducer round accepted no candidate.
///
/// Fixed-point detection must be based on acceptance, not source length. Some
/// syntax-preserving edits reduce structural score without changing byte count,
/// and stopping after such a round can miss follow-up candidates that only
/// become visible after the accepted structural rewrite.
fn fixed_point_reached(round_changed: bool) -> bool {
    !round_changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_point_depends_on_acceptance_not_byte_size() {
        assert!(!fixed_point_reached(true));
        assert!(fixed_point_reached(false));
    }
}
