//! Result diff helpers used by the oracle and report writer.
//!
//! The reducer does not interpret output semantics. It only tracks whether the
//! configured stdout/stderr result remains different between side A and side B.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{config::DiffMode, runner::InvocationOutcome};

/// Normalizes side-specific trial paths out of captured output before diffing.
///
/// A and B intentionally run in different directories. Many compilers include
/// the input path in diagnostics, so raw output can differ only because one side
/// says `/a/Test.java` while the other says `/b/Test.java`. Those path-only
/// differences are runner artifacts, not program behavior.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OutputNormalizer {
    rules: Vec<OutputNormalizationRule>,
}

impl OutputNormalizer {
    /// Creates an empty normalizer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a side A/B path pair that should compare as the same token.
    pub fn add_path_pair(&mut self, a_path: &Path, b_path: &Path, replacement: &[u8]) {
        self.add_rule(path_bytes(a_path), replacement);
        self.add_rule(path_bytes(b_path), replacement);
        self.rules
            .sort_by(|left, right| right.needle.len().cmp(&left.needle.len()));
    }

    /// Returns a normalized copy of captured output bytes.
    pub fn normalize(&self, bytes: &[u8]) -> Vec<u8> {
        if self.rules.is_empty() || bytes.is_empty() {
            return bytes.to_vec();
        }

        let mut normalized = Vec::with_capacity(bytes.len());
        let mut offset = 0;
        while offset < bytes.len() {
            if let Some(rule) = self
                .rules
                .iter()
                .find(|rule| bytes[offset..].starts_with(&rule.needle))
            {
                normalized.extend_from_slice(&rule.replacement);
                offset += rule.needle.len();
            } else {
                normalized.push(bytes[offset]);
                offset += 1;
            }
        }

        normalized
    }

    /// Adds one byte replacement rule when the needle is meaningful.
    fn add_rule(&mut self, needle: Vec<u8>, replacement: &[u8]) {
        if needle.is_empty() || self.rules.iter().any(|rule| rule.needle == needle) {
            return;
        }

        self.rules.push(OutputNormalizationRule {
            needle,
            replacement: replacement.to_vec(),
        });
    }
}

/// One byte-level output normalization rule.
#[derive(Clone, Debug, Eq, PartialEq)]
struct OutputNormalizationRule {
    needle: Vec<u8>,
    replacement: Vec<u8>,
}

/// Converts a path to the same UTF-8 bytes used by command-template expansion.
fn path_bytes(path: &Path) -> Vec<u8> {
    path.to_string_lossy().as_bytes().to_vec()
}

/// Diff state for observable run results between two invocations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutputDiff {
    /// Whether captured stdout bytes differ.
    pub stdout_differs: bool,
    /// Whether captured stderr bytes differ.
    pub stderr_differs: bool,
    /// Whether run exit-status classes differ. This is report metadata and is
    /// not a valid diff preservation mode.
    pub exit_differs: bool,
}

impl OutputDiff {
    /// Compares two invocation outcomes.
    pub fn compare(a: &InvocationOutcome, b: &InvocationOutcome) -> Self {
        Self::compare_normalized(a, b, &OutputNormalizer::new())
    }

    /// Compares two invocation outcomes after output normalization.
    pub fn compare_normalized(
        a: &InvocationOutcome,
        b: &InvocationOutcome,
        normalizer: &OutputNormalizer,
    ) -> Self {
        Self {
            stdout_differs: normalizer.normalize(&a.run.stdout)
                != normalizer.normalize(&b.run.stdout),
            stderr_differs: normalizer.normalize(&a.run.stderr)
                != normalizer.normalize(&b.run.stderr),
            exit_differs: a.run.status.class() != b.run.status.class(),
        }
    }

    /// Returns whether this diff satisfies the configured preservation mode.
    pub fn satisfies(&self, mode: DiffMode) -> bool {
        match mode {
            DiffMode::AnyChannel => self.stdout_differs || self.stderr_differs,
            DiffMode::Stdout => self.stdout_differs,
            DiffMode::Stderr => self.stderr_differs,
            DiffMode::Both => self.stdout_differs && self.stderr_differs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::{CommandOutcome, ExitStatusSummary, InvocationOutcome};
    use std::time::Duration;

    fn invocation(stdout: &[u8], stderr: &[u8]) -> InvocationOutcome {
        InvocationOutcome {
            build: None,
            run: CommandOutcome {
                status: ExitStatusSummary::Code(0),
                stdout: stdout.to_vec(),
                stderr: stderr.to_vec(),
                timed_out: false,
                duration: Duration::from_millis(1),
                stdout_truncated: false,
                stderr_truncated: false,
            },
        }
    }

    #[test]
    fn compare_reports_stream_differences() {
        let diff = OutputDiff::compare(&invocation(b"a", b"x"), &invocation(b"b", b"x"));
        assert_eq!(
            diff,
            OutputDiff {
                stdout_differs: true,
                stderr_differs: false,
                exit_differs: false
            }
        );
    }

    #[test]
    fn satisfies_honors_diff_mode() {
        let diff = OutputDiff {
            stdout_differs: true,
            stderr_differs: false,
            exit_differs: false,
        };
        assert!(diff.satisfies(DiffMode::AnyChannel));
        assert!(diff.satisfies(DiffMode::Stdout));
        assert!(!diff.satisfies(DiffMode::Stderr));
        assert!(!diff.satisfies(DiffMode::Both));
    }

    #[test]
    fn exit_status_difference_is_report_metadata_only() {
        let diff = OutputDiff {
            stdout_differs: false,
            stderr_differs: false,
            exit_differs: true,
        };

        assert!(!diff.satisfies(DiffMode::AnyChannel));
        assert!(!diff.satisfies(DiffMode::Stdout));
        assert!(!diff.satisfies(DiffMode::Stderr));
        assert!(!diff.satisfies(DiffMode::Both));
    }

    #[test]
    fn compare_normalized_ignores_side_specific_trial_paths() {
        let a = invocation(
            b"",
            b"/tmp/session/trials/current/a/Test.java:4: error: missing return statement\n",
        );
        let b = invocation(
            b"",
            b"/tmp/session/trials/current/b/Test.java:4: error: missing return statement\n",
        );
        let mut normalizer = OutputNormalizer::new();
        normalizer.add_path_pair(
            Path::new("/tmp/session/trials/current/a/Test.java"),
            Path::new("/tmp/session/trials/current/b/Test.java"),
            b"{input}",
        );

        let diff = OutputDiff::compare_normalized(&a, &b, &normalizer);

        assert!(
            !diff.stderr_differs,
            "Side-specific trial input paths should not create an output diff"
        );
    }
}
