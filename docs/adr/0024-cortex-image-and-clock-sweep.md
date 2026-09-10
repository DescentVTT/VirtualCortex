---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0023
---

# ADR-0024: The `.cortex` image — section directory, table-free CRC-64/XZ, a read-into-arenas loader and writer, a write-ahead log for the clock sweep, and the Tier-2 delta record (format version 7)

## Context and Problem Statement

`CortexFileHeader` existed with no directory, no checksum computation, no writer, no loader and no eviction; whitepaper §8.7 specified a section directory of `(kind, offset, length, crc64)` entries, an image whose bytes are the arenas, a fail-closed rule for a foreign version, and atomics stored as their plain values, zero at rest; §8.6 specified the clock sweep of axiom A5 that writes a quiet unit's 64 bytes out and re-hydrates them on the next spike; finding F-20 recorded that `plastic_delta_head` (`u16`) cannot address the Tier-2 delta table Appendix A sizes at 10⁹; finding F-23 ([ADR-0022](0022-synapse-fan-out-and-stdp.md)) left the synapse arena's size to the loader. Brief 015 asked the ADR to decide the directory layout and kinds, the CRC polynomial and its vectors, header validation, the loader's strategy (`mmap` under an `unsafe` invariant, or a plain read), the writer, the sweep's threshold and hand, the write-ahead log or its absence, the 16-byte delta record and the 32-bit head, and the format version.

## Decision Drivers

- §8.7: a foreign version MUST fail closed; every byte the loader trusts must be sealed.
- TC-9: `unsafe` only under an ADR naming the invariant and the test; [ADR-0023](0023-executor.md) already holds the one `unsafe`, and `mmap` would add a second with a different invariant (the mapping outlives every borrow; the file is not truncated while mapped).
- TC-5: nothing in the tick loop allocates or makes a system call; eviction and re-hydration happen between ticks, on the coordinator, where the arenas are exclusively the caller's.
- §8.3: the bytes of a record must be the same on every target, so the encoding is explicit little-endian field by field, never a transmute.
- ADR-0022's index + 1 encoding: zero is nothing, so a zeroed arena is a valid empty one and a unit at rest names no delta.
- "Latest ≠ Newest": CRC-64/XZ (ECMA-182, reflected) has published check values and thirty years of use; a table larger than a cache line is a cost the brief excluded from the state crate.

## Considered Options

1. `mmap` the image with `MAP_POPULATE` and hand section offsets to the arenas (R-7 as written); a table-driven CRC; eviction straight into the image with the section checksums re-sealed on every eviction.
2. **Read the whole file and decode every record into the arenas; bitwise CRC-64/XZ in the state crate; a section directory whose kinds are Appendix A's row numbers; a write-ahead log that eviction appends to and re-hydration reads from, the image itself immutable once written; the delta head widened to 32 bits at `[60..64)` as index + 1; a 16-byte `PlasticDelta`; format version 7.**
3. Serialise through a framework (rejected once already by [ADR-0007](0007-cortex-image-format.md)).

## Decision Outcome

Option 2.

