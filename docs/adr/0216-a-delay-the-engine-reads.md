---
status: accepted
date: 2026-10-25
title: A delay the engine reads from its kept addresses
kind: decision
hypothesis: H-45
summary: With the rule set, the engine carries a value critic per kept age, each learning from its own age's counts, and moves once to the age whose error leads. H-45 asks it at one and two trials' delay on seeds 22, 23 and 24, beside a control on time, predicted yes.
depends-on: ADR-0208
decision-makers: VirtualCortex maintainers
---

# ADR-0216: A delay the engine reads from its kept addresses

**In brief**
- **Decision.** With `Config::reads_its_delay` set, the engine carries one value critic per kept age, each predicting from and learning on its own age's counts. It consolidates at age zero until another age's error leads for 64 rewards running by a thirty-second of the reward, then moves to that age once and stays. H-45 asks it at one and two trials' delay, beside a control on time, predicted yes.
- **Why.** H-43 left a delay the engine is not told. Exploration 003 read the kept counts naming the delay early in every arm, and every first move of such a switch going to the true age.
- **Left open.** A delay that changes during a run, and a delay past two trials.

## Context and Problem Statement

[ADR-0210](0210-a-reward-past-the-trial-its-address-kept-measured.md) read H-43 yes: with the addresses kept, a reward one or two trials late teaches, when it names the age of the trial it is for. [ADR-0207](0207-a-reward-past-the-trial-its-address-kept.md) left open *"a delay the engine is not told"*. [ADR-0214](0214-three-in-four-on-the-default-measured.md) named it next.

**What exploration 003 read** ([its note](../explorations/003-a-delay-the-engine-is-not-told.md), exploratory and not evidence). On H-42's networks, with `kept_addresses` 2, the host kept a critic per age 0, 1 and 2, each predicting from and learning on its own age's counts. An age's score was the mean magnitude of its errors over the last 64 rewards.
- In all eight arms on seed 22, told and untold, the true age's score was the lowest at each of the first 64 rewards scored, trials 65 to 128. In the untold arms the network had learned nothing then.
- In all twenty arms, a run of 64 rewards with the true age lowest was complete by trial 235.
- A switch from age zero to an age leading by a thirty-second for 64 rewards running moved first to the true age in all twelve arms, on seeds 22, 23 and 24: by trial 309 in eight, at 1 448 to 4 269 in four.
- With no latch it moved a second time in two arms, to a wrong age, each while a mapping was rising steeply.
- 43 mappings of 48 were learned. Where the switch moved early and stayed, every mapping was learned and all four of H-43's clauses held.
- In the told arms a wrong age led over 64 rewards running by at most 0.018 of the reward, under a thirty-second.
- The lead was chosen on seed 22's arms after they ran; seeds 23 and 24 ran it unchanged.

