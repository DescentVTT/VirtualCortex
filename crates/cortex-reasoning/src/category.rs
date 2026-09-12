//! Categorial reduction over the term arena (whitepaper §5.2.30, §6.9, §8.8; ADR-0040; brief
//! 020). Syntax as type reduction (Steedman): a category is a term. An atomic category is a
//! constant or a compound whose children carry its features and its head (`S(chase)`,
//! `NP(dog)`); a functor category `X/Y` or `X\Y` is a compound over [`CATEGORY_FORWARD`] or
//! [`CATEGORY_BACKWARD`] with three children, the result `X`, the argument `Y` and the role the
//! argument fills in the result (a constant of the caller's role vocabulary, or a variable for
//! "whatever the complement says"). The four combinatory rules are unifications over the
//! categories the two top items of a parse stack dereference to: forward and backward
//! application, forward and backward harmonic composition, in that order. [`reduce`] is a
//! shift-reduce reducer over caller-provided slices: it shifts each lexical category, reduces
//! the top two while a rule applies, logs every reduction, and returns the one category left
//! or why there is not one; every bound is a result. It is greedy: it commits to the first
//! applicable rule and never backtracks, so a sequence whose only derivation needs a later
//! reduction before an earlier one is `NoDerivation`. Type raising and a chart are Specified.
//! Variables are numbered by the caller, who instantiates each lexical entry with fresh ones
//! (standardising apart, ADR-0025) and owns the binding table the parse accumulates into.

use crate::term::{
    Binding, TERM_COMPOUND, TERM_CONSTANT, TermNode, UnifyResult, deref, undo, unify,
};

/// The functor of a forward-slash category `X/Y`: `CATEGORY_FORWARD(X, Y, role)`.
pub const CATEGORY_FORWARD: u32 = 0xFFFF_FF01;
/// The functor of a backward-slash category `X\Y`: `CATEGORY_BACKWARD(X, Y, role)`.
pub const CATEGORY_BACKWARD: u32 = 0xFFFF_FF02;
/// The lowest functor id the grammar reserves; a caller's concept ids stay below it.
pub const CATEGORY_RESERVED: u32 = 0xFFFF_FF00;
/// The children of a functor category: the result, the argument and the argument's role.
pub const SLASH_ARITY: u8 = 3;

/// `X/Y  Y' ⇒ X` when `Y` unifies with `Y'`.
pub const RULE_FORWARD_APPLICATION: u8 = 1;
/// `Y'  X\Y ⇒ X` when `Y` unifies with `Y'`.
pub const RULE_BACKWARD_APPLICATION: u8 = 2;
/// `X/Y  Y'/Z ⇒ X/Z` when `Y` unifies with `Y'`; the new slash carries `Z`'s role.
pub const RULE_FORWARD_COMPOSITION: u8 = 3;
/// `Y'\Z  X\Y ⇒ X\Z` when `Y` unifies with `Y'`; the new slash carries `Z`'s role.
pub const RULE_BACKWARD_COMPOSITION: u8 = 4;

/// One reduction the reducer made: the rule, the two stack items it consumed (term indices as
/// they stood on the stack), the result it pushed, the role of the argument the rule consumed
/// (the third child of the functor whose argument was unified; a term index the caller
/// dereferences) and the category that filled it (the other item, or the right functor of a
/// composition, whose result is what fills the role).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Reduction {
    pub rule: u8,
    pub left: u32,
    pub right: u32,
    pub result: u32,
    pub role: u32,
    pub argument: u32,
}

/// Why a sequence did not reduce to one category. Every value leaves the arena as it was;
/// the bindings the reductions before it made stay in the table, and the trail says which.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// No category to reduce.
    Empty,
    /// The sequence was shifted whole and more than one category remains on the stack.
    NoDerivation { remaining: usize },
    /// The parse stack is too small for the sequence.
    StackFull,
    /// A composition needed a new node and the arena has no free one.
    ArenaFull,
    /// The step log is too small for the derivation.
    StepsFull,
    /// The unification stack or the trail is too small for these categories.
    BoundExceeded,
    /// An index outside the arena or the table, an empty node, or a functor category without
    /// its three children.
    Malformed,
}

/// The caller's slices a reduction runs over (TC-5: nothing is allocated). The arena is read
/// and, for a composition, appended to at `free`; the bindings accumulate the parse's one
/// substitution, its variables pushed onto the trail from `trail_len`; `stack` is
/// unification's work stack; `parse` is the reducer's stack of categories; `steps` the log.
pub struct ParseScratch<'a> {
    pub arena: &'a mut [TermNode],
    pub free: usize,
    pub bindings: &'a mut [Binding],
    pub trail: &'a mut [u32],
    pub trail_len: usize,
    pub stack: &'a mut [u32],
    pub parse: &'a mut [u32],
    pub steps: &'a mut [Reduction],
    pub step_count: usize,
}

/// A forward-slash category `result/argument`, the argument filling `role`. `None` for an
/// index the encoding cannot hold.
pub fn forward(result: u32, argument: u32, role: u32) -> Option<TermNode> {
    TermNode::compound(CATEGORY_FORWARD, &[result, argument, role])
}

/// A backward-slash category `result\argument`, the argument filling `role`.
pub fn backward(result: u32, argument: u32, role: u32) -> Option<TermNode> {
    TermNode::compound(CATEGORY_BACKWARD, &[result, argument, role])
}

/// The slash of a functor category (`CATEGORY_FORWARD` or `CATEGORY_BACKWARD`), or `None` for
/// an atomic category or an empty node. A reserved functor with the wrong arity is reported by
/// [`reduce`] as `Malformed`, not hidden here.
pub const fn slash(node: &TermNode) -> Option<u32> {
    if node.kind == TERM_COMPOUND
        && (node.functor == CATEGORY_FORWARD || node.functor == CATEGORY_BACKWARD)
    {
        Some(node.functor)
    } else {
        None
    }
}

/// True for a functor category.
pub const fn is_functor(node: &TermNode) -> bool {
    slash(node).is_some()
}

/// The result `X` of a functor category `X/Y` or `X\Y`.
pub const fn result(node: &TermNode) -> Option<u32> {
    if is_functor(node) {
        node.child(0)
    } else {
        None
    }
}

