//! Vector Symbolic Architecture (VSA / HDC) & Broca/Wernicke Language Bridge

#![no_std]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SymbolicHypervectorHeader {
    pub vector_id: u32,              // [0..4] Symbolic concept index
    pub dimensionality: u32,         // [4..8] Hypervector width (typically 10,000 bits)
    pub binding_role_id: u32,        // [8..12] Bound relation / predicate role ID
    pub filler_concept_id: u32,      // [12..16] Bound concept filler ID
    pub token_vocab_id: u32,         // [16..20] Corresponding NLP token vocabulary ID
    pub hamming_distance_cache: u32, // [20..24] Nearest-neighbor associative distance
    pub permutation_shift: u16,      // [24..26] Syntax cyclic rotation index Pi^k
    pub flags: u16,                  // [26..28] Feature flags (Bipolar, Clean, Bound)
    pub confidence_score: u32,       // [28..32] Decoding confidence score (Q16.16)
    pub _reserved: [u8; 32],         // [32..64] Strict 64-byte cache-line alignment padding
}

impl SymbolicHypervectorHeader {
    pub const DIMENSIONS: usize = 10_000;

    #[inline(always)]
    pub fn bind(&mut self, role_id: u32, filler_id: u32) {
        self.binding_role_id = role_id;
        self.filler_concept_id = filler_id;
        self.flags |= 0x01; // Marked as bound
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
            token_vocab_id: 5,
            hamming_distance_cache: 77,
            permutation_shift: 3,
            flags: 0,
            confidence_score: 0x8000,
            _reserved: [0; 32],
        }
    }

    #[test]
    fn header_is_one_cache_line() {
        assert_eq!(core::mem::size_of::<SymbolicHypervectorHeader>(), 64);
        assert_eq!(core::mem::align_of::<SymbolicHypervectorHeader>(), 64);
    }

    #[test]
    fn dimensions_are_ten_thousand() {
        assert_eq!(SymbolicHypervectorHeader::DIMENSIONS, 10_000);
    }

    #[test]
    fn bind_stores_the_pair_and_sets_the_bound_bit() {
        let mut h = header();
        h.bind(7, 42);
        assert_eq!(h.binding_role_id, 7);
        assert_eq!(h.filler_concept_id, 42);
        assert_eq!(h.flags & 0x01, 0x01);
    }

    #[test]
    fn bind_preserves_the_other_flag_bits() {
        let mut h = header();
        h.flags = 0xFFFE;
        h.bind(1, 2);
        assert_eq!(h.flags, 0xFFFF);
    }

    #[test]
    fn bind_is_idempotent_and_rebinds() {
        let mut h = header();
        h.bind(1, 2);
        let once = h;
        h.bind(1, 2);
        assert_eq!(h, once);
        h.bind(3, 4);
        assert_eq!((h.binding_role_id, h.filler_concept_id, h.flags), (3, 4, 1));
    }

    #[test]
    fn bind_touches_nothing_else() {
        let before = header();
        let mut h = before;
        h.bind(1, 2);
        h.binding_role_id = before.binding_role_id;
        h.filler_concept_id = before.filler_concept_id;
        h.flags = before.flags;
        assert_eq!(h, before);
    }
}
