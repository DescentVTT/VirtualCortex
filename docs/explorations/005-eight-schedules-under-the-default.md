---
status: exploratory
date: 2026-10-26
---

# 005: Eight schedules under the default

**Branch**: `explore/005-eight-schedules-under-the-default`
**Commit**: `056c76b` for every run; it carries `fa35c0d`, the harness, and adds the inhibitory rule's replay. Nothing
under `src/` differs from `main`.
**Runs**: [40300630723](https://github.com/DescentVTT/VirtualCortex/actions/runs/40300630723) (the harness, short),
[40300651755](https://github.com/DescentVTT/VirtualCortex/actions/runs/40300651755) (eight schedules) and
[40300672820](https://github.com/DescentVTT/VirtualCortex/actions/runs/40300672820) (the step halved), all on `056c76b`

Exploration 004 ran four of H-20's schedules back to back under the default and read the four couplings rising like a
ratchet, with no sign of stopping, and the inhibitory sum falling by less each schedule
([004](004-long-runs-under-the-default.md)). M12 asks whether the couplings and the inhibitory sum stay bounded over
many schedules (whitepaper Appendix C). This note asks three things on seed 22:

1. over eight schedules, whether the couplings' rise levels off and where; how far their weights stand from Q1.15's
   rail; and whether learning keeps getting faster or begins to get worse;
2. where the inhibitory sum's fall comes from: the inhibitory synapses by the unit they reach, against the period the
   inhibitory rule runs to, and whether it has to do with the spikes the task brings in;
3. whether the probe's $\lambda$ halved, an eighth where it is a quarter, makes the rate's error as near to white as
   exploration 004's sign rule did.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host. The rule for this exploration ran seed 22 first and would have extended an item to seeds 23
and 24 only where its reading differed from exploration 004's; none did ([What did not run](#what-did-not-run)).

**In brief**
- **The inhibitory sum stops; the couplings do not.** From the fourth schedule on, the inhibitory sum stood at 0.549 to
  0.556 of the image's. The four couplings rose through all eight schedules: the highest to 1.64 to 1.66 of the
  image's in the eighth, by about 0.05 a schedule over the last four, and the lowest to 1.18 to 1.29.
- **Learning kept getting faster.** Every mapping of all sixteen schedules was learned. A schedule's three reversals
  summed 34 and 40 blocks in the first, 15 and 16 in the eighth.
- **Single synapses reach the rail; the bulk does not.** At a block's end in the first schedule, up to 15 to 26
  synapses of each coupling stood at exactly 32 767, and in the eighth up to 51 to 100. The rest rose slowly: the mean
  weight went from 0.23 to 0.25 of the rail to 0.30 to 0.41. At the rail, every synapse would make a coupling 4.0 to
  4.3 times the image's; it reached 1.66.
- **The fall is onto the excitatory readout units, and it comes from the target.** The inhibitory rule runs to the
  network's mean period. The stimulus units, a tenth of the units, fire at 2.2 to 2.4 times that rate and make 22 to 24 %
  of all spikes, three quarters of theirs in the task's ticks; the excitatory readout units, seven tenths, fire at 0.76
  to 0.81 of it in every schedule. Onto them the inhibitory magnitude fell to 0.43 to 0.44 of the image's; onto the
  stimulus units it rose to the rail in the first schedule.
- **The sum stops when the rule has sorted its synapses to its two bounds.** By the eighth image 29 to 31 % of the
  synapses onto the excitatory readout units were at zero and 27 % at or above seven eighths of the rail. What sorts
  them is a target spike shortly before the inhibitory spike: 2.2 to 2.5 times the depression per presynaptic spike on
  the synapses that end at or above seven eighths, against 0.35 to 0.38 on those that end at zero. Their targets fire
  at 1.0 to 1.26 and at 0.56 to 0.64 of the target rate, with 15 to 40 % and 6.5 to 8.5 % of their spikes in the
  task's ticks.
- **The step halved made the error white, at a longer transient.** From the 120th window its lag-1 autocorrelation was
  −0.007 and +0.010, against the probe's −0.089 and −0.057 and the sign rule's −0.024 and −0.020, and its standard
  deviation 2.55 and 2.60 % against 2.64 and 2.66 %. Learning was the probe's. The gain took 29 to 35 windows to settle
  where the probe's took 14 to 20.

## What ran

**The network, the arm and the path** are exploration 004's. Every run starts from H-42's network of seed 22 at 1 024
units, the image an engine born under `Config::reads_its_targets()` settles; each test first runs that derivation again
and holds it to H-42's pinned tables (`probe_made`). The arm is H-43's twin on time, H-42's arm bit for bit, from the
assignment and from the mirrored one. A schedule ends with the engine run quiet without the drive until it is
quiescent, 2 314 to 2 699 ticks; its image is written, and the next schedule's engine is decoded from it. No target was
read again. Each long run's first schedule was held to H-42's arm, its 120 blocks, its trace and every reward's value,
and the first four schedules are exploration 004's long runs in every figure both read.

**What was read**, beside exploration 004's readings per block, per window and per schedule:
- **The four couplings against the rail.** At every block's end, each coupling's synapses: how many, the highest
  weight, and how many stood at or above three quarters of Q1.15's rail, at or above seven eighths, and at the rail,
  32 767. At every image, each coupling's weights by sixteenths of the width.
- **The synapses by the group of the unit they reach.** Stimulus A, stimulus B, readout 0's excitatory units and its
  inhibitory ones, readout 1's, and the four units past the last whole period of twenty, outside every set. The geometry
  gives each readout nine places of every twenty, two of them inhibitory, and each stimulus one. At the load and at
  every block's end: the inhibitory magnitude onto each group, its synapses at zero, and the excitatory sum onto it.
- **Each unit's spikes** over the schedule, and those in the first 600 ticks of each trial, from the injection to the
  readout window's close: the task's ticks, 3.7 % of a trial.
- **Every inhibitory synapse's weight** at every image, 6 528 synapses from the 204 inhibitory units.
- **The inhibitory rule, replayed.** On the host, from the spike train, at every inhibitory spike $t$ and for every
  synapse of the unit's chain, in the rule's integers: the trace's decay since the block's stamp; the depression of
  the target period, 23 at 57 604 ticks; the gain $A_+ (1 - 2^{-11})^{q - p}$ where the target's last spike $q$ lies
  after the previous presynaptic spike $p$, here called *after the previous*; the gain $A_+ (1 - 2^{-11})^{t - q}$
  where it lies before this spike, *before this*; and half the trace moved into the magnitude, the inhibitory
  baseline. Each term was summed per synapse. At every block's end of every schedule the replay's stamp, trace and
  magnitude were held to the engine's blocks, synapse by synapse, and held in all 2 160 block ends.

**The probe's step halved.** H-42's image with the homeostasis record's step written 539, half the probe's 1 078, and
the section resealed, every other byte the image's: H-44's control's check (`only_the_step`), which held. That is the
probe's $\lambda$ at an eighth. It ran the first schedule, the long run's path, from both arms; its step was 539 in
every window.

## 1. Eight schedules

At each schedule's end, over the first image's:

| Arm | Inhibitory sum | Highest coupling | Lowest coupling | Gain |
| :--- | :--- | :--- | :--- | :--- |
| assignment | 0.743, 0.605, 0.569, 0.556, 0.554, 0.554, 0.554, 0.556 | 1.239, 1.350, 1.428, 1.461, 1.520, 1.592, 1.619, 1.658 | 0.903, 0.930, 0.985, 1.036, 1.090, 1.151, 1.207, 1.286 | 1.7062, 1.7025, 1.7002, 1.6947, 1.6902, 1.6899, 1.6820, 1.6804 |
| mirrored | 0.744, 0.602, 0.564, 0.553, 0.549, 0.550, 0.552, 0.556 | 1.248, 1.298, 1.374, 1.431, 1.444, 1.521, 1.564, 1.637 | 0.889, 0.956, 0.970, 1.018, 1.070, 1.115, 1.150, 1.179 | 1.7073, 1.7036, 1.6996, 1.6941, 1.6941, 1.6913, 1.6862, 1.6813 |

The highest and the lowest coupling are over every block of the schedule and the four pairs; the gain is the one the
last block's end held.

- **The inhibitory sum stopped at about 0.55** after the third schedule and stayed within 0.007 of it for five.
- **The couplings did not level off.** The highest rose by 0.074 and 0.061 a schedule from the first schedule's to the
  fourth's, and by 0.049 and 0.052 from the fourth's to the eighth's. The lowest rose by 0.04 to 0.06 a schedule
  throughout, so the whole band moves up.
- **The gain kept falling**, by about 0.004 a schedule, as the rate's rule held the count at the target: from window
  120 on the count sat within 0.04 % of it in every schedule.

**The rail.** The synapses of each coupling at exactly 32 767 and at or above seven eighths of it, the most at any
block's end, and the mean weight at the schedule's last block, over the rail:

| Schedule | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| At the rail, assignment | 26 | 37 | 61 | 55 | 80 | 85 | 98 | 100 |
| At the rail, mirrored | 24 | 41 | 48 | 69 | 54 | 64 | 77 | 94 |
| At or above 7/8, assignment | 42 | 60 | 83 | 81 | 97 | 114 | 118 | 130 |
| At or above 7/8, mirrored | 42 | 56 | 73 | 83 | 78 | 90 | 113 | 130 |
| Mean weight, assignment | 0.23–0.30 | 0.25–0.32 | 0.26–0.32 | 0.27–0.34 | 0.29–0.36 | 0.30–0.37 | 0.31–0.39 | 0.32–0.41 |
| Mean weight, mirrored | 0.24–0.29 | 0.24–0.32 | 0.25–0.32 | 0.26–0.34 | 0.27–0.35 | 0.29–0.38 | 0.29–0.39 | 0.30–0.41 |

The counts are the largest over the four couplings, of 775 to 809 synapses each; the mean is the range over the four.
The first image's mean was 0.23 to 0.25.

- **A coupling's weights split in two.** Most stay in the bulk, between an eighth and three eighths of the rail at the
  first image and moving up slowly. A growing number go to the top sixteenth of the width under the mapping that rewards
  their pair, to the rail itself, and fall back under the other mapping, many only part of the way: at the eighth
  schedule's image, each pair the last mapping did not reward held 118 to 156 synapses between half and seven eighths
  of the rail, against 10 to 13 at the first image.
- **At an image, the pair the last mapping rewarded carries its rise in the top of the width.** Of the rise from the
  first image to the eighth schedule's, reckoned from the sixteenths at their midpoints, 94 to 101 % on those pairs was
  in synapses at or above seven eighths of the rail; on the pairs the last mapping did not reward, 6 % or less.
- **The coupling as a whole stands far below its ceiling.** At 1.64 to 1.66 of the image's, against 4.0 to 4.3 were
  every synapse at the rail.

**Learning, schedule by schedule:**

| Arm | Lowest of the mappings' last 128 correct | Three reversals summed, blocks | The schedule's first mapping, blocks to 40 of 64 |
| :--- | :--- | :--- | :--- |
| assignment | 120, 121, 124, 126, 126, 127, 124, 127 | 34, 27, 26, 21, 20, 17, 17, 15 | 4, 11, 11, 10, 7, 7, 6, 5 |
| mirrored | 120, 122, 123, 123, 126, 122, 126, 126 | 40, 33, 31, 22, 24, 20, 20, 16 | 2, 12, 10, 9, 8, 7, 7, 6 |

- Every mapping of every schedule was learned, at 120 to 127 of its last 128. Clauses 1, 3 and 4 held in all sixteen
  schedules; clause 2 failed from the second schedule from the assignment and from the third from the mirrored one, as
  in exploration 004.
- From the sixth schedule on, a reversal took 5 to 7 blocks; in the first it took 11 to 14.

**The rate's error** from each schedule's 120th window, 840 windows: lag-1 autocorrelation −0.057 to −0.165, pairs on
opposite sides of the target 51.5 to 57.3 %, standard deviation 2.64 to 2.90 % of the target, in no trend over the
schedules. Exploration 004's reading holds over eight: the whiteness does not follow the drift.

## 2. Where the inhibitory sum goes

By the group of the unit an inhibitory synapse reaches. The magnitude onto the group at the schedule's last block over
the first image's, its rate over the inhibitory rule's target rate, and the share of its spikes in the task's ticks.
The assignment's are shown; the mirrored arm's magnitudes are within 0.013 of them but on the readouts' inhibitory
units, by up to 0.06 and 0.10, its rates within 0.04 and its shares within 4 points:

| Group | Units | Inhibitory synapses onto it | Share of the first image's inhibitory magnitude | Over the first image's, schedules 1, 2, 4, 8 | Rate over the target, schedules 1; 8 | Spikes in the task's ticks, schedules 1; 8 |
| :--- | ---: | ---: | ---: | :--- | :--- | :--- |
| Stimulus A | 51 | 301 | 4.7 % | 1.419, 1.420, 1.417, 1.412 | 2.42; 2.28 | 73; 78 % |
| Stimulus B | 51 | 365 | 5.6 % | 1.439, 1.439, 1.436, 1.422 | 2.36; 2.21 | 76; 81 % |
| Readout 0, excitatory | 357 | 2 413 | 37.5 % | 0.680, 0.520, 0.456, 0.444 | 0.79; 0.81 | 11; 26 % |
| Readout 0, inhibitory | 102 | 493 | 6.9 % | 0.601, 0.481, 0.500, 0.500 | 1.08; 1.08 | 10; 23 % |
| Readout 1, excitatory | 357 | 2 450 | 38.2 % | 0.658, 0.495, 0.434, 0.429 | 0.77; 0.76 | 9; 22 % |
| Readout 1, inhibitory | 102 | 479 | 6.7 % | 0.669, 0.571, 0.555, 0.654 | 1.11; 1.21 | 15; 32 % |
| Outside the sets | 4 | 27 | 0.4 % | 0.790, 0.572, 0.432, 0.419 | 0.85; 0.65 | 8; 8 % |

- **The target is the network's mean.** The read set the inhibitory rule's period to the rate's, 57 604 ticks, and the
  rate's rule held the count at the rate's target throughout: the mean over all 1 024 units was the target rate to
  within 0.2 % in every schedule.
- **A tenth of the units fire far above it, most of it the task's.** The stimulus units made 22.4 to 23.8 % of all
  spikes, three quarters of theirs in the task's ticks, 3.7 % of the time. The readouts' inhibitory units fire a little
  above it. So the excitatory readout units, 714 of the 1 024, fire at 0.76 to 0.81 of the target rate, and did so in
  every schedule: their rate did not move as their inhibition fell.
- **Onto the stimulus units the inhibition went to the rail in the first schedule.** At the first image their
  synapses' mean was 0.69 and 0.70 of the rail; at the second, 0.996, 662 of their 666 synapses at or above seven
  eighths. The 1.42 is that rail.
- **Onto the excitatory readout units it fell to 0.43 to 0.44 and stopped.** It lost 0.32 to 0.34 of the image's in
  the first schedule, 0.16 to 0.17 in the second, 0.04 to 0.05 in the third, and under 0.02 in each of the last five,
  in both arms.
- **The task's ticks took a growing share of the readouts' spikes**, 9 to 15 % in the first schedule and 21 to 32 % in
  the eighth, while their rate held: the evoked spikes grew with the couplings, and the gain took the rest away.

**The rule sorts the synapses onto the excitatory readout units to its two bounds.** At each image, those synapses by
their magnitude, and over the schedule before it their target's rate over the target rate, the share of the target's
spikes in the task's ticks, and the rule's two gains per presynaptic spike over its depression, beside what two
independent trains at the same rates would give:

| Image | Magnitude at the image | Synapses | Target's rate | Target's spikes in the task's ticks | Before this, measured; independent | After the previous, measured; independent |
| :--- | :--- | ---: | ---: | ---: | :--- | :--- |
| 1 | zero | 318 | 0.64 | 6.6 % | 0.36; 0.32 | 0.21; 0.22 |
| 1 | above zero, under 1/8 | 644 | 0.66 | 6.8 % | 0.40; 0.33 | 0.20; 0.22 |
| 1 | 1/8 to 7/8 | 2 862 | 0.74 | 8.4 % | 0.57; 0.37 | 0.16; 0.20 |
| 1 | 7/8 and above | 1 039 | 0.99 | 14.5 % | 2.30; 0.48 | 0.24; 0.26 |
| 8 | zero | 1 459 | 0.56 | 8.5 % | 0.35; 0.28 | 0.21; 0.18 |
| 8 | above zero, under 1/8 | 1 725 | 0.60 | 10.5 % | 0.43; 0.30 | 0.23; 0.19 |
| 8 | 1/8 to 7/8 | 369 | 0.86 | 26.0 % | 0.77; 0.42 | 0.23; 0.24 |
| 8 | 7/8 and above | 1 310 | 1.26 | 39.6 % | 2.49; 0.62 | 0.38; 0.29 |

From the assignment; the mirrored arm's rows differ by at most 7 % in a count, 0.04 in a rate, 3 points in a share and
0.09 in a gain.
"Independent" is the expectation for two Poisson trains at the measured rates $\rho_i$ (the inhibitory unit) and
$\rho_j$ (its target), with $\mu = -\ln(1 - 2^{-11})$: $A_+ \rho_j / (\rho_j + \mu)$ before this, and
$A_+ \rho_i \rho_j / ((\rho_i + \rho_j)(\rho_i + \mu))$ after the previous, each over the depression.

- **What keeps a synapse is its target firing just before the inhibitory unit.** On the synapses that end at or above
  seven eighths, *before this* gave 2.3 to 2.5 times the depression per presynaptic spike, four to five times what
  independent trains would give. Their targets fire at about the target rate or above it, more of it in the task's
  ticks. On those that end at zero it gave 0.35 to 0.36, near independence, onto targets that fire at 0.56 to 0.64 of
  the target rate.
- **The sum stops when the sorting is done, not when the rates reach the target.** In the eighth schedule the
  synapses that ended at zero lost 109 and 157 a synapse on average, those that ended at the top gained 295 and 447,
  and the two groups' sums moved by 0.1 to 1.5 % of their magnitude. The excitatory readout units still fire at 0.76 to
  0.81 of the target rate.
- **The rule as built is depression-dominant at its target.** By the arithmetic of its text, two independent trains
  both at the target rate give the two gains together 16.9 per presynaptic spike, 0.73 of the depression of 23, since
  the nearest-neighbour pairing counts a target's last spike only. The depression, $2 \rho_0 \tau A_+$, is what the
  all-to-all rule would balance. At the target rate a synapse holds its weight only where its target and its
  inhibitory unit fire together more than chance.

## 3. The probe's step halved

Seed 22's first schedule from window 120 to the last, 840 windows, beside H-42's arm, the probe's step, and exploration
004's sign rule at a thirty-second within four times, its pick, read again here from that run's logs:

| Rule | Arm | Opposite pairs of 839 | Lag-1 | Lag-2 | Standard deviation over the target | Lowest of the last 128 correct | Reversals summed |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| The probe's step, 1 078 | assignment | 454 | −0.089 | −0.102 | 2.64 % | 120 | 34 |
| The sign rule, 343 to 1 258 | assignment | 426 | −0.024 | −0.055 | 2.59 % | 118 | 34 |
| Half the probe's, 539 | assignment | 420 | −0.007 | −0.041 | 2.55 % | 120 | 34 |
| The probe's step, 1 078 | mirrored | 432 | −0.057 | −0.088 | 2.66 % | 120 | 40 |
| The sign rule, 467 to 1 513 | mirrored | 418 | −0.020 | −0.068 | 2.65 % | 122 | 42 |
| Half the probe's, 539 | mirrored | 408 | +0.010 | −0.045 | 2.60 % | 122 | 40 |

The sign rule's step is its range over these windows.

- **The error is white under half the step.** Its lag-1 autocorrelation was within 0.01 of zero on both arms, where
  the sign rule's stopped at −0.02 and the probe's was −0.06 to −0.09. Half its pairs lay on opposite sides of the
  target.
- **The spread narrowed by 3.3 and 2.2 %**, more than the sign rule's 1.9 and 0.5 % on these arms. Over 840 windows a
  standard deviation's own sampling error is about 2.4 %; each arm is paired with the probe's on the same stimuli.
- **Learning was the probe's**: the same crossings within one block, the reversals summed 34 and 40.
- **The transient is longer.** The first window's regulation moves the gain half as far, so the gain took 29 and 35
  windows to come within 0.002 of where the first mapping's end left it, where the probe's took 14 and 20. Over the
  first 120 windows the count sat 1.92 and 2.06 % above the target, against 0.99 and 1.04 %, and the lag-1
  autocorrelation over the whole schedule was +0.22 against +0.04 and +0.06.
- **A plain integrator reads both steps.** For a loop that removes a fraction $L$ of each window's error from a
  white count, the error's lag-1 autocorrelation is $-L/2$ and its variance $2/(2 - L)$ times the count's own. With
  $L = \lambda$ that is −0.125 and −0.0625, and a spread 3.4 % narrower at an eighth. The probe's arms read −0.06 to
  −0.09 in the first schedule and −0.10 to −0.17 in the later ones; the halved arms −0.01 to +0.01, and 3.3 and 2.2 %.
  In the first schedule both read about 0.05 above the arithmetic, as a slow drift in the count would make them. The
  count's own spread is then about 2.47 % of the target, where a Poisson count of 2 330 would be 2.07 %: the floor
  exploration 004 named, and no step goes below it.

## What it suggests

**For M12's couplings, a bound over many schedules would read no under the default.** They rose through eight
schedules at about 0.05 a schedule, with no sign of levelling off, and learning got faster all the while. What carries
the rise is a growing number of synapses that go to the rail under their pair's mapping: up to 100 of a coupling's
800 by the eighth schedule. A mechanism for M12 would act on those synapses at the top of the width, not on the bulk.

**ADR-0037 gave, among its reasons not to adopt synaptic downscaling, that nothing in the tree showed weight
saturation under a rule whose depression exceeds its potentiation.** Under the default over long runs, excitatory
synapses of the couplings stand at exactly the rail at a block's end, up to 100 of a coupling's, and the inhibitory
synapses go to both of their bounds. A note decides nothing; whether that reason still stands, and whether the
difference is a finding under principle 1, is the maintainers' to read.

**The inhibitory sum's fall is not a drift.** It stops at about 0.55, once the rule has sorted the synapses onto the
excitatory readout units to zero or to the rail. It goes with the target: the read gives the inhibitory rule the
network's mean period, the rate's rule holds the mean there, and the excitatory majority sits a fifth below it, while
the task's volleys put nearly a quarter of all spikes into a tenth of the units and the inhibitory units fire a tenth
above it. The rule then keeps the synapses whose target fires just before the inhibitory unit and drains the rest.
Questions this suggests for a decision: a target read under the task rather than under the drive alone; a target per
unit or per group; and whether the rule's own balance, 0.73 at independent trains, is meant.

**The step: an eighth does with one constant what the sign rule did with three.** It makes the error white and narrows
the count by 2 to 3 %, about what the integrator's arithmetic gives, at a transient about twice as long; the rest of
the spread is the count's own. A measured round could ask $\lambda = 1/8$ against $1/4$ on three networks, from its own
protocol.

## What did not run

- **Seeds 23 and 24.** The first four schedules are exploration 004's long runs in every figure both read, and the
  last four continue its readings: the inhibitory sum's fall, slowing there, stopped here; the couplings' ratchet and
  the faster learning went on. The inhibitory synapses by group, the rule's replay and the step halved have no reading
  of exploration 004's to differ from. So by this exploration's rule nothing was extended. Each would take about 2 h
  for eight schedules and 20 minutes for the step halved, the arms in parallel.
- **More than eight schedules**, and so whether the couplings stop, or the synapses at the rail stop accumulating.
- **The rates under the drive alone**, at the first image or after: whether the excitatory readout units fire at the
  target without the task's volleys, as the derivation that set the target read them.
- **The excitatory wiring.** Whether the inhibitory synapses kept at the rail close a loop, their target exciting their
  inhibitory unit on the prior's ring, is not read: the replay reads the spikes' timing, not the synapses behind it.
- **The quiet runs.** The replay covers each schedule's lead-in and trials; at a schedule's end the 2 314 to 2 699 ticks
  run quiet moved the inhibitory synapses a little outside it, and the next schedule's replay began from the image.
- **The step halved over a long run, other values of $\lambda$**, or a step halved with the sign rule.
- **A re-read of the targets** (B, shelved), a target per group, or any change to the inhibitory rule.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `eight_schedules_under_the_default_short_on_seed_22_explore` | [40300630723](https://github.com/DescentVTT/VirtualCortex/actions/runs/40300630723) | `056c76b` | 241 |
| `eight_schedules_under_the_default_on_seed_22_from_the_assignment_explore` | [40300651755](https://github.com/DescentVTT/VirtualCortex/actions/runs/40300651755) | `056c76b` | 7162 |
| `eight_schedules_under_the_default_on_seed_22_from_the_mirrored_assignment_explore` | [40300651755](https://github.com/DescentVTT/VirtualCortex/actions/runs/40300651755) | `056c76b` | 7130 |
| `a_probe_step_halved_on_seed_22_from_the_assignment_explore` | [40300672820](https://github.com/DescentVTT/VirtualCortex/actions/runs/40300672820) | `056c76b` | 1101 |
| `a_probe_step_halved_on_seed_22_from_the_mirrored_assignment_explore` | [40300672820](https://github.com/DescentVTT/VirtualCortex/actions/runs/40300672820) | `056c76b` | 1092 |

Exploration 004's sign rule and probe arms were read again from run
[40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) on `bca69cb`.