- **Header (version 7).** `section_count: u32` at `[56..60)` (the padding shrinks to `[60..64)`); `num_synapses` counts blocks; `crc64` at `[48..56)` is the CRC-64/XZ of all 64 bytes with that field read as zero. `validate` checks magic, version, checksum and padding in that order and reports the first failure; `new` seals; `encode` and `decode` are explicit little-endian.
- **Directory.** `SectionEntry`, 64 bytes: `kind: u32`, `record_size: u32`, `offset: u64` (a multiple of 64), `length: u64` (bytes of records; the section is padded to 64 after it), `crc64: u64` over the `length` bytes, 32 reserved. A kind is the row number of Appendix A: `SECTION_NEURON` = 2, `SECTION_SYNAPSE` = 3, `SECTION_PLASTIC_DELTA` = 37, with `SECTION_MACRO_COLUMN` = 1, `SECTION_LAMINAR` = 38 and `SECTION_ROUTING` = 39 named and Specified. `is_well_formed` and `record_count` are the loader's first checks.
- **CRC.** CRC-64/XZ: polynomial `0x42F0E1EBA9EA3693` reflected (`0xC96C5795D7870F42`), initial and final value all ones, one shift-and-conditional-xor per bit, no table; `Crc64` streams, `crc64` seals in one call; the check value of `"123456789"` is `0x995DC9BBDF1939FA`, and the empty string's is zero. Eight operations per byte is slow for a gigabyte image (a T-7 subject); a sliced table in the runtime is the optimisation, Specified, and the state crate stays table-free.
- **Bytes of a record.** `DendriticSuperNeuron::{encode, decode, restore_plain_fields, same_bytes, is_image_ready, is_at_rest_image}`, `SynapseBlock::{encode, decode}`, `PlasticDelta::{encode, decode}` in `cortex-core`: explicit little-endian, the atomics as their values. No `bytemuck`, no `unsafe`, the same bytes on x86-64 and AArch64.
- **Writer.** `Image::encode(&Executor)` at a quiescent point (`is_quiescent`: every mailbox empty, no token in flight); a scheduled unit with an empty mailbox is written idle, and the loader wakes every unit that is not at rest; an evicted unit's record is folded back from the log. Sections: neurons, synapses and, when the executor has one, deltas. `Image::write` puts the bytes at a path. A hot checkpoint of a run with messages or tokens in flight is Specified (it needs the wheels' and mailboxes' contents in the image).
- **Loader.** `Image::open(path, config)` and `Image::decode(bytes, config)`: read the whole file; validate the header; validate every directory entry (well-formed, a known kind with its record size, in bounds, checksum); require the neuron and synapse sections and let their counts size the arenas; refuse more blocks than a synapse token can name (F-23); decode every block, refusing a target or a chain link outside the arenas and a delay at or beyond the wheel's horizon (§6.2); decode every delta, refusing a dangling block or link; decode every unit, refusing one not at rest (§8.7) or with a dangling chain or delta head; wake the units that are not at rest. The copy is the cost; `mmap` (R-7's `MAP_POPULATE`, huge pages, NUMA pinning) is Specified with its invariant named above.
- **Sweep and log.** `Executor::attach_log(path)` creates a write-ahead log: append-only entries of `(unit index, 64 bytes)`, the newest offset per unit kept in memory, positional reads and writes. `Executor::sweep(quiet_ticks, budget)`, between ticks, walks the unit arena with a clock hand and evicts at most `budget` units that are idle with an empty mailbox, at rest, and quiet for `quiet_ticks` since their last spike: the record goes to the log and the slot keeps only its id, its last spike stamp (which STDP pairs against, ADR-0022), its gate and its mailbox. A delivery or a wake to an evicted unit marks it; after the tick, before the turn that drains the message, the coordinator re-hydrates it from the log (`restore_plain_fields`, keeping the gate and the mailbox the delivery left). The slot is not returned to a free list: that needs an id-to-slot indirection the arenas do not have, and is Specified with §8.6.
- **Delta record and head.** `PlasticDelta`, 16 bytes: `block_idx: u32` (index + 1), `slot: u8`, `_pad: u8`, `delta_q1_15: i16`, `epoch: u32`, `next: u32` (index + 1). `DendriticSuperNeuron::plastic_delta_head` is `u32` at `[60..64)` as index + 1; `[52..54)` is reserved. `push_delta`, `delta_head`, `set_delta_head`, `deltas` (a bounded walk) exist; the executor stores the arena and the image carries it; applying deltas is Specified (milestone M5). F-20 is resolved.
- **F-23.** The loader refuses an image with more than $2^{26}$ blocks, and Appendix A's row 3 follows the token: 67 108 864 blocks (268 M synapse slots, 4.29 GB). Widening the token is an amendment of [ADR-0013](0013-timing-wheel-geometry.md) when the capacity is needed.

### Consequences

- Good: milestone M4's exit test passes: a run that sweeps and re-hydrates ends in an image byte-identical to one that never evicted, over sixty kicks into a 96-unit network with STDP; a written image opens byte-identical and runs identically; a flipped byte, a truncated file, a foreign version, a bad header checksum, a wrong magic, an over-horizon delay and a dangling index each fail closed with a named error.
- Good: no `unsafe` and no dependency entered; the log keeps the image immutable, so its checksums stay true until the next checkpoint.
- Bad: the loader copies; cold boot is bounded by the read and the decode, not by the page cache, until `mmap` lands (T-7 stays a Target).
- Bad: eviction frees no memory: the slot is zeroed, not reclaimed. What it proves is the write-out and re-hydration path bit for bit; reclamation needs the indirection.
- Bad: a checkpoint needs quiescence; a live run with tokens in flight cannot be written.
- Bad: the bitwise CRC is slow on a large image.

## Alternatives considered and why rejected

- **`mmap`** needs `unsafe` with a second invariant (mapping lifetime against borrows, and against truncation) and a dependency (`rustix` or `nix`) for the flags R-7 names; the read-into-arenas loader proves the format first, and the brief allowed it.
- **A table-driven CRC** in the state crate is 2 KB for a 256-entry table; the brief excluded a table larger than a cache line, and a 16-entry nibble table is still two lines.
- **Eviction into the image itself** invalidates the neuron section's checksum on every eviction, so the image would have to be re-sealed (a full pass) or trusted unsealed; the log keeps the image sealed.
- **A `u16` head widened in place at `[52..54)`** has no room; the reserved bytes at `[60..64)` do, and index + 1 keeps zero as none.

## Confirmation

`cortex-connectome`: five tests (the constants; the CRC check value, the empty string, streaming; a header round trip; the four refusals in order; a section entry's round trip, count and shape). `cortex-core`: three delta tests (an empty delta, a byte round trip, push and walk with a cycle and an out-of-arena head) and four serial tests (a unit round-trips and is image-ready at rest; a scheduled, running or full unit is not; restore keeps the slot's gate and mailbox; a block round-trips). `runtime/cortex-runtime/tests/image.rs`: the round trip and identical run, the corrupted, truncated and foreign refusals, the horizon and dangling refusals, the quiescence rule, and the M4 exit test. `npx spec-guard` asserts `SectionEntry`, `fn crc64`, `PlasticDelta`, `fn sweep` and `FORMAT_VERSION: u32 = 7` exist.
