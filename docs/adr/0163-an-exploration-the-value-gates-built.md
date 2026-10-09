---
status: proposed
date: 2026-10-09
depends-on: ADR-0162
decision-makers: VirtualCortex maintainers
---

# ADR-0163: An exploration the value gates, built — ADR-0162's rule as an option of the task, `Exploration::ValueGated`; the engine's value read by the task at the trial's end from what the executor exposes, the number the reward is then taken against; a coin of sixteen bits and a draw of fifteen, bits 16 to 31 and 33 to 47 of the trial's own draw, held apart from the stimulus's, the shuffled control's and the misleading coin's at compile time; the coin compared with the value in integers, so that the probability is the rule's rounded up to the coin's width and the rule's itself at a reward of 1.0; a drawn selection one of the channels by a multiplication and a shift, each within one draw of 32 768; a trial records whether its selection was drawn; refused without the engine's critic, with a hold and with a reward's magnitude of zero; no rule of the engine, no record and nothing of the image changed; unset, every trial the trial it was

## Context and Problem Statement

[ADR-0162](0162-an-exploration-the-value-gates.md) decided the rule and left its placement to brief 066. Its decision:
- *"$V$ is the engine's own value: its critic's value of the counts its window holds, the value the trial's reward is then taken against;"*
- *"$p$ is the part of $V$ below zero, over the reward's magnitude $r$: $p = \min(\max(-V, 0), r) / r$;"*
- *"with probability $p$ the selection is one of the readout's channels, each with the same chance, whatever the counts; otherwise it is the gate's;"*
- *"the coin and the draw are bits of the trial's own draw that the stimulus, the shuffled control and the misleading coin do not read."*
- *"Where it lives. In the task, which composes the selection. It reads the value from what the executor exposes. No rule of the engine, no record, nothing of the image and no format changes. It is refused without the engine's critic."*

Brief 066's empowerment names the choices: the option's name and shape, and how a trial records a drawn selection; which bits the coin and the draw read, and how a draw of a few bits is made even among three channels; how the probability is compared with the coin in integers, at the reward's magnitude and at the width.

What was read first (principle 2), on 2026-10-09:

1. **The selection** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`). After the trial's last tick the train is read once over the task's window, `Readout::select` hands the counts to `cortex-basal-ganglia`'s gate, and the selection goes three ways: into `correct`, into the address the delivery writes, and through `correct` into the reward's sign. `Readout::select` leaves the gate's reading in the readout's channels, which a hold reads.
2. **A hold** (`Task::hold`, [ADR-0144](0144-the-gates-output-built.md)). With one the selection is made before the tick the readout window closes at, and the channels the gate holds take its messages from there. That is inside the trial, where the critic's counts may still be open.
3. **The value** (`runtime/cortex-runtime/src/executor.rs`, `Executor::reward`). It is `ValueCritic::value_q16` over each unit's `value_weight` and its count in `Executor::features`, in unit order. `Executor::critic`, `Executor::features` and `Executor::units` give all three between ticks. `Task::trial` calls `reward` between the trial's last tick and the next trial's first, with no tick between the selection and the reward, so the counts at the trial's end are the counts `reward` reads, with the critic's window set or unset.
4. **The trial's draw** (`Task::stimulus_at`, `coin_at`, `misleading_at`). One `mix64` of the seed and the trial's index. Bit 0 is the stimulus, bit 32 the shuffled control's coin and bits 48 to 50 the misleading coin. Of the draw's four sixteen-bit words the second, bits 16 to 31, is read by no draw; the third holds fifteen free bits above bit 32, 33 to 47.
5. **The refusals** (`Task::check`). A reward's magnitude of zero is refused only under a feedback that delivers it (`NoReward`): a task whose reward is withheld runs with none.

## Decision Drivers

- ADR-0162's decision and brief 066's standing directives: the one mechanism is a selection that is sometimes drawn, in the task; unset, bit for bit; set, nothing changes where the value is at or above zero; the probability is the rule's and is not fitted; no dependency, no float, no `unsafe`.
- **Nothing of the executor changes** (brief 066, *Not empowered*): the task reads what is exposed.
- **A structural boundary beats a reviewed one** (principle 5): the bits held apart by a compile-time assertion; a combination no rule describes refused by name and not defined in passing.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)): a comparison and two bit fields, each with a case that fails when it moves.
- **A run stays a function of its seed** (§8.3): the coin and the draw are functions of the seed and the trial's index, as the stimulus is.

## Considered Options

1. **The option's shape**: (a) a field of the task, `exploration: Exploration`, an enumeration of two, `Unset` and `ValueGated`; (b) a flag; (c) an optional record of parameters, as `Hold` is.
2. **Where the value is read**: (a) by the task at the trial's end, as the composition of `Executor::reward` over what the executor exposes; (b) by a new method of the executor; (c) at the selection, wherever it is made.
3. **A hold with the exploration**: (a) refused; (b) the hold acting on the gate's reading and the draw made at the trial's end.
4. **The coin**: (a) sixteen bits, 16 to 31 of the trial's draw; (b) fewer bits; (c) bits beside another draw's.
5. **The comparison**: (a) `coin × r < below × 2^16` in `i64`, no division; (b) the probability formed first as a quotient, then compared.
6. **The draw among the channels**: (a) fifteen bits, 33 to 47, dealt by the floor of `draw × channels / 2^15`; (b) a remainder; (c) a draw repeated until it falls evenly.
7. **What a trial records**: (a) whether its selection was drawn; (b) the gate's selection beside the trial's; (c) the probability.
8. **A reward's magnitude of zero**: (a) a refusal of its own; (b) `NoReward`'s meaning widened; (c) no refusal, since the comparison draws at no coin.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a), 7(a) and 8(a).**

- **The option** (`runtime/cortex-runtime/src/task.rs`): `Task::exploration`, of type `Exploration`, `Unset` or `ValueGated`. Every task of the tree names it, unset.
  - *Why not a flag* (option 1(b)): the rule's form is ADR-0162's and has a name; a flag would say only that something is set.
  - *Why not a record of parameters* (option 1(c)): the rule has none. A record would be a place for the constant ADR-0162 declined.
- **The value** (`engine_value`, private to the task): `ValueCritic::value_q16` over `Executor::units` and `Executor::features`, in unit order, read after the trial's last tick and before the delivery and the reward. It is the composition `Executor::reward` then makes over the same weights and counts, so the value the coin is compared with is the value the reward is taken against. A test holds the two equal at every trial it runs: what the trial records as the value is the engine's reading after the reward.
  - *Why not a method of the executor* (option 2(b)): brief 066 changes nothing of the executor. The cost is one composition written twice, held by that test.
  - *Why not at the selection* (option 2(c)): under a hold the selection is made inside the trial. See the next point.
- **A hold is refused** (`TaskError::ExplorationWithHold`). The hold delivers the gate's output from the readout window's close, before the trial's end, and no ADR says which channel it holds at a trial whose selection is then drawn. Composing the two (option 3(b)) would define that in passing. The hold stays in the tree unset ([ADR-0146](0146-the-tag-the-address-already-is.md)), and no run sets both.
- **The coin** (`EXPLORATION_COIN_BITS`, `Task::exploration_coin_at`): bits 16 to 31 of `mix64` of the seed and the trial's index, the draw's second sixteen-bit word, as a number below 65 536.
  - *Why sixteen bits*: it is the width of a Q16.16 fraction, so at a reward of 1.0 one coin is one LSB of the value. Fewer bits (option 4(b)) would round a value near zero up to a coarser step.
  - *Why that word*: no other draw reads a bit of it.
- **The comparison** (`Exploration::draws(coin, value_q16, reward_q16)`): with `below` the part of the value below zero, at most the magnitude — `min(max(−V, 0), r)`, the negation saturating — a coin draws when `coin × r < below × 2^16`. The factors are within $2^{16}$ and $2^{31}$ and the product is in `i64`.
  - So the coins that draw are the first $\lceil \text{below} \cdot 2^{16} / r \rceil$ of the 65 536. That is $p$ rounded up to the coin's width, and $p$ itself wherever $r$ divides $\text{below} \cdot 2^{16}$, as 1.0 does at every value.
  - At a value at or above zero no coin draws. At any value below zero at least one does. At minus the magnitude and beyond every coin does.
  - *Why not a quotient first* (option 5(b)): the floor of $\text{below} \cdot 2^{16} / r$ is zero for a value one LSB below zero under a magnitude above 1.0, where the rule's probability is not zero; and it adds a division by the magnitude.
- **The draw among the channels** (`EXPLORATION_CHANNEL_BITS`, `Task::drawn_at`, `Exploration::channel(draw, channels)`): bits 33 to 47 of the same draw, fifteen, dealt as the floor of `draw × channels / 2^15`. The 32 768 draws go in order into as many runs as there are channels, whose lengths differ by at most one: equal among two and four, and 10 923, 10 923 and 10 922 among three. A draw is read by its low fifteen bits, so the channel is below the number of channels.
  - *Why not a remainder* (option 6(b)): it is as even, and it reads the draw's low bits alone.
  - *Why not a repeated draw* (option 6(c)): exact thirds would need a second draw at some trials, a loop over the draw's bits for one part in 32 768.
- **The two fields are held apart at compile time**: each a whole run of bits at the shift it is read at, the two sharing no bit, and neither holding bit 0, bit 32 or a bit of `MISLEADING_BITS`.
- **The trial** (`Task::trial`): the counts and the gate's selection are made as they were. With the exploration set the task then reads the value, and where the trial's coin draws at it the selection is `Task::drawn_at(trial)`; otherwise it is the gate's. Everything after is as it was and reads that selection: `correct` is it against the stimulus's answer, the delivery's targets are its readout's units, and the reward's sign is the outcome's under each feedback. The readout's channels hold the gate's own reading, whatever was drawn.
- **The record** (`Outcome::drawn`): whether the selection was drawn. The gate's reading is a function of the counts the trial records, so a second selection (option 7(b)) would record what a reader can form; the probability (option 7(c)) is a function of the value the trial records.
- **The refusals** (`Task::check`), each before anything is injected:
  - `ExplorationWithoutCritic`: an engine without the critic holds no value.
  - `ExplorationWithHold`: above.
  - `ExplorationWithoutReward`: a magnitude of zero, over which the probability is taken. A task whose reward is withheld runs with a magnitude of zero, so `NoReward` does not cover it, and widening `NoReward` (option 8(b)) would give one name two reasons. The comparison itself draws at no coin there (option 8(c)), which would run a set exploration that can never draw.
  - The refusals that stood before are read before these: a magnitude below zero, a task's own critic beside the engine's, and a drawn delivery on an engine that does not draw.
- **Under a reward withheld** the exploration is taken: the value is the critic's of the counts, and the trial records the draw and no value, as it records none without the exploration.
- **The tests** (`runtime/cortex-runtime/src/task.rs`):
  - *The two draws against an oracle written apart from the tree*: SplitMix64's finaliser in another language. The coin over the first eight trials and the channel among two, three and four over the first sixteen, at seeds 0 and 27; over 4 096 trials at seed 27, the coins below a quarter, a half and three quarters of the width and each channel among three by the stimulus presented.
  - *The coin against the value*: by hand at a value of zero, one LSB below it, at a half, one LSB above minus the reward, at it and beyond it, at magnitudes of a quarter, 1.0, three LSBs, 3.0 and the width; and over every coin, the count against $\lceil \text{below} \cdot 2^{16} / r \rceil$. A magnitude at or below zero draws at no coin.
  - *The channel*: over every draw of fifteen bits among one, two, three, four, five and sixty-four channels, against the division, in order, with the counts by channel; the sixteenth bit not read.
  - *Over the lattice* (`testkit/prop.rs`): for every seed and trial built from the `u32` lattice's words and over seeded pairs, the coin and the channel's draw by a hand rule over the draw's bytes, each unchanged when the stimulus's bit, the shuffled coin's and the misleading coin's three are flipped and when the other's bits are, and the three draws beside them the ones they were; the comparison against a hand rule in `i128` over the lattice of values and magnitudes and 20 000 seeded triples; the channel against the division for one to sixty-four channels.
  - *The refusals*: each by name, under each feedback, with nothing injected; none with the exploration unset.
  - *Unset, a trial is the trial it was*: `trial` against the trial as ADR-0155 left it, written out as the oracle with its four feedbacks and four deliveries, on twin engines over eight trials under a drive, a cancel and a sub-window. Under the engine's critic every value weight is written at the negative rail, so that a set exploration would draw at every trial a reward is delivered at, 96 of them: unset, none is drawn and the outcome, the units, the train, the address, the signal, the counts, the reading and the weights are the oracle's after every trial.
  - *Set, nothing changes at or above zero*: under each feedback, at a value of zero and above it, with no readout cued, the answer's, the other's and both, the trial with the exploration set is the trial with it unset on a twin engine in every field of the outcome and of the engine.
  - *The edge on the engine*: over eight trials, each on a fresh engine whose weights put the value the engine reads one LSB to either side of the trial's coin's edge. The selection is drawn on one side and the gate's on the other; drawn, it is the trial's channel whatever the gate read, judged and rewarded as a selection; the value recorded is the engine's reading; and the channels hold the gate's reading.
  - *Under each feedback* a drawn selection's reward takes its sign as any selection's does.
  - *Under each delivery*, among three readouts, what is addressed at a drawn selection where the gate selected another channel and where it selected none: the drawn readout's units under the addressed and the drawn deliveries, every unit under the released and the global ones, and the gate's selection's with the exploration unset.
- **Every pinned number unchanged with the exploration unset.** Every task of the tree names the option unset, and no line a trial runs with it unset is new but the read of the option. The calibration of brief 066's measurement reproduces every whole-domain test's pinned numbers ([ADR-0164](0164-an-exploration-the-value-gates-measured.md)).
- **The mutation gate** ([ADR-0030](0030-verification-governance.md)): `cargo-mutants` makes 36 mutants in the changed lines of `src/`. On the developer machine 32 are caught by the unit tests above and four do not compile, the four that replace `Task::trial`'s result. The tool's own run there marked ten more unviable where the linker could not open the test binary (LNK1104, a known fault of that machine); each of the ten was run again, five by the tool and five made by hand, and caught. The pull request's in-diff job is the gate's reading.

### Consequences

- Good: ADR-0162's rule exists in the task with nothing to tune, and reads nothing the engine does not already expose.
- Good: unset, a trial is the trial it was by construction and by a test against that trial written out.
- Good: the gate, the critic, the address, the executor, every record and the image are untouched.
- Good: the probability is exact at the reward every learning run uses, 1.0.
- Neutral: under another magnitude the probability is rounded up by less than one coin of 65 536.
- Neutral: among three the channels' chances differ by one draw of 32 768.
- Bad: the value's composition is written in two places, the executor's reward and the task. A test holds them equal; a change to one that the test does not run would part them.
- Bad: one option more in the task, and a rule that stays in the tree unset if H-32 reads no.
- Bad: a hold and the exploration cannot be set together.

## Alternatives considered and why rejected

- **A flag**, **a record of parameters**, **a method of the executor**, **the value read at the selection**, **a hold composed**, **fewer bits**, **a quotient first**, **a remainder**, **a repeated draw**, **a second selection recorded**, **`NoReward` widened**: see above.
- **Bits beside another draw's** (option 4(c)): the draw's low word holds 31 free bits beside the stimulus's. Any of them would do as well; the second word is the one no draw touches.
- **Noise on the counts, a draw among the channels the gate left, a constant share**: ADR-0162 rejected them, and brief 066 does not empower them.

## Confirmation

- `runtime/cortex-runtime/src/task.rs`: `Exploration`, `Exploration::{draws, channel}`, `EXPLORATION_COIN_BITS`, `EXPLORATION_CHANNEL_BITS` and their compile-time assertion, `Task::exploration`, `Task::{exploration_coin_at, drawn_at}`, `Outcome::drawn`, `TaskError::{ExplorationWithoutCritic, ExplorationWithHold, ExplorationWithoutReward}`, `engine_value`, `Task::{check, trial}`, and the ten tests named above.
- `runtime/cortex-runtime/src/lib.rs`: the three names exported.
- `runtime/cortex-runtime/tests/instrument/harness.rs`, `tests/inhibition.rs`, `tests/learning.rs`: every task names the option unset.
- Whitepaper §6.5 and §9; `CHANGELOG.md`.
