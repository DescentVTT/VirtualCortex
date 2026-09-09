---
status: archived
date: 2026-09-10
---

> **Executed 2026-09-10 in pull request #12.** Narrows finding F-17 to the torque decoder, the
> mapping and the loop; writes [ADR-0015](../../docs/adr/0015-embodiment-frame-abi.md). The report
> is in the pull request and in `CHANGELOG.md`. The body below describes the tree before execution
> and is not maintained, apart from relative links, which gained one `../` so that they still
> resolve from `archive/`.

# Brief 008 — Embodiment payload rings: the frame ABI and the single-producer single-consumer protocol

## Mission

`cortex-embodiment` defines the two 64-byte frame records that cross the shared-memory boundary
(`TorqueFrame` out, `JointStateFrame` in), the single-producer single-consumer ring protocol over
them using only acquire/release atomics on the existing control block, and the heartbeat and
epoch semantics the watchdog relies on; an ADR fixes the frame ABI as a versioned wire contract;
two-thread tests prove ordering, backpressure and wrap-around; whitepaper finding **F-17** is
narrowed from "payload rings and torque decoder do not exist" to "torque decoder does not exist".

## Standing directives

- **The repository wins over the document.** Record a disagreement as a numbered finding in
  whitepaper §11; do not fix it silently.
- **Verify before asserting.** Read the file; run the command.
- **Label every claim** Implemented, Specified, Target or Hypothesis.
- **Latest ≠ Newest.** Stable Rust only; no dependencies
  ([ADR-0005](../../docs/adr/0005-crate-per-subsystem.md)); **no `unsafe`** (whitepaper TC-9): the
  mapping of shared memory is the runtime's job, this crate defines the protocol over storage the
  caller provides.
- **Say what you did not do** in the closing report.
- Branch and pull request; Conventional Commits with a real body; run every command in
  `CLAUDE.md` before pushing.

## Context

Re-derived against `main` on 2026-09-10.

- `crates/cortex-embodiment/src/lib.rs`: `EmbodimentRingBuffer` is a 64-byte control record with
  `write_cursor`, `read_cursor`, `epoch_id`, `heartbeat_ms` (all `AtomicU64`) and
  `reserved: [u8; 32]`; `const fn new()`, `Default`, `Debug`. Nothing else exists: no frame type,
  no push or pop, no capacity, no test.
