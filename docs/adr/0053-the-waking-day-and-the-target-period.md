---
status: accepted
date: 2026-09-14
depends-on: ADR-0049
decision-makers: VirtualCortex maintainers
---

# ADR-0053: The waking day and the inhibitory target period — the rate the inhibitory rule drives a target toward is a parameter of the image, not of the registry, since the behaviour gate of ADR-0031 admits no parameter that changes what the engine does (finding F-37); the rule takes the depression per spike as an argument; a day on the reference network measured at 256 and 1 024 units under two periods; the sleep placeholders kept with the reading

## Context and Problem Statement

[ADR-0049](0049-dale-principle-in-plasticity.md) gave inhibitory synapses the symmetric rule of Vogels et al. 2011 with a target rate of 5 Hz as a constant (`ISTDP_TARGET_PERIOD_TICKS` 20 000, `ISTDP_ALPHA_Q1_15` 67), and whitepaper §11.1 left the open item: the target against the reference network's regime (its units fire at 8 to 30 Hz under the drive at a gain of 2.0, "so inhibition onto an active target grows over a long waking run"), the interaction of two rules that move the population's activity (the inhibitory rule per synapse, the criticality controller globally), and a sentence: "the round that runs a waking day on the reference network measures both and makes the rate a `REGISTRY` entry".

