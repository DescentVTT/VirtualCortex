---
status: accepted
date: 2026-10-09
depends-on: ADR-0153
decision-makers: VirtualCortex maintainers
---

# ADR-0154: A punishment the value does not soften — H-29's stopping rule taken at the reading it handed beside its three: after a flip the critic's value falls below zero and a punishment of the old answer delivers a third to an eighth of the reward; a parameter of the image under which a reward below zero is delivered whole where the value is below zero, the critic's own error and every other case unchanged; unset every run the run it was; H-30 written before any run on H-29's readouts, schedule and arms, with the revision held to H-25's 23 blocks; brief 063 builds it and runs H-30 once

## Context and Problem Statement

[ADR-0153](0153-three-answers-measured.md) read H-29 as no, on clause 1: with three readouts the first mapping was learned as between two, and a reversal took 21 to 31 of a mapping's 32 blocks, where H-25's took 14 to 21. Its stopping rule, step 5, names an ADR choosing between the selection, an exploration among its candidates and the readout's resolution. ADR-0153 handed that ADR a fourth reading, which the rule's list does not name: **the size of a punishment under the critic's value**. The maintainers took the fourth on 2026-10-09.

**The sweep of that round's dispatch**, which the merge did not wait for ([ADR-0150](0150-a-round-waits-for-what-it-checks.md)): run [37810004245](https://github.com/DescentVTT/VirtualCortex/actions/runs/37810004245) ended at 19:46:25Z on 2026-10-08, two hours after the merge, green in all seven of the sweep's jobs.
- **3 805 mutants over the tree, 3 629 caught and none missed**: 2 398 caught in the state crates and 1 231 in the runtime, 154 unviable and 22 timeouts.
- **In `runtime/cortex-runtime/src/task.rs`**, the file ADR-0152 changed: 177 caught, 10 unviable, none missed and none timed out.
- **The 22 timeouts** are mutants of `cortex-core`'s three chain iterators, of the injector, of the barrier's wait and of the workers' stop and run, the places [ADR-0062](0062-the-first-complete-sweeps-list.md) reads as inherent.
- There is no survivor, so the round leaves no finding and no list for this one.

**What ADR-0153 read:**
- After a flip a stimulus's value fell to −0.58 to −0.95 of the reward, and its block mean stayed below minus half the reward for 5 to 24 blocks.
- Over a mapping after a flip, the trials that selected the old answer delivered a mean error of −0.31 to −0.67 of the reward. In the slowest reversals most of them delivered less than a quarter of it, and in blocks 9 to 16 an eighth to a fifth.
- The old answer took 62 to 90 per cent of a stimulus's wrong selections and led for 5 to 24 blocks.
- The two fastest reversals, 14 blocks each, are the two whose punishments stayed at two thirds of the reward.

