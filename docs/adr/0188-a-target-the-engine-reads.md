---
status: accepted
date: 2026-10-18
depends-on: ADR-0183
decision-makers: VirtualCortex maintainers
---

# ADR-0188: A target the engine reads — ADR-0183's next decision taken at the target, the maintainers' choice: every target so far was derived offline, from a frozen copy of a network settled by a criterion the engine does not hold, so an image no derivation has settled has none; the engine is given a read: with its gain held for 128 windows from its first, the mean count of the last sixteen becomes its target, and the rate's rule runs at the image's step from the next window; built in the engine with the read's progress in the image (format 23), unset bit for bit; H-40: on seeds 22, 23 and 24 at 1 024 units, at a sixteenth, a target the engine reads serves as a derived one did, the step holding and H-25's two arms holding H-25's four clauses; predicted yes, with three predicted readings; the default is chosen after it reads; brief 074 builds and runs it once, its runs dispatched as ADR-0184 has them

## Context and Problem Statement

[ADR-0183](0183-a-sixteenth-on-three-networks-measured.md) read H-39 as yes: under the rate's rule at a sixteenth, on seeds 22, 23 and 24 at 1 024 units, each network under its own target, the engine learns and revises. Its stopping rule, step 3, names an ADR *"on what the engine carries by default — the rule, the step and a target's rule for a network no derivation has settled"*. [ADR-0187](0187-the-upgrade-suite-planned.md) put a reward past the trial, M10, after it. **The maintainers took the target first, on 2026-10-18: a target the engine reads for itself.**

**Where every target so far came from** (`runtime/cortex-runtime/tests/inhibition.rs`, the derivation):
1. The lead-in runs from a gain of 1.75 under ADR-0044's drive, one window of $2^{17}$ ticks at a time, until it is settled by [ADR-0055](0055-a-weight-that-settles.md)'s criterion. That criterion is over the excitatory sums, not the spikes: sixteen windows each within 0.75 per cent of the sum sixteen windows before.
2. The settled image is frozen, with the modulation's baseline at zero so that nothing learns, and run under the drive for $2^{20}$ ticks, eight windows.
3. The target is the period of their spikes, $1\,024 \times 2^{20}$ over the count, which is 58 201 ticks on seed 22: 2 306 spikes a window.
4. The same period is the inhibitory rule's target at the modulator section's `[20..24)` ([ADR-0128](0128-a-target-the-network-fires-at.md)). H-38 and H-39 carried it into the rate's target at `[56..60)`.

**What the engine holds of that.** Nothing: no criterion of a settled network, no lead-in and no frozen copy. The rate's rule runs from the first window against a period that is a field of the configuration and of the image (`Config::rate_target_period_ticks`, [ADR-0180](0180-a-rate-the-controller-holds-built.md)). An image no derivation has settled has no target to give it.

