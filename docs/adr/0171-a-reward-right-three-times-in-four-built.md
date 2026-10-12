---
status: proposed
date: 2026-10-12
depends-on: ADR-0170
decision-makers: VirtualCortex maintainers
---

# ADR-0171: A reward right three times in four, built — ADR-0170's feedback in the task: `Feedback::ThreeInFour`, beside the four, under which the reward's sign is the outcome's unless the trial's coin is misleading and then the opposite, whatever the outcome was; the coin two of the misleading coin's three bits, 48 and 49 of the trial's own draw, misleading when both are zero whatever bit 50 is, held at compile time to be two, to be among the three with bit 50 the one left out, and to be no bit the stimulus, the shuffled control or the exploration reads; so every trial misleading at seven in eight is misleading here and the ones beside them are those whose bit 50 is set; no rule of the engine, no record and nothing of the image changed, and under the four feedbacks before it a trial is the trial it was

## Context and Problem Statement

[ADR-0170](0170-a-reward-right-three-times-in-four.md) decided a reward that is right three times in four and left its placement to brief 069. Its decision:
- *"The rule. The reward's sign is the outcome's in three trials of four and its opposite in one, whatever the outcome was: a correct selection is punished in one trial of four, and a wrong one or a tie is rewarded in one of four."*
- *"The coin is bits 48 and 49 of the task's draw for the trial, two of the three `MISLEADING_BITS`. The reward is misleading when both are zero, whatever bit 50 is."*
- *"Nested. Seven in eight misleads when bits 48, 49 and 50 are all zero. Every such trial misleads here, and so do the trials where bit 50 alone of the three is set."*
- *"Where it lives. A feedback of the task beside `Answer`, `Shuffled`, `Withheld` and `SevenInEight`. Under the four, every run is the run it was, bit for bit, H-28's arms among them."*

Brief 069's empowerment names the choice this ADR makes: the feedback's name and shape, *"a variant of its own beside `SevenInEight`, or one form that carries the share, with `SevenInEight` the run it was either way."* This ADR is the build. H-35's protocol and its run are the measurement's, in the ADR that follows this one.

