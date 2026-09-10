//! The term arena and first-order unification (whitepaper §5.2.30, §6.10 step 3; ADR-0025;
//! brief 014). A term is a 64-byte node: a constant, a variable or a compound of a functor and
//! up to eight children, every reference an arena index + 1 so that zero is "none" and a
//! zeroed arena is empty (the encoding of ADR-0022). Functors and constants are concept ids
//! from `cortex-symbolic`; a variable is a number that indexes the caller's binding table.
//! Robinson's unification runs over caller-provided slices, a binding table, a trail of the
//! variables it bound (undone on failure) and a work stack whose bound is the recursion bound;
//! an exceeded bound is a result, never a panic, and the occurs check refuses `X = f(X)`.

use crate::{Clause, LITERAL_NEGATED, LITERAL_NONE, atom};

/// `kind`: an empty node.
pub const TERM_EMPTY: u8 = 0;
/// `kind`: a constant; `functor` is its concept id.
pub const TERM_CONSTANT: u8 = 1;
/// `kind`: a variable; `functor` is its number in the binding table.
pub const TERM_VARIABLE: u8 = 2;
/// `kind`: a compound; `functor` is its concept id and `children[..arity]` its arguments.
pub const TERM_COMPOUND: u8 = 3;
/// The most arguments a compound holds.
pub const MAX_ARITY: usize = 8;
/// A child slot, a binding or a literal that names no term.
pub const TERM_NONE: u32 = 0;

/// 64-byte term node (whitepaper §5.2.30, ADR-0025).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct TermNode {
    pub kind: u8,                   // [0] TERM_*
    pub arity: u8,                  // [1] Arguments of a compound, 0 to 8
    pub _pad: u16,                  // [2..4] Reserved; MUST be zero
    pub functor: u32,               // [4..8] Concept id (constant, compound) or variable number
    pub children: [u32; MAX_ARITY], // [8..40] Argument term index + 1 per slot; 0 = none
    pub _reserved: [u8; 24],        // [40..64] Reserved; MUST be zero
}

impl TermNode {
    /// A constant.
    pub const fn constant(concept: u32) -> Self {
        Self {
            kind: TERM_CONSTANT,
            arity: 0,
            _pad: 0,
            functor: concept,
            children: [TERM_NONE; MAX_ARITY],
            _reserved: [0; 24],
        }
    }

    /// A variable numbered `var`, an index into the binding table.
    pub const fn variable(var: u32) -> Self {
        Self {
            kind: TERM_VARIABLE,
            arity: 0,
            _pad: 0,
            functor: var,
            children: [TERM_NONE; MAX_ARITY],
            _reserved: [0; 24],
        }
    }

    /// A compound of `concept` over `args` (term indices). `None` for more than eight or an
    /// index the encoding cannot hold.
    pub fn compound(concept: u32, args: &[u32]) -> Option<Self> {
        if args.len() > MAX_ARITY || args.contains(&u32::MAX) {
            return None;
        }
        let mut node = Self {
            kind: TERM_COMPOUND,
            arity: args.len() as u8,
            _pad: 0,
            functor: concept,
            children: [TERM_NONE; MAX_ARITY],
            _reserved: [0; 24],
        };
        for (slot, &arg) in args.iter().enumerate() {
            node.children[slot] = arg + 1;
        }
        Some(node)
    }

    /// The argument in `slot`, decoded, or `None` past the arity or for an empty slot.
    pub const fn child(&self, slot: usize) -> Option<u32> {
        if slot >= self.arity as usize || slot >= MAX_ARITY || self.children[slot] == TERM_NONE {
            None
        } else {
            Some(self.children[slot] - 1)
        }
    }
}

/// One entry of the binding table: the term a variable is bound to, as index + 1; 0 unbound.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Binding(pub u32);

impl Binding {
    /// Unbound.
    pub const UNBOUND: Binding = Binding(TERM_NONE);

