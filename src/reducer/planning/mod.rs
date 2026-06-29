//! Shared candidate planning utilities.
//!
//! Planning modules produce and normalize candidate work from the current
//! snapshot. They do not choose an algorithm-level schedule or execute oracle
//! trials; algorithms consume the planned groups through the runtime context.

pub mod ddmin;
pub mod grouping;
pub mod ordering;
pub mod phase;
