---
status: accepted
date: 2026-09-14
decision-makers: VirtualCortex maintainers
depends-on: ADR-0047
---

# ADR-0051: The estimator at 4 096 units — the measurement of ADR-0047 on both priors at the third size as the weekly job's `exhaustive` test, the decision rule for a record line written before the run, and its outcome: no line, since the fine-bin slope reads above the gross causal ratio where the population nears the ceiling

## Context and Problem Statement

[ADR-0047](0047-second-prior-and-the-estimator.md) measured the lag-one estimator of [ADR-0036](0036-criticality-control.md) on two priors at 256 and 1 024 units and added no line to the record, with one thing left undecided: "the size at which the fine slope reads the gross ratio is not yet run", and whitepaper §11.1: "at 1 024 units within 0.15 of the gross ratio, so the size that would justify a line is the next one, 4 096, run before any line is added". Brief 024 asked for that run with the decision rule written before it (its Context, item 4): a line of the record is justified when, at 4 096 units on both priors at every gain below the ceiling, the lag-one slope at the fine bin is within 0.15 of the gross causal ratio the oracle attributes, and the coarse estimate's two windows differ from each other by more than 0.15; otherwise no line, the reading recorded as the third size, and the causal count inside the loop the Specified alternative.

The run is the harness of ADR-0047 at 4 096 units: the lattice of [ADR-0044](0044-reference-network.md) and the sparse random network, two workers, the drive of ADR-0044 (32 messages per tick at this size), the modulation baseline at 0, three fixed gains with two windows each, thirty-two kicks per gain attributed through the connectome, and the closed loop from a gain of 1.0 under a step of an eighth for twelve windows. Its train is the executor's own ([ADR-0050](0050-the-train-inside-the-executor.md)), not a worker's report: the report's capacity is shared with the delivered-message trace, which the drive alone fills at this size.

## Decision Drivers

- ADR-0010 and principle 3: the numbers are the engine's on a stated network at a stated size; the weekly job runs them, the gate does not.
- The rule was written before the run and is applied as written; what the numbers say beyond it is recorded as a reading, not folded into the rule after the fact.
- A line of the record is a format bump and a controller that acts on the line; it is taken only where the line would read what the controller regulates.

## Considered Options

1. The line: the sums for lags 1 to $K$ at a bin of $2^8$ ticks in a second line of `HomeostaticDrivePool`, format 14, `regulate` reading the fine lag-one slope in place of the coarse estimate.
2. **No line: the reading recorded beside the two smaller sizes; the coarse estimate stays the record's; the ceiling stays the actuator's bound; the causal count inside the loop stays Specified.**
3. The line taken on the strength of five rows of six.

## Decision Outcome

Option 2, by the rule as written.

- **What the two priors read at 4 096 units** (`tests/reference.rs`, `the_estimate_the_causal_ratios_and_the_loop_at_4096_units_exhaustive`; Q16.16 in the test, decimals here; the ceiling is 131 072 spikes per window):

  | Prior | Units | Gain | Estimate, windows 1, 2 | Spikes per window | Gross $\sigma$ | Net $\sigma$ | Fine slopes $r_1, r_2, r_4, r_8, r_{16}$ |
  | :--- | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
  | lattice | 4 096 | 1.75 | 0.000, 0.000 | 9 851, 9 207 | 0.515 | 0.455 | 0.368, 0.203, 0.027, 0.095, 0 |
  | lattice | 4 096 | 2.00 | 0.745, 0.000 | 51 087, 45 854 | 0.887 | 0.557 | 0.851, 0.779, 0.644, 0.507, 0.289 |
  | lattice | 4 096 | 2.25 | 0.704, 0.000 | 112 272, 102 769 | 0.612 | 0.272 | 0.887, 0.787, 0.566, 0.501, 0.346 |
  | random | 4 096 | 1.75 | 0.000, 0.086 | 7 562, 7 516 | 0.177 | 0.161 | 0.118, 0.098, 0, 0.002, 0.019 |
  | random | 4 096 | 2.00 | 0.732, 0.000 | 42 321, 38 544 | 0.675 | 0.390 | 0.788, 0.717, 0.565, 0.310, 0.096 |
  | random | 4 096 | 2.25 | 0.120, 0.000 | 105 042, 95 996 | 0.856 | 0.365 | 0.906, 0.744, 0.331, 0, 0.022 |

  The attributions behind the ratios: on the lattice 66, 97 and 103 ancestor spikes with 34, 86 and 63 first-generation descendants (4, 32 and 35 of them advanced baseline spikes); on the random network 62, 77 and 104 ancestors with 11, 52 and 89 descendants (1, 22 and 51 advanced); two kicks at 2.25 on the lattice and one at 2.0 and three at 2.25 on the random network fired no ancestor. The two forks drift apart far beyond the first generation at the higher gains (2 341 extra and 2 427 missing spikes at 2.0 on the lattice; 25 702 and 25 336 at 2.25 on the random network). The closed loop from 1.0 climbs as at the smaller sizes, its estimate zero for the first four or five windows (0, 0, 19, 215 and 1 958 spikes per window on the lattice), reads 0.35 to 0.56 once the population fires, and crosses the ceiling once on the lattice (the eleventh window, 163 418 spikes at a gain of 2.15) and twice on the random network (the tenth and the twelfth, 171 658 and 144 274 spikes), where the gain turns.

