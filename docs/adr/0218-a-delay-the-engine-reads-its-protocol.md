---
status: accepted
date: 2026-10-25
depends-on: ADR-0217
title: A delay the engine reads from its kept addresses, its protocol
kind: protocol
hypothesis: H-45
summary: H-45's protocol, committed before any arm runs. Twelve arms and six controls, each its own weekly test, and two checks of the calibration. Its three clauses and three predicted readings are rules over the tables, and the engine's move is read after each trial.
decision-makers: VirtualCortex maintainers
---

# ADR-0218: A delay the engine reads from its kept addresses, its protocol

**In brief**
- **Decision.** H-45 runs as `untold_arm` in `tests/inhibition.rs`: H-25's arm on H-42's network, an engine keeping two addresses and reading its delay, the task's delay given without its age. Its clauses, constants and three predicted readings are tests' constants and rules from this commit on.
- **Why.** [ADR-0216](0216-a-delay-the-engine-reads.md) and brief 082 require them committed before any arm runs, and fixed after it.
- **Left open.** The verdict, the readings and the next decision: [H-45's report](../research/H-45.md) and the measured ADR, after the one run.

## Context and Problem Statement

ADR-0216 wrote H-45 on 2026-10-25 before any run, and [ADR-0217](0217-a-delay-the-engine-reads-built.md) built the rule and the task's untold delay.
Brief 082 leaves the round four choices for the protocol:
- how the arms, the controls and the readings are dealt into tests and dispatches;
- what of exploration 003's tests is taken;
- how each reading is computed and pinned;
- how the calibration's check of the stopping rule's step 2 is a test.

H-43's harness is the base ([ADR-0209](0209-a-reward-past-the-trial-its-address-kept-its-protocol.md)): `probe_made` runs a network's derivation under the probe and holds it to H-42's pins, and `run_on_scheduled_over` runs the arm with the task giving the reward.
Exploration 003's `untold_arm` gave the reward from the host. Its name is taken, and its path is not: here the task gives each sign through `Executor::reward_untold`.

## Decision Drivers

- **Nothing moves after a run**: clauses, constants and readings are code, held by the gate's test.
- **The step 2 check is a structural one**: with the rule set, a reward that names its age is H-43's arm on its network, bit for bit.
- **The move is read from the engine**, the age in use after each trial, not from a model of it.
- **One run each**, at `scope=arms`, beside the calibration.

## Considered Options

1. **The step 2 check of a reward that names its age**: (a) H-43's arm on seed 22 from the assignment at both delays, the rule set; (b) all twelve of H-43's arms so; (c) the unit twins alone.
2. **A control's clause 3**: (a) read against H-42's pinned arm, the move and the comparison pinned; (b) asserted bit for bit, as H-43's twins were.
3. **Clause 2's coupling and band**: (a) over the blocks from the first of the mapping after the move's; (b) over the whole arm.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**
Committed before any dispatch ran an arm, in brief 082's pull request.

- *Why not 1(b)*: twelve jobs more to hold a path `reward_untold` does not enter. Two arms hold it at both delays on a network, and `a_reward_that_names_its_age_is_the_one_without_the_rule_bit_for_bit` on every walk.
- *Why not 1(c)*: the step names the networks, and a unit twin runs none of them.
- *Why not 2(b)*: a control that moves is clause 3's no, a reading. An assertion would fail its job and void the run.
- *Why not 3(b)*: H-45's clause 2 is *"over every mapping and reversal after the mapping the move comes in"*. A coupling past its bound before the move is under age zero's consolidation, which reading 2 reads over the whole arm.

### The run

`untold_run(n, arm, delay, told)`, one body for the checks, the controls and the arms:
1. **The calibration of the network**: `probe_made`, its derivation under the probe run again and held to H-42's pinned tables.
2. **The engine**: the image it wrote, decoded under H-25's configuration with `kept_addresses` 2 and `reads_its_delay` set (`untold_from`).
3. **The task**: H-25's, with the cancel, the drawn delivery and the answer; `Delay::of(delay)` told, `Delay::untold(delay)` not.
4. **The run**: `run_on_scheduled_over`, H-20's 7 680 trials and three flips.
5. **Read after each trial**: the age in use (`delay_age`), whose first change is the move, with its trial and every age's score; the address each reward wrote, as H-43's arm reads it (`names_rewarded`); a trial from `delay` on whose reward reached no modulator, a reward lost.

### H-45's constants, restated from ADR-0216 and pinned in the tests

| Constant | Value | In the tests |
| :--- | :--- | :--- |
| The networks | seeds 22, 23 and 24 at 1 024 units, H-42's | `UNTOLD_SEEDS` |
| The oldest age kept, the ages read | 2; 0, 1 and 2 | `KEPT_ADDRESSES`, `UNTOLD_AGES` |
| The delays | 1 and 2 trials, untold; the controls on time, untold | `UNTOLD_DELAYS` |
| The arms | H-25's two on H-20's schedule | `UNTOLD_ARMS` |
| The window, the run, the lead | 64 rewards, 64 rewards, 1/32 | `DELAY_WINDOW`, `DELAY_RUN`, `DELAY_LEAD_SHIFT` |
| The prediction | yes, and each reading | `UNTOLD_PREDICTED`, `UNTOLD_READINGS_PREDICTED` |
| Clause 1's bound | the move before trial 5 632 | `LAST_MAPPING_FROM` |
| Reading 1's bounds | before trial 1 536, in 8 arms of 12 | `FIRST_MAPPING_TO`, `EARLY_ARMS` |
| Reading 3's bound | within 1.25, `untold × 4 ≤ told × 5` | `TOLD_QUARTERS` |

### H-45's clauses and verdict

| Clause | Rule |
| :--- | :--- |
| 1, each arm | the move to the true age at a trial below 5 632 (`moved_right`) |
| 2, each arm | over the mappings after the one the move came in (`after_clauses`): each learned, 80 of its last 128; no coupling past 1.30 of its image's and the outside sum in the band at each block's end from the next mapping's first; each such reversal within 23 blocks; all four false where clause 1's trial bound is not met |
| 3, each control | no move, and its blocks, its trace and every reward's value H-42's arm's on its network (`control_held`) |

- **The verdict** (`untold_verdict`): yes when all three hold in all twelve arms and six controls.
- **The stopping rule's step** (`untold_stop`): 3 for yes; 4 for a no on clause 1; 5 for a no on clause 3 with clause 1 whole; 6 otherwise.

### The three predicted readings as rules (`untold_predicted`)

| Reading | Rule |
| :--- | :--- |
| 1. When the engine moves | the move at a trial below 1 536, to any age, in at least 8 arms (`moved_early`) |
| 2. Those arms' clauses | H-25's four over the whole arm (`kept_clauses`) in each such arm |
| 3. Those arms' reversals | `reversals_summed` within 1.25 of H-43's told arm on its network at its delay (`within_told`), in each such arm |

Over no arm moved early, readings 2 and 3 hold vacuously, and the report says so.
None is read until every arm and control is pinned.

### The tests and the dispatches

- **The gate's one test**, `the_delay_read_of_h_45_its_clauses_and_its_predicted_readings_rules`:
  - the constants above;
  - the three clauses, a trial's mapping, the readings' rules and the stopping rule's steps at their edges, over tables written by hand;
  - `untold_tables` over the pinned tables, empty until the runs.
- **The step 2 checks**: `a_reward_that_names_its_age_{one_trial,two_trials}_late_with_the_delay_read_on_seed_22_from_the_assignment_at_1024_units_exhaustive`, held to H-43's pins with no move, no reward lost and no entry let go.
- **The controls**: `a_delay_read_on_time_untold_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`, six.
- **The arms**: `a_delay_read_{one_trial,two_trials}_late_untold_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`, twelve.
- **The order** ([briefs/README.md](../../briefs/README.md#evidencing-a-round)):
  1. on the commit that carries this ADR, the calibration at `scope=exhaustive` with the twenty tests above excluded, and the twenty at `scope=arms`, dispatched together;
  2. the calibration's shards and the two step 2 checks read first; no control's or arm's log is read before all are green;
  3. the tables, the verdict and the readings pinned from the logs;
  4. the reproduction at `scope=exhaustive` with only the twenty, as soon as those are pushed, where nothing under `src/` and no test the calibration ran changed since this commit.

### What `npm run round:additive` read on this commit

Exit 1: the change does not only add, so the calibration runs every test but the round's own ([ADR-0215](0215-the-calibration-a-change-needs.md)).
`npm run round:additive -- origin/main <this commit>` gave six reasons:

```text
runtime/cortex-runtime/src/executor.rs: a file a Rust build reads, outside a package's tests/
runtime/cortex-runtime/src/lib.rs: a file a Rust build reads, outside a package's tests/
runtime/cortex-runtime/src/task.rs: a file a Rust build reads, outside a package's tests/
runtime/cortex-runtime/tests/contention.rs:56: an insertion inside an item, or after a line that does not end one
runtime/cortex-runtime/tests/differential.rs:373: an insertion inside an item, or after a line that does not end one
runtime/cortex-runtime/tests/inhibition.rs:236: 1 line(s) of the base removed or changed
check-additive: e2d0f7d...<this commit> does not only add, 6 reason(s): the calibration runs every test (ADR-0215)
```

### What does not move after a run

`kept_addresses`, the delays, the window, the run, the lead, the latch, the networks, the task, the schedule, the clauses, the constants and the three readings.
There is no second attempt.

## Consequences

- Good: a control that moves, or one not H-42's, is read and pinned, not lost to a failed job.
- Good: the move is the engine's own reading, its trial and its scores pinned beside the blocks.
- Bad: twenty weekly tests, about 1 100 s each on AArch64, about 22 000 s in all.
- Neutral: each arm runs its network's derivation again, as H-42's and H-43's did.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: the `UNTOLD_*` constants, the rules above, `untold_run`, `untold_arm`, `named_with_the_rule`, `untold_tables`, and the twenty `exhaustive` tests and the gate's test named above.
- [H-45's report](../research/H-45.md) holds the hypothesis as ADR-0216 wrote it.