/// The argument `Y` of a functor category.
pub const fn argument(node: &TermNode) -> Option<u32> {
    if is_functor(node) {
        node.child(1)
    } else {
        None
    }
}

/// The role the argument of a functor category fills.
pub const fn role(node: &TermNode) -> Option<u32> {
    if is_functor(node) {
        node.child(2)
    } else {
        None
    }
}

/// The head of a category under the bindings: for a functor, the head of its result; for a
/// compound, its first child dereferenced to a constant, whose concept id this is; `None` for
/// a constant category, a variable, a compound with no child or a child that is not a
/// constant, or a malformed index. Bounded by the arena.
pub fn head(term: u32, arena: &[TermNode], bindings: &[Binding]) -> Option<u32> {
    let mut current = term;
    for _ in 0..=arena.len() {
        let index = deref(current, arena, bindings)?;
        let node = arena.get(index as usize)?;
        match result(node) {
            Some(inner) => current = inner,
            None => {
                if node.kind != TERM_COMPOUND {
                    return None;
                }
                let first = deref(node.child(0)?, arena, bindings)?;
                let feature = arena.get(first as usize)?;
                return (feature.kind == TERM_CONSTANT).then_some(feature.functor);
            }
        }
    }
    None
}

/// Shift-reduce reduction of `categories` (term indices of the lexical categories, in order)
/// to one category, whose index is returned; the reductions are in `scratch.steps[..step_count]`
/// and the substitution in the table. After every shift the top two items are reduced while a
/// rule applies: forward application, backward application, forward composition, backward
/// composition, the first that unifies. A rule whose unification clashes or fails the occurs
/// check does not apply; a bound that is exceeded, a malformed index or node, a full stack,
/// arena or log is the error, with the arena and the bindings as the reductions before it
/// left them (a reduction that could not be completed is undone).
pub fn reduce(categories: &[u32], scratch: &mut ParseScratch) -> Result<u32, ParseError> {
    if categories.is_empty() {
        return Err(ParseError::Empty);
    }
    let mut top = 0usize;
    for &category in categories {
        if top >= scratch.parse.len() {
            return Err(ParseError::StackFull);
        }
        scratch.parse[top] = category;
        // Below the stack's length after the check above.
        top = top.wrapping_add(1);
        while top >= 2 {
            // Both indices are inside the stack: `top` is at least 2.
            let left = scratch.parse[top.wrapping_sub(2)];
            let right = scratch.parse[top.wrapping_sub(1)];
            match try_rules(left, right, scratch)? {
                Some((reduction, bound)) => {
                    if scratch.step_count >= scratch.steps.len() {
                        retract(&reduction, bound, scratch);
                        return Err(ParseError::StepsFull);
                    }
                    scratch.steps[scratch.step_count] = reduction;
                    scratch.step_count = scratch.step_count.wrapping_add(1);
                    scratch.parse[top.wrapping_sub(2)] = reduction.result;
                    top = top.wrapping_sub(1);
                }
                None => break,
            }
        }
    }
    if top == 1 {
        Ok(scratch.parse[0])
    } else {
        Err(ParseError::NoDerivation { remaining: top })
    }
}

/// The first rule that applies to the pair, with the number of variables its unification
/// bound, or `None`.
fn try_rules(
    left: u32,
    right: u32,
    scratch: &mut ParseScratch,
) -> Result<Option<(Reduction, usize)>, ParseError> {
    let (Some(l), Some(r)) = (
        deref(left, scratch.arena, scratch.bindings),
        deref(right, scratch.arena, scratch.bindings),
    ) else {
        return Err(ParseError::Malformed);
    };
    let ln = scratch.arena[l as usize];
    let rn = scratch.arena[r as usize];
    let left_slash = functor_parts(&ln)?;
    let right_slash = functor_parts(&rn)?;

    // Forward application: `X/Y  Y' ⇒ X`. (Nested `if let`s rather than a `let` chain: the
    // chain is stable from Rust 1.88 and the floor is 1.85, ADR-0009.)
    if let Some((CATEGORY_FORWARD, x, y, rho)) = left_slash {
        if let Some(bound) = unify_in(y, r, scratch)? {
            let reduction = Reduction {
                rule: RULE_FORWARD_APPLICATION,
                left,
                right,
                result: x,
                role: rho,
                argument: r,
            };
            return Ok(Some((reduction, bound)));
        }
    }
    // Backward application: `Y'  X\Y ⇒ X`.
    if let Some((CATEGORY_BACKWARD, x, y, rho)) = right_slash {
        if let Some(bound) = unify_in(y, l, scratch)? {
            let reduction = Reduction {
                rule: RULE_BACKWARD_APPLICATION,
                left,
                right,
                result: x,
                role: rho,
                argument: l,
            };
            return Ok(Some((reduction, bound)));
        }
    }
    // Forward composition: `X/Y  Y'/Z ⇒ X/Z`, the new slash carrying `Z`'s role.
    if let (Some((CATEGORY_FORWARD, x, y, rho)), Some((CATEGORY_FORWARD, y2, z, rho2))) =
        (left_slash, right_slash)
    {
        if let Some(bound) = unify_in(y, y2, scratch)? {
            let composed = push(forward(x, z, rho2), bound, scratch)?;
            let reduction = Reduction {
                rule: RULE_FORWARD_COMPOSITION,
                left,
                right,
                result: composed,
                role: rho,
                argument: r,
            };
            return Ok(Some((reduction, bound)));
        }
    }
    // Backward composition: `Y'\Z  X\Y ⇒ X\Z`, the new slash carrying `Z`'s role.
    if let (Some((CATEGORY_BACKWARD, y2, z, rho2)), Some((CATEGORY_BACKWARD, x, y, rho))) =
        (left_slash, right_slash)
    {
        if let Some(bound) = unify_in(y, y2, scratch)? {
            let composed = push(backward(x, z, rho2), bound, scratch)?;
            let reduction = Reduction {
                rule: RULE_BACKWARD_COMPOSITION,
                left,
                right,
                result: composed,
                role: rho,
                argument: l,
            };
            return Ok(Some((reduction, bound)));
        }
    }
    Ok(None)
}

