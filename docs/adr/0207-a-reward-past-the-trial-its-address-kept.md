---
status: proposed
date: 2026-10-25
title: A reward past the trial, its address kept
kind: decision
hypothesis: H-43
summary: M10 taken. The engine keeps each trial's drawn address and critic counts until the reward that names the trial's age arrives, and consolidates under them. H-43 asks it at one and two trials' delay on seeds 22, 23 and 24 under the default, predicted yes. F-77, F-78 and brief 079's sweep are recorded.
depends-on: ADR-0206
decision-makers: VirtualCortex maintainers
---

# ADR-0207: A reward past the trial, its address kept

**In brief**
- **Decision.** With `Config::kept_addresses` set, each trial's close keeps its drawn address, the sources its critic counted and the targets it selected. A reward names the age of the trial it is for, and the engine consolidates under that trial's address and counts. H-43 asks it at one and two trials' delay.
- **Why.** Exploration 001 read a delayed reward fail under the default, with the rewarded pair's trace still 56 to 73 % there, because the address named the delivering trial's pair, not the rewarded one.
- **Left open.** A delay the engine is not told, which a trace of addresses would have to cross. The reward right three times in four, asked again on the default, comes after this round.

## Context and Problem Statement

[ADR-0187](0187-the-upgrade-suite-planned.md) planned M10 first: *"Does the configuration learn when the reward arrives trials after the choice?"*.
Its candidate was a critic over several timescales (UPG-03).
[ADR-0205](0205-the-probe-in-the-engines-default.md) and STATUS put it next once the default was built, and [ADR-0206](0206-the-probe-in-the-engines-default-built.md) built it.

**What a reward reaches today** (`runtime/cortex-runtime/src/executor.rs`, `task.rs`):
- A trial is 16 384 ticks. Between its last tick and the next trial's first, the task draws the address (`address_drawn`) and gives the reward (`reward`).
- `address_drawn` takes its sources from the critic's counts since the previous reward, and its targets from the trial's selection.
- `reward` reads the value from those counts, moves the value weights by the error times the counts, zeroes every count and reopens the critic's window.
- The reward is consolidated during the next trial, at presynaptic spikes ([ADR-0090](0090-the-old-answer-held-from-the-trial-after-the-flip.md)).
- The eligibility trace decays with τ = 65 536 ticks, four trials, and keeps 0.778 of itself over one trial and 0.605 over two.
- Neither the address nor the counts are in the image; a learning run does not resume from one ([ADR-0059](0059-a-task-a-readout-and-a-reward.md)).

**What exploration 001 read** ([its note](../explorations/001-the-old-noes-under-the-default.md), exploratory and not evidence).
On seed 22 under the default, the host gave trial t's sign at the end of trial t + k with no new mechanism:

| Delay | Mappings learned of 8 | The address names the rewarded pair | The rewarded pair's trace at the reward |
| ---: | :--- | :--- | :--- |
| 0 | 8 | 0.94 to 0.96 | 1.00 |
| 1 | 0 | 0.20 to 0.25 | 0.56 to 0.59 |
| 2 | 1, at 80 of 128 | 0.20 to 0.27 | 0.69 to 0.73 |

- The first mapping fell below chance in all four arms, so the misaddressed sign moved the network away from it.
- The sources were always the delivering trial's stimulus, since every reward zeroes the counts.
- The note's suggestion: *"an engine that keeps each trial's drawn address until its reward arrives, and consolidates under that address"*.

**Also found, and recorded here.**
- **F-77.** [ADR-0054](0054-the-causal-count-inside-the-loop.md)'s text says its rule held *"on the lattice at 1.75 at every size"*. Its own table puts the lattice 0.19 and 0.22 below the gross ratio at 1 024 units, and 0.18 and 0.22 at 4 096. Exploration 001 noticed it. The rule's outcome does not depend on it.
- **F-78.** The sweep of brief 079's dispatch had one job cut at its 330-minute bound: runtime-2 ran from 16:58Z to 22:29Z on 2026-10-24, and 18 of its 123 mutants were not tested.

