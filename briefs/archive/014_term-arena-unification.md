---
status: archived
date: 2026-09-10
---

> **Executed 2026-09-10 in pull request #30.** Writes
> [ADR-0025](../../docs/adr/0025-term-arena-and-unification.md); `TermNode` is the second
> record of `cortex-reasoning`, first-order unification and the first-order resolution step are
> Implemented, Socrates is mortal in two steps, and whitepaper §11.1's open question is resolved.
> The report is in the pull request and in `CHANGELOG.md`. The body below describes the tree
> before execution and is not maintained, apart from relative links, which gained one `../` so
> that they still resolve from `archive/`.

# Brief 014 — A term arena and first-order unification for `cortex-reasoning` (R-10, track 1)

## Mission

`cortex-reasoning` gains its second record, a 64-byte term node, and first-order unification
over an arena of them: functor and arity, child indices, variables, a bounded binding trail
with an occurs check, no allocation and no recursion deeper than a stated bound; resolution then
applies to first-order clauses whose literals are terms, not only to propositional atoms. An
ADR admits the second record under the test of
[ADR-0016](../../docs/adr/0016-thirty-two-crate-architecture.md) (a record no crate has, in the
crate that owns the quantity) and fixes the layout, the binding discipline and the bounds. The
open question in whitepaper §11.1 is resolved; R-10's native track can prove statements with
variables.

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; no dependencies
  ([ADR-0005](../../docs/adr/0005-crate-per-subsystem.md)); no `unsafe`; no `Box`, `Vec` or
  recursion without a bound; every record `#[repr(C, align(64))]`, 64 bytes, asserted.
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` (`20c6243`) on 2026-09-10.

- `crates/cortex-reasoning/src/lib.rs`: literals are `u32` atoms with bit 31 as the sign and
  atom 0 as "no literal"; `Clause = (u32, u32)`; `resolve`, `apply_resolution`, `is_refutation`
  on `SymbolicRuleNode` (whitepaper §5.2.30). Unification is Specified: "needs a term arena"
  (§6.10 step 3; §11.1 open question).
- Whitepaper [§5.2.9](../../docs/WHITEPAPER.md#529-cortex-symbolic--vector-symbolic-bridge):
  `cortex-symbolic` grounds concepts as hypervector ids; a term's functor and constants are
  concept ids from there, so a term node holds indices, never strings (rule L-3).
- ADR-0016's admission test applies to crates; a second record in a crate needs "an ADR under
  the test": a named gap (§11.1 has it), no other owner (§5.2.30 owns deduction), a mechanism
  in §8.8 with the layout, widths that fit, no boundary crossed.
- Robinson's unification: two terms unify if they are the same variable, or one is an unbound
  variable that does not occur in the other (occurs check), or they have the same functor and
  arity and their children unify pairwise under the bindings so far. Without allocation this
  needs a fixed work stack and a binding trail whose capacities are records of the caller's;
  the bound is a parameter and an exceeded bound is a refusal, not a panic.
- `crates/cortex-arithmetic` shows a record whose function reports failure in flags rather than
  saturating; unification's "does not unify" and "bound exceeded" are two different results and
  both must be visible.


## Deliverables

- [x] A new ADR at the next free number (`ls docs/adr`), `status: proposed` in the PR: the
      second record under ADR-0016's test; `TermNode` layout (kind: constant, variable or
      compound; functor concept id; arity up to a stated maximum; child indices; a binding
      index for a variable; reserved bytes); the binding discipline (a trail of (variable,
      previous binding) pairs in a caller-provided slice, undone on failure); the occurs check;
      the work-stack bound and the result type (`Unified`, `Clash`, `OccursCheck`, `BoundExceeded`);
      how a first-order literal names a term node and keeps the sign bit. Committed `accepted`
      per `docs/adr/README.md`.
- [x] `TermNode` (64 B, asserted, the five derives and `Default`), `Binding`, and
      `unify(a, b, arena, bindings, trail, stack) -> UnifyResult` as free functions over caller
      slices; `deref(index, bindings)` following bound variables; `apply_resolution` extended
      or a `resolve_first_order` that unifies the complementary pair and applies the bindings
      to the resolvent.
- [x] Tests: constants unify with themselves only; a variable binds and is dereferenced; the
      occurs check refuses $X = f(X)$; nested compounds unify with consistent bindings and refuse
      inconsistent ones; the trail is undone on failure; an exceeded bound is reported, not
      overflowed; a first-order resolution step (Socrates: `mortal(X) :- human(X)`, `human(s)`,
      negated `mortal(s)`) reaches the empty clause; determinism.
- [x] Whitepaper §5.2.30 (second record's layout and API), §6.10 step 3 (unification
      Implemented), §8.8 row, §11.1 question resolved, Appendix A row for the term arena with a
      stated count; `CHANGELOG.md`; archive this brief.

## Not empowered

- Not to add a crate (ADR-0016) or a dependency.
- Not to implement clause search, indexing or constraint propagation; one resolution step with
  unification is the round.
- Not to change `SymbolicRuleNode`'s layout except by the format bump of rule L-6 with the
  ADR's argument.

## Architectural empowerment

You may choose a different representation for variables (de Bruijn indices, or an arena slot
per variable) if the ADR shows the occurs check and the trail stay bounded and allocation-free;
you may cap arity lower than the layout allows. The empowerment reaches this brief's
instructions, not the whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo +1.85 check --workspace --all-targets
npm run spec                      # the precondition directive above must be gone (archived)
```

## Report

State: the term layout and the binding discipline in three sentences; the bounds and what
exceeding them does; the first-order refutation the tests prove; the state of R-10's native
track; and anything in this brief that turned out to be wrong when re-derived.
