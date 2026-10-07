---
status: accepted
date: 2026-10-07
depends-on: ADR-0147
decision-makers: VirtualCortex maintainers
---

# ADR-0148: A reward right seven times in eight, built — ADR-0147's feedback in the task: `Feedback::SevenInEight`, beside the three, under which the reward's sign is the outcome's unless the trial's coin is misleading and then the opposite, whatever the outcome was; the coin three bits of the trial's own draw, 48 to 50 of `mix64` of the seed and the trial's index, misleading when all three are zero, held at compile time to be three and to be neither the stimulus's bit nor the shuffled control's; what a trial records as correct stays the selection; no rule of the engine, no record and nothing of the image changed, and under the three feedbacks before it a trial is the trial it was, held against that trial written out as an oracle

## Context and Problem Statement

[ADR-0147](0147-a-reward-right-seven-times-in-eight.md) decided a reward that is right seven times in eight and left its placement to brief 061. Its decision:
- *"The rule. The reward's sign is the outcome's in seven trials of eight and its opposite in one, whatever the outcome was: a correct selection is punished in one trial of eight, and a wrong one or a tie is rewarded in one of eight."*
- *"The coin is three bits of the task's draw for the trial that neither the stimulus's bit nor the shuffled control's reads. The reward is misleading when all three are zero."*
- *"Where it lives. A feedback of the task beside `Answer`, `Shuffled` and `Withheld`. Under the three, every run is the run it was, bit for bit."*
- *"What "correct" means under it: the selection equals the stimulus's answer under the mapping in force, whatever reward the trial then received."*

Brief 061's empowerment names the choices: the feedback's name and shape, which three bits the coin reads, and how the harness's oracles read a trial whose reward is not its outcome's. This ADR is the build. H-28's protocol and its run are the measurement's, in the ADR that follows this one.

