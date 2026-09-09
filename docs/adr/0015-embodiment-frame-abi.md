---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0006
---

# ADR-0015: Embodiment frame ABI and single-producer single-consumer ring protocol

## Context and Problem Statement

`cortex-embodiment` had a 64-byte control block (`EmbodimentRingBuffer`) and nothing else (whitepaper finding F-17): no frame type, no capacity, no protocol. The engine must exchange a motor command and an observation with a physics engine or robot controller every millisecond over shared memory (whitepaper §6.4). What crosses the boundary, how big is the ring, how do the two sides synchronise, and where does the ABI version live?

## Decision Drivers

- The 64-byte record discipline (ADR-0001); the numeric model (ADR-0002); no `unsafe` without an ADR (whitepaper TC-9) and no dependencies in state crates (ADR-0005).
- Whitepaper §8.5: shared-memory rings use acquire/release on their cursors and nothing else.
- Whitepaper §8.9: an external watchdog engages braking after five missed periods of the heartbeat; the engine must not be the last line of defence.
- The frames are IPC records, not part of the `.cortex` image, so `CortexFileHeader::FORMAT_VERSION` is unaffected and the frame ABI needs its own version.
- Both sides run on the same host: native endianness; no serialisation.

## Considered Options

1. Frames of twelve Q16.16 positions **and** twelve velocities (128 B, two cache lines).
2. Frames of six positions and six velocities (64 B), two frames per period for twelve joints.
3. **Sixty-four-byte frames: `TorqueFrame` (epoch, twelve Q16.16 torques), `JointStateFrame` (epoch, twelve Q16.16 positions); velocity derived by the consumer as the finite difference of consecutive positions at the known period.**
4. A variable-length message format.

## Decision Outcome

Option 3.

- **Frames.** `TorqueFrame { epoch: u64, torques_q16: [i32; 12], _reserved: [u8; 8] }` and `JointStateFrame { epoch: u64, positions_q16: [i32; 12], _reserved: [u8; 8] }`, both `#[repr(C, align(64))]`, 64 bytes, size and alignment asserted at compile time, deriving `Clone, Copy, Debug, Default, PartialEq, Eq`. `DOF = 12` covers the quadruped and arm targets; unused entries are zero. Velocities are not carried: at a fixed 1 ms period they are `Δposition × 1000` and a plant reports positions natively from its encoders; a plant that reports velocities natively would get a second ring, which is a new ADR.
- **Capacity.** `CAPACITY = 16` frames per ring, a power of two so that the slot of a cursor is `cursor & 15`. Sixteen milliseconds of backpressure at the period is three times the watchdog window; a producer that finds the ring full has already lost the loop.
- **Cursor protocol.** Monotonic 64-bit cursors in the control block; empty when equal, full when they differ by `CAPACITY`. Producer: `producer_claim` (relaxed-load own write cursor, acquire-load the read cursor; `None` when full), write the frame, `producer_publish(epoch, now_ms)` (store epoch and heartbeat relaxed, release-store the write cursor). Consumer: `consumer_peek` (relaxed-load own read cursor, acquire-load the write cursor; `None` when empty), read the frame, `consumer_release` (release-store the read cursor). The release/acquire pair on each cursor orders the payload writes before the payload reads, and the payload needs no synchronisation of its own; this is the argument in the doc comments and the two-thread test.
- **Heartbeat.** `heartbeat_ms` is the producer's monotonic clock in milliseconds at the moment of publish, supplied by the caller (the crate reads no clock). The watchdog compares it with its own clock; five missed periods is five milliseconds behind.
- **Version.** `FRAME_ABI_VERSION = 1` and `CAPACITY` are written once into the control block's former reserved bytes (`abi_version: u32` at `[32..36)`, `capacity: u32` at `[36..40)`, `reserved: [u8; 24]`) by `new()`; `is_compatible()` checks both and a consumer MUST call it before reading a frame. Any change to the frames, the control block or the protocol bumps the version.
- **Storage is the caller's.** The protocol is index-only and contains no `unsafe`. The runtime that maps `/dev/shm` owns the frame storage and will need `unsafe` for the mapping, under its own ADR; this crate can be exercised over any storage, which is how its two-thread test works.
- **Endianness.** Native; both sides are one host.

### Consequences

- Good: one frame per period in each direction, one cache line each; the whole exchange per period is two lines plus two cursor stores.
- Good: the protocol is testable without an operating system; the test drives 10⁵ frames through two threads over atomic slots ordered only by the cursors.
- Bad: no velocities on the wire; a consumer that needs them at the first frame has none until the second.
- Bad: twelve degrees of freedom is a fixed ceiling; more needs a multi-frame message and a new ADR.
- Bad: `abi_version` and `capacity` are plain fields written once; a producer that forgets `new()` publishes zeros and every consumer refuses the ring, which is the safe failure.

## Alternatives considered and why rejected

- **Option 1**: two cache lines per frame doubles the transfer for data the consumer can derive.
- **Option 2**: two frames per period complicates epoch matching and halves the effective capacity.
- **Option 4**: variable length needs parsing and length checks on the hot path; a fixed record does not.

## Confirmation

Eight tests in `cortex-embodiment`, seven unit tests and one integration test (`tests/spsc.rs`, kept out of the library so that the library itself stays free of thread spawning and heap types under whitepaper TC-5): layouts; `new`/`Default` carry the ABI and `is_compatible` refuses a foreign version; default frames are zero; an empty ring refuses the consumer and offers slot 0 to the producer; empty and full behaviour with slot reuse after release; wrap-around over forty frames with cursors still counting; epoch and heartbeat follow the last publish and the heartbeat is monotonic; two threads exchange 100 000 frames in order with no loss over relaxed-stored payload. `npx spec-guard` asserts `TorqueFrame`, `JointStateFrame` and `producer_claim` exist.