What was read first (principle 2, and the brief's directive that the engine and the harness are read before a description of them is trusted):

1. **The feedback** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`): after the selection and the address, one `match` on `Feedback` gives the reward's sign — `correct` under `Answer`, `coin_at(trial)` under `Shuffled`, `correct != self.misleading_at(trial)` under `SevenInEight` — and returns before any reward call under `Withheld`. It is as [ADR-0148](0148-a-reward-right-seven-times-in-eight-built.md) left it.
2. **The draw**: one `mix64(seed ^ trial)`. Bit 0 is the stimulus, bit 32 the shuffled control's coin, bits 16 to 31 the exploration's coin and 33 to 47 its channel ([ADR-0163](0163-an-exploration-the-value-gates-built.md)); `MISLEADING_BITS` is bits 48, 49 and 50. Two compile-time assertions hold the fields apart.
3. **Every `match` on the feedback** in the tree: the trial's; the harness's, in `earned_run_held_on` (`tests/instrument/harness.rs`), which holds every trial's reward to a rule; and four in `task.rs`'s tests — the oracle of the trial before ADR-0148, where the arm of `SevenInEight` is unreachable, the oracles of the trial before [ADR-0152](0152-three-answers-built.md) and before ADR-0163, each written out with the four feedbacks, and the exploration's test of a drawn selection.
4. **The harness's coin** (`misleads`): the draw shifted down forty-eight places leaving no remainder over eight, written apart from the task's mask.
5. **`Task::check`** refuses a reward of no magnitude and a reward at the ceiling under every feedback but `Withheld`, by one comparison; a new feedback that delivers is refused as the others are with no change there.

## Decision Drivers

- ADR-0170's decision and brief 069's standing directives: one mechanism, a feedback that flips the reward's sign by a coin of the task's own draw at one trial in four; no change to the critic, its window, the address, the modulator, the signed gate, STDP, the membrane or the inhibitory rule; unset, bit for bit; `Feedback::SevenInEight` and `Task::misleading_at` return what they returned.
- **A structural boundary beats a reviewed one** (principle 5): that the coin is nested in seven in eight's and reads no other draw's bit is a compile-time assertion on its mask.
- Latest ≠ Newest (§2.1): two bits of a draw the task already makes, read by a mask. Nothing is adopted.
- The mutation gate ([ADR-0030](0030-verification-governance.md)): every mutant in the changed lines is caught by a test.

## Considered Options

1. **The feedback's shape**: (a) a variant of its own with no field, `Feedback::ThreeInFour`, beside `SevenInEight`; (b) one variant that carries the share — a mask, a number of bits or a fraction — in `SevenInEight`'s place; (c) a field of the task beside the feedback.
2. **Where the coin's rule is held**: (a) a mask constant of its own with a compile-time assertion, and `Task::misleading_in_four_at`; (b) `Task::misleading_at` given a mask.
3. **How nesting is held**: (a) by the compiler, the mask a subset of `MISLEADING_BITS` with bit 50 the one left out, and by a test over the lattice of seeds and trials; (b) by the test alone.
4. **The harness's coin**: (a) written a second time, by a shift and a remainder; (b) the task's own function called from the harness.
5. **The oracles of earlier rounds in `task.rs`'s tests**: (a) the new feedback written into the two that hold the four, beside them, and run with them; (b) a third oracle written out for this round.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

- **The feedback** (`runtime/cortex-runtime/src/task.rs`, option 1(a)): `Feedback::ThreeInFour`, beside `Answer`, `Shuffled`, `Withheld` and `SevenInEight`. In the trial's `match` its sign is `correct != self.misleading_in_four_at(trial)`: the outcome's where the coin is not misleading, the opposite where it is, for a correct selection, a wrong one and a tie alike.
  - A variant that carried the share (option 1(b)) would change `Feedback::SevenInEight`, which every run of H-28 and three oracles name, for a generality with two callers; and a share given as a value could name bits another draw reads, which a constant with an assertion cannot. ADR-0148 said a second reliability would be *"another variant or a field, by another ADR"*; this is the variant.
  - A field of the task (option 1(c)) would let a coin be set beside `Shuffled` or `Withheld`, two combinations with no meaning, as ADR-0148 found.
- **The coin** (option 2(a)): `MISLEADING_IN_FOUR_BITS`, the mask `0x0003_0000_0000_0000`, and `Task::misleading_in_four_at(trial)`, true when `mix64(seed ^ trial) & MISLEADING_IN_FOUR_BITS` is zero. `Task::misleading_at` is untouched (option 2(b) would have changed its signature and every caller).
- **Nested, by the compiler** (option 3(a)): one `const` assertion holds the mask to two bits, to no bit outside `MISLEADING_BITS`, to leave of that mask exactly bit 50, and to no bit of the stimulus's, the shuffled control's or the exploration's two fields. So:
  - the share is one in four by construction;
  - a draw whose three bits are zero has these two zero: **every trial misleading at seven in eight is misleading here**;
  - a trial misleading here is misleading there exactly when its bit 50 is clear, so **the trials misleading here and not there are those whose bit 50 is set**.
- **What is flipped** is the sign, before either critic takes the reward, as under `SevenInEight`. `Outcome::correct` is the selection, whatever reward the trial then received, and the address, written before the `match`, is the outcome's.
- **The bits are ADR-0170's**, on `main` at `d090b30` before this round began and before the coin's count over any schedule was computed.
- **Nothing else moves.** No rule of the engine, no record, no byte of the image, no format. `Task` and `Outcome` keep their fields. `runtime/cortex-runtime/src/lib.rs` exports the mask.

### The harness

- `misleads_in_four(seed, trial)` (`tests/instrument/harness.rs`, option 4(a)): the coin written a second time, the draw shifted down forty-eight places leaving no remainder over four. The harness's `match` on the feedback holds every trial's reward to `correct != misleads_in_four(…)` under the new feedback, as it holds it to `correct != misleads(…)` under `SevenInEight`. Calling the task's function (option 4(b)) would hold the task to itself.
- Both oracles are untouched. They replay from the signal each reward left.

### The tests

Each number below is from an oracle written apart from the tree: SplitMix64's finaliser in another language, with bits 48 and 49 read by a shift and a remainder. It was held first to the tree's pins of ADR-0148 — the stimuli and the shuffled coin of the first sixteen trials at seed 27, the misleading coin's two masks, its 513 of 4 096, its 949 over H-25's schedule and its 502 among the lattice test's seeded pairs — and reproduced each.

- `the_coin_at_three_in_four_of_the_first_sixty_four_trials_at_seeds_0_and_27`: the masks `0x8114_000c_130e_0045` and `0x0300_8218_2a00_078c`, fifteen trials of sixty-four at each seed; the seven in eight's eight are among them at each, and the seven beside them are named.
- `the_coin_at_three_in_four_is_one_trial_in_four_whatever_the_other_two_draws`: at seed 27 over 4 096 trials, 1 003 misleading; by the stimulus's bit and the shuffled coin's, 268, 228, 258 and 249 of 1 039, 991, 1 051 and 1 015, each between a fifth and three tenths of its cell; the seven in eight's 513 all among them, and the 490 beside them the trials whose bit 50 is set, trial by trial; another seed's coin differs at 1 478.
- `the_reward_s_sign_at_three_in_four_is_the_outcome_s_exactly_where_the_coin_is_not_misleading`: over the first eight trials at the unit tests' seed, the coin misleading at the first, the third and the seventh — of both stimuli — and for each outcome, the answer's readout cued, the other's, and none:
  - the trial is the trial under the answer's feedback on a twin engine in every field where the coin is not misleading, and in every field but the reward and the signal, which are the opposite, where it is;
  - it is the trial under `SevenInEight` on a third engine at every trial but the seventh, which misleads here alone, and there the two rewards are opposite;
  - the addressed set, the units' fields and the train are the twin's in all twenty-four;
  - under the engine's critic at weights of zero the error is the misleading reward itself;
  - the two refusals are named.
- `the_coin_at_three_in_four_is_the_hand_rule_and_nested_over_the_lattice` (over `testkit/prop.rs`): for seeds and trials built from the `u32` lattice's words and for 4 096 seeded pairs:
  - the coin is a hand rule, the draw's second byte from the top with its low two bits zero;
  - it is the rule of a draw with bit 0, bit 32, the exploration's thirty-one bits and bit 50 flipped, so it reads none of them;
  - **nested**: a trial misleading at seven in eight is misleading here; a trial misleading here is misleading there exactly when bit 50 is clear; the trials misleading here and not there are those whose bit 50 is set;
  - the three draws beside it are the ones they were;
  - 986 of the seeded pairs mislead here, the seven in eight's 502 and 484 beside them;
  - each single bit of a draw clears the coin exactly when it is bit 48 or 49.
- **Under the four feedbacks before it a trial is the trial it was** (option 5(a)). Two tests already hold `Task::trial` against a trial written out by hand with the four: `a_trial_under_two_readouts_is_the_trial_it_was_and_a_flip_the_flip_it_was`, the task as ADR-0148 left it, under four deliveries from either mapping with two flips; and `with_the_exploration_unset_a_trial_is_the_trial_it_was`, the trial as [ADR-0155](0155-a-punishment-the-value-does-not-soften-built.md) left it, under the task's critic and under the engine's with each delivery. Their arms for the four are as they were. The new feedback is written into each oracle beside them — by the mask's value in the first and by a shift and a remainder in the second — and runs with them, so the same two tests hold the new feedback against a rule written apart from `Task::trial` under every delivery. `a_trial_under_the_three_feedbacks_before_is_the_trial_it_was` stays the oracle of the trial before ADR-0148, where neither coin is reachable. A third oracle written out (option 5(b)) would be a copy of the second.
- The exploration's two tests that run under each feedback run under this one: set, nothing changes where the value is at or above zero; and a drawn selection is judged and rewarded by this feedback's coin.

**Every run under the four feedbacks before it is the run it was, bit for bit.** The whole-domain tests reproduce their pinned numbers in the measurement's calibration and in the weekly dispatched on its branch; the measurement's ADR records both.

### Consequences

- Good: the second share is two bits of a draw the task already made, nested in the first by the compiler, so a run at it stands beside H-28's trial for trial until the first trial that misleads here alone.
- Good: a run stays a function of its seed, and an accuracy keeps its meaning.
- Good: `SevenInEight`, its coin and every test of it are untouched.
- Bad: a share is a variant. A third would be a third variant, a third mask and a third function; at that point one form that carries the share is the better shape, by its own ADR.
- Bad: two coins that share bits cannot be drawn independently in one run. No run asks for both.
- Neutral: `Feedback` grows a variant, and every `match` on it names one more arm. There are six: the trial's, the harness's, and four in the tests.

## Alternatives considered and why rejected

- **One variant that carries the share** (option 1(b)) **or a field of the task** (option 1(c)): see above.
- **`Task::misleading_at` given a mask** (option 2(b)): see above.
- **Nesting held by a test alone** (option 3(b)): a mask edited to bits 49 and 51 would pass every test of its share and fail only the nested one; the assertion fails the build.
- **The task's own coin in the harness** (option 4(b)): see above.
- **A third oracle written out** (option 5(b)): see above.

## Confirmation

- `runtime/cortex-runtime/src/task.rs`: `Feedback::ThreeInFour`, `MISLEADING_IN_FOUR_BITS` and its assertion, `Task::misleading_in_four_at`, `Task::trial`, and the tests named above; `runtime/cortex-runtime/src/lib.rs` exports the mask.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `misleads_in_four`, and the feedback's arm in `earned_run_held_on`.
- Whitepaper §6.5 and §9; `CHANGELOG.md`.
