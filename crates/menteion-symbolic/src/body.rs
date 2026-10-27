//! The hypervector body and the vector-symbolic algebra over it (whitepaper §5.2.9, §8.8;
//! ADR-0039; brief 020). A body is 160 words of 64 bits, 10 240 bits, twenty cache lines: a
//! dense binary hypervector in the binary spatter code of Kanerva. Binding is XOR, which is
//! its own inverse, so unbinding a role from a sealed vector is binding it again; permutation
//! is a cyclic rotation of the bits; bundling is the per-bit majority of up to fifteen bodies,
//! an even count made odd by a fixed tie-breaker; the distance between two bodies is the
//! population count of their XOR; clean-up is the nearest entry of a codebook, the lowest
//! index on a tie; a decode confidence is one minus twice the distance as a fraction of the
//! width, floored at zero. Every rule is a loop over the words with named operations: no
//! intrinsic, no `unsafe` (forbidden in this crate), no allocation; the compiler vectorises
//! the loops for the target it builds for, and the result is the same integer on every target.

/// Words of 64 bits in a body: twenty cache lines.
pub const BODY_WORDS: usize = 160;
/// Bits in a body: the hypervector's width, `BODY_WORDS × 64`.
pub const BODY_BITS: u32 = 10_240;
/// The most bodies one bundle takes: fifteen, so that an even count plus the tie-breaker
/// still fits the four-bit counter the majority is taken over.
pub const BUNDLE_MAX: usize = 15;
/// The seed of the body that breaks ties: an even bundle adds it as one more operand.
pub const TIE_SEED: u64 = 0x5EED_7A1E_B4EA_4E4D;
/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;

/// A 10 240-bit hypervector body: 1 280 bytes, twenty cache lines (whitepaper §5.2.9,
/// ADR-0039). The zero body is the identity of binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct HypervectorBody {
    pub words: [u64; BODY_WORDS], // [0..1280) The bits, word w holding bits 64w to 64w + 63, least significant first
}

// Arrays longer than 32 elements do not implement Default, so the zero body is spelled out.
impl Default for HypervectorBody {
    fn default() -> Self {
        Self::ZERO
    }
}

impl HypervectorBody {
    /// The zero body: every bit clear; `x.bind(&ZERO) == x`.
    pub const ZERO: Self = Self {
        words: [0; BODY_WORDS],
    };

    /// A dense pseudo-random body from a seed: the 160 outputs of splitmix64 (Steele, Lea and
    /// Flood 2014) started at `seed`, the same on every target. Two seeds give bodies about
    /// half the width apart, which is what a codebook needs of its entries.
    pub fn from_seed(seed: u64) -> Self {
        let mut state = seed;
        let mut words = [0u64; BODY_WORDS];
        for word in words.iter_mut() {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            *word = z ^ (z >> 31);
        }
        Self { words }
    }

    /// Binding: the XOR of the two bodies, bit for bit. Its own inverse, so the concept a role
    /// was bound with is recovered by binding the sealed vector with the role again,
    /// $\text{Concept} \approx S \otimes \text{Role}^{-1} = S \otimes \text{Role}$.
    pub fn bind(&self, other: &Self) -> Self {
        let mut words = [0u64; BODY_WORDS];
        for (out, (a, b)) in words.iter_mut().zip(self.words.iter().zip(&other.words)) {
            *out = a ^ b;
        }
        Self { words }
    }

