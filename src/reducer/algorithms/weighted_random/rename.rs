//! Conservative whole-identifier rename sweep.
//!
//! This pass is intentionally language-neutral and speculative. It rewrites
//! long whole-token identifiers globally, then relies on parsing and the oracle
//! to reject unsafe renames.

use std::collections::HashSet;

use crate::{
    edit::{Edit, TextRange},
    logging,
    reducer::{
        candidate::{Candidate, StageKind, TransformKind},
        edit_plan::{EditIntent, EditPlanId, EditSafety},
        engine::{ReductionContext, SimplificationObjective, TrialAttempt},
        group::{CandidateGroup, CandidateGroupKind, GroupStrategy},
        ordering::normalize_candidates,
    },
};

/// Runs conservative whole-identifier renames for names that are unusually long.
pub(super) fn run_rename_sweep(
    context: &mut ReductionContext<'_>,
    round: usize,
) -> anyhow::Result<()> {
    // Renaming is speculative because this project does not yet have a precise
    // cross-language symbol resolver. The pass therefore rewrites whole
    // identifier tokens globally, then lets parse validation and the oracle
    // decide whether the rewrite is legal and interesting.
    loop {
        if context.trial_limit_reached() {
            break;
        }

        let candidates = rename_candidates(context);
        if candidates.is_empty() {
            break;
        }

        let mut report = context.stage_report(round, StageKind::ExpressionLiteralTypeCleanup);
        report.generated_candidates = candidates.len();
        let mut accepted = None;
        for candidate in candidates {
            if context.trial_limit_reached() {
                break;
            }

            // Each rename is wrapped as one single-candidate group so logs show
            // exactly which identifier rewrite was tested.
            let group = CandidateGroup::new(
                format!("weighted-random:rename:{}", candidate.id.0),
                context.current().version,
                StageKind::ExpressionLiteralTypeCleanup,
                CandidateGroupKind::LiteralShrinkSet,
                "Rename long identifier",
                vec![candidate.clone()],
                GroupStrategy::SinglesOnly,
                candidate.priority,
            );
            match context.try_candidates(
                std::slice::from_ref(&candidate),
                &group,
                &mut report,
                "Try one long-name rename",
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
            // Only one rename is accepted per pass. The next pass recomputes
            // identifier names from the accepted source, preventing overlapping
            // rename plans from applying to stale text.
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
                "long-name rename sweep: accepted {}, rejected {}, size {} -> {}",
                report.accepted, report.rejected, report.size_before, report.size_after
            ));
            context.push_stage_report(report);
        }
        break;
    }

    Ok(())
}

/// Conservative candidates that rename long whole-file identifiers.
fn rename_candidates(context: &ReductionContext<'_>) -> Vec<Candidate> {
    let source = &context.current().source;

    // Start with lexical identifier-like tokens rather than syntax-node names.
    // This keeps the pass language-neutral for Java and JavaScript.
    let mut names = collect_identifier_names(source);

    // Very short names are not worth spending oracle trials on. Reserved and
    // platform-looking names are skipped to avoid obvious global API rewrites.
    names.retain(|name| name.len() >= 12 && !reserved_identifier(name));

    // Try longer names first because they give the largest byte reduction.
    names.sort_by(|left, right| right.len().cmp(&left.len()).then_with(|| left.cmp(right)));

    // Bound the speculative rename search so very large fuzz cases do not spend
    // thousands of trials on identifiers before structural cleanup finishes.
    names.truncate(32);

    let mut candidates = Vec::new();
    for (index, name) in names.into_iter().enumerate() {
        let replacement = short_name(index);
        if source.contains(&replacement) {
            // Avoid introducing a name that already exists somewhere in the
            // file. This is conservative and prevents obvious capture bugs.
            continue;
        }

        // Replace every whole-token occurrence as one atomic edit. Partial
        // substring replacements are rejected by `whole_identifier_ranges`.
        let edits = whole_identifier_ranges(source, &name)
            .into_iter()
            .map(|range| Edit::Replace {
                range,
                replacement: replacement.clone(),
            })
            .collect::<Vec<_>>();
        if edits.is_empty() {
            continue;
        }
        let removed = edits.len() * name.len();
        let inserted = edits.len() * replacement.len();
        let mut candidate = Candidate::new(
            StageKind::ExpressionLiteralTypeCleanup,
            TransformKind::Cleanup,
            format!("weighted-random:rename:{name}"),
            "Rename long identifier",
            Edit::Multi(edits),
            80,
            inserted as isize - removed as isize,
        );

        // Mark the candidate as speculative cleanup. The oracle remains the
        // authority, but reports should show that this is riskier than a trivia
        // deletion.
        candidate.plan.intent = EditIntent::Cleanup;
        candidate.plan.safety = EditSafety::Speculative;
        candidate.plan.id = EditPlanId(candidate.id.0.clone());
        candidates.push(candidate);
    }

    normalize_candidates(candidates)
}

