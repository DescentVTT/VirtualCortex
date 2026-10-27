---
status: accepted
date: 2026-10-28
amends: [ADR-0098, ADR-0105]
title: A unit served on arrival
kind: decision
summary: The executor stops serving a unit that provably cannot fire before its next message, and brings it up to date bit for bit when a message reaches it or its state is read. Per-tick service stays the model and every pin holds. Brief 084 builds it. F-86 opens; F-84 is resolved.
decision-makers: Menteion maintainers
---

# ADR-0224: A unit served on arrival

**In brief**
- **Decision.** Per-tick service stays the model ([ADR-0098](0098-the-integration-model.md)), and the executor carries it out on arrival. A unit leaves the schedule when its state proves it cannot fire with no input. When a message reaches it or its state is read, the quiet ticks' exact catch-up brings it to the present. Brief 084 builds it, held to every pin.
- **Why.** Exploration 008 found the condition and the catch-up, and every pin of a heavy arm held bit for bit on both architectures. The arm took 0.581 of its time on x86-64 and 0.771 on AArch64. The weekly job's shards run on x86-64.
- **Also.** F-86 opens: ADR-0037's reason that nothing showed weight saturation no longer holds. F-84 is resolved: §5.2.2's cell is split and names format 26.

## Context and Problem Statement

**The active set.**
- [ADR-0097](0097-the-active-set-measured.md): one message of 0.125 at the gain 1.75 keeps a unit awake 2 503 ticks, 1 023 of them a one-LSB tail. Under ADR-0044's drive the executor serves 99.99 % of the units every tick.
- ADR-0098 kept per-tick service. Its option 3, service on arrival with a faster catch-up iterating the membrane as written in fewer steps than ticks, had no candidate. Its drivers named what service on arrival needs to pay: a catch-up cheaper than Δ integrations.
- [ADR-0105](0105-the-speed-line-closed.md) closed the speed line and named what was left.
- [Exploration 006](../explorations/006-where-an-arm-spends-its-time.md) found the units' turns at 84 % of a heavy arm's test thread on x86-64 and 65 % on AArch64, `integrate` alone at 45 % and 38 %.

**[Exploration 008](../explorations/008-a-unit-served-on-arrival.md)**, exploratory, on the tag `explore/008-a-unit-served-on-arrival` at `e547c45`:
- **The condition**, `quiet_cannot_fire`: a unit cannot fire with no input while its soma, and a weighted mean of its compartments' positive parts, stay under the lower of its threshold and the base. Of `testkit/prop.rs`'s lattice, 11 628 states are admitted and none fired run to rest; of the 8 145 refused, 4 862 fire.
- **Two catch-ups**: `integrate(0, 0, t)` once a skipped tick, and `quiet_catch_up`, the threshold by segments, the windows by subtraction and the potentials stepped alone. The second was held to the first field by field at 23 gaps over the lattice and on a driven walk.
- **Every pin held** on both architectures under both: H-45's control arm, its derivation and its 120 blocks, and every unit's and block's bytes at each block's end. The turns served and the steps caught up together were per-tick service's exactly, 128 845 414 112.
- **The mailbox head** was read as holding mail or not: it is a pool index that two runs of `main`'s own engine on two workers need not share.
- **On the developer machine** 819 of 820 tests passed under each catch-up. The one left is ADR-0097's test of the turn count: it reads per-tick service's active set tick by tick, which service on arrival does not serve.

**The time**, the whole arm once, each beside per-tick service on the same runner (exploration 008):

| Service | x86-64, EPYC 7763 | AArch64, Neoverse-N2 |
| :--- | ---: | ---: |
| Per-tick | 1 073.4 s | 1 074.6 s |
| On arrival, `integrate` per tick | 0.907 | 0.910 |
| On arrival, `quiet_catch_up` | 0.581 | 0.771 |

The 16-block alternation, three rounds on one runner, read 0.583 and 0.773. A woken unit had slept 117.8 ticks on average, so the gain is cheaper steps, not fewer.

**What a round must settle**, from the note:
- the arena brought up to date through `Executor::units`, which takes `&self` under [ADR-0023](0023-executor.md)'s invariant, and through `Executor::sweep`, which reads the unit arena directly;
- what `Executor::turns` counts, and which count holds the steps caught up at a read;
- a reward's write of the critic's weights through `units_mut`, which puts every asleep unit back on the schedule.

## Decision Drivers

- **Quality goal 1**: the same seed and inputs give the same bytes on both architectures. A change to how the work is scheduled is admissible only held to every pin.
- **The maintainers' preference** (ADR-0098): per-tick service, so that every membrane dynamic stays admissible. The catch-up is the quiet tick's own rule, so the semantics do not move.
- **[ADR-0078](0078-the-rewards-direction.md)**: a lever waits for a measured need. The weekly job's tests cost 229 444 s, 95.6 % of it `inhibition`'s. Its longest shard read 59.8 % of its 240-minute bound on 2026-10-26, and each round of eighteen arms adds about 12 minutes a shard.
- **[ADR-0010](0010-measured-or-target.md)**: the figures are hosted runners', exploratory, not Measured.
- **Principle 5**: the catch-up's equality with the quiet ticks is held by property tests over the lattice, not by review.

## Considered Options

1. **Service on arrival with `quiet_catch_up`**, the executor's only service, per-tick service kept as the tests' reference.
2. **The same with `integrate` once a skipped tick** as the catch-up.
3. **Either, behind a mode** off by default.
4. **Per-tick service as built**, ADR-0098 and ADR-0105 as they stand.

## Decision Outcome

