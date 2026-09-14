---
status: accepted
date: 2026-09-14
depends-on: ADR-0053
decision-makers: VirtualCortex maintainers
---

# ADR-0055: A weight that settles — the excitatory depression scales with the weight's magnitude, so that under stationary pairing a magnitude has a fixed point where its depression equals its potentiation instead of draining to nothing; the potentiation additive as it is; measured over the day at 256 and 1 024 units under a criterion written before the run; §8.8's per-unit gain stays Specified, since a gain cannot lift a weight from zero

## Context and Problem Statement

The day of [ADR-0053](0053-the-waking-day-and-the-target-period.md) read the reference network's excitatory arena draining under a stationary drive: at 1 024 units under the controller the excitatory sum fell from 225.9 million to nothing within fifteen windows (twenty seconds of simulated time), at 256 units with the gain held by seventeen per cent in eight, and nothing in the tree held a weight up but a night's replay of a tagged pattern. Whitepaper §11.1 left the item: "a stabilising rule (this row's, or a rate-dependent balance of $A_-$ against $A_+$) is the next controller round's subject", the row being §8.8's per-unit synaptic scaling, a gain per unit at `[52..54)` of the unit record, Specified by [ADR-0036](0036-criticality-control.md).

Why the arena drains is the pair rule's geometry, not only its amplitudes. The nearest-neighbour rule of [ADR-0022](0022-synapse-fan-out-and-stdp.md) depresses at a presynaptic spike $t$ by $A_- f(t - q)$ against the target's last spike $q$, which is always on record once the target has fired, and potentiates by $A_+ f(q - p)$ only when that spike fell after the previous presynaptic spike $p$, decaying with $q - p$. Under stationary firing with no correlation between a synapse's two units the depression's expected window factor exceeds the potentiation's at every rate the population reaches, so an additive rule drifts down until the floor whatever $A_- / A_+$ is; the 1.05 of ADR-0022 only sets how fast. A gain per unit applied to the inputs (§8.8's row) multiplies what arrives; a weight at zero times any gain is zero, and a rate homeostat on the inputs changes nothing in the pairing statistics that drain the weights, so the row's rule does not answer the finding.

Brief 026 asked for the rule that does, its measurement on the same day harness under a criterion written first, and a decision on the row.

## Decision Drivers

- A weight must have a fixed point under stationary pairing, positive and stable, whatever the rate: the property the additive rule lacks and the day exposed.
- The nights of [ADR-0044](0044-reference-network.md) and [ADR-0048](0048-episodes-tagged-from-the-train.md) pin "every synapse among a tagged pattern at the rail after a night": the potentiation must still reach the rail.
- Latest ≠ Newest (§2.1): the form must be one with a stable specification, third-party production use and documented failure modes.
- Every amount stays in Q1.15 with `saturating` arithmetic by name; the depression fits the width beside a potentiation at every magnitude.
- The reference prior's excitatory band ($[6\,000, 12\,000]$, 0.18 to 0.37 of the width): the rule at the prior's weights should be near the rule as it was, so that the reference tests' regime does not change by fiat.
- ADR-0010 and principle 3: the criterion is written before the run and applied as written; no constant is tuned after the run it was measured in.

## Considered Options

1. **A depression proportional to the magnitude, the potentiation additive** (van Rossum, Bi and Turrigiano 2000; the $\mu = 1$ depression of Gütig, Aharonov, Rotter and Sompolinsky 2003; reviewed by Morrison, Diesmann and Gerstner 2008): $\Delta e = -A_- f(t - q) \cdot M / M_{\text{ref}}$ with $M$ the slot's magnitude before the pairing and $M_{\text{ref}}$ a reference magnitude, so that a magnitude settles at $M^* = M_{\text{ref}} (A_+ / A_-)(f_+ / f_-)$.
2. The per-unit gain of §8.8's row, at `[52..54)` of the unit, applied by the turn beside the population gain.
3. A rate-dependent balance of $A_-$ against $A_+$: the depression scaled by the target's rate over a target rate.
4. The power law on both sides ($\mu = 1$ on the potentiation too): $M^* = M_{\text{max}} A_+ / (A_+ + A_-)$.
5. A retuned $A_-$ (or $A_+$) with the additive form.

