---
status: accepted
date: 2026-09-27
depends-on: ADR-0111
decision-makers: VirtualCortex maintainers
---

# ADR-0112: An assembly that holds, measured — ADR-0111's first round, nothing of the engine changed: on ADR-0077's settled image with every weight frozen, assemblies of 16, 32 and 64 excitatory units wired at four recurrent weights and run through one protocol by rules written before any run; the kick derived from the membrane rule and, after its first reading and before any cell, given a reset; no cell is usable — three hold by the rule (32 units at the top weight, 64 at 0.75 and at the top), each of them also ignites without a kick and does not let go, and the tables say what the holds are: population bursts, one set off by each kick and, in the stronger cells, more of their own, each draining the members' pool to a few per cent, never a sustained rate; F-54; the next decision named and not taken

## Context and Problem Statement

[ADR-0111](0111-a-rule-held-by-the-network.md) took a representation of the rule in force as the learning line's next question and staged it in three rounds. Its first round, brief 048, asks whether the substrate exists: **can an assembly of the reference network hold its activity once kicked, stay quiet when not kicked, and let go on a signal?** Nothing of the engine changes. On [ADR-0077](0077-the-background-side.md)'s settled image, with every weight frozen, a grid of assemblies is wired as the test's own network, and each cell is measured by rules written before any run.

Four facts of the tree shaped how the round is built. Each was read before anything was written (principle 2).

1. **No assembly of sixteen units or more can be disjoint from the task's sets at 1 024 units.** Brief 048 asks for assemblies disjoint from the task's stimulus and readout sets, so that a later round can put the task and the context in one network. The task's geometry (`geometry` in `tests/instrument/harness.rs`, [ADR-0065](0065-the-instrument-recalibrated.md)) reads the ring in periods of twenty from rotation 0, and every place of a period is in exactly one set: stimulus A the first place, B the twelfth, readout 0 nine odd places and readout 1 nine even places. Fifty-one whole periods cover units 0 to 1 019, so only units 1 020 to 1 023 are in no set. The brief's disjointness names assemblies the network does not have room for. This is finding **F-54** (whitepaper §11).
2. **The prior's blocks are full, and the loader sizes the arena from the image.** ADR-0044's prior gives each unit 32 synapses in eight blocks and `blocks_for` sizes the arena at exactly that (`synthesis.rs`). `Image::decode` takes the block count from the image's synapse section, not from the configuration. So an assembly's synapses need blocks beyond the prior's, and the settled image has none to give.
3. **ADR-0076's stimulus fires each unit once only because its cancel then holds the unit below rest.** The cancel lands on ticks 202 to 210, as each unit's refractory window ends. It takes the basal compartment to about −187 and keeps the soma below the drive's mean standing for about 3 500 ticks. Every recurrent message of an assembly lands either in the refractory window, which drops it (ADR-0076), or on a unit the cancel is holding down. Kicked that way, an assembly could not hold by construction. The cancel is a release, and the brief names it as one.
4. **Short-term plasticity is the presynaptic unit's.** `stp_u_rel` and `stp_r_ves` are fields of `DendriticSuperNeuron`, stepped once per spike of the unit (`step_stp`, `executor.rs`'s turn). Every synapse of a member releases with the member's one pair `(u, R)`, the prior's synapses and the assembly's alike. "The assembly's short-term state" is the members' pairs.

## Decision Drivers

- Brief 048's standing directives: no rule of the engine changes; every weight is frozen and shown unchanged at each run's end; the grid, the measures and their thresholds are written before any run and do not move after one; the rules' arithmetic is written first; no float, oracles included; every loop ends by construction; the engine is read before a description of it is trusted; no pinned number moves.
- Latest ≠ Newest: no dependency, no tool, no rule. The assembly is test wiring: blocks appended to an image the harness already writes and reads, chained by the core's own `link` and `set_synapse`.
- [ADR-0061](0061-the-learning-runs-leave-the-gate.md): the runtime's gate grows by at most one test, and the runs are weekly `exhaustive` tests.
- The brief's empowerment: where the assemblies sit and how their synapses are chained, the delays, the epochs' number and windows within the measures' definitions, the release, a widened grid, where the tests live, one ADR or two.

