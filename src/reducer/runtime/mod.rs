//! Shared reducer runtime services.
//!
//! Concrete algorithms should depend on this layer through
//! [`context::ReductionContext`] and [`engine::ReducerEngine`]. It owns mutable
//! session state, oracle trial execution, report writing, cache state, and
//! validation. No scheduling algorithm should keep private copies of this logic.

pub mod attempt;
pub mod cache;
pub mod context;
pub mod engine;
pub mod objective;
pub mod reporting;
pub mod state;
pub mod trial_runner;
pub mod validation;
