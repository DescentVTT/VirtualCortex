---
status: accepted
date: 2026-09-14
depends-on: ADR-0051
decision-makers: VirtualCortex maintainers
---

# ADR-0054: The causal count inside the loop — a message carries one bit that says it is a synapse's, a unit keeps the tick a synapse's message last reached it, a spike within the oracle's latency of that tick is a descendant and the executor tallies descendants beside spikes; measured against the oracle on both priors at 256, 1 024 and 4 096 units under a rule written before the run, the count reads the input's density and not the branching ratio, so the record and the controller are untouched and the count stays the executor's reading

## Context and Problem Statement

[ADR-0047](0047-second-prior-and-the-estimator.md) and [ADR-0051](0051-the-estimator-at-4096-units.md) left one alternative for a controller that reads the branching ratio below the ceiling: "the causal count inside the loop (an ancestor stamp carried per delivery), which stays Specified". The oracle of [ADR-0044](0044-reference-network.md) attributes by perturbation of forks: a spike in the kicked fork within `LATENCY` (128 ticks) of a kicked unit's message reaching its target, absent from the baseline fork, is a descendant, and one the baseline fires within `ADVANCE` after it is the same spike advanced (the net ratio subtracts these). Inside the loop there is one fork, so the counterfactual is not available; what is available is the oracle's first condition: whether a synapse's message reached the unit within the latency before its spike.

Brief 025 wrote the decision rule before the run: the in-loop ratio (descendants over spikes, per window) is the controller's reading when, at 256 and 1 024 units on both priors at every gain of `GAINS` below the ceiling, it is within 0.15 of the oracle's gross causal ratio of the same run and it rises with the gain on both priors at both sizes; then the record takes the two counts (where `energy_level`, written by no rule, and the unused upper half of `sum_prev` are) and `regulate` reads them below the ceiling; otherwise the count stays the executor's reading, the record keeps its layout and the controller is untouched.

## Decision Drivers

- ADR-0010 and principle 3: the count is measured on a stated network at stated sizes against the oracle of the same run before any controller reads it; the rule is applied as written.
- The message-bit budget: bits 0–17 are the efficacy, bit 18 the compartment, bits 19–31 were zero ([ADR-0022](0022-synapse-fan-out-and-stdp.md)); one bit says whose the message is, and the batch's order by value is preserved (a synapse's message sorts after a plain one of the same efficacy).
- The unit record's reserved word at `[16..24)` (the ABA tag until [ADR-0017](0017-mailbox-and-gate-protocol.md)) has room for the stamp; a field the loop writes moves the determinism pin, for a stated reason.
- A rule of `cortex-core` takes what it needs as an argument and reads the unit it is called on; the tally is the executor's, as the spikes' is (ADR-0035).

## Considered Options

1. **The latency stamp: `MESSAGE_SYNAPTIC` (bit 19) on every message the fan-out and the deliveries make; `last_synaptic_tick` at `[16..20)` of the unit, stamped by the turn holder from the batch it drained; a spike within `CAUSAL_LATENCY_TICKS` (128) of the stamp a descendant; the workers' counts summed by the tally into an executor counter, read per window by the harness.**
2. Fractional attribution: the synaptic and the injected efficacy accumulated since the unit's last spike, a spike credited to the synapses by their share.
3. The kind of the last input before the spike, synaptic or injected.
4. A generation stamp carried on every message, so that a spike inherits its cause's generation.

## Decision Outcome

Option 1 for the reading; no line of the record, the controller untouched; image format 14 (with ADR-0052 and ADR-0053); the determinism pin moved.

