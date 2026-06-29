//! Adaptive weighted-random statement deletion loop.
//!
//! This module owns the only stochastic part of the algorithm. It repeatedly
//! samples one current point, evaluates the corresponding candidate through the
//! shared reducer context, and updates or removes points based on the outcome.

use crate::{
    logging,
    reducer::{
        candidate::StageKind,
        engine::{ReductionContext, SimplificationObjective, TrialAttempt},
    },
};

use super::{
    points::{collect_points, point_group},
    rng::SmallRng,
    weights::{
        decrease_neighbor_weights, increase_neighbor_weights, patience_limit, remove_point,
        weighted_index,
    },
};

/// Runs the random weighted statement-deletion loop.
pub(super) fn run_weighted_point_loop(context: &mut ReductionContext<'_>) -> anyhow::Result<()> {
    // A "round" starts from a freshly accepted snapshot and a freshly collected
    // point list. The loop may accept multiple points inside one round, but
    // each acceptance rebuilds the snapshot and therefore refreshes all ranges.
    let mut round = 1_usize;
    loop {
        if context.trial_limit_reached() {
            break;
        }

        context.reject_invalid_current("Accepted source became unparsable")?;
        let mut points = collect_points(context)?;
        if points.is_empty() {
            // No points means the current parser/candidate generator cannot
            // express any more simple statement deletions for this snapshot.
            logging::info(format_args!(
                "weighted random point loop found no deletable statement points"
            ));
            break;
        }

        let mut report = context.stage_report(round, StageKind::StatementAndSiblingReduction);
        report.generated_candidates = points.len();

        // The PRNG is deterministic for a given accepted source and round. That
        // makes runs reproducible while still exploring candidates in a
        // non-priority order.
        let mut rng =
            SmallRng::seed_from_snapshot(context.current().source_hash.0.as_bytes(), round as u64);

        // Patience is the "no accepted deletion for this many attempts" stop
        // condition. It scales with the current search space so tiny cases do
        // not spin, while large cases still get some random exploration.
        let patience = patience_limit(points.len());
        let mut rejected_without_accept = 0_usize;
        let mut accepted_any = false;

        logging::info(format_args!(
            "weighted random point loop round {round}: points={}, patience={patience}",
            points.len()
        ));

        while !points.is_empty()
            && rejected_without_accept < patience
            && !context.trial_limit_reached()
        {
            let index = weighted_index(&points, &mut rng);
            let point = points[index].clone();

            // The shared trial path expects a group for logging and reporting.
            // This synthetic group wraps exactly one sampled candidate and does
            // not change the edit being tested.
            let group = point_group(context.current().version, &point.candidate);
            let attempt = context.try_candidates(
                std::slice::from_ref(&point.candidate),
                &group,
                &mut report,
                "Try one weighted random point",
                SimplificationObjective::AnyScoreDecrease,
            )?;

            match attempt {
                TrialAttempt::Accepted(snapshot) | TrialAttempt::CachedAccepted(snapshot) => {
                    // An accepted candidate becomes the new source of truth.
                    // All old byte ranges and node ids are stale after this
                    // point, so the point list must be regenerated immediately.
                    report.accepted += 1;
                    context.set_current(snapshot)?;
                    accepted_any = true;
                    rejected_without_accept = 0;
                    points = collect_points(context)?;
                    report.generated_candidates += points.len();

                    // We no longer have the accepted point after regeneration,
                    // but its old byte center is still a useful locality hint
                    // for boosting nearby points in the new snapshot.
                    let accepted_center = point.center;
                    increase_neighbor_weights(&mut points, accepted_center);
                    logging::info(format_args!(
                        "weighted random point accepted; rebuilt point list with {} remaining points",
                        points.len()
                    ));
                }
                TrialAttempt::InvalidEdit | TrialAttempt::NotSimpler => {
                    // These failures happen before oracle execution. In the
                    // current snapshot, retrying this exact range would only
                    // repeat the same invalid/no-improvement result.
                    report.rejected += 1;
                    context.record_rejected_attempt();
                    remove_point(&mut points, index);
                    rejected_without_accept = rejected_without_accept.saturating_add(1);
                }
                TrialAttempt::Rejected(reason) | TrialAttempt::CachedRejected(reason) => {
                    // Oracle-level rejections split into two categories:
                    // syntax-like fragility removes the point, while a valid
                    // but uninteresting result only reduces local probability.
                    report.rejected += 1;
                    context.record_rejected_attempt();
                    if syntax_like_rejection(&reason) {
                        remove_point(&mut points, index);
                    } else {
                        decrease_neighbor_weights(&mut points, point.center);
                    }
                    rejected_without_accept = rejected_without_accept.saturating_add(1);
                }
                TrialAttempt::Interrupted => {
                    break;
                }
            }
        }

        report.size_after = context.current().source.len();
        report.runtime_cost_after = context.current().score.runtime_cost_total;
        logging::info(format_args!(
            "weighted random point loop round {round}: accepted {}, rejected {}, size {} -> {}",
            report.accepted, report.rejected, report.size_before, report.size_after
        ));
        context.push_stage_report(report);

        if !accepted_any || context.stopped_by_limit() {
            // If a round found no accepted deletion before patience expired,
            // the random phase has reached its local stopping condition.
            break;
        }
        round = round.saturating_add(1);
    }

    Ok(())
}

/// Returns true when an oracle rejection indicates syntax, compile, or timeout fragility.
fn syntax_like_rejection(reason: &str) -> bool {
    // Oracle reasons are human-facing strings today. This classifier is
    // deliberately broad and conservative: if a deletion appears to break
    // parsing/building/execution shape, removing it from the point list is safer
    // than repeatedly lowering nearby weights for a point that cannot work.
    let lower = reason.to_ascii_lowercase();
    lower.contains("parse")
        || lower.contains("build")
        || lower.contains("compile")
        || lower.contains("command failed")
        || lower.contains("exit status changed")
        || lower.contains("timed out")
        || lower.contains("timeout")
        || lower.contains("not found")
        || lower.contains("cannot find symbol")
        || lower.contains("undeclared")
        || lower.contains("undefined")
}