## Considered Options

1. **Where the assembly's synapses live**:
   - (a) blocks appended to the settled image's synapse section, the image laid out again and decoded, each member's new blocks chained after its own;
   - (b) a fresh executor with a larger arena, synthesized from the prior and led in again, whose lead-in would have to be held to ADR-0077's tables with an arena of another size;
   - (c) the prior's own slots, overwritten.
2. **The kick**:
   - (a) ADR-0076's pick, F-46's drive with its cancel;
   - (b) F-46's drive alone;
   - (c) one message;
   - (d) a ramp derived from the membrane rule.
3. **The release**:
   - (a) ADR-0076's cancel as built, into the members;
   - (b) that cancel spread over the refractory window;
   - (c) a volley into nearby inhibitory units.
4. **Where the members sit**:
   - (a) contiguous;
   - (b) two places of each period, beyond the prior's window of one another.
5. **The background the thresholds read against**:
   - (a) the members' rate on the same image unwired, under the same drive over the same ticks, with no kick;
   - (b) each cell's own lead-in.
6. **Where the tests live**: a binary of their own on the shared harness, or `instrument.rs`.
7. **One ADR or two.**

## Decision Outcome

Options 1(a), 2(d), 3(a), 4(b), 5(a), a binary of its own (`runtime/cortex-runtime/tests/assembly.rs`, the harness shared as one module, [ADR-0083](0083-plasticity-everywhere-measured.md)), and one ADR. Everything below was committed before any run of the grid.

### The grid

- **Sizes**: 16, 32 and 64 excitatory units, the brief's. The grid is not widened. The arithmetic below says why a size of 128 would ask nothing new: at 64 units a member already receives the most synapses a member sends, 32.
- **Weights**: 0x2000, 0x4000, 0x6000 and `i16::MAX` in Q1.15. That is a quarter, a half and three quarters of the range, and its top, one LSB below 1.0. The prior's excitatory weights are 6 000 to 12 000.
- **Placement**: places 5 and 16 of each period of twenty, from the ring's start, over the first `size / 2` periods (`PLACES`, `assembly`). Both places are excitatory by the prior's rule, since every fifth unit from the fifth is inhibitory (places 4, 9, 14 and 19). Neither is a stimulus place (0 and 11). They are eleven and nine places apart, beyond the prior's window of eight, so no local synapse of the prior joins two members. Place 5 is readout 0's and place 16 readout 1's, so F-54's forced overlap with the readouts is split evenly between them. The sizes nest: the members of 16 are the first sixteen of 32's, and those of 32 the first 32 of 64's. The members of 64 run from unit 5 to unit 636.
- **Wiring** (`wire`): member $k$ sends one basal synapse at the cell's weight to each of the next $\min(\text{size} - 1, 32)$ members in the assembly's order, wrapping. That is 15 targets at 16 units, 31 at 32, and 32 of 63 at 64. Every member receives as many as it sends. The delays are drawn from the prior's local band, 100 to 300 ticks inclusive, by `mix64` of seed 48, the source and the target (`delay_of`).
- **Chaining** (`grown`): the settled image with `size × ⌈fan / 4⌉` empty blocks appended to its synapse section — 64, 256 and 512 — laid out again as `Image::encode` lays an image out, with a new header seal, directory and section CRCs. Member $k$'s blocks follow in order, the first linked after the last block of its own chain. Every record is the image's. The gate shows that the grown image unwired runs as the image bit for bit, and each control run shows it on the settled network: its lead-in and first epoch are the background's.

### The protocol

Each run starts from the frozen image decoded at its written tick, so every run of the round meets the same drive at the same ticks. It reads ADR-0044's drive the whole time, the gain 1.75 carried in the image and the controller off.

