//! Support code for the VirtualCortex benchmarks (ADR-0014). The benchmarks themselves are in
//! `benches/`; this library holds the deterministic input generator they share so that every
//! run measures the same sequence.

/// A 32-bit linear congruential generator (Numerical Recipes constants). Deterministic, no
/// dependencies, good enough to spread inputs over a range; not a statistical PRNG.
#[derive(Clone, Copy, Debug)]
pub struct Lcg(pub u32);

impl Lcg {
    /// The seed every benchmark starts from, so runs are comparable.
    pub const SEED: u32 = 0x9E37_79B9;

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.0
    }

    /// A value in `lo..hi` (upper bound exclusive), using the high bits.
    #[inline]
    pub fn range(&mut self, lo: u32, hi: u32) -> u32 {
        lo + (self.next_u32() >> 8) % (hi - lo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_is_deterministic_and_in_range() {
        let mut a = Lcg(Lcg::SEED);
        let mut b = Lcg(Lcg::SEED);
        for _ in 0..1000 {
            let x = a.range(1, 2560);
            assert_eq!(x, b.range(1, 2560));
            assert!((1..2560).contains(&x));
        }
    }
}
