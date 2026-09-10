---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
amends: ADR-0006
---

# ADR-0017: Mailbox and gate protocol — an index stack drained whole, no ABA tag, four sequentially consistent operations

## Context and Problem Statement

[ADR-0006](0006-virtual-actor-turn-invariant.md) decided the turn gate (idle / scheduled / running on `gate_state`) and "a lock-free MPSC mailbox whose head pointer is paired with an ABA tag", and left both Specified. Whitepaper finding F-19 observed that `mailbox_tag` is a plain `u64` beside the atomic head: a tag that is not updated in the same atomic operation as the head guards nothing. Brief 009 asked for the protocol as code without `unsafe`, for the head encoding, for a rule that no wakeup is lost, and for the drain order. What is the encoding, is a tag needed, which orderings close the lost-wakeup window, and what order does a drained batch have?

## Decision Drivers

- Axiom A3: at most one worker writes a unit's plain fields per tick, decided by one atomic operation on the record itself; no lock word ([ADR-0006](0006-virtual-actor-turn-invariant.md)).
- TC-5: no allocation on the hot path; nodes come from a per-worker pool the caller owns (whitepaper §8.5), as frame storage does in [ADR-0015](0015-embodiment-frame-abi.md).
- TC-9: no `unsafe`. Payloads and links are atomics so that a producer can write them through a shared reference; their relaxed stores are ordered by the operation on the head.
- §8.7: an image at rest holds every atomic as its plain integer value, and it MUST read as idle with an empty mailbox. Zero must therefore mean empty.
- A wakeup lost between a push and the end of a turn would strand a message until the next unrelated push; the rule that prevents it must be in the record's own methods, not in every caller.
- §8.3 asks for a total order on delivery; a mailbox fed by several workers cannot supply one by itself.

## Considered Options

1. A Treiber stack of node indices with a tag packed into the head word (index in the low 32 bits, tag in the high 32), popped one node at a time.
2. **A stack of node indices pushed by compare-exchange and drained whole by one swap; no tag; the head holds `index + 1` so that zero is empty; the four operations around the lost-wakeup window are sequentially consistent.**
3. A Michael–Scott queue (two atomics, a dummy node) for arrival order.

## Decision Outcome

Option 2.

- **Encoding.** `mailbox_head_ptr` holds `node index + 1`; `MAILBOX_EMPTY` is 0. `MailboxNode { next: AtomicU32, payload: AtomicU32 }` is 8 bytes; `next` uses the same encoding with `MAILBOX_NIL` = 0. The field keeps its name (an index despite the suffix; rule L-3 reserves `_ptr` for it) so that no other document moves.
- **No tag.** The mailbox is only pushed and drained whole. A pusher loads the head, links its node to it and compare-exchanges; it never dereferences the head it loaded. The consumer swaps the head to empty and walks the list it took, which no pusher can reach again. ABA needs a participant that dereferences a value and later compares it against a recycled one; there is none, so a stale head that has become current again produces a valid list (the new node in front of the current one) and nothing else. `mailbox_tag` is removed; its bytes are `mailbox_reserved: u64`, MUST be zero. Image format version 4.
- **Gate.** `try_schedule` (idle → scheduled), `begin_turn` (scheduled → running, acquire), `end_turn` (store idle, then re-read the head and, if a message waits, `try_schedule` again and return `true`, which obliges the caller to enqueue the unit). A pusher pushes first and schedules second.
- **Orderings.** `begin_turn` is acquire on success and the idle store releases the turn's plain-field writes, as [ADR-0006](0006-virtual-actor-turn-invariant.md) said. Four operations are sequentially consistent: the pusher's head compare-exchange and gate compare-exchange, and the worker's idle store and head load in `end_turn`. With only acquire/release, a pusher whose gate compare-exchange fails on a running unit and a worker whose head load sees an empty mailbox could both happen, each having stored before the other loaded (the store-buffering case), and the message would wait for an unrelated push. Under a single total order of those four operations that outcome is impossible: the pusher's push precedes its failed gate operation, which precedes the worker's idle store, which precedes the worker's head load, so the head load sees the push. This is the one place in the workspace where acquire/release is not enough, and whitepaper §8.5 says so.
- **Drain order and determinism.** A drain yields the most recently pushed node first. Under concurrent pushers arrival order is a race, so the mailbox guarantees exactly-once delivery and not an order; the executor sorts a drained batch by its payload key in a bounded buffer of its own before integrating it, which is how §8.3's total order is met (Specified, milestone M2). The payload is opaque here.
- **Robustness.** A push refuses a node outside the arena, node `u32::MAX` (its `+ 1` does not fit) and a head above `u32::MAX`; a drain stops at the end of the list, at an index outside the arena and after `arena.len()` nodes, so a corrupt or cyclic list terminates it rather than the process.

### Consequences

- Good: a push is one compare-exchange, a drain is one swap, a turn is two compare-exchanges and one store; no lock, no allocation, no `unsafe`, no tag word.
- Good: an image at rest needs no fix-up: zero is idle and empty.
- Good: the lost-wakeup rule lives in `end_turn`; a caller cannot forget it, only ignore its return value, which the doc comment forbids.
- Bad: eight bytes of the record are now reserved rather than used; a future field has a home.
- Bad: reverse arrival order and no determinism from the mailbox alone; the executor pays for the sort, bounded by the pool size.
- Bad: four sequentially consistent operations per push-and-turn on x86-64 are `lock`-prefixed instructions the acquire/release forms would not need; measured by the new benchmarks, and the correctness argument is worth them.

## Alternatives considered and why rejected

- **Option 1**: a packed tag is atomic, but a stack drained whole does not need one, and a stack popped one node at a time would give the consumer a per-node compare-exchange instead of one swap.
- **Option 3**: arrival order from a queue is still the order of a race between workers, so it does not give §8.3 its total order either, and the enqueue side needs two atomics and a dummy node per unit, which is sixteen bytes the record does not have.
- **Keeping the tag field as documentation**: a field that claims to guard against a hazard that does not exist is finding F-19 restated.

## Confirmation

On the change that proposes this decision (2026-09-10): unit tests in `cortex-core` pin the gate transitions and their refusals, push-then-drain order and emptiness, the refusals, termination on corrupt and cyclic lists, and the lost-wakeup rule on a single-thread interleaving; `crates/cortex-core/tests/mailbox.rs` runs four producer threads against one consumer over a shared arena, 100 000 messages, each delivered exactly once, none left behind, every node returned; `benches/cortex-bench` measures `mailbox/push_drain_x16` and `gate/schedule_begin_end`; `npx spec-guard` asserts `try_schedule` and `MailboxNode` exist and that `FORMAT_VERSION` is 4.
