//! One-candidate final sweep for the structured algorithm.
//!
//! The sweep reuses the normal stage runner but forces single-candidate chunk
//! plans. This catches leftovers that did not survive earlier grouped attempts.

use crate::{logging, reducer::engine::ReductionContext};

use super::{stage::run_stage, stage_order::final_sweep_stages};

/// Runs the final single-candidate sweep until no candidate is accepted.
pub(super) fn run_final_sweep(context: &mut ReductionContext<'_>) -> anyhow::Result<()> {
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
