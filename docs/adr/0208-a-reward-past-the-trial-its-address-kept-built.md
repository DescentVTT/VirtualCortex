---
status: accepted
date: 2026-10-25
depends-on: ADR-0207
title: A reward past the trial, its address kept, built
kind: built
summary: '`Config::kept_addresses`, `Executor::keep_address`, `Executor::reward_kept` and `Task::delay`. Unset, every run is the run it was; kept and rewarded at age zero, too, under every feedback. The engine holds one entry more than the oldest age, so that a delay of two runs at `kept_addresses` 2 (F-79).'
decision-makers: VirtualCortex maintainers
---

# ADR-0208: A reward past the trial, its address kept, built

**In brief**
- **Decision.** [ADR-0207](0207-a-reward-past-the-trial-its-address-kept.md)'s mechanism is built in the runtime: each trial's close keeps the critic's counts and the targets, and a reward of age $a$ consolidates under the entry kept $a$ closes before the newest. The task carries its delay.
- **Why.** H-43 asks whether a reward one or two trials late teaches under the address its trial kept. It needs the mechanism, with the run unchanged where it is unset or on time.
- **Left open.** H-43's protocol is [ADR-0209](0209-a-reward-past-the-trial-its-address-kept-its-protocol.md), and its verdict the measured ADR's.

## Context and Problem Statement

ADR-0207 decided the mechanism, in its words:
- *"`Config::kept_addresses`, the number of trials whose addresses the engine keeps, 0 unset, at most 4"*;
- *"A trial's close (`Executor::keep_address(targets)`, between ticks)"* keeps the sources, the targets and the counts, zeroes the counts and reopens the critic's window;
- *"A reward for a trial of age a (`Executor::reward_kept(reward, a)`, a below `kept_addresses`)"* writes that entry's address, reads the value and moves the value weights from its counts, and spends it.

What was read first (principle 2), on 2026-10-25 at `main` `97d0c0e`:
1. **The reward** (`runtime/cortex-runtime/src/executor.rs`). `reward` reads the value from the live counts, moves the weights, zeroes the counts in the same loop and reopens the window. `address_drawn` writes the sources from the counts not zero and the targets given.
2. **The task** (`runtime/cortex-runtime/src/task.rs`). `Task::trial` draws the address under the drawn and the released deliveries, whatever the feedback, then gives the reward. Under `Feedback::Withheld` it draws and gives none, so the counts stand.
3. **Exploration 001's arms** (`explore/001-the-old-noes-under-the-default`) gave the reward through the host, after the task had drawn the delivering trial's address.

**F-79.** ADR-0207's bound on the age contradicts its own runs.
- With `kept_addresses` 2 and *"a below `kept_addresses`"*, the ages are 0 and 1.
- A delay of two is a reward at age 2: at the end of trial $t + 2$ the entries of $t$, $t + 1$ and $t + 2$ are all still to be rewarded.
- So ADR-0207's text refuses the delay of two it runs at `kept_addresses` 2.

## Decision Drivers

- ADR-0207's mechanism, and brief 080's standing directives: no record, no section, no format, nothing in the image.
- **Unset, every run the run it was**, bit for bit; **kept and rewarded at age zero**, the same.
- **`kept_addresses` 2 and the delays 1 and 2** are ADR-0207's and do not move.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)), and a new rule over the lattice.

## Considered Options

1. **The age's bound and the entries held** (F-79):
   - (a) `kept_addresses` the oldest age a reward may name, `kept_addresses + 1` entries held, the trial just closed among them;
   - (b) `kept_addresses` entries, the reward given before the delivering trial's close, its age one less;
   - (c) ADR-0207's text, the ages below `kept_addresses`.
2. **The reward's arithmetic**:
   - (a) one function of the counts given, which `reward` calls with the live counts and `reward_kept` with an entry's;
   - (b) a second copy in `reward_kept`.
3. **Under the withheld feedback**, with addresses kept:
   - (a) the close draws as before, since no reward names it;
   - (b) the close keeps.
4. **How the task carries its delay**:
   - (a) a field `Task::delay`, a `Delay` that holds the signs still to come;
   - (b) the host, as exploration 001 gave it.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**
Built in `6e0b4fd`, brief 080's one commit under `src/`, in pull request [#232](https://github.com/DescentVTT/VirtualCortex/pull/232). Its mutation gate caught 55 of the 61 mutants in the changed lines, the other 6 unviable.
Brief 080's empowerment names each choice: the field, the calls and their errors, how the entries are held, and how the task carries its delay.

- *Why not 1(b)*: it holds the same entries and gives the same runs, but its age is not the delay. A reward at age zero would then come before its own close.
- *Why not 1(c)*: it refuses the delay of two at the `kept_addresses` ADR-0207 fixed.
- *Why not 2(b)*: a second copy of the critic's step can drift from the first. One function makes age zero a reward by construction.
- *Why not 3(b)*: the close would zero counts that the withheld feedback leaves standing. Under 3(a) every feedback on time is the trial it was.
- *Why not 4(b)*: the delay is then outside the engine's tests.

### The kept addresses (Implemented)

