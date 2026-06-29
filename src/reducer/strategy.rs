//! Reducer strategy helpers.
//!
//! This module keeps small policy decisions separate from the engine. Larger
//! adaptive strategies can grow here without coupling language adapters to the
//! oracle loop.

use super::group::{CandidateGroup, CandidateGroupKind};

/// Returns true when a group should prefer single-candidate attempts.
pub fn prefers_single_attempts(group: &CandidateGroup) -> bool {
    matches!(
        group.kind,
        CandidateGroupKind::LiteralShrinkSet | CandidateGroupKind::ControlPathSet
    )
}
