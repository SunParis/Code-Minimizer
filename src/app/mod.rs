//! Application-facing API surface.
//!
//! These modules translate user intent into immutable reducer inputs and expose
//! the stable data structures consumed by CLIs, tests, and automation.

pub mod cli;
pub mod config;
pub mod error;
pub mod logging;
pub mod report;
