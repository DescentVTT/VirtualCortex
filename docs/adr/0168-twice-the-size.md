---
status: accepted
date: 2026-10-12
depends-on: ADR-0167
title: Twice the size
kind: protocol
hypothesis: H-34
summary: 'H-33''s stopping rule is taken at another size: ADR-0044''s generator at 2 048 units from seeds 22 and 23, every constant derived by its rule at that size, then H-25''s two arms. The critic''s rule is extended past 64 units a stimulus before any run.'
decision-makers: VirtualCortex maintainers
---

# ADR-0168: Twice the size — H-33's stopping rule taken at another size: ADR-0044's generator at 2 048 units from seeds 22 and 23, every constant of the learning configuration derived by its rule at that size, then H-25's two arms; 2 048 the largest size whose arm fits one test of a weekly shard; the critic's rule, which as written returns nothing past 64 units a stimulus, extended by ADR-0132's own reasons before any run; nothing under `src/` changed; H-34 written with no prediction for the verdict; brief 068 runs it once

## Context and Problem Statement

[ADR-0167](0167-another-network-of-the-same-size-measured.md) read H-33 as yes: on two priors of other seeds the rules derived the learning configuration, and it learned and revised as on the first. Its stopping rule, step 3, names an ADR choosing among another size, the operating regime, a reward delayed past a trial and a less reliable reward. **The maintainers took another size on 2026-10-12, and took 2 048 units.**

**No sweep is owed.** Brief 067's round changed no file under `src/` and dispatched the whole-domain shards alone ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).

**What H-33 left, for this question:**
- **The harness's constants are what the rules return for a prior** (`derive`, `Rules`, `prior_seeded` in `runtime/cortex-runtime/tests/inhibition.rs` and `tests/instrument/harness.rs`). `derive` writes its size, 1 024, in place at every step.
- **Three networks of one size**: seeds 22, 23 and 24 at 1 024 units. ADR-0167 states its scope as that size and that generator.
- **Five of the eight constants came back the same on all three.** Those rules were tested in returning a value that worked, not in returning another one.

**What a size changes**, by the generator's rule and the geometry's:
- **The geometry**: a period of twenty places holds one unit of each stimulus and nine of each readout. At 2 048 units a stimulus is 102 units from a rotation of at most eight, and 101 from a later one, where it is 51; a readout is nine times that.
- **The local structure does not change**: 32 synapses a unit, a window of eight, a quarter rewired, the same delays. A stimulus unit reaches what it reached.
- **The drive scales with the units**: one message a tick for every 128 units (`drive`).
- **The capacities**: `config` sizes the train by the units, and holds the injector at 4 096 messages and each worker's nodes at 65 536, whatever the size.
- **The cost**: under ADR-0044's drive every unit is served on every tick ([ADR-0097](0097-the-active-set-measured.md)), so an arm is expected to take twice as long. A developer machine ran six windows of the settling at each size, six pairs one after another, from a probe that is not in the tree: the five pairs after the first read 1.8 to 2.0 times the seconds at 2 048 units, and the first 3.3, its run at 1 024 being the process's first and taking 5.7 s where the five after it took 8.8 to 10.5. That is not a measurement ([ADR-0010](0010-measured-or-target.md)); it is why the expectation is twice, and why it is an expectation.

**What has run at another size**: the reference tests run ADR-0044's network under the drive at 256, 1 024 and 4 096 units (`tests/reference.rs`). The task has run at 256 and 1 024 units, and the learning configuration at 1 024 only.

**The critic's rule does not reach this size as written.** `critic_for` restates [ADR-0132](0132-a-critic-of-the-engines-own-measured.md)'s arithmetic for a stimulus of $N$ units:
- the sum of the shift and the scale is the least at which a volley moves its own value by at most a thirty-second of the error, $32 N \le 2^{\text{sum}}$;
- it keeps ADR-0132's scale, 2, and gives the shift the rest;
- it returns nothing where the shift's dead band, $2^{\text{shift}}$, is not under one per cent of the reward.

At 51 units the sum is 11 and the rule returns a shift of 9 and a scale of 2. At 102 units the sum is 12, the shift would be 10, its dead band of 1 024 is 1.56 per cent of the reward, and **the rule returns nothing**; the gate pins that at 65 units. A run of the rule at 2 048 units would read a no on its first clause for a reason known today.

**ADR-0132 gave three reasons for its split**, at 51 units. Each beside the two splits a sum of 12 allows:

| ADR-0132's reason | A shift of 9, a scale of 2, at 51 units | A shift of 10, a scale of 2, at 102 | A shift of 9, a scale of 3, at 102 |
| :--- | ---: | ---: | ---: |
| the dead band, of the reward | 512, 0.78 % | 1 024, 1.56 % | 512, 0.78 % |
| the floor's drift a trial, one LSB down on each unit counted, by the volley's units alone | 51 / 4, about 13 of 65 536 | 102 / 4, about 26 | 102 / 8, about 13 |
| a volley's weights when it carries the whole reward, of the rail | 5 140, a sixth | 2 570, half of that | 5 140, a sixth |

