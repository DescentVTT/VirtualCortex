---
status: accepted
date: 2026-10-11
depends-on: ADR-0164
decision-makers: VirtualCortex maintainers
---

# ADR-0165: Another network of the same size — the line stops among three answers and states what it holds there; every learning result since H-13 is one seeded prior's, and the next question is whether the learning configuration is the rules' or that network's: on two priors of the next two seeds, every constant derived again by the rule that derived it, H-25's two arms; no file under `src/` changed; H-33 written before any run with no prediction for the verdict; brief 067 runs it once

## Context and Problem Statement

[ADR-0164](0164-an-exploration-the-value-gates-measured.md) read H-32 as no. Its stopping rule, step 5, stops the line among three answers and names the next decision: *"an ADR that states what the learning configuration holds of a choice among three"*, and returns the line to the list H-28's stopping rule left: a less reliable reward, a reward delayed past a trial, another size and the operating regime. The maintainers took it on 2026-10-11, and took **another network**: the same size, another seed.

**The sweep of that round's dispatch**, written down here as [ADR-0158](0158-the-sweep-off-the-next-decisions-path.md) asks of the first ADR that merges after a sweep has ended: run [38111246283](https://github.com/DescentVTT/VirtualCortex/actions/runs/38111246283) ended at 07:36Z on 2026-10-11, fifty minutes after that round merged, green in all thirteen of the sweep's jobs. It is the first sweep of the runtime in twelve shards.
- **3 849 mutants over the tree, 3 673 caught and none missed**: 2 404 caught in the state crates and 1 269 in the runtime, 154 unviable and 22 timeouts.
- **In `runtime/cortex-runtime/src/task.rs`**, the file [ADR-0163](0163-an-exploration-the-value-gates-built.md) changed: 208 caught, 10 unviable, none missed and none timed out.
- **The 22 timeouts** are in the places [ADR-0062](0062-the-first-complete-sweeps-list.md) reads as inherent: `cortex-core`'s three chain iterators, six; the injector, twelve; the barrier's wait, one; the workers' stop and run, three.
- **Its twelve shards over the runtime took 1 h 13 m to 2 h 30 m each**, and the sweep ended 3 h 16 m after the dispatch, its jobs having started over the dispatch's first hour ([ADR-0166](0166-the-sweep-after-the-shards.md)).
- There is no survivor, so the round leaves no finding.

### What the learning configuration holds of a choice among three

Four rounds read it, each on H-25's configuration with one thing changed:

| Round | The one change | Mappings learned, of eight | What it bought | What it cost |
| :--- | :--- | ---: | :--- | :--- |
| H-29 ([ADR-0153](0153-three-answers-measured.md)) | three readouts where H-25 had two | 7 | — | — |
| H-30 ([ADR-0156](0156-a-punishment-the-value-does-not-soften-measured.md)) | a punishment the value does not soften | 4 | the first reversal in 19 and 22 blocks | the two after it, the couplings worn to 0.93 |
| H-31 ([ADR-0161](0161-the-span-a-choice-among-three-is-given-measured.md)) | a mapping after a flip of 48 blocks | 6 | the first reversal's mapping learned | the reversals after it slower, a coupling past 1.30 |
| H-32 (ADR-0164) | a selection drawn where the value is below zero | 5 | the first reversal's mapping learned, in 23 and 18 blocks | the second reversal's mapping in both arms, the third's in one |

**What holds:**
- **A first choice among three is learned as one between two**: 118 and 119 of the last 128, where H-25 read 122 and 124.
- **The task takes any number of readouts** up to 64, and the selection among them is the gate's rule unchanged ([ADR-0152](0152-three-answers-built.md)).
- **The couplings stay bounded, the network outside them holds, and the critic holds each stimulus's expected reward**, in every round but H-31's longer span.

**What does not hold:**
- **A revision among three, over three reversals, inside the two-answer schedule.** H-29 learned seven mappings of eight; each of three changes bought the first reversal and none held the two after it.
- **What the readings name, unmeasured**: a reversal is a gap between two couplings, crossed at about five parts in ten thousand of a coupling a rewarded trial (ADR-0164), and each change that closed the first gap sooner left a wider one for the next.

**What stays in the tree, unset**: the whole punishment ([ADR-0155](0155-a-punishment-the-value-does-not-soften-built.md)) and the exploration ([ADR-0163](0163-an-exploration-the-value-gates-built.md)). The learning configuration is H-25's as it was, and [ADR-0151](0151-three-answers.md)'s second step, the third stimulus, is not taken.

### Why another network, and why before another size

**Every learning result since H-13 was read on one network**: ADR-0044's prior at 1 024 units from seed 22. The line's last four decisions each name it where they say how the engine's premise differs from the account they cite: [ADR-0147](0147-a-reward-right-seven-times-in-eight.md), ADR-0151, [ADR-0154](0154-a-punishment-the-value-does-not-soften.md) and [ADR-0162](0162-an-exploration-the-value-gates.md) each write *"one seeded prior"*.

