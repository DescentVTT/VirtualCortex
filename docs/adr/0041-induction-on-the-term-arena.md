---
status: accepted
date: 2026-09-13
decision-makers: VirtualCortex maintainers
depends-on: ADR-0025
---

# ADR-0041: Induction on the term arena — a definite clause is a term over a reserved functor; Plotkin's least general generalisation and the inverse-resolution operators (absorption, identification, intra-construction with a predicate invented from a reserved band) as bounded rules over caller slices, each output resolving back to its inputs by one definite-clause resolution step; first-fit matching; every failure restoring the scratch

## Context and Problem Statement

`cortex-reasoning` deduced and never induced. Its clauses were pairs of propositional literals (`Clause = (u32, u32)`, `resolve`, `apply_resolution`), its terms lived on the arena of [ADR-0025](0025-term-arena-and-unification.md) with `unify`, `deref`, `undo` and `resolve_first_order` over two-literal clauses, and since [ADR-0040](0040-categorial-reduction.md) categories were terms. No rule generalised two terms, inverted a resolution step, measured a term or hashed one; whitepaper §5.2.30's Specified list named "clause search, standardising apart, a proof store, constraint propagation, type raising and a chart" and induction appeared in no document. The proposal of brief 021 asked for "inductive logic operators over `TermNode` slices: Absorption ($V \leftarrow p \land A$, $W \leftarrow A$), Identification (inverse substitution from instances to general clauses), Intra-Construction (factor out shared sub-terms across multiple observed clauses to invent a brand new latent predicate $Q$)", "bound the search space using `ParseScratch` memory arenas with zero heap allocation", and asked whether "inductive logic operators require extending `TermNode` or implementing dedicated combinatorial algorithms over caller scratch slices".

Three facts shaped the answer. A clause of terms needs no new record: a compound over a reserved functor holds a head and up to seven body literals in the eight child slots `TermNode` has, as a category holds its three. The inverse-resolution operators of Muggleton and Buntine (1988) are defined by one identity, that resolving their outputs reproduces their inputs, and that identity is a test once the crate has one definite-clause resolution step; their non-determinism (which literals to pair, which terms to turn into variables) is what makes them a search in the literature, and a deterministic engine must fix it by a rule and say what the rule cannot find. And the generality the proposal's "inverse substitution" asks for has a separate, older operator with its own definition, Plotkin's least general generalisation (1970), which composes with the others instead of being folded into them.

## Decision Drivers

- ADR-0025: every walk over the arena uses a caller slice as its stack, every bound is a result, a failure leaves the binding table as it was.
- ADR-0040's precedent: a syntactic object as a term over a reserved functor above `CATEGORY_RESERVED`, matched by unification, with a greedy discipline stated as a rule and its gap as a hypothesis (H-10).
- Rule L-3 and [ADR-0031](0031-policy-amendment.md): a clause holds ids; an invented predicate is an id in a term, not a rule of the engine.
- "Latest ≠ Newest": inverse resolution (1988) and the least general generalisation (1970) have three decades of documented failure modes (the operators are non-deterministic; a generalisation of clauses grows as the product of their lengths; invented predicates multiply without a compression criterion), where the meta-interpretive and answer-set learners of the last decade need a solver a `no_std` crate cannot hold.
- [ADR-0030](0030-verification-governance.md): a new rule carries an exit test, a property walk and a pinned number from an independent oracle.

## Considered Options

1. A new record: a clause node with its own layout (head, literals, a signature).
2. **A clause as a compound over `CLAUSE`; four operators and one resolution step as functions over `InduceScratch`, matching first fit by unification; the invented predicate from a reserved band.**
3. The operators with Plotkin's generalisation folded in (every output generalised), the ILP literature's "relative least general generalisation".
4. The operators over the propositional clauses the crate already had.

## Decision Outcome

Option 2.

