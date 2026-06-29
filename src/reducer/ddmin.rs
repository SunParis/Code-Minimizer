//! Chunking helpers for delta-debugging style stage attempts.
//!
//! The engine uses these chunks to try deleting or replacing groups before
//! falling back to individual candidates.

/// Splits `len` items into at most `parts` contiguous chunks.
pub fn chunk_ranges(len: usize, parts: usize) -> Vec<std::ops::Range<usize>> {
    if len == 0 || parts == 0 {
        return Vec::new();
    }

    let parts = parts.min(len);
    let mut ranges = Vec::with_capacity(parts);
    let mut start = 0;

    for part in 0..parts {
        let remaining_items = len - start;
        let remaining_parts = parts - part;
        let chunk_len = remaining_items.div_ceil(remaining_parts);
        let end = start + chunk_len;
        ranges.push(start..end);
        start = end;
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_ranges_cover_all_items() {
        let ranges = chunk_ranges(5, 2);
        assert_eq!(ranges, vec![0..3, 3..5]);
    }

    #[test]
    fn chunk_ranges_do_not_exceed_length() {
        let ranges = chunk_ranges(3, 10);
        assert_eq!(ranges, vec![0..1, 1..2, 2..3]);
    }
}
