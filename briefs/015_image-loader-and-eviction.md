---
status: proposed
date: 2026-09-10
---

# Brief 015 — The `.cortex` image: section directory, CRC-64, loader and writer, the clock sweep, and the Tier-2 delta record (milestone M4; finding F-20)

## Mission

An image round-trips. `cortex-connectome` gains the section directory record and a CRC-64 the
state crate computes without a table larger than a cache line; `CortexFileHeader` validates
itself. The runtime crate (brief 012) writes an image from its arenas and opens one, either by
`mmap` under an ADR that names the invariant and the test (TC-9) or by a plain read into the
arenas with `mmap` left Specified; the clock sweep of axiom A5 evicts a quiet unit's 64 bytes and
re-hydrates them on the next spike. Finding **F-20** is resolved by the Tier-2 plastic-delta
record and a 32-bit head in the unit. Milestone M4's exit test passes: evict, spike, re-hydrate
preserves state bit for bit. The format version moves once more (6 at the time of writing, ADR-0022).

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; state crates dependency-free and `no_std`; a runtime
  dependency only through [ADR-0005](../docs/adr/0005-crate-per-subsystem.md)'s allow-list;
  **`unsafe` only under an ADR** naming the invariant and the test (whitepaper TC-9, §8.10).
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` (`20c6243`) on 2026-09-10.

- `crates/cortex-connectome/src/lib.rs`: `CortexFileHeader { magic, version, reserved_flags,
  num_columns, num_neurons, num_synapses, layers_offset, crc64, _padding }`, `MAGIC`,
  `FORMAT_VERSION = 6` (4 at the commit above; ADR-0020 and ADR-0021 carved fields from six
  records, ADR-0022 re-encoded the synapse indices); no validation, no CRC, no section record. Whitepaper
  [§8.7](../docs/WHITEPAPER.md#87-persistence-and-serialisation): a section directory of
  `(kind: u32, offset: u64, length: u64, crc64: u64)` entries padded to 64 B, then sections
  whose bytes are the arenas; an image with a foreign version MUST fail closed; atomics are
  stored as their plain values and MUST be zero at rest (idle, empty mailbox: ADR-0017's
  encoding makes zero empty).
- Whitepaper [§6.7](../docs/WHITEPAPER.md#67-scenario-r-7-cold-boot-from-a-cortex-image-specified)
  (R-7): validate `magic`, `version`, `crc64`; `mmap` with `MAP_POPULATE`; huge pages; NUMA
  pinning; hand section offsets to the arenas; no per-record deserialisation. Target T-7.
- Whitepaper [§8.6](../docs/WHITEPAPER.md#86-memory-management): the metabolic sweep walks
  unit records with a clock hand; a unit that is idle, unscheduled and quiet beyond a threshold
  has its 64 bytes written to the image or the write-ahead log and its slot returned to the free
  list; a later spike to its id re-hydrates it. The gate and the mailbox exist
  ([ADR-0017](../docs/adr/0017-mailbox-and-gate-protocol.md)), so "idle and unscheduled" is
  readable; "quiet" is `ticks_since_spike` against a threshold.
- Whitepaper §11 finding **F-20**: `DendriticSuperNeuron::plastic_delta_head` is a `u16` into a
  table Appendix A sizes at 10⁹ entries; sixteen bits address 65 536. The unit has four
  reserved bytes at `[60..64)`; a 32-bit head fits by taking two of them or by widening in
  place with a layout change under rule L-6. Appendix A row 37: "Plastic deltas ΔW (Tier 2,
  Specified; no record type yet)", 16 B each.
- A CRC-64 (ECMA-182 or ISO 3309 polynomial) can be computed bitwise without a table or with a
  256-entry table built by a `const fn` at compile time (2 KB); either is `no_std`; the ADR
  picks one and records test vectors from a published source.
- `mmap` needs `unsafe` and a dependency on the allow-list (`rustix` or `nix`); TC-9 requires an
  ADR naming the invariant (the mapping outlives every borrow; the file is not truncated while
  mapped) and the test. A plain `std::fs::read` into the arenas needs neither and is slower by
  the copy, which T-7 bounds; the ADR decides which one this round builds.

<!-- @assert-absence target="crates/cortex-connectome" symbol="SectionEntry" word="true" reason="precondition: no section directory record exists; archive this brief when it does" -->

## Deliverables

- [ ] A new ADR at the next free number (`ls docs/adr`), `status: proposed` in the PR: the
      section directory layout and the section kinds (one per arena of §5.2, numbered), the CRC
      polynomial and its vectors, header validation and the fail-closed rule, the loader's
      strategy (`mmap` with its invariant and test, or read-into-arenas with `mmap` Specified),
      the writer, the sweep's quiet threshold and its clock hand, the write-ahead log or its
      absence, the Tier-2 delta record (16 B: synapse block and slot, delta in Q1.15, epoch)
      and the 32-bit head in the unit resolving F-20, and the format bump. Committed
      `accepted` per `docs/adr/README.md`.
- [ ] `cortex-connectome`: `SectionEntry` (64 B, asserted), `SectionKind` constants,
      `crc64(bytes) -> u64` (`no_std`, table-free or `const` table), `CortexFileHeader::{validate,
      new}`; tests with published CRC vectors, a header round trip, and refusal of a foreign
      version and a bad CRC.
- [ ] `cortex-core`: `PlasticDelta` (16 B, asserted, per the ADR) and `plastic_delta_head: u32`
      (F-20 resolved), the format bump, §5.2.1 table updated.
- [ ] `runtime/cortex-runtime`: `Image::write(&arenas, path)` and `Image::open(path)` per the
      ADR; `ClockSweep::step()` evicting one quiet unit per call and `rehydrate(id)`; tests: an
      image written from arenas and re-opened is byte-identical; evict, spike, re-hydrate
      preserves a unit bit for bit (M4 exit); a truncated or corrupted image fails closed.
- [ ] Whitepaper §5.2.1, §5.2.2 (directory record, the next version), §6.7 (which steps are
      Implemented), §8.6, §8.7, §11 F-20 resolved, Appendix A row 37 with its record, Appendix
      C M4; `docs/benchmarks/README.md` if a T-7 measurement subject now exists; `CHANGELOG.md`;
      archive this brief.

## Not empowered

- Not to write `unsafe` without the ADR's invariant and test; not to add a dependency outside
  the allow-list.
- Not to implement huge pages, NUMA pinning or `MADV_HUGEPAGE`; they are Specified with the
  syscall named.
- Not to implement structural plasticity or reclamation (ADR-0011); the delta record is stored
  and read, not yet applied.

## Architectural empowerment

You may decide the write-ahead log is not needed this round if the ADR shows eviction writes
straight into the image safely; you may choose a 32-entry directory cap; you may put the delta
record's arena in the runtime rather than in the image if the ADR shows Tier 2 is never at
rest. The empowerment reaches this brief's instructions, not the whitepaper's invariants or
`CLAUDE.md`.

## Verification

```bash
cargo test --workspace
cargo test --workspace --release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo +1.85 check --workspace --all-targets
npm run spec                      # the precondition directive above must be gone (archived)
```

## Report

State: the directory and CRC as decided; the loader's strategy and, if `mmap`, the invariant
and the test; the sweep's threshold; the delta record and the state of F-20; the M4 exit test's
result; what stays Specified toward T-7; and anything in this brief that turned out to be wrong
when re-derived.
