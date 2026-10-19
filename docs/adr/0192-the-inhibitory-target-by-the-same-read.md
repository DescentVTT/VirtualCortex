---
status: proposed
date: 2026-10-19
depends-on: ADR-0190
decision-makers: VirtualCortex maintainers
---

# ADR-0192: The inhibitory target by the same read — ADR-0190's next decision taken at the inhibitory rule's target, the maintainers' choice: the derivation settles every network under the inhibitory rule's default period, 20 000 ticks, and writes the network's own period only into the arms' images, from a frozen copy's eight windows; carried into the arms alone the read's period would change no run on these networks, every such period giving the rule the same depression; so with a flag set the read, at its end, sets the inhibitory rule's period beside the rate's, and the rest of the lead-in settles under the network's own targets; the flag in the modulator section's `[26]` (format 24), unset bit for bit, a read whose period the inhibitory rule would refuse refused; H-41 on seeds 22, 23 and 24 at a sixteenth, predicted yes with three predicted readings; F-69; brief 075 builds and runs it once, the first brief with a declared scope

## Context and Problem Statement

[ADR-0190](0190-a-target-the-engine-reads-measured.md) read H-40 as yes: with the engine reading its own rate target, the configuration learns and revises on three networks. Its stopping rule, step 3, names an ADR on the engine's default for an image no derivation has settled: *"the rule, the step, the read and its span, and the inhibitory rule's target by the same read"*. **The maintainers took the inhibitory target first, on 2026-10-18.**

**The inhibitory rule and its target** (`crates/cortex-core/src/dynamics/synapse.rs`, `runtime/cortex-runtime/src/executor.rs`):
- `Config::istdp_target_period_ticks` holds a period, carried at the modulator section's `[20..24)`. Its default, `ISTDP_TARGET_PERIOD_TICKS`, is 20 000 ticks, 5 Hz. It has no unset state: a period outside 100 to 1 000 000 is refused.
- The period becomes one depression a presynaptic spike, `istdp_alpha_q1_15(period)`, which is 1 343 488 over the period, rounded down.
- At each presynaptic spike an inhibitory slot's trace loses that depression, and it gains for each pairing with its target's last spike. A target that fires at one over the period is balanced on average.

**What the derivation runs with** (`runtime/cortex-runtime/tests/inhibition.rs`):
- The lead-in, the read, the quiet run and the frozen block all run under the default 20 000, a depression of 67.
- After the frozen block, the derivation counts a frozen copy over eight windows and writes that period into the images the arms decode. The network settles under 5 Hz, and its own period first acts in the arms.

**Why the read's period carried into the arms alone asks nothing.**
- H-40's read periods are 57 604, 58 002 and 57 579 ticks. The derivation under the read returned 57 512, 58 046 and 57 890.
- Every one of the six gives a depression of 23, as every period from 55 979 to 58 412 does.
- An arm under the read's period would be H-40's arm, run for run.

**What the default leaves undone.** Under a depression of 67 the inhibition keeps falling through the lead-in, the network firing at about 1.7 Hz against a rule balanced at 5:

| Seed | Inhibitory sum at window 96 | At the read's end, window 128 | At the lead-in's settling, window 160 |
| :---: | ---: | ---: | ---: |
| 22 | 165.9 M | 148.3 M | 129.9 M |
| 23 | 160.7 M | 148.8 M | 130.9 M |
| 24 | 164.0 M | 148.2 M | 130.2 M |

In H-40's arms, under the network's own period, it went on falling to 0.771 to 0.779 of its image's.

**What the engine has room for.**
- The modulator section has two reserved bytes left, `[26..28)`. The read's state took `[39]`, `[54..56)` and `[60..64)`.
- Keying the new write on the read alone would change H-40's pinned derivations and arms, since windows 129 to 160 ran under 67. So a flag is needed.
- `rate_target_of` accepts a period the inhibitory rule refuses: at 1 024 units a mean below about 134 spikes a window gives a period past 1 000 000.

