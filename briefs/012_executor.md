---
status: proposed
date: 2026-09-10
---

# Brief 012 — The executor: a runtime crate, core-pinned workers, work stealing, batch draining (milestone M2)

## Mission

A runtime crate outside `crates/` composes the state crates for the first time: a fixed pool of
worker threads, one per configured core, each with a work-stealing deque of unit indices and its
own `WorkerWheel`; a turn is `begin_turn`, drain the mailbox whole, order the batch
deterministically (whitepaper §8.3), integrate (`integrate`, `step_stp`), `end_turn` and
re-enqueue when it says so; each fine tick drains the worker's wheel into mailboxes. An ADR
decides the crate's shape, the deque (in-house and index-only without `unsafe`, or a vetted
dependency added to [ADR-0005](../docs/adr/0005-crate-per-subsystem.md)'s allow-list), the batch
order, and core pinning. Milestone M2's exit test passes: 10⁶ events delivered under contention
with no loss and no deadlock. Whitepaper R-1 steps 2 to 5 are Implemented end to end.

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; a dependency enters only through the allow-list of
  ADR-0005 and an ADR that names its stability record; **no `unsafe`** without an ADR naming the
  invariant and the test (whitepaper TC-9); state crates stay dependency-free and `no_std`.
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` (`20c6243`) on 2026-09-10.

- `Cargo.toml` lists thirty-two state crates under `crates/` and `benches/cortex-bench`; no
  runtime crate exists. Whitepaper [§4.3](../docs/WHITEPAPER.md#43-decomposition-principle):
  "When a runtime crate is introduced it will compose them"; ADR-0005: dependencies flow only from
  a runtime crate downward, on an allow-list (initially `crossbeam-epoch`, `rustix` or `nix`,
  `bytemuck`), each addition recorded in the changelog.
- `crates/cortex-core`: `DendriticSuperNeuron::{try_schedule, begin_turn, end_turn, mailbox_push,
  mailbox_drain}` ([ADR-0017](../docs/adr/0017-mailbox-and-gate-protocol.md)): a pusher pushes
  then schedules; `end_turn` returns `true` when the caller must enqueue the unit again; a drain
  yields reverse arrival order and the executor is to sort a batch in a bounded buffer of its own
  (whitepaper §8.3, Specified). `integrate(basal, apical, now_tick)` and
  `step_stp(elapsed_ticks)` ([ADR-0018](../docs/adr/0018-membrane-integration.md),
  [ADR-0019](../docs/adr/0019-short-term-plasticity.md)). `WorkerWheel::{schedule, advance}`
  ([ADR-0013](../docs/adr/0013-timing-wheel-geometry.md)): `advance` returns the due tokens in a
  deterministic order; a token is a 28-bit unit index or `SynapseBlock` offset (opaque to the wheel).
- Whitepaper [§6.1](../docs/WHITEPAPER.md#61-scenario-r-1-lifecycle-of-one-spike): step 1 wheel
  (Implemented), steps 2 to 5 Implemented as record methods, "the deque is the executor's
  (Specified)"; [§7.3](../docs/WHITEPAPER.md#73-process-and-thread-model-specified): one process,
  workers pinned one per isolated core, telemetry and scrub threads on non-isolated cores, a
  seccomp filter after initialisation (Specified); §8.5: nodes come from a per-worker pool.
- Whitepaper TC-5: no allocation, blocking or syscalls on the hot path after initialisation. The
  `spec-guard` directives that hold TC-5 target `crates/`; a crate under `runtime/` is outside
  them, which is where its threads, its arenas and its start-up allocation belong.
- Whitepaper [§8.3](../docs/WHITEPAPER.md#83-determinism-model): a run is `(image, seed, input
  trace)`; two runs MUST produce bit-identical arenas. A batch's order must not depend on which
  worker pushed first: sort by the payload key.
- `benches/cortex-bench` shows a non-state workspace member with `publish = false` and a
  dev-dependency; `crates/cortex-core/tests/mailbox.rs` models the deque with one flag and four
  producers; that model is the shape of M2's test at scale.
- Fan-out through `SynapseBlock` chains is brief 013's; until it lands, the executor delivers
  tokens the test enqueues, and a token's payload is a unit index.

<!-- @assert-absence target="Cargo.toml" symbol="cortex-runtime" reason="precondition: no runtime crate exists; archive this brief when it does" -->

## Deliverables

- [ ] A new ADR at the next free number (`ls docs/adr`), `status: proposed` in the PR: the
      runtime crate's name and path (`runtime/cortex-runtime`, `publish = false` until 1.0), its
      dependency decisions against the allow-list (the deque: an in-house index-only Chase–Lev
      over a fixed array without `unsafe`, or `crossbeam-deque` with its stability record and an
      amendment of ADR-0005's list), core pinning (a vetted crate, or Specified with the syscall
      named), the batch order (sort by payload key in a per-worker fixed buffer; capacity and the
      overflow rule), how a worker's wheel tokens become mailbox pushes, and what happens to a
      unit whose turn cannot be claimed. Committed `accepted` per `docs/adr/README.md`.
- [ ] `runtime/cortex-runtime`: `Executor::new(config)` allocating every arena and pool once;
      `enqueue(unit)`; the worker loop as above; `tick()` advancing every wheel and delivering
      due tokens; `shutdown()`; no allocation after `new` (asserted by a counting allocator in a
      test, or by a reviewed invariant if the test is not possible without `unsafe`).
- [ ] Tests: 10⁶ events from N producers into M units across W workers, every event delivered
      exactly once, no deadlock, run for W in {1, 2, 4}; bit-identical arena contents after the
      same input trace on W = 1 and W = 4 (the first differential test toward T-1); a unit that
      keeps receiving is never starved; `end_turn`'s re-enqueue is exercised under contention.
- [ ] `benches/cortex-bench` gains `executor/push_to_turn` (R-1 steps 2 to 4 end to end on one
      worker); `docs/benchmarks/README.md` follows.
- [ ] Whitepaper §5.1 (a runtime layer above the state crates, with its dependency edges now
      real), §6.1 (the deque and the batch order Implemented), §7.3, §8.3 (the batch order as
      built), §8.5, Appendix C M2; TC-2's directive text if the allow-list grew;
      `CHANGELOG.md`; archive this brief.

## Not empowered

- Not to change any state crate's record or rule; the executor composes them.
- Not to implement fan-out, STDP (brief 013), eviction or the image (brief 015).
- Not to add a dependency outside the allow-list without amending ADR-0005 in the ADR.
- Not to write `unsafe`; if pinning or a deque needs it, it is Specified with the invariant
  named for a later ADR.

## Architectural empowerment

You may decide that the deque is a bounded per-worker ring with a global overflow queue rather
than a Chase–Lev deque if the ADR shows the stealing pattern the engine needs is served; you may
decide that ticks are driven by one coordinator thread rather than by every worker; you may
place the runtime crate elsewhere than `runtime/` if the TC-5 directives still exclude it. The
empowerment reaches this brief's instructions, not the whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo test --workspace
cargo test --workspace --release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo bench -p cortex-bench --bench hot_path -- --test
cargo +1.85 check --workspace --all-targets
npm run spec                      # the precondition directive above must be gone (archived)
```

## Report

State: the crate's shape and every dependency it took, with the stability record of each; the
deque decided and why; the batch order and how §8.3 is met; the exit test's figures (events,
workers, wall time, not admissible); what remains Specified in §7.3; and anything in this brief
that turned out to be wrong when re-derived.
