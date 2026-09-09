//! Vector Symbolic Architecture (VSA / HDC) & Broca/Wernicke Language Bridge

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
