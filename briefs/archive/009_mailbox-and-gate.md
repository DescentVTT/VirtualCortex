---
status: archived
date: 2026-09-10
---

> **Executed 2026-09-10 in pull request #17.** Resolves finding F-19; writes
> [ADR-0017](../../docs/adr/0017-mailbox-and-gate-protocol.md); image format version 4. The
> report is in the pull request and in `CHANGELOG.md`. The body below describes the tree before
> execution and is not maintained, apart from relative links, which gained one `../` so that
> they still resolve from `archive/`.

# Brief 009 — Mailbox and gate: the turn invariant as code (milestone M1; finding F-19)

## Mission

`cortex-core` carries the turn gate of axiom A3 as code: the idle / scheduled / running
transitions on `DendriticSuperNeuron::gate_state` with the orderings whitepaper §8.5 states and a
rule that no wakeup is lost; and an index-only, lock-free, multi-producer single-consumer mailbox
over a node arena the caller provides, pushed by any worker and drained whole by the worker that
holds the turn, without `unsafe`. An ADR amends [ADR-0006](../../docs/adr/0006-virtual-actor-turn-invariant.md)
with the head encoding and answers finding **F-19** (the ABA tag is a plain field beside the
atomic head and guards nothing). Two-thread tests prove exactly-once delivery and the absence of
lost wakeups. Milestone M1's exit test, push → gate → callback, exists. The parts of target T-3
that had no subject (R-1 steps 2 and 3) are benchmarked.

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; no dependencies
  ([ADR-0005](../../docs/adr/0005-crate-per-subsystem.md)); **no `unsafe`** (whitepaper TC-9): node
  storage is the caller's, as frame storage is in [ADR-0015](../../docs/adr/0015-embodiment-frame-abi.md).
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against the `adr-0016-state-crate-admission` branch on 2026-09-10.

- `crates/cortex-core/src/dynamics/neuron.rs`: `DendriticSuperNeuron` holds
  `mailbox_head_ptr: AtomicU64` at `[8..16)`, `mailbox_tag: u64` at `[16..24)` and
  `gate_state: AtomicU8` at `[56..57)`. No method touches any of them. The record is a control
  record (rule L-5): `Debug` only, not `Copy`.
