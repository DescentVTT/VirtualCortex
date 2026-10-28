---
status: exploratory
date: 2026-10-28
---

# 012: The zero sum's rounding

**Branch**: `explore/012-the-zero-sums-rounding`
**Commit**: `c68a9d5` for the short run, the first halves and the control; `96ca056` for the second halves, which adds
the four images the first halves wrote and their CRC-64s and nothing else that runs. Both carry `e311a12`, the merge of
exploration 011's tag, and `4cddaf7`, this exploration's rounding under `src/`. Unset, each is the rule bit for bit, and
rounded down each is exploration 011's zero sum bit for bit.
**Runs**: [40641556801](https://github.com/Menteion/Menteion/actions/runs/40641556801) (the short run, the first halves
and the control) and [40653304865](https://github.com/Menteion/Menteion/actions/runs/40653304865) (the second halves),
both at `scope=arms`

Exploration 011 held each stimulus set's couplings to the two readout sets, summed, at their value in H-42's image. Both
ways it held the highest coupling at 1.13 to 1.16 of the image's over sixteen schedules, with every mapping learned and
three reversals summing 11 to 13 blocks a schedule from the third ([011](011-zero-sum-at-the-stimulus.md)). It also
split each coupling: by the eighth image 536 to 575 of a coupling's about 800 synapses stood in the bottom sixteenth of
the width and up to 158 at the rail. 011 read the split from the arithmetic, and did not measure it, as the rounding: a
weight scaled by a quotient near one and rounded down loses an LSB whenever it is above zero, and only the largest gain
one back. This note asks how much of the split is the rounding and how much the zero sum, and whether the couplings stay
bounded and learning as fast once the rounding is changed. On seed 22:

1. to the nearest: 011's zero sum both ways, each scaled weight rounded to the nearest integer instead of down, a tie
   stated;
2. the remainder kept: what a round's rounding drops carried to the next round, so that no direction is favoured over
   many rounds; or the largest remainder, every weight rounded down and the deficit given back an LSB at a time in the
   order of the fractions. Which, and why, written here;
3. each from H-42's image over sixteen schedules in two halves, from both arms, read as 011 read them;
4. 011's zero sum both ways, rounded down, as the comparison, schedule by schedule, in the distribution and in how fast
   learning is.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host.

**In brief**
- **The split was the rounding.** Rounded to the nearest, 1 to 41 of a coupling's synapses stood in the bottom
  sixteenth of the width at any image's end over sixteen schedules from both arms; with each synapse's fraction carried,
  4 to 93. Rounded down, 536 to 575 did by the eighth image and 556 to 582 by the sixteenth. H-42's image had 2 to 9.
- **The couplings stayed bounded.** The highest coupling stood at 1.14 to 1.21 of the image's to the nearest and 1.13
  to 1.20 carried, over sixteen schedules from both arms; rounded down, 1.13 to 1.16. Every mapping of all 64 schedules
  was learned, at 117 to 127 of its last 128, and H-25's four clauses held in every one.
- **Learning was not as fast.** From the third schedule a schedule's three reversals summed 23 to 34 blocks to the
  nearest and 16 to 32 carried; rounded down, 11 to 13. The default's took 15 to 31 over schedules 3 to 8 and 12 to 16
  over 9 to 16. Carried, they fell to 16 to 24 over schedules 9 to 16, as 011's ceiling's did.
- **Rounded down, the scaling moved weight from the small synapses to the large ones.** Run again with counters, 011's
  zero sum took 113 to 170 rails a schedule out of the synapses below ADR-0055's reference magnitude and gave them to
  those above it. To the nearest and carried it took from every band in all but two schedules, the bottom one at most
  37 rails a schedule.
- **Carried, the tails still grew** over schedules 9 to 16: by the sixteenth image up to 93 of a coupling's synapses
  stood in the bottom sixteenth and 95 in the top, and up to 73 at the rail at a block's end. To the nearest they grew
  little.
- **The rounding is cheap.** A round took 16 µs to the nearest and 20 µs carried where 011's took 10 to 12, and a
  schedule 0.75 to 1.36 s of its about 700.

## What ran

**The network, the arm and the path** are exploration 011's. Every test starts from H-42's network of seed 22 at 1 024
units, the image an engine born under `Config::reads_its_targets()` settles. Each test first runs that derivation again
and holds it to H-42's pinned tables (`probe_made`), and all ten held. The arm is H-43's twin on time, H-42's arm bit for
bit, from the assignment and from the mirrored assignment. A schedule ends with the engine run quiet without the drive
until it is quiescent; its image is written, and the next schedule's engine is decoded from it.

**The harness** is exploration 011's as its tag holds it (`2f6e563`), merged onto `main` at `f6ba406`. 011's readings
are taken in every schedule, 009's rail paths among them. Beside them:
- what each round's scaling moved, from counters in the engine: the LSBs it added to the weights and took from them, by
  the band each weight stood in before, exploration 009's three: below ADR-0055's reference magnitude, from it to three
  quarters of the rail, and at or above three quarters;
- the farthest a stimulus set's sum stood from its value at a block's end.

**The rounding, under `src/` on this branch alone.** Commit `4cddaf7` adds `ZeroSumRounding` and
`Executor::set_zero_sum_rounding`. A round of 011's zero sum scales every one of a group's synapses onto a target to
$w V / S$, $w$ its weight, $V$ the group's value and $S$ its sum, within the rail:
- **Down** (`ZeroSumRounding::Down`): $\lfloor w V / S \rfloor$, exploration 011's, bit for bit. It is the default.
- **To the nearest** (`ZeroSumRounding::Nearest`): the nearest integer to $w V / S$. A tie, $w V / S$ exactly halfway
  between two integers, goes to the even one, so that ties favour neither direction. The engine counts them.
- **Carried** (`ZeroSumRounding::Carried`): $\lfloor w V / S + f \rfloor$, $f$ the fraction the synapse's last round
  left over, the new fraction kept as its next $f$, in $2^{-32}$ LSB. Every $f$ starts at a half, so that over all its
  rounds a synapse's weight stays within half an LSB of where the exact quotients would have taken it, either way. The
  fractions are four bytes a slot of the synapse arena, kept in the engine at each slot's place.
- **It is in no image**, so the test sets it on every schedule's engine, and under `Carried` each schedule starts its
  fractions again at a half.

**Why the fraction is carried per synapse**, and not as one remainder per group or by the largest remainder:
- **A group's own remainder is carried already.** Each round reads the sum again, so what one round's rounding took
  from it the next round scales back. 011's reading is about which synapses pay for a round and which are paid, and a
  group's single remainder says nothing about that.
- **The largest remainder** rounds each weight to the integer below or above $w V / S$ and gives the group its value
  exactly. Under a quotient near one it moves a weight only when the weight's exact share comes near an LSB, as rounding
  to the nearest does: a synapse whose share is a tenth of an LSB a round keeps its weight round after round. It would
  differ from item 1 in hitting the sum, not in which synapses move.
- **Carried**, that synapse moves an LSB in ten rounds. Over many rounds every synapse pays its exact share, to within
  half an LSB, which is what "no direction favoured" asks of the long run.

**Held.** Unset, and rounded down, the branch is what it was: the short run's unheld schedule held H-42's arm as far as
it ran and wrote 011's image, CRC-64 `0x841f164eb6fdefb2`, and its schedule rounded down wrote 011's image both ways,
`0x9fac5bf29a24b2ec`. **The control**, 011's zero sum both ways from the assignment rounded down over eight schedules,
wrote 011's eight images, the eighth `0x688ea9d657e2b91a`, so its counters are 011's run's. Two unit tests hold the three
roundings over a weight and over a hand-made arena; 011's test of a group's round holds `Down`'s weights unchanged.

**Sixteen schedules in two halves**, as exploration 011 carried its long runs. Each first half started from H-42's
image and printed its eighth image whole, in hexadecimal. The branch keeps the four images in
`runtime/menteion-runtime/tests/explore-012/`, each decoded from its run's log and held to the CRC-64 printed beside it.
Each second half read its image, held it to that CRC-64, decoded it and ran schedules 9 to 16.

## 1 and 2. The two roundings over sixteen schedules

Each cell lists eight schedules in order. The default's rows are exploration 005's and 007's, and rounded down is 011's,
as 011 tabled them:

| Rounding | Arm | Schedules | Highest coupling | Lowest coupling | Lowest of the mappings' last 128 correct | Three reversals summed, blocks |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Default | assignment | 1–8 | 1.239, 1.350, 1.428, 1.461, 1.520, 1.592, 1.619, 1.658 | 0.903, 0.930, 0.985, 1.036, 1.090, 1.151, 1.207, 1.286 | 120, 121, 124, 126, 126, 127, 124, 127 | 34, 27, 26, 21, 20, 17, 17, 15 |
| Default | mirrored | 1–8 | 1.248, 1.298, 1.374, 1.431, 1.444, 1.521, 1.564, 1.637 | 0.889, 0.956, 0.970, 1.018, 1.070, 1.115, 1.150, 1.179 | 120, 122, 123, 123, 126, 122, 126, 126 | 40, 33, 31, 22, 24, 20, 20, 16 |
| Down | assignment | 1–8 | 1.150, 1.159, 1.141, 1.141, 1.141, 1.141, 1.149, 1.154 | 0.863, 0.855, 0.862, 0.863, 0.842, 0.842, 0.833, 0.828 | 123, 125, 122, 124, 125, 124, 123, 125 | 33, 17, 13, 12, 12, 12, 12, 12 |
| Down | mirrored | 1–8 | 1.146, 1.130, 1.133, 1.135, 1.139, 1.145, 1.143, 1.147 | 0.860, 0.858, 0.852, 0.851, 0.855, 0.861, 0.855, 0.860 | 121, 124, 123, 125, 125, 124, 127, 125 | 37, 21, 13, 13, 12, 12, 12, 12 |
| Nearest | assignment | 1–8 | 1.149, 1.171, 1.161, 1.183, 1.167, 1.161, 1.159, 1.162 | 0.863, 0.847, 0.856, 0.837, 0.851, 0.850, 0.858, 0.846 | 119, 120, 119, 121, 123, 125, 121, 121 | 34, 36, 29, 30, 30, 27, 26, 28 |
| Nearest | mirrored | 1–8 | 1.141, 1.156, 1.162, 1.177, 1.172, 1.171, 1.172, 1.176 | 0.870, 0.860, 0.855, 0.842, 0.846, 0.847, 0.846, 0.842 | 121, 121, 121, 123, 124, 126, 125, 124 | 42, 35, 34, 31, 27, 28, 28, 26 |
| Carried | assignment | 1–8 | 1.153, 1.172, 1.161, 1.169, 1.161, 1.155, 1.149, 1.147 | 0.855, 0.836, 0.854, 0.849, 0.856, 0.852, 0.861, 0.860 | 119, 117, 121, 125, 126, 126, 122, 124 | 34, 37, 32, 32, 27, 23, 24, 22 |
| Carried | mirrored | 1–8 | 1.137, 1.143, 1.132, 1.152, 1.147, 1.154, 1.143, 1.143 | 0.869, 0.872, 0.874, 0.862, 0.867, 0.853, 0.852, 0.850 | 119, 118, 124, 126, 123, 125, 127, 125 | 41, 34, 31, 31, 28, 26, 25, 23 |
| Default | assignment | 9–16 | 1.702, 1.786, 1.839, 1.878, 1.928, 2.001, 2.018, 2.072 | 1.296, 1.363, 1.411, 1.436, 1.471, 1.495, 1.533, 1.572 | 127, 127, 127, 127, 127, 126, 128, 128 | 15, 15, 13, 15, 13, 14, 13, 13 |
| Default | mirrored | 9–16 | 1.682, 1.737, 1.783, 1.835, 1.915, 1.983, 2.019, 2.061 | 1.222, 1.329, 1.360, 1.397, 1.434, 1.482, 1.522, 1.569 | 127, 126, 127, 126, 125, 127, 128, 127 | 16, 13, 16, 15, 13, 14, 12, 12 |
| Down | assignment | 9–16 | 1.152, 1.144, 1.149, 1.153, 1.149, 1.154, 1.153, 1.153 | 0.830, 0.840, 0.848, 0.842, 0.843, 0.851, 0.845, 0.848 | 125, 124, 124, 124, 127, 125, 123, 125 | 13, 12, 12, 12, 11, 13, 12, 12 |
| Down | mirrored | 9–16 | 1.138, 1.148, 1.148, 1.147, 1.148, 1.150, 1.155, 1.147 | 0.861, 0.860, 0.860, 0.860, 0.858, 0.861, 0.856, 0.867 | 125, 126, 123, 126, 126, 124, 124, 125 | 12, 12, 12, 12, 12, 12, 13, 12 |
| Nearest | assignment | 9–16 | 1.164, 1.147, 1.158, 1.154, 1.147, 1.163, 1.136, 1.152 | 0.853, 0.860, 0.855, 0.852, 0.860, 0.854, 0.870, 0.864 | 124, 122, 124, 123, 125, 122, 123, 123 | 27, 30, 26, 27, 23, 28, 23, 23 |
| Nearest | mirrored | 9–16 | 1.181, 1.189, 1.181, 1.187, 1.205, 1.196, 1.201, 1.181 | 0.838, 0.831, 0.838, 0.832, 0.816, 0.825, 0.820, 0.838 | 125, 126, 123, 126, 123, 125, 126, 125 | 28, 26, 27, 23, 25, 24, 28, 27 |
| Carried | assignment | 9–16 | 1.156, 1.155, 1.153, 1.164, 1.174, 1.165, 1.152, 1.160 | 0.851, 0.852, 0.854, 0.843, 0.833, 0.842, 0.855, 0.857 | 125, 127, 125, 126, 124, 123, 127, 127 | 23, 21, 24, 18, 19, 19, 21, 20 |
| Carried | mirrored | 9–16 | 1.159, 1.170, 1.180, 1.184, 1.194, 1.188, 1.198, 1.196 | 0.857, 0.848, 0.839, 0.835, 0.826, 0.832, 0.823, 0.825 | 126, 123, 125, 127, 127, 124, 126, 127 | 22, 24, 20, 16, 17, 17, 19, 17 |

The couplings are over every block's end of the schedule and the four pairs, over the first image's; a mapping is
learned at 80 or more of its last 128.

- **Both roundings held the couplings under 1.30.** From the assignment the highest stood at 1.136 to 1.183 to the
  nearest and 1.147 to 1.174 carried, against 1.141 to 1.159 rounded down. From the mirrored assignment both rose a
  little over schedules 9 to 16: to the nearest to 1.205 in the thirteenth, carried from 1.159 to 1.198, as 011's ceiling
  rose to 1.20 there.
- **Learning held.** H-25's four clauses held in every schedule of the four arms, and every mapping was learned.
- **It was slower than rounded down, and than the default.** The first schedule's reversals were alike under every rule,
  33 to 42 blocks. From the second, rounded down they fell to 12 or 13 by the third; to the nearest they stayed at 23 to
  36 through the sixteenth; carried they fell slowly, to 17 to 19 over the last four schedules from the mirrored
  assignment and 19 to 21 from the assignment. 011's ceiling took 16 to 24 over schedules 9 to 16.
- **More trials were punished.** A schedule's punished trials were 23 to 36 % of its trials to the nearest and 19 to
  37 % carried, against 14 to 33 % rounded down; from the ninth schedule, 23 to 28 %, 19 to 24 % and 14 to 16 %.

Beside them, at each schedule's last block over the first image's, and the most synapses of one coupling at the rail at
a block's end, a schedule's eight in order:

| Rounding | Arm | Inhibitory sum, schedules 1 to 8; 9 to 16 | Gain | At the rail, most, schedules 1 to 8 | Schedules 9 to 16 | At or above 7/8, most |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Down | assignment | 0.734 to 0.535; 0.533 to 0.535 | 1.7050 to 1.6797; 1.6772 to 1.6809 | 37, 89, 121, 126, 138, 158, 147, 149 | 141, 140, 142, 154, 145, 142, 157, 140 | 51 to 214 |
| Down | mirrored | 0.735 to 0.527; 0.520 to 0.526 | 1.7032 to 1.6792; 1.6753 to 1.6799 | 44, 87, 103, 126, 126, 139, 153, 142 | 144, 139, 145, 149, 143, 138, 143, 145 | 59 to 215 |
| Nearest | assignment | 0.741 to 0.512; 0.506 to 0.510 | 1.7104 to 1.7063; 1.7018 to 1.7073 | 22, 24, 12, 19, 27, 18, 15, 16 | 17, 21, 10, 9, 20, 9, 20, 21 | 31 to 63 |
| Nearest | mirrored | 0.742 to 0.512; 0.507 to 0.511 | 1.7101 to 1.7036; 1.7007 to 1.7049 | 15, 10, 22, 18, 8, 17, 14, 31 | 32, 32, 17, 22, 48, 22, 15, 22 | 34 to 68 |
| Carried | assignment | 0.741 to 0.515; 0.507 to 0.512 | 1.7101 to 1.7019; 1.6951 to 1.7001 | 18, 32, 16, 23, 24, 34, 32, 42 | 33, 60, 66, 55, 48, 51, 50, 66 | 32 to 99 |
| Carried | mirrored | 0.742 to 0.518; 0.506 to 0.516 | 1.7104 to 1.7006; 1.6928 to 1.6985 | 13, 10, 17, 36, 27, 29, 41, 34 | 34, 67, 64, 65, 70, 62, 73, 64 | 38 to 106 |

The default's inhibitory sum fell from 0.743 to 0.556 over its first eight schedules and turned to rise to 0.589 and
0.597 by the sixteenth; its gain fell to 1.654 and 1.656; its most at the rail rose from 26 to 100 over schedules 1 to 8
from the assignment and to 186 and 170 by the sixteenth (005, 007).

- **The inhibitory sum stopped** under both roundings, at 0.506 to 0.516 over schedules 9 to 16, a little below
  rounded down's 0.520 to 0.535. **The gain stopped** at 1.693 to 1.707, above rounded down's 1.675 to 1.681; carried
  it still fell, by 0.0028 and 0.0025 over schedules 9 to 16, where the default's fell by about 0.003 a schedule.
- **Fewer synapses stood at the rail.** To the nearest at most 8 to 48 of a coupling at a block's end, carried 10 to 73,
  against 37 to 158 rounded down and the default's 26 to 186.
- **The mean weight over the rail** stood at 0.19 to 0.29 in every schedule under every rounding; H-42's image's was 0.23
  to 0.25.

**The sums.** Every set's sum stood at 0.9999 to 1.0000 of its value at every block's end under both roundings, on
either side of it: the farthest, over every block of both halves, was 34 parts in a million to the nearest and 12
carried, where rounded down it was 138 and always below. The four couplings summed to 775 rails throughout, and carried
13.15 to 13.17 % of the readout units' excitatory input, as under 011's zero sum.

## Where the weight went inside a coupling

The couplings' synapses by sixteenths of the width at each image's end, from the assignment. The pairs are A→R0, A→R1,
B→R0 and B→R1, of 775 to 809 synapses each:

| Rounding | Image | In the bottom sixteenth, by pair | In the top half, by pair | In the top sixteenth, by pair |
| :--- | ---: | :--- | :--- | :--- |
| H-42's image | 0 | 7, 9, 4, 2 | 10, 13, 10, 17 | 0, 0, 0, 0 |
| Down | 2 | 318, 273, 190, 259 | 136, 165, 141, 105 | 5, 97, 83, 10 |
| Down | 4 | 492, 482, 431, 479 | 178, 217, 197, 188 | 0, 142, 143, 11 |
| Down | 8 | 567, 550, 536, 575 | 193, 233, 220, 215 | 0, 174, 159, 12 |
| Down | 16 | 575, 559, 556, 582 | 198, 230, 224, 225 | 0, 176, 166, 4 |
| Nearest | 2 | 15, 7, 1, 21 | 8, 59, 62, 4 | 0, 33, 18, 0 |
| Nearest | 4 | 22, 12, 5, 34 | 10, 68, 78, 11 | 0, 33, 20, 0 |
| Nearest | 8 | 24, 11, 2, 25 | 13, 72, 67, 15 | 0, 31, 34, 0 |
| Nearest | 12 | 41, 3, 3, 21 | 18, 75, 75, 18 | 0, 10, 40, 0 |
| Nearest | 16 | 16, 8, 7, 23 | 45, 85, 82, 29 | 0, 62, 33, 0 |
| Carried | 2 | 23, 8, 5, 27 | 12, 62, 90, 9 | 0, 33, 13, 0 |
| Carried | 4 | 32, 15, 12, 46 | 25, 66, 97, 26 | 0, 52, 40, 0 |
| Carried | 8 | 64, 17, 20, 41 | 35, 96, 92, 49 | 0, 59, 62, 0 |
| Carried | 12 | 72, 16, 25, 58 | 67, 107, 117, 75 | 0, 63, 74, 0 |
| Carried | 16 | 49, 54, 27, 44 | 83, 121, 127, 101 | 0, 72, 79, 0 |

From the mirrored assignment, at the sixteenth image, the bottom sixteenth held 563 to 581 rounded down, 10 to 30 to the
nearest and 26 to 93 carried; the top half 210 to 227, 19 to 87 and 93 to 133; the top sixteenth, on the two pairs the
last mapping rewarded, 185 and 193, 53 and 37, and 95 and 79.

- **Rounded otherwise than down, the couplings did not split.** At no image's end did more than 41 of a coupling's
  synapses stand in the bottom sixteenth to the nearest, or more than 93 carried; rounded down 190 to 325 did by the
  second image. This is the measurement 011 did not make: the split was the rounding's.
- **Carried, both tails grew slowly**, the top half from 0 to 62 synapses at the first image to 83 to 133 at the
  sixteenth, the top sixteenth on the pairs the last mapping rewarded to 72 to 95, and the bottom sixteenth to 27 to
  93, still growing at the sixteenth schedule, as under 011's ceiling. To the nearest, the top half grew from 2 to 54 at
  the first image to 19 to 87 at the sixteenth, and the bottom sixteenth stayed at 1 to 41.

**What the scaling moved, by band.** The LSBs each schedule's rounds added and took, net, in rails, by the band a weight
stood in before, from the assignment, schedules 1 to 8:

| Rounding | Below the reference magnitude | Reference to three quarters | At or above three quarters |
| :--- | :--- | :--- | :--- |
| Down | −140, −170, −149, −138, −129, −113, −126, −117 | +74, +87, +75, +44, +40, +26, +17, +21 | +35, +82, +99, +77, +81, +62, +55, +65 |
| Nearest | −29, −17, −7, −9, −12, −9, −5, 0 | −42, −33, −25, −29, −34, −35, −27, −23 | −9, −8, −7, −10, −12, −14, −14, −11 |
| Carried | −37, −22, −18, −24, −20, −23, −22, −16 | −38, −29, −23, −28, −23, −27, −27, −22 | −4, −2, −3, −4, −5, −5, −5, −4 |

Down's rows are the control's, which wrote 011's images.

- **Rounded down, the scaling moved weight from the small synapses to the large.** It took 113 to 170 rails a schedule
  from the synapses below the reference magnitude and gave 17 to 87 to the middle band and 35 to 99 to the top. The rule
  makes no such move; it is what 011 read from the arithmetic. In gross, by the eighth schedule it added 511 rails to
  the top band and took 446 from it, a schedule: the largest synapses were carried down and up again round after round.
- **To the nearest and carried, it took from every band** in every schedule of both halves and both arms, but two in
  which the bottom band's net was 0 and +2 rails to the nearest. To the nearest the bottom band paid +2 to −30 rails a
  schedule; carried, −11 to −37.

**By the signal's sign**, by the band a synapse stood in before it moved, as 009 read it:
- **To the nearest and carried**, in every schedule of both halves, the trials after a reward moved 97 to 148 rails a
  schedule out of the bottom band and 77 to 177 into the top, and the trials after a punishment moved 83 to 146 back into
  the bottom band and 67 to 189 out of the top.
- **Rounded down** they did so in the first three schedules; from the fifth, the trials after a reward put 2 to 38 rails
  back into the bottom band, and over schedules 9 to 16 every band's net was within 38 rails of zero.

## The rail and the signal's sign

The four couplings' arrivals at the rail per schedule from the assignment, after a reward and after a punishment:

| Rounding | Schedules | After a reward | After a punishment |
| :--- | :--- | :--- | :--- |
| Down | 1–8 | 8 974, 23 023, 36 478, 45 182, 48 179, 47 490, 47 798, 47 174 | 1 330, 3 799, 7 096, 9 498, 12 084, 11 258, 11 085, 12 704 |
| Nearest | 1–8 | 1 769, 1 120, 966, 545, 1 039, 1 169, 1 065, 958 | 827, 798, 685, 633, 921, 830, 1 111, 729 |
| Nearest | 9–16 | 976, 739, 749, 758, 869, 891, 913, 1 050 | 920, 600, 628, 707, 861, 1 066, 990, 1 551 |
| Carried | 1–8 | 2 315, 1 919, 2 004, 3 750, 4 549, 8 061, 6 025, 7 768 | 859, 736, 1 019, 1 207, 1 486, 1 511, 1 576, 1 634 |
| Carried | 9–16 | 5 757, 8 279, 11 700, 12 849, 13 113, 12 563, 12 454, 11 818 | 1 332, 1 352, 1 692, 2 104, 2 001, 1 972, 2 087, 2 309 |

- **Rounded down a synapse arrived and left again tens of thousands of times a schedule**, 57 395 to 63 519 arrivals
  over schedules 9 to 16, with at most 158 of a coupling at the rail at a block's end. To the nearest a schedule had
  1 178 to 3 076 arrivals, carried 2 619 to 24 950.
- **To the nearest, as many arrived after a punishment as after a reward**: 32 to 60 % of a schedule's arrivals followed
  a punishment, against 13 to 21 % rounded down, 11 to 34 % carried, and the default's 2 to 4 %. A reading, not
  measured: to the nearest, a round moves only the weights whose share of it reaches half an LSB, the largest, so a sum
  a punishment left below its value is restored by lifting the synapses nearest the rail.

## 3. The mappings' relative differences

At each mapping's last block, per stimulus, its coupling to the readout the mapping in force rewards less its coupling to
the other, over the image's couplings, as 011 read them. Every one of the eight a schedule, in every schedule of the four
arms, favoured the mapping in force. Their mean and their smallest, per schedule:

| Rounding | Arm | Mean, schedules 1 to 16 | Smallest, schedules 1 to 16 |
| :--- | :--- | :--- | :--- |
| Down | assignment | 0.23, 0.24, 0.24, 0.22, 0.23, 0.24, 0.22, 0.24; 0.23, 0.23, 0.22, 0.23, 0.23, 0.24, 0.23, 0.24 | 0.16, 0.16, 0.17, 0.17, 0.18, 0.19, 0.15, 0.17; 0.17, 0.16, 0.16, 0.15, 0.16, 0.18, 0.17, 0.18 |
| Down | mirrored | 0.23, 0.24, 0.22, 0.23, 0.23, 0.23, 0.24, 0.23; 0.24, 0.24, 0.23, 0.24, 0.24, 0.23, 0.23, 0.23 | 0.18, 0.21, 0.18, 0.17, 0.18, 0.19, 0.18, 0.18; 0.17, 0.18, 0.19, 0.18, 0.19, 0.17, 0.19, 0.19 |
| Nearest | assignment | 0.21, 0.22, 0.21, 0.22, 0.22, 0.22, 0.22, 0.22; 0.23, 0.21, 0.22, 0.22, 0.22, 0.23, 0.22, 0.23 | 0.14, 0.13, 0.10, 0.12, 0.14, 0.13, 0.11, 0.14; 0.16, 0.15, 0.12, 0.16, 0.16, 0.17, 0.18, 0.18 |
| Nearest | mirrored | 0.22, 0.22, 0.23, 0.22, 0.21, 0.22, 0.21, 0.21; 0.21, 0.22, 0.21, 0.22, 0.22, 0.22, 0.22, 0.23 | 0.16, 0.13, 0.13, 0.14, 0.11, 0.11, 0.12, 0.11; 0.11, 0.09, 0.10, 0.09, 0.06, 0.08, 0.07, 0.09 |
| Carried | assignment | 0.21, 0.23, 0.23, 0.24, 0.23, 0.23, 0.24, 0.25; 0.24, 0.24, 0.24, 0.25, 0.24, 0.25, 0.25, 0.26 | 0.13, 0.12, 0.14, 0.15, 0.16, 0.17, 0.18, 0.21; 0.18, 0.18, 0.18, 0.19, 0.17, 0.20, 0.19, 0.20 |
| Carried | mirrored | 0.22, 0.22, 0.22, 0.24, 0.23, 0.25, 0.25, 0.24; 0.24, 0.24, 0.24, 0.23, 0.25, 0.25, 0.26, 0.25 | 0.17, 0.15, 0.18, 0.19, 0.17, 0.18, 0.22, 0.19; 0.19, 0.16, 0.14, 0.12, 0.14, 0.14, 0.15, 0.14 |

- **The differences held** at 0.21 to 0.26 on average under both roundings, where rounded down they stood at 0.22 to
  0.24 and the default's rose from 0.22 to 0.27. Carried, they rose a little, to 0.23 to 0.26 over schedules 9 to 16.
- **To the nearest the smallest fell** from the mirrored assignment, to 0.06 to 0.11 over schedules 9 to 16, still
  favouring the mapping in force; carried and rounded down it stayed at 0.12 or more.

## What the rounding costs

| Rounding | Arm | Rounds a schedule, the first, eighth and sixteenth | Rounds that did not scale | Synapses rewritten a round that scaled | µs a round | Seconds a schedule |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Down | assignment | 59 346, 33 154, 31 765 | 17 to 39 | 328 to 421 | 10.5 to 11.6 | 0.36 to 0.62 |
| Nearest | assignment | 65 967, 53 190, 49 018 | 31 to 64 | 267 to 312 | 16.0 to 16.5 | 0.80 to 1.05 |
| Nearest | mirrored | 67 796, 49 029, 49 811 | 30 to 62 | 265 to 311 | 16.0 to 16.5 | 0.77 to 1.09 |
| Carried | assignment | 65 322, 43 404, 38 539 | 240 to 415 | 332 to 372 | 20.2 to 20.8 | 0.78 to 1.34 |
| Carried | mirrored | 68 045, 45 683, 37 358 | 209 to 441 | 328 to 375 | 20.0 to 20.8 | 0.75 to 1.36 |

Down's row is 011's run.

- **A round costs 4 to 10 µs more** than 011's rounded down. The counters by band are part of it: the control, rounded
  down with them, took 12.5 to 14.3 µs. Carried adds a division a synapse. The runners differ from job to job, so the µs
  are an indication only.
- **More rounds ran**: 36 539 to 68 045 a schedule, against 30 724 to 61 485 rounded down. Carried, a round found a set's sum at its value exactly in 209 to 441 rounds a schedule.
- **The rounding cost at most 1.4 s of a schedule's about 700**: the first halves took 5 676 to 5 956 s for eight
  schedules beside the derivation, 011's 5 636 to 5 876.
- **No tie arose**: to the nearest, no $w V / S$ in any round of the 32 schedules stood exactly halfway between two
  integers, so the tie's rule never acted.

## What it suggests

**The split 011 read both ways is the rounding's.** With each scaled weight rounded to the nearest, or rounded down with
its fraction carried, the zero sum held the couplings within 0.82 to 1.21 of the image's over sixteen schedules from both
arms, every mapping learned and the mappings' differences the default's, while no coupling split: the bottom sixteenth of
a coupling's width held at most 41 and 93 synapses where rounded down it held 536 to 582 from the eighth image on.
Rounded down, the scaling itself moved 113 to 170 rails a schedule from the small synapses to the large.

**And so is the speed.** Without the split, learning under the zero sum was slower than rounded down and than the
default's: three reversals took 23 to 30 blocks to the nearest and 16 to 24 carried over schedules 9 to 16, against 11
to 13 rounded down and the default's 12 to 16. What made rounded down fast is not read here. A reading, not measured: a
coupling of a few strong synapses and many near zero is the bimodal regime of an additive rule, which ADR-0055 sets
against its own rule's unimodal one and records the weaker competition among a unit's inputs as its rule's failure mode.
Which regime a task needs it left open, with the depression's exponent between zero and one as the lever.

**The two roundings read differently**:
- **To the nearest**, from the arithmetic, a synapse whose share of a round is under half an LSB keeps its weight, so
  the largest synapses carry every round both ways. The bottom band paid little, the tails grew little, and a third to
  three fifths of the rail's arrivals followed a punishment. Its reversals fell only from 29 and 34 blocks in the third
  schedule to 23 to 28 in the last four.
- **Carried** is the exact scaling to within half an LSB a synapse. Its tails were still growing over schedules 9 to 16,
  the top sixteenth to 95 synapses at the sixteenth image and the rail's most to 73, and its reversals fell from 31 and
  32 blocks in the third schedule to 17 to 21 in the last four. Where either ends was not run.

Questions this suggests for a later exploration or a round:
- carried past sixteen schedules, for where its tails and its reversals stop;
- seeds 23 and 24;
- whether a bimodal coupling is what makes reversals fast, and whether a rule can make one without a rounding's drain:
  ADR-0055's depression exponent between zero and one, under the zero sum;
- the zero sum both ways as a round would build it: in a wider fixed point, or carried, so that no rounding chooses which
  synapses pay.

## What did not run

- **Seeds 23 and 24.**
- **More than sixteen schedules.**
- **The largest remainder, and a remainder per group**, for the reasons above.
- **The fractions carried across schedules.** They are in no image, so each schedule started them again at a half.
- **The ceiling rounded otherwise.** 011's ceiling scales down alone and rounds down; it was not run again.
- **The control from the mirrored assignment, or past its eighth schedule.** Rounded down's counters by band are from
  the assignment, schedules 1 to 8.
- **The default's synapses by sixteenths of the width**, which no run here read.
- **The rate's error**, and the inhibitory rule's replay terms, which the logs hold and this note does not read.
- **The nights** of ADR-0044 and ADR-0048 under the zero sum.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_zero_sums_rounding_short_on_seed_22_explore` | [40641556801](https://github.com/Menteion/Menteion/actions/runs/40641556801) | `c68a9d5` | 194 |
| `the_zero_sum_rounded_to_the_nearest_on_seed_22_from_the_assignment_explore` | [40641556801](https://github.com/Menteion/Menteion/actions/runs/40641556801) | `c68a9d5` | 5676 |
| `the_zero_sum_rounded_to_the_nearest_on_seed_22_from_the_mirrored_assignment_explore` | [40641556801](https://github.com/Menteion/Menteion/actions/runs/40641556801) | `c68a9d5` | 5956 |
| `the_zero_sum_with_its_fractions_carried_on_seed_22_from_the_assignment_explore` | [40641556801](https://github.com/Menteion/Menteion/actions/runs/40641556801) | `c68a9d5` | 5900 |
| `the_zero_sum_with_its_fractions_carried_on_seed_22_from_the_mirrored_assignment_explore` | [40641556801](https://github.com/Menteion/Menteion/actions/runs/40641556801) | `c68a9d5` | 5875 |
| `the_zero_sum_rounded_down_again_on_seed_22_from_the_assignment_explore` | [40641556801](https://github.com/Menteion/Menteion/actions/runs/40641556801) | `c68a9d5` | 5718 |
| `the_zero_sum_rounded_to_the_nearest_second_half_on_seed_22_from_the_assignment_explore` | [40653304865](https://github.com/Menteion/Menteion/actions/runs/40653304865) | `96ca056` | 5823 |
| `the_zero_sum_rounded_to_the_nearest_second_half_on_seed_22_from_the_mirrored_assignment_explore` | [40653304865](https://github.com/Menteion/Menteion/actions/runs/40653304865) | `96ca056` | 5845 |
| `the_zero_sum_with_its_fractions_carried_second_half_on_seed_22_from_the_assignment_explore` | [40653304865](https://github.com/Menteion/Menteion/actions/runs/40653304865) | `96ca056` | 5617 |
| `the_zero_sum_with_its_fractions_carried_second_half_on_seed_22_from_the_mirrored_assignment_explore` | [40653304865](https://github.com/Menteion/Menteion/actions/runs/40653304865) | `96ca056` | 5828 |

Rounded down's rows are exploration 011's runs [40614819069](https://github.com/Menteion/Menteion/actions/runs/40614819069)
and [40625863565](https://github.com/Menteion/Menteion/actions/runs/40625863565); the default's are exploration 005's
and 007's, as 011 tabled them.
