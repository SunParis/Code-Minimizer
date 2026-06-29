//! Language-independent text edit primitives.
//!
//! Language adapters describe reductions as byte-range edits over the current
//! UTF-8 source string. The reducer applies edits only after validating ranges,
//! overlap, and UTF-8 character boundaries so a bad adapter cannot corrupt the
//! current accepted source.

use serde::{Deserialize, Serialize};

use crate::error::CodeMinimizerError;

/// Half-open byte range in a UTF-8 source string.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct TextRange {
    /// Inclusive start byte offset.
    pub start: usize,
    /// Exclusive end byte offset.
    pub end: usize,
}

impl TextRange {
    /// Constructs a range after checking ordering.
    pub fn new(start: usize, end: usize) -> anyhow::Result<Self> {
        if start > end {
            return Err(CodeMinimizerError::InvalidEdit(format!(
                "Range start {start} is greater than end {end}"
            ))
            .into());
        }
        Ok(Self { start, end })
    }

    /// Returns the number of bytes in the range.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Returns true when the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// A single-source edit or a grouped edit that should be tested atomically.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Edit {
    /// Delete the byte range.
    Delete(TextRange),
    /// Replace the byte range with new text.
    Replace {
        /// Range to replace.
        range: TextRange,
        /// Replacement text.
        replacement: String,
    },
    /// Apply multiple non-overlapping edits as one candidate.
    Multi(Vec<Edit>),
}

impl Edit {
    /// Applies the edit to the provided source and returns the modified text.
    pub fn apply(&self, source: &str) -> anyhow::Result<String> {
        let mut replacements = Vec::new();
        collect_replacements(self, &mut replacements);
        apply_replacements(source, &mut replacements)
    }

    /// Returns the total byte length removed by this edit, ignoring replacements.
    pub fn removed_bytes(&self) -> usize {
        match self {
            Self::Delete(range) => range.len(),
            Self::Replace { range, .. } => range.len(),
            Self::Multi(edits) => edits.iter().map(Self::removed_bytes).sum(),
        }
    }
}

/// Internal replacement representation used to sort and validate multi-edits.
#[derive(Clone, Debug)]
struct Replacement {
    range: TextRange,
    replacement: String,
}

/// Recursively flattens an edit into replace operations.
fn collect_replacements(edit: &Edit, replacements: &mut Vec<Replacement>) {
    match edit {
        Edit::Delete(range) => replacements.push(Replacement {
            range: range.clone(),
            replacement: String::new(),
        }),
        Edit::Replace { range, replacement } => replacements.push(Replacement {
            range: range.clone(),
            replacement: replacement.clone(),
        }),
        Edit::Multi(edits) => {
            for edit in edits {
                collect_replacements(edit, replacements);
            }
        }
    }
}

/// Applies sorted, non-overlapping replacements from the end of the source.
fn apply_replacements(source: &str, replacements: &mut [Replacement]) -> anyhow::Result<String> {
    replacements.sort_by_key(|replacement| replacement.range.start);
    validate_replacements(source, replacements)?;

    let mut result = source.to_owned();
    for replacement in replacements.iter().rev() {
        result.replace_range(
            replacement.range.start..replacement.range.end,
            &replacement.replacement,
        );
    }
    Ok(result)
}

/// Ensures every edit range is valid for the current UTF-8 source.
fn validate_replacements(source: &str, replacements: &[Replacement]) -> anyhow::Result<()> {
    let mut previous_end = 0;
    for replacement in replacements {
        let range = &replacement.range;
        if range.end > source.len() {
            return Err(CodeMinimizerError::InvalidEdit(format!(
                "Range {}..{} exceeds source length {}",
                range.start,
                range.end,
                source.len()
            ))
            .into());
        }
        if range.start < previous_end {
            return Err(CodeMinimizerError::InvalidEdit(format!(
                "Overlapping range starts at {} before previous end {}",
                range.start, previous_end
            ))
            .into());
        }
        if !source.is_char_boundary(range.start) || !source.is_char_boundary(range.end) {
            return Err(CodeMinimizerError::InvalidEdit(format!(
                "Range {}..{} does not align to UTF-8 boundaries",
                range.start, range.end
            ))
            .into());
        }
        previous_end = range.end;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_removes_text_range() {
        let edit = Edit::Delete(TextRange::new(1, 4).unwrap());
        assert_eq!(edit.apply("abcdef").unwrap(), "aef");
    }

    #[test]
    fn multi_edit_applies_from_end_without_range_shift() {
        let edit = Edit::Multi(vec![
            Edit::Replace {
                range: TextRange::new(0, 1).unwrap(),
                replacement: "A".into(),
            },
            Edit::Delete(TextRange::new(2, 4).unwrap()),
        ]);
        assert_eq!(edit.apply("abcdef").unwrap(), "Abef");
    }

    #[test]
    fn edit_rejects_overlapping_ranges() {
        let edit = Edit::Multi(vec![
            Edit::Delete(TextRange::new(1, 3).unwrap()),
            Edit::Delete(TextRange::new(2, 4).unwrap()),
        ]);
        assert!(edit.apply("abcdef").is_err());
    }

    #[test]
    fn edit_rejects_invalid_utf8_boundaries() {
        let edit = Edit::Delete(TextRange::new(1, 2).unwrap());
        assert!(edit.apply("é").is_err());
    }
}