A shift of 9 and a scale of 3 at 102 units gives each of the three what ADR-0132 chose at 51. The sum's own rule holds the step: a presentation moves the value by $N / 2^{\text{sum}}$ of the error, 51 of 2 048 and 102 of 4 096, a fortieth both.

**The account.** A result at one size is a result at that size. Nothing here predicts the verdict.

## Decision Drivers

- **The maintainers' choice** (2026-10-12): another size, and 2 048 units.
- **One change from H-33**: the size. The generator's other parameters, the drive's rule, the task and the schedule are H-25's.
- **The rules are run as they are**, but for the one that is known to return nothing. That one is extended here, before any run, by the reasons it was written from.
- **A rule that gives no value is an answer**, as in H-33.
- **A round without a build.** No file under `src/` changes.
- **Predictions**: ADR-0165 wrote none and H-33 read yes; ADR-0154's and ADR-0157's were wrong. This ADR predicts no verdict.
- Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **The size**: (a) 2 048 units; (b) 4 096 with a shorter schedule; (c) 4 096 with the weekly job's bound raised; (d) 512.
2. **The networks**: (a) seeds 22 and 23; (b) seed 22 alone; (c) three.
3. **The critic's rule at a larger stimulus**: (a) the shift held where its dead band is under one per cent of the reward, the scale the rest of the sum; (b) the rule as written, returning nothing; (c) ADR-0132's constants carried over.
4. **The clauses**: (a) H-33's five; (b) H-33's with the reversal's bound as a reading.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

### The size (option 1(a))

- **An arm at 1 024 units read 1 815 to 1 842 s** in ADR-0167's dispatch. At twice that, about 3 700 s, an arm is 51 per cent of a shard's 120 minutes by itself. At 4 096 units it is about 7 400 s, past them.
- **2 048 is the largest size at which H-25's whole schedule, three reversals, fits one test of one shard.**

### The networks (option 2(a))

- **ADR-0044's generator at 2 048 units from seed 22 and from seed 23**, every other parameter as it is. They are the first two of the three seeds read at 1 024; at another size each is another network.
- **Two, so that a no can be told apart**: on both networks it is the size's, on one it is that network's.
- **No network is looked at before this is committed**, none replaces one that reads badly, and no third is run.
- **The drive's rule and seed and the task's seed are H-25's.**

### The critic's rule at a larger stimulus (option 3(a))

For a stimulus of $N$ units, with the sum as `critic_for` finds it:

- **the shift is the lesser of the sum less ADR-0132's scale and 9**, the largest shift whose dead band is under one per cent of the reward;
- **the scale is the sum less the shift**;
- **nothing is returned** where no sum of the range fits, where the sum is below ADR-0132's scale, and where the loader would refuse the constants.

| A stimulus of | The sum | The rule as written | The rule from this ADR |
| :--- | ---: | :--- | :--- |
| 4 units | 7 | 5 and 2 | 5 and 2 |
| 32 | 10 | 8 and 2 | 8 and 2 |
| 33 to 64 | 11 | 9 and 2 | 9 and 2 |
| 65 to 128 | 12 | nothing | 9 and 3 |
| 129 to 256 | 13 | nothing | 9 and 4 |

- **Where the rule returned a value it returns that value.** Seeds 22, 23 and 24 at 1 024 units read 9 and 2 as they did.
- **One assertion of the gate moves, by this ADR**: a stimulus of 65 units, which the tree pins as nothing, reads 9 and 3. No other pinned number moves.
- This is a change to a rule that derives a constant, which brief 067 was not empowered to make. It is decided here from the table above, before any run at the new size.

### The configuration at the new size (Specified; brief 068 builds it in the tests)

On each network, in ADR-0165's order and by the same rules, the size given where 1 024 was written: the rotation; the gain; the settled network; the cancel and the frozen block's two measures; the target period; the critic's window and the critic's constants. Then H-25's configuration is written into the image, and H-25's two arms run on H-20's schedule.

- **Nothing under `src/` changes.**
- **At 1 024 units everything reads what it read**: on seeds 22, 23 and 24 the rules return what the tree pins.
- **No capacity of the run's configuration is reached**: no injection is refused, no worker drops a spike and the train holds every window read from it. A capacity that would be reached at 2 048 units is the harness's to size by the units, as it sizes the train, with 1 024's as it was; the round's ADR says which.

### H-34 (Hypothesis; written before any run)

**On each of the two networks at 2 048 units, with the configuration its rules derive, H-25's two arms on H-20's schedule:**
- **clause 0, the rules derive a configuration**: every rule gives a value and the frozen block passes both measures, every stimulus unit firing once and the stimulus seen in 56 trials of 64;
- **clause 1, the learning holds**: every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the revision is in time**: each of the three reversals passes 40 of 64 within 23 blocks of its flip;
- **clause 4, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the four couplings lies within 0.75 and 1.25 of the image's.

These are H-33's five, by H-33's rules.

**Yes** when all five hold on both networks, in both arms; otherwise **no**, with the network, the arm and the clause.