/// The slash, result, argument and role of a functor category; `None` for an atomic one;
/// `Malformed` for a reserved functor without its three children.
fn functor_parts(node: &TermNode) -> Result<Option<(u32, u32, u32, u32)>, ParseError> {
    match slash(node) {
        None => Ok(None),
        Some(s) => match (node.arity, result(node), argument(node), role(node)) {
            (SLASH_ARITY, Some(x), Some(y), Some(rho)) => Ok(Some((s, x, y, rho))),
            _ => Err(ParseError::Malformed),
        },
    }
}

/// Unifies two categories in the scratch, the variables bound going onto the trail after the
/// ones already there. `Ok(Some(bound))` when they unify, with how many variables were bound;
/// `Ok(None)` on a clash or an occurs failure (the rule does not apply); the error for a bound
/// or a malformed index.
fn unify_in(a: u32, b: u32, scratch: &mut ParseScratch) -> Result<Option<usize>, ParseError> {
    if scratch.trail_len > scratch.trail.len() {
        return Err(ParseError::BoundExceeded);
    }
    let (_, free_trail) = scratch.trail.split_at_mut(scratch.trail_len);
    let (outcome, bound) = unify(
        a,
        b,
        scratch.arena,
        scratch.bindings,
        free_trail,
        scratch.stack,
    );
    match outcome {
        UnifyResult::Unified => {
            // At most the free trail's length, so within the trail.
            scratch.trail_len = scratch.trail_len.wrapping_add(bound);
            Ok(Some(bound))
        }
        UnifyResult::Clash | UnifyResult::OccursCheck => Ok(None),
        UnifyResult::BoundExceeded => Err(ParseError::BoundExceeded),
        UnifyResult::Malformed => Err(ParseError::Malformed),
    }
}

/// Appends a node at the arena's free cursor and returns its index; when there is no room,
/// the `bound` variables the rule just bound are undone before the error.
fn push(
    node: Option<TermNode>,
    bound: usize,
    scratch: &mut ParseScratch,
) -> Result<u32, ParseError> {
    let error = if node.is_none() {
        ParseError::Malformed
    } else if scratch.free >= scratch.arena.len() || scratch.free >= u32::MAX as usize {
        ParseError::ArenaFull
    } else {
        // Refused above unless the node is there and the cursor is inside the arena.
        scratch.arena[scratch.free] = node.unwrap_or_default();
        let index = scratch.free as u32;
        scratch.free = scratch.free.wrapping_add(1);
        return Ok(index);
    };
    retract_bindings(bound, scratch);
    Err(error)
}

/// Undoes the last `bound` variables the parse bound.
fn retract_bindings(bound: usize, scratch: &mut ParseScratch) {
    let start = scratch.trail_len.saturating_sub(bound);
    undo(scratch.bindings, &scratch.trail[start..], bound);
    scratch.trail_len = start;
}

/// Undoes a reduction that could not be logged: its bindings, and the node a composition
/// pushed, which is the last one in the arena.
fn retract(reduction: &Reduction, bound: usize, scratch: &mut ParseScratch) {
    retract_bindings(bound, scratch);
    if matches!(
        reduction.rule,
        RULE_FORWARD_COMPOSITION | RULE_BACKWARD_COMPOSITION
    ) {
        // A composition pushed one node, so the cursor is at least one; the checked form
        // says so without a comparison of its own.
        if let Some(last) = scratch.free.checked_sub(1) {
            scratch.free = last;
            scratch.arena[last] = TermNode::default();
        }
    }
}

