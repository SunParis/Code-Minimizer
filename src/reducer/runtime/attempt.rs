//! Result of one reducer edit attempt.
//!
//! Algorithms need more detail than "accepted or rejected": adaptive schedulers
//! distinguish fresh oracle rejections from cached rejections, invalid edits,
//! and candidates that parse but do not simplify the configured objective.

use crate::ir::ProgramSnapshot;

/// Outcome of one edit attempt.
pub enum TrialAttempt {
    /// Oracle accepted a fresh trial and returned a normalized accepted snapshot.
    Accepted(ProgramSnapshot),
    /// Oracle rejected a fresh trial.
    ///
    /// The string is the human-readable oracle reason. Adaptive algorithms use
    /// it to distinguish syntax/build/timeout-like fragility from valid but
    /// uninteresting results.
    Rejected(String),
    /// Cache accepted a previously tested source and returned an accepted snapshot.
    CachedAccepted(ProgramSnapshot),
    /// Cache rejected a previously tested source.
    ///
    /// The cached reason is preserved so adaptive schedulers can make the same
    /// local weight/removal decision they would have made for a fresh rejection.
    CachedRejected(String),
    /// Edit failed before oracle execution.
    ///
    /// This covers invalid ranges, no-op edits, parser construction failures,
    /// and parse diagnostics in the candidate source.
    InvalidEdit,
    /// Candidate parsed but did not satisfy the requested simplification objective.
    NotSimpler,
    /// A shutdown signal interrupted the active or next oracle trial.
    ///
    /// This is not a rejection of the candidate. The engine will stop scheduling
    /// new work and write the last accepted snapshot plus an interrupted report.
    Interrupted,
}