- **A clause** is `CLAUSE(head, l_1, ..., l_k)`, `CLAUSE` = `0xFFFF_FF03` (above `CATEGORY_RESERVED`, distinct from the slashes; asserted at compile time), $k \le$ `MAX_BODY` = 7; `clause`, `is_clause`, `clause_head`, `clause_body_len`, `clause_literal`. A literal is a positive term; the clauses are definite. An invented predicate takes the next id of `INVENTED_BASE` `0xFFFE_0000` to `INVENTED_LIMIT` `0xFFFF_0000` (65 536 inventions; below `CATEGORY_RESERVED`, and the runtime asserts its role band above the limit).
- **`InduceScratch`** mirrors `ParseScratch`: the arena and `free`, the bindings, the trail and `trail_len`, the work stack, a pair table for the generalisation, `next_variable`, `next_invented`. Every rule marks these on entry and, on any error, undoes the bindings, zeroes the nodes it appended and resets the cursor and the counters; on success the bindings it made stay, the trail saying which, and an output is an instance of its inputs through them.
- **The measures.** `size` is a term's node count through the bindings (a shared subterm counts each time it is reached; saturating); `term_hash` is FNV-1a 32 over the pre-order walk (kind, functor or variable number, arity), the same for the same structure wherever it sits; `free_variables` appends the unbound variables of a term, one node per number, to a caller slice.
- **`lgg(a, b)`** (Plotkin): equal constants and the same node stay; compounds of one functor and arity generalise child by child into a new compound; any other pair is a variable, the same for the same pair of dereferenced nodes, numbered from `next_variable`. A variable is identified by its node, so a caller that shares one node per variable, as the reducer's lexicon does, gets the least general result.
- **The operators**, literals matched **first fit, in order, by unification, without backtracking**. `absorb(c2, c)`: from $q \leftarrow A$ and $p \leftarrow A', B$ with every literal of $A$ paired to a distinct literal of $c$, the clause $p \leftarrow q\theta, B$ ($q\theta$ first); `NoMatch` for a fact or an unpaired literal. `identify(c1, c)`: heads unified, $B$ paired, exactly one literal of $c_1$ left, the clause $q \leftarrow A$ from that literal and the literals of $c$ not taken; `NoMatch` otherwise. `intra_construct(ca, cb)`: heads unified, the shared literals $A$ paired, the clauses $p \leftarrow A, q(V)$, $q(V) \leftarrow B_1$ and $q(V) \leftarrow B_2$ over one shared literal node $q(V)$, $V$ the unbound variables of $p \leftarrow A$ that occur in $B_1$ or $B_2$, in order of first occurrence, at most `MAX_ARITY` (`TooManyArguments`); `NothingToInvent` when nothing is shared or a clause has nothing of its own. `resolve_definite(goal, rule)`: the first literal of the goal that unifies with the rule's head is replaced by the rule's body, in place (`BodyFull` past seven).
- **The identity.** Resolving `absorb`'s output with $c_2$ gives $c$; resolving $c_1$ with `identify`'s output gives $c$; resolving `intra_construct`'s first output with either definition gives the corresponding input; each up to the order of literals, through the bindings. The tests hold it on named examples and on 3 000 random pairs.
- **What first fit cannot find.** A pairing that exists only when an earlier literal takes a later partner (the greedy gap of H-10, restated as part of H-11); an output more general than the inputs (the generalisation is `lgg`, a separate rule the caller composes); a matching that needs the occurs check to fail differently. None is a defect of the rule as stated.

### Consequences

- Good: predicate invention exists as a tested rule with its defining identity checked, in the crate that owns terms, with no record, no allocation and no recursion.
- Good: the runtime's discovery path ([ADR-0043](0043-discovery-path.md)) has an operator whose compression it can measure and a hash a frame can carry.
- Bad: `INVENTED_BASE` and `CLAUSE` are two more reserved bands a caller's concept ids must avoid; `CATEGORY_RESERVED` remains the documented ceiling.
- Bad: the operators produce instances, not generalisations; a store that only absorbs and identifies never becomes more general than its examples.
- Bad: intra-construction with a `SCOPE` of 64 variables in the head and shared literals refuses beyond that; no clause of eight literals over eight-child terms reaches it, but a deeper term could.

## Alternatives considered and why rejected

- **Option 1** would add a record for what eight child slots already hold, against ADR-0016's rule that a quantity an existing record can carry is a field, not a record.
- **Option 3** folds two operators with different definitions into one and makes the identity untestable, since a generalised output no longer resolves back to its inputs.
- **Option 4** cannot invent a predicate with arguments and has no variables to link.
- **Backtracking over pairings** would make the operators a search with a bound the caller cannot see from the outside; the reducer of ADR-0040 made the same choice and named the gap.

## Confirmation

`cortex-reasoning`: a clause holds a head and up to seven literals and refuses eight, a category is not a clause; `size`, `term_hash` and `free_variables` read through the bindings, the hash ignores where a term sits, every bound and malformed case is a result; the generalisation's textbook cases (`f(a, b, a)` with `f(c, b, c)` is `f(X, b, X)`, different pairs are different variables, nested compounds, different functors, a variable against a constant, a term with itself) and its bounds (`PairsFull`, `BoundExceeded` for the stack and for the variable numbers, `ArenaFull` with the arena zeroed above the mark, `Malformed`); each operator on a named example with the identity checked by `resolve_definite`; a variable only a differing literal uses is not an argument; every error variant reached; the exit pair's three hashes pinned; a second invention takes the next id; the resolvent's literal order; a property walk over 3 000 random clause pairs from a three-predicate vocabulary holding the identities for every success (more than a hundred inventions and absorptions, more than thirty identifications) and the restoration of the arena, the bindings and the counters for every failure. `npx spec-guard` asserts `CLAUSE`, `fn intra_construct` and `fn lgg`. The mutation gate on the changed lines ([ADR-0030](0030-verification-governance.md)) passes in CI.
