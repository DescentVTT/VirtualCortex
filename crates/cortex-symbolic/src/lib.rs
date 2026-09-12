//! Vector-symbolic architecture: hypervector headers, role binding and, since ADR-0021, the
//! header of a conceptual blend (whitepaper §5.2.9, §8.13); and, since ADR-0039, the
//! hypervector body itself with the algebra over it ([`body`]: binding by XOR, permutation by
//! rotation, bundling by majority, the Hamming distance, the nearest codebook entry and a
//! decode confidence), all integer rules over 160 words with no intrinsic and no `unsafe`.
//! The frames that unbound roles fill are `cortex-linguistic`'s (ADR-0016); the composition
//! that reads a sealed frame back through a codebook is the runtime's.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here (ADR-0029; migrated under brief 016 on 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]

pub mod body;
pub use body::{
    BODY_BITS, BODY_WORDS, BUNDLE_MAX, HypervectorBody, Q16_ONE, TIE_SEED, confidence_q16,
};

/// `flags` bit: the header holds a role/filler binding.
pub const FLAG_BOUND: u16 = 0x0001;
/// `flags` bit: the header is a conceptual blend of a target with a source domain (ADR-0021).
pub const FLAG_BLENDED: u16 = 0x0004;
/// `flags` bit: the vector's basis has been rotated at least once (ADR-0026).
pub const FLAG_REBASED: u16 = 0x0008;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SymbolicHypervectorHeader {
    pub vector_id: u32,              // [0..4] Symbolic concept index
    pub dimensionality: u32,         // [4..8] Hypervector width (typically 10,000 bits)
    pub binding_role_id: u32, // [8..12] Bound relation / predicate role ID; a blend's target concept
    pub filler_concept_id: u32, // [12..16] Bound concept filler ID
    pub token_vocab_id: u32,  // [16..20] Corresponding NLP token vocabulary ID
    pub hamming_distance_cache: u32, // [20..24] Nearest-neighbor associative distance
    pub permutation_shift: u16, // [24..26] Cyclic rotation index Pi^k: syntax position, or a blend's cross-domain map
    pub flags: u16,             // [26..28] Feature flags (bit 0 bound, bit 2 blended)
    pub confidence_score: u32,  // [28..32] Decoding confidence score (Q16.16)
    pub blend_source_id: u32, // [32..36] The source concept a blend draws its structure from (ADR-0021)
    pub blending_domain_mask: u16, // [36..38] Source domains blended in, one bit each (ADR-0021)
    pub blend_depth: u8,      // [38] Blends applied to this vector, saturating (ADR-0021)
    pub rebase_count: u8,     // [39] Basis rotations applied, saturating (ADR-0026)
    pub _reserved: [u8; 24],  // [40..64] Strict 64-byte cache-line alignment padding
}

impl SymbolicHypervectorHeader {
    /// The width of a body in bits: `BODY_BITS`, 10 240 (ADR-0039). Kanerva's nominal 10 000
    /// is not a whole number of 64-bit words; the body uses every bit of its twenty lines.
    pub const DIMENSIONS: usize = BODY_BITS as usize;

    #[inline(always)]
    pub fn bind(&mut self, role_id: u32, filler_id: u32) {
        self.binding_role_id = role_id;
        self.filler_concept_id = filler_id;
        self.flags |= FLAG_BOUND; // Marked as bound
    }

    /// A readout (ADR-0039): the header records that `role_id` was unbound from the vector
    /// and resolved to `filler_id` at `distance` from that codebook entry, with the confidence
    /// [`confidence_q16`] gives that distance. Binds the pair as [`bind`](Self::bind) does.
    pub fn record_readout(&mut self, role_id: u32, filler_id: u32, distance: u32) {
        self.bind(role_id, filler_id);
        self.hamming_distance_cache = distance;
        self.confidence_score = confidence_q16(distance);
    }

