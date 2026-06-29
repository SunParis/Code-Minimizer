//! Fixed-point pass for the structured algorithm.
//!
//! This module owns the outer deterministic loop over normal reducer stages.
//! It does not generate candidates itself; it delegates one stage invocation to
//! `stage::run_stage`, which uses the shared runtime context.

use crate::{
    logging,
    reducer::{candidate::StageKind, engine::ReductionContext},
};

use super::stage::run_stage;

/// Runs all normal stages to a fixed point.
pub(super) fn run_fixed_point(context: &mut ReductionContext<'_>) -> anyhow::Result<()> {
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
            // The context marks this flag when a global stop condition is
            // reached. The engine later copies it into the final JSON report.
            break;
        }

        if fixed_point_reached(round_changed) {
            logging::info(format_args!("fixed point reached after {round} rounds"));
            break;
        }
    }

    Ok(())
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