- Whitepaper [§8.5](../../docs/WHITEPAPER.md#85-concurrency-and-ownership): a pusher does
  `compare_exchange(idle → scheduled)` and enqueues the unit on success; a worker sets `running`
  on claim and `idle` on release with release ordering. Mailboxes are "lock-free MPSC stacks
  whose head is `mailbox_head_ptr` and whose ABA guard is `mailbox_tag`"; nodes come from a
  per-worker pool, never the allocator (TC-5).
- Whitepaper [§6.1](../../docs/WHITEPAPER.md#61-scenario-r-1-lifecycle-of-one-spike) R-1: steps 2
  (mailbox push), 3 (gate check) and 4 (claim and drain the whole mailbox) are Specified.
- Whitepaper §11, finding **F-19**: `mailbox_tag` is a plain `u64` beside the atomic head; a tag
  that is not updated atomically with the head cannot guard against ABA. Two consequences to
  weigh: a tag packed into the same 64-bit word (index in the low 32 bits, tag in the high 32)
  would be atomic; and a stack that is only ever pushed and drained whole (the consumer swaps the
  head to the empty sentinel and walks the list) has no ABA hazard at all, because no participant
  compares a node it dereferenced with a node it will reuse. If the second holds, the tag is
  unnecessary and its eight bytes are reserved.
- Whitepaper [§8.3](../../docs/WHITEPAPER.md#83-determinism-model): a total order on delivery
  within a tick. A stack drains in reverse arrival order, so the order the worker applies a
  drained batch in must be stated: either the batch is applied as drained and the order is
  defined by the protocol, or the worker sorts a bounded batch by (tick, source) in a fixed
  buffer of its own.
- The lost-wakeup hazard: a push that arrives while the unit is `running` finds the gate not
  idle, does not enqueue, and the message sits in the mailbox after the worker sets `idle`. The
  rule that closes it: after `end_turn` the worker reads the head again and, if the mailbox is
  not empty, performs the pusher's own `try_schedule` and enqueues itself.
- Rule L-6: any change to any byte of the record bumps `CortexFileHeader::FORMAT_VERSION` (3 today)
  and updates whitepaper §5.2.1 and §5.2.2.
- Threads and heap types are excluded from state crates by the TC-5 directives, which exclude
  `crates/*/tests/`; the two-thread test lives in `crates/cortex-core/tests/`, as
  `crates/cortex-embodiment/tests/spsc.rs` does. Node payloads written by a producer and read by
  the consumer are atomics with relaxed stores ordered by the release on the head, the technique
  that test uses.
- `benches/cortex-bench/benches/hot_path.rs` measures wheel schedule and advance, efficacy,
  gating and ignition; `docs/benchmarks/README.md` names "mailbox push and gate (R-1 steps 2–3),
  which do not exist" as not measured.

## Deliverables

- [x] A new ADR at the next free number (`ls docs/adr`), `status: proposed` in the PR, amending
      ADR-0006: the head encoding (packed index and tag, or index only with the ABA argument for
      push and drain-whole); the empty sentinel; node ownership (the caller's arena, per-worker
      pools per §8.5); the three gate transitions with their orderings; the lost-wakeup rule; the
      drain order and how §8.3's total order is obtained from it; the format bump if any byte of
      the record changes. Committed `accepted` per `docs/adr/README.md`.
      ADR-0017: index + 1 encoding with zero empty; no tag (push-and-drain-whole has no ABA hazard), `mailbox_reserved`; the three transitions with acquire on `begin_turn`, release on the idle store, and sequential consistency on the four operations around the lost-wakeup window; reverse arrival order with the executor's sort Specified; format version 4. Committed `accepted` per `docs/adr/README.md`.
- [x] `GateState` (`#[repr(u8)]`: idle 0, scheduled 1, running 2) and, on `DendriticSuperNeuron`,
      `try_schedule(&self) -> bool` (compare-exchange idle → scheduled), `begin_turn(&self) -> bool`
      (scheduled → running), `end_turn(&self)` (running → idle, release); a `gate(&self) -> GateState`
      reader. Each method's doc comment states its ordering and why.
      `gate()` returns `Option<GateState>` so that a corrupt byte is visible; `end_turn` returns `bool` with the recheck built in, so a caller cannot forget the rule, only its return value.
- [x] `MailboxNode` (`#[repr(C)]`, atomics only: `next`, `payload`; size asserted) and, on the
      record, `mailbox_push(&self, nodes: &[MailboxNode], node: u32) -> bool` (refuses an index
      outside the arena or the sentinel) and `mailbox_drain(&self, nodes: &[MailboxNode]) -> impl Iterator<Item = u32>`
      (swaps the head to empty, walks `next`). No `unsafe`; no heap; the arena is the caller's.
      `mailbox_push(nodes, node, payload)` stores the payload itself rather than expecting it pre-written, so the release on the head orders it; `mailbox_drain` returns a `MailboxDrain` iterator of `(node, payload)`.
- [x] Unit tests: the gate transitions and their refusals; push then drain returns every node once
      and leaves the mailbox empty; an index outside the arena is refused; the record's layout is
      unchanged or bumped as the ADR says.
      Six tests, including termination on corrupt and cyclic lists and the lost-wakeup demonstration on a single-thread interleaving.
- [x] `crates/cortex-core/tests/mailbox.rs`: four producer threads push 25 000 payloads each into
      one unit through a shared arena while one consumer drains under the gate; every payload is
      delivered exactly once; when the producers are done the mailbox is empty and the gate is
      idle (no lost wakeup); a run with the recheck rule disabled would leave messages behind,
      which the test demonstrates on a single-threaded interleaving.
      The recheck-disabled demonstration is the single-thread unit test, as the brief allowed.
- [x] `benches/cortex-bench`: `mailbox/push` and `gate/try_schedule` benchmarks from the fixed
      seed; `docs/benchmarks/README.md` updated; no figure enters the whitepaper
      ([ADR-0010](../../docs/adr/0010-measured-or-target.md)).
      Named `mailbox/push_drain_x16` and `gate/schedule_begin_end`.
- [x] Whitepaper §5.2.1 (fields, public API, status), §5.2.2 version history if bumped, §6.1
      steps 2 to 4 Implemented, §8.5 rewritten to the protocol as built, §11 F-19 resolved,
      Appendix C M1 status; `CHANGELOG.md` entry; archive this brief.

## Not empowered

- Not to write `unsafe`, spawn threads in the library, or add a dependency.
- Not to implement the executor, the work-stealing deque or batch integration (milestone M2).
- Not to change `SynapseBlock` or any record other than `DendriticSuperNeuron`.
- Not to write any membrane dynamics; the callback in M1's exit test is a test closure.

## Architectural empowerment

You may change the encoding of `mailbox_head_ptr`, remove or repurpose `mailbox_tag`, and
choose the node layout, provided the ADR states the ABA argument, the orderings and the drain
order, and the format version moves if a byte moves. You may conclude that the mailbox should be
a queue rather than a stack if a deterministic order needs it, provided the queue is still
index-only and free of `unsafe`. The empowerment reaches this brief's instructions, not the
whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo test --workspace            # the two-thread test runs under the standard harness
cargo test --workspace --release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo bench -p cortex-bench --bench hot_path -- --test
cargo +1.85 check --workspace --all-targets
npm run spec                      # the precondition directive above must be gone (archived)
```

## Report

State: the head encoding decided and the ABA argument in two sentences; the lost-wakeup rule and
the test that demonstrates it; the drain order and how §8.3 is satisfied; whether the format
version moved and why; the state of F-19 and of milestone M1; and anything in this brief that
turned out to be wrong when re-derived.