    /// Permutation: the bits rotated toward the high end by `shift` modulo `BODY_BITS`, the
    /// cyclic permutation $\Pi^k$ that `SymbolicHypervectorHeader::permutation_shift` names.
    /// A shift of zero, or of the width, is the identity; `permute(k)` then
    /// `permute(BODY_BITS - k)` is the identity.
    pub fn permute(&self, shift: u32) -> Self {
        // Below `BODY_BITS` after the remainder: at most 159 words and 63 bits.
        let shift = shift.wrapping_rem(BODY_BITS);
        let word_shift = (shift / 64) as usize;
        let bit_shift = shift % 64;
        let mut words = [0u64; BODY_WORDS];
        for (w, out) in words.iter_mut().enumerate() {
            // The source word is `word_shift` words below, wrapping; both are below
            // `BODY_WORDS`, so the sum wraps within twice the length.
            let src = w.wrapping_add(BODY_WORDS).wrapping_sub(word_shift) % BODY_WORDS;
            let prev = src.wrapping_add(BODY_WORDS).wrapping_sub(1) % BODY_WORDS;
            *out = if bit_shift == 0 {
                self.words[src]
            } else {
                // `bit_shift` is 1 to 63, so both shifts are below the width.
                (self.words[src] << bit_shift) | (self.words[prev] >> 64u32.wrapping_sub(bit_shift))
            };
        }
        Self { words }
    }

    /// Bundling: the per-bit majority of `items`. An even count adds the tie-breaker body
    /// (`from_seed(TIE_SEED)`) as one more operand so that every bit has a majority; `None` for
    /// no item or more than `BUNDLE_MAX`. A bundle of one is that body. For independent items
    /// each is expected to agree with the bundle on more than half the bits, which is what lets
    /// a bound role be read back; it is a property of the items, not of the rule (a body
    /// bundled with two copies of its complement is its complement).
    pub fn bundle(items: &[Self]) -> Option<Self> {
        if items.is_empty() || items.len() > BUNDLE_MAX {
            return None;
        }
        // Four bit-planes count each bit's ones, 0 to 15, as carry-save adders over the
        // words: adding `x` to the counter is a ripple of XOR and AND through the planes.
        let mut planes = [[0u64; BODY_WORDS]; 4];
        let mut count = 0u32;
        // The tie-breaker is generated only when an even count needs it.
        let tie = (items.len() % 2 == 0).then(|| Self::from_seed(TIE_SEED));
        let operands = items.iter().chain(tie.as_ref());
        for item in operands {
            count = count.wrapping_add(1);
            for w in 0..BODY_WORDS {
                let mut carry = item.words[w];
                for plane in planes.iter_mut() {
                    let sum = plane[w] ^ carry;
                    carry &= plane[w];
                    plane[w] = sum;
                }
            }
        }
        // The majority: a count at or above `(operands + 1) / 2`, compared bit-sliced from
        // the top plane down: `ge` holds where the count is already above the threshold and
        // `eq` where it still equals the threshold's prefix.
        let threshold = count.wrapping_add(1) / 2;
        let mut words = [0u64; BODY_WORDS];
        for (w, out) in words.iter_mut().enumerate() {
            let mut ge = 0u64;
            let mut eq = u64::MAX;
            for p in (0..4).rev() {
                let plane = planes[p][w];
                if threshold & (1 << p) != 0 {
                    eq &= plane;
                } else {
                    ge |= eq & plane;
                    eq &= !plane;
                }
            }
            *out = ge | eq;
        }
        Some(Self { words })
    }

    /// The Hamming distance: the bits on which the two bodies differ, 0 to `BODY_BITS`.
    pub fn hamming(&self, other: &Self) -> u32 {
        let mut distance = 0u32;
        for (a, b) in self.words.iter().zip(&other.words) {
            // At most 64 per word and 160 words: below 2^14.
            distance = distance.wrapping_add((a ^ b).count_ones());
        }
        distance
    }

    /// Clean-up: the index and distance of the nearest entry of `book`, the lowest index on a
    /// tie; `None` for an empty book.
    pub fn nearest(&self, book: &[Self]) -> Option<(usize, u32)> {
        let mut best: Option<(usize, u32)> = None;
        for (index, entry) in book.iter().enumerate() {
            let distance = self.hamming(entry);
            if best.is_none_or(|(_, d)| distance < d) {
                best = Some((index, distance));
            }
        }
        best
    }
}

