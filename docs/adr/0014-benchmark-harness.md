---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0010
---

# ADR-0014: Benchmark harness — criterion 0.7, confined to a bench-only crate

## Context and Problem Statement

[ADR-0010](0010-measured-or-target.md) allows a performance figure into the whitepaper only when a benchmark in this repository produced it on the reference platform from a committed command. No benchmark existed (whitepaper finding F-13), so every figure was a Target. A harness is a dependency, and [ADR-0005](0005-crate-per-subsystem.md) forbids dependencies in state crates and allows them only on a vetted list for a runtime crate. Which harness, and where does it live?

## Decision Drivers

- The admissibility test of whitepaper §2.1: a stable published API, two years of third-party production use, documented failure modes, and a mechanical-sympathy argument.
- The state crates' `[dependencies]` tables must stay empty and provably so (`cargo tree -e normal`).
- [ADR-0009](0009-rust-edition-and-msrv.md) proposes an MSRV of 1.85; a dev-dependency that needs a newer compiler would make the workspace unbuildable on that MSRV.
- Windows developer machines must be able to run the benchmarks; only admissible runs need Linux.
- CI must keep the benchmarks compiling and executing without asserting timings on shared runners.

## Considered Options

Facts from crates.io on 2026-09-10:

| Harness | First release | Latest | MSRV | Downloads | Notes |
| :--- | :--- | :--- | :--- | ---: | :--- |
| `criterion` 0.7.0 | 2017-12 | 0.8.2 (2026-02, MSRV 1.86) | 1.80 | 277 M total | Statistical (bootstrap, outlier classification), stable API since 0.3, `cargo bench` integration, optional plots. |
| `divan` 0.1.21 | 2023-06 | 0.1.21 (2025-04) | 1.80 | 7.5 M total | Attribute-driven, fast, pleasant; still 0.1.x with no published stability policy. |
| `iai-callgrind` | 2022 | — | — | — | Instruction counts under Valgrind; Linux only; a complement for cycle-exact figures, not a harness for a Windows developer loop. |

1. `criterion` at the newest release, 0.8.2.
2. **`criterion` 0.7.0, pinned exactly, default features off, in a dedicated `benches/cortex-bench` crate.**
3. `divan`.
4. Hand-rolled timing with `std::time::Instant`.

## Decision Outcome

Option 2.

- **`criterion` passes the §2.1 test**; `divan` fails its first clause (a 0.1.x API with no stability policy is not a stable published interface) and option 4 fails the fourth (no statistical treatment of noise, which is the documented failure mode of every micro-benchmark).
- **0.7.0 rather than 0.8.2** because 0.8 raised its MSRV to 1.86, above the 1.85 ADR-0009 proposes, and because the newest line has been out seven months against fourteen for 0.7; the API used here is identical in both. The pin is exact (`=0.7.0`) so that a `cargo update` cannot move it.
- **Default features off** (`plotters`, `rayon`) and `cargo_bench_support` on: the statistical core and `cargo bench` argument parsing are what is needed; plots are not, and every avoided crate is one fewer to audit. The dependency tree adds 43 crates to `Cargo.lock`, all dev-only, all reachable only from `cortex-bench`.
- **A separate crate**, `benches/cortex-bench`, `publish = false`, listed in the workspace after the eighteen state crates. It depends normally on the state crates it measures and carries the harness as a dev-dependency; the state crates' dependency tables stay empty, which `cargo tree -e normal -p <crate>` confirms and CI's `--locked` build enforces.
- **Inputs are deterministic**: a shared 32-bit LCG with a fixed seed (`cortex_bench::Lcg`) drives every benchmark, so runs are comparable.
- **CI runs the benchmarks in `--test` mode** (one iteration each) so that they cannot rot, and asserts no timing: a shared runner cannot hold a latency threshold and a flaky gate gets switched off.
- **Admissibility is a property of the run, not of the harness.** `docs/benchmarks/README.md` states the protocol (reference platform, isolated core, fixed frequency, `perf stat`, sample counts) and the results convention; each results file begins with an `admissible: yes/no` line, and only an admissible file may be cited in whitepaper §10.2's Measured column.

### Consequences

- Good: the parts of T-3 that exist (wheel schedule and advance, synaptic efficacy, gating, ignition) are measured from a committed command; F-13 narrows from "no benchmark" to "no admissible measurement".
- Good: the choice can be revisited by editing one manifest; nothing in the state crates knows the harness exists.
- Bad: `Cargo.lock` grows by 43 packages and CI compiles them once per cache miss.
- Bad: criterion's timings on a Windows developer machine are not admissible and never will be; the protocol requires Linux with core isolation, so an admissible number waits for the reference platform.
- `iai-callgrind` remains a candidate complement for instruction-count figures once a Linux runner exists; adopting it would amend this record.

## Confirmation

`cargo bench -p cortex-bench --bench hot_path -- --test` runs in CI. `npx spec-guard` asserts `benches/cortex-bench/Cargo.toml` and `docs/benchmarks/README.md` are present (whitepaper §10.2). `cargo tree -e normal -p cortex-core` prints no dependencies.