## Decision Outcome

Option 1, with $M_{\text{ref}} = 2^{13}$ (`STDP_DEPRESSION_REFERENCE_Q1_15`, 0x2000, a quarter of the width, inside the reference prior's band).

- **The rule** (`cortex-core`). In `step_stdp`, an excitatory block's depression is $\operatorname{round}(A_- (1 - 2^{-11})^{t - q} \cdot M / 2^{13})$, $M = $ `Polarity::magnitude` of the slot's weight before the pairing (`depression_at`, rounded to nearest, the product below $2^{24}$ in `i64`): 344 at the reference magnitude, 1 376 at the rail, four for a magnitude of 100, one LSB at twelve and nothing at eleven or below, so a weight the rule depresses cannot reach zero from above. The potentiation, the inhibitory rule of [ADR-0049](0049-dale-principle-in-plasticity.md), the decay, the consolidation of [ADR-0032](0032-three-factor-plasticity.md) and every signature are unchanged; `STDP_A_MINUS_Q1_15` keeps its value as the depression at the reference magnitude, and a `const` assertion holds that the largest depression, four windows at the rail, fits the width beside a potentiation. The lattice property bounds a pairing's move by $A_+ + 4 A_-$ and holds the depression monotone in the magnitude.
- **The fixed point.** Under a repeated pairing at fixed intervals ($q - p$ and $t - q$) the magnitude is fixed where $\operatorname{round}(A_- f_- M / 2^{13}) = A_+ f_+$: an oracle band, since the rounding makes a plateau. At $q - p = 500$ and $t - q = 2\,000$ ticks ($f_+ = 257$, $f_- = 129$) the band is $[16\,289, 16\,352]$ around $2^{13} \cdot 257 / 129 = 16\,322$; the unit test walks into it from 1 000 (stopping at 16 289) and from 30 000 (at 16 352) and stays. Under a population's firing the two window factors are expectations over its intervals, and the fixed point is where the day's sum settles.
- **The day's reading** (`tests/reference.rs::day`, the lattice, sixteen windows at 256 units with the gain held at 2.0 and eighty at 1 024 under the controller's step of an eighth at shift 5, each at the two periods of ADR-0053; the sums over the arena):

  | Prior | Period (ticks) | Window | Rate (Hz) | Gain | Stage | Inhibitory sum | Excitatory sum | At target |
  | :--- | ---: | ---: | ---: | ---: | :--- | ---: | ---: | ---: |
  | lattice 256 | 20 000 | 1 | 9.2 | 2.00 | awake | 53.2 M | 56.3 M | 0.59 |
  | lattice 256 | 20 000 | 4 | 7.4 | 2.00 | awake | 52.7 M | 51.9 M | 0.74 |
  | lattice 256 | 20 000 | 8 | 6.9 | 2.00 | awake | 52.0 M | 47.8 M | 0.81 |
  | lattice 256 | 20 000 | 12 | 6.9 | 2.00 | awake | 51.4 M | 44.8 M | 0.80 |
  | lattice 256 | 20 000 | 16 | 6.6 | 2.00 | awake | 50.6 M | 42.5 M | 0.85 |
  | lattice 256 | 5 000 | 1 | 9.1 | 2.00 | awake | 49.3 M | 56.3 M | 0.44 |
  | lattice 256 | 5 000 | 4 | 7.8 | 2.00 | awake | 37.5 M | 51.7 M | 0.31 |
  | lattice 256 | 5 000 | 8 | 7.8 | 2.00 | awake | 21.6 M | 46.9 M | 0.26 |
  | lattice 256 | 5 000 | 12 | 8.3 | 2.00 | awake | 6.8 M | 42.8 M | 0.34 |
  | lattice 256 | 5 000 | 16 | 8.3 | 2.00 | awake | 0.5 M | 39.4 M | 0.35 |
  | lattice 1024 | 20 000 | 1 | 9.2 | 2.09 | awake | 213.0 M | 224.7 M | 0.52 |
  | lattice 1024 | 20 000 | 4 | 26.5 | 2.17 | awake | 213.9 M | 153.8 M | 0.00 |
  | lattice 1024 | 20 000 | 8 | 18.5 | 2.67 | awake | 213.9 M | 117.2 M | 0.02 |
  | lattice 1024 | 20 000 | 16 | 11.7 | 2.49 | awake | 213.8 M | 107.7 M | 0.24 |
  | lattice 1024 | 20 000 | 32 | 19.5 | 2.69 | awake | 213.8 M | 106.8 M | 0.01 |
  | lattice 1024 | 20 000 | 48 | 34.3 | 2.37 | awake | 213.9 M | 106.4 M | 0.00 |
  | lattice 1024 | 20 000 | 64 | 16.5 | 2.63 | awake | 213.9 M | 106.5 M | 0.03 |
  | lattice 1024 | 20 000 | 66 | 15.1 | 2.59 | slow-wave | 213.9 M | 106.7 M | 0.06 |
  | lattice 1024 | 20 000 | 72 | 36.1 | 2.40 | slow-wave | 213.9 M | 106.6 M | 0.00 |
  | lattice 1024 | 20 000 | 80 | 21.1 | 2.75 | awake | 213.9 M | 106.7 M | 0.00 |
  | lattice 1024 | 5 000 | 1 | 9.2 | 2.07 | awake | 197.0 M | 224.6 M | 0.48 |
  | lattice 1024 | 5 000 | 4 | 27.3 | 2.14 | awake | 127.2 M | 150.8 M | 0.98 |
  | lattice 1024 | 5 000 | 8 | 21.2 | 2.63 | awake | 47.1 M | 114.9 M | 1.00 |
  | lattice 1024 | 5 000 | 16 | 36.1 | 2.22 | awake | 0.2 M | 108.8 M | 0.71 |
  | lattice 1024 | 5 000 | 32 | 28.7 | 2.12 | awake | 0.0 M | 107.6 M | 0.98 |
  | lattice 1024 | 5 000 | 48 | 13.1 | 2.40 | awake | 0.0 M | 108.3 M | 0.88 |
  | lattice 1024 | 5 000 | 64 | 8.9 | 2.32 | awake | 0.0 M | 107.7 M | 0.44 |
  | lattice 1024 | 5 000 | 66 | 29.4 | 2.12 | slow-wave | 0.0 M | 107.7 M | 0.97 |
  | lattice 1024 | 5 000 | 72 | 10.7 | 2.34 | slow-wave | 0.0 M | 107.9 M | 0.67 |
  | lattice 1024 | 5 000 | 80 | 22.0 | 2.44 | awake | 0.0 M | 107.4 M | 1.00 |

  The sums before the first window are the priors' (`the_priors_sums_before_any_window`): 58 968 247 excitatory and 53 475 744 inhibitory at 256 units, 235 822 619 and 213 902 976 at 1 024. Under the additive rule ADR-0053 read 56.7 to 47.0 million in eight windows at 256 units and 225.9 million to nothing in fifteen at 1 024.