/// The confidence of a readout at `distance` from its nearest codebook entry, in Q16.16:
/// $1 - 2d / \text{BODY\_BITS}$ floored at zero, so 1.0 at an exact match, 0.5 at a quarter of
/// the width (what a bundle of three leaves), 0 at half (an unrelated body) and beyond.
pub const fn confidence_q16(distance: u32) -> u32 {
    const HALF: u32 = BODY_BITS / 2;
    let margin = HALF.saturating_sub(distance);
    // At most `HALF × 2^16`, below 2^30: no widening needed, and the division is by a
    // non-zero constant.
    margin.wrapping_mul(Q16_ONE) / HALF
}

const _: () = {
    assert!(core::mem::size_of::<HypervectorBody>() == 1280);
    assert!(core::mem::size_of::<HypervectorBody>() == 64 * 20);
    assert!(core::mem::align_of::<HypervectorBody>() == 64);
    assert!(BODY_BITS as usize == BODY_WORDS * 64);
    assert!(BUNDLE_MAX < 16);
};

#[cfg(test)]
mod tests {
    use super::*;

    /// The bit `i` of a body.
    fn bit(body: &HypervectorBody, i: u32) -> bool {
        body.words[(i / 64) as usize] & (1 << (i % 64)) != 0
    }

    /// The per-bit oracle of the majority: counts ones bit by bit.
    fn majority_oracle(items: &[HypervectorBody]) -> HypervectorBody {
        let tie = HypervectorBody::from_seed(TIE_SEED);
        let mut out = HypervectorBody::ZERO;
        let n = items.len().wrapping_add(usize::from(items.len() % 2 == 0));
        for i in 0..BODY_BITS {
            let mut ones = items.iter().filter(|b| bit(b, i)).count();
            if items.len() % 2 == 0 && bit(&tie, i) {
                ones = ones.wrapping_add(1);
            }
            if ones.wrapping_mul(2) > n {
                out.words[(i / 64) as usize] |= 1 << (i % 64);
            }
        }
        out
    }

    #[test]
    fn the_body_is_twenty_cache_lines_and_the_seeded_words_are_splitmix64() {
        assert_eq!(core::mem::size_of::<HypervectorBody>(), 1280);
        assert_eq!(core::mem::align_of::<HypervectorBody>(), 64);
        assert_eq!(BODY_BITS, 10_240);
        let one = HypervectorBody::from_seed(1);
        assert_eq!(
            one.words[0], 0x910a_2dec_8902_5cc1,
            "splitmix64(1), first output"
        );
        assert_eq!(one.words[1], 0xbeeb_8da1_658e_ec67);
        assert_eq!(one.words[159], 0xef6c_8982_a607_2624);
        assert_eq!(one.hamming(&HypervectorBody::ZERO), 5133, "its weight");
        let zero = HypervectorBody::from_seed(0);
        assert_eq!(
            zero.words[0], 0xe220_a839_7b1d_cdaf,
            "a zero seed is a body too"
        );
        assert_eq!(HypervectorBody::default(), HypervectorBody::ZERO);
        assert_eq!(
            one.hamming(&HypervectorBody::from_seed(2)),
            5105,
            "two seeds are about half the width apart"
        );
    }

    #[test]
    fn binding_is_an_involution_with_the_zero_body_as_its_identity() {
        let seeds = [0u64, 1, 2, u64::MAX, u64::MAX - 1, 1 << 63, 0x8000_0000, 7];
        for &s in &seeds {
            let a = HypervectorBody::from_seed(s);
            for &t in &seeds {
                let b = HypervectorBody::from_seed(t);
                assert_eq!(a.bind(&b).bind(&b), a, "unbinding is binding again");
                assert_eq!(a.bind(&b), b.bind(&a), "commutative");
            }
            assert_eq!(a.bind(&HypervectorBody::ZERO), a);
            assert_eq!(
                a.bind(&a),
                HypervectorBody::ZERO,
                "a body bound with itself"
            );
        }
    }

