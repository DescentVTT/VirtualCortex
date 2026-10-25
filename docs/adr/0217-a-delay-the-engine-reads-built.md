---
status: proposed
date: 2026-10-25
depends-on: ADR-0216
title: A delay the engine reads from its kept addresses, built
kind: built
summary: '`Config::reads_its_delay`, `Executor::reward_untold` and `Delay::untold`. A critic per kept age, a score of 64 errors, one move latched at a lead of 1/32. Unset, every run is the run it was; a reward that names its age is ADR-0208''s, set or not; on time with no move, the drawn run.'
decision-makers: VirtualCortex maintainers
---

# ADR-0217: A delay the engine reads from its kept addresses, built

**In brief**
- **Decision.** [ADR-0216](0216-a-delay-the-engine-reads.md)'s rule is built in the runtime: `Config::reads_its_delay`, a value critic per kept age, a score per age, one move latched, and `Executor::reward_untold`. The task gains `Delay::untold(k)`.
- **Why.** H-45 asks whether the engine reads a delay of one or two trials it is not told. It needs the rule, with every run unchanged where it is unset, and with every reward that names its age.
- **Left open.** H-45's protocol is [ADR-0218](0218-a-delay-the-engine-reads-its-protocol.md), and its verdict the measured ADR's.

## Context and Problem Statement

ADR-0216 decided the rule, in its words:
- *"The engine carries `kept_addresses` + 1 value critics, one per age, each starting from the critic's weights when the rule starts."*
- At a reward given without an age, *"each age's critic reads its value from the counts of the trial that age closes before the newest, whether that trial's entry is spent or not."*
- *"While the age in use is zero, when one other age has the strictly lowest score at 64 rewards running, each time below every other by at least 64 × a thirty-second of the reward, the engine moves to that age. It never moves again."*

What was read first (principle 2), on 2026-10-25 at `main` `e2d0f7d`:
1. **The kept addresses** (`runtime/cortex-runtime/src/executor.rs`, [ADR-0208](0208-a-reward-past-the-trial-its-address-kept-built.md)). `reward_kept` takes an entry's counts out, reads the value with `read_reward`, and puts them back. A spent entry keeps its counts until a close writes over it.
2. **The critic's weights** are each unit's `value_weight`, an `i16` in the unit's record. `reward_kept` and `reward` move them and nothing else.
3. **Exploration 003's harness** (`untold_arm` on `explore/003-a-delay-the-engine-is-not-told`). The host's critics read a ring of copies of each close's counts. An age with no trial yet read nothing. At a switch's reward the critics read first, then the switch moved, then the engine was rewarded at the new age.

## Decision Drivers

- ADR-0216's rule, and brief 082's standing directives: no record, no section, no format, nothing in the image.
- **Unset, every run the run it was**, bit for bit; **a reward that names its age is ADR-0208's**, set or not.
- **On time with no move, the drawn run**, bit for bit, which H-45's controls hold on the networks.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)), and a new rule over the lattice.

## Considered Options

1. **Where the critics' weights are held**:
   - (a) the age in use's in the units' `value_weight`, the engine's critic, and every other age's in a vector of the executor's, swapped at the move;
   - (b) every age's in a vector, the age in use's copied into the units after each reward.
2. **When the rule starts**: (a) at its first reward given without an age; (b) at `Executor::new`.
3. **The order at a reward**: (a) every critic reads, then the move, then the age in use takes the reward; (b) the age in use takes the reward, then the move.
4. **The lead in fixed point**: (a) from the reward given, $(\lvert r \rvert \cdot 64) \gg 5$; (b) from a constant reward of 1.0.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**
Built in `49cc45e`, in brief 082's pull request.
Brief 082's empowerment names each choice: the field, the call and their errors, how the critics, their windows and the counts of spent entries are held, how the task carries its delay, and how a score and a lead are computed.

- *Why not 1(b)*: a reward that names its age moves the units' weights alone, so a copy of the age in use's critic would drift from the engine's. Under 1(a) it is the engine's by construction, and the rule costs two weights a unit at `kept_addresses` 2, 4 KiB at 1 024 units.
- *Why not 2(b)*: an image's weights are written after `Executor::new`, so the critics would start from zero. At the first reward they start from the engine's weights as they then are.
- *Why not 3(b)*: both lose the same rewards at a move. Under 3(a) the reward that completes the run is the new age's, as exploration 003's switch gave it.
- *Why not 4(b)*: the lead is a thirty-second of the reward. Read from the reward given, it is that whatever the task's magnitude, and equal to 4(b) at H-45's 1.0.

### The delay read (Implemented)

