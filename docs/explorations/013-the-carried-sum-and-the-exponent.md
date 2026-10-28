---
status: exploratory
date: 2026-10-28
---

# 013: The carried sum and the exponent

**Branch**: `explore/013-the-carried-sum-and-the-exponent`
**Commit**: `795b1b8` for the short run, schedules 17 to 24 and the exponents' first halves; `8d0c0e2` for schedules 25
to 32 and the exponents' second halves, which adds the six images the first runs wrote and their CRC-64s and nothing
else that runs. Both carry `9bd1cae`, the merge of exploration 012's tag, and `707d255`, this exploration's exponent
under `src/`. Unset, and at an exponent of one, each is the rule bit for bit.
**Runs**: [40671679938](https://github.com/Menteion/Menteion/actions/runs/40671679938) (the short run, schedules 17 to
24 and the exponents' first halves) and [40687049717](https://github.com/Menteion/Menteion/actions/runs/40687049717)
(schedules 25 to 32 and the exponents' second halves), both at `scope=arms`

Exploration 012 held each stimulus set's couplings to the two readout sets, summed, at their value in H-42's image, with
each scaled weight's fraction carried to its next round. Over sixteen schedules the couplings stayed within 0.82 to 1.20
of the image's with every mapping learned and no coupling split; but three reversals took 16 to 32 blocks a schedule from
the third, where exploration 011's zero sum rounded down took 11 to 13, and the tails at the rail and near zero were
still growing ([012](012-the-zero-sums-rounding.md)). 012 read, without measuring it, a few strong synapses and many near
zero as an additive rule's two-group regime, which [ADR-0055](../adr/0055-a-weight-that-settles.md) set against its own
rule's one group, a depression in proportion to the magnitude ($\mu = 1$), recording the weaker competition among a
unit's inputs as its rule's failure mode and the exponent between zero and one as the lever. This note asks, on seed 22:

1. where the carried zero sum's tails and its reversals stop: schedules 17 to 32, from the sixteenth images 012's
   second halves wrote, in two runs of eight;
2. whether the depression's exponent between zero and one makes a fast-learning two-group coupling by the rule itself
   rather than by a rounding's drain, and whether the couplings stay bounded: under the carried zero sum, the excitatory
   depression in proportion to $(M / M_{\text{ref}})^{\mu}$ with $\mu = 1/2$ and $\mu = 1/4$, in integers, from H-42's
   image over sixteen schedules in two halves, from both arms, reading also the whole excitatory arena's sum, since
   [ADR-0053](../adr/0053-the-waking-day-and-the-target-period.md) read the additive rule ($\mu = 0$) draining it;
3. against 012's carried zero sum ($\mu = 1$) and 011's rounded down, schedule by schedule, in the distribution, the
   reversals and the couplings.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host.

**In brief**
- **At $\mu = 1/4$ learning was as fast as the default's, with the couplings held.** Its three reversals summed 32, 31,
  26, 23, 19, 17, 16, 15 blocks over the first eight schedules from the assignment, the default's 34, 27, 26, 21, 20, 17,
  17, 15, and 12 to 16 over schedules 9 to 16 from both arms, the default's 12 to 16. The highest coupling stood at 1.13
  to 1.19 of the image's. At $\mu = 1/2$ the reversals took 13 to 18 over schedules 9 to 16; at $\mu = 1$, carried, 16 to
  24; rounded down, 11 to 13.
- **The rule split the couplings itself, and faster as $\mu$ fell.** By the sixteenth image, of the four couplings'
  3 188 synapses, 1 665 to 1 667 stood in the bottom eighth of the width at $\mu = 1/4$ and 257 to 268 at its top
  sixteenth, the middle half emptying to 464 to 474; H-42's image had 106, 0 and 1 260. At $\mu = 1/2$, 1 397 to 1 407;
  at $\mu = 1$, 1 063 to 1 138, and 1 402 to 1 578 by the 32nd image; rounded down, 2 291 to 2 301.
- **The reversals followed the split, whatever made it.** Over 152 schedules of the four forms and both arms, a schedule
  that started with 800 to 999 synapses in the bottom eighth summed its reversals at 16 to 24 blocks, whether rounded
  down in its second schedule or at $\mu = 1$ in its fifteenth; one that started with 1 600 or more, at 11 to 13.
- **The carried zero sum had not stopped by the 32nd schedule.** Its reversals fell from 15 to 18 blocks over schedules
  17 to 24 to 13 to 17 over 25 to 32, about a block in eight schedules; its bottom eighth and top sixteenth were still
  filling. From the mirrored assignment its highest coupling rose to 1.228 in the 31st schedule.
- **No arena drained.** The whole excitatory arena stood at 0.9994 to 1.0005 of the image's at every block's end under
  both exponents; outside the four couplings it moved by at most 4.1 rails of its 5 743, as under 011's and 012's zero
  sums.
- **Every mapping of all 96 schedules here was learned**, at 120 to 128 of its last 128, and H-25's four clauses held in
  every one.

## What ran

**The network, the arm and the path** are exploration 012's. Every test starts from H-42's network of seed 22 at 1 024
units, the image an engine born under `Config::reads_its_targets()` settles, and first runs that derivation again and
holds it to H-42's pinned tables (`probe_made`); all thirteen held. The arm is H-43's twin on time, H-42's arm bit for
bit, from the assignment and from the mirrored assignment. A schedule ends with the engine run quiet without the drive
until it is quiescent; its image is written, and the next schedule's engine is decoded from it.

**The harness** is exploration 012's as its tag holds it (`96ca056`), merged onto `main` at `8ddcaaa` (`9bd1cae`).
Every reading 012 took is taken in every schedule. The whole excitatory arena's sum was already in each block's reading
and after each schedule's quiet; this note reads it, and the arena less the four couplings beside it.

**The zero sum** is exploration 012's carried form: 011's zero sum both ways, each stimulus set's couplings to the two
readout sets, summed, held at their value in H-42's image, each scaled weight rounded down with its fraction carried to
its next round from a half. Every exponent here runs under it.

**The exponent, under `src/` on this branch alone.** Commit `707d255` adds `DepressionExponent` and
`Executor::set_depression_exponent`. ADR-0055's excitatory depression is
$\mathrm{round}(A_- (1 - 2^{-11})^{t - q} M / 2^{13})$, $M$ the slot's magnitude before the pairing. Under an exponent
$\mu$ it reads, in place of $M$, the shaped magnitude $M_\mu = 2^{13} (M / 2^{13})^{\mu}$, in integers and rounded down:
- **$\mu = 1$** (`One`, the default): $M$ itself, so the depression is ADR-0055's bit for bit;
- **$\mu = 1/2$** (`Half`): $\lfloor \sqrt{2^{13} M} \rfloor$, by `u64::isqrt`;
- **$\mu = 1/4$** (`Quarter`): $\lfloor \sqrt{\lfloor \sqrt{2^{39} M} \rfloor} \rfloor$, two integer square roots,
  which is $\lfloor (2^{39} M)^{1/4} \rfloor$ exactly.

**The error.** Each shaped magnitude stands below its exact value by less than one LSB of the magnitude, and is exact at
zero and at the reference, where every exponent gives ADR-0055's $A_-$. So the depression, rounded to nearest as before,
is short of the exact one by less than $A_- / 2^{13}$, 0.042 LSB, before its rounding. A unit test holds every magnitude
from zero to the rail against its root by the root's powers, and monotone.

| Magnitude $M$ | $\mu = 1$ | $\mu = 1/2$ | $\mu = 1/4$ |
| ---: | ---: | ---: | ---: |
| 1 | 0 | 4 | 36 |
| 12 | 1 | 13 | 67 |
| 100 | 4 | 38 | 114 |
| 1 000 | 42 | 120 | 203 |
| 4 096 | 172 | 243 | 289 |
| 8 192, the reference | 344 | 344 | 344 |
| 16 384 | 688 | 486 | 409 |
| 32 767, the rail | 1 376 | 688 | 486 |

The depression at the window's full amount, in LSB. Below the reference a smaller exponent depresses more, above it
less. Under $\mu = 1$ nothing below a magnitude of twelve is depressed; under $\mu = 1/2$ and $1/4$ every magnitude above
zero is, so a weight can reach zero from above. Under stationary pairing a magnitude settles at
$M^* = 2^{13} (A_+ f_+ / (A_- f_-))^{1/\mu}$: a smaller exponent carries a synapse whose pairings favour the potentiation
further above the reference, and one whose pairings favour the depression further below it.

- **The potentiation, the inhibitory rule, the consolidation and the zero sum do not move.**
  `SynapseBlock::step_stdp_shaped` and `step_stdp_all_shaped` carry the exponent; the rule's `step_stdp_bounded` and
  `step_stdp_all_bounded` call them with `One`.
- **It is in no image**, so the test sets it on every schedule's engine, as it sets the zero sum.
- **Held.** The short run's schedule unset held H-42's arm as far as it ran and wrote exploration 011's image, CRC-64
  `0x841f164eb6fdefb2`; its schedule under the zero sum carried at $\mu = 1$ wrote exploration 012's short image,
  `0xcd817b511c5403d5`. Two unit tests hold the shaped magnitudes and a pairing under each exponent, and the word the
  workers read.

**The long runs**, as exploration 012 carried its own:
- **Past sixteen schedules.** Exploration 012's second halves printed their sixteenth images whole. The branch keeps
  them in `runtime/menteion-runtime/tests/explore-013/`, decoded from run 40653304865's logs and held to the CRC-64s
  printed beside them, `0xfbcd9607cdbf4163` from the assignment and `0xf12c021f1ea17f31` from the mirrored assignment.
  Schedules 17 to 24 ran from them, and schedules 25 to 32 from the images those printed, held likewise.
- **The two exponents**, each from H-42's image over schedules 1 to 8, then schedules 9 to 16 from its eighth image.

## 1. The carried zero sum, schedules 17 to 32

Each cell lists eight schedules in order:

| Arm | Schedules | Highest coupling | Lowest coupling | At the rail, most | Lowest of the mappings' last 128 correct | Three reversals summed, blocks |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| assignment | 17–24 | 1.163, 1.165, 1.160, 1.176, 1.177, 1.178, 1.175, 1.187 | 0.854, 0.852, 0.845, 0.842, 0.841, 0.841, 0.843, 0.833 | 73, 59, 69, 78, 88, 69, 62, 71 | 126, 126, 127, 125, 124, 127, 127, 128 | 17, 18, 16, 16, 16, 15, 15, 15 |
| assignment | 25–32 | 1.202, 1.201, 1.202, 1.201, 1.188, 1.192, 1.188, 1.192 | 0.820, 0.820, 0.819, 0.820, 0.831, 0.828, 0.831, 0.828 | 74, 68, 82, 93, 79, 84, 98, 92 | 126, 127, 128, 127, 126, 127, 126, 128 | 14, 15, 15, 16, 15, 16, 13, 13 |
| mirrored | 17–24 | 1.186, 1.182, 1.183, 1.187, 1.191, 1.193, 1.195, 1.183 | 0.834, 0.837, 0.836, 0.833, 0.829, 0.827, 0.826, 0.836 | 55, 67, 68, 86, 78, 81, 71, 75 | 124, 127, 127, 126, 125, 126, 126, 125 | 16, 18, 17, 15, 16, 16, 18, 18 |
| mirrored | 25–32 | 1.182, 1.180, 1.183, 1.186, 1.201, 1.214, 1.228, 1.218 | 0.837, 0.839, 0.836, 0.833, 0.820, 0.808, 0.796, 0.805 | 52, 72, 84, 71, 75, 64, 69, 69 | 125, 128, 125, 126, 127, 126, 127, 127 | 17, 15, 15, 15, 15, 14, 15, 15 |

The couplings are over every block's end of the schedule and the four pairs, over the first image's; a mapping is
learned at 80 or more of its last 128. 012's schedules 9 to 16 summed their reversals at 18 to 24 blocks from the
assignment and 16 to 24 from the mirrored assignment.

- **The reversals still fell.** Over schedules 17 to 24 they summed 15 to 18 blocks from both arms, over 25 to 32 13 to
  16 from the assignment and 14 to 17 from the mirrored assignment, the last two schedules 13 and 13 from the
  assignment and 15 and 15 from the mirrored assignment. The default's
  took 12 to 16 over schedules 9 to 16 and 011's rounded down 11 to 13.
- **The tails still grew, slowly.** The four couplings' bottom eighth held 1 063 synapses at the sixteenth image, 1 380
  at the 24th and 1 578 at the 32nd from the assignment, and 1 138, 1 384 and 1 402 from the mirrored assignment; their
  top sixteenth 151, 195 and 227, and 174, 210 and 224. At most 98 of a coupling stood at the rail at a block's end, and
  at most 137 at or above seven eighths of it; 012 read 73 and 106 by the sixteenth schedule.
- **The couplings held under 1.30**, at 0.796 to 1.228 of the image's; from the mirrored assignment the highest rose from
  1.180 to 1.228 over schedules 26 to 31, and the smallest difference between a mapping's two couplings fell to 0.08 in
  the 31st, still favouring the mapping in force.
- **The rest as over schedules 9 to 16**: every set's sum at 0.9999 to 1.0000 of its value at every block's end, 29 parts
  in a million at the farthest; the couplings' share of the readouts' input 13.16 to 13.17 %; the inhibitory sum at
  0.498 to 0.508 of the image's at the last block and the gain at 1.6864 to 1.6943, each falling a little; 18 to 22 % of
  the trials punished.

## 2. The exponent under the carried zero sum

Each cell lists sixteen schedules in order, the first eight before the semicolon:

| $\mu$ | Arm | Highest coupling | Lowest coupling | Three reversals summed, blocks |
| :--- | :--- | :--- | :--- | :--- |
| 1/2 | assignment | 1.147, 1.160, 1.164, 1.172, 1.157, 1.162, 1.159, 1.155; 1.152, 1.160, 1.158, 1.162, 1.165, 1.159, 1.154, 1.162 | 0.859, 0.847, 0.844, 0.836, 0.850, 0.845, 0.852, 0.852; 0.857, 0.847, 0.849, 0.845, 0.842, 0.848, 0.853, 0.855 | 33, 33, 26, 21, 21, 20, 18, 17; 17, 18, 15, 15, 15, 13, 14, 13 |
| 1/2 | mirrored | 1.141, 1.147, 1.142, 1.148, 1.147, 1.158, 1.151, 1.168; 1.174, 1.173, 1.186, 1.189, 1.196, 1.198, 1.201, 1.205 | 0.873, 0.869, 0.873, 0.868, 0.868, 0.858, 0.865, 0.849; 0.844, 0.845, 0.833, 0.830, 0.825, 0.822, 0.820, 0.817 | 36, 31, 25, 23, 24, 19, 19, 18; 18, 16, 18, 17, 17, 16, 16, 15 |
| 1/4 | assignment | 1.138, 1.151, 1.159, 1.166, 1.172, 1.165, 1.168, 1.155; 1.149, 1.148, 1.149, 1.159, 1.165, 1.164, 1.172, 1.162 | 0.868, 0.856, 0.848, 0.842, 0.835, 0.847, 0.850, 0.852; 0.857, 0.858, 0.858, 0.848, 0.842, 0.844, 0.836, 0.845 | 32, 31, 26, 23, 19, 17, 16, 15; 16, 16, 13, 14, 14, 13, 13, 13 |
| 1/4 | mirrored | 1.127, 1.135, 1.133, 1.140, 1.158, 1.159, 1.155, 1.163; 1.162, 1.166, 1.169, 1.167, 1.175, 1.182, 1.182, 1.186 | 0.881, 0.879, 0.874, 0.861, 0.858, 0.855, 0.849, 0.852; 0.850, 0.851, 0.849, 0.851, 0.844, 0.837, 0.837, 0.833 | 35, 28, 28, 22, 23, 18, 17, 18; 15, 14, 16, 13, 14, 14, 13, 12 |

- **The couplings held**, at 0.817 to 1.205 of the image's at $\mu = 1/2$ and 0.833 to 1.186 at $\mu = 1/4$. From the
  mirrored assignment the highest rose over schedules 9 to 16 under both, from 1.174 to 1.205 and from 1.162 to 1.186.
- **Learning held.** Every mapping of the 64 schedules was learned, at 120 to 128 of its last 128, and H-25's four
  clauses held in every one. At each mapping's last block every difference between its two couplings favoured the
  mapping in force, by 0.20 to 0.26 of the image's coupling on average and 0.09 at the smallest.
- **The smaller the exponent, the faster.** Over schedules 9 to 16 the reversals summed 13 to 18 blocks at $\mu = 1/2$ and
  12 to 16 at $\mu = 1/4$, against 16 to 24 at $\mu = 1$; over 13 to 16, 13 to 17 and 12 to 14.
- **The punished trials fell as fast**: from 30 to 33 % in the first schedule to 17 to 20 % over schedules 9 to 16, where
  at $\mu = 1$ they were 19 to 24 %.
- **The inhibitory sum stopped** at 0.508 to 0.519 of the image's over schedules 9 to 16, and the gain stood at 1.687 to
  1.697, falling by 0.0005 to 0.0013 a schedule from the eighth, where the default's fell by about 0.003.
- **The sums held** at 0.9999 to 1.0000 of their values at every block's end, 88 parts in a million at the farthest, and
  the couplings' share of the readouts' input at 13.16 to 13.17 %.

**The whole arena.** At every block's end of all 64 schedules the excitatory arena's sum stood at 0.9994 to 1.0005 of
the image's. Its part outside the four couplings, 5 743 rails in the image, stood within −4.1 and +3.3 rails of it; under
$\mu = 1$ carried over 32 schedules within −3.4 and +3.5, and rounded down within −2.8 and +0.5. Neither exponent moved
it toward ADR-0053's drain on this arm: the four couplings, whose sum the zero sum holds, are where the weight moved, and
the rest of the arena moved by a few rails whatever the exponent. Its distribution was not read.

## 3. The four forms side by side

**The reversals**, three summed a schedule, from the assignment; the default's are explorations 005's and 007's:

| Schedule | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Default | 34 | 27 | 26 | 21 | 20 | 17 | 17 | 15 | 15 | 15 | 13 | 15 | 13 | 14 | 13 | 13 |
| Rounded down | 33 | 17 | 13 | 12 | 12 | 12 | 12 | 12 | 13 | 12 | 12 | 12 | 11 | 13 | 12 | 12 |
| Carried, $\mu = 1$ | 34 | 37 | 32 | 32 | 27 | 23 | 24 | 22 | 23 | 21 | 24 | 18 | 19 | 19 | 21 | 20 |
| Carried, $\mu = 1/2$ | 33 | 33 | 26 | 21 | 21 | 20 | 18 | 17 | 17 | 18 | 15 | 15 | 15 | 13 | 14 | 13 |
| Carried, $\mu = 1/4$ | 32 | 31 | 26 | 23 | 19 | 17 | 16 | 15 | 16 | 16 | 13 | 14 | 14 | 13 | 13 | 13 |

From the mirrored assignment the default's took 40, 33, 31, 22, 24, 20, 20, 16 and 16, 13, 16, 15, 13, 14, 12, 12;
$\mu = 1/4$ 35, 28, 28, 22, 23, 18, 17, 18 and 15, 14, 16, 13, 14, 14, 13, 12. Carried at $\mu = 1$ took 13 to 18 over
schedules 17 to 32 from the assignment (table 1).

**The distribution**, the four couplings' synapses summed, 3 188 of them, by where they stood at each image's end, from
the assignment: the bottom eighth of the width, its middle half (a quarter to three quarters of the rail), and its top
sixteenth:

| Form | Image | Bottom eighth | Middle half | Top sixteenth |
| :--- | ---: | ---: | ---: | ---: |
| H-42's image | 0 | 106 | 1 260 | 0 |
| Rounded down | 2, 4, 8, 12, 16 | 1 561, 2 049, 2 258, 2 294, 2 291 | 740, 412, 212, 190, 188 | 195, 296, 345, 358, 346 |
| Carried, $\mu = 1$ | 2, 4, 8, 12, 16 | 396, 579, 724, 923, 1 063 | 1 012, 857, 743, 624, 561 | 46, 92, 121, 137, 151 |
| Carried, $\mu = 1$ | 20, 24, 28, 32 | 1 262, 1 380, 1 503, 1 578 | 461, 426, 406, 393 | 206, 195, 218, 227 |
| Carried, $\mu = 1/2$ | 2, 4, 8, 12, 16 | 480, 680, 991, 1 270, 1 397 | 1 031, 893, 649, 559, 533 | 42, 83, 174, 220, 211 |
| Carried, $\mu = 1/4$ | 2, 4, 8, 12, 16 | 493, 768, 1 111, 1 430, 1 667 | 1 054, 985, 746, 569, 474 | 59, 80, 179, 225, 268 |

From the mirrored assignment, at the sixteenth image: rounded down 2 301, 195 and 436; $\mu = 1$ 1 138, 538 and 174, and
1 402, 389 and 224 at the 32nd; $\mu = 1/2$ 1 407, 544 and 224; $\mu = 1/4$ 1 665, 464 and 257.

One coupling whole, A→R1 from the assignment, which the last mapping rewards, by sixteenths of the width from the bottom:

| Form | Image | Synapses by sixteenth |
| :--- | ---: | :--- |
| H-42's image | 0 | 9, 18, 151, 283, 203, 89, 30, 10, 3, 0, 4, 2, 4, 0, 0, 0 |
| Rounded down | 16 | 559, 8, 5, 2, 1, 0, 0, 1, 1, 0, 0, 3, 5, 29, 16, 176 |
| Carried, $\mu = 1$ | 16 | 54, 189, 242, 105, 52, 28, 11, 4, 7, 7, 0, 7, 10, 5, 13, 72 |
| Carried, $\mu = 1$ | 32 | 51, 341, 193, 55, 20, 6, 5, 0, 0, 4, 0, 7, 0, 3, 7, 114 |
| Carried, $\mu = 1/2$ | 16 | 86, 223, 208, 90, 28, 12, 15, 11, 6, 7, 2, 1, 1, 3, 0, 113 |
| Carried, $\mu = 1/4$ | 16 | 208, 193, 123, 76, 27, 5, 23, 5, 4, 1, 0, 0, 0, 5, 0, 136 |

- **Every form moved along one path at its own speed**: the bottom eighth filling, the middle emptying and the top
  sixteenth growing. The bottom eighth passed 1 550 by the second image rounded down (1 561), by the fourteenth at
  $\mu = 1/4$ (1 568) and by the thirtieth at $\mu = 1$ (1 579); at $\mu = 1/2$ it stood at 1 397 at the sixteenth.
- **Where the low group stands differs.** Rounded down it is at zero, 559 of the coupling in the bottom sixteenth. At
  $\mu = 1/4$ it spans the bottom eighth, 208 and 193. At $\mu = 1$ it stands above the bottom sixteenth, at 1/16 to 3/16
  of the width: 341 of the coupling in the second sixteenth by the 32nd image, since under ADR-0055's rule the depression
  of a small weight vanishes before the weight does.
- **The couplings the last mapping does not reward** kept a group from half the width to seven eighths of it under every
  form: at the sixteenth image A→R0 held 131 of its 775 synapses there at $\mu = 1/4$, 98 at $\mu = 1/2$, 81 at
  $\mu = 1$ and 154 rounded down.

**The reversals against the split.** Each schedule's three reversals summed, by the bottom eighth of the image it started
from, over the four forms, both arms and every schedule but the first, 152 schedules:

| Bottom eighth at the start | Schedules | Reversals summed, blocks |
| :--- | ---: | :--- |
| 200 to 599 | 18 | 21 to 37 |
| 600 to 799 | 16 | 19 to 26 |
| 800 to 999 | 22 | 16 to 24 |
| 1 000 to 1 199 | 18 | 15 to 20 |
| 1 200 to 1 399 | 32 | 13 to 18 |
| 1 400 to 1 599 | 18 | 13 to 16 |
| 1 600 and more | 28 | 11 to 13 |

Each band mixes the forms at different schedules: from 800 to 999, rounded down in its second schedule took 17 and 21
blocks, $\mu = 1/4$ in its sixth and seventh 16 to 18, $\mu = 1/2$ in its sixth to ninth 17 to 19, and $\mu = 1$ in its
tenth to fifteenth 16 to 24.

**What the rule and the scaling moved, by band**, in rails a schedule, by the band a synapse stood in before it moved:
- **After a reward** the rule took 135 to 185 rails a schedule out of the synapses below ADR-0055's reference magnitude
  under both exponents, and put 92 to 208 into those at or above three quarters of the rail; at $\mu = 1$ over 012's
  sixteen schedules it took 108 to 148 and put 78 to 177. In the first schedule it took 159 to 185 under the exponents
  and 108 and 117 at $\mu = 1$. After a punishment the moves reversed: 128 to 172 rails back into the bottom band and 80
  to 217 out of the top, where at $\mu = 1$ 90 to 146 and 68 to 189.
- **The scaling took from every band** under both exponents in every schedule, 10 to 34 rails from the bottom, 10 to 37
  from the middle and 2 to 41 from the top, as at $\mu = 1$ carried. It moved no weight from the small synapses to the
  large ones, which rounded down it did.

## What it costs

A round took 19.8 to 20.8 µs under every exponent, as under 012's carried zero sum, and a schedule's rounds 0.62 to 1.29
s of its about 700. The jobs took 5 517 to 6 081 s for eight schedules beside the derivation, 012's 5 617 to 5 956.
The exponent itself adds one or two integer square roots to an excitatory depression; it was not timed apart.

## What it suggests

**Under the carried zero sum, $\mu = 1/4$ makes the two-group coupling by the rule itself, and with it the default's
learning, while the couplings stay within 0.83 to 1.19 of the image's.** No rounding drains the small synapses here: the
scaling takes from every band, and it is the depression, larger than ADR-0055's below the reference and smaller above
it, that empties the middle. Its three reversals took the default's blocks, schedule by schedule, where the default's
couplings rose to 2.07 by the sixteenth schedule. A verdict is a measured round's: a hypothesis written before its runs,
on H-42's three networks.

**A reading, not measured: the speed is the split's.** Under all four forms, rounded down, carried at $\mu = 1$, $1/2$ and
$1/4$, a schedule's reversals tracked how far its couplings had split, and not how many schedules had passed. The
exponent sets how fast the split comes: at $\mu = 1$ the bottom eighth held by the 32nd image what it held at
$\mu = 1/4$ by the fourteenth. ADR-0055's failure mode, the weaker competition, would on this reading be slowness, not
its absence.

**Neither long run had stopped.** At $\mu = 1$ carried the reversals still fell by about a block in eight schedules at
the 32nd, and the tails still grew. At $\mu = 1/2$ and $1/4$ the bottom eighth was still filling at the sixteenth image.
From the mirrored assignment the highest coupling rose over the last eight schedules of each, to 1.228 at $\mu = 1$,
1.205 at $\mu = 1/2$ and 1.186 at $\mu = 1/4$; where it ends was not run.

Questions this suggests for a later exploration or a round:
- $\mu = 1/4$ under the carried zero sum past sixteen schedules, and on seeds 23 and 24;
- the exponent without the zero sum, under the default: whether the couplings rise as the default's do;
- $\mu$ below a quarter, toward the additive rule, and whether the arena drains then, which this arm does not read: its
  synapses outside the couplings moved by a few rails under every form;
- whether the reversals follow the split on a network the split was not made on: a prediction a round could write
  from the table above.

## What did not run

- **Seeds 23 and 24.**
- **The exponents past sixteen schedules, and the carried zero sum past 32.**
- **Other exponents**: $\mu$ at 3/4, below 1/4 or at zero; an exponent on the potentiation.
- **The exponent without the zero sum.**
- **The distribution of the arena outside the couplings**, whose sum alone this note reads.
- **The fractions carried across schedules.** They are in no image, so each schedule started them again at a half, as in
  012.
- **The default's synapses by sixteenths of the width**, which no run here read.
- **The rate's error**, and the inhibitory rule's replay terms, which the logs hold and this note does not read.
- **The nights** of ADR-0044 and ADR-0048 under the zero sum or an exponent.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `the_carried_sum_and_the_exponent_short_on_seed_22_explore` | [40671679938](https://github.com/Menteion/Menteion/actions/runs/40671679938) | `795b1b8` | 188 |
| `the_carried_sum_past_sixteen_schedules_on_seed_22_from_the_assignment_explore` | [40671679938](https://github.com/Menteion/Menteion/actions/runs/40671679938) | `795b1b8` | 5540 |
| `the_carried_sum_past_sixteen_schedules_on_seed_22_from_the_mirrored_assignment_explore` | [40671679938](https://github.com/Menteion/Menteion/actions/runs/40671679938) | `795b1b8` | 6081 |
| `the_carried_sum_at_the_half_power_on_seed_22_from_the_assignment_explore` | [40671679938](https://github.com/Menteion/Menteion/actions/runs/40671679938) | `795b1b8` | 5517 |
| `the_carried_sum_at_the_half_power_on_seed_22_from_the_mirrored_assignment_explore` | [40671679938](https://github.com/Menteion/Menteion/actions/runs/40671679938) | `795b1b8` | 5749 |
| `the_carried_sum_at_the_quarter_power_on_seed_22_from_the_assignment_explore` | [40671679938](https://github.com/Menteion/Menteion/actions/runs/40671679938) | `795b1b8` | 5679 |
| `the_carried_sum_at_the_quarter_power_on_seed_22_from_the_mirrored_assignment_explore` | [40671679938](https://github.com/Menteion/Menteion/actions/runs/40671679938) | `795b1b8` | 5888 |
| `the_carried_sum_past_twenty_four_schedules_on_seed_22_from_the_assignment_explore` | [40687049717](https://github.com/Menteion/Menteion/actions/runs/40687049717) | `8d0c0e2` | 5780 |
| `the_carried_sum_past_twenty_four_schedules_on_seed_22_from_the_mirrored_assignment_explore` | [40687049717](https://github.com/Menteion/Menteion/actions/runs/40687049717) | `8d0c0e2` | 5919 |
| `the_carried_sum_at_the_half_power_second_half_on_seed_22_from_the_assignment_explore` | [40687049717](https://github.com/Menteion/Menteion/actions/runs/40687049717) | `8d0c0e2` | 5960 |
| `the_carried_sum_at_the_half_power_second_half_on_seed_22_from_the_mirrored_assignment_explore` | [40687049717](https://github.com/Menteion/Menteion/actions/runs/40687049717) | `8d0c0e2` | 5797 |
| `the_carried_sum_at_the_quarter_power_second_half_on_seed_22_from_the_assignment_explore` | [40687049717](https://github.com/Menteion/Menteion/actions/runs/40687049717) | `8d0c0e2` | 5850 |
| `the_carried_sum_at_the_quarter_power_second_half_on_seed_22_from_the_mirrored_assignment_explore` | [40687049717](https://github.com/Menteion/Menteion/actions/runs/40687049717) | `8d0c0e2` | 5608 |

Carried at $\mu = 1$ over schedules 1 to 16 is exploration 012's runs
[40641556801](https://github.com/Menteion/Menteion/actions/runs/40641556801) and
[40653304865](https://github.com/Menteion/Menteion/actions/runs/40653304865); rounded down is exploration 011's
[40614819069](https://github.com/Menteion/Menteion/actions/runs/40614819069) and
[40625863565](https://github.com/Menteion/Menteion/actions/runs/40625863565); the default's are explorations 005's and
007's, as 011 tabled them. This note read their logs again for the arena's sum and the bands of the width.