    #[test]
    fn permutation_rotates_every_bit_by_the_shift_and_composes_to_the_identity() {
        let a = HypervectorBody::from_seed(9);
        assert_eq!(a.permute(0), a);
        assert_eq!(a.permute(BODY_BITS), a, "the width is the identity");
        assert_eq!(a.permute(BODY_BITS.wrapping_mul(3)), a);
        let by_one = a.permute(1);
        for i in 0..BODY_BITS {
            let from = i.wrapping_add(BODY_BITS).wrapping_sub(1) % BODY_BITS;
            assert_eq!(bit(&by_one, i), bit(&a, from), "bit {i} came from {from}");
        }
        for k in [1u32, 63, 64, 65, 127, 128, 5_000, 10_239] {
            let there = a.permute(k);
            assert_eq!(
                there.permute(BODY_BITS.wrapping_sub(k)),
                a,
                "k = {k} and back"
            );
            assert_eq!(there.permute(1), a.permute(k.wrapping_add(1)), "one more");
            for i in 0..BODY_BITS {
                let from = i.wrapping_add(BODY_BITS).wrapping_sub(k) % BODY_BITS;
                assert_eq!(bit(&there, i), bit(&a, from), "k = {k}, bit {i}");
            }
        }
        assert_eq!(a.permute(64).words[1], a.words[0], "a whole word moves up");
        assert_eq!(
            a.permute(64).words[0],
            a.words[159],
            "and the top word wraps"
        );
    }

    #[test]
    fn the_distance_counts_differing_bits_and_is_a_metric() {
        let a = HypervectorBody::from_seed(3);
        let b = HypervectorBody::from_seed(4);
        let c = HypervectorBody::from_seed(5);
        assert_eq!(a.hamming(&a), 0);
        assert_eq!(a.hamming(&b), b.hamming(&a), "symmetric");
        let oracle = (0..BODY_BITS).filter(|&i| bit(&a, i) != bit(&b, i)).count() as u32;
        assert_eq!(a.hamming(&b), oracle);
        assert!(
            a.hamming(&b) <= a.hamming(&c).wrapping_add(c.hamming(&b)),
            "the triangle"
        );
        let mut complement = a;
        for w in complement.words.iter_mut() {
            *w = !*w;
        }
        assert_eq!(a.hamming(&complement), BODY_BITS, "every bit differs");
        assert_eq!(
            HypervectorBody::ZERO.hamming(&complement),
            BODY_BITS.wrapping_sub(a.hamming(&HypervectorBody::ZERO))
        );
        let mut one_bit = HypervectorBody::ZERO;
        one_bit.words[159] = 1 << 63;
        assert_eq!(
            one_bit.hamming(&HypervectorBody::ZERO),
            1,
            "the last bit counts"
        );
    }

    #[test]
    fn a_bundle_is_the_per_bit_majority_and_an_even_count_is_broken_by_the_fixed_body() {
        let items: [HypervectorBody; 16] =
            core::array::from_fn(|i| HypervectorBody::from_seed(100u64.wrapping_add(i as u64)));
        assert_eq!(HypervectorBody::bundle(&[]), None, "nothing to bundle");
        assert_eq!(
            HypervectorBody::bundle(&items),
            None,
            "sixteen is one too many"
        );
        for n in 1..=BUNDLE_MAX {
            let bundle = HypervectorBody::bundle(&items[..n]).unwrap();
            assert_eq!(bundle, majority_oracle(&items[..n]), "n = {n}");
            for item in &items[..n] {
                assert!(
                    item.hamming(&bundle) < BODY_BITS / 2,
                    "n = {n}: every item is closer than chance to the bundle"
                );
            }
        }
        assert_eq!(
            HypervectorBody::bundle(&items[..1]).unwrap(),
            items[0],
            "one is itself"
        );
        let two = HypervectorBody::bundle(&items[..2]).unwrap();
        let tie = HypervectorBody::from_seed(TIE_SEED);
        for i in 0..BODY_BITS {
            let (a, b) = (bit(&items[0], i), bit(&items[1], i));
            let expected = if a == b { a } else { bit(&tie, i) };
            assert_eq!(
                bit(&two, i),
                expected,
                "bit {i}: agreement wins, else the tie-breaker"
            );
        }
        let three = HypervectorBody::bundle(&items[..3]).unwrap();
        for i in 0..BODY_BITS {
            let ones = [0, 1, 2].iter().filter(|&&k| bit(&items[k], i)).count();
            assert_eq!(bit(&three, i), ones >= 2, "bit {i}: two of three");
        }
        // The agreement of an item with the bundle is the items' property, not the rule's: a
        // body with two copies of its complement bundles to its complement.
        let mut complement = items[0];
        for w in complement.words.iter_mut() {
            *w = !*w;
        }
        let outvoted = HypervectorBody::bundle(&[items[0], complement, complement]).unwrap();
        assert_eq!(outvoted, complement);
        assert_eq!(items[0].hamming(&outvoted), BODY_BITS, "every bit lost");
        // The counter's top: fifteen copies of one body is that body, and fifteen distinct
        // bodies still take a majority of eight.
        let same = [items[7]; 15];
        assert_eq!(HypervectorBody::bundle(&same).unwrap(), items[7]);
        let fifteen = HypervectorBody::bundle(&items[..15]).unwrap();
        for i in 0..BODY_BITS.min(2_048) {
            let ones = items[..15].iter().filter(|b| bit(b, i)).count();
            assert_eq!(bit(&fifteen, i), ones >= 8, "bit {i}: eight of fifteen");
        }
    }

