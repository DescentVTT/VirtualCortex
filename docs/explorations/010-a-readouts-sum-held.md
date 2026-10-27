---
status: exploratory
date: 2026-10-28
---

# 010: A readout's sum held

**Branch**: `explore/010-a-readouts-sum-held`
**Commit**: `3058a60` for every run. It carries `7272fbc` and `8f66f74`, explorations 007's and 009's bounds under
`src/` as 009's tag holds them, and `f3bd5fb`, this exploration's hold under `src/`. Unset, each is the rule bit for bit.
**Runs**: [40563989689](https://github.com/Menteion/Menteion/actions/runs/40563989689) (the short run and the four
arms), at `scope=arms`

Explorations 007 and 009 tried four bounds, each on one synapse at a time: on the pairing's potentiation over the whole
width, in its top eighth and above the reference magnitude, and at consolidation. Each either lost learning with the
couplings or did not hold them ([007](007-sixteen-schedules-and-a-soft-bound.md),
[009](009-a-bound-at-consolidation.md)). 009 read the default's rise as the rewards' surplus: a schedule's rewards
netted +439 to +482 rails over the four couplings, its punishments −363 to −450, and learning was unharmed over sixteen
schedules. This note asks whether holding each readout unit's excitatory input sum at its value in the image, all of
the unit's inputs scaled in proportion, bounds the couplings while the mappings keep their relative differences and
learning goes on. On seed 22:

1. a ceiling: after every consolidation that touches a readout unit, a sum above its value has every excitatory input
   of the unit scaled by the value over the sum, in Q1.15 integers, saturating, rounded down; below its value nothing
   moves. And what reading the sum costs, since the tree keeps a synapse with its presynaptic unit;
2. the same both ways: a sum below its value scaled up likewise, at most the rail;
3. each from H-42's image over eight schedules, from both arms, read as 009 read them, and beside them the difference
   between the two couplings each stimulus has, for whether the relative differences hold;
4. if either held the couplings at or under 1.30 of the image's with every mapping learned over eight schedules, the
   same over sixteen.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host.

**In brief**
- **The hold held the sums and not the couplings.** Under the ceiling the highest coupling rose to 1.42 and 1.44 of the
  image's in the eighth schedule, past 1.30 in the fourth and the fifth. Both ways it rose to 1.66 and 1.69, where the
  default's reached 1.66 and 1.64. The readout units' sums stood at 0.952 to 0.954 of their values under the ceiling
  and at their values both ways.
- **The couplings are an eighth of a readout unit's input, and they took more of it.** The four couplings carried
  13.2 % of the readout units' excitatory input in the image, and 17.5 and 17.6 % under the ceiling and 18.9 and
  19.2 % both ways in the eighth schedule. The readouts' other excitatory inputs fell from 5 111 rails to 4 624 to 4 771.
- **Learning went on.** Every mapping of all 32 schedules was learned, at 103 to 128 of its last 128. The reversals were
  slower than the default's: a schedule's three summed 18 to 42 blocks under the ceiling and 22 to 58 both ways,
  against 15 to 40.
- **The mappings' relative differences held.** At every mapping's last block of every schedule, each stimulus's
  coupling to the readout the mapping rewards stood above its coupling to the other, under both holds. The differences
  averaged 0.21 to 0.27 of the image's coupling a schedule, the default's 0.22 to 0.27.
- **The scan is cheap here because it is rare.** A round reads all 26 240 excitatory synapses of the arena, in 67 to
  86 µs on the host. Under the reward's gate an excitatory weight onto a readout unit moves in 0.03 to 0.06 % of the
  ticks, so a schedule ran 41 835 to 73 108 rounds, 3.0 to 5.4 s of about 900.
- **Item 4 did not run**: neither hold met its condition.

## What ran

