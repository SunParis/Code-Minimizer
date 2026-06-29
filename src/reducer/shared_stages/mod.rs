//! Algorithm-independent finishing stages.
//!
//! These stages run outside a concrete scheduling algorithm but still use the
//! same reducer context, parser checks, oracle validation, rollback, and report
//! machinery.

pub mod blank_lines;