    #[test]
    fn the_nearest_entry_is_exact_on_the_book_and_the_lowest_index_wins_a_tie() {
        let book: [HypervectorBody; 8] =
            core::array::from_fn(|i| HypervectorBody::from_seed(200u64.wrapping_add(i as u64)));
        assert_eq!(HypervectorBody::ZERO.nearest(&[]), None);
        for (i, entry) in book.iter().enumerate() {
            assert_eq!(entry.nearest(&book), Some((i, 0)));
        }
        let doubled = [book[3], book[1], book[1]];
        assert_eq!(
            book[1].nearest(&doubled),
            Some((1, 0)),
            "the first of two equal entries"
        );
        let mut near = book[5];
        near.words[0] ^= 0b111;
        assert_eq!(near.nearest(&book), Some((5, 3)));
        let far = HypervectorBody::from_seed(999);
        let (index, distance) = far.nearest(&book).unwrap();
        let expected = book.iter().map(|b| far.hamming(b)).min().unwrap();
        assert_eq!(distance, expected);
        assert_eq!(far.hamming(&book[index]), expected);
        assert!(
            book.iter().take(index).all(|b| far.hamming(b) > expected),
            "no earlier tie"
        );
    }

    #[test]
    fn three_bound_roles_bundled_read_back_through_the_codebook_and_a_fourth_reads_as_noise() {
        let book: [HypervectorBody; 64] =
            core::array::from_fn(|i| HypervectorBody::from_seed(1_000u64.wrapping_add(i as u64)));
        let roles: [HypervectorBody; 4] =
            core::array::from_fn(|i| HypervectorBody::from_seed(7_000u64.wrapping_add(i as u64)));
        let fillers = [17usize, 42, 5];
        let pairs: [HypervectorBody; 3] =
            core::array::from_fn(|i| roles[i].bind(&book[fillers[i]]));
        let sealed = HypervectorBody::bundle(&pairs).unwrap();
        let distances = [2_545u32, 2_556, 2_501];
        for (i, &filler) in fillers.iter().enumerate() {
            let unbound = sealed.bind(&roles[i]);
            let (index, distance) = unbound.nearest(&book).unwrap();
            assert_eq!(
                (index, distance),
                (filler, distances[i]),
                "role {i} recovers its filler"
            );
            let runner_up = book
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != filler)
                .map(|(_, b)| unbound.hamming(b))
                .min()
                .unwrap();
            assert!(
                runner_up > 4_900,
                "the nearest wrong entry is far: {runner_up}"
            );
            assert_eq!(
                confidence_q16(distance),
                5_120u32.wrapping_sub(distance).wrapping_mul(Q16_ONE) / 5_120
            );
        }
        let (index, distance) = sealed.bind(&roles[3]).nearest(&book).unwrap();
        assert_eq!(
            (index, distance),
            (25, 4_979),
            "an unbound role: the nearest entry is chance"
        );
        assert_eq!(confidence_q16(distance), 1_804, "a confidence of 0.028");
    }

    #[test]
    fn the_confidence_falls_from_one_at_a_match_to_zero_at_half_the_width() {
        assert_eq!(confidence_q16(0), Q16_ONE);
        assert_eq!(
            confidence_q16(1),
            Q16_ONE - 13,
            "one bit off: 1 - 2/10240 is 65 523.2, rounded down"
        );
        assert_eq!(confidence_q16(BODY_BITS / 4), Q16_ONE / 2);
        assert_eq!(confidence_q16(BODY_BITS / 2 - 1), 12);
        assert_eq!(confidence_q16(BODY_BITS / 2), 0);
        assert_eq!(confidence_q16(BODY_BITS / 2 + 1), 0);
        assert_eq!(confidence_q16(BODY_BITS), 0);
        assert_eq!(confidence_q16(u32::MAX), 0);
    }
}