**Option 1.**

### The service (Specified; brief 084 builds it)

- A unit not at rest leaves the schedule at the end of its turn when `quiet_cannot_fire` admits it. Its gate is recorded as per-tick service records it.
- A message or an activation puts it back on the schedule. At the start of its next turn `quiet_catch_up` brings it from the tick it left to the present, and the turn is then the turn it was.
- Before the arena is read between ticks, every asleep unit is brought up to date; `Executor::sweep` is among the reads. Before it is written, every asleep unit is brought up to date and put back on the schedule.
- A unit marked `FLAG_SLOW` under the image's slow current never sleeps.
- No rule, record, constant or image byte changes. The format stays 26.

### The reference, and what is held

- **Per-tick service stays in the executor** as a construction option for the tests, in neither `Config` nor the image. A differential test holds the two services to each other at 256 units under ADR-0044's drive and under ADR-0097's drive 256 times sparser: after every tick for 2 048 ticks, then every 4 999 ticks over 32 768, so that gaps longer than a tick are caught up.
- **The condition**: no admitted state of the lattice fires with no input, and the soma never rises above its bound.
- **The catch-up**: `quiet_catch_up` over a gap equals that many calls of `integrate(0, 0, t)`, field by field and in turns, over the lattice and a driven walk.
- **Every whole-domain test**, by one dispatch at `scope=exhaustive` with every test on the round's last code commit.
- **The count**: `Executor::turns` counts the turns served, as each worker's count does. A second count holds the steps caught up, at a turn and at a read. Over a run, once a read has brought every asleep unit up to date, the two together equal per-tick service's turns, which the differential test holds.
- **ADR-0097's tests** read per-tick service's active set tick by tick. They run on the per-tick reference, their assertions unchanged.

### The arena between ticks

The round brings asleep units up to date under `&mut self`, `Executor::units` taking it, or under ADR-0023's invariant restated for the write, with its test, in its ADR. It adds no other `unsafe`.

### The time

- The round reads, on AArch64, the 16-block workload of the Context and the reference network at 1 024 units under ADR-0097's drive 256 times sparser. The two services alternate on one runner, three rounds, inside one test, since an arms job runs one test; each run builds its own executor and network.
- It reads x86-64 from the cost table its dispatch regenerates, beside the table before it, test by test: an indication only, since the shards' processors differ and each runs two tests at once.
- Each on-arrival run is held to the per-tick run of its round: every unit's and block's bytes at its end, the mailbox head read as holding mail or not, the messages sent and delivered, the spikes, and its turns and steps together against the per-tick run's turns. The 16-block workload is also held to its pins. A sparse run lasts at least a minute.
- The service is kept when every pin and every such comparison holds, on-arrival service reads below per-tick service in every round under the reference drive, and under the sparse drive it reads no slower than the spread of per-tick service's own three runs. The readings are exploratory, not Measured.

### F-86 and F-84

- **F-86 opens.** [ADR-0037](0037-sleep-regulation.md) gave, among its reasons not to adopt synaptic downscaling, that nothing in the tree showed weight saturation under a rule whose depression exceeds its potentiation. Explorations 005, 007 and 009 read excitatory synapses of the couplings at the rail, up to 186 of a coupling's 800 by the sixteenth schedule. ADR-0037's decision stands on its other reasons. M12's decision resolves F-86.
- **F-84 is resolved.** §5.2.2's Public API cell is split under the limit of [ADR-0197](0197-one-place-for-each-kind-of-fact.md), names format 26, and a directive holds the number.

### The sweep

The schedule's next run, 2026-10-28 at 20:17Z, has not begun when this is written. The first ADR that merges after it ends records it.

## Consequences

- **Good, expected from the Context's reading**: the weekly job's tests and every round's runs take less time, most on x86-64, where the weekly shards run. The semantics, every pin and the image do not move.
- **Bad**: the executor carries a second service, the reference, and every change to the turn is tested on both. ADR-0097's tests run on the reference, and its budget is read from the two counts together.
- **Bad**: [ADR-0101](0101-the-sweep-measured-again.md)'s timed runs in `active.rs`, with ADR-0097's other tests, time the per-tick reference, no longer the engine as built. The engine's speed is then read from the alternation and the weekly job.
- **Bad**: AArch64 gains less, 0.771. Exploration 008 names where its time goes as a question.
- **Neutral**: the cost table is regenerated by the round's dispatch.

## Amended

- **[ADR-0098](0098-the-integration-model.md)**: per-tick service is the model, carried out on arrival. A catch-up cheaper than Δ integrations, the need its drivers named, exists: as many steps as ticks, each cheaper. Option 3 as written, fewer steps than ticks, still has no candidate.
- **[ADR-0105](0105-the-speed-line-closed.md)**: of the speed line, service on arrival is taken. What else it named stays as named.

## Alternatives considered and why rejected

- **Option 2**: 0.907 and 0.910 in the Context's table, most of the gain left behind for a simpler catch-up.
- **Option 3**: a mode off by default speeds no run that does not set it, and doubles what each run's evidence must name.
- **Option 4**: the Context's reading held every pin and ran the arm faster on both architectures.
- **A closed form of the soma, or a jump to rest**: the Context found neither. A jump would pay only under a drive sparse enough that a gap outlasts 2 503 ticks.

## Confirmation

- Brief 084's built ADR names the property tests, the differential test, the dispatch with every test green on its last code commit, and the alternation's three rounds.
- `npm run spec` runs F-84's new directive on `FORMAT_VERSION` and brief 084's precondition on the condition's absence from `main`.
