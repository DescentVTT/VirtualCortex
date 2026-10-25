---
status: accepted
date: 2026-10-25
depends-on: ADR-0208
title: A reward past the trial, its address kept, its protocol
kind: protocol
hypothesis: H-43
summary: H-43's protocol, committed before any arm runs at a delay. Twelve arms and six twins on time, each its own weekly test. The twins are held to H-42's arms bit for bit. The four clauses and three predicted readings are rules over the tables, and the address each reward writes is read after it.
decision-makers: VirtualCortex maintainers
---

# ADR-0209: A reward past the trial, its address kept, its protocol

**In brief**
- **Decision.** H-43 runs as `kept_arm` in `tests/inhibition.rs`: H-25's arm on H-42's network, an engine keeping two addresses, the task's delay set. Its clauses, constants and three predicted readings are tests' constants and rules from this commit on.
- **Why.** [ADR-0207](0207-a-reward-past-the-trial-its-address-kept.md) and brief 080 require them committed before any arm runs at a delay, and fixed after it.
- **Left open.** The verdict, the readings and the next decision: [H-43's report](../research/H-43.md) and the measured ADR, after the one run.

## Context and Problem Statement

ADR-0207 wrote H-43 on 2026-10-25 before any run, and [ADR-0208](0208-a-reward-past-the-trial-its-address-kept-built.md) built the kept addresses and the delay.
Brief 080 leaves the round four choices for the protocol:
- how the twin, the arms and the readings are dealt into tests and dispatches;
- what of exploration 001's tests is taken;
- how each reading is computed and pinned;
- how the address each reward writes is read against the rewarded trial's pair.

H-42's harness is the base ([ADR-0203](0203-a-step-the-engine-makes-from-its-own-loop-protocol.md)): `probe_made` runs a network's derivation under the probe and holds it to H-42's pins.
Exploration 001's arm (`explore_arm`, on its branch) ran H-25's arm through `run_on_scheduled_over` and reproduced H-42's arms on seed 22 in their blocks, trace and values.

## Decision Drivers

- **Nothing moves after a run**: clauses, constants and readings are code, held by the gate's test.
- **The twin is a structural check**: kept and rewarded at age zero, an arm is H-42's on its network, bit for bit.
- **The address is read from the engine**, not from the task's record, so that the reading tests the mechanism.
- **One run each**, at `scope=arms`, after the calibration.

## Considered Options

1. **The arm's body**: (a) `run_on_scheduled_over` with the task's own feedback, exploration 001's path with the reward given by the task; (b) `arm_on` and its oracles.
2. **The twins**: (a) six, both arms on each network, each its own weekly test; (b) three, one arm a network.
3. **A reward's address against its trial's pair**: (a) read from the engine's addressed set after the reward, against the stimulus and selection the harness recorded for the trial it was for; (b) taken from the kept entry.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**
Committed in `e087b47`, pull request [#232](https://github.com/DescentVTT/VirtualCortex/pull/232), before any dispatch ran an arm at a delay.

- *Why not 1(b)*: `arm_on`'s oracles model the reward at the delivering trial's end, its count and address. They would refuse a late reward by construction.
- *Why not 2(b)*: an arm that is not H-42's in one arm on one network would go unseen. Six twins cost six jobs of one dispatch.
- *Why not 3(b)*: the entry is what the engine keeps, and reading it back tests nothing the unit tests do not.

### The arm

`kept_arm(n, arm, delay)`, one body for the twins and the arms:
1. **The calibration**: `probe_made`, the network's derivation under the probe run again and held to H-42's pinned tables.
2. **The engine**: the image it wrote, decoded under H-25's configuration with `kept_addresses` 2 (`kept_from`).
3. **The task**: H-25's, with the cancel, the drawn delivery and the answer, `Task::delay` at `Delay::of(delay)`.
4. **The run**: `run_on_scheduled_over`, H-20's 7 680 trials and three flips.
5. **Held**: no entry let go. The twin's blocks, trace and every trial's value are H-42's arm's (`PROBE_ARM_*`). An arm at a delay is held to its own tables once pinned.

### H-43's constants, restated from ADR-0207 and pinned in the tests

| Constant | Value | In the tests |
| :--- | :--- | :--- |
| The networks | seeds 22, 23 and 24 at 1 024 units, H-42's | `KEPT_SEEDS` |
| The oldest age kept | 2 | `KEPT_ADDRESSES` |
| The delays | 1 and 2 trials | `KEPT_DELAYS` |
| The arms | H-25's two on H-20's schedule | `KEPT_ARMS` |
| The prediction | yes, and each reading | `KEPT_PREDICTED`, `KEPT_READINGS_PREDICTED` |
| Reading 1's bound | 94 of 100 rewards | `NAMED_PER_CENT` |
| A reversal never crossed | 32 blocks | `UNCROSSED_BLOCKS` |

### The address each reward writes, read

After each trial whose end gave a reward, for the trial `delay` before (`names_rewarded`):
- **the targets**: that trial selected a readout, and the addressed targets are exactly that readout's units;
- **the sources**: more addressed sources lie in that trial's stimulus set than in the other stimulus set.
- Both hold: the address names the rewarded trial's pair.

Per block the harness sums the rewards that named the pair, the rewards, and the values the engine read at them (`KeptBlock`).

### H-43's clauses and verdict

- **Clauses 1 to 4**, per network, delay and arm, are H-25's four by `drawn_of` against the image that network's derivation wrote (`kept_clauses`).
- **The verdict** (`kept_verdict`): yes when all four hold in all twelve arms.
- **The stopping rule's step** (`kept_stop`): 3 for yes; 4 where every arm at one trial's delay holds all four; 5 otherwise.

### The three predicted readings as rules (`kept_predicted`)

| Reading | Rule | Where none |
| :--- | :--- | :--- |
| 1. The address names the rewarded pair | `named × 100 ≥ rewards × 94` over each arm, a reward at least (`named_within`) | until all twelve arms are pinned |
| 2. The reversals' summed blocks | per network and arm, at delay 1 at least the twin's, at delay 2 at least delay 1's (`reversals_summed`) | as above |
| 3. Each mapping's last block's value | its summed value times 2 above its rewards times the reward, strictly (`values_above_half`) | as above |

The twin's reversals are H-42's arm's, `PROBE_ARM_BLOCKS`, which the twin reproduces.

### The tests and the dispatches

- **The gate's one test**, `the_kept_addresses_of_h_43_its_clauses_and_its_predicted_readings_rules`:
  - the constants above;
  - the address's rule, the reversals' sum, an arm's reading and the three readings' rules at their edges;
  - the four clauses, each failed alone, and the stopping rule's steps, over tables written by hand;
  - `kept_tables` over the pinned tables, empty until the runs.
- **The twins**: `a_reward_kept_on_time_on_seed_{22,23,24}_from_the_assignment_at_1024_units_exhaustive` and `…_from_the_mirrored_assignment_…`.
- **The arms**: `a_reward_one_trial_late_its_address_kept_on_seed_{22,23,24}_from_the_assignment_at_1024_units_exhaustive`, `a_reward_two_trials_late_…` and the mirrored assignment's, twelve.
- **The order** ([briefs/README.md](../../briefs/README.md#evidencing-a-round)):
  1. the calibration at `scope=exhaustive`, the eighteen tests above excluded, on the commit that carries this ADR;
  2. the six twins at `scope=arms`, beside it;
  3. the twelve arms at `scope=arms`, once, after both are green, their tables, the verdict and the readings then pinned;
  4. the reproduction at `scope=both`, as soon as those are pushed.

### What does not move after a run

`kept_addresses`, the delays, the networks, the task, the schedule, the clauses, the constants and the three readings.
There is no second attempt.

## Consequences

- Good: the twin makes the calibration a check of the mechanism on the networks it runs on.
- Good: the address is read from the engine, so a misaddressed reward is a reading, not an assumption.
- Bad: eighteen weekly tests, about 1 100 s each on AArch64, about 20 000 s in all.
- Neutral: each arm runs its network's derivation again, as H-42's did.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: the `KEPT_*` constants, the rules above, `kept_arm`, `kept_tables`, and the eighteen `exhaustive` tests and the gate's test named above.
- [H-43's report](../research/H-43.md) holds the hypothesis as ADR-0207 wrote it.