    /// Conceptual blending (Fauconnier–Turner) in vector-symbolic form (ADR-0021, whitepaper
    /// §8.13): `blend = target ⊗ M ⊕ source`, where the cross-domain map `M` is a cyclic
    /// permutation by `cross_domain_shift`. The header records the target as the bound role,
    /// the source as `blend_source_id`, the map as `permutation_shift`, the source's domain
    /// bits in the mask, and one more blend in the depth. The vector arithmetic itself is over
    /// the bodies in their arena and is Specified. Refused, with nothing changed, when
    /// `domain_mask` is zero: a blend must say which domain it borrowed from.
    pub fn blend(
        &mut self,
        target_id: u32,
        source_id: u32,
        domain_mask: u16,
        cross_domain_shift: u16,
    ) -> bool {
        if domain_mask == 0 {
            return false;
        }
        self.binding_role_id = target_id;
        self.blend_source_id = source_id;
        self.permutation_shift = cross_domain_shift;
        self.blending_domain_mask |= domain_mask;
        self.blend_depth = self.blend_depth.saturating_add(1);
        self.flags |= FLAG_BLENDED;
        true
    }

    /// A basis rotation (ADR-0026, whitepaper §8.15): the cyclic permutation the vector is
    /// read through advances by `shift` modulo the dimensionality, composing with any earlier
    /// rotation (a permutation of a bipolar hypervector is an orthogonal change of basis, so
    /// two rotations are one). Refused, with nothing changed, for a zero shift, a zero
    /// dimensionality, or one the sixteen-bit shift cannot index. Returns the new shift.
    pub fn rebase(&mut self, shift: u16) -> Option<u16> {
        // One past `u16::MAX`: the largest dimensionality a sixteen-bit shift indexes.
        const SHIFT_SPAN: u32 = 1 << 16;
        if shift == 0 || self.dimensionality == 0 || self.dimensionality > SHIFT_SPAN {
            return None;
        }
        // Two sixteen-bit shifts cannot leave the `u32`; the modulus was checked non-zero.
        let composed = (self.permutation_shift as u32)
            .wrapping_add(shift as u32)
            .checked_rem(self.dimensionality)?;
        self.permutation_shift = composed as u16;
        self.rebase_count = self.rebase_count.saturating_add(1);
        self.flags |= FLAG_REBASED;
        Some(self.permutation_shift)
    }

    /// True for a vector whose basis has been rotated.
    #[inline]
    pub const fn is_rebased(&self) -> bool {
        self.flags & FLAG_REBASED != 0
    }

    /// True for the header of a blend.
    #[inline]
    pub const fn is_blend(&self) -> bool {
        self.flags & FLAG_BLENDED != 0
    }
}