**The rules this acts through**, read on 2026-10-09:
- **The critic** (`Executor::reward`, `cortex-neuromod`'s `ValueCritic`): at a reward the value is the units' value weights times their counted spikes, scaled. The error is the reward less the value. Every counted unit's weight moves by the error, and **the modulator receives the error in the reward's place**.
- **The signal** (`NeuromodulatorState::reward`, `decay_dopamine`): the error is added to the dopamine signal, which decays by $2^{-14}$ of itself a tick. A trial is $2^{14}$ ticks, so $e^{-1}$ of a trial's signal stands at the next reward (F-53).
- **The signed gate** (`SynapseBlock::consolidate_signed`): an addressed synapse moves by its trace times the signal, the signal clamped to $[-1, 1]$. Below zero the weight moves against the trace.

**The arithmetic of a punishment under a value below zero**, with the reward's magnitude 1:
- The error is $-1 - V$. At a value of −0.6 it is −0.4, and at −0.95 it is −0.05.
- A run of equal errors leaves the signal at the error over $1 - e^{-1}$, 1.58 times it: −0.63 and −0.08.
- A whole punishment, −1, leaves −1.58, which the gate clamps at −1.
- So at each consolidation the old answer's pairs move between about a twelfth and about two thirds of what a whole punishment moves them, by the gate's rule.

**The two-answer record bears on the account, and does not settle it:**
- H-18 had no critic, so every punishment was whole. Its old answer's pairs stood at about 1.4 of the image's at the flip, and a reversal took 14 to 20 blocks ([ADR-0096](0096-the-punished-pair-measured.md)).
- H-25 has the critic. Its pairs stood at 1.19 at most, its punishments were softened as above, and a reversal took 14 to 21 blocks.
- Read together: the critic left less to unlearn and less to unlearn it with, and between two answers the two cancelled.
- Among three the selections that leave the old answer also go to the third readout and to nothing. The accuracy recovers more slowly, the value stays low for longer, and the softening lasts.

**What the reading does not hold.** No run has varied the critic. The twelve rows of ADR-0153 also fit the reverse order: a reversal that is fast for another reason keeps the value up, and its punishments are large because of it. Only a run with the softening removed tells the two apart.

**The account and the literature.**
- With a critic of a state's value, an outcome the critic expects delivers no error, and a learner taught by the error alone stops changing while it keeps choosing the same wrong action (Sutton and Barto 2018, on the critic as a baseline).
- Bayer and Glimcher (2005) read dopamine neurons as encoding errors above zero in proportion and errors below zero only down to a floor.
- Daw, Kakade and Dayan (2002) proposed that the prediction of punishment is an opponent system's, not the reward signal's.

The account predicts that a punishment which is not softened shortens a reversal.

**Where the engine's premise differs:**
- the signal is a signed integer with the same range on both sides, and the gate clamps it at ±1;
- the signal carries over from trial to trial, so what a punishment delivers is also what the next reward meets;
- the selection is a comparison of counts with no noise of its own (H-17);
- one seeded prior at 1 024 units, and readouts of 204.

## Decision Drivers

- **A measured need**: ADR-0153's readings of the punishment after a flip, on twelve reversals.
- **The maintainers' choice** (2026-10-09) among the rule's three and the fourth.
- **An intervention on the suspected cause**: the one run that tells the account from its reverse.
- **One change from H-29**: what a punishment delivers where the value is below zero.
- **Unset, bit for bit**, and unchanged wherever the value is at or above zero, which is where every yes since H-23 was read.
- **Literature is a prior**: the account predicts a yes, and the measurement decides.
- Latest ≠ Newest: no dependency, no tool. One rule of the critic, a parameter of the image.

## Considered Options

1. **What is changed**:
   - (a) a reward below zero is delivered whole where the value is below zero;
   - (b) the value is floored at zero for every outcome;
   - (c) the value itself never falls below zero;
   - (d) a punishment is never compared with the value;
   - (e) the critic's step.
2. **What moves the critic's weights**: (a) the reward less the value, as now; (b) what the modulator received.
3. **Where it lives**: (a) a rule of `cortex-neuromod` beside the critic's, set by a parameter of the image; (b) the task.
4. **The run**: (a) H-29's readouts, schedule and arms; (b) those and H-25's two-answer arms.
5. **The clauses**: (a) H-29's four and the revision in time; (b) H-29's four.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

### The rule (Specified; brief 063 builds it)

With the parameter set, at a reward $r$ against a value $V$:
- **the critic's error is $r - V$, and it moves the weights**, as it does now;
- **the modulator receives $r - V$, unless $r$ and $V$ are both below zero; then it receives $r$.**

Read case by case:

| Reward | Value | The modulator receives | Now |
| :--- | :--- | :--- | :--- |
| below zero | below zero | $r$ | $r - V$, less than $r$ in magnitude |
| below zero | at or above zero | $r - V$ | the same |
| at or above zero | any | $r - V$ | the same |

- **Unset, every run is the run it was**, bit for bit.
- **Set, nothing changes where the value is at or above zero.** That is the regime of a learned mapping.
- **The value still holds a stimulus's expected reward**, below zero as above it, since the weights move by the critic's own error.
- **Where it lives.** The rule is `cortex-neuromod`'s, beside `ValueCritic::error_q16`. The executor's reward path calls it. The parameter is a flag in the modulator section's reserved bytes, so the image's format goes to 21. It is refused without the critic.

### H-30 (Hypothesis; written before any run)

**On H-29's configuration, readouts, schedule and arms, with the parameter set:**
- **clause 1, the learning holds**: in both arms every mapping is learned. At least 80 of its last 128 trials are correct, and among them each stimulus selects its answer in more than half of its presentations.
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end.
- **clause 3, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the six couplings lies within 0.75 and 1.25 of the image's.
- **clause 4, the critic holds the expected reward**: for each stimulus and each mapping over its last 128 trials, the engine's mean value lies within a quarter of the reward of $(2p - 1)$ of it.
- **clause 5, the revision is in time**: each of an arm's three reversals passes 40 of 64 within 23 blocks of its flip, the crossing block counted.

Clauses 1 to 4 are H-29's. Clause 5 is H-25's clause 3, the two-answer bound: H-25 read 14 to 21 blocks, and H-29 read 31, 31 and 21 from the assignment and 30, 23 and 23 from the mirrored assignment.

**Yes** when all five hold in both arms; otherwise **no**, with the clauses that failed.

**Predicted: yes.** By the gate's rule a whole punishment moves the old answer's pairs one and a half to twelve times what the softened ones moved them, and the couplings it has to bring down are H-25's size. What the prediction does not hold is the reverse order above, and what a signal held at its floor for many trials does to the pairs of the two other readouts.

**Predicted readings:**
- **(a) the old answer lets go sooner.** At each of the twelve flips and stimuli the old answer leads for fewer blocks than in H-29, where it led for 5 to 24.
- **(b) every reversal is faster than its own in H-29**: fewer blocks than 31, 31 and 21, and than 30, 23 and 23.
- **(c) the value recovers sooner.** A stimulus's block mean is below minus half the reward in fewer blocks of a mapping than H-29's 5 to 24.
- **(d) the old answer's coupling ends every mapping after a flip below the image's.** In H-29's failed mapping it ended at 1.010.

**Readings, no clause:**
- per mapping, the punished trials whose value was below zero: how many, the error the critic took and what the modulator received;
- the signal at each reward, and the trials it stood at or beyond the gate's bounds;
- where the wrong selections went, the ties and the margin, beside H-29's;
- the six couplings' courses, the third readout's pairs among them, beside H-29's;
- where the consolidation went and the inhibitory sum's course, beside H-29's.

### H-30's stopping rule

1. **One round**: brief 063 builds the rule, its tests and its ADR, then runs H-30. The parameter's place in the image and H-30's constants are committed before the first rewarded run.
2. **A calibration** stops the round before any rewarded run, and is a finding, if any of these fails:
   - with the parameter unset, every pinned number of the tree holds, H-29's arms among them; the whole-image pins a format moves are re-pinned with it, and nothing else is;
   - with the parameter set, each arm is H-29's arm trial for trial up to the first trial at which a reward below zero meets a value below zero, and differs from it there.
3. **Yes**: a punishment softened by the value is what the revision among three lacked. The next decision is an ADR on whether the parameter joins the learning configuration, with H-25's and H-28's two-answer arms read under it, and then on ADR-0151's second step. It is named and not taken.
4. **No on clause 3**: a whole punishment moves the network the learning stands on. The next decision is an ADR on what it reaches.
5. **Otherwise no on clause 1**: the punishment's size is not what the revision lacks. The next decision is the one ADR-0153's step 5 names, among the selection, an exploration and the readout's resolution, with both rounds' readings.
6. **Otherwise no on clause 5**: every mapping is learned and a reversal among three is still slower than between two. The next decision is an ADR choosing between an exploration and the schedule a choice among three is given.
7. **Otherwise no on clause 2**: the answers are learned and revised with couplings past the bound. The next decision is an ADR on the bound at this readout's size.
8. **Otherwise no on clause 4 alone**: the next decision is an ADR on the critic's step.
9. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: the run that tells ADR-0153's account from its reverse, with one thing changed and H-29 beside it trial for trial until the rule first acts.
- Good: nothing changes where the value is at or above zero, so no reading since H-23 is moved by the parameter's presence.
- Good: the critic stays a predictor. Clause 4 reads the same quantity it has read since H-23.
- Bad: a parameter more in the image, a format more, and a rule that stays in the tree unset if H-30 is a no.
- Bad: the two-answer configuration is not read under the parameter in this round. A yes names it as the next decision's first question.
- Bad: one form of the rule. A floor above or below zero is not tried.
- Neutral: files under `src/` and a state crate change, so the dispatch's scope is `both` ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)). Two weekly tests of about H-29's cost, 915 and 1 041 s in the table, raise it from 46 803 s to about 48 800, about 4 060 s a shard of twelve.

## Alternatives considered and why rejected

- **Option 1(b), the value floored at zero for every outcome**: a reward that comes where a punishment was expected would deliver 1 and not $1 - V$. After a run of whole punishments the signal stands near −0.58 at the next reward. A reward of 1 leaves it at 0.42, where a reward of 1.6 to 1.95 leaves it at the gate's ceiling. The new answer's pairs would consolidate under less than half of what they consolidate under now, and the round would change two things.
- **Option 1(c), a value that never falls below zero**: the value would stop holding a stimulus's expected reward where the accuracy is below a half, which clause 4 has read since H-23.
- **Option 1(d), a punishment never compared with the value**: where the value is above zero a punishment delivers more than itself, and H-28 read what that does. It would move the regime every yes since H-23 was read in.
- **Option 1(e), the critic's step**: a slower step delays the softening and a faster one hastens it, and either moves how the value holds the expected reward. ADR-0153's step 7 names it for a no on clause 4.
- **Option 2(b), the weights moved by what the modulator received**: under a run of punishments the value would fall without the reward as its bound, to the weights' rail.
- **Option 3(b), in the task**: the critic is the engine's since [ADR-0131](0131-the-critic-built.md). The task hands the reward and does not see the value before it is taken.
- **Option 4(b), the two-answer arms in the same round**: two questions, and four weekly tests where two answer the one asked.
- **Option 5(b), H-29's four clauses alone**: H-29 failed clause 1 by three presentations in one mapping of eight. A yes by as little would not say that the punishment's size was the cause. The revision's speed does.
- **The rule's three candidates** (the selection, an exploration, the readout's resolution): each stays open. An exploration is the account's other remedy and the larger design; step 5 and step 6 name it.
- **A longer schedule**: every mapping after a flip was still improving at its end, so it would likely pass, and nothing would have been changed.

## Confirmation

`briefs/063_a-punishment-the-value-does-not-soften.md` builds the rule and runs H-30 once. Whitepaper 4.97.0 carries H-30 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
