---
status: accepted
date: 2026-09-27
depends-on: ADR-0115
decision-makers: VirtualCortex maintainers
---

# ADR-0116: The region before the readout — ADR-0115's next decision deferred by one round: before a readout is gated by the context, the usable region is measured for its width and for whether it survives another seed, a drained network and the task's own stimulus, on the geometry the task and the context will share; F-54 decided: the readouts give up places 5 and 16, eight places each, so that an assembly of up to 102 units is in no set; brief 050 measures, nothing of the engine changed

## Context and Problem Statement

[ADR-0115](0115-the-assemblies-marked.md) measured ADR-0112's assemblies with their members marked under [ADR-0114](0114-the-facilitating-class-built.md)'s class, and read **one cell usable**: 64 units under set (ii) (a release fraction of 26/256, $\tau_f = 2^{16}$, $\tau_d = 2^{13}$) at a quarter of Q1.15's range. It held in 8 of 8 rounds as a train of bursts refreshed inside the class's primed window, ignited in no unkicked span, let go in 8 of 8, and did not spill. By brief 049's branch, the next decision it named is ADR-0111's second round, a readout gated by the context, with F-54's geometry to solve first.

What that one cell does not yet say:
- **How wide the region is.** The cell sits at the grid's lightest weight and largest size. Below 0.25 and above 64 units the grid does not reach, so the region is bounded on two sides and open on two. At 32 units the same weight holds in 5 rounds and ignites in 2; at 64 units and 0.375 it ignites in every round.
- **Whether it is one seed's.** There is one draw of the delays (seed 48), one placement and one drive.
- **Whether it survives learning.** The network was ADR-0077's settled image. Over a learning run the inhibitory sum falls, and under H-20's configuration it levelled at about 0.07 and 0.08 of the image's ([ADR-0110](0110-a-schedule-of-reversals-measured.md)). A network with less inhibition is more excitable, and a cell one step from running away may run away there.
- **Whether it survives the task.** No task ran. Under F-54's geometry the members are readout places, and the stimulus places are within the prior's local window of them: place 5 is five places from stimulus A's place 0, and place 16 five from stimulus B's place 11 and four from the next period's place 0. So the task's stimulus reaches the members directly through the prior's local synapses, every trial.

A readout gated by a context that turns out to be one seed's, or that the task ignites, would be a mechanism whose failure could not be told from its substrate's. The maintainers chose on 2026-09-27 to measure the region first.

**F-54** (whitepaper §11) is the other half. The task's geometry (`geometry` in `tests/instrument/harness.rs`, [ADR-0065](0065-the-instrument-recalibrated.md)) reads the ring in periods of twenty, and every place of a period is in one of four sets:
- stimulus A at place 0 and B at place 11;
- readout 0 at the nine odd places but 11 (`R0_MASK`, `0xAA2AA`: 1, 3, 5, 7, 9, 13, 15, 17 and 19);
- readout 1 at the nine even places but 0 (`R1_MASK`, `0x55554`: 2, 4, …, 18).

At 1 024 units, 51 whole periods cover units 0 to 1 019. F-54 named three ways out: another geometry, fewer readout places, or another size.

## Decision Drivers

- **The maintainers' choice**: measure the region before building on it.
- **The substrate's readings carry over.** ADR-0112 and ADR-0115 measured assemblies at places 5 and 16. A geometry that keeps those places for the context keeps every reading they made; one that moves the context moves the substrate, and its region would have to be found again.
- **Nothing of the engine changes.** This round is a measurement. The geometry is the test's, as the task's geometry has always been.
- **The weekly job's budget.** The whole-domain tests cost 23 357 s on the hosted runners by the cost table ADR-0115's dispatch regenerated, dealt to four shards bounded at 120 minutes, each running as many tests at once as half the runner's cores (`scripts/exhaustive-shard.sh`); ADR-0115's dispatch took 44 to 54 minutes a shard. Brief 049 held every shard under 60 per cent of its bound. A cell of ADR-0115's cost about 100 s there.
- **Latest ≠ Newest**: no dependency, no tool, no rule.

## Considered Options

1. **F-54**:
   - (a) the readouts give up places 5 and 16, eight places each;
   - (b) another size, with the task on part of the ring;
   - (c) another geometry that places the context elsewhere.
2. **What round comes next**:
   - (a) the region measured first, then the gated readout;
   - (b) the gated readout built on the one usable cell, F-54 solved in the same round.
3. **The drained network**:
   - (a) the image H-20's configuration leaves at the end of an arm;
   - (b) the settled image with every inhibitory weight scaled to the fraction H-20 read;
   - (c) none.

## Decision Outcome

**Options 1(a), 2(a) and 3(a), with 3(b) allowed beside it as a reading.**

### F-54, decided (Specified; brief 050 builds it as the test's geometry)

