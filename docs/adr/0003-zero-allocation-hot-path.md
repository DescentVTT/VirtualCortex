---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0003: Zero allocation and zero syscalls on the hot path

## Context and Problem Statement

Tail latency in event-driven systems is dominated by three things: heap allocation, system calls and pointer chasing. The engine's tick loop must deliver spikes with bounded latency (quality goal 3) and hold a 1 ms embodiment period with bounded jitter (quality goal 4).

## Decision Drivers

- `malloc` is unbounded in time and touches global state.
- A syscall on an isolated core defeats `nohz_full` and may migrate the thread.
- Page faults on first touch are a latency cliff; memory must be pre-faulted.

## Considered Options

1. Allocate freely and rely on a fast allocator (mimalloc, jemalloc).
2. Allocate during the tick but only from a thread-local bump arena reset per tick.
3. **Allocate all arenas, mailbox node pools, rings and wheels once at initialisation from pre-faulted huge pages; after that, the tick loop MUST NOT allocate, block or make a system call.**

## Decision Outcome

Option 3. State crates carry no heap-owning types (`Box`, `Vec`, `String`) and SHOULD be `#![no_std]`. Runtime crates allocate only in their constructors. Sleeping until the next embodiment boundary is done with one `clock_nanosleep(TIMER_ABSTIME)` per period, which is the sole permitted blocking call and lives outside the tick loop proper.

### Consequences

- Good: latency is a function of cache behaviour alone.
- Good: `no_std` state crates are trivially portable and auditable.
- Bad: capacities are fixed at start-up; growing an arena is a restart or a planned re-hydration.
- Bad: only 4 of 18 crates are `#![no_std]` today (finding F-6).

## Confirmation

`npx spec-guard` asserts the absence of `Box<`, `Vec<` and `std::thread` in `crates/` (whitepaper §2.2 directives). A runtime crate, when it exists, will be checked with a syscall-tracing test on Linux.
