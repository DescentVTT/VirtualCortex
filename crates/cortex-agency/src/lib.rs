//! # cortex-agency
//!
//! Intersubjective Agency, Self/Other Motor Cancellation, and Theory of Mind (ToM).
//! Engineered to 2026+ Systems Best Practice (`Latest != Newest`).

#![no_std]

/// 64-byte POD cache-line aligned agent perspective and theory-of-mind state.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AgentPerspectiveState {
    pub perspective_frame_hash: u64, // 8 bytes (offset 0..8)
    pub intention_vector_idx: u64, // 8 bytes (offset 8..16): arena index of the intention hypervector, never a pointer (rule L-3)
    pub agent_id: u32,             // 4 bytes (offset 16..20)
    pub trust_score: i32,          // 4 bytes (offset 20..24)
    pub efference_copy_flag: u8,   // 1 byte (offset 24..25)
    pub padding: [u8; 39],         // 39 bytes (offset 25..64)
}

const _: () = {
    assert!(core::mem::size_of::<AgentPerspectiveState>() == 64);
    assert!(core::mem::align_of::<AgentPerspectiveState>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_perspective_state_layout() {
        assert_eq!(core::mem::size_of::<AgentPerspectiveState>(), 64);
        assert_eq!(core::mem::align_of::<AgentPerspectiveState>(), 64);
    }
}
