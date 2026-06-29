//! Small deterministic pseudo-random generator.
//!
//! The reducer only needs a stable pseudo-random schedule, not cryptographic
//! randomness. Keeping this implementation local avoids adding a dependency and
//! makes weighted-random runs reproducible from the accepted source hash.

/// Small deterministic PRNG for weighted scheduling.
pub(super) struct SmallRng {
    /// Internal xorshift state. Zero is avoided during seeding.
    state: u64,
}

impl SmallRng {
    /// Seeds the generator from stable bytes and a round salt.
    pub(super) fn seed_from_snapshot(bytes: &[u8], salt: u64) -> Self {
        // FNV-1a style mixing is enough here because the PRNG is for scheduling,
        // not security. Including the round prevents identical snapshots in
        // different rounds from replaying the same point sequence.
        let mut state = 0xcbf2_9ce4_8422_2325_u64 ^ salt;
        for byte in bytes {
            state ^= u64::from(*byte);
            state = state.wrapping_mul(0x0000_0100_0000_01b3);
        }
        if state == 0 {
            state = 0x9e37_79b9_7f4a_7c15;
        }
        Self { state }
    }

    /// Returns the next pseudo-random u64.
    fn next_u64(&mut self) -> u64 {
        // Xorshift64 is tiny, deterministic, and dependency-free. The reducer
        // only needs a stable pseudo-random order for trial scheduling.
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Returns a number in `[0, 1)`.
    pub(super) fn next_f64(&mut self) -> f64 {
        // Use the high 53 bits so the integer fits exactly in an IEEE-754 f64
        // mantissa before scaling.
        const SCALE: f64 = 1.0 / ((1_u64 << 53) as f64);
        ((self.next_u64() >> 11) as f64) * SCALE
    }

    /// Returns an index below the provided bound.
    pub(super) fn next_usize(&mut self, bound: usize) -> usize {
        // Modulo bias is acceptable for this scheduler because weights dominate
        // selection and the fallback is only used for degenerate zero totals.
        if bound <= 1 {
            return 0;
        }
        (self.next_u64() as usize) % bound
    }
}