- Whitepaper [§5.2.4](../../docs/WHITEPAPER.md#524-cortex-embodiment--sensorimotor-loop): "the payload
  rings (torque frames out, joint state in) follow it in the shared mapping";
  [§6.4](../../docs/WHITEPAPER.md#64-scenario-r-4-embodiment-period-specified) (R-4): write frame,
  release-store `write_cursor`, bump `heartbeat_ms`, acquire-load joint state,
  `clock_nanosleep` to the next 1 ms boundary; [§8.5](../../docs/WHITEPAPER.md#85-concurrency-and-ownership):
  "shared-memory rings use acquire/release on their cursors and nothing else";
  [§8.9](../../docs/WHITEPAPER.md#89-error-handling-and-fail-safe): an external watchdog engages
  braking after 5 missed periods of `heartbeat_ms`.
- Targets T-4 (1.000 ms period) and T-5 (jitter) in §10.2 are about the loop, not this round; the
  loop needs `clock_nanosleep` and a mapping and is runtime work (milestone M6).
- The previous specification (2.8.0) sketched `joint_torques: [i32; 12]` inside the control block
  and got 72 bytes; whitepaper 3.0.0 finding F-1 records that. Twelve degrees of freedom in
  Q16.16 is 48 bytes, which fits a 64-byte frame with an epoch and reserved bytes.
- The frames are IPC records, not part of the `.cortex` image, so `CortexFileHeader::FORMAT_VERSION`
  is untouched; the frame ABI needs its own version, in the control block's reserved bytes or in
  the frame.
- Rule L-5 (§8.2): a record with atomics is a control record; a frame with none derives the five
  traits. Rule L-4: padding is explicit and zero.

<!-- @assert-absence target="crates/cortex-embodiment" symbol="TorqueFrame" word="true" reason="precondition: F-17 is open and no frame type exists; archive this brief when it does" -->

## Deliverables

- [x] A new ADR at the next free number (`ls docs/adr`), `status: proposed` in the PR, fixing the
      frame ABI: `TorqueFrame` (64 B: epoch, twelve Q16.16 torques, reserved) and `JointStateFrame`
      (64 B: epoch, twelve Q16.16 joint positions or six positions and six velocities, decided and
      justified, reserved); the ring capacity (a power of two, and why that size at 1 ms per frame);
      the cursor protocol (monotonic 64-bit counters, index = cursor & (capacity − 1), full when
      `write − read == capacity`, empty when equal); the heartbeat unit and update rule; a
      `FRAME_ABI_VERSION` and where it lives; and endianness (native; both sides are the same host).
      ADR-0015: twelve positions, velocities derived; capacity 16; version and capacity in the
      control block's former reserved bytes. Committed `accepted` per `docs/adr/README.md`.
- [x] The two frame records with `#[repr(C, align(64))]`, `const _` size/alignment assertions and
      the five derives; `EmbodimentRingBuffer` gains capacity and version in its reserved bytes if
      the ADR puts them there, keeping 64 bytes.
- [x] The protocol as index-only methods on the control block, generic over storage the caller
      provides (`&[T]`-shaped, no `unsafe`): `producer_claim() -> Option<usize>`,
      `producer_publish(index, epoch)` (release-store, heartbeat bump), `consumer_peek() -> Option<usize>`
      (acquire-load), `consumer_release(index)`. State the ordering argument in doc comments.
      The index parameters were dropped: the cursors already know which slot is claimed or peeked,
      so passing the index back would only invite a mismatch. `producer_publish(epoch, now_ms)`
      takes the producer's clock because the crate reads none.
- [x] Tests, in `#[cfg(test)]` with the standard test harness: two `std` threads exchange 10⁵
      frames through the protocol with no loss and in order; a full ring returns `None` to the
      producer; an empty ring returns `None` to the consumer; wrap-around at the capacity
      boundary; epoch and heartbeat are monotonic; the frame layout assertions.
- [x] Whitepaper §5.2.4 (new layout tables, public API, status), §3.2 interfaces row, §6.4
      (which steps are now Implemented), §8.9 (heartbeat semantics), §11 F-17 narrowed, Appendix C
      M6 status.
- [x] `CHANGELOG.md` entry under Unreleased.
- [x] Archive this brief.

## Not empowered

- Not to write `unsafe`, open `/dev/shm`, call `mmap`, or add `nix`/`rustix`; the mapping is the
  runtime crate's (Specified).
- Not to implement the torque decoder (layer-5 bursts to torques) or any dynamics; that is the
  remainder of F-17.
- Not to implement the 1 ms loop or `clock_nanosleep`; T-4 and T-5 stay Targets.
- Not to change the control block's four atomics or its 64-byte size.

## Architectural empowerment

If you conclude that a 64-byte frame is the wrong unit (for example that joint state needs
positions, velocities and torques together, or that more than twelve degrees of freedom must be
supported), you may decide a different frame size or a multi-frame message, provided the ADR
shows the layout, the alignment argument and the cost per period, and provided every record stays
`#[repr(C)]` with asserted size. The empowerment reaches this brief's instructions, not the
whitepaper's invariants or `CLAUDE.md`.

## Verification

```bash
cargo test --workspace            # the two-thread test runs under the standard harness
cargo test --workspace --release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
npm run spec                      # the precondition directive above must be gone (archived)
```

## Report

State: the frame ABI as decided (fields, capacity, version, where the version lives); the
ordering argument for the protocol in two sentences; the tests and what each proves; the state of
F-17 and of milestone M6; and anything in this brief that turned out to be wrong when re-derived.
