---
status: exploratory
date: 2026-10-27
---

# 007: Sixteen schedules and a soft bound

**Branch**: `explore/007-sixteen-schedules-and-a-soft-bound`
**Commit**: `6119910` for the long run's first halves, the soft bounds' arms and the short carry's first half and run in
one go; `b98c65c` for the short carry's second half and `aec0c43` for the long run's second halves, each of which adds
the images the runs before it wrote and their CRC-64s and nothing else that runs; `0bbccf7` for the rail paths, which
adds their reader. All four carry `6c57976`, the soft bound under `src/`. Unset, the bound is the rule bit for bit, and
the long run held exploration 005's traces under it.
**Runs**: [40464237750](https://github.com/Menteion/Menteion/actions/runs/40464237750) (the short carry's first half and
its run in one go, and the soft bounds short), [40465823549](https://github.com/Menteion/Menteion/actions/runs/40465823549)
(the short carry's second half), [40464298839](https://github.com/Menteion/Menteion/actions/runs/40464298839) (the long
run's first halves and the soft bounds' arms), [40479042086](https://github.com/Menteion/Menteion/actions/runs/40479042086)
(the long run's second halves) and [40479387831](https://github.com/Menteion/Menteion/actions/runs/40479387831) (the rail
paths), all at `scope=arms`

Exploration 005 ran eight of H-20's schedules on seed 22 under the default. The couplings rose by about 0.05 a schedule
with no sign of levelling off, the highest to 1.64 to 1.66 of the image's. A growing number of synapses at Q1.15's
rail carried the rise, up to 100 of a coupling's 800, while learning got faster
([005](005-eight-schedules-under-the-default.md)). M12 asks whether the couplings and the inhibitory sum stay bounded
over many schedules (whitepaper Appendix C). This note asks three things on seed 22:

1. over sixteen schedules, whether the couplings stop, whether the synapses at the rail stop accumulating, and whether
   learning begins to slow or to fail;
2. whether a soft bound on the excitatory potentiation, smaller the nearer a weight stands to the rail, holds the
   couplings without harming learning: over the whole width, and in the width's top eighth alone;
3. added when both bounds left synapses at the rail: after which sign of the reward's signal a coupling's synapses
   reach the rail.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host.

**In brief**
- **Over sixteen schedules nothing stops.** The highest coupling rose to 2.06 and 2.07 of the image's in the sixteenth
  schedule, by about 0.05 a schedule over the last eight as over the four before them; the lowest to 1.57. The most
  synapses of one coupling at exactly 32 767 at a block's end went from 94 and 100 in the eighth schedule to 170 and 186
  in the sixteenth, about ten a schedule.
- **Learning neither slowed nor failed.** Every mapping of all 32 schedules was learned, from the ninth schedule at 125
  to 128 of its last 128. The three reversals of a schedule summed 15 and 16 blocks in the eighth and 12 and 13 in the
  sixteenth. Clauses 1, 3 and 4 held in every schedule; clause 2 failed from the second or the third.
- **The inhibitory sum turned.** It stood at 0.549 to 0.556 of the image's over the fourth to the eighth schedule, and
  rose from the ninth, by about 0.005 a schedule, to 0.597 and 0.589 in the sixteenth. The rise is onto the readout
  units, whose share of their spikes in the task's ticks went from 21 to 26 % in the eighth schedule to 34 to 37 % in
  the sixteenth while their rate held.
- **The bound over the width holds the couplings and loses learning.** Over eight schedules the highest coupling stayed
  at 1.05 to 1.17, and clause 2 held in all sixteen schedules of the two arms. From the assignment the network learned
  two of the four mappings in each schedule, and from the fifth on was correct in 116 to 125 of 128 under the mirrored
  mapping and in 5 to 15 under the assignment: it stopped reversing. From the mirrored assignment it learned none from
  the second schedule on, at 56 to 78 of 128.
- **The bound in the top eighth does neither.** The couplings rose higher than the default's, to 1.72 and 1.74 in the
  eighth schedule. Learning was slower: the reversals summed 28 to 53 blocks where they reached 40 of 64, against 15 to
  40, and in four schedules a mapping stood under 100 of its last 128, in two under 80.
- **Under both bounds every synapse that reached the rail did so after a punishment**, in the four schedules read from
  the assignment; under the default 96 to 98 % of the arrivals followed a reward. A reading: under the signed gate a
  signal below zero moves a weight against its trace, and near the rail ADR-0055's depression makes the trace negative,
  so a punishment raises the weight, by a path a bound on the pairing's potentiation does not reach.

## What ran

**The network, the arm and the path** are exploration 005's. Every test starts from H-42's network of seed 22 at 1 024
units, the image an engine born under `Config::reads_its_targets()` settles. Each test first runs that derivation again
and holds it to H-42's pinned tables (`probe_made`), and all fifteen held. Every arm of sections 1 and 2 that starts
from H-42's image, and every short run that does, printed the CRC-64 of that image, `0x750105a595510fa3` in all. The arm
is H-43's twin on time, H-42's arm bit for bit, from the assignment and from the mirrored assignment. A schedule ends
with the engine run quiet without the drive until it is quiescent; its image is written, and the next schedule's engine
is decoded from it. No target is read again.

**The harness** is exploration 005's as its tag holds it (`056c76b`), carried to this branch up to its long run, with
its readings and its lines: per block the four couplings, the inhibitory sum, the gain and the four couplings against
the rail; per image the couplings' weights by sixteenths of the width and every inhibitory synapse's weight; the
inhibitory rule replayed from the spike train and held to the engine's blocks at every block's end. Beside them, each
image's CRC-64 and a summary line per schedule.

**Sixteen schedules in two halves.** Sixteen schedules of an arm take about 14 100 s beside the derivation and the
build, past a job's 240 minutes, so the long run ran as two halves of eight:
- The first half started from H-42's image and printed its eighth image whole, in hexadecimal. The branch keeps the
  590 656 bytes in `runtime/menteion-runtime/tests/explore-007/`, CRC-64 `0xc4a4465d974fb445` from the assignment and
  `0x4d8cc10030ff6f31` from the mirrored one.
- The second half read that file, held it to the CRC-64 the first half printed, decoded it and ran schedules 9 to 16.

**The carry was shown first, short.** Two schedules of two blocks each, run once in one go, and once as one schedule
whose image the branch kept and then one from that image, in another job on another commit:
- the first schedule's image was the same in both, CRC-64 `0x841f164eb6fdefb2`;
- every one of the second schedule's 21 lines, its blocks, trace, windows, rails, synapses by group, replay and image,
  was the same in both, and so was its image, `0x3b66f0e0c0f2fcd1`.

**The first half is exploration 005.** Each schedule's trace and value hash was held to exploration 005's run
40300651755, and every one of that run's 195 lines, per arm, is in this run's log as it was: the derivation, every
schedule's blocks, windows, rails and replay, and every image's weights. The first schedule was held to H-42's arm.

**The soft bound, under `src/` on this branch alone.** Commit `6c57976` adds `SoftBound` and `Executor::set_soft_bound`:
- An excitatory pairing's potentiation, the amount $A_+ (1 - 2^{-11})^{q - p}$ the trace gains where the target fired
  after the previous presynaptic spike, is scaled by the room the slot's magnitude $M$ before the pairing leaves below
  the rail $R$, 32 767. That is the magnitude ADR-0055's depression reads.
- **Over the width**: by $(R - M)/R$, the $\mu = 1$ potentiation of Gütig et al. 2003 beside ADR-0055's $\mu = 1$
  depression.
- **In the top eighth**: by $(R - M)/(R - T)$ from $T$, seven eighths of the width (28 672), to the rail, and not at all
  below $T$; whole at $T$ and nothing at the rail.
- Integers, rounded to nearest with a half going up. The depression, the inhibitory rule and consolidation do not move.
- The bound is in no image, so the test sets it on every schedule's engine. Unset, the pairing is the rule's bit for
  bit: a unit test holds it, and the long run, which leaves it unset, held exploration 005's traces.

Each bound ran eight schedules from H-42's image, from both arms, read as the long run is read.

**The rail paths.** Four schedules from H-42's image from the assignment, under the default and under each bound. A
reward is given at a trial's end and its signal reaches the synapses over the next trial's ticks. So at every trial's
end, each synapse of the four couplings is read against its weight at the trial before, and what it did is set against
the sign of the signal the earlier trial's reward left: the weight gained and lost, and the synapses that arrived at the
rail and that left it. The default's four schedules were held to exploration 005's traces and values, and each bound's
are its arm's, the same traces and values.

## 1. Sixteen schedules

At each schedule's end, over the first image's:

| Arm | Schedules | Inhibitory sum | Highest coupling | Lowest coupling | Gain |
| :--- | :--- | :--- | :--- | :--- | :--- |
| assignment | 1 to 8 | 0.743, 0.605, 0.569, 0.556, 0.554, 0.554, 0.554, 0.556 | 1.239, 1.350, 1.428, 1.461, 1.520, 1.592, 1.619, 1.658 | 0.903, 0.930, 0.985, 1.036, 1.090, 1.151, 1.207, 1.286 | 1.7062, 1.7025, 1.7002, 1.6947, 1.6902, 1.6899, 1.6820, 1.6804 |
| assignment | 9 to 16 | 0.559, 0.563, 0.567, 0.573, 0.578, 0.583, 0.590, 0.597 | 1.702, 1.786, 1.839, 1.878, 1.928, 2.001, 2.018, 2.072 | 1.296, 1.363, 1.411, 1.436, 1.471, 1.495, 1.533, 1.572 | 1.6752, 1.6706, 1.6687, 1.6679, 1.6629, 1.6621, 1.6594, 1.6544 |
| mirrored | 1 to 8 | 0.744, 0.602, 0.564, 0.553, 0.549, 0.550, 0.552, 0.556 | 1.248, 1.298, 1.374, 1.431, 1.444, 1.521, 1.564, 1.637 | 0.889, 0.956, 0.970, 1.018, 1.070, 1.115, 1.150, 1.179 | 1.7073, 1.7036, 1.6996, 1.6941, 1.6941, 1.6913, 1.6862, 1.6813 |
| mirrored | 9 to 16 | 0.559, 0.560, 0.564, 0.567, 0.571, 0.577, 0.583, 0.589 | 1.682, 1.737, 1.783, 1.835, 1.915, 1.983, 2.019, 2.061 | 1.222, 1.329, 1.360, 1.397, 1.434, 1.482, 1.522, 1.569 | 1.6739, 1.6733, 1.6706, 1.6677, 1.6635, 1.6600, 1.6592, 1.6560 |

The highest and the lowest coupling are over every block of the schedule and the four pairs; the gain is the one the
last block's end held. Schedules 1 to 8 are exploration 005's table, every figure of it.

- **The couplings do not level off.** The highest rose by 0.055 and 0.050 a schedule from the eighth to the twelfth, and
  by 0.049 and 0.057 from the twelfth to the sixteenth; from the fourth to the eighth it had risen by 0.049 and 0.052.
  The lowest rose by 0.036 and 0.049 a schedule over the last eight. Each coupling rose in every arm, so the whole band
  moved up, by 0.29 to 0.42 of the image's from the eighth schedule to the sixteenth.
- **The gain kept falling**, by about 0.003 a schedule, as the rate's rule held the count at the target.
- **The inhibitory sum stopped for five schedules and then rose**, by 0.041 and 0.033 over the last eight, about 0.005
  a schedule.

**The rail.** The most synapses of one coupling at exactly 32 767 and at or above seven eighths of it at a block's end,
and the mean weight at the schedule's last block over the rail, as the range over the four couplings:

| Arm | Schedules | At the rail | At or above 7/8 | Mean weight over the rail |
| :--- | :--- | :--- | :--- | :--- |
| assignment | 1 to 8 | 26, 37, 61, 55, 80, 85, 98, 100 | 42, 60, 83, 81, 97, 114, 118, 130 | 0.23–0.30 to 0.32–0.41 |
| assignment | 9 to 16 | 125, 133, 135, 148, 152, 159, 162, 186 | 138, 166, 165, 178, 195, 187, 200, 213 | 0.33–0.41, 0.35–0.44, 0.36–0.44, 0.37–0.45, 0.37–0.46, 0.38–0.47, 0.39–0.47, 0.41–0.49 |
| mirrored | 1 to 8 | 24, 41, 48, 69, 54, 64, 77, 94 | 42, 56, 73, 83, 78, 90, 113, 130 | 0.24–0.29 to 0.30–0.41 |
| mirrored | 9 to 16 | 104, 127, 131, 143, 166, 156, 165, 170 | 135, 156, 160, 174, 181, 183, 201, 209 | 0.33–0.41, 0.34–0.42, 0.35–0.43, 0.36–0.44, 0.37–0.44, 0.38–0.46, 0.39–0.47, 0.40–0.48 |

The counts are of 775 to 809 synapses a coupling.

- **The synapses at the rail keep accumulating**, by about ten a schedule over the last eight, and 6 to 11 over the four
  before them. A coupling's top sixteenth of the width held 0 to 122 synapses at the eighth image and 2 to 207 at the
  sixteenth; on the two pairs the last mapping rewarded, 96 to 122 and 195 to 207.
- **The bulk moves up too.** The mean weight rose by about 0.01 of the rail a schedule throughout. A coupling at 2.07 of
  the image's stands about halfway to its ceiling: with every synapse at the rail it would be 4.0 to 4.3.

**Learning, schedule by schedule:**

| Arm | Schedules | Lowest of the mappings' last 128 correct | Three reversals summed, blocks | The schedule's first mapping, blocks to 40 of 64 |
| :--- | :--- | :--- | :--- | :--- |
| assignment | 1 to 8 | 120, 121, 124, 126, 126, 127, 124, 127 | 34, 27, 26, 21, 20, 17, 17, 15 | 4, 11, 11, 10, 7, 7, 6, 5 |
| assignment | 9 to 16 | 127, 127, 127, 127, 127, 126, 128, 128 | 15, 15, 13, 15, 13, 14, 13, 13 | 6, 5, 5, 5, 5, 5, 5, 4 |
| mirrored | 1 to 8 | 120, 122, 123, 123, 126, 122, 126, 126 | 40, 33, 31, 22, 24, 20, 20, 16 | 2, 12, 10, 9, 8, 7, 7, 6 |
| mirrored | 9 to 16 | 127, 126, 127, 126, 125, 127, 128, 127 | 16, 13, 16, 15, 13, 14, 12, 12 | 6, 5, 5, 5, 4, 4, 4, 4 |

- **Learning got no worse.** Every mapping of every schedule was learned, in the last eight at 125 to 128 of its last
  128. Clauses 1, 3 and 4 held in all 32 schedules; clause 2 failed from the second schedule from the assignment and
  from the third from the mirrored one, as in exploration 005.
- **Its speed levelled off.** From the ninth schedule on every mapping, each a reversal, took 4 to 6 blocks to reach 40
  of 64, where a reversal had taken 11 to 14 in the first schedule and 5 to 7 in the sixth to the eighth.
- **The rate's error widened a little.** From each schedule's 120th window its standard deviation was 2.79 to 3.25 % of
  the target over the last eight schedules, against 2.64 to 2.90 % over the first eight; its lag-1 autocorrelation was
  −0.04 to −0.14.

**Where the inhibitory sum's rise goes.** By the group of the unit an inhibitory synapse reaches, its magnitude at the
schedule's last block over the first image's, the share of the group's spikes in the task's ticks, and its rate over
the inhibitory rule's target, from the assignment. The mirrored arm's magnitudes are within 0.014 of them but onto the
readouts' inhibitory units, by up to 0.10, its shares within 4 points and its rates within 0.04:

| Group | Inhibitory magnitude, schedules 8, 12, 16 | Spikes in the task's ticks, schedules 8, 12, 16 | Rate over the target, schedules 8, 16 |
| :--- | :--- | :--- | :--- |
| Stimulus A | 1.412, 1.387, 1.364 | 78, 80, 81 % | 2.28, 2.19 |
| Stimulus B | 1.422, 1.412, 1.392 | 81, 82, 84 % | 2.21, 2.14 |
| Readout 0, excitatory | 0.444, 0.455, 0.478 | 26, 31, 37 % | 0.81, 0.81 |
| Readout 0, inhibitory | 0.500, 0.572, 0.628 | 23, 30, 35 % | 1.08, 1.13 |
| Readout 1, excitatory | 0.429, 0.446, 0.473 | 22, 28, 34 % | 0.76, 0.76 |
| Readout 1, inhibitory | 0.654, 0.708, 0.747 | 32, 39, 42 % | 1.21, 1.22 |

- The inhibition onto the readout units rose, most onto their inhibitory units, and onto the stimulus units it fell a
  little from the rail it reached in the first schedule.
- The readouts' rate held at 0.76 to 0.81 of the target, as in exploration 005, while more of their spikes came in the
  task's ticks. Exploration 005 read the inhibitory rule keeping the synapses whose target fires just before the
  inhibitory unit; a reading, not run here, is that more of those pairings come as the task's volleys grow with the
  couplings.

## 2. The soft bound

Eight schedules from H-42's image under each bound, beside the default, which is the long run's first half. Each cell
lists the eight schedules in order:

| Rule | Arm | Highest coupling | At the rail, most | Lowest of the mappings' last 128 correct | Mappings learned | Three reversals summed, blocks |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Default | assignment | 1.239, 1.350, 1.428, 1.461, 1.520, 1.592, 1.619, 1.658 | 26, 37, 61, 55, 80, 85, 98, 100 | 120, 121, 124, 126, 126, 127, 124, 127 | 4 in each | 34, 27, 26, 21, 20, 17, 17, 15 |
| Default | mirrored | 1.248, 1.298, 1.374, 1.431, 1.444, 1.521, 1.564, 1.637 | 24, 41, 48, 69, 54, 64, 77, 94 | 120, 122, 123, 123, 126, 122, 126, 126 | 4 in each | 40, 33, 31, 22, 24, 20, 20, 16 |
| Width | assignment | 1.066, 1.056, 1.085, 1.128, 1.136, 1.167, 1.164, 1.163 | 10, 14, 12, 15, 9, 12, 4, 12 | 53, 66, 65, 41, 15, 9, 5, 5 | 2 in each | —, —, 68, —, —, —, —, — |
| Width | mirrored | 1.048, 1.055, 1.083, 1.091, 1.108, 1.130, 1.133, 1.151 | 17, 15, 7, 12, 7, 14, 13, 7 | 72, 61, 56, 64, 60, 62, 60, 61 | 2, then 0 | — in each |
| Top eighth | assignment | 1.267, 1.406, 1.483, 1.528, 1.592, 1.658, 1.699, 1.716 | 21, 23, 40, 37, 37, 51, 69, 66 | 109, 113, 115, 107, 85, 123, 126, 124 | 4 in each | 53, 46, 47, 31, 39, 30, 40, 29 |
| Top eighth | mirrored | 1.243, 1.277, 1.416, 1.528, 1.577, 1.572, 1.625, 1.740 | 22, 27, 42, 35, 56, 54, 44, 87 | 64, 65, 92, 121, 122, 125, 127, 123 | 3, 3, then 4 | —, 33, 47, 36, 38, 28, 31, 44 |

The highest coupling is over every block's end of the schedule and the four pairs, over the first image's; "at the
rail" is the most synapses of one coupling at exactly 32 767 at a block's end; "—" marks a schedule in which a reversal
never reached 40 of 64.

**Over the width.**
- **The couplings were held.** The highest stayed at 1.05 to 1.17, and rose by 0.009 and 0.015 a schedule from the
  fourth schedule to the eighth, against the default's 0.049 and 0.052. Clause 2 held in every schedule of both arms.
  At most 4 to 17 synapses of a coupling stood at the rail at a block's end, and at most 24 at or above seven eighths.
- **Learning was lost.** Clauses 1 and 3 failed in every schedule of both arms; clause 4 held.
  - From the assignment the network learned the first and the third mapping in the first two schedules, the
    assignment's, and the second and the fourth from the third on, the mirrored mapping's. From the fifth on it was
    correct in 116 to 125 of 128 under the mirrored mapping and in 5 to 15 under the assignment: it held the mirrored
    mapping and stopped reversing.
  - From the mirrored assignment it learned two mappings in the first schedule and none after, at 56 to 78 of 128,
    about the 64 of chance.
- **The bound reaches the bulk.** Scaled by $(R - M)/R$, the potentiation at the image's mean magnitude, about a quarter
  of the rail, is three quarters of the rule's, while the depression is the rule's. At ADR-0055's reference magnitude
  $A_- / A_+$ is 1.05; under the bound it is 1.40. From the assignment, at the eighth image the assignment's two pairs
  had sunk: 194 and 220 synapses in the bottom eighth of the width, against 58 and 61 under the default. The inhibitory
  sum fell further than the default's, to 0.513 and 0.517, and was still falling.

**In the top eighth.**
- **The couplings were not held.** The highest rose faster than the default's, to 1.72 and 1.74 in the eighth schedule.
  Fewer synapses stood at the rail at a block's end, at most 66 and 87 in the eighth schedule against 100 and 94, but
  more at or above seven eighths, 139 and 159 against 130. The bulk rose as under the default, its mean weight 0.34 to
  0.41 and 0.32 to 0.43 of the rail at the eighth schedule's end.
- **Learning was slower and missed.** Every reversal from the assignment reached 40 of 64, in 29 to 53 blocks summed
  where the default took 15 to 34. From the mirrored assignment one mapping was not learned in each of the first two
  schedules, at 64 and 65 of 128, and one reversal of the first schedule never reached 40 of 64. Clause 1 failed in
  those two schedules and clause 3 in the first schedule of each arm.

## 3. Where the synapses at the rail come from

The four couplings' synapses over each of four schedules from the assignment, by the sign of the signal the trial
before left. The weight is summed over the four couplings in rails, units of 32 767; "to" is the synapses that arrived
at the rail between two trials' ends, "off" those that left it:

| Rule | Schedule | Trials after a punishment | After a reward: gained, lost; to, off | After a punishment: gained, lost; to, off |
| :--- | ---: | ---: | :--- | :--- |
| Default | 1 | 33 % | 925, 486; 2 619, 1 020 | 382, 744; 92, 1 690 |
| Default | 2 | 31 % | 910, 435; 3 321, 1 441 | 349, 774; 119, 1 974 |
| Default | 3 | 27 % | 837, 355; 3 287, 1 490 | 307, 756; 109, 1 928 |
| Default | 4 | 24 % | 776, 300; 4 739, 2 471 | 257, 684; 106, 2 368 |
| Width | 1 | 52 % | 780, 737; 0, 396 | 646, 678; 412, 0 |
| Width | 2 | 51 % | 769, 724; 0, 427 | 616, 649; 421, 0 |
| Width | 3 | 49 % | 791, 746; 0, 419 | 623, 657; 410, 0 |
| Width | 4 | 50 % | 741, 707; 0, 390 | 592, 621; 392, 0 |
| Top eighth | 1 | 44 % | 977, 575; 0, 1 132 | 438, 743; 1 162, 0 |
| Top eighth | 2 | 42 % | 1 013, 589; 0, 1 379 | 447, 830; 1 385, 0 |
| Top eighth | 3 | 39 % | 972, 527; 0, 1 875 | 410, 792; 1 911, 0 |
| Top eighth | 4 | 33 % | 902, 465; 0, 1 557 | 348, 742; 1 573, 0 |

The first trial of each schedule, which follows the lead-in, and the few trials whose reward left no signal are counted
apart: after them none arrived at the rail but four under the width, and up to 20 left it under the top eighth.

- **Under the default 96 to 98 % of the arrivals at the rail followed a reward.** A synapse reached the rail and left it
  many times: 2 711 to 4 845 arrivals in a schedule, where at most 26 to 61 of a coupling stood there at a block's end.
  A punishment took more synapses off the rail than a reward did in the first three schedules, and about as many in the
  fourth.
- **Under both bounds every arrival followed a punishment, and every departure a reward.** A reading of why: under the
  signed gate a signal below zero moves a weight against its trace. Near the rail ADR-0055's depression, four times its
  reference amount at the rail, makes a slot's trace negative wherever its target fires before it. A punishment then
  raises the weight, and the bound, which scales the pairing's potentiation, does not reach it. A reward then
  consolidates the negative trace and takes the weight off the rail.
- **The punishments' share of the weight gained is about the same under the default and the top eighth**, 25 to 31 %,
  and 44 to 45 % under the width, where half the trials were punished. Under the width a reward netted 34 to 45 rails of
  weight a schedule over the four couplings, against 439 to 482 under the default: the bound took most of what a reward
  adds.
- **The reading is approximate where a signal outlives a trial.** What moved between two trials' ends is set against
  the earlier trial's signal alone. Under the bounds no synapse arrived at the rail after a reward in 7 680 trials a
  schedule.

## What it suggests

**For M12's couplings, the default has no bound in sixteen schedules.** The couplings rose at about 0.05 a schedule
throughout, the synapses at the rail accumulated by about ten a schedule, and the bulk's mean weight rose by about 0.01
of the rail a schedule. Learning was unharmed: every mapping learned, the reversals' speed levelled at 4 to 6 blocks.
A hypothesis that the couplings stay under a bound would read no under the default at clause 2's 1.30 from the second
or third schedule; the sixteen schedules show no level at which a higher bound would hold.

**The inhibitory sum's stop is not where it ends.** It stood at about 0.55 for five schedules and rose by about 0.005 a
schedule from the ninth, onto the readout units, as the task's share of their spikes grew. Exploration 005's reading,
that the sum stops once the rule has sorted its synapses, holds over its eight schedules; over sixteen it turns.

**A bound on the pairing's potentiation does not keep synapses off the rail.** Under the signed gate a punishment moves
a negative trace into the weight. That path carried every arrival at the rail under both bounds in the four schedules
read, and 2 to 4 % of them under the default. Questions this suggests for a later exploration or a round:
- a bound on the magnitude at consolidation, every rise of it scaled by the room below the rail whichever the
  signal's sign;
- a bound over the width normalised to leave the rule as it is at the reference magnitude, so that the bulk keeps
  ADR-0055's balance, since the bound over the width held the couplings by tilting the whole rule toward depression and
  lost learning with it;
- whether the signed gate's potentiation of negative traces near the rail is meant, since ADR-0055's depression is
  largest there.

## What did not run

- **Seeds 23 and 24.** The question named seed 22, and exploration 005's rule extends an item only where a reading
  differs from the earlier run's. The first eight schedules are exploration 005's; the readings past them have no
  earlier run to differ from.
- **More than sixteen schedules**, and so where, if anywhere, the couplings or the inhibitory sum stop.
- **The bounds over sixteen schedules**, or set on an image the default had already ratcheted, such as the eighth.
- **A bound at consolidation**, a bound over the width normalised at the reference magnitude, or a bound with the signed
  gate unset.
- **The rail paths from the mirrored assignment, or past the fourth schedule**; and the replay's terms of the inhibitory
  rule over the second half, which the logs hold and this note does not read.
- **The rate's error over the bounds' arms**, and what widens it over the second half.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `sixteen_schedules_short_in_one_go_on_seed_22_explore` | [40464237750](https://github.com/Menteion/Menteion/actions/runs/40464237750) | `6119910` | 222 |
| `sixteen_schedules_short_first_half_on_seed_22_explore` | [40464237750](https://github.com/Menteion/Menteion/actions/runs/40464237750) | `6119910` | 205 |
| `a_soft_bound_short_on_seed_22_explore` | [40464237750](https://github.com/Menteion/Menteion/actions/runs/40464237750) | `6119910` | 223 |
| `sixteen_schedules_short_second_half_on_seed_22_explore` | [40465823549](https://github.com/Menteion/Menteion/actions/runs/40465823549) | `b98c65c` | 210 |
| `sixteen_schedules_first_half_on_seed_22_from_the_assignment_explore` | [40464298839](https://github.com/Menteion/Menteion/actions/runs/40464298839) | `6119910` | 7288 |
| `sixteen_schedules_first_half_on_seed_22_from_the_mirrored_assignment_explore` | [40464298839](https://github.com/Menteion/Menteion/actions/runs/40464298839) | `6119910` | 7277 |
| `a_soft_bound_over_the_width_on_seed_22_from_the_assignment_explore` | [40464298839](https://github.com/Menteion/Menteion/actions/runs/40464298839) | `6119910` | 7266 |
| `a_soft_bound_over_the_width_on_seed_22_from_the_mirrored_assignment_explore` | [40464298839](https://github.com/Menteion/Menteion/actions/runs/40464298839) | `6119910` | 7244 |
| `a_soft_bound_in_the_top_eighth_on_seed_22_from_the_assignment_explore` | [40464298839](https://github.com/Menteion/Menteion/actions/runs/40464298839) | `6119910` | 7324 |
| `a_soft_bound_in_the_top_eighth_on_seed_22_from_the_mirrored_assignment_explore` | [40464298839](https://github.com/Menteion/Menteion/actions/runs/40464298839) | `6119910` | 7163 |
| `sixteen_schedules_second_half_on_seed_22_from_the_assignment_explore` | [40479042086](https://github.com/Menteion/Menteion/actions/runs/40479042086) | `aec0c43` | 7198 |
| `sixteen_schedules_second_half_on_seed_22_from_the_mirrored_assignment_explore` | [40479042086](https://github.com/Menteion/Menteion/actions/runs/40479042086) | `aec0c43` | 7218 |
| `where_the_couplings_rise_on_seed_22_explore` | [40479387831](https://github.com/Menteion/Menteion/actions/runs/40479387831) | `0bbccf7` | 3722 |
| `where_the_couplings_rise_under_the_soft_bound_over_the_width_on_seed_22_explore` | [40479387831](https://github.com/Menteion/Menteion/actions/runs/40479387831) | `0bbccf7` | 3782 |
| `where_the_couplings_rise_under_the_soft_bound_in_the_top_eighth_on_seed_22_explore` | [40479387831](https://github.com/Menteion/Menteion/actions/runs/40479387831) | `0bbccf7` | 3866 |

Exploration 005's eight schedules were read again from run
[40300651755](https://github.com/Menteion/Menteion/actions/runs/40300651755) on `056c76b`.
