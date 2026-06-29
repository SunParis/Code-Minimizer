//! Stage order helpers for the structured algorithm.
//!
//! The final sweep reuses normal stage candidate generation but forces
//! single-candidate attempts. This is the guardrail that catches removable
//! leftovers such as an otherwise irrelevant `System.out.println`.

use crate::reducer::candidate::StageKind;

/// Returns the stages that should feed the final one-minimal sweep.
pub(super) fn final_sweep_stages() -> &'static [StageKind] {
    StageKind::final_sweep_sources()
}
