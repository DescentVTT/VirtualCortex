---
status: accepted
date: 2026-09-14
decision-makers: VirtualCortex maintainers
depends-on: ADR-0048
---

# ADR-0050: The spike train inside the executor — every worker's spikes of a tick merged after the tick in unit order into a bounded ring the executor owns, bit-identical on every worker count; the capture rules of ADR-0048 on the engine's own run; a rewarded search tagging the coincidence before its reward from that train in one call; the store, the affect state and the association in the image Specified with their format bump

## Context and Problem Statement

[ADR-0048](0048-episodes-tagged-from-the-train.md) tagged episodes from a spike train and left the capture offline: "a worker's spikes are its own thread's until it stops (`WorkerReport::spikes` exists at shutdown), so a capture inside the tick that is bit-identical on every worker count needs a merge across workers the executor does not have", and its exit test obtained the run's train from a fork of the start image under the same drive and cues. It named the requirement: "a per-tick merge of every worker's spikes in unit order, bit-identical on every worker count, and a cadence for the capture, taken by the round that also runs discoveries inside the loop".

Three facts of the executor shape the merge. Work stealing ([ADR-0023](0023-executor.md)) moves units between workers, so which worker runs a unit is not a function of the run: a merge that concatenated the workers' lists would differ by worker count, and the tick's spikes must be sorted by unit. A unit fires at most once per tick, so one slot per unit holds a tick's spikes whatever the workers do. The workers' traces (`Config::trace_capacity`) record delivered messages and spikes into the same capacity with one dropped-count, so at 4 096 units the drive's messages alone fill them and a fork's train is refused for a reason that has nothing to do with spikes (brief 024's 4 096-unit probe met it): a train of the engine's own needs a capacity of its own.

## Decision Drivers

- §8.3 and T-1: the train is part of the run, so it must be the same on every worker count and every architecture; a sort by unit within the tick makes it so by construction.
- TC-5: nothing allocates in the loop; the ring, the tick's slots and the merge buffer are sized once in `Executor::new`.
- ADR-0016's one-owner rule: the merge and the ring are the executor's; the rules over the train stay `cortex-hippocampus`'s; the composition with the ledger, the reward and the search stays the runtime's `episode` module.
- ADR-0043's precedent: the clause store, the affect state and the association are the caller's until the image holds them; a call between ticks that runs the search, rewards and tags is the loop's shape without an executor field.

## Considered Options

1. A per-worker list of the tick's spikes published at the end of the turns phase and concatenated by the coordinator.
2. **One shared slot per unit with an atomic cursor, appended by every worker at the spike; after the tick's last barrier the coordinator takes the tick's spikes, sorts them by unit and appends `(tick, unit)` to a ring of `Config::train_capacity`, letting the oldest go when it is full and counting them; `Executor::train` between ticks; `fork` returning that train; the capture rules and the discovery loop composed over it in `episode.rs`.**
3. A cadence inside `tick()` that runs the search over an executor-owned store on every ripple while awake and tags the rewarded moment.
4. A bitmap of the units that fired, scanned by the coordinator after the tick.

## Decision Outcome

Option 2; no record changes, format 13.

- **The executor** (`executor.rs`). `Config::train_capacity`: the spikes the ring keeps; 0 keeps none, allocates no slot and adds nothing to the tick (the workers test one length). `Shared::fired` is one `AtomicU32` per unit and `fired_len` its cursor; in `turn` a spike appends its unit at `fetch_add` (the slot exists, since the cursor stays below the unit count; a cursor past it aborts). After the deliveries' barrier, before the tally, `merge_spikes(now)` swaps the cursor to zero, copies the tick's units into a buffer sized once, sorts them and pushes `(now, unit)` onto a `VecDeque` sized once, popping the oldest when the ring is at capacity and counting it in `train_overwritten`. `Executor::train(&mut self) -> &[(u32, u32)]` makes the ring contiguous (a move of its entries, no allocation) and returns it: tick order, unit order within a tick, whichever worker ran the unit. `train_overwritten()` and `train_capacity()` read the counters. The cost per spike is one atomic increment and one store; per tick, one swap and a sort of the tick's spikes on the coordinator, both nothing when the capacity is zero.
- **The forks** (`branching.rs`). `fork` returns the executor's train through `train_of(exec)`: `NoTrain` when the configuration keeps none, `Dropped(n)` when the ring let `n` spikes go, else the train; `trace(exec)` stays for a caller that wants the workers' reports, and the determinism test holds the two equal on one and four workers.
- **The compositions** (`episode.rs`). The three functions of ADR-0048 over a caller's train stay. Over the executor's own: `tag_recent(exec, from, window, priority)`, `tag_burst_in(exec, from, to, window, priority)` (the densest span of `window` ticks among the spikes of `[from, to)`) and `tag_discovery_recent(exec, window, coincidence, discovery, priority)` (the reward's tick is the clock's). `discover(exec, ClauseSearch { store, len, scratch, affect, budget }, Tagging { window, coincidence, priority }, discoveries, associations) -> Result<DiscoverReport, DiscoverError>` is the loop in one call between ticks: the search of [ADR-0045](0045-clause-search.md), the committed rewards' total into the modulator when positive, and then the pattern active in the window before now (its densest coincidence) tagged once for the rewarded search, with one association per commit to that episode written in the commits' order; `OutFull` before the search when the association slots are fewer than the discovery slots; the search's and the ledger's refusals as they give them, the commits standing in the store either way and the reward the modulator's once given. The report carries the search's report, the dopamine signal after the reward (or as it stood) and the tagged episode's index with its span.
- **What the exit test holds** (`tests/reference.rs`). The determinism test: the executor's train equals the workers' sorted traces on one worker and on four, and the two worker counts give the same train (256 units, the driven run of ADR-0044). `capture_night` no longer forks the start image: the reward's moment is tagged by `discover` from the executor's own train (437 spikes to the reward's tick at 256 units, 1 370 at 1 024), the invention's association is the first of the search's two commits and the second commit's association names the next invented predicate and the same episode, the network's own pattern is tagged by `tag_burst_in` over the two bins before the experience, and a store of two facts commits nothing, tags nothing and leaves the signal as it stood; every other number of the night is as [ADR-0049](0049-dale-principle-in-plasticity.md) pins it. The executor's unit test holds the ring's rule: six spikes into a ring of four keep the last four in tick order with both units of a tick in unit order although the other was injected first, count two let go, a ring of six keeps them all, and a capacity of zero keeps none; `tests/no_alloc.rs` runs its loop with a train of sixty-four and allocates nothing. The 4 096-unit sweep of [ADR-0051](0051-the-estimator-at-4096-units.md) runs on this train.
- **What is Implemented and what is Specified.** Implemented: the online capture on the engine's own train, triggered between ticks by the caller (`tag_recent`, `tag_burst_in`, `tag_discovery_recent`, `discover`). Specified: the clause store, the affect state and the association in the image, three sections and format 14 with the loader's checks, taken by the round that gives the image a term arena; after it a cadence inside `tick()` can run the search on a rewarded moment without a caller, which is option 3, and the association moves out of the caller's table into the image with the store it refers to.
- **Not adopted.** *Option 1*: not worker-count independent under stealing without the same sort, and a list per worker is the unit count times the workers. *Option 3*: the store is the caller's arena; a loop inside the tick over a caller's arena is a reference the phases cannot hold, and the reward that triggers it comes from the search, which is the round of the image sections. *Option 4*: a scan of every unit per tick, which is the population's size, not the tick's spikes'.

### Consequences

- Good: a capture is the engine's own, from its own run, without a second executor; the reference tests' forks read the same train the engine keeps, and the 4 096-unit sweep runs where the workers' traces could not.
- Good: the train's order is the run's on every worker count by construction (a sort), held by the determinism test, not by review.
- Good: the loop from a search to a tagged episode is one call, so a host that runs discoveries between ticks has the shape the image round will move inside.
- Bad: the merge is a sort per tick on the coordinator; at a population whose spikes per tick are many thousands it is a cost to measure under `docs/benchmarks/README.md` before the cadence moves inside the tick.
- Bad: the ring holds `(tick, unit)` as `u32` pairs; a train longer than the clock's width wraps as the stamps do (§8.4), and a caller that reads across the wrap reads what the forks read.

## Alternatives considered and why rejected

- **The ring as a slice a caller reads without `&mut`** (`as_slices` two halves): the capture rules take one sorted slice; the contiguity is one rotation between ticks and no allocation.
- **`trace_capacity` widened instead**: the workers' traces record deliveries and spikes together, per worker, at a memory cost of the workers times the capacity, and the drive's messages fill them at 4 096 units; the train is the spikes alone, once.
- **The reward-triggered tag inside `Executor::reward`**: every reward would tag, the modulation tests' rewards included, and a ledger without room would turn a reward into an error.

## Confirmation

`executor.rs`: the ring's unit test (order, capacity, overwriting, none). `branching.rs`: the fork's refusals (`NoTrain`, a quiet run's empty train). `episode.rs`: the recent forms' refusals on an empty train, the span's rule, `discover`'s `OutFull` before the search and a search over nothing tagging nothing. `tests/reference.rs`: the determinism test's train equalities and `capture_night` through `discover`. `tests/no_alloc.rs` with a train. The mutation gate on the changed lines (ADR-0030) passes in CI.
