//! Shell command execution utilities.
//!
//! This module group keeps command-template expansion, process spawning,
//! timeout handling, output capture, and shutdown cleanup away from reducer
//! scheduling logic.

pub mod command_template;
pub mod runner;
