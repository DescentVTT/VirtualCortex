//! Memory hygiene: the per-segment scrub record (whitepaper §5.2.13). The scrub daemon,
//! compaction and the checksum audit are Specified (§6.6, §8.6).

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here, one of the crates that passed the lint when it was adopted (ADR-0029).
#![deny(clippy::arithmetic_side_effects)]

/// 64-byte record: one arena segment's scrub state (whitepaper §5.2.13).
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImmuneScrubNode {
    pub ecc_checksum_hash: u64,        // 8 bytes (offset 0..8)
    pub arena_segment_id: u32,         // 4 bytes (offset 8..12)
    pub page_health_score: i32,        // 4 bytes (offset 12..16)
    pub degenerate_synapse_count: u32, // 4 bytes (offset 16..20)
    pub reclamation_active: u8,        // 1 byte (offset 20..21)
    pub padding: [u8; 43],             // 43 bytes (offset 21..64)
}

const _: () = {
    assert!(core::mem::size_of::<ImmuneScrubNode>() == 64);
    assert!(core::mem::align_of::<ImmuneScrubNode>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immune_scrub_node_layout() {
        assert_eq!(core::mem::size_of::<ImmuneScrubNode>(), 64);
        assert_eq!(core::mem::align_of::<ImmuneScrubNode>(), 64);
    }
}