**F-69, found in writing this.** ADR-0190's evidence says the arms held their tables on x86-64 *"in 1 147 to 1 692 s"*. Its reproduction's own artifacts, and the cost table regenerated from them, read seed 24's assignment arm at 1 031 s: 1 031 to 1 692 s.

## Decision Drivers

- **The maintainers' choice** (2026-10-18): the inhibitory target by the same read, before the default.
- **A question that can read something**: the arms' target alone would be H-40's runs again.
- **One change, with twins**: everything up to the read's end is H-40's, so each network's read, its target and every window before the 129th are H-40's. The arms differ by the inhibitory rule's target from window 129 of the lead-in on.
- **Unset is the run it was**, bit for bit.
- **A prediction is written**, with what would refute it.
- **Latest ≠ Newest**: the engine's own rule, given a target its own read already made; nothing outside the tree is depended on.

## Considered Options

1. **When the network's inhibitory target acts**:
   - (a) from the read's end, set by the engine with the rate's;
   - (b) only in the arms, the read's period in place of the derivation's;
   - (c) from the first window, a second read before the first.
2. **How the engine is told**: (a) a flag in `[26]`, format 24; (b) the read always setting both, H-40's pins re-pinned.
3. **The networks and the step**: (a) H-40's, seeds 22, 23 and 24 at 1 024 units at a sixteenth; (b) seed 22 alone.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**

### The flag (Specified; brief 075 builds it in the engine)

- **Unset, every run is the run it was**, bit for bit. An image of format 23 reads as unset.
- **Set beside the read**: at the read's last window the engine sets the inhibitory rule's period to the period it sets the rate's to, through the path an image's period takes, `set_istdp_target_period`. Both rules run against the read from the 129th window.
- **Set without the read**: refused.
- **With it set, a read's mean refused if the inhibitory rule would refuse the period it makes**, as `rate_target_of`'s refusal ends a read now.
- **Its byte** is the modulator section's `[26]`; `[27]` stays reserved. The image's format becomes 24.

### The runs (Specified; brief 075 builds them in the tests)

On each of H-40's three networks, seeds 22, 23 and 24 at 1 024 units, at a sixteenth:
- **the derivation under the read with the flag set**:
  - H-40's lead-in: the read's 128 windows, then the rule;
  - from window 129 the inhibitory rule at the read's period;
  - settled by H-39's two criteria within 320 windows;
  - then the quiet run, the cancel, the frozen block's two measures and the critic's constants.
  - Its image carries the read's period as the inhibitory target. The frozen copy's eight windows are still counted, as a reading beside it.
- **H-25's two arms** on H-20's schedule from that image.

### H-41 (Hypothesis; written before any run)

**On each of the three networks, with the engine reading both targets:**
- **clause 0, the read, the settling and the step hold**: the read gives a period both rules accept; the lead-in settles in its sums and in its gain's mean; every later rule gives a value; and the frozen block passes both measures;
- **clauses 1 to 4, in both arms**: H-25's four.
  - Every mapping learned, at least 80 of its last 128 trials correct.
  - No coupling past 1.30 of its image's.
  - Each of the three reversals passing 40 of 64 within 23 blocks of its flip.
  - The excitatory sum outside the four couplings within 0.75 and 1.25 of the image's.

**Yes** when all five hold on all three networks; otherwise **no**, with the network, the arm and the clause.

**Predicted: yes.**
- **The reason.** The inhibitory rule's target becomes the rate the network fires at, where the default holds it to a rate three times higher. That should stop the inhibition's fall rather than start a new movement. The rate's rule holds the count at the target either way. H-40's arms ran under this very period with every clause to spare.
- **What would refute it**:
  - more inhibition left in the settled image, enough that the stimulus no longer reaches what the frozen block's measure asks;
  - a lead-in whose inhibitory sums still move enough that it does not settle within the bound;
  - learning slower from a more inhibited start, a reversal past twenty-three blocks.

