//! Induction on the term arena (whitepaper §5.2.30, §6.10, §8.8; ADR-0041; brief 021). A
//! definite clause is a term: a compound over [`CLAUSE`] whose first child is the head and
//! whose other children, at most [`MAX_BODY`], are the positive literals of its body. The
//! rules here are the inverse of resolution (Muggleton and Buntine 1988) and Plotkin's least
//! general generalisation (1970) as bounded, deterministic operations over caller slices:
//! [`absorb`] folds one clause's body into another as a single literal, [`identify`] recovers
//! the clause a literal stands for, [`intra_construct`] factors the differing parts of two
//! clauses with one head into a predicate it invents from a reserved band of ids, and [`lgg`]
//! generalises two terms. Each operator's outputs resolve back to its inputs by
//! [`resolve_definite`], the one definite-clause resolution step, which is the identity the
//! tests hold. Literals are matched first fit, in order, without backtracking, by
//! unification: an output is an instance of its inputs, and the bindings the matching made
//! stay in the caller's table with the trail saying which. Every bound is a result, nothing
//! allocates, and a failure leaves the arena, the bindings and the counters as they were.
//! Which pairs of clauses to try is the caller's search (Specified); a clause holds ids,
//! never words (rule L-3).

use crate::category::{CATEGORY_BACKWARD, CATEGORY_FORWARD, CATEGORY_RESERVED};
use crate::term::{
    Binding, MAX_ARITY, TERM_COMPOUND, TERM_CONSTANT, TERM_EMPTY, TERM_NONE, TERM_VARIABLE,
    TermNode, UnifyResult, deref, undo, unify,
};

/// The functor of a definite clause: `CLAUSE(head, literal, ...)`.
pub const CLAUSE: u32 = 0xFFFF_FF03;
/// The most literals a clause body holds: the arity less the head.
pub const MAX_BODY: usize = MAX_ARITY - 1;
/// The first id an invented predicate takes.
pub const INVENTED_BASE: u32 = 0xFFFE_0000;
/// One past the last id an invented predicate may take: 65 536 inventions.
pub const INVENTED_LIMIT: u32 = 0xFFFF_0000;

/// FNV-1a, 32 bits: the offset basis and the prime.
const FNV_OFFSET: u32 = 0x811C_9DC5;
const FNV_PRIME: u32 = 0x0100_0193;
/// A literal without a partner in a pairing.
const UNPAIRED: u8 = u8::MAX;
/// The stack entry that marks the root of a generalisation.
const ROOT: u32 = u32::MAX;
/// The most variables the scope of an invention holds (the head and the shared literals).
const SCOPE: usize = 64;

/// Why a rule did not apply. Every value leaves the arena, the bindings, the trail and the
/// scratch's counters as they were before the call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InduceError {
    /// An index outside the arena or the table, an empty node, or a term that is not a
    /// clause where one is required.
    Malformed,
    /// The heads do not unify, a literal that must be matched has no partner, or (for
    /// `identify`) the number of unmatched literals is not one.
    NoMatch,
    /// Nothing to factor: the two clauses share no literal, or one of them has no literal of
    /// its own.
    NothingToInvent,
    /// The arena has no free node for an output.
    ArenaFull,
    /// The work stack, the trail, the binding table or an output slice is too small.
    BoundExceeded,
    /// The generalisation's pair table is too small for these terms.
    PairsFull,
    /// The invented predicate would need more than `MAX_ARITY` arguments.
    TooManyArguments,
    /// A resolvent would have more than `MAX_BODY` literals.
    BodyFull,
    /// Every id of the invented band is taken.
    InventionsExhausted,
}

/// The caller's slices an induction runs over (TC-5: nothing is allocated). The arena is
/// read and appended to at `free`; the bindings accumulate the matching's substitution, its
/// variables pushed onto the trail from `trail_len`; `stack` is the work stack of every walk
/// and of unification; `pairs` is the generalisation's table of the term pairs it turned into
/// variables; `next_variable` numbers the variables the generalisation creates (each must
/// index the binding table); `next_invented` is the id the next invention takes.
pub struct InduceScratch<'a> {
    pub arena: &'a mut [TermNode],
    pub free: usize,
    pub bindings: &'a mut [Binding],
    pub trail: &'a mut [u32],
    pub trail_len: usize,
    pub stack: &'a mut [u32],
    pub pairs: &'a mut [[u32; 3]],
    pub next_variable: u32,
    pub next_invented: u32,
}

/// What an intra-construction produced: the predicate it invented, how many arguments the
/// invented literal takes, how many literals the inputs shared, the clause that now names the
/// invented predicate in place of the differing literals (`p ← A, q(V)`), and the two clauses
/// that define it (`q(V) ← B₁`, `q(V) ← B₂`), as arena indices.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Invention {
    pub predicate: u32,
    pub arguments: u8,
    pub shared: u8,
    pub common: u32,
    pub definitions: [u32; 2],
}

/// A definite clause `head ← body`, the body at most [`MAX_BODY`] literals (term indices).
/// `None` for more, or for an index the encoding cannot hold.
pub fn clause(head: u32, body: &[u32]) -> Option<TermNode> {
    if body.len() > MAX_BODY {
        return None;
    }
    let mut args = [TERM_NONE; MAX_ARITY];
    args[0] = head;
    args[1..=body.len()].copy_from_slice(body);
    TermNode::compound(CLAUSE, &args[..=body.len()])
}

/// True for a clause node: a compound over `CLAUSE` with at least its head.
pub const fn is_clause(node: &TermNode) -> bool {
    node.kind == TERM_COMPOUND && node.functor == CLAUSE && node.arity >= 1
}

/// The head of a clause, or `None` for a node that is not one.
pub const fn clause_head(node: &TermNode) -> Option<u32> {
    if is_clause(node) { node.child(0) } else { None }
}

/// The literals in a clause's body; zero for a node that is not a clause.
pub const fn clause_body_len(node: &TermNode) -> usize {
    if is_clause(node) {
        (node.arity as usize).saturating_sub(1)
    } else {
        0
    }
}

/// The `index`th body literal of a clause, or `None` past the body or for a node that is
/// not a clause.
pub const fn clause_literal(node: &TermNode, index: usize) -> Option<u32> {
    if !is_clause(node) || index >= MAX_BODY {
        return None;
    }
    // Below `MAX_BODY`, so one past it is within the arity's range.
    node.child(index.wrapping_add(1))
}

/// A pre-order walk of `term` through the bindings with `stack` as the work stack: `visit`
/// sees every node once, its index and the node, children after their parent and the first
/// child first. `Malformed` for an index outside the arena or the table or an empty node;
/// `BoundExceeded` when the stack is too small.
fn walk(
    term: u32,
    arena: &[TermNode],
    bindings: &[Binding],
    stack: &mut [u32],
    mut visit: impl FnMut(u32, &TermNode) -> Result<(), InduceError>,
) -> Result<(), InduceError> {
    if stack.is_empty() {
        return Err(InduceError::BoundExceeded);
    }
    stack[0] = term;
    let mut top = 1usize;
    while top > 0 {
        // Positive above, so at least one.
        top = top.wrapping_sub(1);
        let Some(index) = deref(stack[top], arena, bindings) else {
            return Err(InduceError::Malformed);
        };
        // `deref` returned an index inside the arena.
        let node = arena[index as usize];
        if node.kind == TERM_EMPTY {
            return Err(InduceError::Malformed);
        }
        visit(index, &node)?;
        // Pushed last first, so the first child is popped next.
        for slot in (0..node.arity as usize).rev() {
            let Some(child) = node.child(slot) else {
                return Err(InduceError::Malformed);
            };
            if top >= stack.len() {
                return Err(InduceError::BoundExceeded);
            }
            stack[top] = child;
            top = top.wrapping_add(1);
        }
    }
    Ok(())
}

/// The nodes of `term` through the bindings: a bound variable counts as what it is bound to,
/// an unbound one as one node, a shared subterm each time it is reached. Saturating.
pub fn size(
    term: u32,
    arena: &[TermNode],
    bindings: &[Binding],
    stack: &mut [u32],
) -> Result<u32, InduceError> {
    let mut count = 0u32;
    walk(term, arena, bindings, stack, |_, _| {
        count = count.saturating_add(1);
        Ok(())
    })?;
    Ok(count)
}

/// A hash of `term`'s structure through the bindings: FNV-1a over the pre-order walk, each
/// node contributing its kind, its functor (a variable's number) and its arity; the same for
/// the same structure wherever it sits in an arena.
pub fn term_hash(
    term: u32,
    arena: &[TermNode],
    bindings: &[Binding],
    stack: &mut [u32],
) -> Result<u32, InduceError> {
    let mut hash = FNV_OFFSET;
    walk(term, arena, bindings, stack, |_, node| {
        let functor = node.functor.to_le_bytes();
        let bytes = [
            node.kind, functor[0], functor[1], functor[2], functor[3], node.arity,
        ];
        for byte in bytes {
            hash = (hash ^ u32::from(byte)).wrapping_mul(FNV_PRIME);
        }
        Ok(())
    })?;
    Ok(hash)
}

