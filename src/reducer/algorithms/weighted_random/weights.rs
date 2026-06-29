//! Weighted sampling and neighborhood update rules.
//!
//! The random scheduler stores a mutable weight per point. Successful deletions
//! increase nearby weights; valid oracle rejections decrease nearby weights;
//! syntax-like failures remove the point before this module is called.

use super::{points::Point, rng::SmallRng};

/// Returns a stable patience bound based on the remaining point count.
pub(super) fn patience_limit(points: usize) -> usize {
    if points == 0 {
        return 0;
    }

    // The formula gives a small search budget for tiny point sets and grows
    // sublinearly for large generated programs. It is capped at half the point
    // set so a long unproductive run exits instead of exhaustively testing
    // random singles.
    let sqrt_scaled = (points as f64).sqrt().ceil() as usize * 3;
    sqrt_scaled.min((points / 2).max(1)).max(1)
}

/// Selects a point index proportionally to current weights.
pub(super) fn weighted_index(points: &[Point], rng: &mut SmallRng) -> usize {
    // Sum weights in f64 because updates are multiplicative and decayed. The
    // values are clamped elsewhere, so precision loss is not meaningful here.
    let total = points.iter().map(|point| point.weight).sum::<f64>();
    if total <= f64::EPSILON {
        // This should be rare because weights are clamped above zero, but the
        // fallback keeps selection defined even if future changes allow zeros.
        return rng.next_usize(points.len());
    }

    // Roulette-wheel selection: draw a number inside the total weight and walk
    // the cumulative distribution until the bucket is found.
    let mut target = rng.next_f64() * total;
    for (index, point) in points.iter().enumerate() {
        if target <= point.weight {
            return index;
        }
        target -= point.weight;
    }
    points.len().saturating_sub(1)
}

/// Removes a point that should no longer be tested in this snapshot.
pub(super) fn remove_point(points: &mut Vec<Point>, index: usize) {
    // `swap_remove` is O(1), then sorting restores source order. Keeping source
    // order is not required for random sampling, but it makes logs/debugging and
    // future deterministic tests easier to reason about.
    points.swap_remove(index);
    points.sort_by_key(|point| point.center);
}

/// Increases nearby weights after a successful deletion.
pub(super) fn increase_neighbor_weights(points: &mut [Point], center: usize) {
    // Successful deletions make nearby points more likely, with a hard upper
    // bound so a single region cannot dominate the entire search forever.
    adjust_neighbor_weights(points, center, 1.0, |weight, delta| {
        (weight + delta).min(12.0)
    });
}

/// Decreases nearby weights after an oracle rejection.
pub(super) fn decrease_neighbor_weights(points: &mut [Point], center: usize) {
    // Valid-but-uninteresting deletions reduce local probability but never to
    // zero. A later accepted nearby edit may make the region interesting again.
    adjust_neighbor_weights(points, center, 0.45, |weight, delta| {
        (weight * (1.0 - delta)).max(0.05)
    });
}

/// Applies a distance-decayed weight update around a byte position.
fn adjust_neighbor_weights(
    points: &mut [Point],
    center: usize,
    base_delta: f64,
    apply: impl Fn(f64, f64) -> f64,
) {
    for point in points {
        // Distance is measured in source bytes. This is intentionally simple:
        // byte proximity is a good enough locality proxy for generated single
        // files and does not require language-specific line maps.
        let distance = point.center.abs_diff(center);
        let decay = 1.0 / (1.0 + distance as f64 / 120.0);
        point.weight = apply(point.weight, base_delta * decay);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        edit::{Edit, TextRange},
        reducer::candidate::{Candidate, StageKind, TransformKind},
    };

    use super::*;

    #[test]
    fn patience_scales_with_point_count() {
        assert_eq!(patience_limit(0), 0);
        assert_eq!(patience_limit(1), 1);
        assert!(patience_limit(100) <= 50);
    }

    #[test]
    fn weighted_index_prefers_larger_weights() {
        let mut rng = SmallRng::seed_from_snapshot(b"seed", 1);
        let points = vec![
            Point {
                candidate: dummy_candidate("a", 0, 1),
                center: 0,
                weight: 0.01,
            },
            Point {
                candidate: dummy_candidate("b", 2, 3),
                center: 2,
                weight: 100.0,
            },
        ];
        let hits = (0..20)
            .filter(|_| weighted_index(&points, &mut rng) == 1)
            .count();
        assert!(hits >= 18);
    }

    fn dummy_candidate(id: &str, start: usize, end: usize) -> Candidate {
        Candidate::new(
            StageKind::StatementAndSiblingReduction,
            TransformKind::DeleteStatementChunk,
            id,
            "Delete statement",
            Edit::Delete(TextRange { start, end }),
            1,
            -1,
        )
    }
}
