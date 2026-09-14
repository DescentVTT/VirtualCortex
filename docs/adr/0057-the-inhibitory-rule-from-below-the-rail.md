---
status: accepted
date: 2026-09-14
depends-on: ADR-0053
decision-makers: VirtualCortex maintainers
---

# ADR-0057: The inhibitory rule read from below the rail — the reference generator with its inhibitory gain at 2.0, a day at 256 and 1 024 units at the two periods with the fraction of units at the target as the reading a chosen target needs, under a rule written first; no target chosen; no lane for a parameter that changes behaviour, which `CLAUDE.md`'s invariant and ADR-0031 keep with the maintainers

## Context and Problem Statement

[ADR-0053](0053-the-waking-day-and-the-target-period.md) made the inhibitory rule's target period a parameter of the image and read a day on the reference prior, whose inhibitory weights start at the rail: from there the rule of Vogels et al. 2011 can only weaken inhibition, and the day could not say whether the rule drives a target unit toward the rate the period names. It closed with "a round that chooses one needs an objective over the rate, and if the engine is to amend it by itself, a registry gate of its own (option 4), which this ADR names and does not make"; whitepaper §11.1's registry item said the same. Brief 026 re-derived the second half against `CLAUDE.md`: "What it may amend by itself is a parameter in `cortex-executive`'s `REGISTRY`, through the four gates of `PolicyAmendment` and a trial in two forks of the image whose behaviour hashes must be equal" is an invariant of the switchboard, and a brief's empowerment never reaches it (`briefs/README.md`); §8.18 says the same in words: the engine may change what it costs, never what it does, and a change to behaviour is the maintainers' lane. So option 4 is not a brief's to open, and this round says so where the item stands. What a brief can give the maintainers is the rule read where it can move both ways, with an objective a chosen target would be read against.

The rule at an inhibitory synapse onto a target $j$ loses $\alpha$ at every presynaptic spike and gains $A_+ f(|\Delta t|)$ at each of the two pairings; its balance is at $\rho_j = \rho_0$ with $\alpha = 2 \rho_0 \tau A_+$, so above the target the inhibition onto $j$ grows and below it weakens. The same generator with `inhibitory_gain_q4_4` 32 (2.0) draws the inhibitory weights in $[-24\,000, -12\,000]$, a third to three quarters of the rail, with nothing else changed: room both ways.

## Decision Drivers

- ADR-0010 and principle 3: a reading rule written before the run and applied as written; no target chosen by a reading.
- `CLAUDE.md`'s invariant and [ADR-0031](0031-policy-amendment.md): no lane where the hashes differ.
- The objective must be per unit against the period, since the rule acts per target: a unit is at the target when its spikes in a window are within a factor of two of the period's count per window ($2^{17} / \text{period}$: six at 20 000 ticks, twenty-six at 5 000), inclusive both ways; the fraction of such units, read from the executor's own train ([ADR-0050](0050-the-train-inside-the-executor.md)).
- The same day harness, the same two periods and the same sizes as ADR-0053, with the excitatory rule of [ADR-0055](0055-a-weight-that-settles.md) in place, so that the two days are read side by side.

## Considered Options