#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    fn bit(body: &HypervectorBody, i: u32) -> bool {
        body.words[(i / 64) as usize] & (1 << (i % 64)) != 0
    }

    #[test]
    fn binding_round_trips_permutation_composes_and_the_distance_is_a_metric_on_the_walk() {
        let mut rng = Lcg::new(2_020);
        for _ in 0..300 {
            let a = HypervectorBody::from_seed(rng.next_u64());
            let b = HypervectorBody::from_seed(rng.next_u64());
            let c = HypervectorBody::from_seed(rng.next_u64());
            assert_eq!(a.bind(&b).bind(&b), a);
            assert_eq!(a.bind(&b).bind(&a), b);
            let (k, m) = (rng.next_u32(), rng.next_u32());
            assert_eq!(
                a.permute(k).permute(m),
                a.permute((k % BODY_BITS).wrapping_add(m % BODY_BITS) % BODY_BITS),
                "rotations compose modulo the width"
            );
            assert_eq!(
                a.permute(k).permute(BODY_BITS.wrapping_sub(k % BODY_BITS)),
                a
            );
            assert_eq!(
                a.permute(k).hamming(&b.permute(k)),
                a.hamming(&b),
                "a rotation preserves distance"
            );
            let (ab, bc, ac) = (a.hamming(&b), b.hamming(&c), a.hamming(&c));
            assert!(ab <= BODY_BITS && ac <= bc.wrapping_add(ab) && bc <= ab.wrapping_add(ac));
            assert_eq!(ab, b.hamming(&a));
            assert!(
                ab > 4_600 && ab < 5_640,
                "seeded bodies are about half the width apart: {ab}"
            );
        }
    }

    #[test]
    fn a_bundle_of_any_count_is_the_per_bit_majority_on_the_walk() {
        let mut rng = Lcg::new(4_040);
        let tie = HypervectorBody::from_seed(TIE_SEED);
        for _ in 0..40 {
            let n = (rng.below(BUNDLE_MAX as u32) as usize).wrapping_add(1);
            let mut items = [HypervectorBody::ZERO; BUNDLE_MAX];
            for item in items.iter_mut().take(n) {
                *item = HypervectorBody::from_seed(rng.next_u64());
            }
            let bundle = HypervectorBody::bundle(&items[..n]).unwrap();
            let operands = n.wrapping_add(usize::from(n % 2 == 0));
            for i in (0..BODY_BITS).step_by(7) {
                let mut ones = items[..n].iter().filter(|b| bit(b, i)).count();
                if n % 2 == 0 && bit(&tie, i) {
                    ones = ones.wrapping_add(1);
                }
                assert_eq!(
                    bit(&bundle, i),
                    ones.wrapping_mul(2) > operands,
                    "n = {n}, bit {i}"
                );
            }
            for item in &items[..n] {
                assert!(item.hamming(&bundle) < BODY_BITS / 2);
            }
            let (index, distance) = bundle.nearest(&items[..n]).unwrap();
            assert!(index < n && distance == bundle.hamming(&items[index]));
        }
    }
}
