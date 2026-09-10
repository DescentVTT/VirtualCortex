//! Executive planning: nodes of a goal-directed lookahead tree (whitepaper §5.2.10). Search
//! and regret evaluation are Specified (§8.8); goal-free rehearsal is `cortex-imagination`
//! (ADR-0016).

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here, one of the crates that passed the lint when it was adopted (ADR-0029).
#![deny(clippy::arithmetic_side_effects)]

/// 64-byte record: one node of a lookahead tree (whitepaper §5.2.10).
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutivePlanNode {
    pub goal_hash: u64,             // 8 bytes (offset 0..8)
    pub parent_node_offset: u32,    // 4 bytes (offset 8..12)
    pub branch_confidence: i32,     // 4 bytes (offset 12..16)
    pub counterfactual_regret: i32, // 4 bytes (offset 16..20)
    pub tree_depth: u16,            // 2 bytes (offset 20..22)
    pub pruned_flag: u8,            // 1 byte (offset 22..23)
    pub padding: [u8; 41],          // 41 bytes (offset 23..64)
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