What was read first (principle 2, and the brief's directive that the engine is read before a description of it is trusted):

1. **The feedback** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`): after the selection and the address, one `match` on `Feedback` gives the reward's sign — `correct` under `Answer`, `coin_at(trial)` under `Shuffled` — and returns before any reward call under `Withheld`. The magnitude is the task's constant.
2. **The draws**: `stimulus_at(trial)` is bit 0 of `mix64(seed ^ trial)` and `coin_at(trial)` bit 32 of the same draw. `mix64` is SplitMix64's finaliser (`synthesis.rs`), and both draws are pinned at seed 27 against an oracle written in another language (`the_stimulus_and_the_coin_of_the_first_sixteen_trials_at_seed_27`).
3. **What "correct" is**: `selection == Some(self.answer(stimulus))`, formed before the feedback is read. The harness's blocks count it (`run_on_scheduled`), so an accuracy is the selection's under every feedback.
4. **The address** is written before the feedback's `match`, *"whatever the feedback, so that the set is the outcome's and not the reward's"*. A misleading reward therefore reaches the pair the selection addressed.
5. **The critics** take the signed reward after the `match`: the task's critic by `Critic::predict`, the engine's inside `Executor::reward`, which the trial reads back through `Executor::prediction`. Neither reads `correct`.
6. **The harness's oracles** (`tests/instrument/harness.rs`, `tests/inhibition.rs`): the composer and the network's oracle replay the consolidation from the train, the address the executor holds and the signal each reward left. Neither reads whether a trial was correct to decide a consolidation. One place holds the task to a reward: the `match` on the feedback in `earned_run_held`.

**What the brief's context said and the tree holds otherwise.** The brief wrote that the harness's tallies and "earned" tables were written where a correct trial was a rewarded one. Read on 2026-10-07: the tables that read a reward read the one **delivered** — `EarnedTrial`'s fifth field, the error the modulator received under the engine's critic; `earned_blocks`' positive deliveries; `moves_blocks`' split by the sign of the delivery before. None infers a reward from `correct`. They keep their rules under the new feedback, and what they then say is named in the measurement's ADR. No finding is opened: the brief's sentence was a caution, and the tree does not need the change it warned of.

## Decision Drivers

- ADR-0147's decision and brief 061's standing directives: one mechanism, a feedback that flips the reward's sign by a coin of the task's own draw; no change to the critic, its window, the address, the modulator, the signed gate, STDP, the membrane or the inhibitory rule; no second reliability; unset, bit for bit.
- **A structural boundary beats a reviewed one** (principle 5): that the coin reads neither other draw's bit is a compile-time assertion on its mask, not a sentence.
- Latest ≠ Newest (§2.1): three more bits of a draw the task already makes, read by a mask. Nothing is adopted.
- The mutation gate ([ADR-0030](0030-verification-governance.md)): every mutant in the changed lines is caught by a test.

## Considered Options

1. **The feedback's shape**: (a) a variant with no field, `Feedback::SevenInEight`; (b) a variant carrying a reliability, a number of bits or a fraction; (c) a field of the task beside the feedback.
2. **Which bits the coin reads**: (a) bits 48, 49 and 50 of the trial's draw; (b) the three above the shuffled coin's, 33 to 35; (c) three bits of a second draw from a second seed.
3. **Where the rule is held**: (a) a mask constant with a compile-time assertion, and `Task::misleading_at`; (b) a shift and a comparison inside the trial.
4. **What is flipped**: (a) the reward's sign before either critic takes it, `correct` left the selection's; (b) `correct` itself.
5. **The harness's coin**: (a) written a second time, by a shift and a remainder; (b) the task's own function called from the harness.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

- **The feedback** (`runtime/cortex-runtime/src/task.rs`, option 1(a)): `Feedback::SevenInEight`, beside `Answer`, `Shuffled` and `Withheld`. In the trial's `match` its sign is `correct != self.misleading_at(trial)`: the outcome's where the coin is not misleading, the opposite where it is, for a correct selection, a wrong one and a tie alike. A variant that carried a reliability (option 1(b)) would be the second reliability the brief forbids, with no caller; a later round that lowers the reliability adds it by its own ADR. A field of the task (option 1(c)) would let a coin be set beside `Shuffled` or `Withheld`, two combinations with no meaning.
- **The coin** (options 2(a) and 3(a)): `MISLEADING_BITS`, the mask `0x0007_0000_0000_0000`, and `Task::misleading_at(trial)`, true when `mix64(seed ^ trial) & MISLEADING_BITS` is zero. A `const` assertion holds the mask to three bits and to neither bit 0 nor bit 32. So the share is one in eight by construction, and the coin reads no bit another draw reads.
  - **Why 48 to 50.** The stimulus is bit 0 and the shuffled coin bit 32, each the low bit of a half of the draw. The three sit sixteen places above the second, apart from both. Every output bit of the finaliser is a function of every input bit, so no three are better than another; the choice was fixed before the coin's count over the schedule was computed, and is not fitted to it.
  - Bits 33 to 35 (option 2(b)) would sit against the shuffled coin's with nothing gained. A second seed (option 2(c)) was rejected by ADR-0147: a run stays a function of one seed.
- **What is flipped** (option 4(a)): the sign, before the task's critic or the engine's takes the reward. `Outcome::correct` is the selection, as it was, and its documentation now says *whatever reward the trial then received*. The address, written before the `match`, is the outcome's. Flipping `correct` (option 4(b)) would make an accuracy a count of rewards, which is the reading ADR-0147 rules out.
- **Under the engine's critic** the trial records what it recorded: the error the modulator received and the engine's value. The reward the task gave is their sum.
- **The refusals are the ones there were.** `Task::check` refuses a reward of no magnitude and a reward at the ceiling under every feedback that delivers one, and this one delivers.
- **Nothing else moves.** No rule of the engine, no record, no byte of the image, no format. `Task` and `Outcome` keep their fields.

### The harness

- `misleads(seed, trial)` (`tests/instrument/harness.rs`, option 5(a)): the coin written a second time, the draw shifted down forty-eight places leaving no remainder over eight. The harness's `match` on the feedback holds every trial's reward to `correct != misleads(…)` under the new feedback, as it holds it to `correct` under the answer's. Calling the task's function (option 5(b)) would hold the task to itself.
- `fed_run` (`tests/inhibition.rs`): `held_run` under a feedback. `held_run` calls it with the answer's and is the run it was.
- Both oracles are untouched. They replay from the signal each reward left.

### The tests

- `the_misleading_coin_of_the_first_sixty_four_trials_at_seeds_0_and_27`: the coin against the oracle written apart from the tree, SplitMix64's finaliser in another language with the three bits read by a shift and a mask — the masks `0x0104_0000_030c_0005` and `0x0000_0208_0a00_030c`, eight trials of sixty-four at each seed.
- `the_misleading_coin_is_one_trial_in_eight_whatever_the_other_two_draws`: at seed 27 over 4 096 trials, 513 misleading; by the stimulus's bit and the shuffled coin's, 150, 122, 118 and 123 of 1 039, 991, 1 051 and 1 015, every count the apart oracle's and each between a tenth and a sixth of its cell.
- `the_reward_s_sign_is_the_outcome_s_exactly_where_the_coin_is_not_misleading`: over the first eight trials at the unit tests' seed, two of them misleading, and for each outcome — the answer's readout cued, the other's, and none — the trial is the trial under the answer's feedback on a twin engine in every field where the coin is not misleading, and in every field but the reward and the signal, which are the opposite, where it is. The addressed set, the units' fields and the train are the twin's in all twenty-four. Under the engine's critic at weights of zero the error is the misleading reward itself. The two refusals are named.
- `a_trial_under_the_three_feedbacks_before_is_the_trial_it_was`: `Task::trial` against the trial as [ADR-0144](0144-the-gates-output-built.md) left it, written out in the test as the oracle with the three feedbacks it had, on twin engines over eight trials under a drive, a cancel and a sub-window, once under the task's critic with the addressed delivery and once under the engine's critic and its window with the drawn one: the same outcome, units' fields, train, messages drained, addressed set, signal, counts, weights and expectations after every trial, with a reward of each sign among them.
- `the_misleading_coin_is_the_hand_rule_over_the_lattice` (over `testkit/prop.rs`): for seeds and trials built from the `u32` lattice's words and for 4 096 seeded pairs, the coin is a hand rule — the draw's second byte from the top with its low three bits zero — and is the rule of a draw with bits 0 and 32 flipped; the stimulus's draw and the shuffled coin's are the ones they were; and each single bit of a draw clears the coin exactly when it is one of the three.

**Every run under the three feedbacks before it is the run it was, bit for bit.** The whole-domain tests reproduce their pinned numbers in the measurement's calibration and in the weekly dispatched on its branch; the measurement's ADR records both.

### Consequences

- Good: the first reward that is not always true is three bits of a draw the task already made, and a run stays a function of its seed.
- Good: an accuracy keeps its meaning. A trial is correct by its selection under every feedback.
- Good: the coin's independence of the other two draws is held by the compiler.
- Bad: one reliability, in the variant's name. A second is another variant or a field, by another ADR.
- Bad: under the engine's critic a table that reads a reward's sign reads the error's. The two differ wherever the value is beyond the reward; the measurement reads the reward itself back from the error and the value.
- Neutral: `Feedback` grows a variant, and every `match` on it names one more arm. There are three: the trial's, the harness's, and the oracle's in the test that holds the three feedbacks before it, where the arm is unreachable.

## Alternatives considered and why rejected

- **A variant carrying a reliability** (option 1(b)) **or a field of the task** (option 1(c)): see above.
- **Bits 33 to 35** (option 2(b)) **or a second seed** (option 2(c)): see above.
- **A shift and a comparison inside the trial** (option 3(b)): the mask is where the compile-time assertion can hold the bits apart from the other two.
- **Flipping `correct`** (option 4(b)): see above.
- **The task's own coin in the harness** (option 5(b)): see above.

## Confirmation

- `runtime/cortex-runtime/src/task.rs`: `Feedback::SevenInEight`, `MISLEADING_BITS` and its assertion, `Task::misleading_at`, `Task::trial`, and the tests named above; `runtime/cortex-runtime/src/lib.rs` exports the mask.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `misleads`, and the feedback's arm in `earned_run_held`. `runtime/cortex-runtime/tests/inhibition.rs`: `fed_run`.
- Whitepaper §6.5 and §9; `CHANGELOG.md`.
