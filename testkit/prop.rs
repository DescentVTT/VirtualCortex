// The property-test kit (ADR-0030). Zero dependencies: a crate `include!`s this file into a
// `#[cfg(test)]` module, so the state crates stay free of dependencies of any kind (TC-2) and
// share one generator and one lattice. Everything here is integer arithmetic that saturates or
// wraps by name, so it compiles under every workspace lint, including
// `clippy::arithmetic_side_effects`.
//
// A property test here has two halves: the lattice, the values a rule must survive (the
// extremes, the powers of two around 1.0 in Q16.16, the thresholds' neighbours), enumerated
// exhaustively in pairs; and a seeded pseudo-random walk, which reaches the interior. Both are
// deterministic: the same seed produces the same walk on every target, so a failure is a
// reproducible input, not a flake.

/// Knuth's MMIX linear congruential generator: 64-bit state, the high 32 bits as output.
/// Full period for every seed.
#[allow(dead_code)]
pub struct Lcg(u64);

#[allow(dead_code)]
impl Lcg {
    pub const fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next_u64().wrapping_shr(32) as u32
    }

    pub fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }

    pub fn next_u16(&mut self) -> u16 {
        self.next_u32() as u16
    }

    pub fn next_i16(&mut self) -> i16 {
        self.next_u32() as i16
    }

    pub fn next_u8(&mut self) -> u8 {
        self.next_u32() as u8
    }

    pub fn next_i128(&mut self) -> i128 {
        let hi = self.next_u64() as u128;
        let lo = self.next_u64() as u128;
        (hi.wrapping_shl(64) | lo) as i128
    }

    /// A value in `0..n`; 0 for `n == 0`.
    pub fn below(&mut self, n: u32) -> u32 {
        (self.next_u32() as u64)
            .wrapping_mul(n as u64)
            .wrapping_shr(32) as u32
    }

    /// One of `set`, uniformly.
    pub fn pick<T: Copy>(&mut self, set: &[T]) -> T {
        set[self.below(set.len() as u32) as usize]
    }

    /// A lattice value three times in four, a uniform value otherwise: the edges and the
    /// interior in one walk.
    pub fn i32_edge_biased(&mut self) -> i32 {
        if self.below(4) == 0 {
            self.next_i32()
        } else {
            self.pick(&I32_LATTICE)
        }
    }

    pub fn u32_edge_biased(&mut self) -> u32 {
        if self.below(4) == 0 {
            self.next_u32()
        } else {
            self.pick(&U32_LATTICE)
        }
    }

    pub fn i128_edge_biased(&mut self) -> i128 {
        if self.below(4) == 0 {
            self.next_i128()
        } else {
            self.pick(&I128_LATTICE)
        }
    }
}

/// FNV-1a over the little-endian bytes of `samples`, for a golden test that pins a rendered
/// sequence in one number (a moved pin is a changed rule; the change says why).
#[allow(dead_code)]
pub fn fnv1a_64(samples: &[i32]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for s in samples {
        for byte in s.to_le_bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

/// The `i32` values a Q16.16 rule must survive: the extremes and their neighbours, zero and
/// its neighbours, and the powers of two around 1.0.
#[allow(dead_code)]
pub const I32_LATTICE: [i32; 15] = [
    i32::MIN,
    i32::MIN + 1,
    -0x0002_0000,
    -0x0001_0000,
    -0x8000,
    -0x4000,
    -1,
    0,
    1,
    0x4000,
    0x8000,
    0x0001_0000,
    0x0002_0000,
    i32::MAX - 1,
    i32::MAX,
];

/// The `u32` values a tick count or a delay must survive: zero, the wheel's ring lengths and
/// horizon, the sixteen-bit boundary and the extremes.
#[allow(dead_code)]
pub const U32_LATTICE: [u32; 11] = [
    0,
    1,
    2,
    255,
    256,
    2_559,
    2_560,
    65_535,
    65_536,
    u32::MAX - 1,
    u32::MAX,
];

/// The Q1.15 values a weight must survive.
#[allow(dead_code)]
pub const I16_LATTICE: [i16; 7] = [i16::MIN, i16::MIN + 1, -1, 0, 1, i16::MAX - 1, i16::MAX];

/// The Q0.8 values a plasticity factor must survive.
#[allow(dead_code)]
pub const U8_LATTICE: [u8; 6] = [0, 1, 51, 127, 128, 255];

/// The `i128` values an exact scratchpad must survive.
#[allow(dead_code)]
pub const I128_LATTICE: [i128; 13] = [
    i128::MIN,
    i128::MIN + 1,
    -(1 << 64),
    -(1 << 16),
    -1,
    0,
    1,
    1 << 16,
    1 << 64,
    (1 << 64) - 1,
    i128::MAX >> 16,
    i128::MAX - 1,
    i128::MAX,
];