const _: () = {
    assert!(CATEGORY_FORWARD > CATEGORY_RESERVED && CATEGORY_BACKWARD > CATEGORY_RESERVED);
    assert!(CATEGORY_FORWARD != CATEGORY_BACKWARD);
    assert!(SLASH_ARITY as usize <= crate::term::MAX_ARITY);
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::{TERM_NONE, TERM_VARIABLE, undo};

    // Atomic category functors.
    const S: u32 = 10;
    const NP: u32 = 11;
    const N: u32 = 12;
    // Concepts.
    const DOG: u32 = 100;
    const CAT: u32 = 101;
    const CHASE: u32 = 102;
    const CAN: u32 = 103;
    const QUICKLY: u32 = 104;
    // Roles.
    const AGENT: u32 = 900;
    const PATIENT: u32 = 901;
    const NONE: u32 = 902;

    /// A small lexicon over a fixed arena: every entry is instantiated with fresh variables.
    struct Lexicon<const N_: usize> {
        nodes: [TermNode; N_],
        len: usize,
        vars: u32,
    }

    impl<const N_: usize> Lexicon<N_> {
        fn new() -> Self {
            Self {
                nodes: [TermNode::default(); N_],
                len: 0,
                vars: 0,
            }
        }

        fn push(&mut self, node: TermNode) -> u32 {
            let i = self.len;
            self.nodes[i] = node;
            self.len = self.len.wrapping_add(1);
            i as u32
        }

        fn var(&mut self) -> u32 {
            let v = self.vars;
            self.vars = self.vars.wrapping_add(1);
            self.push(TermNode::variable(v))
        }

        fn constant(&mut self, c: u32) -> u32 {
            self.push(TermNode::constant(c))
        }

        fn atom(&mut self, functor: u32, feature: u32) -> u32 {
            self.push(TermNode::compound(functor, &[feature]).unwrap())
        }

        fn fwd(&mut self, x: u32, y: u32, rho: u32) -> u32 {
            self.push(forward(x, y, rho).unwrap())
        }

        fn bwd(&mut self, x: u32, y: u32, rho: u32) -> u32 {
            self.push(backward(x, y, rho).unwrap())
        }

        /// A determiner: `NP(X)/N(X)`, the head passing up, no role.
        fn the(&mut self) -> u32 {
            let x = self.var();
            let np = self.atom(NP, x);
            let n = self.atom(N, x);
            let none = self.constant(NONE);
            self.fwd(np, n, none)
        }

        /// A noun: `N(c)`.
        fn noun(&mut self, c: u32) -> u32 {
            let head = self.constant(c);
            self.atom(N, head)
        }

        /// A bare noun phrase: `NP(c)`.
        fn np(&mut self, c: u32) -> u32 {
            let head = self.constant(c);
            self.atom(NP, head)
        }

        /// A transitive verb: `(S(v)\NP(A):agent)/NP(P):patient`.
        fn transitive(&mut self, v: u32) -> u32 {
            let head = self.constant(v);
            let s = self.atom(S, head);
            let a = self.var();
            let np_a = self.atom(NP, a);
            let agent = self.constant(AGENT);
            let s_np = self.bwd(s, np_a, agent);
            let p = self.var();
            let np_p = self.atom(NP, p);
            let patient = self.constant(PATIENT);
            self.fwd(s_np, np_p, patient)
        }

        /// A modal: `(S(V)\NP(A):R)/(S(V)\NP(A):R)`, the subject and its role passed through.
        fn modal(&mut self) -> u32 {
            let v = self.var();
            let a = self.var();
            let r = self.var();
            let s = self.atom(S, v);
            let np_a = self.atom(NP, a);
            let inner = self.bwd(s, np_a, r);
            let s2 = self.atom(S, v);
            let np_a2 = self.atom(NP, a);
            let outer = self.bwd(s2, np_a2, r);
            let none = self.constant(NONE);
            self.fwd(outer, inner, none)
        }

        /// A sentence-final adverb: `S(V)\S(V)`, no role.
        fn adverb(&mut self) -> u32 {
            let v = self.var();
            let s = self.atom(S, v);
            let s2 = self.atom(S, v);
            let none = self.constant(NONE);
            self.bwd(s, s2, none)
        }
    }

    /// Runs a reduction over fresh scratch slices and returns the outcome and the log.
    fn run<const N_: usize>(
        lex: &mut Lexicon<N_>,
        categories: &[u32],
    ) -> (
        Result<u32, ParseError>,
        [Reduction; 8],
        usize,
        [Binding; 32],
    ) {
        let mut bindings = [Binding::UNBOUND; 32];
        let mut trail = [0u32; 32];
        let mut stack = [0u32; 32];
        let mut parse = [0u32; 8];
        let mut steps = [Reduction::default(); 8];
        let free = lex.len;
        let mut scratch = ParseScratch {
            arena: &mut lex.nodes,
            free,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut steps,
            step_count: 0,
        };
        let outcome = reduce(categories, &mut scratch);
        let count = scratch.step_count;
        lex.len = scratch.free;
        (outcome, steps, count, bindings)
    }

    fn constant_of(term: u32, arena: &[TermNode], bindings: &[Binding]) -> Option<u32> {
        let i = deref(term, arena, bindings)?;
        let node = arena[i as usize];
        (node.kind == TERM_CONSTANT).then_some(node.functor)
    }

    #[test]
    fn a_functor_category_has_a_slash_a_result_an_argument_and_a_role() {
        let f = forward(1, 2, 3).unwrap();
        let b = backward(4, 5, 6).unwrap();
        assert_eq!(
            (slash(&f), slash(&b)),
            (Some(CATEGORY_FORWARD), Some(CATEGORY_BACKWARD))
        );
        assert!(is_functor(&f) && is_functor(&b));
        assert_eq!(
            (result(&f), argument(&f), role(&f)),
            (Some(1), Some(2), Some(3))
        );
        assert_eq!(
            (result(&b), argument(&b), role(&b)),
            (Some(4), Some(5), Some(6))
        );
        assert_eq!(f.arity, SLASH_ARITY);
        let atom = TermNode::compound(NP, &[7]).unwrap();
        assert_eq!(slash(&atom), None);
        assert!(!is_functor(&atom));
        assert_eq!(
            (result(&atom), argument(&atom), role(&atom)),
            (None, None, None)
        );
        assert_eq!(slash(&TermNode::constant(S)), None);
        assert_eq!(
            slash(&TermNode::default()),
            None,
            "an empty node is no category"
        );
        assert_eq!(
            forward(u32::MAX, 2, 3),
            None,
            "an index the encoding cannot hold"
        );
    }

    #[test]
    fn forward_application_consumes_the_argument_to_the_right_and_reports_its_role() {
        let mut lex = Lexicon::<32>::new();
        let the = lex.the();
        let dog = lex.noun(DOG);
        let (outcome, steps, count, bindings) = run(&mut lex, &[the, dog]);
        let root = outcome.unwrap();
        assert_eq!(count, 1);
        assert_eq!(steps[0].rule, RULE_FORWARD_APPLICATION);
        assert_eq!((steps[0].left, steps[0].right), (the, dog));
        assert_eq!(steps[0].argument, dog);
        assert_eq!(
            constant_of(steps[0].role, &lex.nodes, &bindings),
            Some(NONE)
        );
        let np = lex.nodes[root as usize];
        assert_eq!(
            (np.kind, np.functor),
            (TERM_COMPOUND, NP),
            "the result is NP(X)"
        );
        assert_eq!(
            head(root, &lex.nodes, &bindings),
            Some(DOG),
            "with X bound to dog"
        );
        assert!(
            bindings[0].term().is_some(),
            "the determiner's variable is bound"
        );
    }

    #[test]
    fn backward_application_consumes_the_argument_to_the_left() {
        let mut lex = Lexicon::<32>::new();
        let dog = lex.np(DOG);
        let head_v = lex.constant(CHASE);
        let s = lex.atom(S, head_v);
        let a = lex.var();
        let np_a = lex.atom(NP, a);
        let agent = lex.constant(AGENT);
        let intransitive = lex.bwd(s, np_a, agent);
        let (outcome, steps, count, bindings) = run(&mut lex, &[dog, intransitive]);
        assert_eq!(outcome, Ok(s));
        assert_eq!(count, 1);
        assert_eq!(steps[0].rule, RULE_BACKWARD_APPLICATION);
        assert_eq!(
            (steps[0].left, steps[0].right, steps[0].result),
            (dog, intransitive, s)
        );
        assert_eq!(steps[0].argument, dog);
        assert_eq!(
            constant_of(steps[0].role, &lex.nodes, &bindings),
            Some(AGENT)
        );
        assert_eq!(head(steps[0].argument, &lex.nodes, &bindings), Some(DOG));
        assert_eq!(head(s, &lex.nodes, &bindings), Some(CHASE));
    }

    #[test]
    fn a_transitive_sentence_with_determiners_reduces_to_s_and_the_log_names_both_heads() {
        // the dog chased the cat: NP/N N (S\NP)/NP NP/N N.
        let mut lex = Lexicon::<64>::new();
        let the1 = lex.the();
        let dog = lex.noun(DOG);
        let chased = lex.transitive(CHASE);
        let the2 = lex.the();
        let cat = lex.noun(CAT);
        let free_before = lex.len;
        let (outcome, steps, count, bindings) = run(&mut lex, &[the1, dog, chased, the2, cat]);
        let root = outcome.unwrap();
        assert_eq!(head(root, &lex.nodes, &bindings), Some(CHASE), "S(chase)");
        assert_eq!(
            lex.nodes[deref(root, &lex.nodes, &bindings).unwrap() as usize].functor,
            S
        );
        assert_eq!(count, 4, "four reductions for five categories");
        let rules: [u8; 4] = core::array::from_fn(|i| steps[i].rule);
        assert_eq!(
            rules,
            [
                RULE_FORWARD_APPLICATION,
                RULE_FORWARD_COMPOSITION,
                RULE_FORWARD_APPLICATION,
                RULE_BACKWARD_APPLICATION
            ],
            "the dog | chased the | cat | subject"
        );
        assert_eq!(
            lex.len,
            free_before.wrapping_add(1),
            "one node for the composition"
        );
        // The roles and the heads that fill them, read through the bindings after the parse.
        let mut agent = None;
        let mut patient = None;
        for step in &steps[..count] {
            match constant_of(step.role, &lex.nodes, &bindings) {
                Some(AGENT) => agent = head(step.argument, &lex.nodes, &bindings),
                Some(PATIENT) => patient = head(step.argument, &lex.nodes, &bindings),
                _ => {}
            }
        }
        assert_eq!((agent, patient), (Some(DOG), Some(CAT)));
        assert_eq!(
            steps[1].argument, the2,
            "the composition's argument is the determiner, whose result's head is the cat"
        );
    }

    #[test]
    fn a_modal_composes_forward_with_the_verb_and_passes_the_subject_through() {
        // dogs can chase cats: NP (S\NP)/(S\NP) (S\NP)/NP NP.
        let mut lex = Lexicon::<64>::new();
        let dogs = lex.np(DOG);
        let can = lex.modal();
        let chase = lex.transitive(CHASE);
        let cats = lex.np(CAT);
        let (outcome, steps, count, bindings) = run(&mut lex, &[dogs, can, chase, cats]);
        let root = outcome.unwrap();
        assert_eq!(count, 3);
        assert_eq!(
            [steps[0].rule, steps[1].rule, steps[2].rule],
            [
                RULE_FORWARD_COMPOSITION,
                RULE_FORWARD_APPLICATION,
                RULE_BACKWARD_APPLICATION
            ]
        );
        assert_eq!(
            head(root, &lex.nodes, &bindings),
            Some(CHASE),
            "the modal's S(V) took the verb"
        );
        assert_eq!(
            constant_of(steps[1].role, &lex.nodes, &bindings),
            Some(PATIENT)
        );
        assert_eq!(head(steps[1].argument, &lex.nodes, &bindings), Some(CAT));
        assert_eq!(
            constant_of(steps[2].role, &lex.nodes, &bindings),
            Some(AGENT),
            "the modal's variable role was bound to the verb's"
        );
        assert_eq!(head(steps[2].argument, &lex.nodes, &bindings), Some(DOG));
        let _ = CAN;
    }

    #[test]
    fn an_adverb_composes_backward_with_a_verb_phrase_and_the_result_takes_its_subject_later() {
        // chase cats quickly: (S\NP)/NP NP S\S. With the subject absent, the verb phrase `S\NP`
        // and the adverb `S\S` compose to `S\NP` (with a subject present the reducer would have
        // applied it first, greedily). Then `dogs` and that phrase reduce to `S` over the same
        // scratch, the log continuing.
        let mut lex = Lexicon::<64>::new();
        let chase = lex.transitive(CHASE);
        let cats = lex.np(CAT);
        let quickly = lex.adverb();
        let dogs = lex.np(DOG);
        let mut bindings = [Binding::UNBOUND; 32];
        let mut trail = [0u32; 32];
        let mut stack = [0u32; 32];
        let mut parse = [0u32; 8];
        let mut steps = [Reduction::default(); 8];
        let free = lex.len;
        let mut scratch = ParseScratch {
            arena: &mut lex.nodes,
            free,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut steps,
            step_count: 0,
        };
        let phrase = reduce(&[chase, cats, quickly], &mut scratch).unwrap();
        assert_eq!(scratch.step_count, 2);
        assert_eq!(
            scratch.free,
            free.wrapping_add(1),
            "one node for the composition"
        );
        let sentence = reduce(&[dogs, phrase], &mut scratch).unwrap();
        assert_eq!(scratch.step_count, 3);
        // The scratch is not used again, so its borrows end here.
        let arena = &lex.nodes[..];
        let bindings = &bindings[..];
        assert_eq!(
            [steps[0].rule, steps[1].rule, steps[2].rule],
            [
                RULE_FORWARD_APPLICATION,
                RULE_BACKWARD_COMPOSITION,
                RULE_BACKWARD_APPLICATION
            ]
        );
        assert_eq!(head(phrase, arena, bindings), Some(CHASE));
        assert_eq!(head(sentence, arena, bindings), Some(CHASE));
        assert_eq!(
            arena[deref(sentence, arena, bindings).unwrap() as usize].functor,
            S
        );
        assert_eq!(
            constant_of(steps[1].role, arena, bindings),
            Some(NONE),
            "the adverb's role is what the composition consumed"
        );
        assert_eq!(
            steps[1].argument, steps[0].result,
            "the composition's argument is the verb phrase, whose result fills the adverb's slot"
        );
        assert_eq!(head(steps[1].argument, arena, bindings), Some(CHASE));
        let composed = arena[steps[1].result as usize];
        assert_eq!(slash(&composed), Some(CATEGORY_BACKWARD));
        assert_eq!(
            constant_of(role(&composed).unwrap(), arena, bindings),
            Some(AGENT),
            "the composed slash keeps the subject's role"
        );
        assert_eq!(constant_of(steps[2].role, arena, bindings), Some(AGENT));
        assert_eq!(head(steps[2].argument, arena, bindings), Some(DOG));
        assert_eq!(constant_of(steps[0].role, arena, bindings), Some(PATIENT));
        assert_eq!(head(steps[0].argument, arena, bindings), Some(CAT));
        let _ = QUICKLY;
    }

    #[test]
    fn a_non_sentence_an_empty_sequence_and_a_lone_category_are_results() {
        let mut lex = Lexicon::<64>::new();
        let dog = lex.np(DOG);
        let cat = lex.np(CAT);
        let (outcome, _, count, _) = run(&mut lex, &[dog, cat]);
        assert_eq!(outcome, Err(ParseError::NoDerivation { remaining: 2 }));
        assert_eq!(count, 0);
        let (outcome, _, _, _) = run(&mut lex, &[]);
        assert_eq!(outcome, Err(ParseError::Empty));
        let (outcome, _, count, _) = run(&mut lex, &[dog]);
        assert_eq!(outcome, Ok(dog), "one category is its own derivation");
        assert_eq!(count, 0);
        // An object relative needs a reduction the greedy reducer never makes.
        let the = lex.the();
        let cat2 = lex.noun(CAT);
        let the2 = lex.the();
        let dog2 = lex.noun(DOG);
        let chased = lex.transitive(CHASE);
        let (outcome, _, _, _) = run(&mut lex, &[the, cat2, the2, dog2, chased]);
        assert_eq!(
            outcome,
            Err(ParseError::NoDerivation { remaining: 3 }),
            "two noun phrases and the verb: the object is never type-raised"
        );
    }

    #[test]
    fn every_bound_is_a_result_and_binds_nothing_new() {
        let mut lex = Lexicon::<64>::new();
        let the = lex.the();
        let dog = lex.noun(DOG);
        let chased = lex.transitive(CHASE);
        let the2 = lex.the();
        let cat = lex.noun(CAT);
        let sentence = [the, dog, chased, the2, cat];
        let base = lex.len;
        // A parse stack of two reduces `the dog` once and fills at the fourth shift.

        let mut bindings = [Binding::UNBOUND; 32];
        let mut trail = [0u32; 32];
        let mut stack = [0u32; 32];
        let mut parse = [0u32; 2];
        let mut steps = [Reduction::default(); 8];
        let mut scratch = ParseScratch {
            arena: &mut lex.nodes,
            free: base,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut steps,
            step_count: 0,
        };
        assert_eq!(reduce(&sentence, &mut scratch), Err(ParseError::StackFull));
        assert_eq!(
            scratch.step_count, 1,
            "the first reduction happened before the stack filled"
        );
        // An arena with no free node refuses the composition, after the first application.
        let mut bindings = [Binding::UNBOUND; 32];
        let mut parse = [0u32; 8];
        let mut steps = [Reduction::default(); 8];
        let mut full = ParseScratch {
            arena: &mut lex.nodes[..base],
            free: base,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut steps,
            step_count: 0,
        };
        assert_eq!(reduce(&sentence, &mut full), Err(ParseError::ArenaFull));
        assert_eq!(full.step_count, 1);
        assert_eq!(
            full.trail_len, 1,
            "the first application bound X; the composition's binding was undone"
        );
        assert_eq!(full.free, base, "and nothing was pushed");
        assert_eq!(bindings.iter().filter(|b| b.term().is_some()).count(), 1);
        // A log of one step.
        let mut bindings = [Binding::UNBOUND; 32];
        let mut one_step = [Reduction::default(); 1];
        let mut parse = [0u32; 8];
        let mut short = ParseScratch {
            arena: &mut lex.nodes,
            free: base,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut one_step,
            step_count: 0,
        };
        assert_eq!(reduce(&sentence, &mut short), Err(ParseError::StepsFull));
        assert_eq!(short.step_count, 1);
        assert_eq!(
            (short.free, short.trail_len),
            (base, 1),
            "the composition that had no room in the log was undone, node and binding"
        );
        assert_eq!(bindings.iter().filter(|b| b.term().is_some()).count(), 1);

        // A trail too small for the bindings.
        let mut bindings = [Binding::UNBOUND; 32];
        let mut tiny_trail = [0u32; 1];
        let mut parse = [0u32; 8];
        let mut steps = [Reduction::default(); 8];
        let mut narrow = ParseScratch {
            arena: &mut lex.nodes,
            free: base,
            bindings: &mut bindings,
            trail: &mut tiny_trail,
            trail_len: 0,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut steps,
            step_count: 0,
        };
        assert_eq!(
            reduce(&sentence, &mut narrow),
            Err(ParseError::BoundExceeded)
        );
        assert_eq!(
            bindings.iter().filter(|b| b.term().is_some()).count(),
            1,
            "the first application bound one variable; the composition found no trail"
        );
        // A unification stack too small.
        let mut bindings = [Binding::UNBOUND; 32];
        let mut tiny_stack = [0u32; 1];
        let mut parse = [0u32; 8];
        let mut steps = [Reduction::default(); 8];
        let mut shallow = ParseScratch {
            arena: &mut lex.nodes,
            free: base,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut tiny_stack,
            parse: &mut parse,
            steps: &mut steps,
            step_count: 0,
        };
        assert_eq!(
            reduce(&sentence, &mut shallow),
            Err(ParseError::BoundExceeded)
        );
        assert!(bindings.iter().all(|b| b.term().is_none()), "nothing bound");
        // An index outside the arena, and a slash without its three children.
        let (outcome, _, _, _) = run(&mut lex, &[the, 9_999]);
        assert_eq!(outcome, Err(ParseError::Malformed));
        let bare = lex.push(TermNode::compound(CATEGORY_FORWARD, &[dog]).unwrap());
        let (outcome, _, _, _) = run(&mut lex, &[bare, dog]);
        assert_eq!(outcome, Err(ParseError::Malformed));
        let (outcome, _, _, _) = run(&mut lex, &[dog, bare]);
        assert_eq!(outcome, Err(ParseError::Malformed));
        // A trail length past the trail is a bound.
        let mut bindings = [Binding::UNBOUND; 32];
        let mut parse = [0u32; 8];
        let mut steps = [Reduction::default(); 8];
        let mut past = ParseScratch {
            arena: &mut lex.nodes,
            free: base,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 33,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut steps,
            step_count: 0,
        };
        assert_eq!(reduce(&sentence, &mut past), Err(ParseError::BoundExceeded));
    }

    #[test]
    fn a_full_trail_still_reduces_a_pair_that_binds_nothing() {
        // `S(sleep)\NP(dog)`, a verb that wants exactly the dog: the application unifies
        // ground categories and binds no variable, so a trail with no room is enough.
        let mut lex = Lexicon::<32>::new();
        let dog = lex.np(DOG);
        let head_v = lex.constant(CHASE);
        let s = lex.atom(S, head_v);
        let dog_again = lex.np(DOG);
        let agent = lex.constant(AGENT);
        let verb = lex.bwd(s, dog_again, agent);
        let mut bindings = [Binding::UNBOUND; 4];
        let mut no_trail = [0u32; 0];
        let mut stack = [0u32; 8];
        let mut parse = [0u32; 4];
        let mut steps = [Reduction::default(); 4];
        let free = lex.len;
        let mut scratch = ParseScratch {
            arena: &mut lex.nodes,
            free,
            bindings: &mut bindings,
            trail: &mut no_trail,
            trail_len: 0,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut steps,
            step_count: 0,
        };
        assert_eq!(
            reduce(&[dog, verb], &mut scratch),
            Ok(s),
            "a full trail, nothing to bind"
        );
        assert_eq!((scratch.step_count, scratch.trail_len), (1, 0));
        // A cat does not unify with the dog: no derivation, still no binding, no panic.
        let cat = lex.push(TermNode::compound(NP, &[]).unwrap());
        let mut scratch = ParseScratch {
            arena: &mut lex.nodes,
            free: free.wrapping_add(1),
            bindings: &mut bindings,
            trail: &mut no_trail,
            trail_len: 0,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut steps,
            step_count: 0,
        };
        assert_eq!(
            reduce(&[cat, verb], &mut scratch),
            Err(ParseError::NoDerivation { remaining: 2 })
        );
    }

    #[test]
    fn an_application_that_cannot_be_logged_is_undone_without_touching_the_arena() {
        let mut lex = Lexicon::<32>::new();
        let the = lex.the();
        let dog = lex.noun(DOG);
        let base = lex.len;
        let before = lex.nodes;
        let mut bindings = [Binding::UNBOUND; 4];
        let mut trail = [0u32; 4];
        let mut stack = [0u32; 8];
        let mut parse = [0u32; 4];
        let mut no_steps: [Reduction; 0] = [];
        let mut scratch = ParseScratch {
            arena: &mut lex.nodes,
            free: base,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut no_steps,
            step_count: 0,
        };
        assert_eq!(
            reduce(&[the, dog], &mut scratch),
            Err(ParseError::StepsFull)
        );
        assert_eq!(
            (scratch.free, scratch.trail_len, scratch.step_count),
            (base, 0, 0),
            "an application pushes no node, so none is retracted"
        );
        assert_eq!(lex.nodes, before, "the arena is as it was");
        assert!(
            bindings.iter().all(|b| b.term().is_none()),
            "the binding was undone"
        );
    }

    #[test]
    fn the_head_of_a_category_is_read_through_functors_and_bindings() {
        let mut lex = Lexicon::<32>::new();
        let chase = lex.transitive(CHASE);
        let bindings = [Binding::UNBOUND; 8];
        assert_eq!(
            head(chase, &lex.nodes, &bindings),
            Some(CHASE),
            "through two slashes"
        );
        let s = lex.constant(S);
        assert_eq!(
            head(s, &lex.nodes, &bindings),
            None,
            "a constant has no feature"
        );
        let v = lex.var();
        assert_eq!(head(v, &lex.nodes, &bindings), None, "an unbound variable");
        let np_v = lex.atom(NP, v);
        assert_eq!(head(np_v, &lex.nodes, &bindings), None, "an unbound head");
        let mut bound = [Binding::UNBOUND; 8];
        let dog = lex.constant(DOG);
        bound[lex.nodes[v as usize].functor as usize] = Binding(dog.wrapping_add(1));
        assert_eq!(head(np_v, &lex.nodes, &bound), Some(DOG), "a bound head");
        assert_eq!(
            head(v, &lex.nodes, &bound),
            None,
            "a variable bound to a constant is not a category"
        );
        let empty = lex.push(TermNode::compound(NP, &[]).unwrap());
        assert_eq!(head(empty, &lex.nodes, &bindings), None, "no child");
        let nested = lex.atom(NP, np_v);
        assert_eq!(
            head(nested, &lex.nodes, &bound),
            None,
            "a child that is a compound"
        );
        assert_eq!(
            head(9_999, &lex.nodes, &bindings),
            None,
            "outside the arena"
        );
        let _ = (TERM_NONE, TERM_VARIABLE);
    }

    #[test]
    fn a_reduction_is_deterministic_and_the_trail_undoes_it() {
        let mut a = Lexicon::<64>::new();
        let mut b = Lexicon::<64>::new();
        let sa = [
            a.the(),
            a.noun(DOG),
            a.transitive(CHASE),
            a.the(),
            a.noun(CAT),
        ];
        let sb = [
            b.the(),
            b.noun(DOG),
            b.transitive(CHASE),
            b.the(),
            b.noun(CAT),
        ];
        let (oa, la, ca, ba) = run(&mut a, &sa);
        let (ob, lb, cb, bb) = run(&mut b, &sb);
        assert_eq!((oa, ca), (ob, cb));
        assert_eq!(la[..ca], lb[..cb]);
        assert_eq!(ba, bb);
        assert_eq!(a.nodes, b.nodes);
        // The trail names every variable the parse bound; undoing it clears the table.
        let mut lex = Lexicon::<64>::new();
        let s = [
            lex.the(),
            lex.noun(DOG),
            lex.transitive(CHASE),
            lex.the(),
            lex.noun(CAT),
        ];
        let mut bindings = [Binding::UNBOUND; 32];
        let mut trail = [0u32; 32];
        let mut stack = [0u32; 32];
        let mut parse = [0u32; 8];
        let mut steps = [Reduction::default(); 8];
        let free = lex.len;
        let mut scratch = ParseScratch {
            arena: &mut lex.nodes,
            free,
            bindings: &mut bindings,
            trail: &mut trail,
            trail_len: 0,
            stack: &mut stack,
            parse: &mut parse,
            steps: &mut steps,
            step_count: 0,
        };
        assert!(reduce(&s, &mut scratch).is_ok());
        let bound = scratch.trail_len;
        assert_eq!(bound, 4, "X1, P (to X2), X2 and A");
        assert_eq!(
            bindings.iter().filter(|b| b.term().is_some()).count(),
            bound
        );
        undo(&mut bindings, &trail, bound);
        assert!(bindings.iter().all(|b| b.term().is_none()));
    }
}