/// Collects likely identifiers from source text.
fn collect_identifier_names(source: &str) -> Vec<String> {
    let mut names = HashSet::new();
    let mut start = None;

    // The scanner recognizes Java/JavaScript identifier characters well enough
    // for whole-token replacement. It intentionally ignores parser context so it
    // can operate after either language adapter.
    for (index, ch) in source.char_indices() {
        if is_identifier_part(ch) {
            start.get_or_insert(index);
        } else if let Some(name_start) = start.take() {
            push_identifier(source, name_start, index, &mut names);
        }
    }
    if let Some(name_start) = start {
        push_identifier(source, name_start, source.len(), &mut names);
    }
    names.into_iter().collect()
}

/// Adds one identifier candidate when it has a valid leading character.
fn push_identifier(source: &str, start: usize, end: usize, names: &mut HashSet<String>) {
    let text = &source[start..end];

    // Digits are allowed after the first character but not as the first
    // character. This filters numeric literals that were collected as runs of
    // identifier-part characters.
    if text
        .chars()
        .next()
        .is_some_and(|ch| ch == '_' || ch == '$' || ch.is_ascii_alphabetic())
    {
        names.insert(text.to_owned());
    }
}

/// Finds whole-identifier ranges for one name.
fn whole_identifier_ranges(source: &str, name: &str) -> Vec<TextRange> {
    // `match_indices` can find substrings inside longer identifiers. The
    // boundary checks below keep replacements token-aligned.
    source
        .match_indices(name)
        .filter_map(|(start, _)| {
            let end = start + name.len();
            let before = source[..start].chars().next_back();
            let after = source[end..].chars().next();
            (!before.is_some_and(is_identifier_part) && !after.is_some_and(is_identifier_part))
                .then_some(TextRange { start, end })
        })
        .collect()
}

/// Returns a compact replacement name.
fn short_name(index: usize) -> String {
    // `vN` is valid in both Java and JavaScript and is short enough to reduce
    // names of length 12 or more even when many occurrences are rewritten.
    format!("v{index}")
}

/// Returns true when a name should not be renamed blindly.
fn reserved_identifier(name: &str) -> bool {
    // This is not a complete keyword list. It is a deliberately small denylist
    // for common built-ins and runtime hooks that global textual renaming would
    // almost always break.
    matches!(
        name,
        "String"
            | "System"
            | "Object"
            | "Integer"
            | "Boolean"
            | "Exception"
            | "RuntimeException"
            | "constructor"
            | "prototype"
            | "arguments"
            | "undefined"
    )
}

/// Returns true when a character can appear in a Java/JavaScript identifier.
fn is_identifier_part(ch: char) -> bool {
    // The reducer primarily handles generated Java and JavaScript. ASCII
    // alphanumerics plus `_` and `$` cover the common fuzzing identifiers and
    // avoid Unicode normalization questions in byte-range edits.
    ch == '_' || ch == '$' || ch.is_ascii_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_identifier_ranges_ignore_substrings() {
        let ranges = whole_identifier_ranges("longName longName2 xlongName longName", "longName");
        assert_eq!(ranges.len(), 2);
    }
}