**What is one network's in the configuration, and what is a rule's.** Each constant below was derived on that network by a rule written before it was read:

| Constant | The rule | Its value on seed 22 |
| :--- | :--- | :--- |
| the geometry's rotation | the first from which the four stimulus–readout couplings, by the prior's census, are within ten per cent with none zero ([ADR-0065](0065-the-instrument-recalibrated.md)) | 0 |
| the gain | the first of 1.75 and 2.0 at which a frozen block sees the stimulus in 56 trials of 64 ([ADR-0066](0066-the-reward-path-measured-again.md)) | 1.75 |
| the settled network | [ADR-0055](0055-a-weight-that-settles.md)'s criterion as an integer rule over the lead-in's windows ([ADR-0077](0077-the-background-side.md)) | held at the 96th window |
| the stimulus's cancel | the offset after the refractory window, the ticks the volley spreads over, and the least messages that leave a unit firing once ([ADR-0076](0076-two-injections.md)) | 201, 9 and 6 |
| the inhibitory rule's target period | the settled network's own rate ([ADR-0129](0129-a-target-the-network-fires-at-measured.md)) | 58 201 ticks |
| the critic's window | the shortest delay of any synapse the image carries ([ADR-0134](0134-the-critics-window-built.md)) | 100 ticks |
| the critic's step and scale | an arithmetic over the stimulus's units and the reward ([ADR-0132](0132-a-critic-of-the-engines-own-measured.md)) | shift 9, scale 2 |

The rest is the configuration's and is no network's: the signed gate, the excitatory baseline at zero, the inhibitory baseline at a half, the drawn address, the schedule and the task's seed.

**So the question has two parts.** Do the rules derive a working configuration on a network they were not written on? And does that configuration then learn and revise as H-25 did?

**Why not another size first.** At 4 096 units a stimulus is four times the units, which moves the critic's step and the cancel, an arm is about four times as long, past the weekly job's bound for one shard, and a no would not say whether the size or the network read it. Another seed changes the network and nothing else, at the cost the line already pays.

**The account and the literature.** A result read on one random network is a result about that network until it is read on another; a seeded prior is a sample of one. The account predicts nothing about the verdict. It says only that the question is owed.

## Decision Drivers

- **The line's own caveat**, written in each of its last four decisions: one seeded prior.
- **The maintainers' choice** (2026-10-11).
- **One change from H-25**: the prior's seed. Everything else is H-25's, or is what a rule gives on the new network.
- **The seeds are not chosen**: the next two after the one in use.
- **A rule that gives nothing is an answer**: whether the rules transfer is half of what is asked.
- **A round without a build.** No file under `src/` changes, so the dispatch is the whole-domain shards alone ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)).
- **Predictions**: ADR-0154's and ADR-0157's were wrong. This ADR predicts no verdict.
- Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **The next question**: (a) another network of the same size; (b) another size; (c) the operating regime; (d) a reward delayed past a trial; (e) a less reliable reward.
2. **How many networks**: (a) two; (b) one; (c) more.
3. **Which seeds**: (a) 23 and 24, the next two after 22; (b) seeds picked after a look.
4. **The constants**: (a) each derived again on each network by its rule; (b) seed 22's kept.
5. **The task**: (a) H-25's two arms on H-20's schedule; (b) H-28's reward right seven times in eight as well; (c) three readouts.
6. **The clauses**: (a) H-25's four, on both networks; (b) H-25's four with the reversal's bound widened.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The networks

- **Two priors**: ADR-0044's generator at 1 024 units with every parameter as it is and the seed **23**, and the same with the seed **24**. Seed 22 is the one every round so far ran on.
- **No seed is looked at before it is taken**, and none replaces one that reads badly.
- **The drive's seed and the task's seed are H-25's.** The prior's seed is the one change.

### The configuration, derived again (Specified; brief 067 builds it in the tests)

On each network, in this order, each by the rule in the table above, with no value carried over from seed 22:
1. the rotation;
2. the gain;
3. the settled network and its image;
4. the cancel, and on a frozen block from the settled image the two measures every round since H-13 has stood on: every stimulus unit fires once, and the window after the volley holds more readout spikes than the window before the injection in at least 56 trials of 64;
5. the target period;
6. the critic's window, and the critic's arithmetic restated for the stimulus's units on that network.

Then H-25's configuration is written into the image as H-25's arms write it.

- **A rule that gives no value on a network stops that network there**, and so does a frozen block that fails a measure. That is a reading of H-33, not a failed calibration of the round: the rules did not derive a configuration on that network.
- **Nothing under `src/` changes.** The harness's constants for seed 22 become what the rules return for a prior.
- **On seed 22 the harness reads what it read**: every pinned number of the tree holds.

### H-33 (Hypothesis; written before any run)