**What the live network reads beside the frozen copy**, with the gain held (ADR-0183's tables):

| Seed | Derived target, a window | Settled at window | The lead-in's count | Its last sixteen windows beside the target |
| :---: | ---: | ---: | :--- | ---: |
| 22 | 2 306 | 96 | 2 132 to 2 462 | 2.7 % below |
| 23 | 2 255 | 106 | 2 117 to 2 459 | 0.47 % above |
| 24 | 2 268 | 99 | 2 103 to 2 487 | 0.60 % above |

A count read from the live network is a target within a few per cent of the derived one on these three networks. Under the rule at a sixteenth, the count's last sixteen windows came within 0.14 per cent of their target on each.

**What the record and the image have room for.**
- `HomeostaticDrivePool` has no byte free.
- The modulator section has nine reserved: `[26..28)`, `[39]`, `[54..56)` and `[60..64)` (`MODULATOR_RESERVED`).
- The executor's cadences are masks on the tick, so a loaded engine continues the window it was written in. A read in progress that the image did not carry would make an image written during it read back to another run.

## Decision Drivers

- **The maintainers' choice** (2026-10-18): a target the engine reads, before the default and before M10.
- **One change**: the target's source. The step, the networks, the task, the schedule and everything a derivation does besides the rate's target are H-39's, so that each arm stands beside the same network's arm under H-39.
- **No criterion the engine cannot hold**: the read is a span of windows, not ADR-0055's criterion over the sums, which is a test's oracle.
- **An image is the run** (§8.3): a read in progress is part of the image.
- **Unset is the run it was**, bit for bit.
- **A prediction is written**, with what would refute it.
- **Latest ≠ Newest**: a rate read over a window and then held by a rule is decades old; nothing outside the tree is depended on.

## Considered Options

1. **The target's source**:
   - (a) the engine reads the live network's count over a fixed span with its gain held;
   - (b) the engine settles the network by ADR-0055's criterion over the excitatory sums, then reads;
   - (c) the inhibitory rule's own period, ADR-0178's option 2(b), itself a derivation's;
   - (d) a frozen copy counted as the derivation counts it.
2. **The span**: (a) 128 windows held, the last sixteen read; (b) 160, the derivation's bound, the last sixteen read; (c) 112, the last sixteen read.
3. **The networks**: (a) seeds 22, 23 and 24 at 1 024 units, H-39's; (b) seed 22 alone.
4. **What the read sets**: (a) the rate's target alone, the inhibitory rule's target staying the derivation's; (b) both targets.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

### The read (Specified; brief 074 builds it in the engine)

- **Unset, every run is the run it was**, bit for bit, and an image of format 22 reads as unset.
- **Set, the engine holds its gain for 128 windows from its first window**: neither the rate's rule nor the estimate's moves it, whatever the step.
  - The window's count is still taken and cleared, and the estimate is kept as a reading.
  - The counts of the last sixteen of those windows, windows 113 to 128, are summed.
- **At the 128th window's end the target's count is their mean**, to nearest.
  - The period the image carries at `[56..60)` becomes the units times $2^{17}$ over that count, to nearest, so that an image written after the read is the configuration H-39 ran.
  - A count `rate_target_of` would refuse stops the run with its error.
- **From the 129th window the rate's rule moves the gain** at the image's step (ADR-0180), against that target.
- **The read's span and its progress are part of the image**, in the modulator section's reserved bytes: the windows read so far and the sum. An image written during the read reads back to the same run. The image's format becomes 23.
- **A read set beside a period already set is refused**: an image has one target.
- **The inhibitory rule's target is not touched.** It stays the period the derivation writes at `[20..24)`.

### The runs (Specified; brief 074 builds them in the tests)

On each of the three networks, ADR-0044's generator at 1 024 units from seeds 22, 23 and 24:
- **the derivation under the read, at a sixteenth.**
  - The lead-in runs from the first window with the read set, its 128 windows of held gain first. It continues under the rule until it settles by H-39's two criteria, in its sums and in its gain's mean, within 320 windows.
  - Then H-39's quiet run, the cancel, the frozen block's two measures, the inhibitory target period and the critic's constants.
- **H-25's two arms** on H-20's schedule from that derivation's image, the step and the read target carried through every trial.

### H-40 (Hypothesis; written before any run)

**On each of the three networks, with the controller holding a target the engine read:**
- **clause 0, the read and the step hold**: the read gives a target `rate_target_of` accepts; the lead-in settles in its sums and in its gain's mean; every later rule gives a value; and the frozen block passes both measures;
- **clauses 1 to 4, in both arms**: H-25's four.
  - Every mapping learned, at least 80 of its last 128 trials correct.
  - No coupling past 1.30 of its image's.
  - Each of the three reversals passing 40 of 64 within 23 blocks of its flip.
  - The excitatory sum outside the four couplings within 0.75 and 1.25 of the image's.

**Yes** when all five hold on all three networks; otherwise **no**, with the network, the arm and the clause.

**Predicted: yes.**
- **The reason.** A read target lies within a few per cent of the derived one on these networks. H-39's yes did not rest on the target's last per cent: a sixteenth held each network's count at its target with the gain held's scatter, and the task's mappings and reversals were learned with every clause to spare.
- **What would refute it**:
  - a network still drifting in its count at windows 113 to 128, so that its target is the drift's and not its rate's;
  - a target a few per cent low that moves the gain far enough from 1.75 to change what the stimulus reaches;
  - the 128 windows of held gain leaving the lead-in under the rule too few windows to settle within the bound.

**Three readings are predicted**; a miss is recorded as one:
1. **each network's read target is within five per cent of its derived one**, seed 22's below it;
2. **in each of the six arms the mean count a window is within one per cent of its read target**;
3. **through each of the six arms the gain stays within five per cent of 1.75** after every window.

**Readings, no clause:**
- **each network beside its own arms under H-39**: each mapping's last 128, each reversal's blocks, the highest coupling, a window's count and its scatter, the gain's band and mean, the inhibitory sum's course, and the value and its troughs;
- per network: the read's sixteen windows, the read target beside the derived one, the lead-in's window of settling beside H-39's, and the inhibitory target period beside H-39's;
- over the three networks: the mappings learned of twenty-four, and the reversals' blocks beside H-39's.

### H-40's stopping rule

1. **One round**: brief 074 builds the read in the engine, with its tests and its ADR, then runs H-40.
   - The span, H-40's clauses, its constants and its predicted readings are committed before any lead-in is run with the read set.
   - The runs are dispatches, as [ADR-0184](0184-the-runs-off-the-developer-machine.md) has them.
2. **A calibration stops the round before any such run, and is a finding, if a pinned number of the tree moves**: with the read unset, every run is the run it was at format 23, H-39's derivations and six arms among them.
3. **Yes**: a target the engine reads serves as a derived one did. The next decision is an ADR on the engine's default for an image no derivation has settled: the rule, the step, the read and the inhibitory rule's target by the same read. Then M10. Named and not taken.
4. **No on clause 0, on a network**: the read's span does not serve that network. The next decision is an ADR on the span or on a criterion in the engine, with the read's windows read.
5. **Otherwise no**: a read target serves worse than a derived one. The next decision is an ADR on what differs between the live count and the frozen one, each network beside its arms under H-39.
6. **No constant moves after such a run.** There is no second attempt, no other span, no other step and no other network.

## Consequences

- Good: the engine's target needs no derivation. The question a default rests on is read under the task before the default is chosen.
- Good: one change from H-39, so each arm has a twin.
- Bad: a build. The image moves to format 23, and every whole-image pin moves with it by one mask, as at format 22.
- Bad: the span is a constant, not a criterion. A network that settles later than 112 windows reads a drifting count. On these three the latest settled at 106.
- Bad: the inhibitory rule's target is still a derivation's, so the engine is not yet free of the derivation.
- Bad: three derivations and six arms more in the weekly job, about 11 500 s. The table goes from 107 093 s to about 118 600, and the slowest shard reads about 57 per cent of its bound at the last run's ratios. One past 60 is reported (ADR-0179).
- Neutral: the runs are dispatches, so the developer machine runs none of them.

## Alternatives considered and why rejected

- **Option 1(b), ADR-0055's criterion in the engine**: a scan of every excitatory coupling each window, and constants written for a test's oracle, for a decision a span can make on these networks.
- **Option 1(c), the inhibitory rule's period**: a derivation's, the question unanswered.
- **Option 1(d), a frozen copy in the engine**: a second engine inside the first.
- **Option 2(b), 160 windows**: 32 windows more of held gain, after every network has settled.
- **Option 2(c), 112 windows**: seed 23's sums settle at window 106, six windows before its read would begin.
- **Option 3(b), seed 22 alone**: the target is a network's own by a rule, and a rule's yes on one network is that network's.
- **Option 4(b), both targets**: a second change, whose arms would not have H-39's as twins.

## Confirmation

`briefs/074_a-target-the-engine-reads.md` builds the read and runs H-40 once. Whitepaper 4.122.0 carries H-40 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's contract and the version.