1. **The generator at a lower inhibitory gain, the fraction at the target as the day's ninth reading, a reading rule written first, no target chosen.**
2. A second gate class of the registry with an objective the trial reads in both forks (ADR-0053's option 4).
3. A chosen target from the reading.
4. A new field of `Prior` for the inhibitory weight's band.

## Decision Outcome

Option 1.

- **The harness** (`tests/reference.rs`). `prior_below_rail(units)` is `prior(units)` with `inhibitory_gain_q4_4` 32; `day` returns per window the fraction of units at the target (`at_target_q16`, Q16.16) beside the eight readings of ADR-0053; a gate test on the below-rail prior at 256 units over sixteen windows with the gain held at 2.0, and an `exhaustive` test at 1 024 units over eighty windows under the controller's step of an eighth at shift 5, each at the two periods, every window pinned from the run. The days of ADR-0053 on the prior at the rail read the ninth reading too, at sixteen windows at 256 units.
- **The reading rule, as written.** The rule moves the population toward the target at a period when, at 256 units with the gain held over sixteen windows, the fraction at the target averaged over the last four windows exceeds the average over the first four by at least 0.1, and the inhibitory sum after the sixteenth window is neither at the rail (the inhibitory synapses' count times 32 767) nor below a tenth of the sum before the first.
- **What the days read** (the sums over the arena; the fraction at the target as a decimal):

  | Prior | Period (ticks) | Window | Rate (Hz) | Gain | Stage | Inhibitory sum | Excitatory sum | At target |
  | :--- | ---: | ---: | ---: | ---: | :--- | ---: | ---: | ---: |
  | below the rail 256 | 20 000 | 1 | 9.8 | 2.00 | awake | 30.1 M | 55.9 M | 0.48 |
  | below the rail 256 | 20 000 | 4 | 7.9 | 2.00 | awake | 30.5 M | 51.0 M | 0.70 |
  | below the rail 256 | 20 000 | 8 | 7.4 | 2.00 | awake | 30.7 M | 46.5 M | 0.78 |
  | below the rail 256 | 20 000 | 12 | 7.1 | 2.00 | awake | 30.7 M | 43.3 M | 0.77 |
  | below the rail 256 | 20 000 | 16 | 6.9 | 2.00 | awake | 30.3 M | 41.0 M | 0.83 |
  | below the rail 256 | 5 000 | 1 | 10.1 | 2.00 | awake | 25.1 M | 55.7 M | 0.55 |
  | below the rail 256 | 5 000 | 4 | 8.6 | 2.00 | awake | 13.1 M | 50.3 M | 0.42 |
  | below the rail 256 | 5 000 | 8 | 8.7 | 2.00 | awake | 1.7 M | 44.5 M | 0.39 |
  | below the rail 256 | 5 000 | 12 | 8.5 | 2.00 | awake | 0.0 M | 40.2 M | 0.39 |
  | below the rail 256 | 5 000 | 16 | 8.0 | 2.00 | awake | 0.0 M | 37.4 M | 0.31 |
  | below the rail 1024 | 20 000 | 1 | 10.3 | 2.07 | awake | 121.5 M | 221.8 M | 0.39 |
  | below the rail 1024 | 20 000 | 4 | 26.4 | 2.14 | awake | 170.7 M | 148.5 M | 0.00 |
  | below the rail 1024 | 20 000 | 8 | 15.8 | 2.59 | awake | 206.8 M | 118.9 M | 0.04 |
  | below the rail 1024 | 20 000 | 12 | 12.6 | 2.51 | awake | 213.1 M | 110.3 M | 0.16 |
  | below the rail 1024 | 20 000 | 16 | 37.9 | 2.43 | awake | 213.9 M | 107.6 M | 0.00 |
  | below the rail 1024 | 20 000 | 32 | 14.9 | 2.59 | awake | 213.9 M | 106.8 M | 0.07 |
  | below the rail 1024 | 20 000 | 48 | 27.4 | 2.26 | awake | 213.9 M | 106.0 M | 0.00 |
  | below the rail 1024 | 20 000 | 64 | 11.7 | 2.49 | awake | 213.8 M | 106.5 M | 0.24 |
  | below the rail 1024 | 20 000 | 72 | 30.0 | 2.30 | slow-wave | 213.9 M | 106.4 M | 0.00 |
  | below the rail 1024 | 20 000 | 80 | 17.0 | 2.64 | awake | 213.9 M | 106.6 M | 0.03 |
  | below the rail 1024 | 5 000 | 1 | 10.4 | 2.09 | awake | 101.0 M | 221.4 M | 0.61 |
  | below the rail 1024 | 5 000 | 4 | 9.0 | 2.19 | awake | 42.5 M | 163.7 M | 0.43 |
  | below the rail 1024 | 5 000 | 8 | 24.4 | 2.04 | awake | 1.6 M | 122.2 M | 1.00 |
  | below the rail 1024 | 5 000 | 12 | 8.6 | 2.30 | awake | 0.0 M | 113.9 M | 0.38 |
  | below the rail 1024 | 5 000 | 16 | 21.5 | 2.54 | awake | 0.0 M | 108.9 M | 1.00 |
  | below the rail 1024 | 5 000 | 32 | 14.1 | 2.42 | awake | 0.0 M | 108.3 M | 0.93 |
  | below the rail 1024 | 5 000 | 48 | 30.1 | 2.13 | awake | 0.0 M | 108.0 M | 0.96 |
  | below the rail 1024 | 5 000 | 64 | 13.9 | 2.44 | awake | 0.0 M | 108.1 M | 0.92 |
  | below the rail 1024 | 5 000 | 72 | 25.3 | 2.07 | slow-wave | 0.1 M | 107.9 M | 1.00 |
  | below the rail 1024 | 5 000 | 80 | 9.6 | 2.24 | awake | 0.0 M | 108.0 M | 0.53 |

  The sums before the first window: 58 968 247 excitatory at both priors; 29 320 198 inhibitory at 256 units below the rail (53 475 744 at the rail) and 117 719 176 at 1 024 (213 902 976 at the rail). The days on the prior at the rail read the ninth reading too: at 256 units with the gain held the fraction at the target rises from 0.59 to 0.85 over sixteen windows at the 20 000-tick period and stays at 0.3 at 5 000.

- **The rule's outcome.** At the 20 000-tick period at 256 units the rule as written held: the fraction at the target averaged 0.646 over the first four windows and 0.834 over the last four (a rise of 0.188), and the inhibitory sum after the sixteenth window, 30 329 484, is neither the rail nor below a tenth of the prior's. But the same rise is read on the prior at the rail (0.715 to 0.844), where the inhibitory rule cannot grow: the rise is the rate's fall from 9.8 to 6.9 Hz under the excitatory drift of ADR-0055, into the band of 2.3 to 9.2 Hz the period names, and not the inhibitory rule's work. The rule itself moved the inhibition by little: up 4.8 per cent to 30.74 million at the tenth window, while the rate stood above the target, and back to 3.4 per cent above the prior by the sixteenth as the rate fell toward it. At the 5 000-tick period the rule as written failed: the fraction fell from 0.418 to 0.297, and the inhibition was stripped to nothing by the fifteenth window while the rate stayed at 8 to 9 Hz, since a target above what the drive and the recurrence give leaves the rule nothing to do but remove inhibition, and removing all of it raised the rate by a hertz. At 1 024 units under the controller the population sits on the limit cycle at the ceiling, 10 to 39 Hz whatever the inhibition: at the 20 000-tick period the inhibition climbs from 121.5 million to the rail (213.9 million) within thirteen windows and stays there with the fraction at the target near zero, at the 5 000-tick period it is stripped within nine and the fraction reads the limit cycle's passage through the band (0.4 to 1.0); the excitatory sum settles at 0.45 of the prior's at both, as on the prior at the rail.
- **What the maintainers have to choose a target with.** The rule is a rate homeostat per target that saturates at the rail above the population's regime and empties below it, so a target the rule can hold lies inside the regime the drive and the recurrence give: on this drive at 256 units with the gain held, 7 to 10 Hz, where the 20 000-tick period moved the inhibition by less than five per cent in sixteen windows and the 5 000-tick period stripped it. At 1 024 units under the controller the regime is the ceiling's limit cycle, and the rule can do nothing but saturate. A choice therefore needs two things this round did not have: an input that is not the reference drive (the stationary drive is the measurement's, and the population's own rate under it is not a target's rationale) and the controller's fixed point away from the ceiling (ADR-0036's open question). The objective a choice is read against is the fraction at the target per window, in the harness and in the image's own train; the reading here is what it does at the two periods.
- **The registry item.** Whitepaper §11.1's item is restated under the invariant: a parameter that changes what the engine does is the maintainers' to choose, through the configuration's default (`Config::istdp_target_period_ticks`) and the image, and no gate class is made; the objective a choice would be read against is this reading.

### Consequences

- Good: the inhibitory rule is read where it can move both ways, at both sizes, beside the day on the prior at the rail; the reading a target needs exists in the harness and in this record; the invariant is stated where the item stood, so the next reader does not re-derive it.
- Neutral: the ninth reading moved the days' pins, which ADR-0055 moved in the same round; the 1 024-unit day at the lower gain is a second day in the weekly job.
- Bad: none found.

## Alternatives considered and why rejected

- **Option 2**: outside a brief's empowerment (`CLAUDE.md`'s invariant); and ADR-0031's boundary stands on its own merits: equality of two hashes is decidable by the engine, a score of its own choosing is not a test.
- **Option 3**: the reading says what each period does to the rates and the sums on this drive; which rate the reference network should fire at is not a reading, and a value chosen from one day of one drive would be a constant with a story.
- **Option 4**: the generator's gain draws the band; a field would name what a parameter already does.

## Confirmation

- `runtime/cortex-runtime/tests/reference.rs`: `prior_below_rail`, `at_target_q16`, the ninth element of `DayWindow`, the two below-rail days pinned; the days of ADR-0053 pinned again with the ninth reading; `the_priors_sums_before_any_window` pinning the sums the drifts are read against.
- The mutation gate on the changed lines (ADR-0030): the harness is test code.
