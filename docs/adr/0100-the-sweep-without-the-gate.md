---
status: accepted
date: 2026-09-26
depends-on: ADR-0099
decision-makers: VirtualCortex maintainers
---

# ADR-0100: The sweep without the gate, built and not kept — each worker owning a fixed range of the unit arena and serving it in unit order from a schedule bitmap, with no deque, no stealing and no compare-and-swap in the turn, held every reading of behaviour bit for bit and ran 2.0 to 2.7 times faster wherever most units are served, and 1.18 times slower at ADR-0097's sparsest drive; under ADR-0099's criterion, written before the first timed run, it is not kept; two diagnostics beside the criterion place the regression in the workload's census of every unit between ticks; the code stays in the history, and the next decision is named and not taken

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

What owns a unit, how does a worker find the units it must serve, what becomes of the deque, the stealing and the gate, what holds axiom A3 and phase 1's `unsafe` afterwards — and is the result worth keeping?

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

**Option 3, with contiguous ranges fixed at construction, was built and measured, and is not kept**: its wall time per tick at ADR-0097's run (c) read 1.178 of the base's against a bound of 1.10. The code is commit `3887e83` of this round's branch and is reverted in the same pull request; what the pull request keeps is this record, the readings in `docs/benchmarks/results/2026-09-26-dancr-win11.md`, and the tests that hold behaviour on the executor as it is.

### The design built (`3887e83`)