| Item | What it is |
| :--- | :--- |
| `Config::reads_its_delay: bool` | the rule; unset by default; refused without `kept_addresses`, which needs the critic and its window |
| The critics | the age in use's is the engine's, the units' `value_weight`; every other age's a vector of a weight a unit, sized in `Executor::new`, started from the units' weights at the rule's first reward |
| The scores | per age, the magnitudes of its last `DELAY_WINDOW` (64) errors in a ring, and their sum, a `u64`; a score once the ring is full |
| `Executor::reward_untold(reward)` | every age a close has reached reads, records and moves; the move, from age zero only; then the age in use's live entry takes the reward as `reward_kept` takes it; returns the signal, or none where that entry is spent |
| The leader (`delay_leader`) | the age below every other's score by at least the lead, $(\lvert r \rvert \cdot 64) \gg 5$, and strictly; none until every age has a score |
| The run (`delay_run_after`) and the move | the rewards running at which one age other than zero leads, one more when it leads again and one when another such age does; at `DELAY_RUN` (64) its weights and the units' are swapped, and it is the age in use for good |
| `delay_age`, `delay_score`, `delay_run`, `delay_weight`, `reads_its_delay` | the readings: the age in use, each age's score, the run, each age's weight of a unit |
| `KeptEntry::closed` | whether a close wrote the entry since the engine's start, its build or its load; an age whose entry no close wrote reads nothing |

| Refusal | When |
| :--- | :--- |
| `ConfigError::DelayReadWithoutKept` | `reads_its_delay` set with `kept_addresses` 0 |
| `AddressError::DelayUnread` | `reward_untold` while the rule is unset, nothing changed |

A reward whose age in use has no live entry reaches no address and no modulator, and leaves the reading as it was.
At the move that is the move's own reward: age zero spent the entry of the age moved to.
At a move to age $a$, $a$ rewards are lost so.
Under age zero the first $k$ closes of a delay of $k$ are never spent, and a close writes over them.
Nothing of it is in the image, and the format stays 25.

### The task's untold delay (Implemented)

| Item | What it is |
| :--- | :--- |
| `Delay::untold(k)` | trial $t$'s sign at the end of trial $t + k$, given without its age; `Delay::is_told` false; `Delay::of(k)` and `Delay::ON_TIME` told, as before |
| The sign | at $k$ held, the oldest is given by `reward_untold`; a reward that reaches no modulator records no reward and no value, as a trial that gives none |
| `TaskError::DelayUnread` | an untold delay on an engine that does not read its delay |
| `TaskError::DelayUndrawn`, `ExplorationWithDelay` | as for a delay past zero, and for an untold delay at zero too |

### After the calibration's commit

The mutation gate, run on the diff before the pull request, left three survivors of 81 mutants, 72 caught and 6 unviable.
- `<` to `<=` in `delay_leader`'s search for the lowest score was equivalent: at a tie it picked the other tied age, and a tie is no leader either way.
- Two in the run's step were not: no test moved the lead between two ages other than zero during a run.

`a3840cf` rewrites `delay_leader` as a check of each age against every other, and moves the run's step into `delay_run_after`, held at its edges by hand.
Their 31 mutants are caught.
The arms ran on `b418b75`, and their code is the same rule; the reproduction runs every test on `a3840cf`'s code, so that every pinned number is read again on it.

### Its tests

- **By hand at the edges** (`executor.rs`, `delay_prop`):
  - `the_delay_read_is_unset_by_default_and_refused_without_the_kept_addresses`, with a reward before any close reaching nothing;
  - `an_age_leads_below_every_other_by_the_lead_and_none_before_every_score`;
  - `the_run_counts_one_age_leading_again_and_starts_again_at_another`;
  - `the_engine_moves_once_to_the_age_that_leads_and_never_again`: no move before 127 rewards, one move to age one losing its reward, and none in 1 000 rewards after, though age zero then leads.
- **Over the lattice against an oracle** (`delay_prop`): `every_reward_given_without_its_age_is_the_oracle_s`, 48 walks of 400 trials, half learnable, every age's critic, score, the run, the move, the entries, the address, the reading and the signal after every call.
- **The twins** (`delay_prop`):
  - `on_time_with_no_move_a_reward_given_without_its_age_is_the_drawn_reward_bit_for_bit`, 48 walks of 126 trials;
  - `a_reward_that_names_its_age_is_the_one_without_the_rule_bit_for_bit`, 48 walks of 48.
- **The task** (`task.rs`):
  - `a_delay_given_without_its_age_is_refused_on_an_engine_that_does_not_read_it`;
  - `a_sign_given_without_its_age_is_given_at_the_age_the_engine_reads`, at delays 0, 1 and 2 with a flip, against an oracle of the value and the address;
  - `on_time_given_without_its_age_with_no_move_a_trial_is_the_trial_it_was_bit_for_bit`, under the five feedbacks and both deliveries that draw.

## Consequences

- Good: the engine can give a late reward to the trial it is for without being told the delay, by a reading of its own.
- Good: unset, told, and on time with no move, the runs are the runs they were, by construction and by the twins.
- Bad: two value weights a unit more, 4 KiB at 1 024 units, and three critic readings a reward in place of one.
- Bad: the move's own reward, and $a - 1$ after it, reach nothing.
- Neutral: ADR-0216 counted three weights a unit, 6 KiB, with the engine's own; 1(a) holds the age in use's in the record.

## Confirmation

- `runtime/cortex-runtime/src/executor.rs`: `Config::reads_its_delay`, `DELAY_WINDOW`, `DELAY_RUN`, `DELAY_LEAD_SHIFT`, `reward_untold`, `delay_leader`, `delay_run_after`, the readings, and `delay_prop`.
- `runtime/cortex-runtime/src/task.rs`: `Delay::untold`, `Delay::is_told`, `TaskError::DelayUnread` and the tests above.
