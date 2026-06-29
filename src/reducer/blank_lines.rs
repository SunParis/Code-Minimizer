//! Shared blank-line cleanup pass.
//!
//! Blank-line removal is intentionally implemented outside concrete algorithms.
//! Every algorithm can focus on candidate scheduling, while this module gives
//! the whole reducer one final, validated byte-oriented cleanup stage. Each
//! blank or whitespace-only line is still tested through the normal edit,
//! parse, objective, cache, oracle, and rollback path before it becomes current.

use crate::{
    edit::{Edit, TextRange},
    ir::SnapshotId,
    logging,
    reducer::{
        candidate::{Candidate, StageKind, TransformKind},
        engine::{ReductionContext, SimplificationObjective, TrialAttempt},
        group::{CandidateGroup, CandidateGroupKind, GroupStrategy},
        ordering::normalize_candidates,
    },
};

/// Runs the final blank-line cleanup stage for any reducer algorithm.
pub fn run_final_blank_line_cleanup(
    context: &mut ReductionContext<'_>,
    round: usize,
) -> anyhow::Result<()> {
    // We accept at most one line per inner pass because every accepted edit
    // shifts byte offsets. Rebuilding the snapshot and recomputing candidates
    // keeps all following edit ranges tied to the current accepted source.
    loop {
        if context.trial_limit_reached() {
            break;
        }

        let candidates = blank_line_candidates(context);
        if candidates.is_empty() {
            break;
        }

        let mut report = context.stage_report(round, StageKind::BlankLineCleanup);
        report.generated_candidates = candidates.len();
        let mut accepted = None;

        for candidate in candidates {
            if context.trial_limit_reached() {
                break;
            }

            // A single-candidate group gives logs and reports the same context
            // as structural stages while making rollback as simple as keeping
            // the previous accepted snapshot when the oracle rejects the edit.
            let group = CandidateGroup::new(
                format!("blank-line-cleanup:{}", candidate.id.0),
                context.current().version,
                StageKind::BlankLineCleanup,
                CandidateGroupKind::LiteralShrinkSet,
                "Remove blank or whitespace-only lines",
                vec![candidate.clone()],
                GroupStrategy::SinglesOnly,
                candidate.priority,
            );

            match context.try_candidates(
                std::slice::from_ref(&candidate),
                &group,
                &mut report,
                "Try one blank-line cleanup",
                SimplificationObjective::AnyScoreDecrease,
            )? {
                TrialAttempt::Accepted(snapshot) | TrialAttempt::CachedAccepted(snapshot) => {
                    report.accepted += 1;
                    accepted = Some(snapshot);
                    break;
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

        if let Some(snapshot) = accepted {
            context.set_current(snapshot)?;
            report.size_after = context.current().source.len();
            report.runtime_cost_after = context.current().score.runtime_cost_total;
            context.push_stage_report(report);
            continue;
        }

        report.size_after = context.current().source.len();
        report.runtime_cost_after = context.current().score.runtime_cost_total;
        if report.generated_candidates > 0 || report.trials > 0 {
            logging::info(format_args!(
                "blank-line cleanup: accepted {}, rejected {}, size {} -> {}",
                report.accepted, report.rejected, report.size_before, report.size_after
            ));
            context.push_stage_report(report);
        }
        break;
    }

    Ok(())
}

/// Builds one deletion candidate for each blank or whitespace-only source line.
fn blank_line_candidates(context: &ReductionContext<'_>) -> Vec<Candidate> {
    blank_line_candidates_for_source(&context.current().source, context.current().version)
}

/// Builds blank-line candidates from raw source for the provided snapshot.
fn blank_line_candidates_for_source(source: &str, snapshot: SnapshotId) -> Vec<Candidate> {
    let bytes = source.as_bytes();
    let mut candidates = Vec::new();
    let mut line_start = 0_usize;

    // Work with byte offsets rather than `str::lines()` so the edit range can
    // include the exact trailing newline when one exists. Blank-line detection
    // uses `trim()` on valid UTF-8 slices, which also treats whitespace-only
    // lines as removable candidates.
    for (index, byte) in bytes.iter().enumerate() {
        if *byte == b'\n' {
            push_blank_line_candidate(source, line_start, index + 1, &mut candidates);
            line_start = index + 1;
        }
    }

    // `lines()` would ignore a final unterminated line. Treat an all-whitespace
    // tail as a candidate too; parser/oracle validation will reject it if
    // removing it would make the source invalid or uninteresting.
    if line_start < bytes.len() {
        push_blank_line_candidate(source, line_start, bytes.len(), &mut candidates);
    }

    normalize_candidates(candidates)
        .into_iter()
        .map(|candidate| candidate.for_snapshot(snapshot, None))
        .collect()
}

/// Adds a candidate when the byte range covers only whitespace.
fn push_blank_line_candidate(
    source: &str,
    line_start: usize,
    line_end: usize,
    candidates: &mut Vec<Candidate>,
) {
    if !source[line_start..line_end].trim().is_empty() {
        return;
    }

    let candidate = Candidate::new(
        StageKind::BlankLineCleanup,
        TransformKind::Cleanup,
        format!("blank-line:{line_start}"),
        "Remove blank or whitespace-only line",
        Edit::Delete(TextRange {
            start: line_start,
            end: line_end,
        }),
        30,
        -((line_end - line_start) as isize),
    );
    candidates.push(candidate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_blank_lines_with_and_without_final_newline() {
        let candidates = blank_line_candidates_for_source("let x = 1;\n  \n\t", SnapshotId::ROOT);

        assert_eq!(candidates.len(), 2);
        assert!(
            candidates
                .iter()
                .all(|candidate| candidate.stage == StageKind::BlankLineCleanup)
        );
    }
}
