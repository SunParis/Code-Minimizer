//! Weighted random reduction algorithm.
//!
//! The algorithm is intentionally a scheduler, not a second reducer engine. It
//! asks the normal candidate generators for statement-deletion candidates,
//! treats each candidate as a "point", chooses points with weighted random
//! sampling, and sends every trial through `ReductionContext`. That keeps parse
//! validation, oracle execution, cache behavior, accepted-source writes, and
//! reports identical to the deterministic reducer.
//!
//! The implementation is split by responsibility:
//!
//! - `point_loop` owns the adaptive random deletion loop.
//! - `points` converts generated reducer candidates into weighted points.
//! - `weights` owns patience, sampling, and neighborhood update math.
//! - `cleanup` and `rename` own algorithm-specific late deterministic sweeps.
//! - Shared reducer cleanup removes blank lines after every algorithm finishes.
//! - `rng` owns the tiny deterministic pseudo-random generator.

mod cleanup;
mod point_loop;
mod points;
mod rename;
mod rng;
mod weights;

use crate::reducer::{
    algorithms::ReducerAlgorithm,
    candidate::StageKind,
    engine::{ReductionContext, SimplificationObjective},
};

/// Weighted random point reducer.
pub struct WeightedRandomAlgorithm;

impl ReducerAlgorithm for WeightedRandomAlgorithm {
    fn run(&self, context: &mut ReductionContext<'_>) -> anyhow::Result<()> {
        // The main loop handles the exploratory part of the algorithm: sample
        // statement points, delete one, and adapt weights from the result.
        point_loop::run_weighted_point_loop(context)?;

        // After random deletion stops, reuse existing cleanup candidates as a
        // deterministic single-candidate sweep. This implements the requested
        // "try one dead declaration/output cleanup, accept or rollback" phase.
        cleanup::run_single_candidate_sweep(
            context,
            StageKind::DeadDeclarationAndOutputCleanup,
            "weighted cleanup sweep",
            1,
            SimplificationObjective::AnyScoreDecrease,
        )?;

        // Name shrinking is intentionally late because it is byte-oriented and
        // should not consume trials before structural deletion has had a chance
        // to remove larger regions. Blank-line cleanup is shared by the engine
        // and runs after every algorithm, including this one.
        rename::run_rename_sweep(context, 1)
    }
}