**The network, the arm and the path** are exploration 009's. Every test starts from H-42's network of seed 22 at 1 024
units, the image an engine born under `Config::reads_its_targets()` settles. Each test first runs that derivation again
and holds it to H-42's pinned tables (`probe_made`), and all five held. Every arm printed the CRC-64 of that image,
`0x750105a595510fa3`. The arm is H-43's twin on time, H-42's arm bit for bit, from the assignment and from the mirrored
assignment. A schedule ends with the engine run quiet without the drive until it is quiescent; its image is written,
and the next schedule's engine is decoded from it.

**The harness** is exploration 009's as its tag holds it (`6f106f2`), carried to this branch with 007's and 009's
commits under `src/`, all unset here: exploration 005's readings, 007's summary per schedule, and 009's rail paths in
every schedule of every arm. Beside them:
- at every block's end, each readout unit's sum against its value: how many stood above it, all of them summed over
  their values summed, and the lowest and the highest unit's;
- at each mapping's last block, the four couplings over the image's, and per stimulus the coupling the assignment
  rewards less the one the mirrored assignment rewards, A→R0 − A→R1 and B→R1 − B→R0;
- what the hold cost, from counters in the engine.

**The hold, under `src/` on this branch alone.** Commit `f3bd5fb` adds `SumHold` and `Executor::set_sum_hold`:
- **The units held** are the readout units, both readout sets, excitatory and inhibitory: 918 of the 1 024. The
  couplings are the stimulus units' synapses onto both kinds.
- **A unit's value** is its excitatory input sum in H-42's image: every synapse from an excitatory unit onto it, the
  stimulus units' and the recurrent ones'. The values ran from 82 720 to 437 270, 2.5 to 13.3 rails, over 11 to 42
  inputs a unit; 23 605 of the arena's 26 240 excitatory synapses end on a readout unit. Every schedule is held at
  H-42's values, not at those of the image it starts from.
- **When.** A worker that consolidates an excitatory synapse and moves its weight marks the target, if it is held.
  After the tick's last barrier and before the next tick, the coordinator reads the sum of every unit marked and holds
  it. A synapse's release in the tick of its consolidation is made before the hold, from the weight consolidation left.
- **The ceiling** (`SumHold::Ceiling`): a sum $S$ above its value $V$ has every excitatory synapse onto the unit set to
  $\lfloor w V / S \rfloor$, in integers, within the rail. A sum at or below its value is left.
- **Both ways** (`SumHold::Both`): the same, and a sum below its value scaled up by the same quotient, rounded down,
  each weight at most the rail.
- **The traces are left** as they are, so a scaled weight and its trace no longer conserve the sum consolidation
  conserves.
- **It is in no image**, so the test sets it on every schedule's engine. Unset, nothing is marked and the tick is the
  rule's bit for bit: the short run's unheld schedule held H-42's arm as far as it ran and wrote the image 007's and
  009's branches wrote, CRC-64 `0x841f164eb6fdefb2`. A unit test holds a round on a hand-made arena.
- **Held at every block's end** of every schedule of the four arms: under either hold, no readout unit's sum stood
  above its value.

Each hold ran eight schedules from H-42's image, from both arms.

## 1 and 2. The two holds over eight schedules

Each cell lists the eight schedules in order. The default's rows are exploration 005's, as 007 and 009 tabled them:

