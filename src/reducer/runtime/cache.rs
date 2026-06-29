//! Trial cache keyed by source content and oracle configuration.
//!
//! Reduction frequently revisits equivalent source text through different edit
//! paths. The cache avoids re-running expensive build/run commands for sources
//! that have already been accepted or rejected under the same oracle settings.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Stable SHA-256 cache key for one source and oracle configuration.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CacheKey(String);

impl CacheKey {
    /// Builds a key from source bytes and an oracle configuration fingerprint.
    pub fn new(source: &str, oracle_fingerprint: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(oracle_fingerprint.as_bytes());
        hasher.update(b"\0");
        hasher.update(source.as_bytes());
        Self(to_hex(&hasher.finalize()))
    }

    /// Returns the hexadecimal key text used in reports and tests.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Encodes bytes as lowercase hexadecimal without adding an extra dependency.
fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

/// Cached result for a previously evaluated trial source.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CachedTrial {
    /// The source was accepted by the oracle.
    Accepted,
    /// The source was rejected with a stable reason.
    Rejected(String),
}

/// In-memory trial cache for one reduction session.
#[derive(Clone, Debug, Default)]
pub struct TrialCache {
    entries: HashMap<CacheKey, CachedTrial>,
    hits: usize,
}

impl TrialCache {
    /// Looks up a cached trial and increments hit counters when present.
    pub fn get(&mut self, key: &CacheKey) -> Option<CachedTrial> {
        let value = self.entries.get(key).cloned();
        if value.is_some() {
            self.hits += 1;
        }
        value
    }

    /// Records a trial result.
    pub fn insert(&mut self, key: CacheKey, value: CachedTrial) {
        self.entries.insert(key, value);
    }

    /// Returns the number of cache hits in this session.
    pub fn hits(&self) -> usize {
        self.hits
    }

    /// Returns the number of stored entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when the cache has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_key_changes_with_source_and_fingerprint() {
        let a = CacheKey::new("source", "oracle-a");
        let b = CacheKey::new("source", "oracle-b");
        let c = CacheKey::new("source2", "oracle-a");
        assert_ne!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn trial_cache_counts_hits() {
        let key = CacheKey::new("x", "o");
        let mut cache = TrialCache::default();
        cache.insert(key.clone(), CachedTrial::Accepted);
        assert_eq!(cache.get(&key), Some(CachedTrial::Accepted));
        assert_eq!(cache.hits(), 1);
    }
}
