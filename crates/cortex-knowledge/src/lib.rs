//! Semantic ontology nodes: decontextualised world knowledge, one concept per record, with its
//! category, affordances, typical mass and hazard (whitepaper §5.2.29, §8.8; admitted by
//! ADR-0016).
//!
//! `cortex-symbolic` holds transient role/filler bindings and `cortex-hippocampus` the
//! episodes they came from; this crate holds what survives consolidation. Nodes form a tree by
//! `parent_category_id`; the root is its own parent. Consolidation and the affordance test are
//! Implemented; the replay that drives consolidation (whitepaper §6.6) is Specified.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here (ADR-0029; migrated under brief 016 on 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]

/// `affordance_action_mask` bit: the node is a certified theorem; `property_vector_hash` is the
/// hash of its statement, and the certificate came through the brokered prover (§6.10).
pub const AFFORDANCE_CERTIFIED_THEOREM: u32 = 1 << 31;

/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;
/// `anomaly_q16` moves by $2^{-3}$ of its gap to the latest error, and by at least one LSB.
pub const ANOMALY_SHIFT: u32 = 3;
/// An anomaly at or above 0.5 marks the concept's representation stale (ADR-0026).
pub const ANOMALY_THRESHOLD_Q16: u32 = Q16_ONE / 2;
/// `representation_flags` bit: the anomaly crossed the threshold since the last re-representation.
pub const REPRESENTATION_STALE: u16 = 0x0001;
/// `representation_flags` bit: the last re-representation rotated the concept's hypervector basis.
pub const REPRESENTATION_REBASED: u16 = 0x0002;

