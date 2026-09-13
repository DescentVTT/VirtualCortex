//! Symbolic rule nodes: propositional deduction over predicates that `cortex-symbolic`
//! bindings ground, and Robinson's resolution on clauses of up to two literals (whitepaper
//! §5.2.30, §6.10, §8.8; admitted by ADR-0016).
//!
//! A node is one rule: a condition literal, a consequence literal, and the operator that
//! combines the condition with the parent rule's satisfaction (AND, OR, NOT, IMPLIES, EQUIV).
//! A node whose operator is RESOLVE is a clause, the disjunction of its two literals, derived
//! from two parent clauses by one resolution step; a chain of such nodes is a refutation proof
//! when it reaches the empty clause. Literals are `u32` atoms with the top bit as negation;
//! atom 0 is "no literal", so a unit clause is `(lit, LITERAL_NONE)` and the empty clause is
//! `(LITERAL_NONE, LITERAL_NONE)`. The truth tables and the propositional resolution step are
//! Implemented, and so are the term arena and first-order unification of [`term`] (ADR-0025):
//! a literal may name a term node, and a resolution step unifies the complementary pair; and,
//! since ADR-0040, syntax as type reduction over the same arena ([`category`]): a category is
//! a term, the four combinatory rules are unifications, and a greedy shift-reduce reducer
//! reads a sequence of lexical categories into one with a log of what it did. Clause search,
//! constraint propagation, type raising and a chart are Specified.

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here (ADR-0029; migrated under brief 016 on 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]

pub mod category;
pub mod induce;
pub mod term;
pub use category::{
    CATEGORY_BACKWARD, CATEGORY_FORWARD, CATEGORY_RESERVED, ParseError, ParseScratch,
    RULE_BACKWARD_APPLICATION, RULE_BACKWARD_COMPOSITION, RULE_FORWARD_APPLICATION,
    RULE_FORWARD_COMPOSITION, Reduction, SLASH_ARITY, argument, backward, forward, head,
    is_functor, reduce, result, role, slash,
};
pub use induce::{
    CLAUSE, Frame, INVENTED_BASE, INVENTED_LIMIT, InduceError, InduceMark, InduceScratch,
    Invention, MAX_BODY, Proof, absorb, clause, clause_body_len, clause_head, clause_literal,
    free_variables, identify, intra_construct, is_clause, lgg, next_pair, prove, resolve_definite,
    resolve_literal, size, term_hash,
};
pub use term::{
    Binding, MAX_ARITY, TERM_COMPOUND, TERM_CONSTANT, TERM_EMPTY, TERM_NONE, TERM_VARIABLE,
    TermNode, UnifyResult, WALK_LIMIT, deref, is_negated, literal_of_term, resolve_first_order,
    term_of_literal, undo, unify,
};

/// Satisfied when the condition holds and the parent is satisfied.
pub const OP_AND: u8 = 0;
/// Satisfied when the condition holds or the parent is satisfied.
pub const OP_OR: u8 = 1;
/// Satisfied when the condition does not hold; the parent is ignored.
pub const OP_NOT: u8 = 2;
/// Satisfied when the parent being satisfied implies the condition (`!parent || condition`).
pub const OP_IMPLIES: u8 = 3;
/// Satisfied when the condition and the parent agree.
pub const OP_EQUIV: u8 = 4;
/// The node is a clause derived by resolution; `evaluate` reads it as the disjunction of its
/// literals, and [`SymbolicRuleNode::apply_resolution`] is what derives it.
pub const OP_RESOLVE: u8 = 5;

/// `satisfaction_state`: not yet evaluated, or evaluated with an unknown operator.
pub const STATE_UNKNOWN: u8 = 0;
/// `satisfaction_state`: the last evaluation satisfied the rule, or the resolution step was valid.
pub const STATE_SATISFIED: u8 = 1;
/// `satisfaction_state`: the last evaluation violated the rule, or the resolution step was not valid.
pub const STATE_VIOLATED: u8 = 2;

