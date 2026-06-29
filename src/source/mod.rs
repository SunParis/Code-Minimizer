//! Language-agnostic source text utilities.
//!
//! Reducer candidates eventually become byte-range edits and oracle comparisons
//! eventually become output diffs. Those operations are independent of any
//! particular programming language, so they live together here.

pub mod edit;
pub mod output_diff;
