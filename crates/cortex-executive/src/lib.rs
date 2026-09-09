//! # cortex-executive
//!
//! Prefrontal Executive Planning, Counterfactual Mental Simulation, and Multi-Step Lookahead Rollouts.
//! Engineered to 2026+ Systems Best Practice (`Latest != Newest`).

#![no_std]

/// 64-byte POD cache-line aligned executive planning node.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutivePlanNode {
    pub goal_hash: u64,              // 8 bytes (offset 0..8)
    pub parent_node_offset: u32,     // 4 bytes (offset 8..12)
    pub branch_confidence: i32,      // 4 bytes (offset 12..16)
    pub counterfactual_regret: i32,  // 4 bytes (offset 16..20)
    pub tree_depth: u16,             // 2 bytes (offset 20..22)
    pub pruned_flag: u8,             // 1 byte (offset 22..23)
    pub padding: [u8; 41],           // 41 bytes (offset 23..64)
}

const _: () = {
    assert!(core::mem::size_of::<ExecutivePlanNode>() == 64);
    assert!(core::mem::align_of::<ExecutivePlanNode>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executive_plan_node_layout() {
        assert_eq!(core::mem::size_of::<ExecutivePlanNode>(), 64);
        assert_eq!(core::mem::align_of::<ExecutivePlanNode>(), 64);
    }
}
