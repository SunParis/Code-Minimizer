//! Versioned program snapshots.
//!
//! A snapshot is the reducer's durable representation of the current accepted
//! program. After every accepted edit the reducer creates a new snapshot instead
//! of reusing old ranges or ids.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::lang::{LanguageAdapter, ParsedProgram};

use super::{ComplexityScore, ProgramAnalysis, ProgramIndex};

/// Monotonic snapshot identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct SnapshotId(pub u64);

impl SnapshotId {
    /// Root snapshot id used before the engine assigns a real version.
    pub const ROOT: Self = Self(0);

    /// Returns the next snapshot id.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

/// Stable SHA-256 hash of source text.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SourceHash(pub String);

/// Stable hash of the structural index.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct StructureHash(pub String);

/// Complete parsed and indexed source snapshot.
#[derive(Debug)]
pub struct ProgramSnapshot {
    /// Snapshot version.
    pub version: SnapshotId,
    /// Source text.
    pub source: String,
    /// File name used in trial directories.
    pub file_name: String,
    /// Parsed program.
    pub parsed: ParsedProgram,
    /// Language-neutral index.
    pub index: ProgramIndex,
    /// Structural complexity score.
    pub score: ComplexityScore,
    /// Advisory analyses used for stage scheduling and objective checks.
    pub analysis: ProgramAnalysis,
    /// Source hash.
    pub source_hash: SourceHash,
    /// Structural hash.
    pub structure_hash: StructureHash,
}

impl ProgramSnapshot {
    /// Parses, indexes, scores, and hashes source for one version.
    pub fn build(
        version: SnapshotId,
        source: String,
        file_name: &str,
        adapter: &dyn LanguageAdapter,
    ) -> anyhow::Result<Self> {
        let parsed = adapter.parse(&source, file_name)?;
        let index = adapter.build_index(&parsed)?;
        let score = ComplexityScore::compute(&index, &source);
        let analysis = ProgramAnalysis::compute(&index, &source);
        let source_hash = SourceHash(hash_bytes(source.as_bytes()));
        let structure_hash = StructureHash(hash_bytes(format!("{:?}", score).as_bytes()));

        Ok(Self {
            version,
            source,
            file_name: file_name.to_owned(),
            parsed,
            index,
            score,
            analysis,
            source_hash,
            structure_hash,
        })
    }
}

/// Encodes a SHA-256 digest as lowercase hexadecimal.
fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut text = String::with_capacity(digest.len() * 2);
    for byte in digest {
        text.push_str(&format!("{byte:02x}"));
    }
    text
}