/// The literal that is not there: atom 0.
pub const LITERAL_NONE: u32 = 0;
/// Literal bit: the atom is negated.
pub const LITERAL_NEGATED: u32 = 1 << 31;

/// A clause of at most two literals; `LITERAL_NONE` fills an empty slot.
pub type Clause = (u32, u32);
/// The empty clause: a contradiction has been derived.
pub const EMPTY_CLAUSE: Clause = (LITERAL_NONE, LITERAL_NONE);

/// The literal's atom, without its sign.
#[inline]
pub const fn atom(literal: u32) -> u32 {
    literal & !LITERAL_NEGATED
}

/// The literal with its sign flipped; `LITERAL_NONE` stays itself.
#[inline]
pub const fn negate(literal: u32) -> u32 {
    if literal == LITERAL_NONE {
        LITERAL_NONE
    } else {
        literal ^ LITERAL_NEGATED
    }
}

/// True when the two literals are the same atom with opposite signs.
#[inline]
pub const fn complementary(a: u32, b: u32) -> bool {
    a != LITERAL_NONE && b != LITERAL_NONE && a == negate(b)
}

/// True when the clause contains an atom and its negation, so it is always satisfied and
/// useless as a premise.
#[inline]
pub const fn is_tautology(clause: Clause) -> bool {
    complementary(clause.0, clause.1)
}

/// Robinson's resolution on two clauses of at most two literals: the first complementary pair
/// across them, in the order (a.0, b.0), (a.0, b.1), (a.1, b.0), (a.1, b.1), is cancelled and
/// the resolvent is the two remaining literals. `None` when no pair is complementary. The
/// resolvent of two unit clauses is [`EMPTY_CLAUSE`]; a resolvent may be a tautology, which
/// [`is_tautology`] reports.
pub const fn resolve(a: Clause, b: Clause) -> Option<Clause> {
    if complementary(a.0, b.0) {
        Some((a.1, b.1))
    } else if complementary(a.0, b.1) {
        Some((a.1, b.0))
    } else if complementary(a.1, b.0) {
        Some((a.0, b.1))
    } else if complementary(a.1, b.1) {
        Some((a.0, b.0))
    } else {
        None
    }
}

/// 64-byte rule node (whitepaper §5.2.30).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct SymbolicRuleNode {
    pub rule_id: u32,                // [0..4] This rule
    pub condition_predicate_id: u32, // [4..8] Condition literal (or the clause's first literal)
    pub consequence_action_id: u32,  // [8..12] Consequence literal (or the clause's second literal)
    pub parent_rule_idx: u32,        // [12..16] The rule this one chains from (arena index)
    pub resolved_with_idx: u32,      // [16..20] The second parent of a RESOLVE node (arena index)
    pub support_count: u32,          // [20..24] Evaluations that satisfied the rule
    pub confidence_q16: u32,         // [24..28] Confidence in the rule (Q16.16, Specified)
    pub logical_operator: u8,        // [28] OP_*
    pub proof_depth: u8,             // [29] Distance from the axiom
    pub satisfaction_state: u8,      // [30] STATE_*
    pub _reserved: [u8; 33],         // [31..64] Reserved; MUST be zero
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
            resolved_with_idx: 0,
            support_count: 0,
            confidence_q16: 0,
            logical_operator: 0,
            proof_depth: 0,
            satisfaction_state: 0,
            _reserved: [0; 33],
        }
    }
}