**Three readings are predicted**; a miss is recorded as one:
1. **each network's inhibitory sum at its lead-in's settling stands above H-40's**: 129.9, 130.9 and 130.2 M;
2. **in each of the six arms the mean count a window is within one per cent of its read target**;
3. **through each of the six arms the gain stays within five per cent of 1.75** after every window.

**Readings, no clause:**
- **each network beside its own arms under H-40**: each mapping's last 128, each reversal's blocks, the highest coupling, a window's count and its scatter, the gain's band, the inhibitory sum's course, and the value and its troughs;
- per network: the lead-in's inhibitory sum by window beside H-40's, its window of settling beside H-40's 160, and the frozen copy's period beside the read's;
- over the three networks: the mappings learned of twenty-four, and the reversals' blocks beside H-40's.

### H-41's stopping rule

1. **One round**: brief 075 builds the flag in the engine, with its tests and its ADR, then runs H-41.
   - H-41's clauses, its constants and its predicted readings are committed before any lead-in is run with the flag set.
   - The runs are dispatches ([ADR-0184](0184-the-runs-off-the-developer-machine.md), [ADR-0191](0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)).
2. **A calibration stops the round before any such run, and is a finding, if a pinned number of the tree moves**: with the flag unset, every run is the run it was at format 24, H-40's derivations and six arms among them.
3. **Yes**: the engine reads both of its network's targets for itself. The next decision is an ADR on the engine's default for an image no derivation has settled: the rate's rule at a sixteenth, the read of 128 windows, both targets by it. Then M10, a reward past the trial. Named and not taken.
4. **No on clause 0, on a network**: the network's own inhibitory target does not let it settle. The next decision is an ADR on when the inhibitory target is set, with the lead-in's sums read.
5. **Otherwise no**: a network settled under its own inhibitory target learns worse than one settled under 5 Hz. The next decision is an ADR on the inhibitory rule's target during the lead-in, each network beside its arms under H-40.
6. **No constant moves after such a run.** There is no second attempt, no other span, step or network.

### F-69

**ADR-0190 misstates the least of its arms' seconds** on the reproduction: *"1 147 to 1 692 s"*, where its run's artifacts and the cost table read seed 24's assignment arm at 1 031 s. Recorded in §11 as resolved by this record. ADR-0190 is accepted and is not edited, and the cost table is the run's own. No reading or verdict depends on it.

## Consequences

- Good: no derivation step would be left for a network's targets, should H-41 read yes.
- Good: a clean twin. The read, its target and every window up to the 129th are H-40's, so a difference in the arms is the inhibitory target's in the lead-in.
- Bad: a build. The image moves to format 24 and every whole-image pin moves by one mask.
- Bad: the modulator section's last free byte but one is spent.
- Bad: three derivations and six arms more in the weekly job, about 9 500 s at H-40's seconds. At eighteen shards a shard plans about 6 900 s.
- Neutral: the task-side steps of a derivation (the rotation, the gain's calibration, the cancel) are not touched. They are the task's on its network, not the network's own rates.

## Alternatives considered and why rejected

- **Option 1(b), the arms alone**: every period at stake gives the same depression, so the runs would be H-40's.
- **Option 1(c), from the first window**: there is no read to take a target from before the read. A second read before it would be a second span and a second change.
- **Option 2(b), always both**: H-40's pinned derivations and arms would move, and the twin would be lost.
- **Option 3(b), seed 22 alone**: a target's rule is the network's own, and a rule's yes on one network is that network's.

## Confirmation

`briefs/075_the-inhibitory-target-by-the-same-read.md` builds the flag and runs H-41 once. It is the first brief whose front matter declares its scope ([ADR-0191](0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md)). Whitepaper 4.128.0 carries H-41 in §11.1 with its stopping rule, F-69 in §11, and §9's row. `npm run spec` holds the links, the rows, the brief's contract and scope, and the version.
