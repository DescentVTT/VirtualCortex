//! Semantic ontology nodes: decontextualised world knowledge, one concept per record, with its
//! category, affordances, typical mass and hazard (whitepaper §5.2.29, §8.8; admitted by
//! ADR-0016).
//!
//! `cortex-symbolic` holds transient role/filler bindings and `cortex-hippocampus` the
//! episodes they came from; this crate holds what survives consolidation. Nodes form a tree by
//! `parent_category_id`; the root is its own parent. Consolidation and the affordance test are
//! Implemented; the replay that drives consolidation (whitepaper §6.6) is Specified.

#![no_std]

/// `affordance_action_mask` bit: the node is a certified theorem; `property_vector_hash` is the
/// hash of its statement, and the certificate came through the brokered prover (§6.10).
pub const AFFORDANCE_CERTIFIED_THEOREM: u32 = 1 << 31;

/// 64-byte ontology node (whitepaper §5.2.29).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SemanticOntologyNode {
    pub concept_node_id: u32,        // [0..4] This concept (an arena index)
    pub parent_category_id: u32,     // [4..8] Its category; the root names itself
    pub property_vector_hash: u32, // [8..12] Hash of the consolidated property hypervector (Specified)
    pub affordance_action_mask: u32, // [12..16] Actions the concept affords, one bit each
    pub typical_mass_grams_q16: u32, // [16..20] Typical mass in grams (Q16.16)
    pub consolidation_count: u32,  // [20..24] Replays that reinforced this node
    pub safety_hazard_level: u8,   // [24] 0 none; higher is more hazardous
    pub _reserved: [u8; 39],       // [25..64] Reserved; MUST be zero
}

// Arrays longer than 32 elements do not implement Default, so the all-zero record is
// spelled out; every field of a default record is zero.
impl Default for SemanticOntologyNode {
    fn default() -> Self {
        Self {
            concept_node_id: 0,
            parent_category_id: 0,
            property_vector_hash: 0,
            affordance_action_mask: 0,
            typical_mass_grams_q16: 0,
            consolidation_count: 0,
            safety_hazard_level: 0,
            _reserved: [0; 39],
        }
    }
}

impl SemanticOntologyNode {
    /// True when every bit of `action_bits` is afforded.
    #[inline]
    pub const fn affords(&self, action_bits: u32) -> bool {
        self.affordance_action_mask & action_bits == action_bits
    }

    /// True for the root of the category tree.
    #[inline]
    pub const fn is_root(&self) -> bool {
        self.parent_category_id == self.concept_node_id
    }

    /// Consolidates a theorem the brokered prover certified: the statement hash is stored, the
    /// theorem bit is set and the replay is counted. A theorem is never hazardous; the hazard
    /// level is left as it was. Returns the count.
    pub fn certify(&mut self, statement_hash: u32) -> u32 {
        self.property_vector_hash = statement_hash;
        self.consolidate(AFFORDANCE_CERTIFIED_THEOREM, 0)
    }

    /// True for a node that holds a certified theorem.
    #[inline]
    pub const fn is_certified_theorem(&self) -> bool {
        self.affords(AFFORDANCE_CERTIFIED_THEOREM)
    }

    /// One consolidation replay: affordances accumulate, the hazard level keeps its maximum,
    /// and the count grows (saturating). Returns the count.
    pub fn consolidate(&mut self, affordance_bits: u32, hazard_level: u8) -> u32 {
        self.affordance_action_mask |= affordance_bits;
        if hazard_level > self.safety_hazard_level {
            self.safety_hazard_level = hazard_level;
        }
        self.consolidation_count = self.consolidation_count.saturating_add(1);
        self.consolidation_count
    }
}

const _: () = {
    assert!(core::mem::size_of::<SemanticOntologyNode>() == 64);
    assert!(core::mem::align_of::<SemanticOntologyNode>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_is_one_cache_line_and_default_is_an_unconsolidated_root() {
        assert_eq!(core::mem::size_of::<SemanticOntologyNode>(), 64);
        assert_eq!(core::mem::align_of::<SemanticOntologyNode>(), 64);
        let d = SemanticOntologyNode::default();
        assert!(d.is_root(), "node 0 with parent 0 is the root");
        assert_eq!(d.consolidation_count, 0);
        assert!(d.affords(0), "the empty action set is always afforded");
    }

    #[test]
    fn affords_requires_every_requested_bit() {
        let n = SemanticOntologyNode {
            affordance_action_mask: 0b0110,
            ..Default::default()
        };
        assert!(n.affords(0b0010));
        assert!(n.affords(0b0110));
        assert!(!n.affords(0b0111));
    }

    #[test]
    fn consolidation_accumulates_affordances_and_keeps_the_worst_hazard() {
        let mut n = SemanticOntologyNode {
            concept_node_id: 5,
            parent_category_id: 2,
            ..Default::default()
        };
        assert!(!n.is_root());
        assert_eq!(n.consolidate(0b0001, 3), 1);
        assert_eq!(n.consolidate(0b0100, 1), 2);
        assert_eq!(n.affordance_action_mask, 0b0101);
        assert_eq!(
            n.safety_hazard_level, 3,
            "a later, milder replay does not lower the hazard"
        );
    }

    #[test]
    fn certifying_a_theorem_stores_its_statement_and_keeps_the_hazard() {
        let mut n = SemanticOntologyNode {
            concept_node_id: 8,
            parent_category_id: 1,
            safety_hazard_level: 2,
            ..Default::default()
        };
        assert!(!n.is_certified_theorem());
        assert_eq!(n.certify(0xC0FFEE), 1);
        assert!(n.is_certified_theorem());
        assert_eq!(n.property_vector_hash, 0xC0FFEE);
        assert_eq!(
            n.safety_hazard_level, 2,
            "a theorem does not lower or raise a hazard"
        );
        assert!(n.affords(AFFORDANCE_CERTIFIED_THEOREM));
        assert!(!n.affords(AFFORDANCE_CERTIFIED_THEOREM | 0b1));
    }

    #[test]
    fn the_consolidation_count_saturates() {
        let mut n = SemanticOntologyNode {
            consolidation_count: u32::MAX,
            ..Default::default()
        };
        assert_eq!(n.consolidate(0, 0), u32::MAX);
    }
}