- **The bit and the stamp** (`cortex-core`). `synaptic_message(efficacy, apical)` is `spike_message` with `MESSAGE_SYNAPTIC`; `message_is_synaptic` reads it; the efficacy and the compartment read back unchanged, and bits 20–31 stay zero. `DendriticSuperNeuron::last_synaptic_tick` at `[16..20)` (`NO_SPIKE_ON_RECORD`, zero, for none: a delivery is integrated the tick after it, so no message reaches a turn at tick 0) and `[20..24)` reserved; `note_synaptic_input(now)` writes the stamp, `is_descendant(now)` is a stamp on record within `CAUSAL_LATENCY_TICKS` before now as a wrapping difference (§8.4). The image's at-rest check leaves the stamp as it is: a field the loop writes, like the last spike's tick.
- **The executor.** The fan-out's zero-delay delivery and the wheel's due token make synaptic messages; the injector's and the replay's stay plain. In the turn, a batch that holds a synaptic message stamps the unit before it integrates, and a spike that is a descendant counts on the worker; `phase_turns` publishes the count beside the spikes and the tally sums it into `Executor::descendants()`. The cost is one branch per drained message, one comparison per spike and one atomic per worker per tick. The determinism pin of [ADR-0030](0030-verification-governance.md) moves from `0x1724f3486c1d674e` to `0x27e12eea1ee625a5` for the stamp in the unit's bytes; the spike count stayed at 95 and the dynamics did not change (the message's value changed, the batch's sum did not).
- **What the count reads** (`tests/reference.rs`, the in-loop ratio per window beside the oracle's gross and net ratios; Q16.16 in the tests, decimals here):

  | Prior | Units | Gain | In-loop, windows 1, 2 | Gross $\sigma$ | Net $\sigma$ |
  | :--- | ---: | ---: | ---: | ---: | ---: |
  | lattice | 256 | 1.75 | 0.274, 0.249 | 0.389 | 0.389 |
  | lattice | 256 | 2.00 | 0.556, 0.476 | 0.300 | 0.200 |
  | lattice | 256 | 2.25 | 0.664, 0.617 | 0.583 | 0.306 |
  | lattice | 1 024 | 1.75 | 0.343, 0.314 | 0.529 | 0.441 |
  | lattice | 1 024 | 2.00 | 0.575, 0.504 | 0.638 | 0.340 |
  | lattice | 1 024 | 2.25 | 0.674, 0.638 | 1.000 | 0.475 |
  | lattice | 4 096 | 1.75 | 0.332, 0.296 | 0.515 | 0.455 |
  | lattice | 4 096 | 2.00 | 0.574, 0.513 | 0.887 | 0.557 |
  | lattice | 4 096 | 2.25 | 0.678, 0.631 | 0.612 | 0.272 |
  | random | 256 | 1.75 | 0.160, 0.098 | 0.188 | 0.125 |
  | random | 256 | 2.00 | 0.478, 0.411 | 0.095 | 0.048 |
  | random | 256 | 2.25 | 0.672, 0.627 | 0.464 | 0.107 |
  | random | 1 024 | 1.75 | 0.161, 0.151 | 0.206 | 0.118 |
  | random | 1 024 | 2.00 | 0.453, 0.433 | 0.578 | 0.222 |
  | random | 1 024 | 2.25 | 0.673, 0.644 | 0.710 | 0.194 |
  | random | 4 096 | 1.75 | 0.149, 0.148 | 0.177 | 0.161 |
  | random | 4 096 | 2.00 | 0.467, 0.415 | 0.675 | 0.390 |
  | random | 4 096 | 2.25 | 0.681, 0.636 | 0.856 | 0.365 |

  The gate's forms at 256 units read the same: the lattice at 2.0 over one window 0.556 against a gross ratio of 0.455 from four kicks, the random network at 2.0 0.478 and 0.411 against 0.083. The closed loop from 1.0 reads the ratio climbing with the population, 0.31 to 0.74 as the windows fill (0.556, 0.524, 0.663, 0.781 at 256 units from 2.0 as the spikes go from 3 156 to 11 846).

- **The rule, applied.** The first condition fails at 256 units on both priors (the lattice at 2.0 reads 0.256 above the gross ratio, the random network 0.38 above it at 2.0 and 0.21 at 2.25) and at 1 024 units on the lattice at 2.25 (0.674 against 1.000); it holds on the random network at 1 024 units at every gain (within 0.05, 0.13 and 0.04) and on the lattice at 1.75 at every size; at 4 096 units, a reading beyond the rule, the random network is within the margin at 1.75 and 0.21 and 0.18 below the gross ratio at 2.0 and 2.25, and the lattice 0.31 below it at 2.0. The second condition holds: the ratio rises with the gain on both priors at every size. So no line, no change to `regulate`, and the count stays the executor's reading.
- **What the reading says beyond the rule.** *The in-loop count is a function of the gain, not of the size*: 0.27, 0.56, 0.66 at 256 units, 0.34, 0.58, 0.67 at 1 024 and 0.33, 0.57, 0.68 at 4 096 on the lattice, while the gross ratio goes 0.39, 0.30, 0.58, then 0.53, 0.64, 1.00, then 0.52, 0.89, 0.61. What the count reads is the share of spikes that a synapse's message reached within the latency, which under a stationary drive is the density of synaptic input at that gain; what it cannot read is whether the drive would have fired the unit anyway, which is the oracle's counterfactual and the whole of the difference between the two where they part (0.48 against 0.10 on the random network at 256 units and a gain of 2.0: nearly every spike there has a synapse's message within 1.28 ms before it, and one in twenty is caused by it). That the random network at 1 024 units reads within the margin at every gain is the density and the causal ratio happening to rise together there, not the count reading the ratio: the same count on the lattice at 2.25 reads 0.67 where the oracle reads 1.00 at 1 024 units and 0.61 at 4 096. A controller reading it would read the population's density, which the ceiling already reads.
- **Not adopted.** *Options 2 and 3*: a share by efficacy or a last-input rule is a different heuristic with the same missing counterfactual, and the comparison could not have read either against the oracle's own criterion. *Option 4*: a generation per message needs bits the message has and a rule for a spike with several causes, and reads the depth of a cascade, not its width; not the quantity the controller wants. *The record's two fields and `regulate` reading the count*: the rule failed.

### Consequences

- Good: the question ADR-0051 left is answered by a measurement at three sizes on two priors under a rule stated first: the first-generation criterion without its counterfactual does not read $\sigma$ on a driven network, and the controller keeps the regression and the ceiling until a reading does.
- Good: the reading costs a bit, a stamp and a counter, is in every window of the exit tests and the day, and is the executor's on every worker count.
- Bad: the pin moved; a field the loop writes is in the unit's bytes, and every image written before this format reads a stamp of zero, which is none.
- Bad: bits 20–31 of a message are what is left of the budget; a generation stamp would take most of them.

## Alternatives considered and why rejected

- **The stamp in a side table of the executor** instead of the unit: a loaded engine would misattribute the spikes of its first latency after the load, and a reading not in the image is not the run's (§8.3).
- **The latency as the basal time constant (512 ticks)**: the oracle's criterion is 128, and the comparison must use the oracle's; a longer latency reads a higher share and the same absence of the counterfactual.

## Confirmation

`crates/cortex-core`: `is_descendant` at the latency, past it, before a message and across the wrap; the bit's encoding. `runtime/cortex-runtime/tests/modulation.rs`: a spike from a synapse's message is a descendant and an injected one is not, the stamp in the image. `tests/reference.rs`: the in-loop ratio per window beside the oracle's on both priors at 256 units in the gate and at 1 024 and 4 096 in the weekly job; `tests/differential.rs`: the pin. The mutation gate on the changed lines (ADR-0030) passes in CI.