| Rule | Arm | Highest coupling | Lowest coupling | Lowest of the mappings' last 128 correct | Three reversals summed, blocks |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Default | assignment | 1.239, 1.350, 1.428, 1.461, 1.520, 1.592, 1.619, 1.658 | 0.903, 0.930, 0.985, 1.036, 1.090, 1.151, 1.207, 1.286 | 120, 121, 124, 126, 126, 127, 124, 127 | 34, 27, 26, 21, 20, 17, 17, 15 |
| Default | mirrored | 1.248, 1.298, 1.374, 1.431, 1.444, 1.521, 1.564, 1.637 | 0.889, 0.956, 0.970, 1.018, 1.070, 1.115, 1.150, 1.179 | 120, 122, 123, 123, 126, 122, 126, 126 | 40, 33, 31, 22, 24, 20, 20, 16 |
| Ceiling | assignment | 1.202, 1.234, 1.276, 1.301, 1.318, 1.371, 1.404, 1.424 | 0.873, 0.862, 0.893, 0.944, 0.967, 0.975, 1.035, 1.093 | 119, 122, 118, 124, 124, 125, 127, 124 | 42, 41, 32, 28, 22, 21, 21, 22 |
| Ceiling | mirrored | 1.171, 1.190, 1.217, 1.261, 1.317, 1.365, 1.415, 1.441 | 0.861, 0.893, 0.901, 0.940, 0.955, 0.995, 1.008, 1.040 | 119, 116, 122, 124, 124, 124, 126, 124 | 41, 36, 41, 31, 28, 22, 24, 18 |
| Both ways | assignment | 1.238, 1.298, 1.389, 1.450, 1.550, 1.584, 1.614, 1.658 | 0.904, 0.919, 0.943, 1.005, 1.065, 1.112, 1.157, 1.180 | 103, 119, 117, 120, 114, 123, 125, 126 | 58, 48, 44, 40, 35, 29, 30, 28 |
| Both ways | mirrored | 1.219, 1.271, 1.324, 1.442, 1.516, 1.566, 1.639, 1.687 | 0.876, 0.946, 0.976, 1.013, 1.051, 1.122, 1.161, 1.216 | 116, 115, 118, 121, 123, 122, 124, 124 | 52, 48, 40, 34, 33, 32, 28, 22 |

The couplings are over every block's end of the schedule and the four pairs, over the first image's; a mapping is
learned at 80 or more of its last 128, and every mapping of every schedule here was.

H-25's clauses, by schedule: under the ceiling clause 2, no coupling past 1.30, failed from the fourth schedule from the
assignment and from the fifth from the mirrored assignment, where under the default it failed from the second and the
third. Both ways it failed from the third, and clause 3, each reversal within 23 blocks of its flip, failed in the
first schedule from the assignment and in the second from the mirrored assignment. Clauses 1 and 4 held throughout.

Beside them, at each schedule's last block, over the first image's, and the most synapses of one coupling at the rail
or at or above seven eighths of it at a block's end:

| Rule | Arm | Inhibitory sum | Gain | Mean weight over the rail | At the rail, most | At or above 7/8, most |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Default | assignment | 0.743 to 0.556 | 1.7062 to 1.6804 | 0.23–0.30 to 0.32–0.41 | 26, 37, 61, 55, 80, 85, 98, 100 | 42 to 130 |
| Ceiling | assignment | 0.731, 0.574, 0.528, 0.511, 0.502, 0.501, 0.499, 0.498 | 1.7125 to 1.7000 | 0.22–0.29 to 0.27–0.35 | 14, 26, 26, 32, 40, 49, 48, 64 | 29 to 84 |
| Ceiling | mirrored | 0.732, 0.576, 0.528, 0.507, 0.497, 0.492, 0.491, 0.491 | 1.7129 to 1.7002 | 0.22–0.27 to 0.27–0.35 | 19, 19, 20, 28, 26, 39, 48, 52 | 29 to 86 |
| Both ways | assignment | 0.740, 0.598, 0.561, 0.547, 0.544, 0.546, 0.550, 0.554 | 1.7076 to 1.6748 | 0.24–0.30 to 0.30–0.40 | 15, 30, 47, 53, 59, 64, 75, 95 | 37 to 127 |
| Both ways | mirrored | 0.741, 0.598, 0.560, 0.548, 0.548, 0.550, 0.555, 0.559 | 1.7073 to 1.6745 | 0.23–0.28 to 0.31–0.39 | 21, 28, 45, 52, 63, 74, 95, 100 | 39 to 129 |

The mean weight is the range over the four couplings, in the first schedule and in the eighth; the first image's was
0.23 to 0.25.

**The sums.** The readout units' sums over their values, all of them summed, and the lowest unit's, at each schedule's
last block:

| Rule | Arm | All the sums over all the values | The lowest unit |
| :--- | :--- | :--- | :--- |
| Ceiling | assignment | 0.973, 0.964, 0.958, 0.955, 0.954, 0.953, 0.954, 0.954 | 0.432, 0.407, 0.500, 0.463, 0.450, 0.437, 0.517, 0.486 |
| Ceiling | mirrored | 0.972, 0.963, 0.959, 0.955, 0.954, 0.953, 0.952, 0.952 | 0.436, 0.469, 0.457, 0.473, 0.492, 0.508, 0.488, 0.494 |
| Both ways | either | 1.000 in each | 0.998 to 1.000 |

- **Under the ceiling the sums sank and settled.** A sum is cut when it rises past its value and left when it falls, so
  the readouts' sums lost 2.7 and 2.8 % in the first schedule and stood at 0.952 to 0.955 from the fourth on. Single
  units stood at 0.41 to 0.52 of their values at a schedule's last block, and at 0.31 at the lowest block's end. Under
  the default, after the first and the second block of the first schedule, 454 and 469 of the 918 units stood above
  their values, the highest at 1.10 of it.
- **Both ways every sum stayed at its value**, short of it only by the rounding, at most 0.5 %.

**Where the couplings' rise went.** The four couplings summed at each schedule's last block, against the readout
units' sums, all four in rails:

| Rule | Arm | The couplings, schedules 1, 4, 8 | Their share of the readouts' input, schedules 1, 4, 8 | The readouts' other input, schedule 8 |
| :--- | :--- | :--- | :--- | :--- |
| H-42's image | | 775 | 13.2 % | 5 111 |
| Ceiling | assignment | 807, 871, 986 | 14.1, 15.5, 17.6 % | 4 629 |
| Ceiling | mirrored | 789, 858, 978 | 13.8, 15.3, 17.5 % | 4 624 |
| Both ways | assignment | 838, 983, 1 130 | 14.2, 16.7, 19.2 % | 4 755 |
| Both ways | mirrored | 828, 953, 1 115 | 14.1, 16.2, 18.9 % | 4 771 |

- **The couplings rose inside a held sum.** Both ways the readouts' input stood at 5 885 rails throughout, and the
  couplings' share of it grew by about 0.7 points a schedule. The other inputs onto the readout units, the recurrent
  ones from the readout units and from the units outside the four sets, paid for it: 4 755 and 4 771 rails in the
  eighth schedule, 7 % less than the image's. Under the ceiling they fell 9 %, as the sums sank with them.
- **A reading of why**, from the arithmetic and not measured: under the reward's gate a rise lands on the addressed
  synapses, the couplings among them. The hold scales every input of the unit by the same quotient, so of the cut that
  pays for a coupling's rise about 13 % falls on the couplings and 87 % on the unit's other inputs. A coupling keeps
  most of every rise, whatever the hold.
- **The rewards' surplus stayed.** Over the four couplings, the rewards netted +343 to +493 rails a schedule under the
  holds and the punishments −281 to −462, beside the default's +439 to +482 and −363 to −450 over four schedules
  from the assignment. The net rise was 14 to 36 rails a schedule under the ceiling and 28 to 63 both ways, against
  the default's 33 to 77. The moves read at a trial's end include the hold's.

**The rail and the signal's sign**, the four couplings' arrivals at the rail per schedule from the assignment, after a
reward and after a punishment:

| Rule | After a reward | After a punishment |
| :--- | :--- | :--- |
| Default, schedules 1 to 4 | 2 619, 3 321, 3 287, 4 739 | 92, 119, 109, 106 |
| Ceiling | 1 225, 1 403, 1 450, 2 379, 3 826, 4 725, 6 456, 5 899 | 25, 43, 34, 72, 48, 74, 125, 105 |
| Both ways | 712, 1 376, 2 180, 4 853, 5 163, 6 470, 6 579, 7 818 | 242, 388, 530, 874, 1 127, 1 141, 1 112, 1 251 |

Under the ceiling 1 to 3 % of the arrivals followed a punishment, as under the default's 2 to 4 %. Both ways 14 to
25 % did. A reading, not measured here: a punishment nets a loss, which leaves a sum below its value, and the scaling
up that follows lifts every input of the unit, those near the rail with them.

## 3. The mappings' relative differences

