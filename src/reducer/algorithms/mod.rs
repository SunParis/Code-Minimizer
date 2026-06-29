//! Pluggable reducer algorithms.
//!
//! This module is the boundary between "how candidates are scheduled" and "how
//! a candidate is validated". Algorithm implementations decide which candidate
//! or group should be tried next. They do not parse source, run commands, update
//! the workspace directly, or decide interestingness. Those shared operations
//! remain in `ReducerEngine` and are exposed through `ReductionContext`.

use crate::config::ReductionAlgorithm;

use super::engine::ReductionContext;

pub mod structured;
pub mod weighted_random;

/// Common interface implemented by concrete reducer algorithms.
pub trait ReducerAlgorithm {
    /// Runs the algorithm against the shared reduction context.
    ///
    /// Implementations should keep calling context methods until they reach
    /// their own stopping condition or the context reports a configured limit.
    /// The method returns after all algorithm-specific cleanup passes are done.
    fn run(&self, context: &mut ReductionContext<'_>) -> anyhow::Result<()>;
}

/// Returns the algorithm implementation for a validated configuration value.
pub fn algorithm_for(algorithm: ReductionAlgorithm) -> Box<dyn ReducerAlgorithm> {
    // A boxed trait object keeps `ReducerEngine` independent from concrete
    // algorithm types. Adding another algorithm should only require a new module
    // and one additional match arm here.
    match algorithm {
        ReductionAlgorithm::Structured => Box::new(structured::StructuredAlgorithm),
        ReductionAlgorithm::WeightedRandom => Box::new(weighted_random::WeightedRandomAlgorithm),
    }
}
