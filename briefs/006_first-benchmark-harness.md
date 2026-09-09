---
status: proposed
date: 2026-09-10
---

# Brief 006 — The first benchmark: a harness, a protocol, and a non-admissible number

## Mission

The repository contains a benchmark crate that measures the components of target **T-3** that
exist today (timing-wheel insert, synaptic efficacy, action gating), a written protocol for
producing an *admissible* measurement on the reference platform, a place where results are
recorded with their provenance, and a first run on a developer machine that is recorded as
**not admissible** and therefore does not fill the Measured column; whitepaper finding **F-13** is
narrowed from "no benchmark exists" to "no measurement on the reference platform exists".

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis. A number is Measured only
  under [ADR-0010](../docs/adr/0010-measured-or-target.md): a benchmark in this tree, run on the
  reference platform of whitepaper §7.1, reproducible from a committed command, with the commit
  hash recorded. **Anything else stays a Target.**
- **Latest ≠ Newest.** A benchmark harness is a dependency; apply the admissibility test of
  whitepaper §2.1 to it and record the choice.
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` on 2026-09-10.

- No `benches/` directory, no `[dev-dependencies]` anywhere, no `criterion` or other harness;
  `Cargo.lock` lists the eighteen workspace crates and nothing else; CI builds with `--locked`.
- [ADR-0005](../docs/adr/0005-crate-per-subsystem.md): state crates MUST declare no
  dependencies; dependencies are permitted only from a runtime crate downward, on a vetted
  allow-list, each addition recorded in the changelog. A benchmark crate is neither a state crate
  nor the runtime crate, so it needs its own decision.
- Whitepaper [§10.2](../docs/WHITEPAPER.md#102-scenarios-and-targets), T-3: "one spike enqueued to
  a hot unit on an isolated core; enqueue latency (R-1 steps 1–3); median < 20 ns, p99.99 < 50 ns;
  `criterion` micro-benchmark plus `perf stat`; 10⁸ samples; isolated core, fixed frequency". Of
  R-1's steps 1–3 only step 1 (the wheel insert) exists; steps 2–3 (mailbox push, gate) are
  Specified. T-8 (throughput) has no runnable subject yet.
- Functions that exist and are on or near the hot path: `FlatTimingWheel::schedule_fine` (or the
  API brief 005 leaves), `cortex_core::synaptic_efficacy_q16`
  ([ADR-0012](../docs/adr/0012-synaptic-weight-q1-15.md)), `BasalGangliaChannelState::compute_gating`,
  `GlobalWorkspaceSlot::step_ignition`.
- ADR-0010's confirmation section says: "A future `spec-guard` directive will assert the presence
  of `benches/` once the first benchmark lands."
- Appendix C, milestone M7: "Benchmarks for T-3, T-8; differential test for T-1. Targets become
  Measured or are revised."

<!-- @assert-absence target="Cargo.toml" symbol="cortex-bench" reason="precondition: F-13 is open and no benchmark crate exists; archive this brief when it lands" -->

## Deliverables

- [ ] A new ADR at the next free number (`ls docs/adr`), `status: proposed` in the PR, choosing
      the harness. Apply the §2.1 test to at least `criterion` (2018, the de-facto standard,
      statistical reporting) and one newer alternative (for example `divan`, 2023); expect the
      older one to win unless the analysis says otherwise, and say why. Record the pinned exact
      version, that it is a dev-dependency of a non-state crate only, and that `Cargo.lock` grows
      accordingly.
- [ ] `benches/cortex-bench/` as a workspace member with `publish = false`, depending on the
      state crates it measures and on the harness as a dev-dependency; benchmarks for the wheel
      insert, `synaptic_efficacy_q16`, `compute_gating` and `step_ignition`, each with a fixed
      seed and a fixed input distribution documented in the source.
- [ ] `docs/benchmarks/README.md`: the protocol for an admissible run (reference platform per
      whitepaper §7.1, `isolcpus`/`nohz_full` core, fixed frequency, the exact commands, `perf stat`
      counters to capture, warm-up, sample counts) and the results convention
      `docs/benchmarks/results/<YYYY-MM-DD>-<host>.md` with commit hash, toolchain, CPU model,
      kernel parameters, and an **admissible: yes/no** line at the top.
- [ ] One results file from the machine that executes this brief, marked `admissible: no` with
      the reason (not the reference platform), so that the convention is exercised and the number
      exists without being promoted.
- [ ] CI: a step that compiles the benchmarks (`cargo bench -p cortex-bench --no-run` or the
      harness's `--test` mode) so they cannot rot; no timing is asserted in CI.
- [ ] ADR-0010 confirmation satisfied: a `spec-guard` directive in the whitepaper asserting
      `benches/cortex-bench/Cargo.toml` is present; §10.2 gains a "Benchmark" column or note
      naming the bench for T-3 and leaving Measured empty; §11 F-13 narrowed; Appendix C M7 status
      updated.
- [ ] `CHANGELOG.md` entry under Unreleased, naming the new dependency and its version.
- [ ] Archive this brief.

## Not empowered

- Not to write any number into §10.2's Measured column. The developer-machine run is recorded in
  `docs/benchmarks/results/` as non-admissible and nowhere else.
- Not to add the harness, or any dependency, to a state crate; not to add a runtime dependency to
  anything.
- Not to implement the mailbox or gate in order to "complete" T-3; measure what exists and say
  what is missing.
- Not to add timing thresholds to CI; shared runners cannot hold them and a flaky gate gets
  switched off.

## Architectural empowerment

If you conclude that a separate bench crate is the wrong shape (for example that `criterion`
benches under each state crate's `benches/` with a workspace-level dev-dependency would be
cleaner), you may decide that, provided the ADR shows the state crates' `[dependencies]` tables
stay empty and `cargo tree -e normal` proves it. The empowerment reaches this brief's
instructions, not the whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo check --workspace --all-targets --locked
cargo test --workspace --locked
cargo bench -p cortex-bench --no-run       # or the harness's --test mode
cargo tree -e normal                      # state crates show no dependencies
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
npm run spec                              # the precondition directive above must be gone (archived)
```

If brief 005 has not landed, benchmark `schedule_fine` as it stands and say so in the report.

## Report

State: the harness chosen and the §2.1 argument; the benchmarks and their input distributions;
the developer-machine figures **with the words "not admissible"** beside them; the protocol
file's location; the state of F-13; and anything in this brief that turned out to be wrong when
re-derived.