At each mapping's last block, per stimulus, its coupling to the readout the mapping in force rewards less its coupling
to the other, over the image's couplings: four mappings and two stimuli, eight differences a schedule. Every one of the
eight, in every schedule of the four arms and of the default's two, favoured the mapping in force. Their mean and their
smallest, per schedule:

| Rule | Arm | Mean | Smallest |
| :--- | :--- | :--- | :--- |
| Default | assignment | 0.22, 0.23, 0.23, 0.24, 0.25, 0.26, 0.26, 0.27 | 0.13, 0.14, 0.13, 0.14, 0.16, 0.18, 0.15, 0.18 |
| Default | mirrored | 0.22, 0.23, 0.24, 0.23, 0.24, 0.25, 0.26, 0.26 | 0.15, 0.14, 0.13, 0.12, 0.13, 0.14, 0.15, 0.14 |
| Ceiling | assignment | 0.22, 0.23, 0.22, 0.23, 0.23, 0.23, 0.24, 0.25 | 0.14, 0.12, 0.13, 0.14, 0.17, 0.17, 0.19, 0.20 |
| Ceiling | mirrored | 0.22, 0.23, 0.24, 0.23, 0.23, 0.24, 0.24, 0.24 | 0.17, 0.14, 0.18, 0.15, 0.14, 0.12, 0.12, 0.12 |
| Both ways | assignment | 0.21, 0.23, 0.22, 0.24, 0.25, 0.26, 0.26, 0.27 | 0.14, 0.15, 0.07, 0.06, 0.07, 0.10, 0.12, 0.10 |
| Both ways | mirrored | 0.22, 0.23, 0.24, 0.25, 0.26, 0.27, 0.26, 0.26 | 0.16, 0.14, 0.17, 0.20, 0.17, 0.15, 0.14, 0.11 |

The default's are read from exploration 005's run, its couplings at the same blocks.

- **The differences were the default's**, in the image's couplings, under both holds. Under the ceiling, whose
  couplings stood lower, they were the larger part of each: in the eighth schedule about 19 % of the four couplings'
  mean, against 18 % under the default.
- **Both ways from the assignment, one difference shrank.** Under the assignment's mappings B's fell to 0.06 and 0.07
  in the fourth and the fifth schedule while A's rose to 0.39 to 0.43, and under the mirrored mapping A's fell to 0.07
  in the third. Learning held there, at 114 to 120 of the last 128.

## What the sum costs

The tree keeps a synapse with its presynaptic unit, so a unit's sum is a scan. A round walks every excitatory unit's
chain, the 26 240 excitatory synapses in the arena's 8 192 blocks, sums the units marked, and rewrites the synapses of
those it scales:

| Rule | Arm | Rounds a schedule | Units a round | Units scaled a schedule | Synapses rewritten a schedule | µs a round | Seconds a schedule |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Ceiling | assignment | 71 459, 41 835 | 19.0, 23.0 | 49 934, 14 749 | 1 340 881, 365 994 | 69 to 74 | 5.1, 3.0 |
| Ceiling | mirrored | 73 051, 42 465 | 18.9, 23.2 | 46 873, 15 097 | 1 252 839, 407 401 | 68 to 77 | 4.9, 3.2 |
| Both ways | assignment | 72 090, 47 007 | 19.4, 23.4 | 1 385 820, 1 083 134 | 20 081 687, 13 656 770 | 71 to 86 | 5.4, 3.4 |
| Both ways | mirrored | 73 108, 44 448 | 18.9, 24.2 | 1 368 954, 1 060 898 | 20 382 758, 13 120 539 | 67 to 80 | 4.9, 3.5 |

Each cell is the first schedule's and the eighth's, the µs a round the range over the eight.

- **A round is the scan**: 67 to 86 µs for 26 240 synapses, about 3 ns a synapse, on one thread between ticks while
  the other workers wait at the barrier.
