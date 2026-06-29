//! Deterministic structured reduction algorithm.
//!
//! The structured reducer is a stage-and-chunk scheduler. It repeatedly visits
//! the shared stage order, lets each stage reach a local fixed point, and then
//! runs a one-candidate final sweep. Shared parsing, oracle execution, rollback,
//! workspace writes, and reporting stay in the reducer runtime layer.

mod final_sweep;
mod fixed_point;
mod stage;
mod stage_order;

use crate::reducer::{algorithms::ReducerAlgorithm, engine::ReductionContext};

use self::{final_sweep::run_final_sweep, fixed_point::run_fixed_point};

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