- **The criterion, as written.** At 1 024 units under the controller the clause held at both periods: the excitatory sum after the eightieth window is 0.452 of the prior's at the 20 000-tick period and 0.456 at 5 000, and every one of the last sixteen windows is within 0.75 and 0.53 per cent of the sixty-fourth (the night at windows 66 to 72 inside them, its 320 replays carrying one cluster of 147 synapses to the rail, two per cent of the sum), where the additive rule drained the arena to nothing in fifteen windows. At 256 units with the gain held the clause held at the 20 000-tick period (the sum 0.721 of the prior's after the sixteenth window; the last four windows moving it by 1.37, 1.20, 1.15 and 1.28 per cent of the twelfth's) and failed at the 5 000-tick period by 0.27 points in one window (0.668 of the prior's; 2.27, 1.95, 1.86 and 1.93 per cent), so the criterion as written did not hold in full. The reading behind the clause: at 256 units the sum is still falling at the sixteenth window, decelerating from 2.6 per cent per window in the second to 1.2 at the slower period and 1.9 at the faster, where the inhibition is stripped and the pairings are denser; the mean magnitude fell from 9 000 to 6 500, and the depression at it from 378 to 272 of the window, which is the rule slowing the drift as it should, sixteen windows short of the fixed point that 1 024 units reach in about twenty under the controller's rates. Under the rule as written the pair rule stands with no constant moved, and §11.1's item stays open for the windows at which 256 units settle; the item's substance, a weight that does not drain, is answered at 1 024 units and read at 256.
- **The pins that moved.** Every reading of the reference tests whose synapses are depressed by the rule moved once, taken from the runs and stated as the engine's: the nights' sums before a night (the cluster's 147 synapses at 256 units from 1 281 024 to 1 271 353, the random pattern's five from 40 203 to 40 212; at 1 024 units the cluster's 143 and the two of the random pattern likewise), every night still ending with the cluster at the rail and the cue completing six of six; the capture's spikes (437 to 447 at 256 units) and its burst (62 to 58 spikes, two ticks later), its pattern's twelfth unit (163 to 155), the sums among the invention's and the network's own patterns; every window of the days at both sizes; and every window of the estimator's runs whose depressions the rule scales, with the same in-loop ratios to the second decimal. The determinism pin of [ADR-0030](0030-verification-governance.md) moved from `0x27e12eea1ee625a5` to `0x6c27858ece2dd412`, the spike count 95 as before: the random network's pairings depress at magnitudes that are not the reference, so the arena's weights and every unit's potentials differ from the additive rule's from the first depression on.
- **The row.** §8.8's per-unit gain stays Specified, with the reason above and its precondition of ADR-0036 (the measurement of H-8): it is a rate homeostat on a unit's inputs, which this decision does not need and which cannot answer a drain.

### Consequences

- Good: every excitatory weight has a fixed point under its pairing statistics; the drain of ADR-0053's day is answered in the rule that caused it, with one constant added and no amount changed; the potentiation is additive, so a night's replay still reaches the rail and every readout of the nights stood; the change is one line of the rule, a scaling of one term.
- Neutral: the reference tests' pins moved wherever the rule depresses; a number that moved is stated once here and stands in the tests. The rule's constants are unchanged, so at the reference magnitude the rule is ADR-0022's, above it stronger and below it weaker.
- Bad: the documented failure mode of the multiplicative depression (Gütig et al. 2003): the competition among a unit's inputs is weaker than the additive rule's, since one input's gain is not its rival's loss; the distribution of weights is unimodal around the fixed point rather than bimodal at the rails. Which regime a task needs is not decided here; $\mu$ between zero and one is the lever, and it stays Specified.

## Alternatives considered and why rejected

- **Option 2**, the per-unit gain: a gain multiplies what arrives and cannot lift a weight from zero; ADR-0036's precondition (a second per-unit controller after H-8's measurement) still stands. The bytes stay reserved.
- **Option 3**, a rate-dependent balance: a rate homeostat under an external drive that holds the population above its target drains the recurrent weights by design, which is what ADR-0053's day read at 1 024 units under the controller, and it needs a rate per unit the record does not hold.
- **Option 4**, $\mu = 1$ on both sides: the potentiation would approach the rail asymptotically (a replay of 320 ripples at $A_+$ reaches 96 per cent of it), moving a pinned property of the nights for no gain in stability, and the competition it costs is the failure mode Gütig et al. document for exactly that form.
- **Option 5**, a retuned amount: the additive form drains at any ratio under uncorrelated pairing (the geometry above); a smaller $A_-$ only slows it.

## Confirmation

- `crates/cortex-core/src/dynamics/synapse.rs`: the constants and the `const` assertions; the exact-step tests at the reference magnitude (the `MINUS` table), at the rail, at zero, at eleven and twelve, at a hundred; the fixed-point walk from both sides; the property tests over the lattice (the bound at the rail; the depression monotone in the magnitude).
- `runtime/cortex-runtime/tests/reference.rs`: the days at 256 and 1 024 units at both periods with every window pinned; the nights and the estimator's windows pinned again; `tests/differential.rs`: the pin restated with its reason.
- The mutation gate on the changed lines (ADR-0030).