**What the engine holds today** (`runtime/cortex-runtime/src/executor.rs`): one value critic, its weights an `i16` a unit (`cortex-neuromod`'s `ValueCritic`); the kept entries, each trial's counts and targets (`kept_at`, `keep_address`, `reward_kept`, [ADR-0208](0208-a-reward-past-the-trial-its-address-kept-built.md)). A reward names its age. Neither the entries nor the counts are in the image.

## Decision Drivers

- **The maintainers' choices** (2026-10-25): the engine's own reading, latched after one move; the lead a thirty-second; three networks at delays of one and two, with a control on time.
- **The exploration's two failures** were second moves. A latch keeps the first, which was right in every arm.
- **The control on time** is the case where any move is a mistake. A thirty-second is above every wrong age's lead the told arms read.
- **Latest ≠ Newest**: the engine's own critic, kept once per age; no dependency.

## Considered Options

1. **The choice**: (a) latched after one move; (b) a second move at twice the lead; (c) unlatched, as the exploration.
2. **The lead**: (a) a thirty-second; (b) a sixty-fourth.
3. **The runs**: (a) delays 1 and 2 on three networks, a control at 0 on each; (b) the control on seed 22 alone.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**

### The delay read (Specified; brief 082 builds it)

- **`Config::reads_its_delay`**, unset by default. It requires `kept_addresses` and the critic. The name is brief 082's to change.
- **The critics.** The engine carries `kept_addresses` + 1 value critics, one per age, each starting from the critic's weights when the rule starts.
- **A reward the host gives without an age** (`Executor::reward_untold(reward)`, between ticks): each age's critic reads its value from the counts of the trial that age closes before the newest, whether that trial's entry is spent or not. It records its error's magnitude, then moves its weights by the error times those counts.
- **The age in use** is zero at the start. The engine writes that age's address, sends that age's error to the modulator, and spends its entry. Its critic is the engine's critic. A reward whose age in use has no live entry reaches no address and no modulator; the critics still learn.
- **The score** of an age is the sum of its error magnitudes over its last 64 rewards. Scores are compared once every age has 64.
- **The move.** While the age in use is zero, when one other age has the strictly lowest score at 64 rewards running, each time below every other by at least 64 × a thirty-second of the reward, the engine moves to that age. It never moves again.
- **Unset, every run is the run it was**, bit for bit. A reward that names its age (`reward_kept`) is H-43's, whether the rule is set or not.
- **Not in the image**, as the entries and the counts are not. The format stays 25.
- **The task** gains a delay it does not tell: trial t's sign at the end of trial t + k, given without its age.

### The runs (Specified; brief 082 builds them in the tests)

On each of seeds 22, 23 and 24 at 1 024 units, from the image an engine born under `Config::reads_its_targets()` settles (H-42's derivations, pinned), with `kept_addresses` 2 and the rule set, in H-25's two arms on H-20's schedule:
- **the arms**: the sign given one trial late and two, without its age; twelve arms;
- **the controls**: the sign given on time, without its age; six arms;
- eighteen in all.

### H-45 (Hypothesis; written before any run)

The text, with its clauses, readings and stopping rule, is [H-45's report](../research/H-45.md).
- **Clause 1**: in each of the twelve arms, the engine moves once, to the true age, before the last mapping begins, trial 5 632.
- **Clause 2**: in each of the twelve, H-25's four clauses over every mapping and reversal after the mapping the move comes in.
- **Clause 3**: in each of the six controls, the engine never moves, and the arm is H-42's arm on its network bit for bit.
- **Yes** when all three hold; otherwise **no**, with the network, the arm, the delay and the clause.

**Predicted: yes.** Every first move in the exploration went to the true age, and the latch keeps it.

**Three readings are predicted:**
1. the move comes inside the first mapping, by trial 1 536, in at least eight arms of twelve;
2. in each arm whose move comes inside the first mapping, all four of H-25's clauses hold over the whole arm;
3. in those arms, the reversals' summed blocks are within 1.25 of the told arm's, H-43's at the same delay.

**What would refute it:**
- a wrong age leading first, before the true age's run is complete, on a network the lead was not chosen on;
- a wrong age leading by a thirty-second on time, when the misaddressed critics of ages 1 and 2 learn from older counts;
- consolidations at age zero before the move leaving the network away from the mapping, past what the rest of the arm can undo.

### H-45's stopping rule

1. **One round**: brief 082 builds the rule and the task's untold delay, with their tests and ADR, then runs H-45 once. Its clauses, constants and readings are committed before any arm runs.
2. **A calibration stops the round**, and is a finding, if a pinned number moves with the rule unset, or if a reward that names its age is not H-43's with the rule set.
3. **Yes**: the engine reads a delay of one or two trials it is not told. The next decision is the re-read of the targets by evidence, at M12.
4. **No on clause 1**: an ADR on the reading, its window, its run and its lead.
5. **No on clause 3**: an ADR on the lead when there is no delay.
6. **Otherwise no on clause 2**: an ADR on what the consolidations before the move leave in the network.
7. **No constant moves after such a run.** There is no second attempt, and no other delay, lead, arm or network.

## Consequences

- Good: a reward late by a delay the engine is not told can reach the trial it is for, by a reading of the engine's own.
- Good: on time the rule is the run it was while it does not move, which the control holds bit for bit.
- Bad: three value weights a unit where the engine held one, 6 KiB at 1 024 units, and three critic readings a reward.
- Bad: the mappings before the move are learned under age zero's consolidation, and some are lost.
- Bad: eighteen arms more in the weekly job, about 27 000 s on x86-64 at H-43's 1 100 to 1 900 s an arm.
- Neutral: no record, no section and no format changes.

## Alternatives considered and why rejected

- **Option 1(b), a second move at twice the lead**: one constant more, and the exploration's wrong moves came while a mapping was rising, when a lead grows.
- **Option 1(c), unlatched**: the exploration followed a wrong age twice.
- **Option 2(b), a sixty-fourth**: a wrong age led by 0.018 in the told arms, above a sixty-fourth, and a latched wrong move on time would never be undone.
- **Option 3(b), one control**: a rule that must not move on time is held on every network it runs on.

## Confirmation

`briefs/082_a-delay-the-engine-reads.md` builds the rule and runs H-45 once.
[H-45's report](../research/H-45.md) holds the hypothesis as written before any run.
`npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