**No prediction for the verdict.** What is least known:
- **the gain's rule** has two candidates. It took 2.0 at 256 units, where 1.75 saw the stimulus in 50 trials of 64, and 1.75 at 1 024. At 2 048 either may pass, or neither;
- **the rotation's rule** returned 17 at 256 units and 0 on all three networks at 1 024;
- **the critic's constants** at this size are this ADR's extension, and have not been run;
- **a selection read on twice the spikes** is sharper. Whether that shortens a reversal or lengthens it is not derived;
- **the task at this size** has not been run on the engine.

**Readings, no clause:**
- each derived constant beside the three networks' at 1 024;
- the frozen block: the readouts' counts before and after the volley, the selections by readout, the ties and the margin, beside 1 024's;
- the block each mapping passes 40 of 64 in, and each reversal's blocks, beside the eighteen reversals H-25 and H-33 read at 1 024, 12 to 21 blocks;
- the couplings' courses and their highest, beside H-25's and H-33's six arms at 1 024, 1.181 to 1.195;
- the value beside $2p - 1$ with its troughs, what a presentation moves it by, and the units the critic counts a trial beside the volley's;
- the inhibitory sum's course;
- an arm's seconds beside twice an arm's at 1 024.

### H-34's stopping rule

1. **One round**: brief 068 gives the derivation its size and the critic's rule its extension, and runs H-34. No file under `src/` changes. The seeds, the size and H-34's constants are committed before any rule is run at the new size.
2. **A calibration** stops the round before any such run, and is a finding, if a pinned number of the tree moves other than the one this ADR names: at 1 024 units the rules must return what the tree holds on seeds 22, 23 and 24.
3. **Yes**: the learning configuration is its rules' at twice the size as at the first. The next decision is an ADR choosing among a larger size still, which needs the weekly job's bound decided first, the operating regime, a reward delayed past a trial and a less reliable reward, named and not taken.
4. **No on clause 0, on either network**: a rule does not transfer to this size. The next decision is an ADR on that rule, with what it returned at each size.
5. **Otherwise no on clause 4**: the next decision is an ADR on what the learning reaches at this size.
6. **Otherwise no on clause 1 or 2**: the configuration does not learn, or does not stay bounded, at twice the size. The next decision is an ADR on what the size changes, among the readings above and the critic's constants first, before any other question is asked of the configuration.
7. **Otherwise no on clause 3 alone**: it learns and revises at this size more slowly than the bound read at 1 024. The next decision is an ADR on the bound: what a reversal takes over the sizes read.
8. **No constant moves after a rule is run at the new size, and there is no second attempt, no other seed and no other size.**

## Consequences

- Good: the second thing the line's results were scoped by, read with one change and by the rules as they stand.
- Good: a no that was known beforehand is not spent a round on. The critic's rule is extended by what it was written from, and the ADR says so.
- Good: no build. No rule of the engine, no record and no format changes.
- Bad: twice is a small step. 4 096 units and beyond are not read, and need the weekly job's bound decided first.
- Bad: the critic's extension is new at the size it is first run at. If clause 0 or clause 1 fails there, it is the first thing to be told apart from the size.
- Bad: **the weekly job grows by four arms of about an hour each and two derivations.** The table goes from 66 471 s to about 82 000. Dealt to twelve shards that run their tests two at a time, that is about 3 450 to 3 770 s of wall time a shard, 48 to 52 per cent of the bound, where ADR-0167's deal planned 39 to 42.
- Bad: **an arm alone is half of a shard's bound, and no deal moves that.** On runners a fifth slower, as [ADR-0121](0121-a-sixth-shard.md) read them, a shard that holds one passes the briefs' 60 per cent. The first dispatch costs the new tests at the table's 900 s, so its shards are less even than the plan. **A shard's bound against its longest test is the workflow's next decision**, named and not taken here.
- Bad: the round is long. Its local runs are four arms at twice the size.
- Neutral: the same generator, the same drive's rule, and two networks.

## Alternatives considered and why rejected

- **Option 1(b), 4 096 units with a shorter schedule**: one reversal fits. Three rounds among three answers read that the first reversal holds where the later ones do not, so a run without the later ones would leave the question open.
- **Option 1(c), 4 096 units with the bound raised**: a rule of the workflow moved for one size, and every round's wait for its shards past two hours.
- **Option 1(d), 512 units**: smaller. A stimulus is 25 units and a selection is read on half the spikes, and the question the scope leaves is whether the configuration holds as the network grows.
- **Option 2(b), seed 22 alone**: a no on it would not say whether the size or that network read it.
- **Option 2(c), three networks**: two more arms of an hour each, on a job whose arms are already half a shard's bound.
- **Option 3(b), the critic's rule as written**: it returns nothing at 102 units, so H-34 would be a no on clause 0 by an arithmetic done today.
- **Option 3(c), ADR-0132's constants carried over**: a shift of 9 and a scale of 2 at 102 units moves the value by a twentieth of the error a presentation, twice ADR-0132's step, and that is a second change beside the size.
- **Option 4(b), the reversal's bound as a reading**: 23 blocks is what H-25 and H-33 were held to. If twice the size needs more, that is step 7's reading.

## Confirmation

`briefs/068_twice-the-size.md` runs H-34 once. Whitepaper 4.106.0 carries H-34 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
