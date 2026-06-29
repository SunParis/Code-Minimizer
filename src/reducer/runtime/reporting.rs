//! Reducer-local report and log formatting helpers.
//!
//! JSON report types live in `app::report`; this module only owns the reducer
//! mechanics for writing a report file and compactly formatting complexity
//! scores in progress logs.

use std::{fs, path::PathBuf};

use anyhow::Context;

use crate::{ir::ComplexityScore, report::ReductionReport};

/// Returns a compact score summary for logs.
pub(super) fn score_summary(score: &ComplexityScore) -> String {
    format!(
        "nodes={},stmts={},outputs={},bytes={}",
        score.ast_nodes, score.statements, score.output_operations, score.source_bytes
    )
}

/// Writes a pretty JSON report to disk.
pub(super) fn write_json_report(path: PathBuf, report: &ReductionReport) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(report)?;
    fs::write(&path, json)
        .with_context(|| format!("Failed to write JSON reduction report '{}'", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_summary_mentions_outputs_and_bytes() {
        let score = ComplexityScore {
            ast_nodes: 3,
            statements: 2,
            output_operations: 1,
            source_bytes: 12,
            ..ComplexityScore::default()
        };
        assert_eq!(score_summary(&score), "nodes=3,stmts=2,outputs=1,bytes=12");
    }
}
