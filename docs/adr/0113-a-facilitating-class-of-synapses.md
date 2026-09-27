---
status: accepted
date: 2026-09-27
depends-on: ADR-0112
decision-makers: VirtualCortex maintainers
---

# ADR-0113: A facilitating class of synapses — ADR-0112's branch for nothing usable taken, with the facilitating route chosen over a slower current: a unit may be marked facilitating, its synapses releasing under short-term plasticity whose facilitation outlasts its depression, the class's constants a parameter of the image; unset, every run is the run it was, bit for bit; brief 049 builds it and measures ADR-0112's assemblies again with their members marked, over spans long enough to tell a refreshed hold from a fading one

## Context and Problem Statement

[ADR-0112](0112-an-assembly-that-holds.md) measured ADR-0111's first round and read **no assembly usable**. On ADR-0077's settled image with every weight frozen, assemblies of 16, 32 and 64 excitatory units were wired at four recurrent weights, and none held its activity after a kick, stayed quiet without one and let go on a signal.

What the tables showed:
- **A kick sets off one population burst.** Each member fires three or four times at the refractory limit, about every 220 ticks.
- **The burst drains the vesicle pool.** At the burst windows' ends the members' pool $R$ stands at 9 to 27 of 255, and the assembly is then silent until it recovers.
- **The strong cells burst again without a kick**, 20 to 32 times over the twenty-four epochs at three quarters and the top weight of 32 and 64 units. Those bursts are the ignitions, and after a release they are the failures to let go.
- **Between bursts it is the pool that is low.** At the top weight the members' release fraction $u$ averages 0.61 to 0.72 and their pool 0.13 to 0.24 of its rest.

ADR-0111 named the branch for this reading: an ADR on a mechanism of persistence, with two candidates — synapses whose facilitation outlasts their depression for the context's units, or a slower current. **The maintainers chose the first on 2026-09-27.**

### Why the present rule cannot hold

The synaptic theory of working memory (Mongillo, Barak and Tsodyks 2008) holds an item in the release fraction $u$ of the item's synapses. Its constants are $U = 0.2$, $\tau_F = 1.5$ s and $\tau_D = 0.2$ s, so facilitation outlasts depression. After a population spike the pool recovers within a few hundred milliseconds while $u$ is still high. For as long as $u$ stays high the assembly's synapses are stronger than an unkicked assembly's, so the drive's fluctuations set off the next population spike in the kicked assembly and not in the others, and each spike refreshes $u$. Facilitating synapses between pyramidal cells are common in the prefrontal cortex (Wang et al. 2006). Periodic population spikes are what a network of depressing synapses produces (Tsodyks, Uziel and Markram 2000), and ADR-0112 read them.

`crates/cortex-core/src/dynamics/plasticity.rs` runs the same Tsodyks–Markram rule with the order reversed: $U = 51/256$, $\tau_f = 2^{14}$ ticks (164 ms) and $\tau_d = 2^{15}$ (328 ms). [ADR-0019](0019-short-term-plasticity.md) chose one set of constants for the whole engine and deferred any other: *"Per-type constants are a record question under ADR-0016's test"*.

