---
status: proposed
date: 2026-09-26
depends-on: ADR-0099
amends: [ADR-0023, ADR-0017]
decision-makers: VirtualCortex maintainers
---

# ADR-0100: The sweep without the gate — each worker owns a fixed, contiguous range of the unit arena and serves, in unit order, the units of it that a schedule bitmap names, with no deque, no stealing and no compare-and-swap in the turn; a unit is marked on the bitmap by its owner when its turn leaves it awake and by a pusher when its message finds the unit idle; the gate byte is kept as the record of that schedule and no longer claims a turn; axiom A3 is enforced by the ownership; every reading of behaviour held bit for bit, and the gain read against ADR-0099's criterion

## Context and Problem Statement

[ADR-0099](0099-the-engines-speed.md) took three levers on the engine's speed that change no rule and ordered them. This is the first: a sweep without the gate. [ADR-0097](0097-the-active-set-measured.md) measured that under [ADR-0044](0044-reference-network.md)'s drive the executor serves 99.99 per cent of the reference network's units on every tick. Each of those turns still goes through the scheduling of [ADR-0023](0023-executor.md) and [ADR-0017](0017-mailbox-and-gate-protocol.md), read on 2026-09-26 in `runtime/cortex-runtime/src/executor.rs`:

- a pop from the worker's Chase–Lev deque (`Local::pop`, a sequentially consistent fence), or a steal from another worker's when it is empty;
- `begin_turn`, a compare-and-swap from scheduled to running;
- `end_turn`, a sequentially consistent store of idle and a sequentially consistent load of the mailbox head;
- when the unit is not at rest, `try_schedule`, a compare-and-swap from idle to scheduled, then a push onto the worker's next-tick list, and at the end of phase 3 a push onto its deque;
- `mailbox_drain`, a swap of the mailbox head, on every turn, mail or none.

A developer machine reads the gate's three operations at 9.79 ns and the integration at 11.03 ns of a turn of about 25 ns (`docs/benchmarks/results/2026-09-25-dancr-win11.md`, not admissible).

The facts the design rests on, read in the tree on the same day:

