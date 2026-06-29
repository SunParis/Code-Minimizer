//! Validation helpers shared by the engine and algorithm context.
//!
//! Parsing diagnostics are checked before oracle execution so broken syntax does
//! not waste command executions or pollute interestingness results.

use crate::{error::CodeMinimizerError, ir::ProgramSnapshot};

/// Rejects snapshots with parse diagnostics.
pub(super) fn reject_invalid_snapshot(
    snapshot: &ProgramSnapshot,
    context: &str,
) -> anyhow::Result<()> {
    if !snapshot.parsed.diagnostics.is_empty() {
        return Err(CodeMinimizerError::ParseFailed(format!(
            "{context}: source contains {} parse diagnostics",
            snapshot.parsed.diagnostics.len()
        ))
        .into());
    }
    Ok(())
}