- **Ownership.** With $N$ units and $W$ workers, the share is $s = \lceil N / W \rceil$ and worker $w$ owns the units $[ws, \min((w+1)s, N))$; a worker past the arena owns none. The ranges partition the arena and are fixed in `Executor::new` (`partition`, `range_of`). `Executor::owner(unit)` names a unit's owner. Contiguous ranges keep a worker's records adjacent, which the working layout needs, and balance the count to within one share; interleaving would balance a clustered activity better and break both.
- **The schedule.** `Shared::scheduled` is one bit per unit in 64-bit words. Worker $w$'s bits are its own region of $\lceil s / 64 \rceil$ words, rounded up to eight (a cache line), so no two owners' words share a line. Unit $u$ of owner $o = \lfloor u/s \rfloor$ is bit $(u - os) \bmod 64$ of word $o \cdot \mathit{stride} + \lfloor (u - os)/64 \rfloor$.
- **Phase 1, the sweep.** Each worker takes its region's words in order and, in each, the set bits from the lowest: its units in increasing index. For each it runs the turn — drain the mailbox if it holds mail, sort, sum, scale by the gain, integrate, and on a spike the short-term step, the descendant count, the spiked list, the spike trace and the train's slot — and keeps the unit's bit for the next tick if the unit is not at rest. Then it stores the word back. No other worker writes the region in phase 1, because no push happens in it, so the word is loaded and stored, not exchanged. The turn takes no compare-and-swap and no fence for its schedule: the deque's push, pop and fence and the gate's three operations are gone, and a unit with no mail costs a load of its mailbox head instead of a swap. A message still costs what ADR-0017 and ADR-0023 made it cost: the push's compare-exchange, the drain's swap and its node's return to its pool.
- **Marking a unit.** A delivery in phase 2 or 3 pushes its message (ADR-0017's compare-exchange on the head, unchanged) and then marks the target; an activation from the injector in phase 3, and `wake_now` between ticks, mark without a message. To mark: if the unit's gate byte is not scheduled, store scheduled into it and set the unit's bit with a fetch-or. Two pushers may both find it idle; both then store the same byte and set the same bit, which is idempotent, so no claim is needed.
- **The gate byte** is kept as the record of the schedule: between ticks it is scheduled exactly when the unit's bit is set, so the next tick serves it, and idle otherwise; it is never running. The owner writes it at the end of the turn and a marker writes scheduled, through `cortex-core`'s `set_gate`, a relaxed store. What reads the byte reads what it read before: `is_image_ready`, ADR-0097's census, the differential test's snapshot. `try_schedule`, `begin_turn` and `end_turn` stay in `cortex-core` with their tests and their bench case; the executor no longer calls them.
- **Orderings.** Every access to the bitmap and to the gate byte is relaxed. The spin barrier's release and acquire order everything a phase wrote before everything the next phase reads, as they already order the plain fields (ADR-0023). Within phases 2 and 3 the only concurrent writes to a word are fetch-ors and to a byte are stores of one value; within phase 1 only a region's owner touches it. No lock-free structure is added.
- **Removed:** `deque.rs` and its four tests, `Config::deque_capacity`, `Worker::steal`, and the two exclusions of `.cargo/mutants.toml` that named `steal`. The sparse case keeps neither a list nor a sweep of the records: the bitmap is the list, eight words a worker a tick at (c).
- **Axiom A3, as it would have read:** "At most one worker writes a record's plain fields in any tick, enforced by ownership: each worker owns a fixed range of the unit arena and alone runs its units' turns; the barriers keep every other worker's access to the record — a push onto its mailbox, a mark of its gate byte, a read of its spike stamp — out of the phase in which the owner writes it." Held by `a_unit_is_served_by_its_owner_alone_and_a_woken_unit_at_the_next_tick` (every message drained by its unit's owner; a unit at rest is not served until a message from another worker's range marks it, and its owner serves it at the next tick), by `the_ranges_partition_the_arena_and_the_places_partition_the_schedule` (over a grid of twelve arenas and seven worker counts), and by the differential test on one, two and four workers.
- **Phase 1's `unsafe`, as it would have read:** "`unit` is a set bit of this worker's region of the schedule, and so in this worker's range, which no other worker's range overlaps: the ranges partition the arena and are fixed in `Executor::new`. In phase 1 a worker references only its own range's units, one at a time, and no push happens; phases 2 and 3 have ended at the barrier."
- **Held bit for bit on `3887e83`:** `PINNED_ARENA_HASH` `0x6c27858ece2dd412` with its spike count 95, masked and unmasked alike, so no pin was restated; the differential test on one, two and four workers; ADR-0097's four tables with their census of the scheduled gates on every tick; every test of the workspace in the debug and release profiles (635 passed and 57 ignored in release), under the MSRV, `clippy -D warnings`, `rustdoc -D warnings` and the bench's smoke run.

### The measure, fixed before the first timed run

ADR-0099's criterion and the protocol were written into this record before any run was timed (`0f7fee9`), and how the idle machine is checked was added before the session that is read (`5eba9be`):

- *Builds:* the base is `main` at the round's start (`391ad6d`), the change is this round's commit of the code (`3887e83`); each exported with `git archive` into a directory of its own and built with `cargo test -p cortex-runtime --release --locked --test active --no-run` into a target directory of its own.
- *Runs:* ADR-0097's four, `tests/active.rs`, each test run alone by its binary: `<binary> --ignored --exact <test> --nocapture --test-threads=1`. The reading is the `ns_per_tick` of the run's `DUMP network k` line; the control's line at (a) to (c) is read beside it and is not a criterion.
- *Pairs:* five pairs in one session on one machine with nothing else started, the base first in the first, third and fifth pairs and the change first in the second and fourth, the four runs in order (a) to (d) within each build's turn.
- *The idle machine, as checked:* the session starts only after the machine's total processor time has stayed below 15 per cent for thirty seconds with none of the round's processes running, and the total is sampled every five seconds through the session and reported beside the readings. Every run's whole output is kept. A session started before this sentence was written is discarded, not read: its parser anchored the DUMP lines at the start of a line, where libtest prints `test <name> ... ` before a run's first line, so it recorded the controls and none of the network readings the criterion reads; and another process tree (a mutation run of another repository, fifteen workers) held all sixteen logical processors at 100 per cent throughout it. Its control readings are reported with the round, not as a reading of the gain.
- *Criterion:* the median of each run's five readings per build; the change is **kept** if the median's ratio change/base is at most **0.80** at (a) and at (d) and at most **1.10** at (b) and at (c). Otherwise the pull request carries the readings, this ADR and the tests that hold behaviour, not the change. The figures are a developer machine's, recorded in `docs/benchmarks/results/` as `admissible: no`, and never written into §10.2. None of this moves after a timed run.

### The readings

The session ran from 20:11:14 to 20:24:03 UTC on 2026-09-25 (04:11 on 2026-09-26 in the machine's zone), on the developer machine of ADR-0097's bench (AMD Ryzen 7 7735HS, eight cores, sixteen threads, Windows 11, unpinned; not admissible). The processor total read 6 to 12 per cent over the thirty seconds before it; through it, 153 samples averaged 29 per cent (the two workers' spinning barrier alone is 12.5), 19 of them at or above 40 per cent and one at 94. Wall time per tick in nanoseconds, the five readings in pair order:

| Run | Base | Median | Change | Median | Change/base | Bound | |
| :--- | :--- | ---: | :--- | ---: | ---: | ---: | :--- |
| (a) 1 024 units, ADR-0044's drive | 19 565, 17 511, 16 880, 17 244, 17 582 | 17 511 | 6 647, 7 108, 7 345, 7 160, 6 324 | 7 108 | **0.406** | 0.80 | met |
| (b) sixteen times sparser | 11 329, 11 496, 11 135, 11 170, 12 197 | 11 329 | 4 024, 4 177, 4 338, 4 001, 4 202 | 4 177 | **0.369** | 1.10 | met |
| (c) 256 times sparser | 1 250, 2 287, 1 230, 1 202, 1 191 | 1 230 | 1 432, 1 492, 1 450, 1 449, 1 437 | 1 449 | **1.178** | 1.10 | **not met** |
| (d) 4 096 units, ADR-0044's drive | 75 449, 77 379, 79 894, 78 634, 75 322 | 77 379 | 38 813, 37 741, 38 330, 36 505, 37 545 | 37 741 | **0.488** | 0.80 | met |

The controls beside them (not a criterion): (a) 19 931 against 9 924, 0.498; (b) 14 181 against 7 511, 0.530; (c) 1 252 against 1 462, 1.168. Every run of both builds passed its pinned table. The change's turns per worker were its partition's: 335 456 545 and 335 450 928 at (a), 241 061 746 and 241 370 153 at (b), 24 852 223 and 24 612 022 at (c), 1 341 843 507 and 1 341 843 219 at (d).

The discarded session's controls, under the other tree's 100 per cent: (a) base 32 407 and 33 259, change 15 091 and 17 462; (b) base 17 460, change 28 495 and 27 264; (c) base 3 857, change 6 214 and 6 630.

### Two diagnostics beside the criterion

Read after the session and before the other process tree started again (20:34:34 UTC), five alternating pairs each, the load not sampled; they are readings for the next decision and change no verdict:

- **One worker.** The same four runs' (a) and (c), both trees with the executor at one worker: (a) 21 275 against 10 981 ns a tick, **0.516**; (c) 1 135 against 746, **0.657** (controls 0.595 and 0.649). On one worker the sweep is faster at the sparsest drive too, so the regression is not its own cost per tick.
- **Two workers without the census.** Run (c) with the harness's per-tick read of every unit's gate byte (`scheduled`, ADR-0097's census, which reads all 1 024 records between ticks on worker 0's thread) taken out: 1 012 against 757 ns a tick, **0.748** (control 1 083 against 777, 0.717). With the census the ratio is 1.178; without it, 0.748.

So the ratio the criterion read at (c) comes from the workload's census interacting with two owners, not from the sweep. The mechanism is not measured. The readings are consistent with this one: the census pulls every unit's line into worker 0's cache between ticks; under ownership worker 1 then takes back each of its served units' lines, scattered at (c) where no prefetch helps; under the deque, worker 0 serves most of (c)'s units itself, because the drive's messages are delivered, and so queued, by worker 0. The census is the round's instrument, not the engine's: no rule of the engine reads every record between ticks. But it is in the workload ADR-0099 fixed before the run, and the criterion reads that workload. The whitepaper records this as finding **F-50**, open until the ADR that takes the next decision says on which workload a lever's gain is read.

### The per-turn breakdown

What a turn costs on the sweep, for the working layout's ADR, from the runs and the bench of one developer machine (not admissible; `docs/benchmarks/results/2026-09-26-dancr-win11.md`):

- **On one worker**, where no barrier waits and no balance enters, the sweep's tick at (a) is 10 981 ns for 1 024 turns: about **10.7 ns a turn**, against the base's 20.8 (21 275 ns). The bench's `neuron/integrate`, one integration under a drive that fires the unit now and then, reads 13.45 ns on the same day's machine and 11.03 on 2026-09-25's. So on the sweep the integration is the turn, to the resolution of this machine; what the base paid besides it, about 10 ns, is the gate and the deque, as `gate/schedule_begin_end` (14.0 ns today, 9.79 on 2026-09-25) said it would be.
- **On two workers**, the runs' own figure of one worker's nanoseconds a turn is 13 at (a), 11 at (b) and 18 at (d), against the base's 34, 30 and 37: the barriers and the balance between two ranges are in it.
- **A message** costs `mailbox/push_drain_x16` / 16, 7.6 ns today (4.70 on 2026-09-25); at (a) the drive and the synapses bring about 8.6 messages a tick against 1 024 turns.

The working layout's measured need is therefore the integration itself: on one worker the sweep leaves nothing else in a turn at (a). The bench of the base in the same session is not read: the other process tree took the machine back part way through it (98 to 100 per cent). No bench case was added; the one-worker run is the sweep's turn in isolation.

### The verdict and what the pull request carries

- **Not kept.** Three of the four bounds are met with room (0.37 to 0.49 against 0.80 and 1.10), and (c) is not (1.178 against 1.10). Under ADR-0099 no constant moves and no second attempt is made in this round.
- **Reverted** in `4b2f694` (on the branch): `deque.rs`, `Config::deque_capacity`, the steal, the gate's use in the turn, the exclusions naming the steal; `set_gate`, `Executor::owner`, `WorkerReport::turns` and the ownership's tests go with the change. Axiom A3's enforcement, §4 and §8.5, and ADR-0023's and ADR-0017's decisions stand as they were.
- **Kept:** the determinism pin's assertion that it holds no scheduler state, and the differential test on one, two and four workers (`014db40`), both true on the executor as it is.
- **The mutation gate** has no source line of this pull request to mutate: its diff changes tests and documents only. The code of `3887e83` was not put through the gate, since it is not merged.
- **The weekly dispatch's scope** ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)): `scope=exhaustive`. Against `main` the diff changes no file under `src/` (the change and its revert net to nothing), deletes no test and takes none out of the swept suite (the two differential tests are renamed and assert more on the same binary), and leaves `.cargo/mutants.toml` as it was.

### The next decision (named, not taken)

ADR-0099: when a lever is not kept, "an ADR decides whether the next lever proceeds without it". That ADR chooses among:

1. **The working layout without the sweep.** Its premise was a contiguous range served in order, which only the sweep gives.
2. **The sweep again, in a later round, under a workload without the census in the timed ticks** (F-50): ADR-0097's reading held some other way (the counter alone, or the census taken outside the ticks that are timed), with the criterion written again before its first run. The two diagnostics say what that round should expect; they are not its reading.
3. **The sweep with a balance for the sparse case**, words of an idle worker's region served by another, which is a lock-free structure the tree does not have and needs its published algorithm.
4. **The lookahead next**, which needs ownership too.
5. **The speed line closed** and the learning line resumed from H-18's named next decision (ADR-0098).

### Consequences

- Good: the round read what it was built to read. The sweep removes the gate and the deque from a turn: one worker's turn at (a) goes from about 21 ns to about 11 ns, the integration's cost (see the breakdown). Every behaviour pin held with no pin restated. Its failure is placed, by two diagnostics, in the instrument rather than the engine.
- Good: the tree keeps ADR-0097's census and ADR-0099's criterion as written; the criterion did what ADR-0099 said it was for, which is to keep nothing on a hope.
- Bad: the engine is not faster. The lever with the most gain at the sizes the tree runs waits for the next decision.
- Bad: ADR-0099's workload carries an instrument whose cost depends on who serves a unit, which no one saw before the run.
- Neutral: the ownership, the bitmap and their tests are in the history at `3887e83` for the round that takes option 2, 3 or 4.

## Alternatives considered and why rejected

- **Option 1** needs a claim of the same cost as the one it removes.
- **Option 2** reads every record every tick and changes the served set for a unit written awake between ticks with no message.
- **Option 4** costs $W \times N$ entries.
- **Interleaved ranges** balance a clustered activity better and give up the contiguous range the working layout needs.
- **Dropping the gate byte** (always idle): the clock sweep would evict an activated unit at rest before its turn, and ADR-0097's census would lose its subject.
- **Keeping the change on the diagnostics** (0.748 at (c) without the census): the criterion reads ADR-0099's workload, written before the run; reading a different one after it is the move ADR-0099's "no constant moves after a timed run" exists to forbid.

## Confirmation

- `3887e83`: the design, with `a_unit_is_served_by_its_owner_alone_and_a_woken_unit_at_the_next_tick`, `the_ranges_partition_the_arena_and_the_places_partition_the_schedule` and `set_gate_records_each_state_without_a_claim`; every pin held on it.
- `014db40`, kept: `the_random_network_hashes_to_the_pinned_value_on_every_architecture` asserts its hash with the scheduler's bytes masked equals the pin; `the_ring_is_identical_on_one_two_and_four_workers_and_goes_round` and `a_random_network_with_stdp_is_bit_identical_on_one_two_and_four_workers`.
- `4b2f694`: the revert; against `391ad6d` the tree's code differs only in `tests/differential.rs`.
- `docs/benchmarks/results/2026-09-26-dancr-win11.md`: every reading above, the protocol's script and the bench.
- The evidence: the weekly dispatched on this round's branch, run `36188866157` at `df38fa8` with `scope=exhaustive` (the clause above), green in every job it runs: the four whole-domain shards took 44m42s, 50m04s, 1h19m35s and 58m44s, their tests' wall 2 631, 2 957, 4 722 and 3 471 s, 37, 41, 66 and 48 per cent of the 7 200-second bound, every pinned number of those tests reproduced. Beside them, as the secondary reading, ADR-0097's run `36050252444` on the same code (ADR-0098 and ADR-0099 changed documents only): 95m38s, 50m03s, 44m37s and 77m11s. The same tests moved by 0.55 to 1.99 times between the two runs on identical code, the runners' (F-45's kind): the longest, H-14's run, 2 475 s then and 3 097 now, and H-18's assignment arm 4 121 then and 2 284 now. The cost table is regenerated from this run, 57 lines, 26 945 seconds against the previous table's 29 300. No sweep was dispatched, so there is no survivor to disposition; Monday's schedule sweeps `main` as it always does.