- **Phase 1 has no push.** Messages reach a mailbox only in phase 2 (a zero-delay synapse) and phase 3 (the wheel, the injector, the ripple), and between ticks only the coordinator schedules a unit (`wake_now`, the loader's). The barriers separate the phases (ADR-0023). So in phase 1 nothing races a turn, and the lost-wakeup rule of `end_turn` is, as ADR-0023 already said, "unreachable".
- **Nothing in a turn reads another unit.** A turn reads its own record, its own mailbox, the tick's gain and the clock. The batch is sorted by message value before it is summed (§8.3). The coordinator sorts the tick's spikes before the train (ADR-0050) and sums the workers' counts. So the order in which units are served changes no reading of behaviour; it changes only which node of which pool a later message takes, and the order of a worker's own traces.
- **The gate byte is read outside the turn.** `is_image_ready` (the clock sweep's eviction test and the image writer's refusal) reads it as idle; `tests/active.rs` holds every tick's turns to the units whose gate the tick before left scheduled (ADR-0097); `tests/differential.rs` compares it across worker counts. A unit activated at rest has a scheduled gate and an empty mailbox; the sweep must not evict it before its turn.
- **Which bytes the pins hold.** Every pin over a unit's or an image's bytes, found before the change:

  | Pin | Bytes it covers | The scheduler's own |
  | :--- | :--- | :--- |
  | `PINNED_ARENA_HASH` (`tests/differential.rs`) | every unit's 64 bytes as `encode` writes them, atomics as plain values; every block's 64; the spike train | yes, the mailbox head `[8..16)` and the gate byte `[56]` |
  | `REVERSAL_IMAGE_CRC_1024`, `REVERSAL_IMAGE_CRC_FORMAT_15_1024`, `PUNISHED_IMAGE_CRC_1024` (`tests/inhibition.rs`) | a whole image | no: `Image::encode` writes every gate byte as idle and refuses a unit whose mailbox holds a message |
  | a trial fork's hash (`trial.rs`, compared between the two forks and stored in the amendment record) | every unit's bytes with the gate byte written idle, blocks, spikes | no: the fork must be quiescent, so every mailbox is empty |
  | the equalities across runs (`tests/differential.rs` on one, two and four workers; `tests/criticality.rs` on one and four; `tests/image.rs` and `tests/reference.rs` between an engine and its image loaded) | every unit's bytes, raw | yes, but each is an equality between two runs of one executor, not a number |

  Read on the executor before the change, `PINNED_ARENA_HASH` with the mailbox heads and the gate bytes masked to zero is `0x6c27858ece2dd412`, the pin itself: after 20 000 ticks every gate is idle and every mailbox empty, so the pin holds the dynamics alone. The test now asserts that it still does.

What owns a unit, how does a worker find the units it must serve, what becomes of the deque, the stealing and the gate, and what holds axiom A3 and phase 1's `unsafe` afterwards?

## Decision Drivers

- **No reading of behaviour moves** (ADR-0099): every potential, window, stamp, threshold, short-term factor, weight and trace; the spike train and count; the messages delivered and the turns served, summed over the workers; every pin; the differential test.
- **The gain is read** under ADR-0099's criterion, written before the first timed run: at most 0.80 of the base's wall time per tick at ADR-0097's runs (a) and (d), at most 1.10 at (b) and (c).
- **The sparse case.** At (c) the executor serves about 7.4 per cent of the units a tick. A sweep that reads every record would read about thirteen times more records than it serves.
- **The next levers.** The working layout for the vector units wants a contiguous range served in order; the lookahead wants units owned by workers ([ADR-0035](0035-cadence-and-the-population-tally.md), ADR-0099).
- **Latest ≠ Newest** (whitepaper §2.1): no new lock-free structure unless it has a published algorithm and a memory-model argument; the round should need none.
- A structural boundary beats a reviewed one: A3's new enforcement is named with the test that holds it.

## Considered Options

1. **Keep the deque and stealing; drop the gate.** Without the gate's claim a unit can be queued twice, by two pushers or by a pusher and its own previous turn, and two workers can run it; the deque's capacity argument ("a unit is queued at most once at a time, by its gate") fails with it. Not possible without replacing the claim by something of the same cost.
2. **A full sweep.** Each worker owns a range and, every tick, reads every record of it and serves those that are not at rest or hold mail. No shared structure; but every unit's record is read every tick, awake or not, and an unwired unit that a test wrote above its threshold between ticks, which is served today only when a message or an activation wakes it, would be served at once: the served set would change.
3. **Ownership and a schedule bitmap.** Each worker owns a fixed range; one bit per unit names the units the next tick serves; the owner scans its words and serves the set bits in unit order; the owner marks a unit whose turn leaves it awake, a pusher or an activation marks one it finds idle.
4. **Ownership and per-worker lists.** A pusher appends a unit it wakes to a list per (pusher, owner). The lists are $W^2$ and each must hold the owner's whole range: $W \times N$ entries at 64 workers, 11 GB at Appendix A's count.

For the partition: contiguous ranges, or units interleaved by index or by word across the workers.

## Decision Outcome

**Option 3, with contiguous ranges fixed at construction.**

- **Ownership.** With $N$ units and $W$ workers, the share is $s = \lceil N / W \rceil$ and worker $w$ owns the units $[ws, \min((w+1)s, N))$; a worker past the arena owns none. The ranges partition the arena and are fixed in `Executor::new`. `Executor::owner(unit)` names a unit's owner. Contiguous ranges keep a worker's records adjacent, which the working layout needs, and balance the count to within one share; interleaving would balance a clustered activity better and break both.
- **The schedule.** `Shared::scheduled` is one bit per unit in 64-bit words. Worker $w$'s bits are its own region of $\lceil s / 64 \rceil$ words, rounded up to eight (a cache line), so no two owners' words share a line. Unit $u$ of owner $o = \lfloor u/s \rfloor$ is bit $(u - os) \bmod 64$ of word $o \cdot \mathit{stride} + \lfloor (u - os)/64 \rfloor$.
- **Phase 1, the sweep.** Each worker takes its region's words in order and, in each, the set bits from the lowest: its units in increasing index. For each it runs the turn — drain the mailbox if it holds mail, sort, sum, scale by the gain, integrate, and on a spike the short-term step, the descendant count, the spiked list, the spike trace and the train's slot — and keeps the unit's bit for the next tick if the unit is not at rest. Then it stores the word back. No other worker writes the region in phase 1, because no push happens in it, so the word is loaded and stored, not exchanged. The turn takes no compare-and-swap and no fence for its schedule: the deque's push, pop and fence and the gate's three operations are gone, and a unit with no mail costs a load of its mailbox head instead of a swap. A message still costs what ADR-0017 and ADR-0023 made it cost: the push's compare-exchange, the drain's swap and its node's return to its pool.
- **Marking a unit.** A delivery in phase 2 or 3 pushes its message (ADR-0017's compare-exchange on the head, unchanged) and then marks the target; an activation from the injector in phase 3, and `wake_now` between ticks, mark without a message. To mark: if the unit's gate byte is not scheduled, store scheduled into it and set the unit's bit with a fetch-or. Two pushers may both find it idle; both then store the same byte and set the same bit, which is idempotent, so no claim is needed.
- **The gate byte.** It is kept, as the record of the schedule: between ticks it is scheduled exactly when the unit's bit is set, so the next tick serves it, and idle otherwise; it is never running. The owner writes it at the end of the turn (scheduled if the unit stays awake, idle if it rests) and a marker writes scheduled. `cortex-core` gains `set_gate`, a relaxed store. What reads the byte reads what it read before: `is_image_ready`, so the clock sweep still never evicts a unit with a turn to come and the image writer still writes the byte idle; ADR-0097's census; the differential test's snapshot. `try_schedule`, `begin_turn` and `end_turn`, ADR-0017's protocol for a scheduler without ownership, stay in `cortex-core` with their tests and their bench case (`gate/schedule_begin_end`, the cost this removes from the turn); the executor no longer calls them.
- **Orderings.** Every access to the bitmap and to the gate byte is relaxed. The spin barrier's release and acquire order everything a phase wrote before everything the next phase reads, as they already order the plain fields (ADR-0023). Within phases 2 and 3 the only concurrent writes to a word are fetch-ors and to a byte are stores of one value, so no interleaving leaves a marked unit unset. Within phase 1 only a region's owner touches it. No lock-free structure is added.
- **The deque and stealing** are removed (`deque.rs`, `Config::deque_capacity`, `Worker::steal`, and the two exclusions of `.cargo/mutants.toml` that named `steal`). The sparse case keeps neither a list nor a sweep of the records: the bitmap is the list, $1/64$ of a word per unit per tick to scan and no read of a record the schedule does not name. At (c) at 1 024 units on two workers that is eight words a worker a tick.
- **Load balance.** A worker's turns are its own units', so a clustered activity is served by the workers whose ranges hold it and nobody steals. At (a) and (d) nearly every unit is served every tick and the shares are equal to within one unit. At (c) the drive is uniform over the units, so the shares differ by the drive's own variance. The task of the learning line (two and four workers, 1 024 units) puts its stimulus and readout sets where its seed draws them. The fan-out of phase 2 was always the spiking unit's runner's; it is now the owner's. The per-worker turns are read beside the runs (`WorkerReport::turns`), and the weekly shards' times beside the base's are the secondary reading. A range rebalanced per window is left to the lookahead's round, which needs a partition anyway.
- **Axiom A3, restated.** "At most one worker writes a record's plain fields in any tick, enforced by ownership: each worker owns a fixed range of the unit arena and alone runs its units' turns; the barriers keep every other worker's access to the record — a push onto its mailbox, a mark of its gate byte, a read of its spike stamp — out of the phase in which the owner writes it." The gate is no longer the enforcement. It is held by `a_unit_is_served_by_its_owner_alone_and_a_woken_unit_at_the_next_tick` (each worker drains only its own units' messages, and a unit woken across ranges is served by its owner at the next tick), by the unit test that the ranges and the bitmap's places partition the arena and its words, and by the differential test on one, two and four workers, where a turn run twice or on two workers would break bit-identity.
- **Phase 1's `unsafe`, restated.** "`unit` is a set bit of this worker's region, and so in this worker's range, which no other worker's range overlaps (the ranges partition the arena, fixed in `Executor::new`); in phase 1 a worker references only its own range's units, one at a time, and no push happens; phases 2 and 3 have ended at the barrier." Held by the same tests.
- **The pools** are unchanged. The worker that delivers a message is now the owner of the unit that spiked (phase 2, and phase 3 from its own wheel), or worker 0 (the injector, the ripple), so on one worker, and on any worker count up to the order in which several workers return nodes to one pool, which node a message takes is a function of the run.
- **Acceptance**, ADR-0099's, unchanged: every reading of behaviour bit for bit; a pin holding the scheduler's bytes restated only under the masked check; any other pin that moves stops the round.
- **The measure of the gain**, ADR-0099's, fixed here before the first timed run:
  - *Builds:* the base is `main` at the round's start (`391ad6d`), the change is this round's commit of the code; each exported with `git archive` into a directory of its own and built with `cargo test -p cortex-runtime --release --locked --test active --no-run` into a target directory of its own.
  - *Runs:* ADR-0097's four, `tests/active.rs`, each test run alone by its binary: `<binary> --ignored --exact <test> --nocapture --test-threads=1`. The reading is the `ns_per_tick` of the run's `DUMP network k` line; the control's line at (a) to (c) is read beside it and is not a criterion.
  - *Pairs:* five pairs in one session on one machine with nothing else started, the base first in the first, third and fifth pairs and the change first in the second and fourth, the four runs in order (a) to (d) within each build's turn.
  - *The idle machine, as checked:* the session starts only after the machine's total processor time has stayed below 15 per cent for thirty seconds with none of the round's processes running, and the total is sampled every five seconds through the session and reported beside the readings. Every run's whole output is kept. A session started before this sentence was written is discarded, not read: its parser anchored the DUMP lines at the start of a line, where libtest prints `test <name> ... ` before a run's first line, so it recorded the controls and none of the network readings the criterion reads; and another process tree (a mutation run of another repository, fifteen workers) held all sixteen logical processors at 100 per cent throughout it. Its control readings are reported with the round, not as a reading of the gain.
  - *Criterion:* the median of each run's five readings per build; the change is **kept** if the median's ratio change/base is at most **0.80** at (a) and at (d) and at most **1.10** at (b) and at (c). Otherwise the pull request carries the readings, this ADR and the tests that hold behaviour, not the change. The figures are a developer machine's, recorded in `docs/benchmarks/results/` as `admissible: no`, and never written into §10.2. None of this moves after a timed run.
- **The scope of the weekly dispatch** ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)): `scope=both`, by three of its clauses — the diff changes files under `src/`, deletes tests (`deque.rs`'s), and changes `.cargo/mutants.toml`.

### Consequences

- Good: a turn takes no compare-and-swap, no fence and no queue operation of its own; a unit with no mail takes no swap.
- Good: the order in which a worker serves its units is the units' order, so its spiked list, its fan-out and its node allocations are a function of the run; on one worker every byte of the arena is.
- Good: A3 is enforced by a partition fixed at construction, which a test can check over its whole domain, instead of by a protocol whose correctness argument needed four sequentially consistent operations.
- Good: the lookahead's precondition, units owned by workers, now holds.
- Bad: nobody steals. A worker whose range holds more of the tick's work is the tick's critical path; the barrier waits for it.
- Bad: a unit's mark costs a pusher a fetch-or on a word other pushers may be marking, where `try_schedule` was a compare-and-swap on the unit's own line, which the push had already taken.
- Neutral: the gate byte keeps its bytes and its values between ticks; `GateState::Running` is no longer written by the executor.
- Neutral: the per-worker shares of `tests/contention.rs` are now a function of the partition, not of timing; they are still reported, not asserted.

## Alternatives considered and why rejected

- **Option 1** needs a claim of the same cost as the one it removes.
- **Option 2** reads every record every tick and changes the served set for a unit written awake between ticks with no message.
- **Option 4** costs $W \times N$ entries.
- **Interleaved ranges** balance a clustered activity better and give up the contiguous range the working layout needs.
- **Dropping the gate byte** (always idle): the image writer and the trial's hash already write it idle, but the clock sweep would evict an activated unit at rest before its turn, and ADR-0097's census would lose its subject.
- **A rebalancing per window**: a partition that moves is the lookahead's question, with the lookahead's constraints; not before a measured imbalance asks for it.

## Confirmation

To be completed with the round's readings: the pins; the tests (`a_unit_is_served_by_its_owner_alone_and_a_woken_unit_at_the_next_tick`, the partition's unit test, the differential test on one, two and four workers, ADR-0097's tables, `tests/no_alloc.rs`); the five pairs' medians and ratios against the criterion, and whether the change was kept; the per-turn breakdown after the change; the in-diff mutation gate; the dispatched weekly run, its shards beside the base's and the cost table regenerated from it.