- **A lead-in** of one epoch, unkicked, read as the run's first row.
- **Twenty-four epochs** of $2^{14}$ ticks, one trial's length: an unkicked, a kicked and a released epoch, eight times over (`ORDER`). So every unkicked epoch after the first follows a release, and every kicked one follows an unkicked one.
- **Windows**: each epoch is read in eight windows of 2 048 ticks. The last half, windows 4 to 7, is the hold's window and the tail's.
- **The kick** (`KICK_MESSAGE_Q16`, `KICK_RESET_Q16`, `KICK_TICKS`): one basal message of 1/64 into every member before each of the epoch's first 160 ticks, a ramp. A member fires when its soma crosses the threshold, then drops the rest of the ramp in its refractory window. So it fires once whatever its standing potential, and is left holding about what firing takes. The message is derived by a rule (`kick_rule`): the first of 1/256, 1/128, 1/64, 1/32 and 1/16 under which the oracle fires a unit exactly once within the span and not again within a pair window after it, from every standing of `KICK_STANDINGS` under the drive's mean input. The standings are rest, the drive's mean standing, ADR-0076's extreme, and one and two thresholds below rest. 1/128 leaves the two lowest unfired within the span, and 1/64 passes. The ramp stops forty ticks before the span's end, five of the soma's time constants of eight ticks, so that a unit the last message brings to the threshold fires inside the span. The span is the epoch's ticks 1 to 200 (`KICK_SPAN`), in which no unit fires twice.
- **The kick read on the engine, before any cell** (`the_kick_and_the_background_at_1024_units_exhaustive`): each size's control is the grown image, unwired, run through the protocol. Over its sixteen kicks the kick must fire every member once by the measure that picked ADR-0076's stimulus (ADR-0074's `fires_once`), read per kick:
  - the volley: within `VOLLEY_TOLERANCE_TENTHS` tenths of one spike per member per kick, and at most one;
  - the after: at most `AFTER_MAX_TENTHS` tenths of a spike per member per kick in the pair window after the span.
  If it does not, no cell is run and the kick is derived again. The gate holds the probe: the ramp into one unit at rest, with no drive, fires it on the tick the oracle says, once.
- **The kick derived again, after its first reading and before any cell.** The first run of `the_kick_and_the_background_at_1024_units_exhaustive` (at `a95a1c3`, 70 s in the release profile on a developer machine) read the ramp alone:
  - the growth changing nothing unkicked, at sixteen units, before the test stopped;
  - the volley within the tolerance at every size: 254 of 256 at 16 units, 510 of 512 at 32 and 1 022 of 1 024 at 64, each size missing one member in the first kicked epoch and one other kick, 14 of 16 kicks a full volley;
  - the after: 26, 49 and 84 spikes over the sixteen kicks. Against the marks of 25.6, 51.2 and 102.4, sixteen units failed by one spike.

  By the rule above, no cell ran. The background run in the same test put the members' spikes in the same windows with no kick at 7 at sixteen units, so the kick's own excess was about 19 spikes, 0.07 a member a kick. The oracle says where they come from: a member leaves its refractory window with its basal compartment at about 1.49 and its soma at about 0.75, while the drive alone holds a unit at 0.875 and 0.436. So the kick leaves a residual in every member it fires.

  A ramp cannot lower that residual. Firing takes a basal of about 2.0, which decays over the window to about 1.35 whatever the ramp.

  The re-derivation adds **a reset** (`KICK_RESET_Q16`, `reset_rule`). The protocol reads each member's kick spike from its record, `last_soma_spike_tick`, on the tick it fires inside the span. It then injects one basal message into that member before the epoch's tick after the member's refractory window, so the message lands on the first tick the member integrates again. The message's efficacy is the one the gain takes nearest to putting the member's basal compartment at the drive's mean standing, from what the ramp left there, by the oracle kicked from that standing: **−23 062** of $2^{16}$ before the gain, −0.616 after it. From every standing of `KICK_STANDINGS` the oracle then fires the unit once. On the tick the reset lands, it puts the basal at:
  - 0.875 from rest and from the drive's mean standing;
  - 0.870 and 0.866 from one and two thresholds below rest;
  - 0.754 from the extreme.

  Sixty-four ticks later the soma stands within 0.004 of the drive's mean standing, and at 0.382 from the extreme. The gate holds the engine to the oracle on one unit at rest with no drive: it fires on the oracle's tick, once, and its basal potential after every tick of the span and the pair window after it is the oracle's, the reset's tick included.

  The reset is the kick's and not a release. It takes back only what the kick put in, and it lands on the member's first tick after its own refractory window. A recurrent message of its assembly that lands while the member is refractory is dropped either way. One that lands after adds to the standing the reset leaves, not to the kick's residual. The measure, its marks and every other constant are unchanged. Nothing else is derived again.
- **The release** (`release`): ADR-0076's cancel as built — six messages at −2.0 into every member before each of nine ticks — from the epoch's quarter, tick 4 096. A member refractory through all nine ticks escapes it: at a rate of $r$ Hz, about $192r / 10^5$ of the members, 3.8 per cent at 20 Hz and 19 per cent at 100 Hz. The tail is the last half, from tick 8 192. It opens after the release's direct effect has passed on a member it reached: at the drive's mean standing, the oracle has the soma back within a tenth of the threshold of that standing 3 510 ticks after the first message (`RELEASE_RECOVERED`).
- **Rows** (`EpochRow`): per window, the members' spikes, the rest of the network's spikes, and the sums over the members of the pair `(u, R)` a spike on the next tick would release with. That pair is `step_stp` on a copy of each member's factors, with the ticks since its last spike as the executor reads them. Per epoch, the kick's reading: the members' spikes in the span and in the pair window after it.
- **The runs**:
  - the background: the frozen image, unwired, all 25 epochs unkicked, read for each size's members;
  - three controls: the grown image unwired, the protocol;
  - twelve cells: the grown image wired at each weight, the protocol.

  Every run is dumped before any is held to its table. Every weight of the arena at each run's end is asserted equal to its value at the start.

### The measures

The background (`background_of`) is the members' spikes and the rest's over the background run's twenty-four epochs, 393 216 ticks. Every comparison is in integers: $s$ spikes over $t$ ticks are at least $f$ times $b$ over $T$ when $s T \ge f b t$.

- **Holds**: in at least 7 of the 8 kicked epochs, the members' spikes over the last half are at least five times the background.
- **Ignites**: the unkicked epochs whose last half reaches that rate. A cell may have at most 1 of 8.
- **Lets go**: in at least 7 of the 8 released epochs, the members' spikes over the tail are at most twice the background.
- **Spills**: over the last halves of the kicked epochs that held, the rest of the network's spikes are more than twice the rest's background. If none held, spills is not read.
- **Usable**: holds, lets go, ignites in at most 1 of 8, and does not spill.
- **Read, no clause**: the released epochs whose window before the release held at five times the background.
- **What failed, where nothing is usable**: never holding (holds is false), running away (ignites more than once, or spills), not letting go (lets go is false).

**What the thresholds read by chance**, as arithmetic before the run, for a Poisson background at the settled network's 1.7 Hz (ADR-0077). A quiet assembly's last half expects 2.2, 4.5 and 8.9 spikes at 16, 32 and 64 units. So a quiet assembly fails "lets go" in one released epoch by chance with probability about 7.6, 3.8 and 0.5 per cent. It fails the cell's seven of eight with probability about 12, 3.5 and 0.06 per cent. A quiet assembly reads "held" or "ignited" by chance with probability below $10^{-4}$ per epoch.

### The arithmetic, before any run

Every number below comes from an oracle in `tests/assembly.rs` and is pinned in the gate. The oracles are `step_stp` and `integrate` themselves, stepped alone, with the executor's scaling by the gain.

**Short-term plasticity at a sustained rate** (`steady`: `step_stp` from rest at a fixed interval until the pair repeats). The pair a spike after a long rest releases with is $(92, 255)$, a factor $u R$ of 23 460 of $2^{16}$.

| Rate | Interval (ticks) | Steady pair $(u, R)$, Q0.8 | $uR$ | Of the factor at rest | Transmitted a second, in spikes at rest |
| ---: | ---: | :--- | ---: | ---: | ---: |
| 1.76 Hz, the background (ADR-0097) | 56 818 | (93, 237) | 22 041 | 0.940 | 1.65 |
| 5 Hz | 20 000 | (105, 172) | 18 060 | 0.770 | 3.85 |
| 10 Hz | 10 000 | (122, 110) | 13 420 | 0.572 | 5.72 |
| **20 Hz** | **5 000** | **(149, 57)** | **8 493** | **0.362** | **7.24** |
| 40 Hz | 2 500 | (182, 26) | 4 732 | 0.202 | 8.07 |
| 100 Hz | 1 000 | (214, 10) | 2 140 | 0.091 | 9.12 |
| 200 Hz | 500 | (234, 4) | 936 | 0.040 | 7.98 |
| 400 Hz | 250 | (242, 2) | 484 | 0.021 | 8.25 |

**At a sustained 20 Hz a synapse's efficacy is 0.362 of its efficacy at rest** (`STEADY_20_HZ_PER_MILLE`, 362). Facilitation raises $u$ from 0.36 to 0.58, and the pool falls from 1.0 to 0.22. A member firing at 20 Hz transmits 4.4 times what it transmits at the background rate. No rate transmits more than about 9.1 spikes at rest a second, 5.5 times the background's.

**What one spike of a member delivers to a target**, after the gain, against the threshold (`DELIVERED`). The drive's mean input is 112 of $2^{16}$ a tick after the gain. It holds a unit at a mean standing of 0.875 basal and 0.436 soma (`DRIVE_STANDING`), 0.564 below the base threshold of 1.0.

| Weight | At rest: delivered | Soma's peak from the drive's mean standing | Spikes together to fire | At 20 Hz: delivered | Peak | Spikes together |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 0.25 | 0.157 | 0.509 (+0.073) | 8 | 0.057 | 0.462 (+0.027) | 22 |
| 0.5 | 0.313 | 0.583 (+0.147) | 4 | 0.113 | 0.489 (+0.053) | 11 |
| 0.75 | 0.470 | 0.656 (+0.220) | 3 | 0.170 | 0.515 (+0.080) | 8 |
| 1.0 | 0.626 | 0.729 (+0.293) | 2 | 0.227 | 0.542 (+0.106) | 6 |

No single spike fires a unit at the drive's mean standing at any weight. At rest it takes 2 to 8 members' spikes landing together, and at 20 Hz 6 to 22.

**The members' own mean input while every member fires at a rate** (`recurrent`): fan × delivered × 512 / interval as a basal potential, and the soma that holds against it, 128/257 of it (the soma leaks and is coupled by a sixteenth to each compartment, the apical at zero). Below is the soma, beside the drive's mean standing of 0.436:

| Size (fan) | Weight | At the background's rate | At 20 Hz | At 100 Hz | Mean soma at 100 Hz, with the drive's |
| :--- | :--- | ---: | ---: | ---: | ---: |
| 16 (15) | 0.25 / 0.5 / 0.75 / 1.0 | 0.010 / 0.020 / 0.030 / 0.040 | 0.043 / 0.087 / 0.130 / 0.173 | 0.055 / 0.109 / 0.164 / 0.218 | 0.49 / 0.55 / 0.60 / 0.65 |
| 32 (31) | 0.25 / 0.5 / 0.75 / 1.0 | 0.020 / 0.041 / 0.061 / 0.082 | 0.090 / 0.179 / 0.269 / 0.359 | 0.113 / 0.226 / 0.339 / 0.451 | 0.55 / 0.66 / 0.77 / 0.89 |
| 64 (32) | 0.25 / 0.5 / 0.75 / 1.0 | 0.021 / 0.042 / 0.063 / 0.085 | 0.093 / 0.185 / 0.278 / 0.370 | 0.117 / 0.233 / 0.350 / 0.466 | 0.55 / 0.67 / 0.79 / 0.90 |

**In no cell does the members' mean input, at any rate, bring the mean soma to the threshold.** The largest, 64 units at 1.0 firing at 100 Hz, stands at 0.90 of it, and depression caps what a higher rate adds. A hold, if one exists, is the fluctuations above a mean below threshold: the drive's shot noise and the members' own messages. It is not a mean above threshold. This is the arithmetic the readings are set beside. It makes no prediction of the region, and the brief asks for none.

**The kick by the oracle** (`KICK_ORACLE`: the drive's mean input every tick, the ramp on its first 160 ticks, and, as derived again after the kick's first reading, the reset on the tick after the refractory window), from each standing:

| Standing (basal, soma) | Fires on | Basal at the spike | Soma on the window's last tick | Basal then | Basal as the reset lands | Soma 64 ticks after |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| rest (0, 0) | 82 | 2.204 | 0.753 | 1.492 | 0.875 | 0.436 |
| the drive's mean (0.875, 0.436) | 51 | 2.204 | 0.753 | 1.492 | 0.875 | 0.436 |
| the extreme (2.0⁻, 1.0⁻) | 1 | 2.025 | 0.692 | 1.371 | 0.754 | 0.382 |
| one threshold below (−1.0, −0.5) | 115 | 2.197 | 0.751 | 1.487 | 0.870 | 0.434 |
| two thresholds below (−2.0, −1.0) | 146 | 2.190 | 0.749 | 1.483 | 0.866 | 0.432 |

Each fires once. When its window ends, a unit stands about 0.27 below the adapted threshold of about 1.02, where F-46's drive left it above (ADR-0076). The reset takes the rest.

**The release by the oracle** (`RELEASE_ORACLE`, the drive's mean input every tick):
- from the drive's mean standing: no spike for the rest of the epoch, the basal down to −186.7, the soma back within a tenth of the threshold of the standing after 3 510 ticks;
- from the extreme: −185.6, and 3 507 ticks.

### The order of the work

1. This ADR, the constants, the rules and the oracles, and the gate's checks of them, committed before any run (`6cfc32f`, `a95a1c3`).
2. The kick and the background run; the kick, read once and derived again with its reset before any cell (above), read again, its tables pinned, and shown to fire every member once.
3. The twelve cells run against the background pinned in step 2, their tables pinned.
4. The readings recorded below.

### The readings

Every reading below is from the pinned tables of `tests/assembly.rs`, and a second run of every test reproduced them. The runs are in the order above:
- the kick and the background at `cc05cd8`, and again at `712ef21` after the kick's helpers were shared with the gate;
- the three cell tests at `712ef21` side by side, and again against their tables.

On a developer machine in the release profile the kick and background test took 70 s, and each cell test 93 s alone or 138 s three at a time (a ratio, not admissible). No weight of any arena moved in any run.

- **The background** (`BACKGROUNDS_1024`: the unwired frozen image under the drive, with no kick, over the protocol's 393 216 ticks after the lead-in). The members fired 108, 218 and 440 spikes at 16, 32 and 64 units, 1.72, 1.73 and 1.75 Hz a member. The rest fired 6 954, 6 844 and 6 622, 1.75 Hz a unit, ADR-0077's settled rate.
- **The kick, read on the engine with its reset** (`KICKS_1024`, `KICKED_ONCE_1024`, each size's control). Every member fires once by the measure at every size:
  - the volley is 254 of 256, 510 of 512 and 1 022 of 1 024 over the sixteen kicks, 14 of the 16 a full volley;
  - the after is 10, 22 and 42 against marks of 25.6, 51.2 and 102.4, about 0.04 a member a kick, which is what the background puts in those windows.

  Each control's lead-in and first epoch are the background's bit for bit, so the growth changes nothing unkicked on the settled network.
- **The controls** (`CONTROLS_1024`: the grown image unwired, with the kick and the release). At every size no kicked epoch holds, no unkicked epoch ignites, and every released epoch lets go. The kick alone holds nothing.
- **The grid** (`GRID_1024`, by the rules committed first, against the background pinned before any cell ran):

  | Size | Weight | Holds, of 8 | Ignites, of 8 | Lets go, of 8 | Spills | Usable | What failed |
  | ---: | :--- | ---: | ---: | ---: | :--- | :--- | :--- |
  | 16 | 0.25 | 0 | 0 | 8 | not read | no | never holding |
  | 16 | 0.5 | 0 | 0 | 8 | not read | no | never holding |
  | 16 | 0.75 | 0 | 1 | 8 | not read | no | never holding |
  | 16 | 1.0 | 1 | 2 | 6 | no | no | never holding; running away; not letting go |
  | 32 | 0.25 | 0 | 0 | 8 | not read | no | never holding |
  | 32 | 0.5 | 1 | 1 | 5 | no | no | never holding; not letting go |
  | 32 | 0.75 | 3 | 5 | 4 | no | no | never holding; running away; not letting go |
  | 32 | 1.0 | **7** | 5 | 2 | no | no | running away; not letting go |
  | 64 | 0.25 | 0 | 0 | 8 | not read | no | never holding |
  | 64 | 0.5 | 0 | 3 | 7 | not read | no | never holding; running away |
  | 64 | 0.75 | **7** | 2 | 1 | no | no | running away; not letting go |
  | 64 | 1.0 | **7** | 6 | 0 | no | no | running away; not letting go |

  **No cell is usable, and the grid has no usable region.** At a quarter and a half of the range nothing holds. The three cells that hold by the rule each also ignite without a kick in two to six of eight epochs, and let go in at most two. Wherever the spill was read, the rest of the network fired at 0.99 to 1.08 of its background over the held last halves, so nothing spills. In no released epoch of any cell did the window before the release hold at five times the background.
- **What the holds are** (`BURSTS_1024`, `bursts`). This is a reading written after the runs, from what the tables showed, and it is no clause. A burst window is one in which the members fire at least once each in 2 048 ticks, about 28 times the background.

  | Size | Weight | Burst windows, of 192 | Not a kick's first window | Held last halves | Fewest quiet windows in one | Members' pool $R$ at a burst window's end, of 255 |
  | ---: | :--- | ---: | ---: | ---: | ---: | ---: |
  | 16 | control | 14 | 0 | 0 | — | 102 |
  | 16 | 0.25 / 0.5 / 0.75 / 1.0 | 16 / 16 / 19 / 25 | 0 / 0 / 3 / 9 | 0 / 0 / 0 / 1 | — / — / — / 3 | 88 / 27 / 19 / 14 |
  | 32 | control | 15 | 0 | 0 | — | 104 |
  | 32 | 0.25 / 0.5 / 0.75 / 1.0 | 18 / 21 / 36 / 42 | 2 / 5 / 20 / 28 | 0 / 1 / 3 / 7 | — / 3 / 2 / 2 | 27 / 14 / 13 / 10 |
  | 64 | control | 15 | 0 | 0 | — | 103 |
  | 64 | 0.25 / 0.5 / 0.75 / 1.0 | 16 / 23 / 37 / 48 | 0 / 7 / 22 / 32 | 0 / 0 / 7 / 7 | — / — / 2 / 1 | 24 / 14 / 11 / 9 |

  **No held last half of any cell is without a quiet window**, one in which the members fire at most twice the background. A held last half averages 7 to 27 times the background (23 to 27 in the three cells that hold), and one to three of its four windows are quiet. What the rule reads as a hold is one burst, not a sustained rate. The rows show the course:
  - **Each kick sets off a burst.** In every wired cell but the weakest, a member fires 1.5 to 2.4 more times a kick in the pair window after the span (0.25 at sixteen units and a quarter of the range), against 0.04 unwired. It fires at the refractory limit, a spike about every 220 ticks. The gate's 800 ticks on the prior's network show this at sixteen units and the top weight: every member fires at about 80, 300, 530 and 750 ticks.
  - **The burst drains the pool.** At the burst windows' ends the members' pool $R$ stands at 9 to 27 of 255, 3.5 to 10.6 per cent, in every wired cell but the weakest (88 there), against 102 to 104 after the controls' kick volleys. That is the pool the arithmetic's steady pair has at 100 Hz (10); a sustained 20 Hz would hold 57.
  - **The assembly is then silent** until the pool recovers, with $\tau_d$ of $2^{15}$ ticks. So the window after a kick's window is never a hold, and a kick leaves the assembly silent, not active.
  - **In the stronger cells the next burst comes without a kick.** Once the pool has recovered enough, the drive's fluctuations set off another. At 0.75 and the top weight of 32 and 64 units, 20 to 32 of the burst windows fall outside a kick's first window, the bursts starting two to twelve windows apart. Those bursts are the ignitions, and after a release they are the failures to let go. The release reaches the members but does not drain their pool, so the assembly bursts again once the release's effect has passed; in the three cells that hold, the tails that do not let go read 19 to 31 times the background.
  - **Between bursts it is the pool that is low.** At the top weight of each size, over the whole run, the members' release fraction $u$ averages 0.61 to 0.72, against 0.39 unwired, and their pool 0.13 to 0.24 of its rest, against 0.82. In their silent windows $u$ stands at 0.60 to 0.68 and the pool at 0.18 to 0.27. What silences a strong assembly between bursts is the pool.
- **The arithmetic beside the readings.** Before any run, the arithmetic said that no cell's mean input can hold a member at threshold at any rate: depression caps what a rate transmits at about 5.5 times the background's. So a hold would have to be the fluctuations riding above a mean below threshold. The runs read no such state. They read the other behaviour the rule allows: a regenerative burst on the pool the members have, then silence while the pool recovers. The bursts' strength follows the arithmetic's order:
  - at a quarter of the range nothing bursts on its own at any size; there one spike at rest delivers 0.157, and eight must land together to fire a unit;
  - the most bursts come where the fan and the weight are largest.
- **Not done:**
  - The grid was not widened. A size of 128 gives a member no more synapses than 64 does. A weight between two of the four would place the edge between burst-free and bursting, and the question does not turn on that edge.
  - The grid was not read on a network drained as a learning run leaves it.
  - There is one seed of the delays, one placement and one drive.
  - No rule of the engine changed, and no context, gated readout or reward was built.
- **The next decision, named and not taken.** This is ADR-0111's own branch for "nothing usable": **an ADR on a mechanism of persistence**, with these readings as its need:
  - the assemblies do not hold: a kick sets off one burst and then silences them, and the strong ones burst of their own every few windows, kicked or not;
  - each burst leaves the pool at a few per cent, and between bursts the pool, not $u$, stays low: $u$ near 0.6 to 0.7 and the pool near a fifth of its rest.

  ADR-0111 named two candidates, each a rule change with its own need:
  - synapses whose facilitation outlasts their depression, for the context's units. In the account of Mongillo, Barak and Tsodyks (2008), $\tau_f$ well above $\tau_d$ keeps the kicked assembly's synapses strong between its reactivations and the unkicked assembly's weak, and this grid lacked both;
  - a slower current.

  That round also carries F-54: a context and the task in one network need another geometry or another size.

### Consequences

- Good: the substrate question is answered for this network as it is — its settled image, its rules and its drive — with the assembly the only difference, and the growth shown to change nothing unwired. The network does not hold a context, and the tables say why: a burst drains the pool; no state persists.
- Good: the kick is derived from the membrane rule and held to the engine. It was read before any cell, and derived again when the measure failed by one spike. The release is ADR-0076's cancel as built. Neither is searched, and no threshold moved.
- Good: the next decision has a measured need, and ADR-0111's two later rounds are not built on a substrate that does not hold.
- Bad: no cell is usable, so the representation of the rule in force waits on a rule change, and ADR-0111's second and third rounds wait with it.
- Bad: no assembly can be disjoint from the task's sets on this network (F-54).
- Neutral: one seed of the delays, one placement and one drive, on the settled network and not the drained one; the grid reads these cells and no others.

## Confirmation

`runtime/cortex-runtime/tests/assembly.rs`: `SIZES`, `WEIGHTS`, `FAN_MAX`, `PLACES`, `DELAY_SEED`, `ORDER`, `ROUNDS`, `KICK_MESSAGE_Q16`, `KICK_TICKS`, `KICK_SPAN`, `KICK_STANDINGS`, `RELEASE_AT`, `HOLD_TIMES`, `LET_GO_TIMES`, `SPILL_TIMES`, `OF_EIGHT_MIN`, `IGNITIONS_MAX`; `grown`, `wire`, `protocol`, `background_of`, `cell`, `kick_reading`, `kicked_once`, `kick_rule`, `reset_rule`, `kick_oracle`, `release_oracle`, `steady`, `delivered`, `recurrent`, `inject_before`, `note_kick_spikes`, `bursts`; the tables `BACKGROUND_ROWS_1024`, `CONTROL_ROWS_1024`, `CELL_ROWS_1024`, `BACKGROUNDS_1024`, `KICKS_1024`, `KICKED_ONCE_1024`, `CONTROLS_1024`, `GRID_1024`, `BURSTS_1024`, `GATE_KICKED`; the gate `the_grid_the_arithmetic_the_rules_and_an_assembly_kicked_on_the_engine`; the weekly `the_kick_and_the_background_at_1024_units_exhaustive` and `an_assembly_of_16_units_at_four_weights_exhaustive`, `…_32_…` and `…_64_…`.
