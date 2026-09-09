---
status: archived
date: 2026-09-10
---

> **Executed 2026-09-10 in pull request #11.** Closes finding F-14. No ADR was written: a
> coverage tool was considered and not proposed; the per-item rule is a review rule with the
> per-crate test module held by `spec-guard`. The report is in the pull request and in
> `CHANGELOG.md`. The body below describes the tree before execution and is not maintained, apart
> from relative links, which gained one `../` so that they still resolve from `archive/`.

# Brief 007 — A test for every public function, and a rule that keeps it that way

## Mission

Every public function and associated constant in `crates/` is exercised by at least one unit
test, the whitepaper's §1.6 "Test" column reads "yes" for all eighteen crates, `CONTRIBUTING.md`
states the rule with its enforcement point, and finding **F-14** is Resolved.

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; the built-in test harness; no test framework crate.
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` on 2026-09-10.

- Thirty-four tests exist: four layout tests (`cortex-agency`, `cortex-executive`,
  `cortex-immune`, `cortex-predictive`), twenty-two boundary tests from brief 001 across five
  crates, seven for `synaptic_efficacy_q16` and one for `CortexFileHeader`'s constants.
- Whitepaper §11 F-14 is narrowed to: `FlatTimingWheel::schedule_fine` and
  `SymbolicHypervectorHeader::bind` remain untested. Re-deriving finds three more public items
  with no test: `EmbodimentRingBuffer::new` / `Default` (all cursors zero),
  `FabricPacketHeader::MAGIC`, and `SymbolicHypervectorHeader::DIMENSIONS`.
- `crates/cortex-symbolic/src/lib.rs`: `bind(&mut self, role_id, filler_id)` stores the two ids
  and sets bit 0 of `flags` (`flags |= 0x01`); other flag bits must survive. No `#[cfg(test)]`
  module exists in the crate.
- `crates/cortex-core/src/dispatch/wheel.rs`: `schedule_fine(delay_ticks, event_mask)` ORs the
  mask into `fine_ring[(cursor + delay_ticks) % 200]`. **Brief 005 changes this API.** If 005 has
  landed, test the API it left and skip this line; if not, test `schedule_fine` as it stands
  (slot arithmetic at the wrap, OR-accumulation, delay 0) and say so.
- Whitepaper §1.6 "Test" column reads `no` for seven crates: `cortex-sensory`,
  `cortex-embodiment`, `cortex-symbolic`, `cortex-neuromod`, `cortex-hippocampus`, `cortex-fabric`,
  `cortex-telemetry` (`cortex-core` and `cortex-connectome` became `yes` in brief 003). Crates
  whose only public item is a record with no function get a layout test in the style of the four
  `no_std` originals, so that the column is honest rather than blank.
- `CONTRIBUTING.md` has no rule about tests per public function; `CLAUDE.md` principle 5 says a
  structural boundary beats a reviewed one.

<!-- @assert-absence target="crates/cortex-symbolic" symbol="#[cfg(test)]" reason="precondition: F-14 is open and cortex-symbolic has no tests; archive this brief when it does" -->

## Deliverables

- [x] `cortex-symbolic`: tests for `bind` (ids stored; bit 0 set; bits 1–15 preserved; calling
      twice is idempotent) and `DIMENSIONS == 10_000`.
- [x] `cortex-core`: tests for the wheel API as it stands after brief 005 (or `schedule_fine` if
      005 has not landed).
      Brief 005 had landed and left eight wheel tests; nothing further was needed.
- [x] `cortex-embodiment`: `new()` and `Default` give all-zero cursors and reserved bytes;
      `cortex-fabric`: `MAGIC == *b"VCFB"`.
- [x] Layout tests (`size_of`, `align_of`) for the seven crates that have none, in the style of
      `cortex-agency`, so every crate has a `#[cfg(test)]` module.
      `cortex-sensory` also gained a stub driver test through `dyn SensoryPeripheral`.
- [x] `CONTRIBUTING.md` code rules: "Every public function and associated constant has at least
      one unit test", with the enforcement point named. If a tool can check it, name the tool; if
      only review can, write it as a review rule and say so (CLAUDE.md principle 5).
      The per-crate module is held by eighteen `spec-guard` directives; the per-item rule is a
      review rule, stated as such.
- [x] Whitepaper §1.6 "Test" column all `yes`; §11 F-14 Resolved.
- [x] `CHANGELOG.md` entry under Unreleased.
- [x] Archive this brief.

## Not empowered

- Not to change any function's behaviour; a test that needs a behaviour change is a finding to
  record in §11, not a fix to make here.
- Not to add a test framework, a mocking crate, or a coverage tool as a dependency.
- Not to add a coverage threshold to CI in this round; see the empowerment.

## Architectural empowerment

If you conclude that the rule needs a structural check (a coverage floor with `cargo-llvm-cov`,
or a script that lists public items without a test), you may propose it in a new ADR at the next
free number (`ls docs/adr`), `status: proposed`, with the §2.1 admissibility argument for the tool
and the cost in CI time, and stop short of installing it. The empowerment reaches this brief's
instructions, not the whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo test --workspace
cargo test --workspace --release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
npm run spec                      # the precondition directive above must be gone (archived)
```

Run after brief 005 where possible; otherwise say in the report that the wheel tests target
`schedule_fine`.

## Report

State: the list of tests added per crate; the total test count before and after; the rule as
written in `CONTRIBUTING.md` and its enforcement point; the state of F-14; whether a coverage ADR
was proposed; and anything in this brief that turned out to be wrong when re-derived.
