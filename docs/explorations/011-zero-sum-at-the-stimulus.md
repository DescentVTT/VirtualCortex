---
status: exploratory
date: 2026-10-28
---

# 011: A zero sum at the stimulus

**Branch**: `explore/011-zero-sum-at-the-stimulus`
**Commit**: `b840fba` for the short run and the first halves; `2f6e563` for the second halves, which adds the four
images the first halves wrote and their CRC-64s and nothing else that runs. Both carry `6dd7252`, the merge of
exploration 010's tag with explorations 007's, 009's and 010's bounds and hold under `src/`, and `c8bf842`, this
exploration's zero sum under `src/`. Unset, each is the rule bit for bit.
**Runs**: [40614819069](https://github.com/Menteion/Menteion/actions/runs/40614819069) (the short run and the first
halves) and [40625863565](https://github.com/Menteion/Menteion/actions/runs/40625863565) (the second halves), both at
`scope=arms`

Explorations 007, 009 and 010 tried six ways to bound the couplings: four bounds on one synapse at a time, and a readout
unit's whole excitatory input held. None held the couplings at 1.30 of the image's with every mapping learned
([007](007-sixteen-schedules-and-a-soft-bound.md), [009](009-a-bound-at-consolidation.md),
[010](010-a-readouts-sum-held.md)). 009 read the default's rise as the rewards' surplus over what the punishments take.
010 read the couplings as an eighth of a readout unit's input: a hold over the whole input took most of their rise from
the other inputs, while the mappings' relative differences held at 0.21 to 0.27 of the image's coupling and learning
went on. This note asks whether a zero sum bounds the couplings while the relative differences and learning hold: each
stimulus set's couplings to the two readout sets, summed, held at their sum in the image, so that one readout's coupling
rises only as the other's falls. On seed 22:

1. both ways: after every consolidation that moves one of a stimulus set's synapses onto a readout unit, every such
   synapse of the set is scaled so that their sum is its value in the image, in Q1.15 integers, saturating, rounded
   down, at most the rail. No other synapse moves;
2. a ceiling: the same, but only where the sum stands above its value;
3. each from H-42's image over eight schedules, from both arms, read as 010 read them, with 009's rail paths, and what
   the zero sum costs;
4. if either held the couplings at or under 1.30 of the image's with every mapping learned over eight schedules, the
   same over sixteen, carried in two halves as 007 carried its long run.

The maintainers wrote one rule beside the question: if this failed too, no further bound would be tried, a decision ADR
would revise M12's criterion, and the main line would turn to M11. It writes no prediction and decides nothing. **Every
figure here is exploratory**, from one run of each test on the repository's AArch64 host.

**In brief**
- **Both zero sums held the couplings and learning, over sixteen schedules from both arms.** The highest coupling stood
  at 1.13 to 1.16 of the image's both ways and at 1.15 to 1.20 under the ceiling, never past 1.30; under the default it
  reached 1.64 to 1.66 in the eighth schedule and 2.06 to 2.07 in the sixteenth. Every mapping of all 64 schedules was
  learned, at 117 to 128 of its last 128, and H-25's four clauses held in every one. Of the eight forms 007, 009, 010
  and this note have tried, these are the first two to do both.
- **The mappings' relative differences were the default's.** At every mapping's last block each stimulus's coupling to
  the readout the mapping rewards stood above its coupling to the other, by 0.22 to 0.27 of the image's coupling on
  average and 0.12 at the smallest.
- **Both ways learning was faster than the default's, under the ceiling slower.** From the third schedule a schedule's
  three reversals summed 11 to 13 blocks both ways; the default's took 15 to 31 over schedules 3 to 8 and 12 to 16 over
  9 to 16, the ceiling's 21 to 33 and 16 to 24.
- **The inhibitory sum and the gain stopped.** Over schedules 9 to 16 the inhibitory sum stood at 0.513 to 0.535 of the
  image's, where under the default it turned and rose to 0.589 and 0.597; the gain stood still where the default's fell.
- **Both ways the couplings split.** By the eighth image 536 to 575 of a coupling's about 800 synapses stood in the
  bottom sixteenth of the width and about 200 in its top half, and up to 158 of a coupling stood at the rail at a
  block's end, more than the default's 100 by its eighth schedule; the split then held. A reading from the arithmetic,
  not measured: rounded down, a scale by a quotient near one takes an LSB from every weight above zero and gives one
  back only to the largest. Under the ceiling the tails grew slowly, to 95 synapses at the rail and 212 in the bottom
  sixteenth by the sixteenth schedule.
- **The zero sum is cheap.** A group's sum is its own 51 chains, 1 632 synapses, read in 5 to 12 µs a round; it cost
  0.20 to 0.64 s of a schedule's about 700.

## What ran

**The network, the arm and the path** are exploration 010's. Every test starts from H-42's network of seed 22 at 1 024
units, the image an engine born under `Config::reads_its_targets()` settles. Each test first runs that derivation again
and holds it to H-42's pinned tables (`probe_made`), and all nine held. Every first half printed the CRC-64 of that
image, `0x750105a595510fa3`. The arm is H-43's twin on time, H-42's arm bit for bit, from the assignment and from the
mirrored assignment. A schedule ends with the engine run quiet without the drive until it is quiescent; its image is
written, and the next schedule's engine is decoded from it.

**The harness** is exploration 010's as its tag holds it (`3058a60`), merged onto `main` at `60038e5`, where a unit is
served on arrival ([ADR-0225](../adr/0225-a-unit-served-on-arrival-built.md)); 007's, 009's and 010's bounds and hold
are unset here. 010's readings are taken in every schedule, 009's rail paths among them. Beside them:
- at every block's end, each stimulus set's couplings to the two readout sets, summed, against its value; the sets above
  it; and the readout units' excitatory input, summed, with the four couplings' share of it;
- what the zero sum cost, from counters in the engine.

**The zero sum, under `src/` on this branch alone.** Commit `c8bf842` adds `ZeroSum` and `Executor::set_zero_sum`:
- **A group** is one stimulus set's excitatory units, 51 for each set. **The targets** are the readout units of both
  sets, excitatory and inhibitory, 918 of the 1 024: the synapses the harness counts as the couplings. Of a set's 1 632
  synapses, 1 581 for A and 1 607 for B end on a readout unit.
- **A group's value** is those synapses' weights summed in H-42's image: 12 470 305 for A and 12 922 530 for B, 380.6
  and 394.4 rails: the image's A→R0 and A→R1, and its B→R0 and B→R1, summed, to the unit. Every schedule is held at
  H-42's values, not at those of the image it starts from.
- **When.** A worker whose consolidation moves the weight of a group's synapse onto a target marks the group. After the
  tick's last barrier and before the next tick, the coordinator walks the marked group's units' chains and sums the
  weights onto the targets. The tree keeps a synapse with its presynaptic unit, so a group's sum is its own 51 chains,
  1 632 synapses, and no scan. A synapse's release in the tick of its consolidation is made before the hold.
- **Both ways** (`ZeroSum::Both`): a sum $S$ that is not its value $V$ has every one of the group's synapses onto a
  target set to $\lfloor w V / S \rfloor$, in integers, within the rail.
- **The ceiling** (`ZeroSum::Ceiling`): the same where $S$ stands above $V$; a sum at or below its value is left.
- **No other synapse moves**, and the traces are left as they are.
- **It is in no image**, so the test sets it on every schedule's engine. Unset, nothing is marked and the tick is the
  rule's bit for bit: the short run's unheld schedule held H-42's arm as far as it ran and wrote the image 007's, 009's
  and 010's branches wrote, CRC-64 `0x841f164eb6fdefb2`. Two unit tests hold a group's round on a hand-made arena and
  what `set_zero_sum` refuses.
- **Held at every block's end** of every schedule of every arm: under either zero sum, no set's sum stood above its
  value.

**Exploration 010's hold, as this branch carries it.** Its round read the unit arena through `arenas_mut` inside the
tick. Under [ADR-0224](../adr/0224-a-unit-served-on-arrival.md) that brings the asleep units up to date before the clock
moves, so on this branch it walks the excitatory units' chains, read when the hold is set, as the zero sum does. A
unit's chain is set when the network is made and never moves. It did not run here.

**Sixteen schedules in two halves**, as exploration 007 carried its long run. Each first half started from H-42's image
and printed its eighth image whole, in hexadecimal. The branch keeps the four images in
`runtime/menteion-runtime/tests/explore-011/`, each decoded from its run's log and held to the CRC-64 printed beside it.
Each second half read its image, held it to that CRC-64, decoded it and ran schedules 9 to 16.

## 1 and 2. The two zero sums over eight schedules

Each cell lists the eight schedules in order. The default's rows are exploration 005's, as 007, 009 and 010 tabled them:

| Rule | Arm | Highest coupling | Lowest coupling | Lowest of the mappings' last 128 correct | Three reversals summed, blocks |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Default | assignment | 1.239, 1.350, 1.428, 1.461, 1.520, 1.592, 1.619, 1.658 | 0.903, 0.930, 0.985, 1.036, 1.090, 1.151, 1.207, 1.286 | 120, 121, 124, 126, 126, 127, 124, 127 | 34, 27, 26, 21, 20, 17, 17, 15 |
| Default | mirrored | 1.248, 1.298, 1.374, 1.431, 1.444, 1.521, 1.564, 1.637 | 0.889, 0.956, 0.970, 1.018, 1.070, 1.115, 1.150, 1.179 | 120, 122, 123, 123, 126, 122, 126, 126 | 40, 33, 31, 22, 24, 20, 20, 16 |
| Both ways | assignment | 1.150, 1.159, 1.141, 1.141, 1.141, 1.141, 1.149, 1.154 | 0.863, 0.855, 0.862, 0.863, 0.842, 0.842, 0.833, 0.828 | 123, 125, 122, 124, 125, 124, 123, 125 | 33, 17, 13, 12, 12, 12, 12, 12 |
| Both ways | mirrored | 1.146, 1.130, 1.133, 1.135, 1.139, 1.145, 1.143, 1.147 | 0.860, 0.858, 0.852, 0.851, 0.855, 0.861, 0.855, 0.860 | 121, 124, 123, 125, 125, 124, 127, 125 | 37, 21, 13, 13, 12, 12, 12, 12 |
| Ceiling | assignment | 1.159, 1.175, 1.175, 1.198, 1.169, 1.177, 1.177, 1.171 | 0.841, 0.832, 0.832, 0.822, 0.843, 0.838, 0.837, 0.833 | 121, 121, 119, 122, 125, 126, 126, 124 | 34, 36, 33, 28, 28, 22, 24, 21 |
| Ceiling | mirrored | 1.150, 1.159, 1.150, 1.153, 1.145, 1.149, 1.172, 1.170 | 0.865, 0.850, 0.860, 0.851, 0.844, 0.821, 0.832, 0.832 | 121, 117, 121, 126, 123, 125, 122, 125 | 40, 34, 30, 31, 30, 26, 25, 24 |

The couplings are over every block's end of the schedule and the four pairs, over the first image's; a mapping is
learned at 80 or more of its last 128, and every mapping of every schedule here was.

**H-25's four clauses held in every schedule of the four arms**: clause 2, no coupling past 1.30, among them, where
under the default it failed from the second schedule and the third.

Beside them, at each schedule's last block, over the first image's, and the most synapses of one coupling at the rail or
at or above seven eighths of it at a block's end:

| Rule | Arm | Inhibitory sum | Gain | Mean weight over the rail | At the rail, most | At or above 7/8, most |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Default | assignment | 0.743 to 0.556 | 1.7062 to 1.6804 | 0.23–0.30 to 0.32–0.41 | 26, 37, 61, 55, 80, 85, 98, 100 | 42 to 130 |
| Both ways | assignment | 0.734, 0.597, 0.562, 0.548, 0.544, 0.540, 0.535, 0.535 | 1.7050 to 1.6797 | 0.21–0.28 to 0.19–0.29 | 37, 89, 121, 126, 138, 158, 147, 149 | 51 to 201 |
| Both ways | mirrored | 0.735, 0.594, 0.556, 0.543, 0.536, 0.532, 0.529, 0.527 | 1.7032 to 1.6792 | 0.21–0.28 to 0.22–0.27 | 44, 87, 103, 126, 126, 139, 153, 142 | 59 to 198 |
| Ceiling | assignment | 0.740, 0.594, 0.553, 0.537, 0.530, 0.525, 0.522, 0.519 | 1.7093 to 1.7010 | 0.21–0.28 to 0.20–0.27 | 26, 26, 29, 34, 46, 55, 58, 66 | 31 to 84 |
| Ceiling | mirrored | 0.740, 0.595, 0.552, 0.535, 0.525, 0.520, 0.517, 0.514 | 1.7090 to 1.7010 | 0.22–0.27 to 0.21–0.28 | 28, 33, 35, 36, 43, 48, 40, 42 | 37 to 79 |

The mean weight is the range over the four couplings, in the first schedule and in the eighth; the first image's was
0.23 to 0.25.

**The sums.** Each stimulus set's couplings to the two readout sets, summed, over its value:
- **Both ways** every set's sum stood at 0.9998 to 0.9999 of its value at every block's end, short of it only by the
  rounding. The four couplings summed to 775 rails throughout, the image's.
- **Under the ceiling** a sum is cut when it rises past its value and left when it falls. At each schedule's last block
  the sets stood at 0.970 to 1.000 of their values, and at 0.892 at the lowest block's end. The four couplings summed to
  761 to 775 rails at a schedule's last block.
- **The couplings' share of the readouts' input stayed the image's**: 13.2 % both ways and 13.0 to 13.2 % under the
  ceiling, where 010's holds raised it to 17.5 to 19.2 %. The readout units' other excitatory inputs, which no zero sum
  scales, stood at 5 109 to 5 113 rails at every schedule's last block, the image's 5 111; 010's holds took 7 to 9 % of
  them.

**Where the rise went.** Over the four couplings, in rails a schedule, the weight the trials after a reward and after a
punishment moved, as 009 read them:
- **Both ways** the rewards and the punishments each netted −2 to +2 rails a schedule: a sum is put back at its value in
  the tick after the consolidation that moved it, so no surplus is left to read at a trial's end. The trials after a
  reward still moved 534 to 838 rails up and as many down.
- **Under the ceiling** the rewards netted +363 to +466 rails a schedule and the punishments −365 to −467, beside the
  default's +439 to +482 and −363 to −450 over its first four schedules from the assignment. A schedule's net was −6 to
  +5 rails: what the rewards add above the value is cut, and what the punishments take is left.
- The share of punished trials fell from 30 to 33 % in the first schedule to 14 to 15 % in the eighth both ways, and
  from 34 to 35 % to 22 to 24 % under the ceiling.

**Where the weight went inside a coupling.** The couplings' synapses by sixteenths of the width at each image's end,
from the assignment. The mirrored arm's are of the same size, its top sixteenth on the two pairs its last mapping
rewarded:

| Rule | Image | In the bottom sixteenth, by pair | In the top half, by pair | In the top sixteenth, by pair |
| :--- | ---: | :--- | :--- | :--- |
| H-42's image | 0 | 7, 9, 4, 2 | 10, 13, 10, 17 | 0, 0, 0, 0 |
| Both ways | 2 | 318, 273, 190, 259 | 136, 165, 141, 105 | 5, 97, 83, 10 |
| Both ways | 4 | 492, 482, 431, 479 | 178, 217, 197, 188 | 0, 142, 143, 11 |
| Both ways | 8 | 567, 550, 536, 575 | 193, 233, 220, 215 | 0, 174, 159, 12 |
| Ceiling | 4 | 58, 20, 32, 59 | 36, 85, 94, 25 | 0, 47, 27, 0 |
| Ceiling | 8 | 80, 40, 43, 106 | 63, 97, 91, 53 | 0, 75, 53, 0 |

The pairs are A→R0, A→R1, B→R0 and B→R1, of 775 to 809 synapses each.

- **Both ways the couplings split into two groups.** By the eighth image 536 to 575 of a coupling's synapses stood in
  the bottom sixteenth of the width and 193 to 233 in its top half; the image had 2 to 9 and 10 to 17. The mean weight
  held, as the sum did. Under the ceiling the bottom sixteenth held 40 to 106.
- **By the band a synapse stood in before it moved**, the trials after a reward moved 71 to 166 rails a schedule out of
  the synapses below ADR-0055's reference magnitude and 107 to 172 into those at or above three quarters of the rail, in
  the first three schedules both ways; the trials after a punishment moved 88 to 151 out of the top. By the eighth
  schedule each band's net was within 35 rails of zero. Under the ceiling a reward's net was above zero in every band
  and a punishment's below.
- **A reading of why, from the arithmetic and not measured: the rounding.** Both ways a group was scaled in all but a
  few of its rounds, 33 120 to 61 473 a schedule, each after a tick's consolidations moved its sum of about 12.5 million
  by far less than the sum, so each quotient stood near one. Scaled down to $\lfloor w V / S \rfloor$, a weight $w$
  above zero with $w (S - V) / S$ below one loses exactly one LSB; scaled up, it gains one only once $w (V - S) / S$
  reaches one, so only the largest weights gain. Every round down takes an LSB from every synapse above zero, and the
  rounds up return it to the largest: a drain on the small weights and a lift on the large ones that the rule does not
  make. Under the ceiling a group was scaled in 478 to 1 117 rounds a schedule, all of them down. How much of the split
  is the rounding and how much the zero sum was not run apart.

**The rail and the signal's sign**, the four couplings' arrivals at the rail per schedule from the assignment, after a
reward and after a punishment:

| Rule | After a reward | After a punishment |
| :--- | :--- | :--- |
| Default, schedules 1 to 4 | 2 619, 3 321, 3 287, 4 739 | 92, 119, 109, 106 |
| Both ways | 8 974, 23 023, 36 478, 45 182, 48 179, 47 490, 47 798, 47 174 | 1 330, 3 799, 7 096, 9 498, 12 084, 11 258, 11 085, 12 704 |
| Ceiling | 2 979, 2 608, 2 941, 4 417, 4 871, 8 354, 8 530, 9 480 | 121, 149, 125, 106, 213, 140, 263, 133 |

Under the ceiling 1 to 5 % of the arrivals followed a punishment, as under the default's 2 to 4 %. Both ways 13 to 21 %
did, and a synapse arrived and left again many times: in the eighth schedule from the assignment at most 149 of a
coupling stood at the rail at a block's end, against 47 174 arrivals after a reward. A reading, not measured here: a
round that scales a group up lifts the synapses near the rail onto it, whatever the sign of the trial's signal.

## 3. The mappings' relative differences

At each mapping's last block, per stimulus, its coupling to the readout the mapping in force rewards less its coupling
to the other, over the image's couplings: four mappings and two stimuli, eight differences a schedule. Every one of the
eight, in every schedule of the four arms, favoured the mapping in force. Their mean and their smallest, per schedule:

| Rule | Arm | Mean | Smallest |
| :--- | :--- | :--- | :--- |
| Default | assignment | 0.22, 0.23, 0.23, 0.24, 0.25, 0.26, 0.26, 0.27 | 0.13, 0.14, 0.13, 0.14, 0.16, 0.18, 0.15, 0.18 |
| Default | mirrored | 0.22, 0.23, 0.24, 0.23, 0.24, 0.25, 0.26, 0.26 | 0.15, 0.14, 0.13, 0.12, 0.13, 0.14, 0.15, 0.14 |
| Both ways | assignment | 0.23, 0.24, 0.24, 0.22, 0.23, 0.24, 0.22, 0.24 | 0.16, 0.16, 0.17, 0.17, 0.18, 0.19, 0.15, 0.17 |
| Both ways | mirrored | 0.23, 0.24, 0.22, 0.23, 0.23, 0.23, 0.24, 0.23 | 0.18, 0.21, 0.18, 0.17, 0.18, 0.19, 0.18, 0.18 |
| Ceiling | assignment | 0.23, 0.24, 0.24, 0.24, 0.24, 0.24, 0.25, 0.25 | 0.14, 0.12, 0.15, 0.14, 0.15, 0.15, 0.17, 0.16 |
| Ceiling | mirrored | 0.23, 0.23, 0.23, 0.24, 0.23, 0.24, 0.24, 0.25 | 0.17, 0.15, 0.17, 0.18, 0.17, 0.15, 0.13, 0.15 |

The default's are 010's, read from exploration 005's run.

- **The differences were the default's** in the image's couplings, under both zero sums, while the couplings stayed
  within 0.82 to 1.20 of the image's. In the eighth schedule they were 23 to 25 % of the four couplings' mean, against
  18 % under the default.
- **Both ways the differences did not grow**, at 0.22 to 0.24 in every schedule, where under the default they rose from
  0.22 to 0.27. Under the ceiling they rose to 0.25.

## What the zero sum costs

A group's sum is a walk of its own units' chains: 51 units, 1 632 synapses, of which 1 581 or 1 607 end on a readout
unit. A round walked one group in all but at most two rounds of a schedule, since a trial presents one stimulus.

| Rule | Arm | Rounds a schedule | Rounds that scaled | Synapses rewritten a round that scaled | µs a round | Seconds a schedule |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Both ways | assignment | 59 346, 33 154 | all but 17 to 34 | 338 to 421 | 10.5 to 11.6 | 0.62, 0.38 |
| Both ways | mirrored | 61 485, 31 871 | all but 12 to 31 | 344 to 424 | 10.3 to 11.6 | 0.64, 0.37 |
| Ceiling | assignment | 65 728, 41 717 | 1 017, 583 | 1 585 to 1 593 | 5.4 to 5.6 | 0.36, 0.23 |
| Ceiling | mirrored | 67 300, 45 185 | 1 117, 720 | 1 588 to 1 597 | 5.6 to 6.0 | 0.39, 0.25 |

Each cell is the first schedule's and the eighth's, or the range over the eight.

- **A round reads 1 632 synapses where 010's read 26 240**, in 5 to 12 µs where 010's took 67 to 86. A round that scales
  rewrites about a quarter of the group's synapses both ways, and nearly all of them under the ceiling, where a sum is
  cut only once it stands above its value.
- **Rounds ran in 0.025 to 0.053 % of a schedule's 125 829 620 ticks**, under the reward's gate as in 010, and fell over
  the schedules with the share of punished trials.
- **The zero sum cost 0.23 to 0.64 s of a schedule's about 700.** The first halves took 5 636 to 5 876 s for eight
  schedules beside the derivation, where 010's arms took 7 291 to 7 426 under per-tick service: a unit is served on
  arrival on this branch ([ADR-0225](../adr/0225-a-unit-served-on-arrival-built.md)).

## 4. Sixteen schedules

Both zero sums met item 4's condition from both arms, so each ran schedules 9 to 16 from its eighth image. Each cell
lists schedules 9 to 16 in order. The default's rows are exploration 007's:

| Rule | Arm | Highest coupling | Lowest coupling | At the rail, most | Lowest of the mappings' last 128 correct | Three reversals summed, blocks |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Default | assignment | 1.702, 1.786, 1.839, 1.878, 1.928, 2.001, 2.018, 2.072 | 1.296, 1.363, 1.411, 1.436, 1.471, 1.495, 1.533, 1.572 | 125, 133, 135, 148, 152, 159, 162, 186 | 127, 127, 127, 127, 127, 126, 128, 128 | 15, 15, 13, 15, 13, 14, 13, 13 |
| Default | mirrored | 1.682, 1.737, 1.783, 1.835, 1.915, 1.983, 2.019, 2.061 | 1.222, 1.329, 1.360, 1.397, 1.434, 1.482, 1.522, 1.569 | 104, 127, 131, 143, 166, 156, 165, 170 | 127, 126, 127, 126, 125, 127, 128, 127 | 16, 13, 16, 15, 13, 14, 12, 12 |
| Both ways | assignment | 1.152, 1.144, 1.149, 1.153, 1.149, 1.154, 1.153, 1.153 | 0.830, 0.840, 0.848, 0.842, 0.843, 0.851, 0.845, 0.848 | 141, 140, 142, 154, 145, 142, 157, 140 | 125, 124, 124, 124, 127, 125, 123, 125 | 13, 12, 12, 12, 11, 13, 12, 12 |
| Both ways | mirrored | 1.138, 1.148, 1.148, 1.147, 1.148, 1.150, 1.155, 1.147 | 0.861, 0.860, 0.860, 0.860, 0.858, 0.861, 0.856, 0.867 | 144, 139, 145, 149, 143, 138, 143, 145 | 125, 126, 123, 126, 126, 124, 124, 125 | 12, 12, 12, 12, 12, 12, 13, 12 |
| Ceiling | assignment | 1.161, 1.150, 1.164, 1.187, 1.187, 1.176, 1.175, 1.182 | 0.825, 0.804, 0.802, 0.804, 0.806, 0.809, 0.819, 0.820 | 69, 67, 83, 76, 64, 79, 76, 95 | 128, 126, 126, 125, 124, 124, 127, 126 | 19, 17, 16, 17, 18, 18, 18, 16 |
| Ceiling | mirrored | 1.186, 1.186, 1.190, 1.189, 1.189, 1.188, 1.200, 1.202 | 0.831, 0.814, 0.807, 0.828, 0.822, 0.825, 0.807, 0.798 | 61, 65, 52, 58, 81, 77, 85, 89 | 124, 124, 126, 127, 126, 125, 127, 126 | 22, 24, 22, 20, 19, 16, 18, 18 |

- **Both ways nothing moved.** The highest coupling stood at 1.138 to 1.155 over schedules 9 to 16, against 1.130 to
  1.159 over the first eight, and the lowest at 0.830 to 0.867; the most synapses of a coupling at the rail at 138 to
  157, at or above seven eighths at 201 to 215. The couplings' split held: 555 to 582 of a coupling's synapses in the
  bottom sixteenth at the twelfth image and at the sixteenth, against 536 to 575 at the eighth.
- **Under the ceiling the highest coupling stood at 1.150 to 1.202**, and from the mirrored assignment reached its
  highest, 1.200 and 1.202, in the fifteenth and the sixteenth schedule. The tails grew: at most 95 and 89 synapses of a
  coupling at the rail in the sixteenth schedule, against 66 and 42 in the eighth, and 59 to 212 in the bottom sixteenth
  at the sixteenth image, against 24 to 106 at the eighth.
- **Learning went on.** Every mapping of all 64 schedules was learned, at 117 to 128 of its last 128, and H-25's four
  clauses held in every one. From the ninth schedule the three reversals summed 11 to 13 blocks both ways, against the
  default's 12 to 16, and 16 to 24 under the ceiling.
- **The inhibitory sum stopped.** At the last block it stood at 0.533 to 0.535 and 0.520 to 0.526 both ways and at 0.513
  to 0.518 under the ceiling over schedules 9 to 16, where under the default it turned and rose from 0.559 to 0.597 and
  0.589 (007).
- **The gain stopped too**: 1.675 to 1.681 both ways and 1.692 to 1.698 under the ceiling, where the default's fell by
  about 0.003 a schedule to 1.654 and 1.656.
- **The rest as in the first half**: each set's sum at 0.9997 to 1.0000 of its value both ways and at 0.885 to 1.000
  under the ceiling at every block's end; the couplings' share of the readouts' input 12.9 to 13.2 %; the differences
  between a mapping's two couplings 0.22 to 0.27 on average and 0.12 at the smallest, every one favouring the mapping in
  force; the punished trials 14 to 16 % both ways and 18 to 25 % under the ceiling.

## What it suggests

**A zero sum at the stimulus is the first form tried here that bounds the couplings with every mapping learned**, on
seed 22, from both arms, over sixteen schedules. Its couplings stayed within 0.798 to 1.202 of the image's, the
mappings' differences were the default's, the inhibitory sum stopped, and both ways learning was faster than the
default's. The maintainers' rule for a failure did not arise on these readings. A verdict is a measured round's: a
hypothesis written before its runs, on H-42's three networks.

**The two forms read differently**, and a round would choose one:
- **Both ways** held every set's sum at its value, the couplings flat throughout, and learning fastest; and it split
  each coupling into a few strong synapses and many near zero, with more synapses at the rail than the default had by
  its eighth schedule. F-86 reads synapses at the rail, and both ways stood more of them there than the default.
- **The ceiling** left the bulk of each coupling nearer the image's and learning slower than the default's, while its
  tails at the rail and near zero were still growing at the sixteenth schedule, and the highest coupling from the
  mirrored assignment reached 1.20 there.

Questions this suggests for a later exploration or a round:
- the zero sum both ways rounded to nearest, or with each group's remainder carried to its next round, to tell the
  rounding's part of the split from the zero sum's;
- seeds 23 and 24, and the ceiling past sixteen schedules;
- whether the split costs anything a later task needs: a stimulus set reached the readouts through about 200 strong
  synapses of a coupling's 800;
- where the groups and the targets would live. Here the test names them, the task's stimulus and readout sets, as the
  task's selection names the address's target side ([ADR-0146](../adr/0146-the-tag-the-address-already-is.md)).

## What did not run

- **Seeds 23 and 24.**
- **More than sixteen schedules.**
- **Other forms of the zero sum**: rounded to nearest or with the remainder carried; held at each schedule's starting
  image rather than H-42's; with the traces scaled beside the weights; over every excitatory unit's outputs rather than
  the stimulus sets' couplings.
- **The rail paths under the default from the mirrored assignment**, or past its fourth schedule.
- **The rate's error**, and the inhibitory rule's replay terms, which the logs hold and this note does not read.
- **The nights** of ADR-0044 and ADR-0048 under the zero sum.
- **Exploration 010's hold as this branch carries it**: its round, changed to walk the chains it reads when it is set,
  is held by its unit test and was not run.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `a_zero_sum_at_the_stimulus_short_on_seed_22_explore` | [40614819069](https://github.com/Menteion/Menteion/actions/runs/40614819069) | `b840fba` | 184 |
| `a_zero_sum_at_the_stimulus_both_ways_on_seed_22_from_the_assignment_explore` | [40614819069](https://github.com/Menteion/Menteion/actions/runs/40614819069) | `b840fba` | 5857 |
| `a_zero_sum_at_the_stimulus_both_ways_on_seed_22_from_the_mirrored_assignment_explore` | [40614819069](https://github.com/Menteion/Menteion/actions/runs/40614819069) | `b840fba` | 5636 |
| `a_zero_sum_at_the_stimulus_as_a_ceiling_on_seed_22_from_the_assignment_explore` | [40614819069](https://github.com/Menteion/Menteion/actions/runs/40614819069) | `b840fba` | 5639 |
| `a_zero_sum_at_the_stimulus_as_a_ceiling_on_seed_22_from_the_mirrored_assignment_explore` | [40614819069](https://github.com/Menteion/Menteion/actions/runs/40614819069) | `b840fba` | 5876 |
| `a_zero_sum_at_the_stimulus_both_ways_second_half_on_seed_22_from_the_assignment_explore` | [40625863565](https://github.com/Menteion/Menteion/actions/runs/40625863565) | `2f6e563` | 5483 |
| `a_zero_sum_at_the_stimulus_both_ways_second_half_on_seed_22_from_the_mirrored_assignment_explore` | [40625863565](https://github.com/Menteion/Menteion/actions/runs/40625863565) | `2f6e563` | 5596 |
| `a_zero_sum_at_the_stimulus_as_a_ceiling_second_half_on_seed_22_from_the_assignment_explore` | [40625863565](https://github.com/Menteion/Menteion/actions/runs/40625863565) | `2f6e563` | 5497 |
| `a_zero_sum_at_the_stimulus_as_a_ceiling_second_half_on_seed_22_from_the_mirrored_assignment_explore` | [40625863565](https://github.com/Menteion/Menteion/actions/runs/40625863565) | `2f6e563` | 5708 |

Exploration 005's eight schedules, the default's rows, were read again from run
[40300651755](https://github.com/Menteion/Menteion/actions/runs/40300651755) on `056c76b`; schedules 9 to 16 of the
default are exploration 007's.