The arithmetic below is an integer replica of `step_stp` in Python, written for this ADR (`relax_q0_8`, `stp_decay_factor_q16` and the spike's update line for line, parameterised by $U$ and the two shifts). At ADR-0019's constants it reproduces every row of ADR-0112's steady table pair for pair, from $(93, 237)$ at 1.76 Hz to $(242, 2)$ at 400 Hz. It is not the engine's. Brief 049 computes every number again with the class's own step, and the build is held to it.

**After a burst, the present rule leaves an assembly weaker than an unkicked one for about four epochs, and never primed.** The burst is four spikes 220 ticks apart, from the unkicked steady state at 1.76 Hz. The table gives the pair a member's next spike would release with $t$ ticks later, and its product $uR$ against the unkicked steady state's:

| $t$ (ticks) | 220 | 2 048 | 8 192 | 16 384 | 32 768 | 65 536 | 131 072 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| ADR-0019's constants: pair | (186, 13) | (176, 26) | (149, 65) | (127, 107) | (105, 166) | (93, 222) | (92, 251) |
| $uR$ of the unkicked's | 0.11 | 0.21 | 0.44 | 0.62 | 0.79 | 0.94 | 1.05 |

By the time the pool has recovered, $u$ has relaxed back to its rest. The product returns to the unkicked one after about 84 000 ticks and never passes it by more than 6.4 per cent, which is the rest's pair against a pool the background keeps slightly drawn. So a kicked assembly holds no advantage over an unkicked one. That is ADR-0112's reading in one line: the strong cells ignite whether kicked or not, and the weak ones never hold.

### What the rule can express

`stp_decay_factor_q16` reads a time constant above $2^{16}$ ticks as $2^{16}$, the longest a Q16.16 base resolves ([ADR-0028](0028-edge-behaviour-audit.md)). A $\tau_f$ of 1.5 s, about $2^{17.2}$ ticks, is therefore not expressible without changing that function. The nearest the rule expresses is $\tau_f = 2^{16}$ ticks (655 ms), with $\tau_d = 2^{13}$ (82 ms). Their ratio is 8 against Mongillo's 7.5, and the timescale is 2.3 times shorter. $U$ stays a free byte in Q0.8.

## Decision Drivers

- **A measured need**: ADR-0112's reading that a kick leaves an assembly silent, not primed, and the arithmetic above that says why.
- **The maintainers' choice** of the facilitating route over a slower current (2026-09-27).
- **ADR-0016's test and ADR-0019's deferral.** Which short-term plasticity a unit's synapses follow belongs to the unit's record, which already holds its short-term state (`stp_u_rel`, `stp_r_ves`) and its polarity (a bit of `flags`). It is a field of an existing record, not a crate.
- **Unset, bit for bit**, as [ADR-0094](0094-the-signed-gate-built.md) built the signed gate: the rule beside `step_stp`, which is not touched, so that every earlier run is the run it was.
- **One change at a time.** Only the synapses change. The membrane, the kick, the release's shape, the drive and the placement stay ADR-0112's, so that the reading is set against that one's.
- **Latest ≠ Newest**: a published rule (Tsodyks and Markram 1997; Mongillo, Barak and Tsodyks 2008) at other constants. No dependency and no tool.

## Considered Options

1. **Where the class lives**:
   - (a) a bit of the unit's `flags` marks it facilitating, and the class's constants are a parameter of the image;
   - (b) the class's constants as `const` items of `cortex-core` beside ADR-0019's;
   - (c) a short-term state per synapse, in `SynapseBlock`.
2. **The constants measured**:
   - (i) $U = 51/256$, $\tau_f = 2^{16}$, $\tau_d = 2^{13}$: Mongillo's, at the longest $\tau_f$ the factor resolves;
   - (ii) $U = 26/256$ at the same time constants;
   - (iii) $\tau_d = 2^{14}$, a ratio of four;
   - (iv) $\tau_f$ beyond $2^{16}$, which changes `stp_decay_factor_q16` and ADR-0028's bound.
3. **The rounds**: the class built and measured in one round, as brief 041 built the signed gate and ran H-18; or two rounds.
4. **The measure of a hold**: ADR-0112's, one epoch's last half; or spans of epochs long enough that a hold still read at the end cannot be what a kick left fading.

## Decision Outcome

**Options 1(a), 2(i) and (ii) side by side, 3 in one round, and 4 in spans.**

### The class (Specified; brief 049 builds it)

- **The mark.** A bit of `DendriticSuperNeuron::flags`, beside `FLAG_BURST_MODE` and `FLAG_INHIBITORY`. `integrate` sets and clears its own bit by mask and leaves the others. A marked unit steps its short-term state under the class's constants; every other unit steps under ADR-0019's, bit for bit.
- **The constants.** The class's $U$ in Q0.8 and its two time constants as shifts, a parameter of the image (whitepaper §8.3), each refused outside what the rule resolves: a shift from 1 to 16, and a $U$ from 1 to 255.
  - Unset, no class exists, and the loader refuses a marked unit.
  - Set with no unit marked, nothing reads it.
- **The rule.** The class's step sits beside `step_stp`, which is not touched. It is Tsodyks–Markram over the class's constants, and at ADR-0019's constants it is `step_stp` bit for bit, which a property test holds over the lattice of `testkit/prop.rs`. The executor calls it for a marked unit where it calls `step_stp` now: once per spike, no word, no allocation and no syscall added to the tick.
- **The image.** `FORMAT_VERSION` moves from 16 to 17. A pin of a whole image moves with the format number and nothing else, restated as [ADR-0095](0095-an-image-pin-moves-with-its-format.md) restated H-17's.
- **Every other state is the image's.** Nothing of ADR-0077's settled image, the prior or any earlier test changes. An image with no unit marked runs as it did.

### The arithmetic of the two sets (the replica's; brief 049 reads it again)

**Steady pairs at a sustained rate.** The last column is $uR$ against ADR-0019's steady state at the background's 1.76 Hz (22 041):

| Rate | (i) $U$ 0.2: pair | $uR$ | of (i)'s rest | vs ADR-0019 at 1.76 Hz | (ii) $U$ 0.1: pair | $uR$ | of (ii)'s rest | vs ADR-0019 at 1.76 Hz |
| :--- | :--- | ---: | ---: | ---: | :--- | ---: | ---: | ---: |
| at rest | (92, 255) | 23 460 | 1 | 1.064 | (49, 255) | 12 495 | 1 | 0.567 |
| 1.76 Hz | (113, 255) | 28 815 | 1.228 | 1.307 | (63, 255) | 16 065 | 1.286 | 0.729 |
| 5 Hz | (149, 242) | 36 058 | 1.537 | 1.636 | (93, 247) | 22 971 | 1.838 | 1.042 |
| 10 Hz | (182, 197) | 35 854 | 1.528 | 1.627 | (128, 211) | 27 008 | 2.162 | 1.225 |
| 20 Hz | (208, 130) | 27 040 | 1.153 | 1.227 | (163, 146) | 23 798 | 1.905 | 1.080 |
| 40 Hz | (226, 74) | 16 724 | 0.713 | 0.759 | (199, 81) | 16 119 | 1.290 | 0.731 |
| 100 Hz | (242, 31) | 7 502 | 0.320 | 0.340 | (225, 33) | 7 425 | 0.594 | 0.337 |
| 400 Hz | (250, 8) | 2 000 | 0.085 | 0.091 | (243, 8) | 1 944 | 0.156 | 0.088 |

At a sustained 20 Hz, set (i) releases 1.153 of its efficacy at rest, where ADR-0019's releases 0.362. A sustained rate leaves set (i) above its efficacy at rest up to about 25 Hz, and set (ii) up to about 55 Hz; above those rates each depresses.

**After a burst** (four spikes 220 ticks apart, from each set's unkicked steady state at 1.76 Hz):

| $t$ (ticks) | 220 | 2 048 | 8 192 | 16 384 | 32 768 | 65 536 | 131 072 | 262 144 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| (i): $uR$ of the unkicked's | 0.11 | 0.43 | 1.05 | 1.33 | 1.35 | 1.15 | 0.93 | 0.82 |
| (ii): $uR$ of the unkicked's | 0.43 | 0.75 | 1.33 | 1.55 | 1.52 | 1.24 | 0.95 | 0.81 |

**Under either set, a kicked assembly is primed.** From about 7 400 to 106 000 ticks after its burst, set (i)'s product is above the unkicked one, at most 1.38 times it at about 24 000 ticks. Set (ii)'s is above from about 4 100 to 117 500 ticks, at most 1.60 times. That window is what Mongillo's account needs. It is also narrow: a hold needs a weight at which 1.4 to 1.6 times the unkicked product sets off a burst on the drive's fluctuations and the unkicked product does not. The weights of brief 049's grid are therefore finer than ADR-0112's.

Without a refresh, the priming has faded below the unkicked product by 131 072 ticks, eight epochs of $2^{14}$. So a hold read in the ninth to the sixteenth epoch after a kick is not what the kick left fading. That is why the measure is taken over spans (option 4). A release, in turn, has to keep the members from firing for about as long.

These are the replica's numbers, from a burst with no background spike after it, and they make no prediction of the region. Brief 049 computes them with the class's step, and the build is held to them.

### The round (brief 049)

- **The build**: the mark, the constants, the rule and the image as above, with tests at their edges and over the lattice, and every pinned number of the tree unchanged when nothing is marked.
- **The measurement**, on ADR-0112's substrate with one difference: the members marked. Same settled image, placement, wiring, delays and kick.
  - A grid of sizes (16, 32 and 64), weights (at least 0.25, 0.375, 0.5, 0.625, 0.75 and 1.0 of Q1.15's range) and the two sets.
  - Rounds of an unkicked span, a kicked hold span, a release and a tail, each span a number of epochs written before any run.
  - Holds, ignites, lets go and spills, by rules written before any run, and a release derived from the class's arithmetic.

## Consequences

- Good: the reading ADR-0112 left is answered at its cause. The kicked assembly gains an advantage over the unkicked one that lasts longer than its pool takes to recover, which the present constants cannot give.
- Good: unset, nothing changes. The class is read only by a marked unit, and the rule beside `step_stp` is `step_stp` at ADR-0019's constants.
- Good: heterogeneity of short-term plasticity, which ADR-0019 deferred to a record, gets its record: one bit and one parameter.
- Bad: a format bump for one bit and a few bytes, and every whole-image pin restated.
- Bad: the timescale the factor resolves is 2.3 times shorter than Mongillo's. A context held this way has to be refreshed within the primed window, between about a tenth of a second and a second after its last burst, and the task's trial of 164 ms has to live with that.
- Bad: the primed window's advantage is 1.4 to 1.6 times, so a usable region, if one exists, may be a narrow band of weights.
- Neutral: a hold under this class is a train of refreshing bursts, not a steady rate. A readout gated by it (ADR-0111's second round) has to read that.

## Alternatives considered and why rejected

- **Option 1(b), constants in `cortex-core`**: a run's behaviour would change with the build rather than with the image (§8.3), and a grid over the constants would need a build per set.
- **Option 1(c), state per synapse**: the short-term state is the presynaptic unit's (ADR-0019), and the class is a property of how the unit releases. A `SynapseBlock` change would move every block of every image for no reading that needs it.
- **Option 2(iii), a ratio of four**: its primed window is shallower (at most 1.23 times the unkicked product for $U = 0.2$, by the replica) and later. It is kept as a reading the round may add, not a set of the grid.
- **Option 2(iv), a longer $\tau_f$**: this changes a function every short-term step reads, and ADR-0028's bound. It is named as the next step if the two sets read a priming that fades too soon.
- **Two rounds**: the build has no reading of its own to give until the measurement runs, and brief 041 did both in one.
- **ADR-0112's one-epoch measure**: under this class a kicked assembly is primed for up to about eight epochs whether or not it refreshes. A hold read within one epoch could not tell the two apart.
- **The slower current**: the maintainers' other candidate, not taken. It stays named as the next decision should this one read nothing usable.

## Confirmation

`briefs/049_a-facilitating-class.md` builds the class and measures the grid. Whitepaper 4.64.0 carries this decision in §11.1's question on a rule held by the network, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
