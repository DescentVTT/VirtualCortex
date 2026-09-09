---
status: archived
date: 2026-09-10
---

> **Executed 2026-09-10 in pull request #5.** Closes findings F-6 and F-7; TC-6 becomes
> Implemented and, with L-5, is now a MUST held by an executable assertion. No ADR was written: the
> `Pod` marker (`bytemuck` versus `zerocopy`) is deferred until the image loader needs it. The
> report is in the pull request and in `CHANGELOG.md`. The body below describes the tree before
> execution and is not maintained, apart from relative links, which gained one `../` so that they
> still resolve from `archive/`.

# Brief 002 — `#![no_std]` for every state crate and derives on every plain record

## Mission

All eighteen crates are `#![no_std]`; every record that contains no atomics derives
`Clone, Copy, Debug, PartialEq, Eq`; the two control records (`DendriticSuperNeuron`,
`EmbodimentRingBuffer`) derive `Debug` only and say why; whitepaper constraint **TC-6** is
Implemented and findings **F-6** and **F-7** are Resolved.

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; no new dependencies
  ([ADR-0005](../../docs/adr/0005-crate-per-subsystem.md)). In particular, do not add `bytemuck` in
  this round; the `Pod` derive is a future decision.
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` on 2026-09-10.

- Four crates are `#![no_std]` and derive the five traits: `cortex-agency`, `cortex-executive`,
  `cortex-immune`, `cortex-predictive`. The other fourteen are neither, and none of them uses a
  `std` item: the only imports are `core::sync::atomic::*` in `cortex-core` and
  `cortex-embodiment`.
- Whitepaper [§8.2](../../docs/WHITEPAPER.md#82-record-layout-and-abi) rule L-5: a record with
  atomics is a control record, `Sync` but not `Copy`; a record without atomics SHOULD derive the
  five traits.
- `SensoryEvent` already derives `Copy, Clone, Debug, Default`; `LfpSamplePacket` derives
  `Copy, Clone, Debug`. Bring both to the full set (`Default` on `SensoryEvent` stays).
- `FlatTimingWheel` is not a record (2 248 bytes, not `repr(C)`); it holds `[u64; 200]` and
  `[u64; 80]`, so `Copy` is legal but unwise for a 2 KB value. Derive `Debug` only. It already has
  `Default`.
- The `#[cfg(test)]` modules in the four `no_std` crates use `assert_eq!`, which is available in
  `core`; tests still link `std` through the test harness, so `#![no_std]` on a library crate does
  not prevent `cargo test`.

<!-- @assert-count target="crates" symbol="#![no_std]" glob="*.rs" max="4" reason="precondition: F-6 is open (four crates); archive this brief when all eighteen carry it" -->

## Deliverables

- [x] `#![no_std]` at the top of every `src/lib.rs` that lacks it (fourteen files).
- [x] Derives per L-5 on: `SynapseBlock`, `CortexFileHeader`, `BasalGangliaChannelState`,
      `CerebellarMicrozone`, `SalienceNodeState`, `GlobalWorkspaceSlot`, `SymbolicHypervectorHeader`,
      `NeuromodulatorState`, `HippocampalAttractorState`, `HomeostaticDrivePool`,
      `FabricPacketHeader`, `LfpSamplePacket`, `SensoryEvent`.
- [x] `#[derive(Debug)]` on `DendriticSuperNeuron`, `EmbodimentRingBuffer` and `FlatTimingWheel`,
      with a one-line comment on the two control records citing L-5.
- [x] Whitepaper: TC-6 to Implemented; F-6 and F-7 Resolved; §1.6 table `no_std` column all "yes";
      §5.2 public-API rows that mention derives updated.
      TC-6 and L-5 were also raised from SHOULD to MUST, with an executable assertion on the count.
- [x] `CHANGELOG.md` entry under Unreleased.
- [x] Archive this brief.

## Not empowered

- Not to add `bytemuck`, `zerocopy` or any dependency. `Pod`/`Zeroable` is a separate decision
  (see empowerment).
- Not to change any field, width, order or padding; `cargo check` must pass with every existing
  `const _` assertion untouched.
- Not to add `#![no_main]`, a panic handler, or any attribute beyond `#![no_std]`.
- Not to remove the existing `Default` on `SensoryEvent`, `FlatTimingWheel` or
  `EmbodimentRingBuffer`.

## Architectural empowerment

If you conclude that the plain records should carry a `Pod` marker now rather than later, write a
new ADR at the next free number (`ls docs/adr`), `status: proposed`, comparing `bytemuck` (older,
minimal, `derive` feature) and `zerocopy`
(stricter derive validation, larger), apply the admissibility test of whitepaper §2.1 to both, and
stop there; do not add the dependency in this round. The empowerment reaches this brief's
instructions, not the whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
npm run spec
```

`cargo test` must still run the four existing layout tests; `npm run spec:guard` must show the
precondition directive above as gone (archived) and the whitepaper's `const _` count directive
unchanged at 18.

## Report

State: the list of files that gained `#![no_std]`; the list of records that gained derives and the
two that did not and why; the state of TC-6, F-6 and F-7; whether a `Pod` ADR was proposed; and
anything in this brief that turned out to be wrong when re-derived.