- **The readouts give up the context's places.** Readout 0 drops place 5 (`0xAA28A`: 1, 3, 7, 9, 13, 15, 17 and 19), and readout 1 drops place 16 (`0x45554`: 2, 4, 6, 8, 10, 12, 14 and 18). The stimulus places are unchanged.
- **So places 5 and 16 are in no set.** At 1 024 units that is 102 units, 51 periods of two, and an assembly of up to 102 members at ADR-0112's placement is disjoint from every set.
- **Placement unchanged.** It is ADR-0112's placement (places 5 and 16 of the first $\text{size}/2$ periods, nested), so ADR-0115's readings are the readings of an assembly on this geometry.
- **Beside the old geometry, not in its place.** The geometry is a second one in the harness, beside `geometry`, which every earlier test keeps. No pinned number moves.
- **The task on it is not H-20's task.** Each readout counts eight places, not nine. A later round that asks H-20's schedule again with a context needs H-20's schedule on this geometry without a context as its baseline, and this ADR names that as a need of ADR-0111's third round.

### The round (brief 050): the region's width and robustness, measured

On ADR-0077's settled image at 1 024 units, every weight frozen, under ADR-0115's protocol, spans, release, stretch, measures and thresholds. The one difference is the conditions below.

- **The core grid** (the settled image, delay seed 48, no task): set (ii); sizes 48, 64, 80 and 96; weights 0.125, 0.1875, 0.25, 0.3125 and 0.375 of Q1.15's range. That is twenty cells, each size with its own background and control.
  - 64 units at 0.25 and at 0.375 are ADR-0115's cells. Each must reproduce ADR-0115's tables bit for bit, which holds the round's harness to ADR-0115's.
- **Three conditions**, on a subset fixed before any run: sizes 64 and 96 at weights 0.1875, 0.25 and 0.3125, six cells each.
  - **(b) Another seed**: the assembly's delays drawn with seed 49 in place of 48.
  - **(c) A drained network**: the image H-20's arm from the assignment leaves at its 7 680th trial, with its inhibitory sum at about 0.07 of the settled image's, frozen: its inhibitory baseline and signed gate are unset before any cell runs on it, since H-20's inhibitory baseline would go on consolidating every inhibitory synapse. The arm learned on the old geometry, where places 5 and 16 were readouts, so the members' incoming couplings from the stimulus sets were answer pairs there. That is a confound, recorded as one.
  - **(d) The task's stimulus**: on the settled image, the task's stimulus as H-20's arms present it (F-46's shape with ADR-0076's cancel, `CANCEL_PICKED_1024`), into A or B by the task's draw at every epoch's start, with the weights frozen and no reward.
- **Robust**: a cell usable in the core grid and under each of (b), (c) and (d).
- **The cell for the second round**: of the robust cells, the one with the most usable neighbours in the core grid, one weight step and one size step either way. Ties go to the lighter weight, then the smaller size.
- **If no cell is robust**, the next decision follows from what failed:
  - under (c), the inhibitory drain (one of H-20's four), or an inhibition of the context's own;
  - under (d), the context's isolation from the task's stimulus: a placement beyond the prior's window from the stimulus places, which needs another geometry;
  - under (b), the region is one draw's, and the class's constants are the question.
- **Readings, no clause**:
  - under (d), the readouts' counts per trial with the context held and with it quiet, which is how much a held context already reaches the readouts through the prior (the second round's need);
  - under (d), whether the eight-place readouts still resolve the stimulus from the background by ADR-0065's measure;
  - under (c) beside it if the round adds option 3(b), the settled image with its inhibitory weights scaled.

The round names its region, its cell and its next decision, and takes none.

## Consequences

- Good: the second round, if it comes, builds on a region whose width and robustness are read, on the geometry it will use.
- Good: F-54 is decided without moving the substrate. ADR-0115's usable cell is already on the new geometry's free places.
- Good: condition (d) asks the question the second round would otherwise meet unasked, whether the task ignites the context.
- Bad: one more round before a readout is gated, and the weekly job grows by about a third of its present cost: some sixty runs at ADR-0115's hundred seconds a run, and the drained image, which is an H-20 arm of 1 500 to 1 700 s on the hosted runners. That is about fifteen minutes a shard, inside brief 049's 60 per cent.
- Bad: each readout loses a ninth of its places, so the task on this geometry is not the task H-13 to H-20 read, and the third round needs its own baseline.
- Neutral: condition (c)'s drained image carries the old geometry's answer pairs onto the members' places.

## Alternatives considered and why rejected

- **Option 1(b), another size**: the task's rotation and gain are calibrated at 256 and 1 024 units only (`rotation`, `gain`), so a size would need ADR-0065's calibration again, and the substrate's readings at 1 024 would not carry.
- **Option 1(c), another geometry**: a context placed beyond the prior's window from the stimulus places would need the stimulus and readout places moved, which changes the task more than dropping two places does, and would move the assembly off the substrate ADR-0112 and ADR-0115 measured. It stays named as the answer if condition (d) fails.
- **Option 2(b), the readout first**: it would rest on one cell of one seed on a network the learning line does not keep.
- **Option 3(b) alone, scaled inhibition**: a drain as the learning rule leaves it is not uniform, and the image an arm leaves is the network a context meets late in a learning run. It is allowed as a reading beside 3(a).
- **Option 3(c), no drained network**: H-20 read the inhibitory sum falling to 0.07. A context that holds only on the settled image would fail where it is needed.

## Confirmation

`briefs/050_the-region-before-the-readout.md` measures the region. Whitepaper 4.66.0 carries this decision in §11.1's question on a rule held by the network and in F-54's row, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