#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    const S: u32 = 10;
    const NP: u32 = 11;
    const N: u32 = 12;
    const AGENT: u32 = 900;
    const PATIENT: u32 = 901;
    const NONE: u32 = 902;

    /// One of eight lexical shapes, instantiated with fresh variables at the arena's end.
    fn entry(shape: u32, arena: &mut [TermNode], len: &mut usize, vars: &mut u32) -> u32 {
        fn push(arena: &mut [TermNode], len: &mut usize, node: TermNode) -> u32 {
            let i = *len;
            arena[i] = node;
            *len = len.wrapping_add(1);
            i as u32
        }
        fn var(arena: &mut [TermNode], len: &mut usize, vars: &mut u32) -> u32 {
            let v = *vars;
            *vars = vars.wrapping_add(1);
            push(arena, len, TermNode::variable(v))
        }
        fn atom(arena: &mut [TermNode], len: &mut usize, f: u32, feature: u32) -> u32 {
            push(arena, len, TermNode::compound(f, &[feature]).unwrap())
        }
        match shape {
            0 | 1 => {
                let c = push(arena, len, TermNode::constant(100u32.wrapping_add(shape)));
                atom(arena, len, NP, c)
            }
            2 => {
                let c = push(arena, len, TermNode::constant(110));
                atom(arena, len, N, c)
            }
            3 => {
                let x = var(arena, len, vars);
                let np = atom(arena, len, NP, x);
                let n = atom(arena, len, N, x);
                let none = push(arena, len, TermNode::constant(NONE));
                push(arena, len, forward(np, n, none).unwrap())
            }
            4 => {
                let c = push(arena, len, TermNode::constant(120));
                let s = atom(arena, len, S, c);
                let a = var(arena, len, vars);
                let np_a = atom(arena, len, NP, a);
                let agent = push(arena, len, TermNode::constant(AGENT));
                let s_np = push(arena, len, backward(s, np_a, agent).unwrap());
                let p = var(arena, len, vars);
                let np_p = atom(arena, len, NP, p);
                let patient = push(arena, len, TermNode::constant(PATIENT));
                push(arena, len, forward(s_np, np_p, patient).unwrap())
            }
            5 => {
                let c = push(arena, len, TermNode::constant(121));
                let s = atom(arena, len, S, c);
                let a = var(arena, len, vars);
                let np_a = atom(arena, len, NP, a);
                let agent = push(arena, len, TermNode::constant(AGENT));
                push(arena, len, backward(s, np_a, agent).unwrap())
            }
            6 => {
                let v = var(arena, len, vars);
                let a = var(arena, len, vars);
                let r = var(arena, len, vars);
                let s = atom(arena, len, S, v);
                let np_a = atom(arena, len, NP, a);
                let inner = push(arena, len, backward(s, np_a, r).unwrap());
                let s2 = atom(arena, len, S, v);
                let np_a2 = atom(arena, len, NP, a);
                let outer = push(arena, len, backward(s2, np_a2, r).unwrap());
                let none = push(arena, len, TermNode::constant(NONE));
                push(arena, len, forward(outer, inner, none).unwrap())
            }
            _ => {
                let v = var(arena, len, vars);
                let s = atom(arena, len, S, v);
                let s2 = atom(arena, len, S, v);
                let none = push(arena, len, TermNode::constant(NONE));
                push(arena, len, backward(s, s2, none).unwrap())
            }
        }
    }

    #[test]
    fn random_sequences_never_panic_and_a_derivation_keeps_every_invariant() {
        let mut rng = Lcg::new(2_020);
        let mut derived = 0u32;
        for _ in 0..3_000 {
            let mut arena = [TermNode::default(); 160];
            let (mut len, mut vars) = (0usize, 0u32);
            let count = rng.below(8) as usize + 1;
            let mut categories = [0u32; 8];
            for slot in categories.iter_mut().take(count) {
                *slot = entry(rng.below(8), &mut arena, &mut len, &mut vars);
            }
            let mut bindings = [Binding::UNBOUND; 64];
            let mut trail = [0u32; 64];
            let mut stack = [0u32; 64];
            let mut parse = [0u32; 8];
            let mut steps = [Reduction::default(); 8];
            let mut scratch = ParseScratch {
                arena: &mut arena,
                free: len,
                bindings: &mut bindings,
                trail: &mut trail,
                trail_len: 0,
                stack: &mut stack,
                parse: &mut parse,
                steps: &mut steps,
                step_count: 0,
            };
            let outcome = reduce(&categories[..count], &mut scratch);
            let (free, step_count, trail_len) =
                (scratch.free, scratch.step_count, scratch.trail_len);
            assert!(free >= len && free <= arena.len());
            assert!(trail_len <= trail.len());
            for step in &steps[..step_count] {
                assert!((step.left as usize) < free && (step.right as usize) < free);
                assert!((step.result as usize) < free && (step.argument as usize) < free);
                assert!((step.role as usize) < free);
                assert!(
                    step.rule >= RULE_FORWARD_APPLICATION && step.rule <= RULE_BACKWARD_COMPOSITION
                );
            }
            assert_eq!(
                bindings.iter().filter(|b| b.term().is_some()).count(),
                trail_len,
                "the trail names exactly the bound variables"
            );
            for i in 0..free as u32 {
                assert!(
                    deref(i, &arena, &bindings).is_some(),
                    "every node dereferences"
                );
            }
            match outcome {
                Ok(root) => {
                    derived = derived.wrapping_add(1);
                    assert!((root as usize) < free);
                    assert_eq!(
                        step_count,
                        count.wrapping_sub(1),
                        "one reduction per shift but the first"
                    );
                }
                Err(ParseError::NoDerivation { remaining }) => {
                    assert!(remaining >= 2 && remaining <= count);
                    assert_eq!(step_count, count.wrapping_sub(remaining));
                }
                Err(other) => panic!("a bound the sizes here cannot reach: {other:?}"),
            }
        }
        assert!(derived > 200, "the walk derives some sentences: {derived}");
    }
}
