---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0007: The `.cortex` memory-mappable image format

## Context and Problem Statement

A reference connectome is on the order of 11 GB of records. Loading it through a serialisation framework that decodes fields and constructs objects would take minutes to hours and double the memory footprint during the load. How is a connectome stored and loaded?

## Decision Drivers

- Cold boot must be bounded by device bandwidth, not by CPU decode time (target T-7).
- The in-memory layout is already a stable, explicit ABI (ADR-0001).
- Integrity must be verifiable before any byte is trusted (whitepaper §8.10).

## Considered Options

1. Protocol Buffers or FlatBuffers with a verification pass.
2. A custom compressed stream decoded into arenas.
3. **A native container, `.cortex`, whose sections are the arena bytes themselves: a 64-byte `CortexFileHeader` (magic `VCORTEX1`, version, counts, section offset, CRC-64), a section directory, then 64-byte-aligned sections. Loaded by `mmap`, populated by the page cache, validated by checksum.**

## Decision Outcome

Option 3. The header is Implemented in `cortex-connectome`; the section directory and loader are Specified (whitepaper §8.7). Any change to any record layout, including reserved bytes, bumps `version`. Atomics are stored as plain integers and MUST be zero at rest. A foreign `version` fails closed.

### Consequences

- Good: the file is the arena; there is no deserialisation step and no second copy.
- Good: page-cache-warm restarts approach the T-7 warm target; cold loads are device-bound and stated as such.
- Bad: the format is endianness- and layout-specific; images are not portable across record versions without a migration tool.
- Bad: huge-page backing of a file mapping depends on filesystem and kernel support; the loader must fall back to 4 KB pages and report it.
- Serialisation frameworks remain acceptable for configuration and telemetry sidecars, where decode cost is irrelevant.

## Confirmation

`npx spec-guard` asserts `CortexFileHeader` exists. Milestone M4's exit test is a write, evict, re-hydrate round trip that preserves state bit-for-bit.