/// The unbound variables of `term` through the bindings, as the index of the first node of
/// each variable number, appended to `out[..len]` without repeating a number already there;
/// returns the new length. `BoundExceeded` when `out` is full or `len` is past it.
pub fn free_variables(
    term: u32,
    arena: &[TermNode],
    bindings: &[Binding],
    stack: &mut [u32],
    out: &mut [u32],
    len: usize,
) -> Result<usize, InduceError> {
    if len > out.len() {
        return Err(InduceError::BoundExceeded);
    }
    let mut len = len;
    walk(term, arena, bindings, stack, |index, node| {
        if node.kind != TERM_VARIABLE {
            return Ok(());
        }
        let seen = out[..len].iter().any(|&v| {
            arena
                .get(v as usize)
                .is_some_and(|n| n.functor == node.functor)
        });
        if seen {
            return Ok(());
        }
        if len >= out.len() {
            return Err(InduceError::BoundExceeded);
        }
        out[len] = index;
        len = len.wrapping_add(1);
        Ok(())
    })?;
    Ok(len)
}

/// Where a call started, so that a failure can put everything back.
struct Mark {
    free: usize,
    trail_len: usize,
    next_variable: u32,
    next_invented: u32,
}

fn mark(s: &InduceScratch) -> Mark {
    Mark {
        free: s.free,
        trail_len: s.trail_len,
        next_variable: s.next_variable,
        next_invented: s.next_invented,
    }
}

/// Undoes the bindings made since the mark, zeroes the nodes appended since it, and resets
/// the cursor and the counters.
fn restore(s: &mut InduceScratch, m: &Mark) {
    let bound = s.trail_len.saturating_sub(m.trail_len);
    if let Some(trail) = s.trail.get(m.trail_len..) {
        undo(s.bindings, trail, bound);
    }
    s.trail_len = m.trail_len;
    if let Some(nodes) = s.arena.get_mut(m.free..s.free) {
        for node in nodes {
            *node = TermNode::default();
        }
    }
    s.free = m.free;
    s.next_variable = m.next_variable;
    s.next_invented = m.next_invented;
}

/// Appends a node at the cursor.
fn alloc(s: &mut InduceScratch, node: TermNode) -> Result<u32, InduceError> {
    let index = u32::try_from(s.free).map_err(|_| InduceError::ArenaFull)?;
    let Some(slot) = s.arena.get_mut(s.free) else {
        return Err(InduceError::ArenaFull);
    };
    *slot = node;
    // Below the arena's length before the step.
    s.free = s.free.wrapping_add(1);
    Ok(index)
}

/// Unifies two terms on the scratch: `Ok(true)` with the bindings kept and trailed,
/// `Ok(false)` for a clash or an occurs-check failure with nothing changed, and the bound
/// or malformed cases as errors.
fn unify_in(a: u32, b: u32, s: &mut InduceScratch) -> Result<bool, InduceError> {
    let Some((_, free_trail)) = s.trail.split_at_mut_checked(s.trail_len) else {
        return Err(InduceError::BoundExceeded);
    };
    let (outcome, bound) = unify(a, b, s.arena, s.bindings, free_trail, s.stack);
    match outcome {
        UnifyResult::Unified => {
            // At most the free trail's length, so within the trail.
            s.trail_len = s.trail_len.wrapping_add(bound);
            Ok(true)
        }
        UnifyResult::Clash | UnifyResult::OccursCheck => Ok(false),
        UnifyResult::BoundExceeded => Err(InduceError::BoundExceeded),
        UnifyResult::Malformed => Err(InduceError::Malformed),
    }
}

/// The clause `index` dereferences to.
fn clause_at(index: u32, s: &InduceScratch) -> Result<TermNode, InduceError> {
    let Some(i) = deref(index, s.arena, s.bindings) else {
        return Err(InduceError::Malformed);
    };
    // `deref` returned an index inside the arena.
    let node = s.arena[i as usize];
    if !is_clause(&node) {
        return Err(InduceError::Malformed);
    }
    Ok(node)
}

/// How the literals of one body were paired with another's: for each literal of `x`, the
/// index of its partner in `y` or `UNPAIRED`; which literals of `y` were taken; the count.
struct Pairing {
    of_x: [u8; MAX_BODY],
    taken_y: [bool; MAX_BODY],
    matched: usize,
}

/// Pairs each literal of `x`'s body, in order, with the first literal of `y`'s body not yet
/// taken that unifies with it: first fit, no backtracking. The bindings accumulate.
fn pair_bodies(x: &TermNode, y: &TermNode, s: &mut InduceScratch) -> Result<Pairing, InduceError> {
    let mut pairing = Pairing {
        of_x: [UNPAIRED; MAX_BODY],
        taken_y: [false; MAX_BODY],
        matched: 0,
    };
    for i in 0..clause_body_len(x) {
        let Some(lx) = clause_literal(x, i) else {
            return Err(InduceError::Malformed);
        };
        for j in 0..clause_body_len(y) {
            if pairing.taken_y[j] {
                continue;
            }
            let Some(ly) = clause_literal(y, j) else {
                return Err(InduceError::Malformed);
            };
            if unify_in(lx, ly, s)? {
                pairing.taken_y[j] = true;
                pairing.of_x[i] = j as u8;
                pairing.matched = pairing.matched.wrapping_add(1);
                break;
            }
        }
    }
    Ok(pairing)
}

/// A clause under construction: its head and literals, at most `MAX_ARITY` in all.
struct Body {
    args: [u32; MAX_ARITY],
    len: usize,
}

impl Body {
    const fn new() -> Self {
        Self {
            args: [TERM_NONE; MAX_ARITY],
            len: 0,
        }
    }

    fn push(&mut self, term: u32) -> Result<(), InduceError> {
        if self.len >= MAX_ARITY {
            return Err(InduceError::BodyFull);
        }
        self.args[self.len] = term;
        self.len = self.len.wrapping_add(1);
        Ok(())
    }

    fn alloc(&self, s: &mut InduceScratch) -> Result<u32, InduceError> {
        let node =
            TermNode::compound(CLAUSE, &self.args[..self.len]).ok_or(InduceError::Malformed)?;
        alloc(s, node)
    }
}

/// Plotkin's least general generalisation of `a` and `b` through the bindings: equal
/// constants and the same node stay; compounds of one functor and arity generalise child by
/// child into a new compound; any other pair becomes a variable, the same variable for the
/// same pair of (dereferenced) nodes, numbered from `next_variable`. Returns the index of the
/// generalisation (an existing node when the terms are equal). A variable is identified by
/// its node: a caller that shares one node per variable, as the reducer's lexicon does, gets
/// the least general result.
pub fn lgg(a: u32, b: u32, s: &mut InduceScratch) -> Result<u32, InduceError> {
    let m = mark(s);
    let out = lgg_in(a, b, s);
    if out.is_err() {
        restore(s, &m);
    }
    out
}

/// Pushes a pair to generalise and where its result goes: four stack entries.
fn push_pair(
    stack: &mut [u32],
    top: &mut usize,
    x: u32,
    y: u32,
    parent: u32,
    slot: u32,
) -> Result<(), InduceError> {
    let Some(end) = top.checked_add(4).filter(|&end| end <= stack.len()) else {
        return Err(InduceError::BoundExceeded);
    };
    // Four entries below `end`, which is within the stack.
    stack[*top] = x;
    stack[top.wrapping_add(1)] = y;
    stack[top.wrapping_add(2)] = parent;
    stack[top.wrapping_add(3)] = slot;
    *top = end;
    Ok(())
}

fn lgg_in(a: u32, b: u32, s: &mut InduceScratch) -> Result<u32, InduceError> {
    let mut pairs_len = 0usize;
    let mut top = 0usize;
    push_pair(s.stack, &mut top, a, b, ROOT, 0)?;
    let mut root = TERM_NONE;
    while top > 0 {
        // Entries come in fours, so a positive `top` is at least four.
        top = top.wrapping_sub(4);
        let (x, y, parent, slot) = (
            s.stack[top],
            s.stack[top.wrapping_add(1)],
            s.stack[top.wrapping_add(2)],
            s.stack[top.wrapping_add(3)],
        );
        let (Some(dx), Some(dy)) = (deref(x, s.arena, s.bindings), deref(y, s.arena, s.bindings))
        else {
            return Err(InduceError::Malformed);
        };
        let (nx, ny) = (s.arena[dx as usize], s.arena[dy as usize]);
        if nx.kind == TERM_EMPTY || ny.kind == TERM_EMPTY {
            return Err(InduceError::Malformed);
        }
        let same_constant =
            nx.kind == TERM_CONSTANT && ny.kind == TERM_CONSTANT && nx.functor == ny.functor;
        let same_shape = nx.kind == TERM_COMPOUND
            && ny.kind == TERM_COMPOUND
            && nx.functor == ny.functor
            && nx.arity == ny.arity;
        let result = if dx == dy || same_constant {
            dx
        } else if same_shape {
            let node = alloc(
                s,
                TermNode {
                    kind: TERM_COMPOUND,
                    arity: nx.arity,
                    _pad: 0,
                    functor: nx.functor,
                    children: [TERM_NONE; MAX_ARITY],
                    _reserved: [0; 24],
                },
            )?;
            for k in (0..nx.arity as usize).rev() {
                let (Some(cx), Some(cy)) = (nx.child(k), ny.child(k)) else {
                    return Err(InduceError::Malformed);
                };
                push_pair(s.stack, &mut top, cx, cy, node, k as u32)?;
            }
            node
        } else if let Some(pair) = s.pairs[..pairs_len]
            .iter()
            .find(|pair| pair[0] == dx && pair[1] == dy)
        {
            pair[2]
        } else {
            if pairs_len >= s.pairs.len() {
                return Err(InduceError::PairsFull);
            }
            if s.next_variable as usize >= s.bindings.len() {
                return Err(InduceError::BoundExceeded);
            }
            let variable = alloc(s, TermNode::variable(s.next_variable))?;
            // Below the table's length, checked above.
            s.next_variable = s.next_variable.wrapping_add(1);
            s.pairs[pairs_len] = [dx, dy, variable];
            pairs_len = pairs_len.wrapping_add(1);
            variable
        };
        if parent == ROOT {
            root = result;
        } else {
            // An arena index, below the length `TermNode::compound` bounds at `u32::MAX`.
            s.arena[parent as usize].children[slot as usize] = result.wrapping_add(1);
        }
    }
    Ok(root)
}

