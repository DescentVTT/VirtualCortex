//! Symbolic rule nodes: propositional deduction over predicates that `cortex-symbolic`
//! bindings ground (whitepaper §5.2.30, §8.8; admitted by ADR-0016).
//!
//! A node is one rule: a condition predicate, a consequence, and the operator that combines the
//! condition with the parent rule's satisfaction. Chains of nodes form proofs; `proof_depth`
//! is the node's distance from the axiom it rests on. The truth-table rule is Implemented; the
//! search over chains and constraint propagation are Specified.

#![no_std]

/// Satisfied when the condition holds and the parent is satisfied.
pub const OP_AND: u8 = 0;
/// Satisfied when the condition holds or the parent is satisfied.
pub const OP_OR: u8 = 1;
/// Satisfied when the condition does not hold; the parent is ignored.
pub const OP_NOT: u8 = 2;
/// Satisfied when the parent being satisfied implies the condition (`!parent || condition`).
pub const OP_IMPLIES: u8 = 3;

/// `satisfaction_state`: not yet evaluated, or evaluated with an unknown operator.
pub const STATE_UNKNOWN: u8 = 0;
/// `satisfaction_state`: the last evaluation satisfied the rule.
pub const STATE_SATISFIED: u8 = 1;
/// `satisfaction_state`: the last evaluation violated the rule.
pub const STATE_VIOLATED: u8 = 2;

/// 64-byte rule node (whitepaper §5.2.30).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SymbolicRuleNode {
    pub rule_id: u32,                // [0..4] This rule
    pub condition_predicate_id: u32, // [4..8] Predicate whose truth is the condition
    pub consequence_action_id: u32,  // [8..12] What follows when the rule is satisfied
    pub parent_rule_idx: u32,        // [12..16] The rule this one chains from (arena index)
    pub support_count: u32,          // [16..20] Evaluations that satisfied the rule
    pub confidence_q16: u32,         // [20..24] Confidence in the rule (Q16.16, Specified)
    pub logical_operator: u8,        // [24] OP_*
    pub proof_depth: u8,             // [25] Distance from the axiom
    pub satisfaction_state: u8,      // [26] STATE_*
    pub _reserved: [u8; 37],         // [27..64] Reserved; MUST be zero
}

// Arrays longer than 32 elements do not implement Default, so the all-zero record is
// spelled out; every field of a default record is zero.
impl Default for SymbolicRuleNode {
    fn default() -> Self {
        Self {
            rule_id: 0,
            condition_predicate_id: 0,
            consequence_action_id: 0,
            parent_rule_idx: 0,
            support_count: 0,
            confidence_q16: 0,
            logical_operator: 0,
            proof_depth: 0,
            satisfaction_state: 0,
            _reserved: [0; 37],
        }
    }
}

impl SymbolicRuleNode {
    /// Evaluates the rule from the truth of its condition and the satisfaction of its parent.
    /// Returns `true` when the rule is satisfied, in which case the support count grows
    /// (saturating). An unknown operator leaves the state unknown and returns `false`.
    pub fn evaluate(&mut self, condition_holds: bool, parent_satisfied: bool) -> bool {
        let satisfied = match self.logical_operator {
            OP_AND => condition_holds && parent_satisfied,
            OP_OR => condition_holds || parent_satisfied,
            OP_NOT => !condition_holds,
            OP_IMPLIES => !parent_satisfied || condition_holds,
            _ => {
                self.satisfaction_state = STATE_UNKNOWN;
                return false;
            }
        };
        if satisfied {
            self.satisfaction_state = STATE_SATISFIED;
            self.support_count = self.support_count.saturating_add(1);
        } else {
            self.satisfaction_state = STATE_VIOLATED;
        }
        satisfied
    }
}

const _: () = {
    assert!(core::mem::size_of::<SymbolicRuleNode>() == 64);
    assert!(core::mem::align_of::<SymbolicRuleNode>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(op: u8) -> SymbolicRuleNode {
        SymbolicRuleNode {
            rule_id: 1,
            logical_operator: op,
            ..Default::default()
        }
    }

    /// The four operators over the four input combinations, in the order (c, p) =
    /// (false, false), (false, true), (true, false), (true, true).
    const TABLE: [(u8, [bool; 4]); 4] = [
        (OP_AND, [false, false, false, true]),
        (OP_OR, [false, true, true, true]),
        (OP_NOT, [true, true, false, false]),
        (OP_IMPLIES, [true, false, true, true]),
    ];

    #[test]
    fn record_is_one_cache_line_and_default_is_an_unevaluated_and() {
        assert_eq!(core::mem::size_of::<SymbolicRuleNode>(), 64);
        assert_eq!(core::mem::align_of::<SymbolicRuleNode>(), 64);
        let d = SymbolicRuleNode::default();
        assert_eq!(
            (d.logical_operator, d.satisfaction_state),
            (OP_AND, STATE_UNKNOWN)
        );
    }

    #[test]
    fn every_operator_matches_its_truth_table() {
        for (op, expected) in TABLE {
            for (i, (c, p)) in [(false, false), (false, true), (true, false), (true, true)]
                .into_iter()
                .enumerate()
            {
                let mut r = rule(op);
                assert_eq!(r.evaluate(c, p), expected[i], "op {op} with ({c}, {p})");
                let state = if expected[i] {
                    STATE_SATISFIED
                } else {
                    STATE_VIOLATED
                };
                assert_eq!(r.satisfaction_state, state);
            }
        }
    }

    #[test]
    fn support_counts_satisfactions_only_and_saturates() {
        let mut r = rule(OP_OR);
        r.evaluate(true, false);
        r.evaluate(false, false);
        r.evaluate(false, true);
        assert_eq!(r.support_count, 2);
        r.support_count = u32::MAX;
        r.evaluate(true, true);
        assert_eq!(r.support_count, u32::MAX);
    }

    #[test]
    fn an_unknown_operator_is_never_satisfied_and_leaves_the_state_unknown() {
        let mut r = rule(4);
        r.satisfaction_state = STATE_SATISFIED;
        assert!(!r.evaluate(true, true));
        assert_eq!(r.satisfaction_state, STATE_UNKNOWN);
        assert_eq!(r.support_count, 0);
    }
}