| Item | What it is |
| :--- | :--- |
| `Config::kept_addresses: u8` | the oldest age a reward may name; 0 unset; refused above `KEPT_ADDRESSES_MAX` (4) and without the critic or its window |
| The entries | `kept_addresses + 1` of them, each the critic's counts, the targets by unit and whether its reward is still to come, sized in `Executor::new`; none while unset |
| `Executor::keep_address(targets)` | the counts and the targets become the newest entry; the counts start from zero and the window opens again; no address, weight or signal moves |
| `Executor::reward_kept(reward, age)` | the entry's address written, the critic's reading from its counts, the entry spent; the live counts and the window untouched; returns the signal |
| `Executor::kept_at(age)` | the counts and the targets kept at an age; none where nothing is kept there |
| `Executor::kept_let_go()` | entries a close wrote over while their reward was still to come |
| `Executor::read_reward` | the critic's reading of the counts given, private, which `reward` and `reward_kept` share |

| Refusal | When |
| :--- | :--- |
| `ConfigError::KeptAddressesOutOfRange` | `kept_addresses` above 4 |
| `ConfigError::KeptAddressesWithoutWindow` | `kept_addresses` set without the critic or its window |
| `AddressError::NoCritic`, `NoWindow` | either call, as `address_drawn` refuses |
| `AddressError::NotKept` | either call while `kept_addresses` is 0 |
| `AddressError::AgeNotKept` | a reward of an age above `kept_addresses` |
| `AddressError::NoKeptAddress` | a reward where no entry is kept at the age: none yet, or spent |
| `AddressError::NoSuchUnit` | a close with a target outside the arena, refused whole |

Nothing of it is in the image, and the format stays 25.
`Image::decode` takes `kept_addresses` from the configuration, as it takes the train's size.

### The task's delay (Implemented)

| Item | What it is |
| :--- | :--- |
| `Delay` | the trials a sign comes late and the signs still to come, oldest first; `Delay::ON_TIME`, `Delay::of(k)` |
| `Task::delay` | the task's `Delay`; every struct literal of the tree sets `Delay::ON_TIME` |
| The close | under the drawn or the released delivery, an engine that keeps addresses and a feedback that gives a sign, `keep_address` in place of `address_drawn` |
| The sign | the trial's earned sign joins those held; at $k$ held, the oldest is given by `reward_kept` at age $k$ |
| No sign due | the trial records no reward and no value, as a withheld one does |
| `TaskError::DelayNotKept` | a delay above the engine's `kept_addresses` |
| `TaskError::DelayUndrawn` | a delay past zero under the global or the addressed delivery |
| `TaskError::ExplorationWithDelay` | a delay past zero with the exploration, which reads the value the trial's own reward is taken against |

A sign is the one its own trial earned, under the mapping of its time: a flip moves no sign held.

### Its tests

- **By hand at the edges** (`executor.rs`):
  - `the_kept_addresses_are_unset_by_default_and_refused_above_four_or_without_the_window`;
  - `a_close_keeps_its_counts_and_targets_and_a_reward_of_its_age_writes_and_spends_them`, an oracle of the value and the delta rule, an entry spent once and refused after, and one let go.
- **Over the lattice against an oracle** (`executor.rs`, `kept_prop`): `every_close_and_every_reward_of_an_age_is_the_oracle_s`, 96 walks of 48 calls under drawn critics, windows, ages and the whole punishment.
- **The twin at age zero**:
  - `a_reward_kept_at_age_zero_is_the_drawn_reward_bit_for_bit`, 64 walks of 32 trials (`kept_prop`);
  - `on_time_under_kept_addresses_a_trial_is_the_trial_it_was_bit_for_bit`, under the five feedbacks, both deliveries that draw and the exploration (`task.rs`).
- **The delay** (`task.rs`):
  - `a_delay_returns_each_sign_at_its_age_and_none_before` by hand, and `a_delay_returns_the_sign_its_trials_before_over_the_lattice`;
  - `a_delay_is_refused_beyond_the_kept_addresses_under_a_delivery_that_draws_none_and_with_the_exploration`;
  - `a_late_sign_is_given_through_the_address_its_trial_kept`, at delays 1 and 2 with a flip, against an oracle of the value and the address.

## Consequences

- Good: a reward can reach the trial it is for through the engine's own address. Unset or on time, nothing else moves.
- Good: the twin at age zero is H-42's arm bit for bit by construction, which ADR-0209's calibration holds on all three networks.
- Bad: the engine holds `kept_addresses + 1` arrays of a count and a flag a unit, 25 KiB at 1 024 units and `kept_addresses` 4.
- Neutral: F-79 is ADR-0207's text. ADR-0207 is accepted and is not edited.

## Confirmation

- `runtime/cortex-runtime/src/executor.rs`: `Config::kept_addresses`, `KEPT_ADDRESSES_MAX`, `keep_address`, `reward_kept`, `kept_at`, `kept_let_go`, `read_reward`, and the tests above.
- `runtime/cortex-runtime/src/task.rs`: `Delay`, `Task::delay`, the three refusals and the tests above.
- Whitepaper §11 records F-79.