/// Absorption (the V operator): from `c2 = q ← A` and `c = p ← A', B`, where every literal
/// of `A` matches a distinct literal of `c`'s body (first fit, in order, by unification, so
/// `A'` is an instance of `A`), the clause `p ← qθ, B`: `c2`'s head in place of the matched
/// literals, first, then the unmatched ones in their order. Resolving the result with `c2`
/// gives `c` back. `NoMatch` when `c2` has no body or a literal of it has no partner.
pub fn absorb(c2: u32, c: u32, s: &mut InduceScratch) -> Result<u32, InduceError> {
    let m = mark(s);
    let out = absorb_in(c2, c, s);
    if out.is_err() {
        restore(s, &m);
    }
    out
}

fn absorb_in(c2: u32, c: u32, s: &mut InduceScratch) -> Result<u32, InduceError> {
    let n2 = clause_at(c2, s)?;
    let nc = clause_at(c, s)?;
    if clause_body_len(&n2) == 0 {
        return Err(InduceError::NoMatch);
    }
    let pairing = pair_bodies(&n2, &nc, s)?;
    if pairing.matched < clause_body_len(&n2) {
        return Err(InduceError::NoMatch);
    }
    let mut body = Body::new();
    body.push(clause_head(&nc).ok_or(InduceError::Malformed)?)?;
    body.push(clause_head(&n2).ok_or(InduceError::Malformed)?)?;
    for j in 0..clause_body_len(&nc) {
        if !pairing.taken_y[j] {
            body.push(clause_literal(&nc, j).ok_or(InduceError::Malformed)?)?;
        }
    }
    body.alloc(s)
}

/// Identification (the other V operator): from `c1 = p ← q, B` and `c = p ← A, B`, whose
/// heads unify and whose `B` literals match first fit in order, the clause `q ← A`: the one
/// literal of `c1` left unmatched as the head, the unmatched literals of `c` as the body.
/// Resolving `c1` with the result gives `c` back. `NoMatch` unless exactly one literal of
/// `c1` is unmatched.
pub fn identify(c1: u32, c: u32, s: &mut InduceScratch) -> Result<u32, InduceError> {
    let m = mark(s);
    let out = identify_in(c1, c, s);
    if out.is_err() {
        restore(s, &m);
    }
    out
}

fn identify_in(c1: u32, c: u32, s: &mut InduceScratch) -> Result<u32, InduceError> {
    let n1 = clause_at(c1, s)?;
    let nc = clause_at(c, s)?;
    let (Some(h1), Some(hc)) = (clause_head(&n1), clause_head(&nc)) else {
        return Err(InduceError::Malformed);
    };
    if !unify_in(h1, hc, s)? {
        return Err(InduceError::NoMatch);
    }
    let pairing = pair_bodies(&n1, &nc, s)?;
    // The count is at most the body's length.
    if clause_body_len(&n1).wrapping_sub(pairing.matched) != 1 {
        return Err(InduceError::NoMatch);
    }
    let Some(q) = (0..clause_body_len(&n1))
        .find(|&i| pairing.of_x[i] == UNPAIRED)
        .and_then(|i| clause_literal(&n1, i))
    else {
        return Err(InduceError::Malformed);
    };
    let mut body = Body::new();
    body.push(q)?;
    for j in 0..clause_body_len(&nc) {
        if !pairing.taken_y[j] {
            body.push(clause_literal(&nc, j).ok_or(InduceError::Malformed)?)?;
        }
    }
    body.alloc(s)
}

/// Intra-construction (the W operator): from `ca = p ← A, B₁` and `cb = p ← A, B₂`, whose
/// heads unify and whose shared literals `A` match first fit in order, three clauses over a
/// predicate `q` it invents (the next id of the band): `p ← A, q(V)`, `q(V) ← B₁` and
/// `q(V) ← B₂`, where `V` are the unbound variables of `p ← A` that occur in `B₁` or `B₂`,
/// in order of first occurrence, at most `MAX_ARITY`. Resolving the first with either
/// definition gives the corresponding input back. `NothingToInvent` when no literal is
/// shared or either clause has no literal of its own.
pub fn intra_construct(ca: u32, cb: u32, s: &mut InduceScratch) -> Result<Invention, InduceError> {
    let m = mark(s);
    let out = intra_construct_in(ca, cb, s);
    if out.is_err() {
        restore(s, &m);
    }
    out
}

/// The variables of `term` that occur in `scope`, appended to `args` unless already there.
fn link_variables(
    term: u32,
    scope: &[u32],
    args: &mut [u32; MAX_ARITY],
    count: &mut usize,
    s: &mut InduceScratch,
) -> Result<(), InduceError> {
    let mut vars = [TERM_NONE; SCOPE];
    let n = free_variables(term, s.arena, s.bindings, s.stack, &mut vars, 0)?;
    for &v in &vars[..n] {
        let number = s.arena[v as usize].functor;
        let in_scope = scope.iter().any(|&w| s.arena[w as usize].functor == number);
        let listed = args[..*count]
            .iter()
            .any(|&w| s.arena[w as usize].functor == number);
        if in_scope && !listed {
            if *count >= MAX_ARITY {
                return Err(InduceError::TooManyArguments);
            }
            args[*count] = v;
            *count = count.wrapping_add(1);
        }
    }
    Ok(())
}

fn intra_construct_in(ca: u32, cb: u32, s: &mut InduceScratch) -> Result<Invention, InduceError> {
    let na = clause_at(ca, s)?;
    let nb = clause_at(cb, s)?;
    let (Some(ha), Some(hb)) = (clause_head(&na), clause_head(&nb)) else {
        return Err(InduceError::Malformed);
    };
    if !unify_in(ha, hb, s)? {
        return Err(InduceError::NoMatch);
    }
    let pairing = pair_bodies(&na, &nb, s)?;
    let (la, lb) = (clause_body_len(&na), clause_body_len(&nb));
    if pairing.matched == 0 || pairing.matched == la || pairing.matched == lb {
        return Err(InduceError::NothingToInvent);
    }
    // The scope: the variables of the head and of the shared literals.
    let mut scope = [TERM_NONE; SCOPE];
    let mut scope_len = free_variables(ha, s.arena, s.bindings, s.stack, &mut scope, 0)?;
    for i in 0..la {
        if pairing.of_x[i] != UNPAIRED {
            let lit = clause_literal(&na, i).ok_or(InduceError::Malformed)?;
            scope_len = free_variables(lit, s.arena, s.bindings, s.stack, &mut scope, scope_len)?;
        }
    }
    // The invented literal's arguments: the scope's variables the differing literals use.
    let mut args = [TERM_NONE; MAX_ARITY];
    let mut count = 0usize;
    for i in 0..la {
        if pairing.of_x[i] == UNPAIRED {
            let lit = clause_literal(&na, i).ok_or(InduceError::Malformed)?;
            link_variables(lit, &scope[..scope_len], &mut args, &mut count, s)?;
        }
    }
    for j in 0..lb {
        if !pairing.taken_y[j] {
            let lit = clause_literal(&nb, j).ok_or(InduceError::Malformed)?;
            link_variables(lit, &scope[..scope_len], &mut args, &mut count, s)?;
        }
    }
    if s.next_invented >= INVENTED_LIMIT {
        return Err(InduceError::InventionsExhausted);
    }
    let predicate = s.next_invented;
    let q = alloc(
        s,
        TermNode::compound(predicate, &args[..count]).ok_or(InduceError::Malformed)?,
    )?;
    let mut body = Body::new();
    body.push(ha)?;
    for i in 0..la {
        if pairing.of_x[i] != UNPAIRED {
            body.push(clause_literal(&na, i).ok_or(InduceError::Malformed)?)?;
        }
    }
    body.push(q)?;
    let common = body.alloc(s)?;
    let mut body = Body::new();
    body.push(q)?;
    for i in 0..la {
        if pairing.of_x[i] == UNPAIRED {
            body.push(clause_literal(&na, i).ok_or(InduceError::Malformed)?)?;
        }
    }
    let first = body.alloc(s)?;
    let mut body = Body::new();
    body.push(q)?;
    for j in 0..lb {
        if !pairing.taken_y[j] {
            body.push(clause_literal(&nb, j).ok_or(InduceError::Malformed)?)?;
        }
    }
    let second = body.alloc(s)?;
    // Below the limit, checked above.
    s.next_invented = predicate.wrapping_add(1);
    Ok(Invention {
        predicate,
        arguments: count as u8,
        shared: pairing.matched as u8,
        common,
        definitions: [first, second],
    })
}