    /// The bound term, decoded.
    pub const fn term(self) -> Option<u32> {
        if self.0 == TERM_NONE {
            None
        } else {
            Some(self.0 - 1)
        }
    }
}

/// What unification found. Every failure leaves the binding table as it was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnifyResult {
    /// The terms unify; the bindings are in the table and the variables bound are on the trail.
    Unified,
    /// A constant, functor or arity differs.
    Clash,
    /// A variable would be bound to a term containing it.
    OccursCheck,
    /// The work stack or the trail is too small for these terms.
    BoundExceeded,
    /// An index outside the arena, a variable outside the table, or an empty node.
    Malformed,
}

/// Follows variable bindings from `term` until a non-variable or an unbound variable, bounded
/// by the arena so that a cyclic binding terminates. `None` for an index outside the arena or
/// a variable outside the table.
pub fn deref(term: u32, arena: &[TermNode], bindings: &[Binding]) -> Option<u32> {
    let mut current = term;
    for _ in 0..=arena.len() {
        let node = arena.get(current as usize)?;
        if node.kind != TERM_VARIABLE {
            return Some(current);
        }
        match bindings.get(node.functor as usize)?.term() {
            Some(bound) => current = bound,
            None => return Some(current),
        }
    }
    None
}

/// Undoes the first `bound` entries of the trail: every variable on it becomes unbound.
pub fn undo(bindings: &mut [Binding], trail: &[u32], bound: usize) {
    for &var in trail.iter().take(bound) {
        if let Some(b) = bindings.get_mut(var as usize) {
            *b = Binding::UNBOUND;
        }
    }
}

/// True when variable `var` occurs in `term` under the bindings; a depth-first walk over
/// `scratch` as its stack. `None` when the walk needs more than `scratch` holds, or on a
/// malformed index.
fn occurs(
    var: u32,
    term: u32,
    arena: &[TermNode],
    bindings: &[Binding],
    scratch: &mut [u32],
) -> Option<bool> {
    let mut top = 0;
    let first = deref(term, arena, bindings)?;
    *scratch.first_mut()? = first;
    top += 1;
    while top > 0 {
        top -= 1;
        let current = scratch[top];
        let node = arena.get(current as usize)?;
        match node.kind {
            TERM_VARIABLE => {
                if node.functor == var {
                    return Some(true);
                }
            }
            TERM_COMPOUND => {
                for slot in 0..node.arity as usize {
                    let child = deref(node.child(slot)?, arena, bindings)?;
                    if top >= scratch.len() {
                        return None;
                    }
                    scratch[top] = child;
                    top += 1;
                }
            }
            TERM_CONSTANT => {}
            _ => return None,
        }
    }
    Some(false)
}

/// Robinson's unification of terms `a` and `b` over `arena`, with the substitution in
/// `bindings` (indexed by variable number), the variables bound by this call pushed onto
/// `trail`, and `stack` as the work stack of pairs still to unify (two entries per pair; its
/// length is the recursion bound; the occurs check walks in the part above the pairs).
/// Returns how many variables were bound with `Unified`; on any failure the bindings this
/// call made are undone and the table is as it was. Bindings already in the table are
/// respected, so a sequence of calls accumulates one substitution.
pub fn unify(
    a: u32,
    b: u32,
    arena: &[TermNode],
    bindings: &mut [Binding],
    trail: &mut [u32],
    stack: &mut [u32],
) -> (UnifyResult, usize) {
    let mut bound = 0usize;
    let outcome = unify_inner(a, b, arena, bindings, trail, stack, &mut bound);
    if outcome != UnifyResult::Unified {
        undo(bindings, trail, bound);
        bound = 0;
    }
    (outcome, bound)
}

