//! Deterministic single-candidate cleanup sweep.
//!
//! After the random point loop stops, this pass reuses normal candidate
//! generation but evaluates candidates one at a time. A single acceptance
//! invalidates all old ranges, so the pass immediately commits and regenerates.

use crate::{
    logging,
    reducer::{
        candidate::{Candidate, StageKind},
        engine::{ReductionContext, SimplificationObjective, TrialAttempt},
        group::{CandidateGroup, GroupStrategy},
    },
};

/// Runs a single-candidate sweep for an existing stage.
pub(super) fn run_single_candidate_sweep(
    context: &mut ReductionContext<'_>,
    stage: StageKind,
    description: &str,
    round: usize,
    objective: SimplificationObjective,
) -> anyhow::Result<bool> {
    // This helper is a deterministic "try each generated candidate once"
    // reducer. It intentionally accepts at most one candidate per pass, then
    // regenerates all candidates from the fresh snapshot to avoid stale ranges.
    let mut accepted_any = false;
    loop {
        if context.trial_limit_reached() {
            break;
        }

        let mut report = context.stage_report(round, stage);
        let groups = context.generate_groups(stage, false)?;

        // The report records how many candidates were visible at the start of
        // this pass. If an edit is accepted, a later loop iteration records the
        // regenerated candidate count separately.
        report.generated_candidates = groups
            .iter()
            .map(|group| group.candidates.len())
            .sum::<usize>();
        if groups.is_empty() {
            break;
        }

        let mut accepted = None;
        'groups: for group in groups {
            // Clone the candidate list because the temporary single-candidate
            // group below owns its candidate, while `group` metadata is still
            // needed for the rest of the loop.
            for candidate in group.candidates.clone() {
                if context.trial_limit_reached() {
                    break 'groups;
                }
                let single_group = single_candidate_group(&group, &candidate);
                match context.try_candidates(
                    std::slice::from_ref(&candidate),
                    &single_group,
                    &mut report,
                    description,
                    objective,
                )? {
                    TrialAttempt::Accepted(snapshot) | TrialAttempt::CachedAccepted(snapshot) => {
                        // Stop immediately after one acceptance. Continuing
                        // with candidates from the old snapshot would risk
                        // applying stale byte ranges.
                        report.accepted += 1;
                        accepted = Some(snapshot);
                        break 'groups;
                    }
                    TrialAttempt::Rejected(_)
                    | TrialAttempt::CachedRejected(_)
                    | TrialAttempt::InvalidEdit
                    | TrialAttempt::NotSimpler => {
                        report.rejected += 1;
                        context.record_rejected_attempt();
                    }
                }
            }
        }

        if let Some(snapshot) = accepted {
            // Commit the accepted source and run another pass with freshly
            // generated candidates.
            context.set_current(snapshot)?;
            accepted_any = true;
            report.size_after = context.current().source.len();
            report.runtime_cost_after = context.current().score.runtime_cost_total;
            context.push_stage_report(report);
            continue;
        }

        report.size_after = context.current().source.len();
        report.runtime_cost_after = context.current().score.runtime_cost_total;
        if report.generated_candidates > 0 || report.trials > 0 {
            logging::info(format_args!(
                "{description}: accepted {}, rejected {}, size {} -> {}",
                report.accepted, report.rejected, report.size_before, report.size_after
            ));
            context.push_stage_report(report);
        }
        break;
    }

    Ok(accepted_any)
}

/// Creates a single-candidate group while preserving parent group metadata.
fn single_candidate_group(group: &CandidateGroup, candidate: &Candidate) -> CandidateGroup {
    // The parent group's stage, kind, and description remain useful for reports.
    // Only the schedule changes: this wrapper forces exactly one candidate.
    CandidateGroup::new(
        format!("{}:single:{}", group.id.0, candidate.id.0),
        group.snapshot,
        group.stage,
        group.kind,
        group.description.clone(),
        vec![candidate.clone()],
        GroupStrategy::SinglesOnly,
        candidate.priority,
    )
}