- **Rounds are rare.** A schedule is 125 829 620 ticks, and a round ran in 0.03 to 0.06 % of them: under the reward's
  gate an excitatory synapse consolidates only where the reward's address reaches it, while its signal lasts. The
  rounds fell over the schedules, as the share of punished trials did, from 37 to 45 % in the first to 20 to 27 % in
  the eighth.
- **The hold cost 3.0 to 5.4 s of a schedule's, about 900.** The four arms took 7 291 to 7 426 s, 009's bounds 7 300
  to 7 414.
- **Under a rule that consolidates at every spike it would not be rare.** A round reads the whole arena to sum 19 to 24
  units of about 26 inputs each, some 500 to 600 synapses. A record of each held unit's inputs would read those alone;
  it was not built.

## What it suggests

**Holding a readout's whole excitatory sum does not bound the couplings.** The couplings are 13 % of what a readout
unit receives. A hold that scales all of a unit's inputs together takes most of a coupling's rise from the inputs that
are not couplings, so both ways the couplings rose as under the default. The ceiling slowed them by letting the sums
sink about 5 %, and the highest still passed 1.30 in the fourth and the fifth schedule. Of the six bounds 007, 009 and
this note have tried, none held the couplings at 1.30 with every mapping learned. Both holds kept every mapping of
every schedule learned, from both arms, which none of the four bounds on one synapse did.

**What the hold reaches is the wrong total for the couplings.** Questions this suggests for a later exploration or a
round:
- a sum held over the stimulus units' synapses onto a readout unit alone, or over the inputs the reward's address
  reaches, the part of the input the couplings are;
- a stimulus unit's outgoing excitatory sum held instead. The tree keeps a synapse with its presynaptic unit, so that
  sum is one chain's walk and no scan, and scaling a unit's outputs together keeps A→R0 against A→R1 as a ratio while
  it bounds them;
- whether the readouts' other inputs, 7 to 10 % lower after eight schedules under either hold, carry anything the task
  or the rate's rule needs over longer runs.

## What did not run

- **Item 4, sixteen schedules.** Neither hold met its condition. The eighth images were printed, CRC-64
  `0x9d3605317185ed90` and `0xc7b37264a468cbab` under the ceiling and `0xc29c548e8d425020` and `0xa310b7e95e5f9851` both
  ways, and are in the runs' artifacts; the branch keeps none, and its second halves refuse to start until one is
  written down.
- **Seeds 23 and 24.**
- **Other forms of the hold**: on the excitatory readout units alone; held at each schedule's starting image rather
  than H-42's; with the traces scaled beside the weights; on every unit; and a record of each unit's inputs in place
  of the scan.
- **The rail paths under the default from the mirrored assignment**, or past its fourth schedule.
- **The rate's error**, and the inhibitory rule's replay terms, which the logs hold and this note does not read.
- **What the readouts' recurrent inputs carry**, by the group of their presynaptic unit.
- **The nights** of ADR-0044 and ADR-0048 under either hold.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `a_readouts_sum_held_short_on_seed_22_explore` | [40563989689](https://github.com/Menteion/Menteion/actions/runs/40563989689) | `3058a60` | 244 |
| `a_readouts_sum_held_as_a_ceiling_on_seed_22_from_the_assignment_explore` | [40563989689](https://github.com/Menteion/Menteion/actions/runs/40563989689) | `3058a60` | 7426 |
| `a_readouts_sum_held_as_a_ceiling_on_seed_22_from_the_mirrored_assignment_explore` | [40563989689](https://github.com/Menteion/Menteion/actions/runs/40563989689) | `3058a60` | 7313 |
| `a_readouts_sum_held_both_ways_on_seed_22_from_the_assignment_explore` | [40563989689](https://github.com/Menteion/Menteion/actions/runs/40563989689) | `3058a60` | 7405 |
| `a_readouts_sum_held_both_ways_on_seed_22_from_the_mirrored_assignment_explore` | [40563989689](https://github.com/Menteion/Menteion/actions/runs/40563989689) | `3058a60` | 7291 |

Exploration 005's eight schedules were read again from run
[40300651755](https://github.com/Menteion/Menteion/actions/runs/40300651755) on `056c76b`.
