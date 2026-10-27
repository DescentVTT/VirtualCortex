---
status: exploratory
date: 2026-10-27
---

# 009: A bound at consolidation

**Branch**: `explore/009-a-bound-at-consolidation`
**Commit**: `6f106f2` for every run. It carries `7338d36`, exploration 007's soft bound under `src/`, and `9bd773a`,
this exploration's two bounds under `src/`. Unset, both are the rule bit for bit.
**Runs**: [40508746883](https://github.com/Menteion/Menteion/actions/runs/40508746883) (the short run) and
[40509719948](https://github.com/Menteion/Menteion/actions/runs/40509719948) (the bounds' arms and the default's rail
paths), both at `scope=arms`

Exploration 007 bounded the excitatory pairing's potentiation, over the whole width and in its top eighth. Neither held
the couplings without harming learning. Under both, every synapse that reached Q1.15's rail did so after a punishment,
where under the default 96 to 98 % of the arrivals followed a reward ([007](007-sixteen-schedules-and-a-soft-bound.md)).
007 read that path as the signed gate moving a negative trace into the weight, which a bound on the pairing does not
reach. This note asks four things on seed 22:

1. whether a bound at consolidation holds the couplings without harming learning: every rise of a magnitude scaled by
   the room below the rail, whichever the signal's sign. And after which sign the synapses that reach the rail arrive;
2. the same of a soft bound on the pairing's potentiation that is whole up to ADR-0055's reference magnitude and falls
   to nothing at the rail, so that the bulk of the synapses keeps the rule's balance;
3. what ADR-0055 and ADR-0094 say of a negative trace moved into the weight, and how much of the couplings' rise under
   the default goes by that path;
4. if the first or the second held the couplings at or under 1.30 of the image's with every mapping learned over eight
   schedules, the same over sixteen.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host.

**In brief**
- **The bound at consolidation drains the couplings.** The highest coupling fell from 1.04 to 1.07 of the image's in
  the first schedule to 0.60 and 0.61 in the eighth, the lowest to 0.11 and 0.10. No synapse reached the rail in sixteen
  schedules. Learning went with the couplings: from the second schedule on each arm learned 0 to 2 of the 4 mappings, at
  46 to 91 of its last 128.
- **The bound above the reference slows the couplings and does not hold them.** The highest rose to 1.44 and 1.47 in
  the eighth schedule, past 1.30 in the third and the fourth, by 0.024 and 0.035 a schedule over the last four against
  the default's 0.049 and 0.052. From the third schedule on each arm learned 2 of the 4 mappings and stopped reversing.
- **Under it, as under 007's two, every arrival at the rail followed a punishment** and every departure a reward. At
  seven eighths of the rail the arrivals came after both signs.
- **Under the default the signed gate's path is a loss.** A punishment's potentiation of negative traces was 25 to 29 %
  of all the weight the couplings gained, but what a punishment took off was 1.9 to 2.7 times it. The punishments'
  net was −363 to −450 rails a schedule, and the rewards' +439 to +482: the rise, 33 to 77 rails, is the rewards'
  surplus. Of the punishments' potentiation 2 to 10 % fell on synapses at or above three quarters of the rail.
- **The decisions read the path once, over one reversal.** ADR-0093 named it, ADR-0094 built it and ADR-0096 measured
  it at the image's couplings; ADR-0055, which made the depression largest at the rail, predates it. None reads it near
  the rail or over more than one schedule.
- **Item 4 did not run**: neither bound met its condition.

## What ran

**The network, the arm and the path** are exploration 007's. Every test starts from H-42's network of seed 22 at 1 024
units, the image an engine born under `Config::reads_its_targets()` settles. Each test first runs that derivation again
and holds it to H-42's pinned tables (`probe_made`), and all six held. Every arm printed the CRC-64 of that image,
`0x750105a595510fa3`. The arm is H-43's twin on time, H-42's arm bit for bit, from the assignment and from the mirrored
assignment. A schedule ends with the engine run quiet without the drive until it is quiescent; its image is written,
and the next schedule's engine is decoded from it.

**The harness** is exploration 007's as its tag holds it (`0bbccf7`): exploration 005's readings per block, per window
and per image, and the inhibitory rule's replay, held to the engine's blocks at every block's end in every arm; 007's
summary per schedule; each image's CRC-64; and each bound's eighth image printed whole, for a second half. **007's rail
paths are read in every schedule of every arm**, and widened:
- at every trial's end each synapse of the four couplings is read against its weight at the trial before, and what it
  did is set against the sign of the signal the earlier trial's reward left, as in 007;
- beside the synapses that arrived at the rail and left it, those that rose to seven eighths of it or above from below,
  and those that fell below it;
- the weight gained and lost by where the synapse stood before the move: below ADR-0055's reference magnitude, 0x2000
  or a quarter of the width; from it to three quarters of the rail; and at or above three quarters.

**The two bounds, under `src/` on this branch alone.** Commit `9bd773a`:
- **At consolidation** (`SynapseBlock::consolidate_signed_bounded`, `Executor::set_consolidation_bound`). Wherever a
  consolidation raises a slot's magnitude $M$, under a modulation above zero or below it, the rise lands scaled by
  $(R - M)/R$, $R$ = 32 767, in integers rounded to nearest. A fall is the rule's.
  - The trace is spent as the rule spends it, by the rise the rule would have made, so the rest of the rise is lost
    rather than kept pending in the trace.
  - It applies to every excitatory synapse. An inhibitory synapse consolidates under the rule, so the inhibitory rule
    is the default's and its replay still holds the engine's.
- **Above the reference** (`SoftBound::AboveReference`): exploration 007's soft bound on an excitatory pairing's
  potentiation, whole at and below ADR-0055's reference magnitude and scaled by $(R - M)/(R - 8\,192)$ above it, nothing
  at the rail. At the reference the rule's $A_- / A_+$ is 1.05, as it is unbounded. The depression, the inhibitory rule
  and consolidation do not move.
- **Neither is in any image**, so the test sets it on every schedule's engine. Unset, each is the rule bit for bit:
  - a property test over the lattice holds the consolidation, and 007's unit test the pairing;
  - the short run's unbounded schedule held H-42's arm, and wrote the image 007's branch wrote, CRC-64
    `0x841f164eb6fdefb2`;
  - the default's four schedules held exploration 005's traces and values, and 007's rail paths figure for figure.

Each bound ran eight schedules from H-42's image, from both arms. The default ran four from the assignment, as 007's
rail paths did.

## 1 and 2. The two bounds over eight schedules

Each cell lists the eight schedules in order. The default's rows are exploration 005's, as 007 tabled them:

| Rule | Arm | Highest coupling | Lowest coupling | Lowest of the mappings' last 128 correct | Mappings learned |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Default | assignment | 1.239, 1.350, 1.428, 1.461, 1.520, 1.592, 1.619, 1.658 | 0.903, 0.930, 0.985, 1.036, 1.090, 1.151, 1.207, 1.286 | 120, 121, 124, 126, 126, 127, 124, 127 | 4 in each |
| Default | mirrored | 1.248, 1.298, 1.374, 1.431, 1.444, 1.521, 1.564, 1.637 | 0.889, 0.956, 0.970, 1.018, 1.070, 1.115, 1.150, 1.179 | 120, 122, 123, 123, 126, 122, 126, 126 | 4 in each |
| At consolidation | assignment | 1.067, 0.765, 0.669, 0.641, 0.605, 0.587, 0.591, 0.599 | 0.536, 0.313, 0.222, 0.158, 0.119, 0.077, 0.109, 0.110 | 72, 70, 58, 46, 53, 56, 50, 57 | 3, 1, 2, 2, 1, 2, 0, 2 |
| At consolidation | mirrored | 1.037, 0.781, 0.666, 0.638, 0.607, 0.607, 0.595, 0.606 | 0.529, 0.303, 0.216, 0.162, 0.126, 0.119, 0.099, 0.102 | 80, 65, 61, 55, 57, 60, 56, 59 | 4, 0, 1, 1, 2, 1, 0, 1 |
| Above the reference | assignment | 1.209, 1.248, 1.311, 1.346, 1.388, 1.403, 1.425, 1.441 | 0.912, 0.964, 0.981, 1.019, 1.019, 1.035, 1.042, 1.044 | 57, 76, 65, 58, 52, 28, 9, 6 | 3, 3, 2, 2, 2, 2, 2, 2 |
| Above the reference | mirrored | 1.147, 1.249, 1.298, 1.334, 1.371, 1.435, 1.454, 1.472 | 0.895, 0.971, 0.981, 1.007, 1.015, 1.024, 1.024, 1.038 | 87, 83, 55, 19, 13, 9, 3, 8 | 4, 4, 2, 2, 2, 2, 2, 2 |

The couplings are over every block's end of the schedule and the four pairs, over the first image's; a mapping is
learned at 80 or more of its last 128.

Beside them, at each schedule's last block, over the first image's, and the most synapses of one coupling at the rail
or at or above seven eighths of it at a block's end:

| Rule | Arm | Inhibitory sum | Gain | Mean weight over the rail | At the rail, most | At or above 7/8, most |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Default | assignment | 0.743 to 0.556 | 1.7062 to 1.6804 | 0.23–0.30 to 0.32–0.41 | 26 to 100 | 42 to 130 |
| At consolidation | assignment | 0.740, 0.580, 0.529, 0.508, 0.497, 0.490, 0.486, 0.484 | 1.7211 to 1.7240 | 0.13–0.19 to 0.03–0.14 | 0 in each | 6, then 0 or 1 |
| At consolidation | mirrored | 0.738, 0.578, 0.528, 0.509, 0.499, 0.492, 0.488, 0.485 | 1.7217 to 1.7236 | 0.13–0.19 to 0.03–0.14 | 0 in each | 6, then 0 or 1 |
| Above the reference | assignment | 0.743, 0.597, 0.556, 0.541, 0.534, 0.532, 0.528, 0.526 | 1.7099 to 1.6971 | 0.24–0.28 to 0.25–0.35 | 9 to 21 | 12 to 31 |
| Above the reference | mirrored | 0.739, 0.595, 0.559, 0.544, 0.537, 0.534, 0.532, 0.533 | 1.7078 to 1.6933 | 0.24–0.28 to 0.25–0.35 | 13 to 31 | 16 to 37 |

The mean weight is the range over the four couplings, in the first schedule and in the eighth; the first image's was
0.23 to 0.25.

**At consolidation.**
- **The couplings sank and settled low.** Over the four couplings the weight's net move in a schedule was −252 and −270
  rails in the first, −125 and −129 in the second, and within 12 rails of zero from the sixth on. By then the mean
  weight stood at 0.03 to 0.14 of the rail and the couplings at 0.08 to 0.61 of the image's.
- **A reading of what drained them**: the rise scaled in the bulk. At the image's mean magnitude, about a quarter of
  the rail, a rise lands at three quarters of the rule's and a fall whole. In the first schedule the rewards netted +181
  and +172 rails where under the default they netted +439, and the punishments −432 and −442 against −363. From the
  sixth schedule the rewards netted +430 to +442 and the punishments −436 to −442.
- **Learning was lost with them.** The first schedule learned 3 and 4 mappings, its three reversals summing 70 and 72
  blocks to 40 of 64 against the default's 34 and 40. From the second schedule each arm learned 0 to 2 mappings, at 46
  to 91 of the last 128, about the 64 of chance, and 53 to 54 % of the trials were punished. Clause 3 failed in every
  schedule and clause 1 in every one but the mirrored arm's first; clauses 2 and 4 held throughout.
- **Nothing reached the rail**, after either sign, in sixteen schedules. At or above seven eighths, 14 and 32 synapses
  arrived in the first schedule, all but one after a reward, and at most one in any schedule after it.
- The inhibitory sum fell further than the default's, to 0.484 and 0.485, by about 0.002 a schedule at the end. The
  gain rose a little, where the default's fell.

**Above the reference.**
- **The couplings rose more slowly and did not stop.** The highest passed 1.30 in the third schedule from the
  assignment and in the fourth from the mirrored assignment, and rose by 0.024 and 0.035 a schedule over the last four.
  The lowest rose to 1.04, where the default's reached 1.29 and 1.18. The bulk stayed nearer the image's, its mean
  weight 0.25 to 0.35 of the rail in the eighth schedule against the default's 0.30 to 0.41.
- **Learning stopped reversing.** In the first two schedules the network learned 3 of the 4 mappings from the
  assignment and all 4 from the mirrored assignment, whose second schedule held all four of H-25's clauses. From the
  third schedule each arm learned 2 of the 4, the same two in every schedule: the second and the fourth, under the
  mapping opposite the one each schedule starts with, at 95 to 127 of their last 128. The first and the third fell, to
  3 to 10 of 128 in the seventh schedule. From the assignment it held the mirrored mapping, and from the mirrored
  assignment the assignment, as 007's bound over the width held the mirrored mapping from the assignment.
- **The rail and the signal's sign**, per schedule, from the assignment and from the mirrored assignment:

  | Arm | Arrived at the rail after a punishment | Left it after a reward | Rose to 7/8 or above after a reward | After a punishment |
  | :--- | :--- | :--- | :--- | :--- |
  | assignment | 376, 494, 299, 138, 281, 420, 367, 126 | 371, 488, 309, 134, 275, 412, 370, 132 | 11, 87, 112, 79, 250, 193, 160, 98 | 72, 98, 69, 103, 293, 291, 188, 152 |
  | mirrored | 405, 750, 252, 303, 682, 641, 580, 153 | 388, 753, 237, 313, 683, 639, 580, 153 | 31, 166, 118, 289, 299, 183, 166, 193 | 106, 151, 243, 287, 296, 215, 259, 274 |

  No synapse arrived at the rail after a reward, and none left it after a punishment. Up to 7 a schedule left it after
  the lead-in's trial.
- **A reading of why**, not measured here: above the reference the pairing's potentiation shrinks to nothing at the
  rail while ADR-0055's depression grows to four times its reference amount there. Near the rail a slot's trace is then
  negative wherever its target fires before it. A reward consolidates that trace and lowers the weight; a punishment,
  under the signed gate, raises it. At the top of the width the sign of what the task teaches is reversed.

## 3. The signed gate near the rail

**What the decisions say.**
- **ADR-0055** (2026-09-14) scaled the excitatory depression by the magnitude before the pairing, 344 at the reference
  magnitude and 1 376 at the rail. It left the potentiation additive so that a night's replay still reaches the rail,
  and among its reasons rejected the $\mu = 1$ potentiation for moving that pin. It reads the rule under stationary
  pairing and a modulation at or above zero; it predates the signed gate and says nothing of a trace consolidated
  below zero.
- **ADR-0093** (2026-09-24) chose the signed gate and wrote what it does to a negative trace: a punishment potentiates
  it, "the product of two negatives", the signed rule of Florian 2007 and Frémaux et al. 2010 and "not a defect". It
  spends the trace by what the magnitude moved, so a punished consolidation never grows the trace it reads, and it
  named how often the path runs as a reading for H-18, not as an argument.
- **ADR-0094** built it (`consolidate_against`) and recorded the same among its Bad consequences: under the
  gate an addressed synapse with a negative trace is potentiated by a punishment. The magnitude is clamped to
  $[0, 2^{15})$ as `consolidate` clamps it.
- **ADR-0096** read it over one reversal at the image's couplings: a punishment potentiated about a fifth of the
  addressed synapse-trials it reached and depressed slightly more. By amount the potentiations were 41 % of what a
  punishment moved, the depression outweighing them by about 1.4 to 1.
- **None reads the path near the rail**, where ADR-0055's depression is largest, or over more than one schedule.

**What the path carries under the default.** The four couplings over the four schedules from the assignment, read
again on this branch, which gave 007's figures to the unit and the bands beside them, in rails of weight summed over
the four couplings:

| Schedule | Trials after a punishment | After a reward: gained, lost, net | After a punishment: gained, lost, net | Net | Punishments' gain at or above 3/4 of the rail |
| ---: | ---: | :--- | :--- | ---: | ---: |
| 1 | 33 % | 925, 486, +439 | 382, 744, −363 | +77 | 8 |
| 2 | 31 % | 910, 435, +476 | 349, 774, −424 | +52 | 13 |
| 3 | 27 % | 837, 355, +482 | 307, 756, −450 | +33 | 20 |
| 4 | 24 % | 776, 300, +476 | 257, 684, −426 | +49 | 25 |

The net includes the first trial of each schedule, which follows the lead-in, under 0.4 rails in each.

- **By amount the path is large**: a punishment's potentiation of negative traces was 25 to 29 % of all the weight
  the couplings gained, and 5.0 to 9.4 times a schedule's net rise.
- **By its net it is a loss.** What a punishment took off was 1.9 to 2.7 times what it gave, so the punishments netted
  −363 to −450 rails in every schedule. The rise is the rewards' surplus.
- **Near the rail it is small.** Of the punishments' potentiation, 2 to 10 % fell on synapses at or above three
  quarters of the rail, against 13 to 23 % of the rewards'. After a punishment 25 to 122 synapses a schedule rose to
  seven eighths or above, against 337 to 683 after a reward; and 92 to 119 reached the rail itself, against 2 619 to
  4 739, 2 to 4 % of the arrivals as 007 read.
- **So 007's reading holds and is narrower than it sounded.** Under the default the reward path carries the synapses to
  the top of the width and the punishment path takes back more than it gives. Under a bound on the pairing the reward
  path no longer reaches the rail, and the punishment path, a tail under the default, is what is left: under each of
  the three pairing bounds every arrival at the rail followed a punishment.

## What it suggests

**A bound at consolidation from zero drains the couplings rather than bounding them.** It is 007's bound over the width
moved from the pairing to consolidation: every rise in the bulk is scaled and no fall is, so the couplings sank to
where the rewards' scaled gains met the losses, at a tenth to three fifths of the image's, and learning went with
them. It kept every synapse off the rail, the punishment path's included, by keeping all of them far below it.

**Normalised at the reference on the pairing, the bulk kept nearer the rule, and the top did not.** The rise slowed by
about a third to a half but passed 1.30 by the fourth schedule, and from the third schedule the network held one mapping
and stopped reversing. Of the four bounds 007 and 009 have tried, none held the couplings at 1.30 with every mapping
learned.

**Where the rise comes from is the reward path at the top of the width.** Under the default the signed gate's
potentiation of negative traces is a net loss and small near the rail. A mechanism aimed at that path alone would not
hold the default's couplings. Questions this suggests for a later exploration or a round:
- a bound at consolidation normalised at the reference magnitude, whole below it, so that it reaches the rises at the
  top whichever the signal's sign and leaves the bulk's consolidation the rule's;
- whether the reversal near the rail under a pairing bound, a reward lowering a weight and a punishment raising it, is
  what stops the network reversing, read per synapse against the pair the mapping rewards;
- ADR-0055 kept the potentiation additive so that a night's replay reaches the rail. A bound at consolidation would
  move that pin as the $\mu = 1$ potentiation would have; the nights were not run here.

## What did not run

- **Item 4, sixteen schedules.** Neither bound met its condition: at consolidation the couplings stayed under 1.30
  and the mappings were not learned, and above the reference neither held. The eighth images were printed, CRC-64 `0x40fef71b2dd8c056` and
  `0xa642856736500334` at consolidation and `0x8abb21285be03bb5` and `0x947d8510de5d4327` above the reference, and are
  in the run's logs; the branch keeps none, and its second halves refuse to start until one is written down.
- **Seeds 23 and 24.**
- **Other forms of the bound at consolidation**: the rest of a rise kept pending in the trace rather than lost; on the
  inhibitory synapses too; normalised at the reference; on a signal below zero alone; and either bound beside the
  other.
- **The rail paths from the mirrored assignment under the default**, or past its fourth schedule.
- **The rate's error**, and the inhibitory rule's replay terms, which the logs hold and this note does not read.
- **The nights** of ADR-0044 and ADR-0048 under either bound.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `a_bound_at_consolidation_short_on_seed_22_explore` | [40508746883](https://github.com/Menteion/Menteion/actions/runs/40508746883) | `6f106f2` | 242 |
| `a_bound_at_consolidation_on_seed_22_from_the_assignment_explore` | [40509719948](https://github.com/Menteion/Menteion/actions/runs/40509719948) | `6f106f2` | 7300 |
| `a_bound_at_consolidation_on_seed_22_from_the_mirrored_assignment_explore` | [40509719948](https://github.com/Menteion/Menteion/actions/runs/40509719948) | `6f106f2` | 7411 |
| `a_bound_above_the_reference_on_seed_22_from_the_assignment_explore` | [40509719948](https://github.com/Menteion/Menteion/actions/runs/40509719948) | `6f106f2` | 7414 |
| `a_bound_above_the_reference_on_seed_22_from_the_mirrored_assignment_explore` | [40509719948](https://github.com/Menteion/Menteion/actions/runs/40509719948) | `6f106f2` | 7356 |
| `where_the_couplings_rise_under_the_default_on_seed_22_explore` | [40509719948](https://github.com/Menteion/Menteion/actions/runs/40509719948) | `6f106f2` | 3686 |

Exploration 007's rail paths were read again from run
[40479387831](https://github.com/Menteion/Menteion/actions/runs/40479387831) on `0bbccf7`.
