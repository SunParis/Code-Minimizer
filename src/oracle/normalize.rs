//! Output normalization for side-specific trial paths.
//!
//! A and B run in different directories, so commands may print paths that are
//! different only because of the trial layout. These rules replace those paths
//! with shared placeholders before stdout/stderr comparison.

use crate::{config::TrialSide, output_diff::OutputNormalizer, workspace::TrialLayout};

/// Builds output normalization rules for paths that differ only by trial side.
pub(super) fn output_normalizer_for_layout(
    layout: &TrialLayout,
) -> anyhow::Result<OutputNormalizer> {
    let a = layout.side(TrialSide::A)?;
    let b = layout.side(TrialSide::B)?;
    let mut normalizer = OutputNormalizer::new();
    normalizer.add_path_pair(&a.input_path, &b.input_path, b"{input}");
    normalizer.add_path_pair(&a.dir, &b.dir, b"{dir}");
    normalizer.add_path_pair(&a.output_dir, &b.output_dir, b"{output}");
    Ok(normalizer)
}
