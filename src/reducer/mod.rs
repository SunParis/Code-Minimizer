//! Language-independent reduction engine.
//!
//! Reducer modules operate on snapshot-local candidate groups produced by a
//! language adapter. They never inspect JavaScript or Java syntax directly.

pub mod algorithms;
pub mod attempt;
pub mod blank_lines;
pub mod cache;
pub mod candidate;
pub mod context;
pub mod ddmin;
pub mod edit_plan;
pub mod engine;
#[cfg(test)]
mod engine_tests;
pub mod final_sweep;
pub mod group;
pub mod grouping;
pub mod objective;
pub mod ordering;
pub mod phase;
pub mod reporting;
pub mod state;
pub mod strategy;
pub mod trial;
pub mod trial_runner;
pub mod validation;
