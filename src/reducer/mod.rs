//! Language-independent reduction engine.
//!
//! Reducer code is split by responsibility:
//!
//! - [`algorithms`] contains concrete scheduling algorithms.
//! - [`runtime`] owns the shared engine, context, trial execution, cache, state,
//!   reporting, and validation services used by every algorithm.
//! - [`model`] owns candidate, group, edit-plan, strategy, and trial data types.
//! - [`planning`] owns shared candidate planning, grouping, ordering, and ddmin
//!   helpers.
//! - [`shared_stages`] owns final stages that every algorithm runs.
//!
//! The thin compatibility modules below keep older `crate::reducer::*` paths
//! stable while the physical files live in clearer subdirectories.

pub mod algorithms;
pub mod model;
pub mod planning;
pub mod runtime;
pub mod shared_stages;

#[cfg(test)]
mod engine_tests;

pub mod attempt {
    pub use super::runtime::attempt::*;
}

pub mod blank_lines {
    pub use super::shared_stages::blank_lines::*;
}

pub mod cache {
    pub use super::runtime::cache::*;
}

pub mod candidate {
    pub use super::model::candidate::*;
}

pub mod context {
    pub use super::runtime::context::*;
}

pub mod ddmin {
    pub use super::planning::ddmin::*;
}

pub mod edit_plan {
    pub use super::model::edit_plan::*;
}

pub mod engine {
    pub use super::runtime::engine::*;
}

pub mod group {
    pub use super::model::group::*;
}

pub mod objective {
    pub use super::runtime::objective::*;
}

pub mod ordering {
    pub use super::planning::ordering::*;
}

pub mod phase {
    pub use super::planning::phase::*;
}

pub mod strategy {
    pub use super::model::strategy::*;
}

pub mod trial {
    pub use super::model::trial::*;
}
