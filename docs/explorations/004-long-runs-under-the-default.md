---
status: exploratory
date: 2026-10-26
---

# 004: Long runs under the default

**Branch**: `explore/004-long-runs-under-the-default`
**Commit**: `bca69cb` for the short run, seed 22's long runs and the sign rule's grid on seed 22; `73f29b2` for the sign rule on seeds 23 and 24; `1fba56c` for the long runs on seeds 23 and 24. All three carry `529fbc9`, the sign rule under `src/`.
**Runs**: [40258363814](https://github.com/DescentVTT/VirtualCortex/actions/runs/40258363814) and [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) on `bca69cb`, [40262001686](https://github.com/DescentVTT/VirtualCortex/actions/runs/40262001686) on `73f29b2`, [40267416432](https://github.com/DescentVTT/VirtualCortex/actions/runs/40267416432) on `1fba56c`

M12 asks whether the couplings and the inhibitory sum stay bounded over many schedules, and names a step adapted
every window by its error's whiteness as a candidate, to be explored first (whitepaper Appendix C). The engine's
default reads its targets once and makes its step once, from a probe of its own loop
([H-42](../research/H-42.md), [ADR-0204](../adr/0204-a-step-the-engine-makes-from-its-own-loop-measured.md)). This
note asks four things of a long run under that default:

1. whether the network and the target drift over four of H-20's schedules run back to back;
2. whether a target read again at a schedule's end, as the default reads, differs from the one the engine holds;
3. whether the rate's error, its sign from window to window and its lag-1 autocorrelation under the probe's step,
   changes as the network drifts;
4. how a step moved every window by its error's sign compares with the probe's step.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host. The rule for this exploration ran seed 22 first and extended a question to seeds 23 and 24
only where its reading differed from what was expected; questions 1 and 2 did, and so, as first read, did question 4.

**In brief**
- **The network does not settle in four schedules.** On all three networks the inhibitory sum fell to 0.74 of the
  image's in the first schedule and to 0.53 to 0.56 by the fourth, each schedule's fall smaller than the last. All four
  couplings rose schedule over schedule, the lowest among them too. The highest passed H-25's clause 2 bound of 1.30 in
  the second or third schedule of every arm, and reached 1.38 to 1.46 in the fourth. The excitatory sum outside the
  four couplings stayed within three parts in ten thousand of the image's.
- **Learning got faster, not worse.** Every mapping of all 24 schedules was learned. Clauses 1, 3 and 4 held in every
  schedule. The three reversals of a schedule summed 30 to 40 blocks in the first and 18 to 28 in the fourth.
- **The target goes stale, the step does not.** Read again at the default's gain on a fork, the count a read would set
  as the target rose 6.7 to 10.1 % over four schedules against the first image's own re-read. Under the task the rule held the count
  at the old target by lowering the gain from about 1.746 to 1.69. The probe, run again on each fork, read a steepness
  of 15.0 to 16.1 and made a step within 7 % of the one held.
- **The error's whiteness did not track the drift.** Under the probe's step, after the first mapping, the error was
  mildly anticorrelated: lag-1 autocorrelation −0.06 to −0.19, and 51 to 57 % of window pairs on opposite sides of the
  target. Over the second to fourth schedules it stayed in that band while the inhibitory sum fell from 0.74 to 0.53.
  The first schedule's first mapping is a transient of its own, at +0.28 to +0.36, while the gain comes down from
  the derivation's 1.745 to the task's 1.71.
- **The sign rule found a smaller step and changed little.** At a factor of a thirty-second the step settled at 0.46
  to 0.78 of the probe's on average. It wandered slowly over a factor of three to six within a schedule, with no burst
  and no bound reached but on one arm. After the transient it moved the lag-1 autocorrelation toward zero, to −0.02 to
  −0.07, and the count's standard deviation down by 0.4 to 2.6 %, in all six arms. Learning was the probe's arm's
  within two blocks. At an eighth within sixteen times, over windows 120 to 720 of seed 22's arm from the assignment,
  the step ranged from 67 to 1 423 and each 120 windows' standard deviation was 2.45 to 2.63 % of the target, against
  the probe's 2.51 to 2.65 % over the same windows: most of the variance is a floor the step does not reach.

## What ran

**The network and the arm.** Every run starts from H-42's network of its seed at 1 024 units, the image an engine born
under `Config::reads_its_targets()` settles. Each test first runs that derivation again and holds it to H-42's pinned
tables (`probe_made`); every run did. The arm is H-43's twin on time: H-25's arm on H-20's schedule from that image,
under the answer's reward and the drawn delivery, with the engine's critic and its window, two addresses kept and
every reward at age zero. That is H-42's arm bit for bit. Each long run's first schedule was held to it: its 120
blocks, its trace and every reward's value. Both of H-25's arms ran, from the assignment and from the mirrored one.

**Four schedules, image to image.** At a schedule's end the engine runs quiet without the drive until it is quiescent,
2 144 to 2 699 ticks, and its image is written (`Image::encode`). The next schedule's engine is decoded from that image,
so nothing is derived again and every record carries over. What an image does not hold starts afresh, as it did
before the first schedule: the spike train, the critic's counts, and the address, which is every unit. Each schedule
begins with the harness's lead-in of one window under the drive, and its trials are numbered from zero again, so its
stimuli are the first schedule's. Its first mapping is the arm's first again. A schedule's three flips end it under the
other mapping, so from the second schedule on its first mapping is a reversal.

**What was read.**
- Per block: the four couplings, the inhibitory sum and the excitatory sum outside the four couplings, each over the
  first image's, and the gain at the block's end.
- Per window: its spikes, and the gain and the step its rule left.
- Per schedule: each mapping's last 128 correct and its crossing, the blocks to 40 of 64; and H-25's four clauses
  against the first image's couplings.
- Per schedule, of the rate's error $e_t = c_t - c^*$, a window's count less the target: the pairs of consecutive
  windows on opposite sides of the target, as ADR-0181 counts them; its lag-1 and lag-2 autocorrelation; and its
  standard deviation over the target.

A watch beside each run counted every window from the spike train. At every trial's end its count of the open window
was the record's own, in all 44 schedules of the runs.

**The re-read on a fork.** Every image, the first and each schedule's, was forked twice. In the fork's image bytes a
read was put under way as the default's engine is born with one:
- the rate's target cleared, 128 windows left to read, the inhibitory target's flag and the probe's set;
- the gain the read holds written 1.75, the default's, in one fork, and left as the engine held it in the other.

The fork then ran under the drive alone, with no trial: the window open at the load, then 143 windows, the read's last
127 and the probe's sixteen. Each fork's target and step were held to the hand rules of H-41 and H-42 over its windows'
counts; all 66 held. Two differences from the read at birth matter:
- **The modulation baseline is the image's, zero.** No excitatory synapse moves without a reward, so a fork reads the
  network as it stands. At birth the read ran at 0.5, on a network still settling. On the first image, which the
  engine was born to, a fork at 1.75 read 4.9 to 6.4 % above the target the engine held.
- **A fork at the engine's own gain reads, under the drive alone, the gain the task held.** Under the trials the rule
  lowered the gain to about 1.70 to hold the count. Under the drive alone that gain gives 24 to 34 % fewer spikes than
  the target. Those forks measure the task's share of the count, not a drift, and are not read further.

So a fork at 1.75 is read against the first image's fork at 1.75, its control, and not against the target the engine
was born with.

**The sign rule, under `src/` on this branch alone.** Commit `529fbc9` adds `SignRule` and `Executor::set_sign_rule`:
- at each window's close under the rate's rule, after the rule moved the gain with the step it had, the step is
  multiplied by up where the window and the one before lay on the same side of the target, and by down where they did
  not;
- a window at the target counts as not above it, as the opposite pairs count it;
- the product is rounded to nearest and held within the rule's bounds.

Up is $1 + f$ and down $1/(1 + f)$, so the step stands still on average where half the pairs agree. It starts at the
probe's step, 1 078 to 1 091 in Q0.16. Unset, every window is ADR-0180's rule bit for bit; the long runs leave it
unset and were held to H-42's arm. Each sign arm is the long run's first schedule from the same image with the rule
set. The grid, on seed 22:
- $f$ a thirty-second or an eighth, the bounds four or sixteen times the probe's step either way;
- and $f = 0$, which moves nothing; that arm was held to H-42's arm bit for bit, and so were its twins on seeds 23 and
  24.

## 1. Drift over four schedules

At each schedule's end, over the first image's:

| Network | Arm | Inhibitory sum | Highest coupling | Lowest coupling | Gain |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22 | assignment | 0.743, 0.605, 0.569, 0.556 | 1.239, 1.350, 1.428, 1.461 | 0.903, 0.930, 0.985, 1.036 | 1.7062, 1.7025, 1.7002, 1.6947 |
| Seed 22 | mirrored | 0.744, 0.602, 0.564, 0.553 | 1.248, 1.298, 1.374, 1.431 | 0.889, 0.956, 0.970, 1.018 | 1.7073, 1.7036, 1.6996, 1.6941 |
| Seed 23 | assignment | 0.739, 0.589, 0.541, 0.529 | 1.206, 1.276, 1.325, 1.410 | 0.894, 0.958, 0.982, 1.000 | 1.7060, 1.7022, 1.6993, 1.6955 |
| Seed 23 | mirrored | 0.739, 0.587, 0.546, 0.532 | 1.199, 1.239, 1.310, 1.393 | 0.886, 0.909, 0.950, 0.996 | 1.7088, 1.7057, 1.7018, 1.6956 |
| Seed 24 | assignment | 0.743, 0.594, 0.549, 0.533 | 1.254, 1.313, 1.341, 1.375 | 0.826, 0.831, 0.858, 0.873 | 1.7076, 1.7023, 1.7006, 1.6966 |
| Seed 24 | mirrored | 0.739, 0.588, 0.543, 0.527 | 1.233, 1.258, 1.343, 1.403 | 0.829, 0.910, 0.953, 1.045 | 1.7081, 1.7025, 1.6983, 1.6884 |

The highest and the lowest coupling are over every block of the schedule and the four pairs. The gain is the one the
last block's end held; the first image's was 1.745 to 1.747.

- **The inhibitory sum fell by less each schedule**: by 0.26 of the image's in the first, 0.14 to 0.15 in the second,
  0.04 to 0.05 in the third and 0.01 to 0.02 in the fourth, on every network. Whether it stops is not read.
- **The couplings ratcheted up.** Within a schedule each pair's coupling rises under the mapping that rewards it and
  falls under the other. Each falls back less than it rose, so the whole band moves up, by 0.025 to 0.11 a schedule at
  its top. Its top shows no plateau in four schedules.
- **Clause 2 failed from the second schedule on seed 22 from the assignment and on seed 24 from the assignment, and
  from the third on the other four arms.** In the fourth schedule the first block past 1.30 was block 8 to block 32,
  counted from zero. The pair first past it was the first stimulus's onto the first readout in four arms, and the
  second stimulus's in two.
- **The excitatory sum outside the four couplings stayed within three parts in ten thousand of the image's** at every
  block's end. The excitatory sum as a whole rose 2.0 to 3.4 %, nearly all of it in the four couplings.

**Learning, schedule by schedule:**

| Network | Arm | Lowest of the mappings' last 128 correct | Three reversals summed, blocks | The schedule's first mapping, blocks to 40 of 64 |
| :--- | :--- | :--- | :--- | :--- |
| Seed 22 | assignment | 120, 121, 124, 126 | 34, 27, 26, 21 | 4, 11, 11, 10 |
| Seed 22 | mirrored | 120, 122, 123, 123 | 40, 33, 31, 22 | 2, 12, 10, 9 |
| Seed 23 | assignment | 124, 126, 126, 123 | 30, 25, 23, 20 | 4, 9, 8, 6 |
| Seed 23 | mirrored | 121, 122, 123, 124 | 37, 30, 28, 26 | 1, 11, 10, 10 |
| Seed 24 | assignment | 123, 126, 125, 126 | 37, 30, 25, 28 | 1, 12, 9, 11 |
| Seed 24 | mirrored | 122, 125, 123, 125 | 37, 29, 24, 18 | 9, 13, 10, 7 |

- Every mapping of every schedule was learned, at 120 to 128 of its last 128, against clause 1's 80.
- From the second schedule on, a schedule's first mapping follows the last schedule's last, the other mapping, so it
  is a reversal too. It crossed in 6 to 13 blocks, as the other reversals did.
- A reading: the couplings carry each mapping's last training into the next presentation of it, so the network
  relearns faster each time, and the ratchet in the couplings is the same carry.

## 2. Is the target stale?

The count each fork at 1.75 read, at the first image and after each schedule, and its rise over the first image's:

| Network | Arm | Held target | First image | After schedules 1 to 4 | Rise over the first image |
| :--- | :--- | ---: | ---: | :--- | :--- |
| Seed 22 | assignment | 2 330 | 2 444 | 2 516, 2 618, 2 657, 2 676 | 2.9, 7.1, 8.7, 9.5 % |
| Seed 22 | mirrored | 2 330 | 2 444 | 2 494, 2 602, 2 634, 2 660 | 2.0, 6.5, 7.8, 8.8 % |
| Seed 23 | assignment | 2 314 | 2 437 | 2 519, 2 587, 2 648, 2 640 | 3.4, 6.2, 8.7, 8.3 % |
| Seed 23 | mirrored | 2 314 | 2 437 | 2 497, 2 552, 2 629, 2 648 | 2.5, 4.7, 7.9, 8.7 % |
| Seed 24 | assignment | 2 331 | 2 480 | 2 535, 2 616, 2 656, 2 647 | 2.2, 5.5, 7.1, 6.7 % |
| Seed 24 | mirrored | 2 331 | 2 480 | 2 553, 2 602, 2 680, 2 730 | 2.9, 4.9, 8.1, 10.1 % |

- **The network's own rate at the default's gain rose by 6.7 to 10.1 % in four schedules**, the rise slowing as the
  inhibitory sum's fall slowed. A read after the fourth schedule sets a period of 49 164 to 50 840 ticks where the
  engine holds 57 579 to 58 002.
- **The engine held the old target all along.** Over every schedule's windows from the 120th, the count sat 0.00 to
  0.03 % above it. The rule did so by moving the gain, 1.745 to 1.747 at the first image and 1.688 to 1.697 at the
  fourth schedule's end.
- **The probe read the same loop.** On each fork at 1.75 it read a steepness of 15.0 to 16.1, against 14.8 to 15.3 on
  the first image's forks and 15.0 to 15.2 at birth. The step it made was 1 019 to 1 092, against the 1 078 to 1 091
  the engine held, within 7 % and in most forks within 4 %.

## 3. The error's whiteness under the probe's step

From each schedule's 120th window to its last, 840 windows:

| Network | Arm | Pairs on opposite sides, schedules 1 to 4 | Lag-1 autocorrelation | Standard deviation over the target |
| :--- | :--- | :--- | :--- | :--- |
| Seed 22 | assignment | 54.1, 55.2, 56.6, 55.2 % | −0.089, −0.127, −0.163, −0.137 | 2.64, 2.70, 2.85, 2.80 % |
| Seed 22 | mirrored | 51.5, 54.7, 57.3, 55.5 % | −0.057, −0.165, −0.164, −0.151 | 2.66, 2.67, 2.90, 2.80 % |
| Seed 23 | assignment | 56.3, 55.5, 53.2, 57.2 % | −0.130, −0.153, −0.149, −0.151 | 2.74, 2.75, 2.77, 2.98 % |
| Seed 23 | mirrored | 52.8, 55.7, 55.4, 54.4 % | −0.098, −0.154, −0.187, −0.121 | 2.74, 2.75, 2.84, 2.79 % |
| Seed 24 | assignment | 52.1, 52.8, 54.4, 56.1 % | −0.113, −0.118, −0.178, −0.173 | 2.81, 2.71, 2.89, 2.85 % |
| Seed 24 | mirrored | 50.9, 54.2, 55.5, 56.5 % | −0.105, −0.156, −0.153, −0.160 | 2.84, 2.71, 2.88, 2.80 % |

- **The first schedule begins with a transient.** Over its first mapping, 192 windows, the lag-1 autocorrelation was
  +0.28 to +0.36, the standard deviation 3.5 to 3.7 % and the count 0.55 to 0.63 % above the target, in every arm.
  That is the gain coming down from 1.745 to about 1.71 as the task's trials add their spikes, most of it in the first
  120 windows, where the tables start. Over the whole first schedule the transient lifts the lag-1 autocorrelation to
  +0.01 to +0.06, which is what a reading over the whole run gives.
- **After it, the error is mildly anticorrelated.** The probe's step overcorrects a little: more than half the pairs
  alternate, and the lag-1 autocorrelation is −0.06 to −0.19.
- **It moved once, not with the drift.** From the first schedule to the second it became more negative on all six arms,
  on seed 24 from the assignment barely. Over the second to the fourth it stayed in −0.12 to −0.19, while the
  inhibitory sum fell from about 0.74 to 0.53 and the target's re-read rose from 2 to 3 % to 7 to 10 %. A schedule's
  lag-2 autocorrelation was −0.025 to −0.127 throughout.

## 4. The sign rule against the probe's step

**The grid on seed 22**, the first schedule of each arm:

| Rule | Arm | Opposite pairs of 959 | Lag-1, whole; from window 120 | Standard deviation from window 120 | Step, least to greatest | Step's geometric mean | Windows at a bound | Reversals summed |
| :--- | :--- | ---: | :--- | ---: | :--- | ---: | ---: | ---: |
| The probe's step | assignment | 509 | +0.042; −0.089 | 2.64 % | 1 078 | 1 078 | — | 34 |
| $f$ = 1/32, within 4 or 16 times | assignment | 485 | +0.087; −0.024 | 2.59 % | 343 to 1 560 | 603 | 0 | 34 |
| $f$ = 1/8, within 4 times | assignment | 491 | +0.047; −0.063 | 2.61 % | 269 to 4 312 | 727 | 44 | 35 |
| $f$ = 1/8, within 16 times | assignment | 485 | +0.094; +0.002 | 2.60 % | 67 to 4 434 | 423 | 8 | 34 |
| The probe's step | mirrored | 485 | +0.064; −0.057 | 2.66 % | 1 078 | 1 078 | — | 40 |
| $f$ = 1/32, within 4 or 16 times | mirrored | 475 | +0.088; −0.020 | 2.65 % | 467 to 1 560 | 845 | 0 | 42 |
| $f$ = 1/8, within 4 times | mirrored | 477 | +0.030; −0.087 | 2.72 % | 269 to 4 312 | 990 | 39 | 42 |
| $f$ = 1/8, within 16 times | mirrored | 477 | +0.073; −0.034 | 2.69 % | 92 to 3 615 | 739 | 0 | 42 |

- At a thirty-second the step never reached four times the probe's either way, so the two bounds ran the same arm, its
  every reading equal.
- At an eighth it reached both bounds of four, and within sixteen it ranged over a factor of 66 on seed 22 from the
  assignment. That is a random walk the bounds alone stop. Its spread was no narrower than at a thirty-second, and its
  lag-1 autocorrelation was nearer zero on one arm and further from it on three.
- Every arm learned all four mappings at 118 to 127 of their last 128, and held all four of H-25's clauses.

**The pick, a thirty-second within four times, on all three networks**, beside the probe's step, H-42's arm:

| Network | Arm | Opposite pairs, probe; sign | Lag-1 from window 120, probe; sign | Standard deviation from window 120, probe; sign | Sign's step, least to greatest; over the probe's | Reversals summed, probe; sign |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Seed 22 | assignment | 509; 485 | −0.089; −0.024 | 2.64; 2.59 % | 343 to 1 560; 0.56 | 34; 34 |
| Seed 22 | mirrored | 485; 475 | −0.057; −0.020 | 2.66; 2.65 % | 467 to 1 560; 0.78 | 40; 42 |
| Seed 23 | assignment | 527; 502 | −0.130; −0.054 | 2.74; 2.67 % | 271 to 1 722; 0.46 | 30; 31 |
| Seed 23 | mirrored | 493; 483 | −0.098; −0.040 | 2.74; 2.70 % | 307 to 1 522; 0.63 | 37; 36 |
| Seed 24 | assignment | 497; 481 | −0.113; −0.071 | 2.81; 2.76 % | 329 to 1 530; 0.61 | 37; 37 |
| Seed 24 | mirrored | 491; 477 | −0.105; −0.069 | 2.84; 2.77 % | 273 to 1 530; 0.61 | 37; 37 |

"Over the probe's" is the sign rule's step's geometric mean over its windows, over the probe's step.

- **The step's course was the same on every arm.** The step rose in the first mapping's transient, where the error
  keeps its sign for many windows, to 1 522 to 1 722. It fell through the second and third mappings, to 271 to 467 at
  its least, and rose again in the fourth on every arm, ending at 0.46 to 1.32 of the probe's. Its logarithm's
  autocorrelation over 50 windows was 0.63 to 0.85: a slow drift, not a step that jumps from window to window.
- **It held half the pairs on each side, as it is built to.** Opposite pairs fell to 475 to 502 of 959.
- **Read after the transient it moved the lag-1 autocorrelation toward zero, and the spread down a little**, in all six
  arms: by 0.04 to 0.08 in the autocorrelation, and by 0.4 to 2.6 % in the standard deviation.
- **It reached a bound on one arm**, seed 23 from the assignment, where the step sat at a quarter of the probe's for 43
  windows.
- **Read over the whole schedule, the lag-1 autocorrelation rose instead**, from +0.01 to +0.06 to +0.05 to +0.09. That
  reading on seed 22, the transient in it, is why this question extended to seeds 23 and 24. Read from the 120th window
  it moved as expected, toward zero, though not to it.
- Over 840 windows a standard deviation's own sampling error is about 2.4 % by arithmetic, as large as the sign rule's
  0.4 to 2.6 %. Its sign agreed on all six arms, each paired with the probe's arm on the same network and stimuli.

## What it suggests

**For M12's first question, the default does not hold the couplings under H-25's bound over four schedules.** They
rose every schedule, and the bound of 1.30 is one schedule's. A hypothesis that the couplings stay under a bound over
many schedules would read no under the default as it stands, at that bound. Whether they plateau, and where, needs
runs longer than four schedules. The inhibitory sum's fall slows by itself: it lost 0.01 to 0.02 of the image's in the
fourth schedule.

**What drifts is narrow.** Nearly every excitatory change is in the four couplings the address reaches, and the
inhibitory sum falls on every network alike. A mechanism for M12, such as UPG-02's sliding threshold, would act on
those two.

**B, a re-read by evidence, has a drift to answer.** The default's read would set a target 7 to 10 % higher after four
schedules. The engine carries a signal of it: after the first schedule, in which its gain fell about 2 % as the task's
trials added spikes, the gain sank a further 0.6 to 1.2 % over three schedules. That signal is small beside the task's
own share, so a trigger on it would have to tell the two apart. The probe's step stays good, so the step is not what
goes stale. What a re-read would do to learning is not read: the engine learned better each schedule while holding
the stale target.

**The whiteness of the error is not a drift signal.** Over the second to the fourth schedule it stayed in one band while
the target went stale. It names the loop's overcorrection: the probe's $\lambda = 1/4$ leaves a lag-1 autocorrelation
near −0.15.

**The sign rule tunes the step, and the step matters little.** At a thirty-second it settles below the probe's step by
a factor of 0.46 to 0.78, without bursting, and moves the error toward white. It narrows the count by 0.4 to 2.6 %, and
learning is the same. A smaller $\lambda$ for the probe might do as much with one constant where the rule needs three.
As a candidate for B the sign rule addresses the part of the controller that did not drift.

## What did not run

- **Runs longer than four schedules**, and so whether the couplings or the inhibitory sum stop.
- **A re-read under the birth's conditions**, at a modulation baseline of 0.5, or under the task's trials with the gain
  held. The forks read the network frozen but for its inhibitory synapses.
- **The sign rule over a long run**, or started after the first mapping's transient; and other factors, bounds, or a
  rule whose standing point is not half the pairs.
- **The sign rule's grid on seeds 23 and 24**: there only the pick ran.
- **Other stimulus orders.** The engine is deterministic, so a repeat on the same commit gives the same run. A
  reading's spread is read across three networks and two arms, and not across the orders of a schedule's stimuli.
- **What drives the inhibitory sum down** while the rate is held at its target. The inhibitory rule runs against the
  read's period throughout; nothing here reads it unit by unit.
- **A smaller $\lambda$ for the probe**, the alternative the last suggestion names.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `long_runs_under_the_default_short_on_seed_22_explore` | [40258363814](https://github.com/DescentVTT/VirtualCortex/actions/runs/40258363814) | `bca69cb` | 1029 |
| `long_runs_under_the_default_on_seed_22_from_the_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 5097 |
| `long_runs_under_the_default_on_seed_22_from_the_mirrored_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 5146 |
| `a_step_by_its_error_s_sign_at_one_on_seed_22_from_the_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 1099 |
| `a_step_by_its_error_s_sign_a_thirty_second_within_four_on_seed_22_from_the_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 1072 |
| `a_step_by_its_error_s_sign_a_thirty_second_within_four_on_seed_22_from_the_mirrored_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 1106 |
| `a_step_by_its_error_s_sign_a_thirty_second_within_sixteen_on_seed_22_from_the_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 1068 |
| `a_step_by_its_error_s_sign_a_thirty_second_within_sixteen_on_seed_22_from_the_mirrored_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 1096 |
| `a_step_by_its_error_s_sign_an_eighth_within_four_on_seed_22_from_the_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 1103 |
| `a_step_by_its_error_s_sign_an_eighth_within_four_on_seed_22_from_the_mirrored_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 1147 |
| `a_step_by_its_error_s_sign_an_eighth_within_sixteen_on_seed_22_from_the_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 1107 |
| `a_step_by_its_error_s_sign_an_eighth_within_sixteen_on_seed_22_from_the_mirrored_assignment_explore` | [40259976706](https://github.com/DescentVTT/VirtualCortex/actions/runs/40259976706) | `bca69cb` | 964 |
| `a_step_by_its_error_s_sign_at_one_on_seed_23_from_the_assignment_explore` | [40262001686](https://github.com/DescentVTT/VirtualCortex/actions/runs/40262001686) | `73f29b2` | 1110 |
| `a_step_by_its_error_s_sign_at_one_on_seed_23_from_the_mirrored_assignment_explore` | [40262001686](https://github.com/DescentVTT/VirtualCortex/actions/runs/40262001686) | `73f29b2` | 1123 |
| `a_step_by_its_error_s_sign_at_one_on_seed_24_from_the_assignment_explore` | [40262001686](https://github.com/DescentVTT/VirtualCortex/actions/runs/40262001686) | `73f29b2` | 1095 |
| `a_step_by_its_error_s_sign_at_one_on_seed_24_from_the_mirrored_assignment_explore` | [40262001686](https://github.com/DescentVTT/VirtualCortex/actions/runs/40262001686) | `73f29b2` | 1046 |
| `a_step_by_its_error_s_sign_a_thirty_second_within_four_on_seed_23_from_the_assignment_explore` | [40262001686](https://github.com/DescentVTT/VirtualCortex/actions/runs/40262001686) | `73f29b2` | 1062 |
| `a_step_by_its_error_s_sign_a_thirty_second_within_four_on_seed_23_from_the_mirrored_assignment_explore` | [40262001686](https://github.com/DescentVTT/VirtualCortex/actions/runs/40262001686) | `73f29b2` | 1108 |
| `a_step_by_its_error_s_sign_a_thirty_second_within_four_on_seed_24_from_the_assignment_explore` | [40262001686](https://github.com/DescentVTT/VirtualCortex/actions/runs/40262001686) | `73f29b2` | 1107 |
| `a_step_by_its_error_s_sign_a_thirty_second_within_four_on_seed_24_from_the_mirrored_assignment_explore` | [40262001686](https://github.com/DescentVTT/VirtualCortex/actions/runs/40262001686) | `73f29b2` | 1070 |
| `long_runs_under_the_default_on_seed_23_from_the_assignment_explore` | [40267416432](https://github.com/DescentVTT/VirtualCortex/actions/runs/40267416432) | `1fba56c` | 5062 |
| `long_runs_under_the_default_on_seed_23_from_the_mirrored_assignment_explore` | [40267416432](https://github.com/DescentVTT/VirtualCortex/actions/runs/40267416432) | `1fba56c` | 5061 |
| `long_runs_under_the_default_on_seed_24_from_the_assignment_explore` | [40267416432](https://github.com/DescentVTT/VirtualCortex/actions/runs/40267416432) | `1fba56c` | 4944 |
| `long_runs_under_the_default_on_seed_24_from_the_mirrored_assignment_explore` | [40267416432](https://github.com/DescentVTT/VirtualCortex/actions/runs/40267416432) | `1fba56c` | 5038 |