/// One step of definite-clause resolution: the first literal of `goal`'s body that unifies
/// with `rule`'s head is replaced by `rule`'s body, in place; the bindings stay. `NoMatch`
/// when no literal unifies; `BodyFull` when the resolvent would have more than `MAX_BODY`
/// literals.
pub fn resolve_definite(goal: u32, rule: u32, s: &mut InduceScratch) -> Result<u32, InduceError> {
    let m = mark(s);
    let out = resolve_in(goal, rule, s);
    if out.is_err() {
        restore(s, &m);
    }
    out
}

fn resolve_in(goal: u32, rule: u32, s: &mut InduceScratch) -> Result<u32, InduceError> {
    let ng = clause_at(goal, s)?;
    let nr = clause_at(rule, s)?;
    let (Some(hg), Some(hr)) = (clause_head(&ng), clause_head(&nr)) else {
        return Err(InduceError::Malformed);
    };
    for i in 0..clause_body_len(&ng) {
        let lit = clause_literal(&ng, i).ok_or(InduceError::Malformed)?;
        if !unify_in(lit, hr, s)? {
            continue;
        }
        let mut body = Body::new();
        body.push(hg)?;
        for k in 0..i {
            body.push(clause_literal(&ng, k).ok_or(InduceError::Malformed)?)?;
        }
        for j in 0..clause_body_len(&nr) {
            body.push(clause_literal(&nr, j).ok_or(InduceError::Malformed)?)?;
        }
        for k in i.wrapping_add(1)..clause_body_len(&ng) {
            body.push(clause_literal(&ng, k).ok_or(InduceError::Malformed)?)?;
        }
        return body.alloc(s);
    }
    Err(InduceError::NoMatch)
}

const _: () = {
    assert!(CLAUSE > CATEGORY_RESERVED);
    assert!(CLAUSE != CATEGORY_FORWARD && CLAUSE != CATEGORY_BACKWARD);
    assert!(INVENTED_BASE < INVENTED_LIMIT && INVENTED_LIMIT <= CATEGORY_RESERVED);
    assert!(MAX_BODY < MAX_ARITY);
    assert!(SCOPE >= MAX_ARITY);
};

#[cfg(test)]
mod tests {
    use super::*;

    const P: u32 = 0x100;
    const R: u32 = 0x101;
    const S: u32 = 0x102;
    const T: u32 = 0x103;
    const U: u32 = 0x104;
    const W: u32 = 0x105;
    const Q: u32 = 0x106;
    const A: u32 = 0x200;
    const B: u32 = 0x201;
    const C: u32 = 0x202;
    const F: u32 = 0x300;
    const G: u32 = 0x301;

    struct Kit<const N: usize> {
        arena: [TermNode; N],
        len: usize,
        bindings: [Binding; 32],
        trail: [u32; 64],
        stack: [u32; 128],
        pairs: [[u32; 3]; 16],
        vars: u32,
    }

    impl<const N: usize> Kit<N> {
        fn new() -> Self {
            Self {
                arena: [TermNode::default(); N],
                len: 0,
                bindings: [Binding::UNBOUND; 32],
                trail: [0; 64],
                stack: [0; 128],
                pairs: [[0; 3]; 16],
                vars: 0,
            }
        }

        fn push(&mut self, node: TermNode) -> u32 {
            let i = self.len;
            self.arena[i] = node;
            self.len = self.len.wrapping_add(1);
            i as u32
        }

        fn constant(&mut self, c: u32) -> u32 {
            self.push(TermNode::constant(c))
        }

        /// A fresh variable node with the next number.
        fn var(&mut self) -> u32 {
            let v = self.vars;
            self.vars = self.vars.wrapping_add(1);
            self.push(TermNode::variable(v))
        }

        fn compound(&mut self, f: u32, args: &[u32]) -> u32 {
            self.push(TermNode::compound(f, args).unwrap())
        }

        fn clause(&mut self, head: u32, body: &[u32]) -> u32 {
            self.push(clause(head, body).unwrap())
        }