The registry cannot hold it. The third gate of [ADR-0031](0031-policy-amendment.md), `PolicyAmendment::record_trial`, rejects an amendment whose candidate fork's behaviour hash differs from the baseline's (`REJECT_BEHAVIOUR_CHANGED`): "the engine may change what it costs, never what it does". A target period changes the depression at every inhibitory presynaptic spike, so every weight the rule touches, so the hash; an entry for it would pass the bounds gate and fail the behaviour gate on every proposal. The sentence prescribed what the registry, as decided, cannot hold: finding F-37, resolved here by putting the period where the modulation baseline lives, in the image. And two facts of the prior shape what a day can show: the reference prior draws every inhibitory weight at the negative rail (`inhibitory_gain_q4_4` 255), so under the rule an inhibitory weight can only lose magnitude ($\alpha$ per presynaptic spike, the pairings' potentiation absorbed by the rail and kept pending in the trace, where it decays); and the pair rule of the excitatory synapses is depression-dominant ($A_- / A_+ = 1.05$), so a population that fires steadily under a stationary drive loses excitatory weight every window whatever inhibition does.

## Decision Drivers

- §8.3: a parameter that changes what a run does is in the image or in the trace; the modulation baseline of [ADR-0032](0032-three-factor-plasticity.md) is the precedent, at `[16..20)` of the modulator section, outranking the configuration on load.
- ADR-0031's own boundary: the registry is for what the engine may amend by itself, under a gate that is decidable by the engine (equal hashes); a plasticity parameter needs an objective over the population's rate, which no gate of the registry has, and adding one is a decision of its own round.
- A rule of `cortex-core` takes what it needs as an argument (ADR-0016, ADR-0049): the depression per spike, not a policy it would read.
- ADR-0010 and principle 3: the day's numbers are the engine's on a stated network at a stated size; a rate is spikes per window over units, stated also in hertz at the fine tick.
- The decision rule for the sleep placeholders was written in brief 025 before the run: a constant moves only when the day shows a stage the constant makes unobservable at shift 5 (a night without a ripple, a slow-wave stage without a replay, a day without a window awake), to the smallest value that makes it observable; otherwise the placeholders stay with the reading attached.

## Considered Options

1. A `REGISTRY` entry for the period, as §11.1 said.
2. **The period as a parameter of the image (`[20..24)` of the modulator section) and of the configuration (`Config::istdp_target_period_ticks`, the image's outranking it), bounded; the rules of `cortex-core` taking the depression per spike as an argument, `istdp_alpha_q1_15(period)`; the day measured at two periods.**
3. The constant as ADR-0049 left it, the day measured under it alone.
4. A registry gate for parameters that change behaviour by design (an objective over the rate the trial reads in both forks), with the period its first entry.

## Decision Outcome

Option 2; image format 14 (with ADR-0052 and ADR-0054).

- **The rule** (`cortex-core`, `synapse.rs`). `istdp_alpha_q1_15(target_period_ticks)` is $2 A_+ 2^{\tau} / \text{period}$, the period clamped to `[ISTDP_PERIOD_MIN_TICKS, ISTDP_PERIOD_MAX_TICKS]` = [100, 1 000 000] ticks (1 kHz to 0.1 Hz at the fine tick; at the shortest the depression is 13 434, the largest the width holds beside a window; at the longest it is one LSB, and a longer period would never depress); at `ISTDP_TARGET_PERIOD_TICKS` it is `ISTDP_ALPHA_Q1_15`, 67, tied by a `const` assertion, and at 5 000 ticks (20 Hz, the reference regime) 268. `step_stdp(slot, pre_now, post_last, polarity, istdp_alpha_q1_15)` and `step_stdp_all(now, posts, polarity, istdp_alpha_q1_15)` take it; the excitatory branch ignores it. Every caller passes it: the executor the engine's, the oracle blocks of the sleep and modulation tests and the benchmark the default's.
- **The executor and the image.** `Config::istdp_target_period_ticks` (default 20 000; refused outside the bounds, `ConfigError::IstdpPeriodOutOfRange`); the depression published to the workers once (`Shared::istdp_alpha`), read in the fan-out phase; `Executor::istdp_target_period_ticks()`. The modulator section carries the period at `[20..24)` beside the baseline; the loader refuses it outside the bounds and takes the image's over the configuration's; the reserved bytes are `[24..64)`.
- **The day** (`tests/reference.rs`, `day(prior, windows, step, shift, period)`). The lattice of [ADR-0044](0044-reference-network.md) from a gain of 2.0 under the drive, the local cluster of twelve tagged at the start so that a night has something to replay; per window the spikes, the gain and the estimate as the window's regulation left them, the stage, the descendants ([ADR-0054](0054-the-causal-count-inside-the-loop.md)), the sum of the inhibitory magnitudes and of the excitatory weights over the arena after the window, and the replays so far. The gate's form: 256 units, the gain held, no sleep, eight windows (10.5 s) at each period. The weekly job's form: 1 024 units, the controller's step of an eighth, the pressure rising at shift 5 from zero, eighty windows (105 s), at each period.
- **What the day reads at 256 units** (the gain held at 2.0; 205 excitatory units with 6 560 synapses drawn in [6 000, 12 000], 51 inhibitory with 1 632 at the rail, 53 475 744 in sum):

  | Period | Window | Spikes | Rate per unit | Descendants | Inhibitory sum | Excitatory sum |
  | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
  | 20 000 | 1 | 3 039 | 9.06 Hz | 1 625 | 53 227 953 | 56 684 837 |
  | 20 000 | 4 | 2 507 | 7.47 Hz | 1 102 | 52 737 333 | 52 050 975 |
  | 20 000 | 8 | 2 299 | 6.85 Hz | 896 | 52 007 040 | 46 951 567 |
  | 5 000 | 1 | 3 075 | 9.17 Hz | 1 631 | 49 247 248 | 56 625 123 |
  | 5 000 | 4 | 2 610 | 7.78 Hz | 1 178 | 37 469 897 | 51 828 909 |
  | 5 000 | 8 | 2 566 | 7.65 Hz | 1 077 | 21 649 131 | 45 843 315 |

  *The inhibitory weights only fall.* From the rail, the rule can only depress: at the 5 Hz period the sum loses 0.46 per cent in the first window and 2.75 per cent in eight (about a third of a per cent per window); at the 20 Hz period it loses 7.9 per cent in the first window and 59.5 per cent in eight, since every presynaptic spike costs 268 of 32 767 and the pairings' potentiation cannot be absorbed at the rail. The open item's sentence had it the other way ("inhibition onto an active target grows"): on this prior it cannot, because the prior starts inhibition where the rule's potentiation has nowhere to go. *The excitatory weights fall faster than either*: 17 per cent in eight windows at either period (56.7 to 47.0 million at 5 Hz, 56.6 to 45.8 million at 20 Hz), the depression-dominant pair rule under a stationary drive, and the rate falls with them, from 9.1 Hz to 6.9 Hz. *The regime is lower than the item said*: 7 to 9 Hz per unit at a gain of 2.0 held, not 8 to 30 (which was a per-unit range across the population, not its mean). *The period's effect on the rate is small over eight windows*: 2 566 against 2 299 spikes in the eighth window, since the weaker inhibition of the 20 Hz period lets the population fire a tenth more while its excitatory weights decay alike.
- **What the day reads at 1 024 units** (the controller's step of an eighth, shift 5, eighty windows, the cluster tagged):

  | Period | Window | Spikes | Rate per unit | Gain | Stage | Descendants | Inhibitory sum | Excitatory sum | Replays |
  | ---: | ---: | ---: | ---: | ---: | :--- | ---: | ---: | ---: | ---: |
  | 20 000 | 1 | 12 424 | 9.3 Hz | 2.065 | awake | 6 993 | 212 996 332 | 225 908 044 | 0 |
  | 20 000 | 4 | 29 304 | 21.8 Hz | 2.568 | awake | 17 636 | 213 863 479 | 142 896 484 | 0 |
  | 20 000 | 8 | 46 974 | 35.0 Hz | 2.488 | awake | 32 222 | 213 902 976 | 11 822 530 | 0 |
  | 20 000 | 12 | 37 594 | 28.0 Hz | 2.389 | awake | 21 903 | 213 900 445 | 394 874 | 0 |
  | 20 000 | 16 | 32 792 | 24.4 Hz | 2.315 | awake | 17 194 | 213 877 160 | 68 284 | 0 |
  | 20 000 | 32 | 33 432 | 24.9 Hz | 2.326 | awake | 17 963 | 213 898 794 | 14 362 | 0 |
  | 20 000 | 65 | 24 066 | 17.9 Hz | 2.741 | awake | 10 107 | 213 881 202 | 29 480 | 0 |
  | 20 000 | 66 | 38 452 | 28.7 Hz | 2.398 | slow-wave | 22 743 | 213 902 850 | 4 212 | 0 |
  | 20 000 | 69 | 20 710 | 15.4 Hz | 2.620 | slow-wave | 8 372 | 213 806 295 | 4 278 048 | 192 |
  | 20 000 | 70 | 33 792 | 25.2 Hz | 2.293 | REM | 18 725 | 213 884 299 | 4 599 265 | 256 |
  | 20 000 | 72 | 29 461 | 22.0 Hz | 2.902 | slow-wave | 14 620 | 213 813 326 | 3 506 757 | 256 |
  | 20 000 | 73 | 50 887 | 37.9 Hz | 2.539 | awake | 36 126 | 213 902 909 | 4 549 552 | 320 |
  | 20 000 | 80 | 19 534 | 14.6 Hz | 2.639 | awake | 6 896 | 213 832 916 | 993 455 | 320 |
  | 5 000 | 1 | 12 529 | 9.3 Hz | 2.063 | awake | 7 030 | 196 786 033 | 225 748 127 | 0 |
  | 5 000 | 4 | 35 109 | 26.2 Hz | 2.141 | awake | 22 944 | 126 799 770 | 121 230 219 | 0 |
  | 5 000 | 8 | 18 707 | 13.9 Hz | 2.555 | awake | 7 160 | 41 440 228 | 19 856 414 | 0 |
  | 5 000 | 12 | 15 497 | 11.6 Hz | 2.476 | awake | 5 091 | 705 040 | 565 074 | 0 |
  | 5 000 | 16 | 54 142 | 40.3 Hz | 2.399 | awake | 41 616 | 238 598 | 0 | 0 |
  | 5 000 | 32 | 28 516 | 21.3 Hz | 2.713 | awake | 14 960 | 528 | 54 963 | 0 |
  | 5 000 | 66 | 13 043 | 9.7 Hz | 2.403 | slow-wave | 3 838 | 0 | 167 546 | 0 |
  | 5 000 | 70 | 48 904 | 36.4 Hz | 2.315 | REM | 35 825 | 1 128 610 | 4 685 921 | 256 |
  | 5 000 | 73 | 22 260 | 16.6 Hz | 2.564 | awake | 10 252 | 1 003 592 | 4 522 756 | 320 |
  | 5 000 | 80 | 29 058 | 21.7 Hz | 2.702 | awake | 15 650 | 214 081 | 1 510 424 | 320 |

  (819 excitatory units with 26 208 synapses drawn in [6 000, 12 000], 205 inhibitory with 6 528 at the rail, 213 902 976 in sum; every window of the eighty is pinned in the test.) *The controller holds the population in a limit cycle at the ceiling*: the gain alternates every window between about 2.3 and 2.9 as the estimate reads 16 at the ceiling one window and nothing the next, and the rate with it between 12 and 38 Hz per unit. *The excitatory arena drains to nothing*: 225.9 million at the first window, 11.8 at the eighth, 0.4 at the twelfth, tens of thousands from the twentieth (a twentieth of one per cent of the start), at either period; the depression-dominant pair rule at 20 to 38 Hz under the stationary drive leaves no excitatory weight standing within fifteen windows (20 s), and the population fires from the drive alone, amplified by the gain, from then on. *The period is a switch on this prior*: at 5 Hz the inhibitory sum stays at the rail throughout (213.9 million, since above the target every pairing potentiates and the rail absorbs it); at 20 Hz it is stripped within fourteen windows (196.8 million at the first, 2.9 at the eleventh, 24 thousand at the fourteenth). *The night*: from a pressure of zero at shift 5 the onset comes at the sixty-sixth window (86 s), the night lasts seven (four slow-wave, two REM, one slow-wave, awake at the seventy-third), the cluster is replayed 64 times per slow-wave window, 320 in all, and its 143 synapses climb from nothing back toward the rail during the night (4.6 million at the seventieth window, the rail's total for them 4.7 million), then drain again awake (1.0 million at the eightieth at 5 Hz, 1.5 at 20 Hz). *The descendants* read 0.3 to 0.7 of the spikes, with the rate ([ADR-0054](0054-the-causal-count-inside-the-loop.md)).

- **The sleep placeholders, under the rule.** At shift 5 from a pressure of zero the day holds an onset, a night of seven windows with both sleep stages and a wake, and 320 replays of the one tagged episode; every stage the constants name is observable, so under the rule no constant moves and the placeholders stand with this reading. What they do not say is what a night does to an arena that waking has drained (the cluster's synapses recover; the other 26 065 do not), and what they should be at a rate an objective chooses.
- **What is decided.** *F-37 is resolved*: the period is a parameter of the image, and the sentence that made it a registry entry is corrected in §11.1. *No target is chosen*: no objective in the tree says what rate the reference network should fire at, and the two periods measured say what each does (a third of a per cent of inhibition per window at 5 Hz, eight per cent at 20 Hz, on a prior that starts at the rail); a round that chooses one needs an objective over the rate, and if the engine is to amend it by itself, a registry gate of its own (option 4), which this ADR names and does not make. *The interaction with the controller* is read at 1 024 units as the gain's course beside the weights' drift, and is stated above as it is.
- **Not adopted.** *Option 1*: the behaviour gate. *Option 3*: a constant is a parameter no image can differ on, and the day at one period says nothing about the rule's sensitivity. *Option 4*: a second gate class is its own decision, with an objective the trial can measure in two forks; named in §11.1.

### Consequences

- Good: the period is in the image, so a run is a function of the image as §8.3 requires, and two images may differ on it.
- Good: the rule takes its parameter as an argument and reads no policy; the default's amounts are unchanged, so every pinned night stood.
- Good: the day is a harness the next controller round starts from: the gain, the rate and the weights' drift per window on the reference network.
- Bad: the reading is one prior at two sizes for a fraction of the engine's own day (80 windows of the 65 536 a circadian cycle holds); what the excitatory drift does over a whole day is extrapolated, not measured.
- Bad: the depression is one `i16` published once per period change; a rule that varied it per synapse or per unit would need a field.

## Alternatives considered and why rejected

- **The period in `Config` alone**: a configuration is not the image; two engines from one image under two configurations would run differently, which §8.3 forbids.
- **A second field in the homeostasis record**: the record has no reserved byte, and the parameter is the plasticity rule's, not the controller's; the modulator section is the runtime's plasticity state and has forty reserved bytes.
- **The day at shift 4 so that a night falls within 64 windows**: the brief named shift 5 and eighty windows hold its onset; the sleep constants are measured as they are.

## Confirmation

`crates/cortex-core`: `istdp_alpha_q1_15` at the bounds and beyond them, the rule taking it. `runtime/cortex-runtime`: the configuration's refusal, the image's byte and the loader's refusal (`tests/image.rs`), the day at 256 units (`a_waking_day_at_256_units_at_two_target_periods`) and at 1 024 units (`a_waking_day_at_1024_units_at_two_target_periods_exhaustive`, the weekly job's). The mutation gate on the changed lines (ADR-0030) passes in CI.