**The sweep of brief 079's dispatch, recorded here** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).
Dispatch [40097581635](https://github.com/DescentVTT/VirtualCortex/actions/runs/40097581635) on `771494d` ended its thirteen sweep jobs at 22:30Z on 2026-10-24, twelve green and runtime-2 cancelled at its bound.

| Mutants | Count |
| :--- | ---: |
| Listed | 3 980 |
| Tested | 3 962 |
| Caught | 3 783: 2 418 in the state crates, 1 365 in the runtime |
| Missed | 0 |
| Unviable | 156 |
| Timeouts | 23 |
| Not tested, runtime-2's | 18 |

- 21 timeouts are where [ADR-0062](0062-the-first-complete-sweeps-list.md) reads them as inherent: `cortex-core`'s three chain iterators, six; the injector, eleven; the barrier, one; the workers' stop and run, three.
- 2 are in `Image::decode`'s loading. `image.rs:1252:24` is F-42's kind by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence; the other had stopped finishing tests beside an injector timeout.
- There is no survivor among the tested. The 18 untested are F-78's.

## Decision Drivers

- **The maintainers' choices** (2026-10-25): the address kept and the age named; this before the reward right three times in four is asked again; delays of one and two trials on three networks.
- **The question the exploration left**: the trace crosses the delay, and the address does not.
- **Unset is the run it was**, bit for bit, and so is a kept address rewarded at age zero.
- **Latest ≠ Newest**: the engine's own address and critic, kept longer; nothing is depended on.

## Considered Options

1. **The mechanism**:
   - (a) keep the last trials' addresses and counts, the reward naming its trial's age;
   - (b) a trace of addresses, the reward shared by age, its delay not named;
   - (c) a critic over several timescales (UPG-03).
2. **The delays and networks**: (a) one and two trials, seeds 22, 23 and 24; (b) one trial, seed 22.

## Decision Outcome

**Options 1(a) and 2(a).**

### The kept addresses (Specified; brief 080 builds them)

- **`Config::kept_addresses`**, the number of trials whose addresses the engine keeps, 0 unset, at most 4, the trace's τ in trials. It requires the critic and its window.
- **A trial's close** (`Executor::keep_address(targets)`, between ticks): the engine keeps the sources its critic counted, the targets, and the counts themselves, as the newest entry. It zeroes the counts and reopens the critic's window, as a reward does today.
- **A reward for a trial of age a** (`Executor::reward_kept(reward, a)`, a below `kept_addresses`): the engine writes that entry's address, reads the value and moves the value weights from that entry's counts, and sends the error to the modulator. The entry is spent.
- **At age zero the run is today's**, bit for bit: keeping then rewarding the newest entry is drawing then rewarding.
- **Not in the image**, as the address and the counts are not today. No record and no section changes, and the format stays 25.
- **The task** gains a delay: with it set to k, trial t's sign is given at the end of trial t + k, at age k. The first k trials' ends give none, and the last k signs are not given.

### The runs (Specified; brief 080 builds them in the tests)

On each of seeds 22, 23 and 24 at 1 024 units, from the image an engine born under `Config::reads_its_targets()` settles (H-42's derivations, pinned), with `kept_addresses` 2:
- H-25's two arms on H-20's schedule, the task's feedback the answer, at a delay of one trial and of two;
- twelve arms in all.

### H-43 (Hypothesis; written before any run)

The text, with its clauses, readings and stopping rule, is [H-43's report](../research/H-43.md).
- **Clauses 1 to 4**: H-25's four, in each of the twelve arms.
- **Yes** when all four hold in all twelve; otherwise **no**, with the network, the arm, the delay and the clause.

**Predicted: yes.** The exploration read the trace there and the address wrong; with the address kept, the reward reaches the pair it is for.

**Three readings are predicted:**
1. in every arm, the address each reward writes names the rewarded trial's pair in at least 0.94 of rewards, as on time;
2. on each network and arm, the reversals' summed blocks at one trial's delay are at least its twin's on time (H-42's), and at two at least at one;
3. in every arm, each mapping's last block has a mean critic value above 0.5.

**What would refute it:**
- traces spent between the trial and its reward, so that the kept address reaches too little;
- rewards of other trials consolidated in between, under their own addresses, moving the same synapses the other way;
- the critic's value, learned from older counts, too far behind the selection it should follow.

### H-43's stopping rule

1. **One round**: brief 080 builds the kept addresses and the task's delay, with their tests and ADR, then runs H-43 once. Its clauses, constants and readings are committed before any arm runs at a delay.
2. **A calibration stops the round**, and is a finding, if a pinned number moves with the addresses unset, or if an arm with them kept and rewarded at age zero is not H-42's arm on its network, bit for bit.
3. **Yes**: the next decision is the reward right three times in four asked again on the default, with a control at step zero. Then a delay the engine is not told.
4. **Yes at one trial, no at two**: an ADR on the trace across two trials, whose shift is at its ceiling of 16.
5. **No at one trial**: an ADR on what else must cross the delay: the value's counts, the signal's timing.
6. **No constant moves after such a run.** There is no second attempt, no other delay, arm or network.

### F-77 and F-78

- **F-77**, recorded and resolved by this record: ADR-0054 is accepted and is not edited, and nothing depends on the sentence.
- **F-78**, recorded and resolved in part: the runtime's sweep is dealt over sixteen shards, not twelve, in this pull request. At twelve, its jobs took 2 h 13 min to 5 h 31 min. The cause is F-76's: the test crates grow with every round's pinned tables.

## Consequences

- Good: a reward can reach the trial it is for, through the engine's own address, at no cost to an on-time run.
- Good: the twin at age zero makes the calibration a structural check of the mechanism.
- Bad: the host names the delay. A world that does not say it is the next question's.
- Bad: twelve arms more in the weekly job, about 13 000 s at 1 100 s an arm.
- Bad: sixteen sweep shards and one crates job are seventeen jobs against the account's twenty at once.
- Neutral: no record, no section and no format changes.

## Alternatives considered and why rejected

- **Option 1(b), a trace of addresses**: it shares each reward with the trials after the one it is for. The exploration read that a sign given to the wrong trial moves the network away from the mapping.
- **Option 1(c), a critic over several timescales**: the value can bridge the delay, but the consolidation would still be addressed to the delivering trial.
- **Option 2(b), one trial on one network**: a mechanism's yes on one network is that network's, and two delays test the trace as well as the address.

## Confirmation

`briefs/080_a-reward-past-the-trial.md` builds the kept addresses and the task's delay, and runs H-43 once.
[H-43's report](../research/H-43.md) holds the hypothesis as written before any run.
`npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
