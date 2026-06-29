//! Pre-oracle simplification objective checks.
//!
//! These checks prevent expensive command executions for candidates that are
//! parseable but do not improve the active reduction goal. The oracle still
//! remains the only authority for interestingness.

use crate::{ir::ComplexityScore, reducer::candidate::StageKind};

/// Simplification objective used before expensive oracle execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SimplificationObjective {
    /// Use the deterministic stage-specific objective.
    Stage(StageKind),
    /// Accept any strict structural score decrease.
    AnyScoreDecrease,
}

/// Returns true when an edited snapshot satisfies the selected simplification objective.
pub(super) fn candidate_satisfies_objective(
    objective: SimplificationObjective,
    after: &ComplexityScore,
    before: &ComplexityScore,
) -> bool {
    match objective {
        SimplificationObjective::Stage(stage) => {
            candidate_satisfies_stage_objective(stage, after, before)
        }
        SimplificationObjective::AnyScoreDecrease => after.is_strictly_less_than(before),
    }
}

/// Returns true when a score decrease is valuable for the current normal stage.
///
/// Source bytes are the last tie-breaker in the structural score. Accepting a
/// byte-only edit inside an expensive structural stage can repeatedly restart
/// that stage without making new structural candidates possible. The final
/// sweep can still accept byte-only edits after higher-value simplifications
/// have reached a fixed point.
pub fn candidate_satisfies_stage_objective(
    stage: StageKind,
    after: &ComplexityScore,
    before: &ComplexityScore,
) -> bool {
    if matches!(
        stage,
        StageKind::FinalOneMinimalSweep | StageKind::BlankLineCleanup
    ) {
        return after.is_strictly_less_than(before);
    }

    if stage == StageKind::RuntimeCostReduction {
        return after.runtime_cost_total < before.runtime_cost_total
            || (after.loops < before.loops && after.source_bytes <= before.source_bytes);
    }

    if !after.is_strictly_less_than(before) {
        return false;
    }

    let mut after_without_bytes = after.clone();
    let mut before_without_bytes = before.clone();
    after_without_bytes.source_bytes = 0;
    before_without_bytes.source_bytes = 0;
    after_without_bytes.is_strictly_less_than(&before_without_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_structural_stages_reject_byte_only_decreases() {
        let before = ComplexityScore {
            statements: 1,
            source_bytes: 100,
            ..ComplexityScore::default()
        };
        let after = ComplexityScore {
            statements: 1,
            source_bytes: 90,
            ..ComplexityScore::default()
        };

        assert!(!candidate_satisfies_stage_objective(
            StageKind::StatementAndSiblingReduction,
            &after,
            &before
        ));
        assert!(candidate_satisfies_stage_objective(
            StageKind::FinalOneMinimalSweep,
            &after,
            &before
        ));
    }

    #[test]
    fn normal_structural_stages_accept_structural_decreases() {
        let before = ComplexityScore {
            statements: 2,
            source_bytes: 100,
            ..ComplexityScore::default()
        };
        let after = ComplexityScore {
            statements: 1,
            source_bytes: 90,
            ..ComplexityScore::default()
        };

        assert!(candidate_satisfies_stage_objective(
            StageKind::StatementAndSiblingReduction,
            &after,
            &before
        ));
    }

    #[test]
    fn runtime_phase_accepts_static_runtime_cost_decrease() {
        let before = ComplexityScore {
            runtime_cost_total: 100,
            source_bytes: 10,
            ..ComplexityScore::default()
        };
        let after = ComplexityScore {
            runtime_cost_total: 50,
            source_bytes: 10,
            ..ComplexityScore::default()
        };

        assert!(candidate_satisfies_stage_objective(
            StageKind::RuntimeCostReduction,
            &after,
            &before
        ));
    }
}