/// 64-byte ontology node (whitepaper §5.2.29).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SemanticOntologyNode {
    pub concept_node_id: u32,        // [0..4] This concept (an arena index)
    pub parent_category_id: u32,     // [4..8] Its category; the root names itself
    pub property_vector_hash: u32, // [8..12] Hash of the consolidated property hypervector (Specified)
    pub affordance_action_mask: u32, // [12..16] Actions the concept affords, one bit each
    pub typical_mass_grams_q16: u32, // [16..20] Typical mass in grams (Q16.16)
    pub consolidation_count: u32,  // [20..24] Replays that reinforced this node
    pub safety_hazard_level: u8,   // [24] 0 none; higher is more hazardous
    pub _pad: [u8; 3],             // [25..28] Reserved; MUST be zero
    pub anomaly_q16: u32, // [28..32] Slow average of the prediction error the concept leaves unexplained (Q16.16, ADR-0026)
    pub paradigm_epoch: u32, // [32..36] Epoch of the last re-representation; 0 = never (ADR-0026)
    pub representation_flags: u16, // [36..38] REPRESENTATION_* bits (ADR-0026)
    pub re_representations: u8, // [38] Re-representations so far, saturating (ADR-0026)
    pub _reserved: [u8; 25], // [39..64] Reserved; MUST be zero
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

    /// The premise check (ADR-0026, whitepaper §8.15): `error_q16` is the prediction error the
    /// concept left unexplained this epoch. The anomaly moves toward it by $2^{-3}$ of the gap
    /// and at least one LSB, so it reaches zero exactly when the concept explains everything.
    /// Returns `true` on the update at which the anomaly reaches the threshold while the
    /// representation is not already marked stale: the moment the concept's framework has
    /// stopped explaining and a re-representation is due.
    pub fn note_anomaly(&mut self, error_q16: u32) -> bool {
        let current = self.anomaly_q16;
        self.anomaly_q16 = if error_q16 > current {
            current.saturating_add((error_q16.abs_diff(current) >> ANOMALY_SHIFT).max(1))
        } else if error_q16 < current {
            current.saturating_sub((current.abs_diff(error_q16) >> ANOMALY_SHIFT).max(1))
        } else {
            current
        };
        if self.anomaly_q16 >= ANOMALY_THRESHOLD_Q16 && !self.is_stale() {
            self.representation_flags |= REPRESENTATION_STALE;
            true
        } else {
            false
        }
    }

    /// True while a re-representation is due.
    #[inline]
    pub const fn is_stale(&self) -> bool {
        self.representation_flags & REPRESENTATION_STALE != 0
    }

    /// The re-representation (ADR-0026): the concept moves under `new_parent` (its category
    /// changes; its affordances, mass and hazard are its own and stay), the epoch is stamped,
    /// the stale mark is cleared, the anomaly halves (the new framework is on trial), the count
    /// grows; `rebase_shift` is the cyclic rotation the runtime applies to the concept's
    /// hypervector (`SymbolicHypervectorHeader::rebase`), recorded as `REPRESENTATION_REBASED`
    /// when non-zero. Refused, with nothing changed, unless the representation is stale, for
    /// an epoch of zero (which means never), or when nothing would change (the same category
    /// and no rotation).
    pub fn re_represent(&mut self, epoch: u32, new_parent: u32, rebase_shift: u16) -> bool {
        if !self.is_stale()
            || epoch == 0
            || (new_parent == self.parent_category_id && rebase_shift == 0)
        {
            return false;
        }
        self.parent_category_id = new_parent;
        self.paradigm_epoch = epoch;
        self.representation_flags &= !REPRESENTATION_STALE;
        if rebase_shift != 0 {
            self.representation_flags |= REPRESENTATION_REBASED;
        } else {
            self.representation_flags &= !REPRESENTATION_REBASED;
        }
        self.anomaly_q16 >>= 1;
        self.re_representations = self.re_representations.saturating_add(1);
        true
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
    fn the_anomaly_moves_by_an_eighth_of_the_gap_each_way_and_marks_stale_at_a_half() {
        assert_eq!(ANOMALY_THRESHOLD_Q16, Q16_ONE / 2);
        let mut n = SemanticOntologyNode::default();
        assert!(
            !n.note_anomaly(Q16_ONE / 2),
            "one step is not the threshold"
        );
        assert_eq!(n.anomaly_q16, Q16_ONE / 16, "an eighth of the half");
        assert!(!n.note_anomaly(Q16_ONE / 2));
        assert_eq!(
            n.anomaly_q16,
            Q16_ONE / 16 + (Q16_ONE / 2 - Q16_ONE / 16) / 8
        );
        let mut down = SemanticOntologyNode {
            anomaly_q16: Q16_ONE / 2,
            ..Default::default()
        };
        assert!(
            !down.note_anomaly(0),
            "already at the threshold, so not the crossing"
        );
        assert_eq!(
            down.anomaly_q16,
            Q16_ONE / 2 - Q16_ONE / 16,
            "and it falls by an eighth"
        );
        let mut edge = SemanticOntologyNode {
            anomaly_q16: Q16_ONE / 2 - 1,
            ..Default::default()
        };
        assert!(
            edge.note_anomaly(Q16_ONE),
            "one LSB below, then at least one LSB up: the crossing"
        );
        assert!(edge.is_stale());
    }

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

    #[test]
    fn a_persistent_unexplained_error_marks_the_representation_stale_once() {
        let mut n = SemanticOntologyNode::default();
        assert!(!n.is_stale());
        let mut fired_at = None;
        for i in 0..40 {
            if n.note_anomaly(Q16_ONE) {
                assert!(fired_at.is_none(), "fires once");
                fired_at = Some(i);
            }
        }
        assert!(fired_at.is_some() && n.is_stale());
        assert!(n.anomaly_q16 >= ANOMALY_THRESHOLD_Q16);
        assert!(!n.note_anomaly(Q16_ONE), "stale already: no second firing");
        // Everything explained: the anomaly reaches zero exactly, but the mark stays until a
        // re-representation clears it.
        for _ in 0..400 {
            n.note_anomaly(0);
        }
        assert_eq!(n.anomaly_q16, 0);
        assert!(n.is_stale());
    }

    #[test]
    fn a_re_representation_moves_the_concept_keeps_its_affordances_and_needs_staleness() {
        let mut n = SemanticOntologyNode {
            concept_node_id: 5,
            parent_category_id: 2,
            affordance_action_mask: 0b1010,
            typical_mass_grams_q16: 99,
            safety_hazard_level: 3,
            ..Default::default()
        };
        assert!(!n.re_represent(10, 7, 3), "not stale");
        for _ in 0..40 {
            n.note_anomaly(Q16_ONE);
        }
        let anomaly = n.anomaly_q16;
        assert!(!n.re_represent(0, 7, 3), "epoch zero means never");
        assert!(
            !n.re_represent(10, 2, 0),
            "the same category and no rotation change nothing"
        );
        assert!(n.is_stale(), "and the mark stays");
        assert!(n.re_represent(10, 7, 3));
        assert_eq!(
            (n.parent_category_id, n.paradigm_epoch, n.re_representations),
            (7, 10, 1)
        );
        assert_eq!(n.anomaly_q16, anomaly >> 1, "the new framework is on trial");
        assert!(!n.is_stale());
        assert_eq!(
            n.representation_flags & REPRESENTATION_REBASED,
            REPRESENTATION_REBASED
        );
        assert_eq!(
            (
                n.affordance_action_mask,
                n.typical_mass_grams_q16,
                n.safety_hazard_level
            ),
            (0b1010, 99, 3),
            "what is the concept's stays"
        );
        assert!(!n.re_represent(11, 8, 0), "stale no longer");
        for _ in 0..40 {
            n.note_anomaly(Q16_ONE);
        }
        assert!(n.re_represent(11, 8, 0));
        assert_eq!(
            n.representation_flags & REPRESENTATION_REBASED,
            0,
            "no rotation this time"
        );
        assert_eq!(n.re_representations, 2);
    }
}