**On each of the two networks, with the configuration its rules derive, H-25's two arms on H-20's schedule:**
- **clause 0, the rules derive a configuration**: every rule gives a value and the frozen block passes both measures;
- **clause 1, the learning holds**: every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the revision is in time**: each of the three reversals passes 40 of 64 within 23 blocks of its flip;
- **clause 4, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the four couplings lies within 0.75 and 1.25 of the image's.

Clauses 1 to 4 are H-25's ([ADR-0140](0140-the-address-drawn-measured.md)), by H-25's rules.

**Yes** when all five hold on both networks, in both arms; otherwise **no**, with the network, the arm and the clause.

**No prediction for the verdict.** What is least known is clause 0, since no rule of the table has been run on a second network, and clause 3, whose 23 blocks were read on seed 22, where H-25 took 14 to 21.

**Readings, no clause:**
- each derived constant beside seed 22's;
- the frozen block: the readouts' counts before and after the volley, the selections by readout and the ties, beside seed 22's;
- the block each mapping passes 40 of 64 in, beside H-25's 6 and 4, then 19, 21 and 16, and 20, 15 and 14;
- the couplings' courses and their highest, beside H-25's 1.191;
- the value beside $2p - 1$, with its troughs;
- the inhibitory sum's course, beside H-25's.

### H-33's stopping rule

1. **One round**: brief 067 makes the harness's constants what the rules return for a prior and runs H-33. No file under `src/` changes. The seeds, the order of the derivations and H-33's constants are committed before any rule is run on a new network.
2. **A calibration** stops the round before any rule is run on a new network, and is a finding, if a pinned number of the tree moves: on seed 22 the rules must return the constants the tree holds.
3. **Yes**: the learning configuration is the rules', not one network's. The next decision is an ADR choosing among another size, the operating regime, a reward delayed past a trial and a less reliable reward, named and not taken.
4. **No on clause 0, on either network**: a rule written on one network does not transfer. The next decision is an ADR on that rule, with what it returned on each network.
5. **Otherwise no on clause 4**: the next decision is an ADR on what the learning reaches on that network.
6. **Otherwise no on clause 1 or 2**: the configuration does not learn, or does not stay bounded, on a network it was not built on. The next decision is an ADR on what differs between the networks, among the readings above, before any other question is asked of the configuration.
7. **Otherwise no on clause 3 alone**: the configuration learns and revises on another network, more slowly than the bound read on the first. The next decision is an ADR on the bound: what a reversal takes over the networks read.
8. **No constant moves after a rule is run on a new network, and there is no second attempt and no other seed.**

## Consequences

- Good: the caveat the line's decisions have carried, asked as a question with one change and at the cost of two more arms a network.
- Good: the rules that derived the configuration are run where they were not written, which is the first test any of them has had as a rule.
- Good: no build. No rule, no record and no format changes.
- Good: the line among three answers is closed with what it holds on record.
- Bad: two networks. Three of three is still a small sample, and a no on one of two is hard to weigh.
- Bad: the same size, the same generator and the same drive. Another topology or another size is not read.
- Bad: the harness's constants for one network become functions of a prior, in a test file of two hundred thousand lines. The round's risk is there.
- Neutral: four weekly tests of about H-25's cost, 1 079 to 1 782 s each in the last dispatch, with each network's derivations in front of its arms.

## Alternatives considered and why rejected

- **Option 1(b), another size**: see above. It is the next step on the same axis if this one reads yes.
- **Option 1(c), the operating regime**: ADR-0077 read that with the criticality controller on the readouts fire seven to twenty-four times as much and the frozen block fails its measures. The instrument would have to be built again for that regime first, which is several rounds.
- **Option 1(d), a reward delayed past a trial**: the address is drawn from the window the reward closes, so a reward one trial late addresses the next trial's units. It would be run to size a failure the rules already show.
- **Option 1(e), a less reliable reward**: H-28 read that the signal at seven in eight is 0.87 to 1.03 of H-25's. A lower reliability is a matter of degree on the one network.
- **Option 2(b), one network**: one more sample, and a no on it would not say whether it or seed 22 is the unusual one.
- **Option 2(c), more than two**: each is two arms of the weekly job and its own derivations. Two is what the first reading needs; a yes or a split names more.
- **Option 3(b), seeds picked after a look**: a seed chosen for a rotation that passes, or a network that settles, is the answer written before the question.
- **Option 4(b), seed 22's constants kept**: the rotation and the cancel are properties of the network they were read on. On another, the stimulus would not be balanced and might not fire once.
- **Option 5(b), H-28's reward as well**, and **5(c), three readouts**: each is a second question. H-25 is the configuration's defining run.
- **Option 6(b), the bound widened**: 23 blocks is what H-25 was held to. If another network needs more, that is step 7's reading.

## Confirmation

`briefs/067_another-network-of-the-same-size.md` runs H-33 once. Whitepaper 4.104.0 carries H-33 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
