---
status: accepted
date: 2026-10-25
depends-on: ADR-0212
title: A reward right three times in four, on the default, its protocol
kind: protocol
hypothesis: H-44
summary: H-44's protocol, committed before any arm runs at three in four. Twelve arms, the default's six and the control's six, each its own weekly test, and three calibration tests of the control's image, the coin and the default's first blocks. The clauses and readings are rules over the tables.
decision-makers: VirtualCortex maintainers
---

# ADR-0213: A reward right three times in four, on the default, its protocol

**In brief**
- **Decision.** H-44 runs as `on_default_arm` in `tests/inhibition.rs`: H-42's arm on its network under `Feedback::ThreeInFour`, from the default's image or the control's. Its clauses, constants and three predicted readings are tests' constants and rules from this commit on.
- **Why.** [ADR-0212](0212-three-in-four-on-the-default.md) and brief 081 require them committed before any arm runs at three in four, and fixed after it.
- **Left open.** The verdict, the readings and the next decision: [H-44's report](../research/H-44.md) and the measured ADR, after the one run.

## Context and Problem Statement

ADR-0212 wrote H-44 on 2026-10-25 before any run, with its control: the default's image with the homeostasis record's step written zero and the image re-sealed.
Brief 081 leaves the round four choices for the protocol:
- how the control's image is made and checked, and where its function lives;
- how the arms and the readings are dealt into tests and dispatches, and what of exploration 001's tests is taken;
- how each reading is computed and pinned;
- how the control's gain is shown held.

Three paths in the tree run H-25's arm on H-42's network:
- `arm_on`, H-42's arms, which refuses a share on a network derived under the rate's rule;
- `kept_arm`, H-43's, through `run_on_scheduled_over` with the task's own feedback, whose twins on time were H-42's arms bit for bit ([ADR-0210](0210-a-reward-past-the-trial-its-address-kept-measured.md));
- exploration 001's `explore_arm`, the same path with the task's feedback withheld and the reward given by the host.

The step is `control_step_q0_16`, bytes 20 to 22 of the homeostasis record (`crates/cortex-homeostasis`).
`Image::decode` takes the step and the gain from the image, not from the configuration.
At a step of zero the rate's rule moves the gain by $\kappa$ times its error, zero, and leaves it bit for bit.

## Decision Drivers

- **Nothing moves after a run**: clauses, constants and readings are code, held by the gate's test.
- **The control is one field apart**, held so by a check over its bytes and by its gain through every arm.
- **The calibration is read before any arm's log**, so its checks of H-44's stopping rule, step 2, are tests of their own.
- **One run each**, at `scope=arms`.

## Considered Options

1. **The arm's body**: (a) H-43's path, `run_on_scheduled_over` with the task's feedback `ThreeInFour`; (b) `arm_on` with H-35's share, its refusal lifted.
2. **Step 2's checks**: (a) three calibration tests, one a network, run by the calibration's dispatch; (b) inside each arm, before its trials.
3. **The control's image**: (a) the step's two bytes written and the section re-sealed, checked by a mask; (b) the record decoded, its step set, and encoded again.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**

- *Why not 1(b)*: `arm_on` under a share holds an arm to the 23 pins of H-28's `MisledPins`, built for H-35's readings, and would be changed under H-42's arms, which the calibration runs.
- *Why not 2(b)*: a check inside an arm is read from that arm's log, and no arm's log is read before the calibration is green.
- *Why not 3(b)*: the masked check names the bytes that move. A record written again could move a byte the rules leave and still decode.

### The control's image

`control_image(n, name, made)` makes it from the image H-42's derivation on network `n` wrote, and holds it before any run:
1. the default's step is the probe's, H-42's pin `PROBE_STEP`, and is not zero;
2. `with_step_bytes` writes the step zero and re-seals the homeostasis section;
3. `only_the_step` holds it to differ from the default's image only in the step's two bytes and the section's entry, and writing the default's step back gives the default's image bit for bit;
4. decoded, its homeostasis record is the default's but for the step, its gain the image's, and its weights, clock, targets, critic and window the default's.

**Its gain held** is read three ways: at every trial's end; in every window by the controller's watch, `GainWatch`, its band one gain; and in the pinned gain by block, every window's gain the image's.

### The run

`on_default_run`, one body for the calibration's blocks and the arms:
1. the engine from the image, held at the load to the network's critic, window, target period, gain, sums and couplings;
2. H-25's task with the cancel, the drawn delivery and the task's feedback, on H-20's schedule;
3. `run_on_scheduled_over`, with the controller watched as H-42's arms watched it;
4. held after it: every trial's reward whole, its sign the outcome's unless the coin misleads; every window's gain the rule's; each window's spikes the record's; no worker's entry dropped.

### The calibration's checks of step 2

`on_default_calibration(n)`, three weekly tests `the_control_at_step_zero_and_the_default_s_first_blocks_on_seed_{22,23,24}_at_1024_units_exhaustive`:
- the network's derivation under the probe, held to H-42's pinned tables (`probe_made`);
- the coin, H-35's: 363, 514, 515 and 521 per mapping, 1 913 of 7 680, the task's at every trial (`on_default_coin`);
- the control's image, as above;
- in each of H-25's two arms, the first block under the true feedback from the default's image, held to H-42's arm on the network in its blocks, its values by block and its gain by block;
- the same block from the control's image, its gain the image's at every trial's end.

A pinned number that moves is read from the calibration's other tests, as every round reads it.

### H-44's constants, restated from ADR-0212 and pinned in the tests

| Constant | Value | In the tests |
| :--- | :--- | :--- |
| The networks | seeds 22, 23 and 24 at 1 024 units, H-42's | `ON_DEFAULT_SEEDS` |
| The arms | H-25's two on H-20's schedule | `ON_DEFAULT_ARMS` |
| The starts | the default's image, the control's | `ON_DEFAULT_STARTS` |
| The control's step | 0 | `CONTROL_STEP_Q0_16` |
| The feedback | `Feedback::ThreeInFour`, H-35's | `IN_FOUR_FEEDBACK` |
| The coin's count | 1 913 of 7 680 | `ON_DEFAULT_COIN` |
| Clause 4's part | a half of $2p - 1$ | `IN_FOUR_RELIABILITY` |
| The prediction | yes, and each reading | `ON_DEFAULT_PREDICTED`, `ON_DEFAULT_READINGS_PREDICTED` |

### H-44's clauses and verdict

- **Clauses 1 to 4**, per network, start and arm, are H-35's by `misled_on` at a half, against the image that network's derivation wrote (`on_default_clauses`).
- Clause 4 reads each stimulus's trials, correct trials and values over each mapping's last two blocks, from the values by block the arm pins (`stimulus_values_of`).
- **The verdict** (`on_default_verdict`): yes when all four hold in the default's six arms; the control's six read the same way, beside it.
- **The stopping rule's step** (`on_default_stop`): 3 at a yes, split (a) by the rule when the control is no and (b) by the network when it is yes; otherwise 4 at a no on clause 3 in a default arm; 5 at a no on clause 1 or 2; 6 at a no on clause 4 alone.

### The three predicted readings as rules (`on_default_predicted`)

| Reading | Rule | Where none |
| :--- | :--- | :--- |
| 1. The control no on clause 1 | in one of the control's six arms a mapping below 80 of its last 128 | until all twelve arms are pinned |
| 2. The default's lowest mapping against its control's | per network and arm, the default's lowest above the control's, strictly | as above |
| 3. The default's reversals | each of the 18 reaching 40 of 64 inside its mapping's 32 blocks (`crossings`) | as above |

### The tests and the dispatches

- **The gate's one test**, `the_reward_right_three_times_in_four_on_the_default_its_control_and_its_predicted_readings_rules`:
  - the constants above and the coin;
  - the step's place in the record, and the control's patch on an image of the instrument's prior at its edges;
  - clause 4's counts from a table of values by block;
  - the four clauses, each failed alone at its edge, the three readings' rules, and the stopping rule's steps with step 3's split, over tables written by hand;
  - `on_default_tables` over the pinned tables, empty until the runs.
- **The arms**: `a_reward_right_three_times_in_four_on_the_default_on_seed_{22,23,24}_from_the_assignment_at_1024_units_exhaustive` and `…_from_the_mirrored_assignment_…`, six; `a_reward_right_three_times_in_four_at_step_zero_on_seed_…`, six.
- **Each arm pins** its blocks, its trace, its values' hash, its values by block and its gain by block.
- **The order** ([briefs/README.md](../../briefs/README.md#evidencing-a-round)):
  1. on the commit that carries this ADR, the calibration at `scope=exhaustive` with the twelve arms excluded, and beside it a trial of the harness, the three calibration tests at `scope=arms`;
  2. the twelve arms at `scope=arms` on the same commit, once the trial is green;
  3. the calibration read first, every shard green; then the arms' tables, the verdict and the readings pinned;
  4. the reproduction at `scope=exhaustive` with `-f only` naming the twelve arms, as soon as those are pushed.

The trial runs no arm at three in four, and takes minutes where the calibration takes about an hour and a half.
A defect of the harness in the first blocks is caught by it before the one run.

### What does not move after a run

The networks, the arms, the control's step, the task, the coin, the schedule, the clauses, the constants and the three readings.
There is no second attempt.

## Consequences

- Good: the control is held one field apart from the default by its bytes and by its gain through every arm, so a split of the yes rests on nothing else.
- Good: the arms are H-43's path with no address kept, whose twins were H-42's arms bit for bit.
- Bad: fifteen weekly tests, twelve arms of about 1 100 to 1 900 s on x86-64 and three calibrations of about 300 s.
- Neutral: each arm and each calibration runs its network's derivation again, as H-42's and H-43's did.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: the `ON_DEFAULT_*` constants, the rules above, `control_image`, `on_default_run`, `on_default_calibration`, `on_default_arm`, `on_default_tables`, and the fifteen `exhaustive` tests and the gate's test named above.
- [H-44's report](../research/H-44.md) holds the hypothesis as ADR-0212 wrote it.