fn unify_inner(
    a: u32,
    b: u32,
    arena: &[TermNode],
    bindings: &mut [Binding],
    trail: &mut [u32],
    stack: &mut [u32],
    bound: &mut usize,
) -> UnifyResult {
    if stack.len() < 2 {
        return UnifyResult::BoundExceeded;
    }
    stack[0] = a;
    stack[1] = b;
    let mut top = 2;
    while top > 0 {
        top -= 2;
        let (x, y) = (stack[top], stack[top + 1]);
        let (Some(x), Some(y)) = (deref(x, arena, bindings), deref(y, arena, bindings)) else {
            return UnifyResult::Malformed;
        };
        if x == y {
            continue;
        }
        let (nx, ny) = (arena[x as usize], arena[y as usize]);
        match (nx.kind, ny.kind) {
            (TERM_VARIABLE, _) | (_, TERM_VARIABLE) => {
                let (var, term) = if nx.kind == TERM_VARIABLE {
                    (nx.functor, y)
                } else {
                    (ny.functor, x)
                };
                let (pairs, scratch) = stack.split_at_mut(top);
                let _ = pairs;
                match occurs(var, term, arena, bindings, scratch) {
                    Some(true) => return UnifyResult::OccursCheck,
                    Some(false) => {}
                    None => return UnifyResult::BoundExceeded,
                }
                let Some(slot) = bindings.get_mut(var as usize) else {
                    return UnifyResult::Malformed;
                };
                if *bound >= trail.len() {
                    return UnifyResult::BoundExceeded;
                }
                *slot = Binding(term + 1);
                trail[*bound] = var;
                *bound += 1;
            }
            (TERM_CONSTANT, TERM_CONSTANT) => {
                if nx.functor != ny.functor {
                    return UnifyResult::Clash;
                }
            }
            (TERM_COMPOUND, TERM_COMPOUND) => {
                if nx.functor != ny.functor || nx.arity != ny.arity {
                    return UnifyResult::Clash;
                }
                for slot in (0..nx.arity as usize).rev() {
                    let (Some(cx), Some(cy)) = (nx.child(slot), ny.child(slot)) else {
                        return UnifyResult::Malformed;
                    };
                    if top + 2 > stack.len() {
                        return UnifyResult::BoundExceeded;
                    }
                    stack[top] = cx;
                    stack[top + 1] = cy;
                    top += 2;
                }
            }
            (TERM_CONSTANT, TERM_COMPOUND) | (TERM_COMPOUND, TERM_CONSTANT) => {
                return UnifyResult::Clash;
            }
            _ => return UnifyResult::Malformed,
        }
    }
    UnifyResult::Unified
}

/// A first-order literal: the predicate's term index + 1 with the sign in bit 31. `None` for
/// an index the encoding cannot hold (at or above $2^{31} - 1$, whose `+ 1` would reach the
/// sign bit; ADR-0028).
pub const fn literal_of_term(term: u32, negated: bool) -> Option<u32> {
    if term >= LITERAL_NEGATED - 1 {
        return None;
    }
    let atom = term + 1;
    Some(if negated {
        atom | LITERAL_NEGATED
    } else {
        atom
    })
}

/// The term a first-order literal names, or `None` for `LITERAL_NONE`.
pub const fn term_of_literal(literal: u32) -> Option<u32> {
    let a = atom(literal);
    if a == LITERAL_NONE { None } else { Some(a - 1) }
}

/// True when the literal is negated.
pub const fn is_negated(literal: u32) -> bool {
    literal & LITERAL_NEGATED != 0
}