impl SymbolicRuleNode {
    /// Evaluates the rule from the truth of its condition and the satisfaction of its parent.
    /// A RESOLVE node reads as the disjunction of its literals: `condition_holds` is the truth
    /// of its first literal and `parent_satisfied` that of its second. Returns `true` when the
    /// rule is satisfied, in which case the support count grows (saturating). An unknown
    /// operator leaves the state unknown and returns `false`.
    pub fn evaluate(&mut self, condition_holds: bool, parent_satisfied: bool) -> bool {
        let satisfied = match self.logical_operator {
            OP_AND => condition_holds && parent_satisfied,
            OP_OR | OP_RESOLVE => condition_holds || parent_satisfied,
            OP_NOT => !condition_holds,
            OP_IMPLIES => !parent_satisfied || condition_holds,
            OP_EQUIV => condition_holds == parent_satisfied,
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

    /// The node's two literals as a clause.
    #[inline]
    pub const fn clause(&self) -> Clause {
        (self.condition_predicate_id, self.consequence_action_id)
    }

    /// One resolution step: makes this node the resolvent of `parent` (at `parent_rule_idx`)
    /// and `other` (at `other_idx`), one step deeper than `depth`. On a valid step the node
    /// holds the resolvent, is a RESOLVE node and is satisfied; when no literals are
    /// complementary the node is left as it was apart from being marked violated. Returns
    /// whether the step was valid.
    pub fn apply_resolution(
        &mut self,
        parent: Clause,
        parent_idx: u32,
        other: Clause,
        other_idx: u32,
        depth: u8,
    ) -> bool {
        match resolve(parent, other) {
            Some(resolvent) => self.record_resolvent(resolvent, parent_idx, other_idx, depth),
            None => {
                self.satisfaction_state = STATE_VIOLATED;
                false
            }
        }
    }

    /// Records a resolvent another step computed (a first-order one through
    /// [`term::resolve_first_order`], whose bindings live in the caller's table): the node
    /// holds it, is a RESOLVE node one step deeper than `depth`, satisfied, with both parents.
    /// Always `true`; the propositional [`apply_resolution`](Self::apply_resolution) is this
    /// after [`resolve`].
    pub fn record_resolvent(
        &mut self,
        resolvent: Clause,
        parent_idx: u32,
        other_idx: u32,
        depth: u8,
    ) -> bool {
        self.condition_predicate_id = resolvent.0;
        self.consequence_action_id = resolvent.1;
        self.parent_rule_idx = parent_idx;
        self.resolved_with_idx = other_idx;
        self.logical_operator = OP_RESOLVE;
        self.proof_depth = depth.saturating_add(1);
        self.satisfaction_state = STATE_SATISFIED;
        self.support_count = self.support_count.saturating_add(1);
        true
    }

    /// True for a RESOLVE node that derived the empty clause: the premises it descends from
    /// are contradictory, which refutes the negated conjecture they include.
    #[inline]
    pub const fn is_refutation(&self) -> bool {
        self.logical_operator == OP_RESOLVE
            && self.condition_predicate_id == LITERAL_NONE
            && self.consequence_action_id == LITERAL_NONE
            && self.satisfaction_state == STATE_SATISFIED
    }
}

const _: () = {
    assert!(core::mem::size_of::<SymbolicRuleNode>() == 64);
    assert!(core::mem::align_of::<SymbolicRuleNode>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const P: u32 = 1;
    const Q: u32 = 2;
    const R: u32 = 3;

    fn rule(op: u8) -> SymbolicRuleNode {
        SymbolicRuleNode {
            rule_id: 1,
            logical_operator: op,
            ..Default::default()
        }
    }

    /// The five operators over the four input combinations, in the order (c, p) =
    /// (false, false), (false, true), (true, false), (true, true).
    const TABLE: [(u8, [bool; 4]); 6] = [
        (OP_AND, [false, false, false, true]),
        (OP_OR, [false, true, true, true]),
        (OP_NOT, [true, true, false, false]),
        (OP_IMPLIES, [true, false, true, true]),
        (OP_EQUIV, [true, false, false, true]),
        (OP_RESOLVE, [false, true, true, true]),
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
        assert_eq!(d.clause(), EMPTY_CLAUSE);
        assert!(
            !d.is_refutation(),
            "an AND node with no literals is not a refutation"
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
        let mut r = rule(6);
        r.satisfaction_state = STATE_SATISFIED;
        assert!(!r.evaluate(true, true));
        assert_eq!(r.satisfaction_state, STATE_UNKNOWN);
        assert_eq!(r.support_count, 0);
    }

    #[test]
    fn literals_negate_and_complement_and_atom_zero_is_no_literal() {
        assert_eq!(negate(P), P | LITERAL_NEGATED);
        assert_eq!(negate(negate(P)), P);
        assert_eq!(negate(LITERAL_NONE), LITERAL_NONE);
        assert_eq!(atom(negate(Q)), Q);
        assert!(complementary(P, negate(P)));
        assert!(!complementary(P, P));
        assert!(!complementary(P, negate(Q)));
        assert!(
            !complementary(LITERAL_NONE, LITERAL_NONE),
            "two absences do not cancel"
        );
        assert!(is_tautology((P, negate(P))));
        assert!(!is_tautology((P, Q)));
    }

    #[test]
    fn resolution_cancels_the_first_complementary_pair_in_each_position() {
        assert_eq!(resolve((P, Q), (negate(P), R)), Some((Q, R)), "(a.0, b.0)");
        assert_eq!(resolve((P, Q), (R, negate(P))), Some((Q, R)), "(a.0, b.1)");
        assert_eq!(resolve((Q, P), (negate(P), R)), Some((Q, R)), "(a.1, b.0)");
        assert_eq!(resolve((Q, P), (R, negate(P))), Some((Q, R)), "(a.1, b.1)");
        assert_eq!(
            resolve((P, Q), (P, R)),
            None,
            "same sign: nothing to cancel"
        );
        assert_eq!(resolve((P, Q), (R, LITERAL_NONE)), None);
    }

    #[test]
    fn unit_clauses_resolve_to_units_and_then_to_the_empty_clause() {
        let unit_p = (P, LITERAL_NONE);
        let unit_not_p = (negate(P), LITERAL_NONE);
        assert_eq!(
            resolve(unit_p, (negate(P), Q)),
            Some((LITERAL_NONE, Q)),
            "a unit clause Q"
        );
        assert_eq!(resolve(unit_p, unit_not_p), Some(EMPTY_CLAUSE));
        assert_eq!(
            resolve((P, Q), (negate(P), negate(Q))),
            Some((Q, negate(Q))),
            "resolving on P first leaves a tautology, which the caller can see"
        );
        assert!(is_tautology(
            resolve((P, Q), (negate(P), negate(Q))).unwrap()
        ));
    }

    #[test]
    fn a_two_step_refutation_of_modus_ponens_reaches_the_empty_clause() {
        // Premises: P; P → Q, as the clause (¬P ∨ Q); negated conjecture: ¬Q.
        let mut step1 = SymbolicRuleNode::default();
        assert!(step1.apply_resolution((P, LITERAL_NONE), 0, (negate(P), Q), 1, 0));
        assert_eq!(step1.clause(), (LITERAL_NONE, Q));
        assert_eq!((step1.logical_operator, step1.proof_depth), (OP_RESOLVE, 1));
        assert_eq!((step1.parent_rule_idx, step1.resolved_with_idx), (0, 1));
        assert!(!step1.is_refutation());
        let mut step2 = SymbolicRuleNode::default();
        assert!(step2.apply_resolution(
            step1.clause(),
            2,
            (negate(Q), LITERAL_NONE),
            3,
            step1.proof_depth
        ));
        assert_eq!(step2.clause(), EMPTY_CLAUSE);
        assert_eq!(step2.proof_depth, 2);
        assert!(step2.is_refutation(), "Q follows from P and P → Q");
        assert_eq!(step2.support_count, 1);
    }

    #[test]
    fn an_invalid_resolution_step_marks_the_node_violated_and_keeps_its_literals() {
        let mut node = SymbolicRuleNode {
            condition_predicate_id: R,
            ..Default::default()
        };
        assert!(!node.apply_resolution((P, Q), 0, (P, R), 1, 0));
        assert_eq!(node.satisfaction_state, STATE_VIOLATED);
        assert_eq!(node.condition_predicate_id, R, "the literals are untouched");
        assert_eq!(node.logical_operator, OP_AND);
        assert!(!node.is_refutation());
        node.proof_depth = u8::MAX;
        assert!(node.apply_resolution((P, LITERAL_NONE), 0, (negate(P), LITERAL_NONE), 1, u8::MAX));
        assert_eq!(node.proof_depth, u8::MAX, "the depth saturates");
    }
}