- **The rule, applied.** The first condition holds in five rows of six: the fine lag-one slope is within 0.15 of the gross ratio on the lattice at 1.75 (0.368 against 0.515, the margin's edge) and 2.0 (0.851 against 0.887), and on the random network at every gain (0.118 against 0.177, 0.788 against 0.675, 0.906 against 0.856). It fails on the lattice at 2.25, where the slope reads 0.887 and the oracle 0.612, a difference of 0.28. The second condition holds in three rows of six (the coarse windows differ by 0.745, 0.704 and 0.732 where one window reads the gross ratio's order and the next zero) and fails in three (0.000 and 0.000 on the lattice at 1.75; 0.000 and 0.086, 0.120 and 0.000 on the random network). So the rule does not hold and no line is added.

- **What the reading says beyond the rule.** *The coarse estimate reads nothing at this size either*: zero in seven windows of twelve, both windows zero where the oracle attributes 0.515, and its non-zero readings are one window's, never repeated by the next. The rule's second condition, written to tell noise from a reading, cannot tell two windows agreeing on a wrong reading from two agreeing on a right one; the first condition decided. *The fine slope reads the gross ratio better at every size above 256 and reads above it where the population nears the ceiling*: at 2.25 on the lattice the window holds 112 272 spikes against a ceiling of 131 072, 86 per cent of one spike per unit per bin, and the slope at lag one is 0.887 with slow decay (0.787, 0.566, 0.501, 0.346 at lags 2 to 16) where the oracle's first generation is 0.612 and its net ratio 0.272; the autocorrelation of a population near saturation is the drive's and the refractory rhythm's as much as the propagation's, and a controller reading it would read supercritical earlier than the oracle does. On the random network at the same gain the slope reads 0.906 against 0.856. *Across sizes*, the lattice's fine slope at 2.0 reads 0.569 at 256 units, 0.640 at 1 024 and 0.851 at 4 096 while the gross ratio reads 0.455, 0.638 and 0.887, and the random network's reads 0.676, 0.549, 0.788 against 0.083, 0.578, 0.675: the two converge with the size at moderate density and part again near the ceiling.

- **What is decided.** No line. The record stays as ADR-0047 left it; the ceiling stays the actuator's bound at every size the tree runs; a controller that reads the branching ratio below the ceiling needs the causal count inside the loop (an ancestor stamp carried per delivery), which stays Specified, and not a finer regression over the population count. The 4 096-unit sweep is the weekly job's third `exhaustive` size and its numbers are pinned as the engine's.

- **Not adopted.** *Option 1*: the rule failed, and the row it failed on is the one a controller must read right (the approach to the ceiling). *Option 3*: a rule applied after the run is not a rule. *A night or a capture at 4 096 units*: the estimator's question is the one this size answers; H-9's transfer stays open with its protocol. *A change to the drive's density at this size* (32 messages per tick fill a window with 9 000 spikes at 1.75 and 112 000 at 2.25): the drive is ADR-0044's function of the tick and the size, and a change to it is a change to every row of every size.

### Consequences

- Good: the open item of ADR-0047 is closed by a measurement at the size it named, with the rule stated before the numbers and the numbers stated whether or not they flatter the rule.
- Good: the weekly job now covers three sizes, so a change to the estimator, the controller or the priors is checked at 4 096 units every week.
- Good: the exhaustive test at this size runs on the executor's own train, whose capacity is the spikes' alone.
- Bad: one row decided; had the lattice at 2.25 read 0.15 closer, the line would have been Specified on a five-of-six reading whose sixth row is the one that matters. The reading beyond the rule says why the sixth row matters, and the next controller round starts from the causal count.
- Bad: the sweep is one run per size and prior; its variance across seeds is not measured (the seed is the prior's parameter, and a second seed is a second sweep).

## Alternatives considered and why rejected

- **The threshold widened to 0.3**: the rule would have held on the lattice at 2.25 by 0.02 and the line would carry the near-ceiling excess into the controller; the threshold was the brief's and stands.
- **The fine slope at a bin of $2^9$ ticks**: a wider bin reads a later lag of the same autocorrelation; ADR-0047's lags 2 and 4 are that reading (0.787, 0.566 on the lattice at 2.25) and it decays toward, not onto, the gross ratio.

## Confirmation

`tests/reference.rs`: `the_estimate_the_causal_ratios_and_the_loop_at_4096_units_exhaustive` pins every number of the table, the attributions, the fine and coarse slopes and the twelve-window loops on both priors; the weekly job runs it with the 256- and 1 024-unit forms (`cargo test --workspace --release --locked -- --ignored exhaustive`). The run's train is the executor's, cross-checked against the workers' reports by the determinism test of the same file (ADR-0050).