/// One first-order resolution step: the first pair of literals across `a` and `b`, in the
/// order (a.0, b.0), (a.0, b.1), (a.1, b.0), (a.1, b.1), whose signs differ and whose terms
/// unify under the bindings. The resolvent is the two remaining literals, read through the
/// bindings the step added, which stay in the table (the substitution accumulates across a
/// proof). `Err(Clash)` when no pair unifies; `Err(OccursCheck)`, `Err(BoundExceeded)` or
/// `Err(Malformed)` when the first pair with opposite signs failed that way.
pub fn resolve_first_order(
    a: Clause,
    b: Clause,
    arena: &[TermNode],
    bindings: &mut [Binding],
    trail: &mut [u32],
    stack: &mut [u32],
) -> Result<Clause, UnifyResult> {
    let candidates = [
        (a.0, b.0, a.1, b.1),
        (a.0, b.1, a.1, b.0),
        (a.1, b.0, a.0, b.1),
        (a.1, b.1, a.0, b.0),
    ];
    let mut first_failure = UnifyResult::Clash;
    for (x, y, rest_a, rest_b) in candidates {
        let (Some(tx), Some(ty)) = (term_of_literal(x), term_of_literal(y)) else {
            continue;
        };
        if is_negated(x) == is_negated(y) {
            continue;
        }
        match unify(tx, ty, arena, bindings, trail, stack).0 {
            UnifyResult::Unified => return Ok((rest_a, rest_b)),
            UnifyResult::Clash => {}
            other => {
                if first_failure == UnifyResult::Clash {
                    first_failure = other;
                }
            }
        }
    }
    Err(first_failure)
}

