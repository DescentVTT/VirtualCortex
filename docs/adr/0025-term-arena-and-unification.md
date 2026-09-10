---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0016
---

# ADR-0025: A term arena and first-order unification for `cortex-reasoning` — a second record under ADR-0016's test, bindings in a caller's table, a trail undone on failure, bounds that are results

## Context and Problem Statement

`cortex-reasoning` resolved propositional clauses: literals were `u32` atoms and `resolve` cancelled a complementary pair. Whitepaper §6.10 step 3 said first-order unification "needs a term arena" and §11.1 carried it as an open question: terms, variables and bindings that no 64-byte rule node can hold. Brief 014 asked for the second record and its admission under [ADR-0016](0016-thirty-two-crate-architecture.md)'s test, the layout, the binding discipline (a trail in a caller-provided slice, undone on failure), the occurs check, the work-stack bound and a result type with `Unified`, `Clash`, `OccursCheck` and `BoundExceeded`, how a first-order literal names a term and keeps its sign, and a resolution step that unifies the complementary pair; and it forbade allocation, recursion without a bound, a new crate and any change to `SymbolicRuleNode` beyond a format bump.

## Decision Drivers

- ADR-0016's test for a second record: a named gap (§11.1), one owner (§5.2.30 owns deduction), the mechanism in §8.8 with the layout, widths that fit, no boundary crossed.
- Rule L-3: a term holds indices, never strings; functors and constants are `cortex-symbolic` concept ids.
- TC-5 and `no_std`: no allocation, no recursion; the work stack and the trail are the caller's slices and their lengths are the bounds.
- `cortex-arithmetic`'s precedent: failure is a value the caller reads, not a panic; "does not unify" and "bound exceeded" are different values.
- [ADR-0022](0022-synapse-fan-out-and-stdp.md)'s encoding: every stored index is index + 1, so zero is none and a zeroed arena is empty.
- Determinism (§8.3): the same terms and bindings give the same result and the same table.

## Considered Options

1. Variables as de Bruijn indices resolved by position, bindings in the term nodes themselves.
2. **Variables numbered into a caller's binding table; bindings never in the arena, so unification reads the arena and writes only the table and the trail; an explicit work stack of term pairs whose length is the recursion bound; the occurs check as a depth-first walk in the free part of the same stack; failure undoes the trail; a first-order literal is a term index + 1 with the sign in bit 31.**
3. Substitution by copying terms into fresh nodes (a resolvent as new terms).

## Decision Outcome

Option 2.

- **Record.** `TermNode`, 64 bytes: `kind: u8` (`TERM_EMPTY` 0, `TERM_CONSTANT` 1, `TERM_VARIABLE` 2, `TERM_COMPOUND` 3), `arity: u8` (0 to `MAX_ARITY` = 8), `_pad: u16`, `functor: u32` (a concept id for a constant or compound; the variable's number for a variable), `children: [u32; 8]` (argument term index + 1; 0 none), 24 reserved. `constant`, `variable`, `compound` (refusing more than eight arguments or an index the encoding cannot hold) and `child` (decoding) build and read it. Admitted under ADR-0016's test: the gap is §11.1's, the owner is §5.2.30, the mechanism is Robinson's unification in §8.8, every width fits, no boundary moves; the crate count stays thirty-two.
- **Bindings.** `Binding(u32)`, index + 1 of the bound term, `UNBOUND` zero, in a slice the caller owns and indexes by variable number. `deref` follows a chain of bindings to a non-variable or an unbound variable, bounded by the arena length so that a cyclic table terminates, and returns `None` for an index outside the arena or a variable outside the table.
- **Unification.** `unify(a, b, arena, bindings, trail, stack) -> (UnifyResult, bound)`: an iterative loop over a stack of pairs (two entries per pair); a variable against a term binds it after the occurs check, which walks depth-first in the part of the stack above the pairs; constants unify when equal; compounds when functor and arity agree, pushing the argument pairs; a constant against a compound clashes. `Unified` returns how many variables were bound, which are the first entries of the trail; `Clash`, `OccursCheck`, `BoundExceeded` (the stack or the trail too small) and `Malformed` (an index outside the arena, a variable outside the table, an empty node) undo the trail first, so the table is as it was. Bindings already in the table are respected, so successive calls accumulate one substitution; `undo` reverts a prefix of the trail when the caller wants a substitution gone.
- **Literals.** `literal_of_term(term, negated)` = term index + 1 with bit 31 the sign; `term_of_literal` decodes; `LITERAL_NONE` stays the absent literal and `EMPTY_CLAUSE` stays `(0, 0)`. A clause set is propositional (atoms are concept ids) or first-order (atoms are term indices + 1), never both; the propositional `resolve`, which compares atoms, is the special case in which equal terms are identical nodes.
- **Resolution.** `resolve_first_order(a, b, arena, bindings, trail, stack) -> Result<Clause, UnifyResult>` tries the four literal pairs in the propositional order, skipping pairs of equal sign, and returns the two remaining literals for the first pair that unifies; the bindings stay in the table, so the resolvent is read through them (`deref`), and a proof accumulates one substitution. `Err(Clash)` when no pair unifies; the first other failure otherwise. `SymbolicRuleNode::record_resolvent(resolvent, parent_idx, other_idx, depth)` stores it, and the propositional `apply_resolution` is now `resolve` followed by it.

### Consequences

- Good: Socrates is mortal in two steps: `{¬human(X), mortal(X)}` with `{human(s)}` binds `X := s` and leaves `{mortal(X)}`; that with `{¬mortal(s)}` reaches the empty clause, and the node reports the refutation.
- Good: every bound is a parameter of the caller and every exceeded bound is a result; the deepest term this crate can unify is the caller's stack length in pairs.
- Good: no allocation, no recursion, no `unsafe`, no crate, no dependency; the arena is one more 64-byte record under the same rules as every other.
- Bad: a resolvent is not standalone: its meaning depends on the binding table, which the caller keeps for the whole proof; copying a resolvent into fresh terms (option 3) is what a proof store needs, and is Specified.
- Bad: one binding table per proof, so two proofs in flight need two tables; the variable numbers of two clause sets must not collide, which is the caller's renaming (standardising apart, Specified).
- Bad: arity eight; a wider predicate is two nested compounds.

## Alternatives considered and why rejected

- **Option 1** puts the substitution in the arena, so a failed unification would have to restore nodes and two proofs could not share terms.
- **Option 3** needs free nodes for every step, an allocator in a state crate; the binding table is the substitution and costs four bytes per variable.
- **Recursive unification** has no bound but the stack; the brief and TC-5 exclude it.

## Confirmation

Eight tests in `cortex-reasoning` (`term.rs`): the record is one cache line and a zeroed node is empty; constants unify with themselves only; a variable binds and is dereferenced through a chain, and a clash after a binding changes nothing; the occurs check refuses `X = f(X)` directly and through a chain and leaves the earlier binding; nested compounds unify with consistent bindings and refuse inconsistent ones with the trail undone, and functor and arity clashes are clashes; an exceeded stack, an exceeded trail, an empty stack, an index outside the arena and a variable outside the table are results and bind nothing; Socrates is mortal by two first-order resolution steps and the wrong constant or equal signs are refused; unification is deterministic. `npx spec-guard` asserts `TermNode` and `fn unify` exist.