const _: () = {
    assert!(core::mem::size_of::<SymbolicHypervectorHeader>() == 64);
    assert!(core::mem::align_of::<SymbolicHypervectorHeader>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    fn header() -> SymbolicHypervectorHeader {
        SymbolicHypervectorHeader {
            vector_id: 9,
            dimensionality: SymbolicHypervectorHeader::DIMENSIONS as u32,
            binding_role_id: 0,
            filler_concept_id: 0,
            token_vocab_id: 0,
            hamming_distance_cache: 0,
            permutation_shift: 0,
            flags: 0,
            confidence_score: 0,
            blend_source_id: 0,
            blending_domain_mask: 0,
            blend_depth: 0,
            rebase_count: 0,
            _reserved: [0; 24],
        }
    }

    #[test]
    fn record_is_one_cache_line_and_dimensions_are_the_body_s_width() {
        assert_eq!(core::mem::size_of::<SymbolicHypervectorHeader>(), 64);
        assert_eq!(core::mem::align_of::<SymbolicHypervectorHeader>(), 64);
        assert_eq!(SymbolicHypervectorHeader::DIMENSIONS, 10_240);
        assert_eq!(SymbolicHypervectorHeader::DIMENSIONS, BODY_BITS as usize);
    }

    #[test]
    fn a_readout_binds_the_pair_and_writes_the_distance_and_its_confidence() {
        let mut h = header();
        h.record_readout(3, 17, 2_560);
        assert_eq!((h.binding_role_id, h.filler_concept_id), (3, 17));
        assert_eq!(h.flags & FLAG_BOUND, FLAG_BOUND);
        assert_eq!(h.hamming_distance_cache, 2_560);
        assert_eq!(
            h.confidence_score,
            Q16_ONE / 2,
            "a quarter of the width is 0.5"
        );
        h.record_readout(4, 18, 0);
        assert_eq!((h.hamming_distance_cache, h.confidence_score), (0, Q16_ONE));
        h.record_readout(4, 18, 5_120);
        assert_eq!(h.confidence_score, 0, "half the width is chance");
    }

    #[test]
    fn bind_stores_the_pair_and_sets_bit_zero() {
        let mut h = header();
        h.bind(3, 4);
        assert_eq!((h.binding_role_id, h.filler_concept_id), (3, 4));
        assert_eq!(h.flags & FLAG_BOUND, FLAG_BOUND);
    }

    #[test]
    fn bind_preserves_other_flag_bits_and_is_idempotent() {
        let mut h = header();
        h.flags = 0x8000;
        h.bind(1, 2);
        h.bind(1, 2);
        assert_eq!(h.flags, 0x8000 | FLAG_BOUND);
        assert_eq!((h.binding_role_id, h.filler_concept_id), (1, 2));
    }

    #[test]
    fn bind_touches_nothing_else() {
        let mut h = header();
        let before = h;
        h.bind(5, 6);
        assert_eq!(h.vector_id, before.vector_id);
        assert_eq!(h.dimensionality, before.dimensionality);
        assert_eq!(h.token_vocab_id, before.token_vocab_id);
        assert_eq!(h.confidence_score, before.confidence_score);
        assert_eq!(h._reserved, before._reserved);
        assert!(!h.is_blend());
    }

    #[test]
    fn a_blend_records_target_source_map_and_domain_and_accumulates_depth() {
        let mut h = header();
        assert!(h.blend(100, 200, 0x0001, 17));
        assert!(h.is_blend());
        assert_eq!((h.binding_role_id, h.blend_source_id), (100, 200));
        assert_eq!(
            (h.permutation_shift, h.blending_domain_mask, h.blend_depth),
            (17, 0x0001, 1)
        );
        assert!(
            h.blend(100, 300, 0x0004, 3),
            "a second blend from another domain"
        );
        assert_eq!(h.blending_domain_mask, 0x0005, "domains accumulate");
        assert_eq!(
            (h.blend_source_id, h.permutation_shift, h.blend_depth),
            (300, 3, 2)
        );
        h.blend_depth = u8::MAX;
        assert!(h.blend(100, 400, 0x0002, 1));
        assert_eq!(h.blend_depth, u8::MAX, "the depth saturates");
    }

    #[test]
    fn a_blend_without_a_domain_is_refused_unchanged() {
        let mut h = header();
        let before = h;
        assert!(!h.blend(1, 2, 0, 5));
        assert_eq!(h, before);
    }

    #[test]
    fn rotations_compose_modulo_the_dimensionality_and_the_refusals_change_nothing() {
        let mut h = header();
        assert_eq!(h.rebase(0), None, "no rotation");
        let before = h;
        assert_eq!(h.rebase(10_235), Some(10_235));
        assert!(h.is_rebased());
        assert_eq!(h.rebase(10), Some(5), "wraps at 10 240");
        assert_eq!((h.permutation_shift, h.rebase_count), (5, 2));
        assert_eq!(h.flags, FLAG_REBASED);
        let mut flat = header();
        flat.dimensionality = 0;
        assert_eq!(flat.rebase(1), None);
        let mut wide = header();
        wide.dimensionality = 65_537;
        assert_eq!(wide.rebase(1), None, "a shift the field cannot index");
        let mut widest = header();
        widest.dimensionality = 65_536;
        widest.permutation_shift = 65_535;
        assert_eq!(
            widest.rebase(1),
            Some(0),
            "65 536 dimensions still index with sixteen bits"
        );
        assert!(!before.is_rebased());
        h.rebase_count = u8::MAX;
        assert!(h.rebase(1).is_some());
        assert_eq!(h.rebase_count, u8::MAX, "the count saturates");
    }
}