const _: () = {
    assert!(core::mem::size_of::<TermNode>() == 64);
    assert!(core::mem::align_of::<TermNode>() == 64);
    assert!(core::mem::size_of::<Binding>() == 4);
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EMPTY_CLAUSE, SymbolicRuleNode};

    #[test]
    fn an_empty_slot_below_the_arity_is_none_and_three_argument_pairs_fit_six_slots() {
        let mut node = TermNode::compound(9, &[1, 2]).unwrap();
        node.children[1] = TERM_NONE;
        assert_eq!(node.child(0), Some(1), "the argument is term 1");
        assert_eq!(node.child(1), None, "an empty slot, not an index minus one");
        let mut arena = Arena::<16>::new();
        let x = arena.var(0);
        let s = arena.constant(S);
        let gx = arena.compound(G, &[x, s, s]);
        let gs = arena.compound(G, &[s, s, s]);
        let mut bindings = [Binding::default(); 4];
        let mut trail = [0u32; 4];
        let mut six = [0u32; 6];
        assert_eq!(
            unify(gx, gs, &arena.nodes, &mut bindings, &mut trail, &mut six).0,
            UnifyResult::Unified,
            "three pairs are pushed after the one they replace was popped"
        );
        let mut bindings = [Binding::default(); 4];
        let mut five = [0u32; 5];
        assert_eq!(
            unify(gx, gs, &arena.nodes, &mut bindings, &mut trail, &mut five).0,
            UnifyResult::BoundExceeded
        );
    }

    #[test]
    fn resolution_reports_the_first_failure_that_is_not_a_clash() {
        let mut arena = Arena::<16>::new();
        let (x, y) = (arena.var(0), arena.var(1));
        let (s, t) = (arena.constant(S), arena.constant(T));
        let fst = arena.compound(F, &[s, t]);
        let gs = arena.compound(G, &[s]);
        let fxy = arena.compound(F, &[x, y]);
        let a: Clause = (literal_of_term(fst, false).unwrap(), LITERAL_NONE);
        let b: Clause = (
            literal_of_term(gs, true).unwrap(),
            literal_of_term(fxy, true).unwrap(),
        );
        let mut bindings = [Binding::default(); 4];
        let mut trail = [0u32; 4];
        // Two slots: the first pair clashes on the functor before it pushes anything; the second
        // has room for one argument pair, not two.
        let mut two = [0u32; 2];
        assert_eq!(
            resolve_first_order(a, b, &arena.nodes, &mut bindings, &mut trail, &mut two),
            Err(UnifyResult::BoundExceeded),
            "the bound, not the clash, is what stopped the proof"
        );
        let mut four = [0u32; 4];
        assert_eq!(
            resolve_first_order(a, b, &arena.nodes, &mut bindings, &mut trail, &mut four),
            Ok((LITERAL_NONE, literal_of_term(gs, true).unwrap())),
            "with room, the second pair resolves"
        );
    }

    #[test]
    fn a_child_past_the_arity_is_none_even_when_the_slot_holds_a_value() {
        let mut node = TermNode::compound(9, &[1]).unwrap();
        node.children[1] = 5;
        assert_eq!(node.child(0), Some(1));
        assert_eq!(
            node.child(1),
            None,
            "the arity bounds the children, not the array"
        );
        assert_eq!(node.child(MAX_ARITY), None);
    }

    #[test]
    fn a_stack_of_exactly_two_unifies_flat_terms_and_a_compound_needs_room_for_its_arguments() {
        let mut arena = Arena::<8>::new();
        let x = arena.var(0);
        let c = arena.constant(S);
        let mut bindings = [Binding::default(); 4];
        let mut trail = [0u32; 4];
        let mut two = [0u32; 2];
        assert_eq!(
            unify(x, c, &arena.nodes, &mut bindings, &mut trail, &mut two),
            (UnifyResult::Unified, 1),
            "two slots hold the one pair"
        );
        let mut one = [0u32; 1];
        assert_eq!(
            unify(x, c, &arena.nodes, &mut bindings, &mut trail, &mut one).0,
            UnifyResult::BoundExceeded
        );
        let fx = arena.compound(F, &[x]);
        let fc = arena.compound(F, &[c]);
        let mut bindings = [Binding::default(); 4];
        assert_eq!(
            unify(fx, fc, &arena.nodes, &mut bindings, &mut trail, &mut two).0,
            UnifyResult::Unified,
            "the pair is popped before its one argument pair is pushed: two slots suffice"
        );
        let gx = arena.compound(G, &[x, c]);
        let gc = arena.compound(G, &[c, c]);
        let mut bindings = [Binding::default(); 4];
        let mut three = [0u32; 3];
        assert_eq!(
            unify(gx, gc, &arena.nodes, &mut bindings, &mut trail, &mut three).0,
            UnifyResult::BoundExceeded,
            "two argument pairs need four slots"
        );
        let mut four = [0u32; 4];
        assert_eq!(
            unify(gx, gc, &arena.nodes, &mut bindings, &mut trail, &mut four).0,
            UnifyResult::Unified
        );
    }

    const F: u32 = 100;
    const G: u32 = 101;
    const HUMAN: u32 = 200;
    const MORTAL: u32 = 201;
    const S: u32 = 300;
    const T: u32 = 301;

    /// A small arena builder over a fixed array.
    struct Arena<const N: usize> {
        nodes: [TermNode; N],
        len: usize,
    }

    impl<const N: usize> Arena<N> {
        fn new() -> Self {
            Self {
                nodes: [TermNode::default(); N],
                len: 0,
            }
        }

        fn push(&mut self, node: TermNode) -> u32 {
            let i = self.len;
            self.nodes[i] = node;
            self.len += 1;
            i as u32
        }

        fn constant(&mut self, c: u32) -> u32 {
            self.push(TermNode::constant(c))
        }

        fn var(&mut self, v: u32) -> u32 {
            self.push(TermNode::variable(v))
        }

        fn compound(&mut self, f: u32, args: &[u32]) -> u32 {
            self.push(TermNode::compound(f, args).unwrap())
        }
    }

    fn run<const N: usize>(
        arena: &Arena<N>,
        a: u32,
        b: u32,
        bindings: &mut [Binding],
    ) -> (UnifyResult, usize) {
        let mut trail = [0u32; 16];
        let mut stack = [0u32; 64];
        unify(
            a,
            b,
            &arena.nodes[..arena.len],
            bindings,
            &mut trail,
            &mut stack,
        )
    }

    #[test]
    fn the_record_is_one_cache_line_and_a_zeroed_node_is_empty() {
        assert_eq!(core::mem::size_of::<TermNode>(), 64);
        let d = TermNode::default();
        assert_eq!((d.kind, d.arity, d.child(0)), (TERM_EMPTY, 0, None));
        let c = TermNode::compound(F, &[0, 5]).unwrap();
        assert_eq!(
            (c.child(0), c.child(1), c.child(2)),
            (Some(0), Some(5), None)
        );
        assert_eq!(c.children[0], 1, "index + 1");
        assert!(TermNode::compound(F, &[0; 9]).is_none(), "nine arguments");
        assert!(TermNode::compound(F, &[u32::MAX]).is_none());
        assert_eq!(Binding::UNBOUND.term(), None);
        assert_eq!(Binding(4).term(), Some(3));
    }

    #[test]
    fn constants_unify_with_themselves_only() {
        let mut arena = Arena::<4>::new();
        let s = arena.constant(S);
        let s2 = arena.constant(S);
        let t = arena.constant(T);
        let mut bindings = [Binding::UNBOUND; 2];
        assert_eq!(run(&arena, s, s2, &mut bindings), (UnifyResult::Unified, 0));
        assert_eq!(run(&arena, s, s, &mut bindings), (UnifyResult::Unified, 0));
        assert_eq!(run(&arena, s, t, &mut bindings), (UnifyResult::Clash, 0));
        let fs = arena.compound(F, &[s]);
        assert_eq!(run(&arena, s, fs, &mut bindings), (UnifyResult::Clash, 0));
    }

    #[test]
    fn a_variable_binds_and_is_dereferenced_through_a_chain() {
        let mut arena = Arena::<6>::new();
        let x = arena.var(0);
        let y = arena.var(1);
        let s = arena.constant(S);
        let mut bindings = [Binding::UNBOUND; 2];
        assert_eq!(run(&arena, x, y, &mut bindings), (UnifyResult::Unified, 1));
        assert_eq!(run(&arena, y, s, &mut bindings), (UnifyResult::Unified, 1));
        assert_eq!(deref(x, &arena.nodes, &bindings), Some(s), "x -> y -> s");
        assert_eq!(bindings[0].term(), Some(y));
        assert_eq!(bindings[1].term(), Some(s));
        // Bound the same way again: nothing new is bound.
        assert_eq!(run(&arena, x, s, &mut bindings), (UnifyResult::Unified, 0));
        let t = arena.constant(T);
        assert_eq!(run(&arena, x, t, &mut bindings), (UnifyResult::Clash, 0));
        assert_eq!(
            deref(x, &arena.nodes, &bindings),
            Some(s),
            "the clash changed nothing"
        );
    }

    #[test]
    fn the_occurs_check_refuses_x_equals_f_of_x_and_its_indirect_forms() {
        let mut arena = Arena::<8>::new();
        let x = arena.var(0);
        let fx = arena.compound(F, &[x]);
        let mut bindings = [Binding::UNBOUND; 2];
        assert_eq!(
            run(&arena, x, fx, &mut bindings),
            (UnifyResult::OccursCheck, 0)
        );
        assert_eq!(bindings[0], Binding::UNBOUND);
        // Indirect: X = Y, then Y = g(f(X)).
        let y = arena.var(1);
        let gfx = arena.compound(G, &[fx]);
        assert_eq!(run(&arena, x, y, &mut bindings), (UnifyResult::Unified, 1));
        assert_eq!(
            run(&arena, y, gfx, &mut bindings),
            (UnifyResult::OccursCheck, 0)
        );
        assert_eq!(bindings[1], Binding::UNBOUND, "undone");
        assert_eq!(bindings[0].term(), Some(y), "the earlier binding stands");
    }

    #[test]
    fn nested_compounds_unify_with_consistent_bindings_and_refuse_inconsistent_ones() {
        let mut arena = Arena::<16>::new();
        let x = arena.var(0);
        let y = arena.var(1);
        let s = arena.constant(S);
        let t = arena.constant(T);
        // f(g(X, s), Y)  with  f(g(t, Y), s): X = t, Y = s, Y = s.
        let gxs = arena.compound(G, &[x, s]);
        let left = arena.compound(F, &[gxs, y]);
        let gty = arena.compound(G, &[t, y]);
        let right = arena.compound(F, &[gty, s]);
        let mut bindings = [Binding::UNBOUND; 2];
        assert_eq!(
            run(&arena, left, right, &mut bindings),
            (UnifyResult::Unified, 2)
        );
        assert_eq!(deref(x, &arena.nodes, &bindings), Some(t));
        assert_eq!(deref(y, &arena.nodes, &bindings), Some(s));
        // f(X, X) with f(s, t): X = s, then s against t clashes; the trail is undone.
        let fxx = arena.compound(F, &[x, x]);
        let fst = arena.compound(F, &[s, t]);
        let mut fresh = [Binding::UNBOUND; 2];
        assert_eq!(run(&arena, fxx, fst, &mut fresh), (UnifyResult::Clash, 0));
        assert_eq!(fresh, [Binding::UNBOUND; 2], "undone on failure");
        // Arity and functor clashes.
        let fs = arena.compound(F, &[s]);
        assert_eq!(run(&arena, fs, fst, &mut fresh), (UnifyResult::Clash, 0));
        let gst = arena.compound(G, &[s, t]);
        assert_eq!(run(&arena, gst, fst, &mut fresh), (UnifyResult::Clash, 0));
    }

    #[test]
    fn an_exceeded_bound_is_reported_not_overflowed_and_leaves_no_binding() {
        let mut arena = Arena::<16>::new();
        let x = arena.var(0);
        let y = arena.var(1);
        let s = arena.constant(S);
        let t = arena.constant(T);
        let deep_a = arena.compound(F, &[x, y, s, t]);
        let deep_b = arena.compound(F, &[s, t, s, t]);
        let mut bindings = [Binding::UNBOUND; 2];
        let mut trail = [0u32; 16];
        let mut tiny = [0u32; 4];
        assert_eq!(
            unify(
                deep_a,
                deep_b,
                &arena.nodes,
                &mut bindings,
                &mut trail,
                &mut tiny
            ),
            (UnifyResult::BoundExceeded, 0)
        );
        assert_eq!(bindings, [Binding::UNBOUND; 2]);
        let mut one_trail = [0u32; 1];
        let mut stack = [0u32; 64];
        assert_eq!(
            unify(
                deep_a,
                deep_b,
                &arena.nodes,
                &mut bindings,
                &mut one_trail,
                &mut stack
            ),
            (UnifyResult::BoundExceeded, 0),
            "two variables need two trail entries"
        );
        assert_eq!(bindings, [Binding::UNBOUND; 2]);
        let mut none = [0u32; 0];
        assert_eq!(
            unify(s, s, &arena.nodes, &mut bindings, &mut trail, &mut none).0,
            UnifyResult::BoundExceeded
        );
        // Malformed inputs are results too.
        assert_eq!(
            unify(99, s, &arena.nodes, &mut bindings, &mut trail, &mut stack).0,
            UnifyResult::Malformed
        );
        let z = arena.var(7);
        assert_eq!(
            unify(z, s, &arena.nodes, &mut bindings, &mut trail, &mut stack).0,
            UnifyResult::Malformed,
            "a variable outside the table"
        );
    }

    #[test]
    fn socrates_is_mortal_by_two_first_order_resolution_steps() {
        let mut arena = Arena::<8>::new();
        let x = arena.var(0);
        let s = arena.constant(S);
        let human_x = arena.compound(HUMAN, &[x]);
        let mortal_x = arena.compound(MORTAL, &[x]);
        let human_s = arena.compound(HUMAN, &[s]);
        let mortal_s = arena.compound(MORTAL, &[s]);
        // mortal(X) :- human(X)  is  {¬human(X), mortal(X)}; human(s); the negated conjecture ¬mortal(s).
        let rule: Clause = (
            literal_of_term(human_x, true).unwrap(),
            literal_of_term(mortal_x, false).unwrap(),
        );
        let fact: Clause = (literal_of_term(human_s, false).unwrap(), LITERAL_NONE);
        let goal: Clause = (literal_of_term(mortal_s, true).unwrap(), LITERAL_NONE);
        let mut bindings = [Binding::UNBOUND; 1];
        let mut trail = [0u32; 8];
        let mut stack = [0u32; 32];
        let nodes = &arena.nodes[..arena.len];
        let step1 = resolve_first_order(rule, fact, nodes, &mut bindings, &mut trail, &mut stack)
            .expect("human(X) unifies with human(s)");
        assert_eq!(
            step1,
            (literal_of_term(mortal_x, false).unwrap(), LITERAL_NONE)
        );
        assert_eq!(deref(x, nodes, &bindings), Some(s), "X := s");
        let mut node = SymbolicRuleNode::default();
        assert!(node.record_resolvent(step1, 0, 1, 0));
        assert!(!node.is_refutation());
        let step2 = resolve_first_order(step1, goal, nodes, &mut bindings, &mut trail, &mut stack)
            .expect("mortal(X)[X := s] unifies with mortal(s)");
        assert_eq!(step2, EMPTY_CLAUSE);
        let mut proof = SymbolicRuleNode::default();
        assert!(proof.record_resolvent(step2, 2, 3, node.proof_depth));
        assert!(
            proof.is_refutation(),
            "the empty clause: Socrates is mortal"
        );
        assert_eq!(proof.proof_depth, 2);
        // Wrong constant: no step.
        let t = arena.constant(T);
        let mortal_t = arena.compound(MORTAL, &[t]);
        let other_goal: Clause = (literal_of_term(mortal_t, true).unwrap(), LITERAL_NONE);
        assert_eq!(
            resolve_first_order(
                step1,
                other_goal,
                &arena.nodes[..arena.len],
                &mut bindings,
                &mut trail,
                &mut stack
            ),
            Err(UnifyResult::Clash)
        );
        // Same signs never resolve.
        assert_eq!(
            resolve_first_order(
                fact,
                fact,
                &arena.nodes[..arena.len],
                &mut bindings,
                &mut trail,
                &mut stack
            ),
            Err(UnifyResult::Clash)
        );
        assert_eq!(term_of_literal(LITERAL_NONE), None);
        assert_eq!(term_of_literal(literal_of_term(7, true).unwrap()), Some(7));
        assert!(is_negated(literal_of_term(7, true).unwrap()));
    }

    #[test]
    fn a_literal_the_encoding_cannot_hold_is_refused_and_the_last_one_round_trips() {
        let last = LITERAL_NEGATED - 2;
        let lit = literal_of_term(last, false).unwrap();
        assert_eq!(term_of_literal(lit), Some(last));
        assert!(!is_negated(lit));
        let neg = literal_of_term(last, true).unwrap();
        assert_eq!(term_of_literal(neg), Some(last));
        assert!(is_negated(neg));
        assert_eq!(
            literal_of_term(LITERAL_NEGATED - 1, false),
            None,
            "would reach the sign bit"
        );
        assert_eq!(literal_of_term(u32::MAX, true), None, "would overflow");
    }

    #[test]
    fn unification_is_deterministic() {
        let mut arena = Arena::<16>::new();
        let x = arena.var(0);
        let y = arena.var(1);
        let z = arena.var(2);
        let s = arena.constant(S);
        let gyz = arena.compound(G, &[y, z]);
        let left = arena.compound(F, &[x, gyz, z]);
        let gs = arena.compound(G, &[s, s]);
        let right = arena.compound(F, &[gs, x, y]);
        let mut first = [Binding::UNBOUND; 3];
        let mut second = [Binding::UNBOUND; 3];
        let a = run(&arena, left, right, &mut first);
        let b = run(&arena, left, right, &mut second);
        assert_eq!(a, b);
        assert_eq!(first, second);
        assert_eq!(a.0, UnifyResult::Unified);
        for v in [x, y, z] {
            assert_eq!(
                deref(v, &arena.nodes, &first),
                deref(v, &arena.nodes, &second)
            );
        }
    }
}