        fn scratch(&mut self) -> InduceScratch<'_> {
            InduceScratch {
                arena: &mut self.arena,
                free: self.len,
                bindings: &mut self.bindings,
                trail: &mut self.trail,
                trail_len: 0,
                stack: &mut self.stack,
                pairs: &mut self.pairs,
                next_variable: self.vars,
                next_invented: INVENTED_BASE,
            }
        }
    }

    /// True when `a` and `b` are the same term through the bindings: they unify without
    /// binding anything.
    fn identical(a: u32, b: u32, s: &mut InduceScratch) -> bool {
        let before = s.trail_len;
        unify_in(a, b, s) == Ok(true) && s.trail_len == before
    }

    /// True when `x` and `y` are the same clause through the bindings up to the order of
    /// their literals.
    fn same_clause(x: u32, y: u32, s: &mut InduceScratch) -> bool {
        let (nx, ny) = (clause_at(x, s).unwrap(), clause_at(y, s).unwrap());
        if clause_body_len(&nx) != clause_body_len(&ny)
            || !identical(clause_head(&nx).unwrap(), clause_head(&ny).unwrap(), s)
        {
            return false;
        }
        let mut taken = [false; MAX_BODY];
        for i in 0..clause_body_len(&nx) {
            let lx = clause_literal(&nx, i).unwrap();
            let partner = (0..clause_body_len(&ny))
                .find(|&j| !taken[j] && identical(lx, clause_literal(&ny, j).unwrap(), s));
            match partner {
                Some(j) => taken[j] = true,
                None => return false,
            }
        }
        true
    }

    /// The exit example: `p(X, Z) ← r(X, Y), s(Y, Z), t(X, Z), u(X)` and the same with
    /// `w(X')` over its own variables. Returns the two clause indices.
    fn exit_pair<const N: usize>(kit: &mut Kit<N>) -> (u32, u32) {
        let (x, y, z) = (kit.var(), kit.var(), kit.var());
        let head = kit.compound(P, &[x, z]);
        let (r, s, t, u) = (
            kit.compound(R, &[x, y]),
            kit.compound(S, &[y, z]),
            kit.compound(T, &[x, z]),
            kit.compound(U, &[x]),
        );
        let ca = kit.clause(head, &[r, s, t, u]);
        let (x2, y2, z2) = (kit.var(), kit.var(), kit.var());
        let head2 = kit.compound(P, &[x2, z2]);
        let (r2, s2, t2, w2) = (
            kit.compound(R, &[x2, y2]),
            kit.compound(S, &[y2, z2]),
            kit.compound(T, &[x2, z2]),
            kit.compound(W, &[x2]),
        );
        let cb = kit.clause(head2, &[r2, s2, t2, w2]);
        (ca, cb)
    }

    #[test]
    fn a_clause_holds_a_head_and_up_to_seven_literals() {
        let seven = [1u32; 7];
        let node = clause(0, &seven).unwrap();
        assert!(is_clause(&node));
        assert_eq!((clause_head(&node), clause_body_len(&node)), (Some(0), 7));
        assert_eq!(clause_literal(&node, 6), Some(1));
        assert_eq!(clause_literal(&node, 7), None, "past the body");
        assert!(clause(0, &[1u32; 8]).is_none(), "eight literals do not fit");
        let fact = clause(5, &[]).unwrap();
        assert_eq!((clause_head(&fact), clause_body_len(&fact)), (Some(5), 0));
        assert_eq!(clause_literal(&fact, 0), None);
        let not = TermNode::compound(CLAUSE, &[]).unwrap();
        assert!(!is_clause(&not), "a clause has at least its head");
        let category = TermNode::compound(CATEGORY_FORWARD, &[1, 2, 3]).unwrap();
        assert!(!is_clause(&category));
        assert_eq!(clause_head(&category), None);
        assert_eq!(clause_body_len(&category), 0);
        assert_eq!(clause_literal(&category, 0), None);
        assert!(!is_clause(&TermNode::constant(CLAUSE)));
    }

    #[test]
    fn size_hash_and_free_variables_read_through_the_bindings() {
        let mut kit = Kit::<16>::new();
        let (x, y) = (kit.var(), kit.var());
        let a = kit.constant(A);
        let fxa = kit.compound(F, &[x, a]);
        let gy = kit.compound(G, &[y]);
        let fga = kit.compound(F, &[gy, a]);
        let x_bound = kit.var();
        let mut s = kit.scratch();
        assert_eq!(size(fxa, s.arena, s.bindings, s.stack), Ok(3));
        assert_eq!(size(fga, s.arena, s.bindings, s.stack), Ok(4));
        let h1 = term_hash(fxa, s.arena, s.bindings, s.stack).unwrap();
        let h2 = term_hash(fga, s.arena, s.bindings, s.stack).unwrap();
        assert_ne!(h1, h2);
        // Bind X to g(Y): f(X, a) reads as f(g(Y), a), the same structure as `fga`.
        assert!(unify_in(x, gy, &mut s).unwrap());
        assert_eq!(size(fxa, s.arena, s.bindings, s.stack), Ok(4));
        assert_eq!(
            term_hash(fxa, s.arena, s.bindings, s.stack),
            Ok(h2),
            "the same structure"
        );
        let mut out = [TERM_NONE; 4];
        let n = free_variables(fxa, s.arena, s.bindings, s.stack, &mut out, 0).unwrap();
        assert_eq!(
            (n, out[0]),
            (1, y),
            "X is bound; Y is free, by its first node"
        );
        let n = free_variables(x_bound, s.arena, s.bindings, s.stack, &mut out, n).unwrap();
        assert_eq!((n, out[1]), (2, x_bound), "appended after Y");
        let n = free_variables(gy, s.arena, s.bindings, s.stack, &mut out, n).unwrap();
        assert_eq!(n, 2, "Y again is not listed twice");
        let mut one = [TERM_NONE; 1];
        assert_eq!(
            free_variables(fga, s.arena, s.bindings, s.stack, &mut one, 2),
            Err(InduceError::BoundExceeded),
            "a length past the slice"
        );
        let mut none = [TERM_NONE; 0];
        assert_eq!(
            free_variables(gy, s.arena, s.bindings, s.stack, &mut none, 0),
            Err(InduceError::BoundExceeded),
            "no room for Y"
        );
        let mut two = [0u32; 2];
        assert_eq!(
            size(fga, s.arena, s.bindings, &mut two),
            Ok(4),
            "two slots walk f(g(Y), a)"
        );
        let mut one_slot = [0u32; 1];
        assert_eq!(
            size(fga, s.arena, s.bindings, &mut one_slot),
            Err(InduceError::BoundExceeded)
        );
        let mut no_slot = [0u32; 0];
        assert_eq!(
            size(a, s.arena, s.bindings, &mut no_slot),
            Err(InduceError::BoundExceeded)
        );
        assert_eq!(
            size(99, s.arena, s.bindings, s.stack),
            Err(InduceError::Malformed)
        );
        assert_eq!(
            size(15, s.arena, s.bindings, s.stack),
            Err(InduceError::Malformed),
            "empty"
        );
        assert_eq!(
            term_hash(99, s.arena, s.bindings, s.stack),
            Err(InduceError::Malformed)
        );
        let mut hole = TermNode::compound(F, &[a]).unwrap();
        hole.children[0] = TERM_NONE;
        s.arena[14] = hole;
        assert_eq!(
            size(14, s.arena, s.bindings, s.stack),
            Err(InduceError::Malformed),
            "a hole"
        );
    }

    #[test]
    fn the_hash_ignores_where_a_term_sits() {
        let mut kit = Kit::<16>::new();
        let a = kit.constant(A);
        let first = kit.compound(F, &[a, a]);
        let x = kit.var();
        let _ = kit.compound(G, &[x]);
        let a2 = kit.constant(A);
        let second = kit.compound(F, &[a2, a2]);
        let s = kit.scratch();
        let h1 = term_hash(first, s.arena, s.bindings, s.stack).unwrap();
        let h2 = term_hash(second, s.arena, s.bindings, s.stack).unwrap();
        assert_eq!(h1, h2);
        let ha = term_hash(a, s.arena, s.bindings, s.stack).unwrap();
        assert_ne!(ha, h1);
        assert_ne!(
            term_hash(x, s.arena, s.bindings, s.stack).unwrap(),
            ha,
            "a variable and a constant differ by kind"
        );
    }

    #[test]
    fn generalisation_follows_plotkin() {
        let mut kit = Kit::<32>::new();
        let (a, b, c) = (kit.constant(A), kit.constant(B), kit.constant(C));
        let faba = kit.compound(F, &[a, b, a]);
        let fcbc = kit.compound(F, &[c, b, c]);
        let faa = kit.compound(F, &[a, a]);
        let fbc = kit.compound(F, &[b, c]);
        let ga = kit.compound(G, &[a]);
        let fga = kit.compound(F, &[ga]);
        let gb = kit.compound(G, &[b]);
        let fgb = kit.compound(F, &[gb]);
        let x = kit.var();
        let mut s = kit.scratch();
        let free = s.free;

        let g = lgg(faba, fcbc, &mut s).unwrap();
        let node = s.arena[g as usize];
        assert_eq!((node.kind, node.functor, node.arity), (TERM_COMPOUND, F, 3));
        assert_eq!(
            node.child(0),
            node.child(2),
            "the same pair (a, c) is one variable"
        );
        assert_eq!(s.arena[node.child(0).unwrap() as usize].kind, TERM_VARIABLE);
        assert_eq!(
            node.child(1),
            Some(b),
            "equal constants stay, as the first's node"
        );
        assert_eq!(s.next_variable, 2, "one variable created");
        assert_eq!(s.free, free.wrapping_add(2), "one compound, one variable");

        let g = lgg(faa, fbc, &mut s).unwrap();
        let node = s.arena[g as usize];
        assert_ne!(
            node.child(0),
            node.child(1),
            "different pairs are different variables"
        );

        let g = lgg(fga, fgb, &mut s).unwrap();
        let node = s.arena[g as usize];
        let inner = s.arena[node.child(0).unwrap() as usize];
        assert_eq!(
            (inner.kind, inner.functor, inner.arity),
            (TERM_COMPOUND, G, 1)
        );
        assert_eq!(
            s.arena[inner.child(0).unwrap() as usize].kind,
            TERM_VARIABLE
        );

        let g = lgg(fga, ga, &mut s).unwrap();
        assert_eq!(
            s.arena[g as usize].kind, TERM_VARIABLE,
            "different functors"
        );
        let g = lgg(x, a, &mut s).unwrap();
        assert_eq!(
            s.arena[g as usize].kind, TERM_VARIABLE,
            "a variable and a constant"
        );
        assert_ne!(g, x, "a new variable, not the old one");
        assert_eq!(
            lgg(faba, faba, &mut s),
            Ok(faba),
            "a term with itself is itself"
        );
        assert_eq!(lgg(a, a, &mut s), Ok(a));
        let a_again = alloc(&mut s, TermNode::constant(A)).unwrap();
        assert_eq!(
            lgg(a_again, a, &mut s),
            Ok(a_again),
            "equal constants in two nodes"
        );

        // Through the bindings: X bound to a generalises with a to a itself.
        assert!(unify_in(x, a, &mut s).unwrap());
        assert_eq!(lgg(x, a, &mut s), Ok(a));
    }

    #[test]
    fn generalisation_reports_every_bound_and_restores() {
        let mut kit = Kit::<12>::new();
        let (a, b) = (kit.constant(A), kit.constant(B));
        let faa = kit.compound(F, &[a, a]);
        let fbb = kit.compound(F, &[b, b]);
        let fab = kit.compound(F, &[a, b]);
        let fba = kit.compound(F, &[b, a]);
        let mut s = kit.scratch();
        let free = s.free;
        let snapshot: [TermNode; 12] = core::array::from_fn(|i| s.arena[i]);

        // One pair (a, b) twice: one variable; the pair table needs one entry.
        let mut one = [[0u32; 3]; 1];
        let mut inner = InduceScratch {
            arena: &mut *s.arena,
            free,
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 0,
            stack: &mut *s.stack,
            pairs: &mut one,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert!(lgg(faa, fbb, &mut inner).is_ok(), "one pair fits one entry");
        assert_eq!(
            lgg(fab, fba, &mut inner),
            Err(InduceError::PairsFull),
            "(a, b) and (b, a) need two"
        );
        assert_eq!(
            inner.free,
            free.wrapping_add(2),
            "the first call's nodes stay"
        );
        assert_eq!(inner.next_variable, 1);
        assert_eq!(
            &inner.arena[inner.free..],
            &snapshot[inner.free..],
            "zeroed after the mark"
        );

        let mut tight = [Binding::UNBOUND; 0];
        let mut inner = InduceScratch {
            arena: &mut *s.arena,
            free,
            bindings: &mut tight,
            trail: &mut *s.trail,
            trail_len: 0,
            stack: &mut *s.stack,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert_eq!(
            lgg(faa, fbb, &mut inner),
            Err(InduceError::BoundExceeded),
            "no variable can be numbered"
        );
        assert_eq!(inner.free, free);

        let mut three = [0u32; 3];
        let mut inner = InduceScratch {
            arena: &mut *s.arena,
            free,
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 0,
            stack: &mut three,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert_eq!(
            lgg(a, b, &mut inner),
            Err(InduceError::BoundExceeded),
            "four entries"
        );
        let mut four = [0u32; 4];
        let mut inner = InduceScratch {
            arena: &mut *s.arena,
            free,
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 0,
            stack: &mut four,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert!(lgg(a, b, &mut inner).is_ok(), "four entries hold one pair");
        assert_eq!(
            lgg(faa, fbb, &mut inner),
            Err(InduceError::BoundExceeded),
            "the children need a second pair on the stack"
        );
        assert_eq!(
            inner.free,
            free.wrapping_add(1),
            "the variable of the first call stays"
        );
        assert_eq!(&inner.arena[inner.free..], &snapshot[inner.free..]);

        // Arena full: two nodes free hold one compound and one variable, not two variables.
        let mut inner = InduceScratch {
            arena: &mut s.arena[..free.wrapping_add(2)],
            free,
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 0,
            stack: &mut *s.stack,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert_eq!(lgg(fab, fba, &mut inner), Err(InduceError::ArenaFull));
        assert_eq!((inner.free, inner.next_variable), (free, 0));
        assert_eq!(inner.arena[free], TermNode::default());
        assert_eq!(inner.arena[free.wrapping_add(1)], TermNode::default());
        assert!(lgg(faa, fbb, &mut inner).is_ok());

        assert_eq!(lgg(99, a, &mut s), Err(InduceError::Malformed));
        assert_eq!(
            lgg(a, 11, &mut s),
            Err(InduceError::Malformed),
            "an empty node"
        );
        let mut hole = TermNode::compound(F, &[a, a]).unwrap();
        hole.children[1] = TERM_NONE;
        s.arena[11] = hole;
        assert_eq!(
            lgg(11, faa, &mut s),
            Err(InduceError::Malformed),
            "a hole in a child"
        );
    }

    #[test]
    fn absorption_folds_a_body_into_one_literal_and_resolves_back() {
        // c2 = q(X) ← r(X, Y); c = p(Z) ← r(a, b), s(Z). Absorbing: p(Z) ← q(a), s(Z).
        let mut kit = Kit::<32>::new();
        let (x, y, z) = (kit.var(), kit.var(), kit.var());
        let (a, b) = (kit.constant(A), kit.constant(B));
        let qx = kit.compound(Q, &[x]);
        let rxy = kit.compound(R, &[x, y]);
        let c2 = kit.clause(qx, &[rxy]);
        let pz = kit.compound(P, &[z]);
        let rab = kit.compound(R, &[a, b]);
        let sz = kit.compound(S, &[z]);
        let c = kit.clause(pz, &[rab, sz]);
        let mut s = kit.scratch();
        let out = absorb(c2, c, &mut s).unwrap();
        let node = s.arena[out as usize];
        assert_eq!(clause_body_len(&node), 2);
        assert_eq!(clause_head(&node), Some(pz));
        assert_eq!(clause_literal(&node, 0), Some(qx), "c2's head, first");
        assert_eq!(
            clause_literal(&node, 1),
            Some(sz),
            "then the unmatched literal"
        );
        assert_eq!(s.trail_len, 2, "X = a and Y = b");
        assert_eq!(deref(x, s.arena, s.bindings), Some(a));
        let back = resolve_definite(out, c2, &mut s).unwrap();
        assert!(same_clause(back, c, &mut s), "the identity");
        assert_eq!(s.trail_len, 2, "resolving bound nothing new");
    }

    #[test]
    fn absorption_refuses_a_fact_a_missing_partner_and_malformed_input() {
        let mut kit = Kit::<32>::new();
        let (x, z) = (kit.var(), kit.var());
        let a = kit.constant(A);
        let qx = kit.compound(Q, &[x]);
        let fact = kit.clause(qx, &[]);
        let tx = kit.compound(T, &[x]);
        let no_partner = kit.clause(qx, &[tx]);
        let pz = kit.compound(P, &[z]);
        let ra = kit.compound(R, &[a]);
        let c = kit.clause(pz, &[ra]);
        let rx = kit.compound(R, &[x]);
        let c2 = kit.clause(qx, &[rx]);
        let mut s = kit.scratch();
        let free = s.free;
        assert_eq!(
            absorb(fact, c, &mut s),
            Err(InduceError::NoMatch),
            "a fact absorbs nothing"
        );
        assert_eq!(
            absorb(no_partner, c, &mut s),
            Err(InduceError::NoMatch),
            "t(X) has none"
        );
        assert_eq!(
            absorb(pz, c, &mut s),
            Err(InduceError::Malformed),
            "not a clause"
        );
        assert_eq!(absorb(c2, 99, &mut s), Err(InduceError::Malformed));
        assert_eq!((s.free, s.trail_len), (free, 0));
        assert_eq!(
            s.bindings[0],
            Binding::UNBOUND,
            "X = a from the refused matching undone"
        );
        // With no node free the arena is full once the matching has bound X = a.
        let mut inner = InduceScratch {
            arena: &mut s.arena[..free],
            free,
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 0,
            stack: &mut *s.stack,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert_eq!(absorb(c2, c, &mut inner), Err(InduceError::ArenaFull));
        assert_eq!(inner.trail_len, 0, "the binding X = a was undone");
        assert_eq!(inner.bindings[0], Binding::UNBOUND);
        let mut one = [0u32; 1];
        let mut inner = InduceScratch {
            arena: &mut *s.arena,
            free,
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 0,
            stack: &mut one,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert_eq!(absorb(c2, c, &mut inner), Err(InduceError::BoundExceeded));
        let mut inner = InduceScratch {
            arena: &mut *s.arena,
            free,
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 65,
            stack: &mut *s.stack,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert_eq!(
            absorb(c2, c, &mut inner),
            Err(InduceError::BoundExceeded),
            "a trail length past the trail"
        );
        assert_eq!(inner.trail_len, 65, "left as it was");
        assert!(absorb(c2, c, &mut s).is_ok(), "and with room it applies");
    }

    #[test]
    fn identification_recovers_the_clause_a_literal_stands_for() {
        // c1 = p(X) ← q(X), s(X); c = p(a) ← r(a), s(a). Identified: q(a) ← r(a).
        let mut kit = Kit::<32>::new();
        let x = kit.var();
        let a = kit.constant(A);
        let px = kit.compound(P, &[x]);
        let qx = kit.compound(Q, &[x]);
        let sx = kit.compound(S, &[x]);
        let c1 = kit.clause(px, &[qx, sx]);
        let pa = kit.compound(P, &[a]);
        let ra = kit.compound(R, &[a]);
        let sa = kit.compound(S, &[a]);
        let c = kit.clause(pa, &[ra, sa]);
        let mut s = kit.scratch();
        let out = identify(c1, c, &mut s).unwrap();
        let node = s.arena[out as usize];
        assert_eq!(clause_head(&node), Some(qx));
        assert_eq!(
            (clause_body_len(&node), clause_literal(&node, 0)),
            (1, Some(ra))
        );
        assert_eq!(
            deref(x, s.arena, s.bindings),
            Some(a),
            "X = a from the heads"
        );
        let back = resolve_definite(c1, out, &mut s).unwrap();
        assert!(same_clause(back, c, &mut s), "the identity");
        let back_node = s.arena[back as usize];
        assert_eq!(
            (clause_literal(&back_node, 0), clause_literal(&back_node, 1)),
            (Some(ra), Some(sx)),
            "the rule's body in place of q(X), then the rest"
        );
    }

    #[test]
    fn identification_needs_exactly_one_unmatched_literal_and_unifying_heads() {
        let mut kit = Kit::<32>::new();
        let x = kit.var();
        let (a, b) = (kit.constant(A), kit.constant(B));
        let px = kit.compound(P, &[x]);
        let qx = kit.compound(Q, &[x]);
        let sx = kit.compound(S, &[x]);
        let tx = kit.compound(T, &[x]);
        let pa = kit.compound(P, &[a]);
        let sa = kit.compound(S, &[a]);
        let sb = kit.compound(S, &[b]);
        let p_atom = kit.constant(P);
        let subsumed = kit.clause(px, &[sx]);
        let two_unmatched = kit.clause(px, &[qx, tx, sx]);
        let c = kit.clause(pa, &[sa]);
        let other_head = kit.clause(p_atom, &[sa]);
        let other_body = kit.clause(pa, &[sb]);
        let c1 = kit.clause(px, &[qx, sx]);
        let mut s = kit.scratch();
        let free = s.free;
        assert_eq!(
            identify(subsumed, c, &mut s),
            Err(InduceError::NoMatch),
            "nothing unmatched"
        );
        assert_eq!(
            identify(two_unmatched, c, &mut s),
            Err(InduceError::NoMatch),
            "two"
        );
        assert_eq!(
            identify(c1, other_head, &mut s),
            Err(InduceError::NoMatch),
            "heads clash"
        );
        assert_eq!(
            identify(c1, other_body, &mut s),
            Err(InduceError::NoMatch),
            "s(a) vs s(b)"
        );
        assert_eq!(identify(px, c, &mut s), Err(InduceError::Malformed));
        assert_eq!(identify(c1, px, &mut s), Err(InduceError::Malformed));
        assert_eq!((s.free, s.trail_len), (free, 0), "every refusal restored");
        assert_eq!(s.bindings[0], Binding::UNBOUND);
        let mut none = [0u32; 0];
        let mut inner = InduceScratch {
            arena: &mut *s.arena,
            free,
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 0,
            stack: &mut none,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert_eq!(identify(c1, c, &mut inner), Err(InduceError::BoundExceeded));
        let out = identify(c1, c, &mut s).unwrap();
        assert_eq!(
            clause_body_len(&s.arena[out as usize]),
            0,
            "q(a) ← , a fact"
        );
    }

    #[test]
    fn intra_construction_invents_a_predicate_whose_definitions_resolve_back() {
        let mut kit = Kit::<64>::new();
        let (ca, cb) = exit_pair(&mut kit);
        let (cc, cd) = exit_pair(&mut kit);
        let mut s = kit.scratch();
        let free = s.free;
        let size_before = size(ca, s.arena, s.bindings, s.stack)
            .unwrap()
            .saturating_add(size(cb, s.arena, s.bindings, s.stack).unwrap());
        assert_eq!(size_before, 30);
        let inv = intra_construct(ca, cb, &mut s).unwrap();
        assert_eq!(
            (inv.predicate, inv.arguments, inv.shared),
            (INVENTED_BASE, 1, 3),
            "q(X): X is the one scope variable the differing literals use"
        );
        assert_eq!(s.next_invented, INVENTED_BASE.wrapping_add(1));
        assert_eq!(s.free, free.wrapping_add(4), "q(X) and three clauses");
        let common = s.arena[inv.common as usize];
        assert_eq!(clause_body_len(&common), 4, "r, s, t and q(X)");
        let q = clause_literal(&common, 3).unwrap();
        let qn = s.arena[q as usize];
        assert_eq!(
            (qn.kind, qn.functor, qn.arity),
            (TERM_COMPOUND, INVENTED_BASE, 1)
        );
        let x_of_a = s.arena[qn.child(0).unwrap() as usize];
        assert_eq!(x_of_a.kind, TERM_VARIABLE);
        for d in inv.definitions {
            let dn = s.arena[d as usize];
            assert_eq!(
                clause_head(&dn),
                Some(q),
                "the definitions share the invented literal"
            );
            assert_eq!(clause_body_len(&dn), 1);
        }
        let size_after = [inv.common, inv.definitions[0], inv.definitions[1]]
            .iter()
            .map(|&c| size(c, s.arena, s.bindings, s.stack).unwrap())
            .sum::<u32>();
        assert_eq!(size_after, 25, "fifteen, five and five");
        let back_a = resolve_definite(inv.common, inv.definitions[0], &mut s).unwrap();
        assert!(
            same_clause(back_a, ca, &mut s),
            "p ← A, q resolved with q ← B₁ is ca"
        );
        let back_b = resolve_definite(inv.common, inv.definitions[1], &mut s).unwrap();
        assert!(same_clause(back_b, cb, &mut s), "and with q ← B₂ is cb");
        assert_eq!(
            term_hash(inv.common, s.arena, s.bindings, s.stack),
            Ok(0x21a1_c619)
        );
        assert_eq!(
            term_hash(inv.definitions[0], s.arena, s.bindings, s.stack),
            Ok(0x3550_790c)
        );
        assert_eq!(
            term_hash(inv.definitions[1], s.arena, s.bindings, s.stack),
            Ok(0x15aa_b9e7)
        );

        // A second invention, over its own variables, takes the next id.
        let second = intra_construct(cc, cd, &mut s).unwrap();
        assert_eq!(second.predicate, INVENTED_BASE.wrapping_add(1));
        assert_eq!(s.next_invented, INVENTED_BASE.wrapping_add(2));
        assert_eq!(
            term_hash(inv.common, s.arena, s.bindings, s.stack),
            Ok(0x21a1_c619),
            "the first invention's clause is unchanged by the second"
        );
    }

    #[test]
    fn intra_construction_refuses_and_restores() {
        let mut kit = Kit::<40>::new();
        let (x, y) = (kit.var(), kit.var());
        let (a, b) = (kit.constant(A), kit.constant(B));
        let px = kit.compound(P, &[x]);
        let rx = kit.compound(R, &[x]);
        let ux = kit.compound(U, &[x]);
        let py = kit.compound(P, &[y]);
        let ry = kit.compound(R, &[y]);
        let wy = kit.compound(W, &[y]);
        let ty = kit.compound(T, &[y]);
        let pa = kit.compound(P, &[a]);
        let pb = kit.compound(P, &[b]);
        let ca = kit.clause(px, &[rx, ux]);
        let cb = kit.clause(py, &[ry, wy]);
        let nothing_shared = kit.clause(py, &[ty, wy]);
        let subsumes = kit.clause(py, &[ry]);
        let no_body = kit.clause(py, &[]);
        let heads_clash_a = kit.clause(pa, &[rx, ux]);
        let heads_clash_b = kit.clause(pb, &[ry, wy]);
        let mut s = kit.scratch();
        let free = s.free;
        let snapshot: [TermNode; 40] = core::array::from_fn(|i| s.arena[i]);
        assert_eq!(
            intra_construct(ca, nothing_shared, &mut s),
            Err(InduceError::NothingToInvent)
        );
        assert_eq!(
            intra_construct(ca, subsumes, &mut s),
            Err(InduceError::NothingToInvent)
        );
        assert_eq!(
            intra_construct(subsumes, ca, &mut s),
            Err(InduceError::NothingToInvent)
        );
        assert_eq!(
            intra_construct(ca, no_body, &mut s),
            Err(InduceError::NothingToInvent)
        );
        assert_eq!(
            intra_construct(heads_clash_a, heads_clash_b, &mut s),
            Err(InduceError::NoMatch)
        );
        assert_eq!(intra_construct(px, cb, &mut s), Err(InduceError::Malformed));
        assert_eq!(intra_construct(ca, 99, &mut s), Err(InduceError::Malformed));
        assert_eq!(
            (s.free, s.trail_len, s.next_invented),
            (free, 0, INVENTED_BASE)
        );
        assert!(s.bindings.iter().all(|&b| b == Binding::UNBOUND));
        assert_eq!(&s.arena[..], &snapshot[..]);

        s.next_invented = INVENTED_LIMIT;
        assert_eq!(
            intra_construct(ca, cb, &mut s),
            Err(InduceError::InventionsExhausted)
        );
        assert_eq!(s.trail_len, 0, "the heads' binding was undone");
        s.next_invented = INVENTED_BASE;

        // Four nodes are needed; three free is ArenaFull after three were written.
        let mut inner = InduceScratch {
            arena: &mut s.arena[..free.wrapping_add(3)],
            free,
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 0,
            stack: &mut *s.stack,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert_eq!(
            intra_construct(ca, cb, &mut inner),
            Err(InduceError::ArenaFull)
        );
        assert_eq!(
            (inner.free, inner.trail_len, inner.next_invented),
            (free, 0, INVENTED_BASE)
        );
        assert_eq!(&inner.arena[free..], &snapshot[free..free.wrapping_add(3)]);
        let mut none = [0u32; 0];
        let mut inner = InduceScratch {
            arena: &mut *s.arena,
            free,
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 0,
            stack: &mut none,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert_eq!(
            intra_construct(ca, cb, &mut inner),
            Err(InduceError::BoundExceeded)
        );
        assert!(
            intra_construct(ca, cb, &mut s).is_ok(),
            "and with room it applies"
        );
    }

    #[test]
    fn an_invented_literal_takes_at_most_eight_arguments() {
        // Head p(V1..V8); shared r(V1..V8); B₁ = u(V1..V8), B₂ = w(V9) with V9 in the head
        // of neither... so it is not linked. Make it linked: the shared literal s(V9).
        let mut kit = Kit::<64>::new();
        let mut v = [0u32; 9];
        for slot in v.iter_mut() {
            *slot = kit.var();
        }
        let head = kit.compound(P, &v[..8]);
        let shared = kit.compound(R, &v[..8]);
        let shared9 = kit.compound(S, &[v[8]]);
        let b1 = kit.compound(U, &v[..8]);
        let b2 = kit.compound(W, &[v[8]]);
        let ca = kit.clause(head, &[shared, shared9, b1]);
        let cb = kit.clause(head, &[shared, shared9, b2]);
        let mut s = kit.scratch();
        let free = s.free;
        assert_eq!(
            intra_construct(ca, cb, &mut s),
            Err(InduceError::TooManyArguments)
        );
        assert_eq!((s.free, s.trail_len), (free, 0));
        // With B₂ = w(V1) the arguments are V1..V8: exactly eight.
        let b2 = alloc(&mut s, TermNode::compound(W, &[v[0]]).unwrap()).unwrap();
        let cb = alloc(&mut s, clause(head, &[shared, shared9, b2]).unwrap()).unwrap();
        let inv = intra_construct(ca, cb, &mut s).unwrap();
        assert_eq!(inv.arguments, 8);
    }

    #[test]
    fn definite_resolution_replaces_the_first_unifying_literal_in_place() {
        // goal = p ← r(X), s(X), t(X); rule = s(a) ← u(a), w(a). Resolvent: p ← r(a), u(a), w(a), t(a).
        let mut kit = Kit::<32>::new();
        let x = kit.var();
        let a = kit.constant(A);
        let p = kit.constant(P);
        let rx = kit.compound(R, &[x]);
        let sx = kit.compound(S, &[x]);
        let tx = kit.compound(T, &[x]);
        let goal = kit.clause(p, &[rx, sx, tx]);
        let sa = kit.compound(S, &[a]);
        let ua = kit.compound(U, &[a]);
        let wa = kit.compound(W, &[a]);
        let rule = kit.clause(sa, &[ua, wa]);
        let seven = kit.clause(p, &[rx, sx, tx, rx, rx, rx, rx]);
        let no_match = kit.clause(p, &[rx, tx]);
        let mut s = kit.scratch();
        let free = s.free;
        let out = resolve_definite(goal, rule, &mut s).unwrap();
        let node = s.arena[out as usize];
        assert_eq!(clause_head(&node), Some(p));
        assert_eq!(clause_body_len(&node), 4);
        assert_eq!(
            [0, 1, 2, 3].map(|i| clause_literal(&node, i).unwrap()),
            [rx, ua, wa, tx]
        );
        assert_eq!(deref(x, s.arena, s.bindings), Some(a));
        assert_eq!(s.trail_len, 1);
        assert_eq!(
            resolve_definite(seven, rule, &mut s),
            Err(InduceError::BodyFull),
            "six kept and two added are eight"
        );
        assert_eq!(s.trail_len, 1, "the binding of the refused step was undone");
        assert_eq!(s.free, free.wrapping_add(1));
        assert_eq!(
            resolve_definite(no_match, rule, &mut s),
            Err(InduceError::NoMatch)
        );
        assert_eq!(
            resolve_definite(p, rule, &mut s),
            Err(InduceError::Malformed)
        );
        assert_eq!(
            resolve_definite(goal, 99, &mut s),
            Err(InduceError::Malformed)
        );
        let mut inner = InduceScratch {
            arena: &mut s.arena[..free.wrapping_add(1)],
            free: free.wrapping_add(1),
            bindings: &mut *s.bindings,
            trail: &mut *s.trail,
            trail_len: 1,
            stack: &mut *s.stack,
            pairs: &mut *s.pairs,
            next_variable: 0,
            next_invented: INVENTED_BASE,
        };
        assert_eq!(
            resolve_definite(goal, rule, &mut inner),
            Err(InduceError::ArenaFull)
        );
        let mut hole = clause(p, &[rx]).unwrap();
        hole.children[0] = TERM_NONE;
        s.arena[30] = hole;
        assert_eq!(
            resolve_definite(30, rule, &mut s),
            Err(InduceError::Malformed),
            "a hole"
        );
        assert_eq!(
            resolve_definite(goal, 30, &mut s),
            Err(InduceError::Malformed)
        );
    }
}

/// Property walks (ADR-0030): over random clause pairs from a small vocabulary, every
/// invention's definitions resolve back to the inputs, every absorption and identification
/// resolves back, and every refusal leaves the arena, the bindings and the counters as they
/// were.
#[cfg(test)]
mod prop {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    const PREDICATES: [u32; 3] = [0x101, 0x102, 0x103];
    const CONSTANTS: [u32; 2] = [0x200, 0x201];

    struct Store {
        arena: [TermNode; 96],
        len: usize,
    }

    impl Store {
        fn push(&mut self, node: TermNode) -> u32 {
            let i = self.len;
            self.arena[i] = node;
            self.len = self.len.wrapping_add(1);
            i as u32
        }

        /// A random clause over its own two variables: a head over `0x100` and one to four
        /// literals, each a predicate of arity one or two over constants and the variables.
        fn clause(&mut self, rng: &mut Lcg) -> u32 {
            let vars = [
                self.push(TermNode::variable(0)),
                self.push(TermNode::variable(1)),
            ];
            let arg = |store: &mut Store, rng: &mut Lcg| -> u32 {
                match rng.below(4) {
                    0 | 1 => vars[rng.below(2) as usize],
                    _ => store.push(TermNode::constant(rng.pick(&CONSTANTS))),
                }
            };
            let (h0, h1) = (arg(self, rng), arg(self, rng));
            let head = self.push(TermNode::compound(0x100, &[h0, h1]).unwrap());
            let count = rng.below(4).wrapping_add(1) as usize;
            let mut body = [TERM_NONE; 4];
            for slot in body.iter_mut().take(count) {
                let f = rng.pick(&PREDICATES);
                let a0 = arg(self, rng);
                *slot = if rng.below(2) == 0 {
                    self.push(TermNode::compound(f, &[a0]).unwrap())
                } else {
                    let a1 = arg(self, rng);
                    self.push(TermNode::compound(f, &[a0, a1]).unwrap())
                };
            }
            self.push(clause(head, &body[..count]).unwrap())
        }
    }

    fn same(x: u32, y: u32, s: &mut InduceScratch) -> bool {
        fn identical(a: u32, b: u32, s: &mut InduceScratch) -> bool {
            let before = s.trail_len;
            unify_in(a, b, s) == Ok(true) && s.trail_len == before
        }
        let (nx, ny) = (clause_at(x, s).unwrap(), clause_at(y, s).unwrap());
        if clause_body_len(&nx) != clause_body_len(&ny)
            || !identical(clause_head(&nx).unwrap(), clause_head(&ny).unwrap(), s)
        {
            return false;
        }
        let mut taken = [false; MAX_BODY];
        for i in 0..clause_body_len(&nx) {
            let lx = clause_literal(&nx, i).unwrap();
            let partner = (0..clause_body_len(&ny))
                .find(|&j| !taken[j] && identical(lx, clause_literal(&ny, j).unwrap(), s));
            match partner {
                Some(j) => taken[j] = true,
                None => return false,
            }
        }
        true
    }

    #[test]
    fn every_success_resolves_back_and_every_refusal_restores() {
        let mut rng = Lcg::new(0x7A1E_0021);
        let mut inventions = 0u32;
        let mut absorptions = 0u32;
        let mut identifications = 0u32;
        for _ in 0..3_000 {
            let mut store = Store {
                arena: [TermNode::default(); 96],
                len: 0,
            };
            let ca = store.clause(&mut rng);
            // The second clause's variables are numbered apart.
            let cb = {
                let mark = store.len;
                let c = store.clause(&mut rng);
                for node in store.arena[mark..store.len].iter_mut() {
                    if node.kind == TERM_VARIABLE {
                        node.functor = node.functor.wrapping_add(2);
                    }
                }
                c
            };
            let snapshot = store.arena;
            let mut bindings = [Binding::UNBOUND; 8];
            let mut trail = [0u32; 32];
            let mut stack = [0u32; 64];
            let mut pairs = [[0u32; 3]; 8];
            for op in 0..3u32 {
                let mut s = InduceScratch {
                    arena: &mut store.arena,
                    free: store.len,
                    bindings: &mut bindings,
                    trail: &mut trail,
                    trail_len: 0,
                    stack: &mut stack,
                    pairs: &mut pairs,
                    next_variable: 4,
                    next_invented: INVENTED_BASE,
                };
                let outcome: Result<(), InduceError> = match op {
                    0 => intra_construct(ca, cb, &mut s).map(|inv| {
                        inventions = inventions.wrapping_add(1);
                        assert_eq!(inv.predicate, INVENTED_BASE);
                        assert_eq!(s.next_invented, INVENTED_BASE.wrapping_add(1));
                        let back =
                            resolve_definite(inv.common, inv.definitions[0], &mut s).unwrap();
                        assert!(same(back, ca, &mut s));
                        let back =
                            resolve_definite(inv.common, inv.definitions[1], &mut s).unwrap();
                        assert!(same(back, cb, &mut s));
                    }),
                    1 => absorb(ca, cb, &mut s).map(|out| {
                        absorptions = absorptions.wrapping_add(1);
                        let back = resolve_definite(out, ca, &mut s).unwrap();
                        assert!(same(back, cb, &mut s));
                    }),
                    _ => identify(ca, cb, &mut s).map(|out| {
                        identifications = identifications.wrapping_add(1);
                        let back = resolve_definite(ca, out, &mut s).unwrap();
                        assert!(same(back, cb, &mut s));
                    }),
                };
                if outcome.is_err() {
                    assert_eq!(s.free, store.len);
                    assert_eq!(s.trail_len, 0);
                    assert_eq!(s.next_invented, INVENTED_BASE);
                    assert!(s.bindings.iter().all(|&b| b == Binding::UNBOUND));
                    assert_eq!(&s.arena[..], &snapshot[..]);
                }
                store.arena = snapshot;
                bindings = [Binding::UNBOUND; 8];
            }
        }
        assert!(inventions > 100, "inventions: {inventions}");
        assert!(absorptions > 100, "absorptions: {absorptions}");
        assert!(identifications > 30, "identifications: {identifications}");
    }
}
