---
status: accepted
date: 2026-10-08
depends-on: ADR-0149
decision-makers: VirtualCortex maintainers
---

# ADR-0151: Three answers — H-28's stopping rule taken at more answers than two, in two steps: first the answers alone, on H-25's configuration, stimuli, schedule and arms, the readout three channels of 204 units where it was two of 459, a channel selected when its count is the largest and none at a tie, each stimulus's answer moving to the next readout at a flip; the third stimulus second, named and not decided; the readouts dealt by a rule from the places both stimuli reach; under two readouts every run the run it was; H-29 written before any run; brief 062 builds it and runs H-29 once

## Context and Problem Statement

[ADR-0149](0149-a-reward-right-seven-times-in-eight-measured.md) read H-28 as yes. Its stopping rule, step 3, left a list of next decisions: a less reliable reward, more answers than two, a reward delayed past a trial, another size and the operating regime. **The maintainers took more answers than two on 2026-10-07, as three stimuli with three answers.**

**Why this task.** Every learning run from H-13 to H-28 chose between two readouts. Three things were never asked of the machinery under that condition:
- **a selection among more than two.** The readout hands two counts to `cortex-basal-ganglia`'s gate, each channel's own as its direct drive and the other's as its indirect drive.
- **a punishment that does not name the answer.** With two readouts, a readout punished down leaves the answer. With three it leaves two.
- **a chance of a third.** At the start a selection is right in one trial of three, so rewards are rarer and the critic's value starts below zero.

**What a third stimulus changes beside the answers**, read against the tree on 2026-10-08:
- **The geometry's rule** ([ADR-0065](0065-the-instrument-recalibrated.md), `tests/instrument/harness.rs`): every stimulus unit lies at least nine places from every other on the ring, beyond the prior's window of eight, so that no stimulus drives another through a local synapse. A period holds one unit of each stimulus, so three stimuli need a period of at least 27. The period is a multiple of five, so that no stimulus unit is one the prior makes inhibitory, and at most 32, the width of a set's mask. That leaves 30.
- **At 1 024 units a period of 30 is 34 whole periods**, so a stimulus is 34 units where it is 51, and they are other units.
- **Three things then move with the stimulus's units:**
  - the critic's step. A value is the stimulus's units' weights summed over four, and a step moves each weight by the error over 512, so one presentation moves the value by $N / 2048$ of the error: 51 units give about a fortieth, and 34 a sixtieth ([ADR-0132](0132-a-critic-of-the-engines-own-measured.md)'s arithmetic).
  - the cancel, which [ADR-0076](0076-two-injections.md) derived from the ticks the volley of those 51 units spreads over;
  - the schedule. A stimulus is presented in a third of the trials and not in half, so H-25's schedule gives a mapping two thirds of the presentations, or is lengthened.
- **H-25's runs would no longer stand beside the new ones** block for block.

So three stimuli with three answers is five changes at once at this size: the answers, the stimulus's units, the critic's step, the cancel and the schedule. A no would not say which of them it read. **This ADR takes the maintainers' choice in two steps**, as [ADR-0138](0138-the-address-drawn.md) took the address: the answers first, with everything else H-25's, and the third stimulus second.

**The rules the first step acts through**, read on 2026-10-08:
- **The gate** (`crates/cortex-basal-ganglia/src/lib.rs`): `compute_gating` selects a channel when its indirect drive plus its hyperdirect drive less its direct drive is below zero.
- **The readout** (`runtime/cortex-runtime/src/task.rs`): `Readout` holds two sets and two channels. `select` gives each channel its own count as the direct drive and the other's as the indirect, and returns the one channel selected, or none.
- **The mapping**: `Task::mirrored`, a flag. Stimulus `s` is rewarded at readout `s`, or at the other. A flip of the schedule negates it (`run_on_scheduled`).
- **The prior** (`crates/cortex-connectome/src/prior.rs`): of a unit's 32 synapses a quarter go anywhere on the ring, and each of the others goes to one of the sixteen places within the window, each place with the same chance (`Prior::target`). Every fifth unit, from the fifth, is inhibitory.
- **The two-answer geometry**: a period of twenty from rotation 0. Stimulus A is place 0 and stimulus B place 11. Readout 0 is the nine odd places but 11, and readout 1 the nine even places but 0: 459 units each, 102 of them inhibitory (F-63). The census counts 775 to 809 synapses from a stimulus into a readout (`SYNAPSES_1024`), where the rule above expects 795.

**The account and the literature.** In the model of Gurney, Prescott and Redgrave (2001) the basal ganglia select among any number of channels: the channel with the largest salience is released and the others are held. Bogacz and Gurney (2007) read the same circuit as a decision among several alternatives. A learner driven by a prediction error learns such a choice, more slowly as chance falls (Frank 2005). The account predicts that the configuration learns three answers.

**Where the engine's premise differs:**
- the selection is a comparison of spike counts with no noise of its own, and no selection at a tie (H-17);
- the readouts are sets of one network of 1 024 units, so a third answer means smaller readouts: fewer synapses from a stimulus into each and fewer units counted;
- integer arithmetic, and one seeded prior.

## Decision Drivers

- **A measured need**: the configuration is named, and every reading of it is a choice between two.
- **The maintainers' choice** (2026-10-07) of three stimuli with three answers.
- **One change from H-25**: the number of answers. The stimuli, the image, the critic, the window, the address, the schedule, the seed and the arms are H-25's, so H-25 stands beside the new runs block for block.
- **The geometry is derived, not chosen**, by ADR-0065's rule.
- **Unset, bit for bit**: under two readouts every run is the run it was.
- **Literature is a prior**: the account predicts a yes, and the measurement decides.
- **The weekly job's budget**: twelve shards since [ADR-0150](0150-a-round-waits-for-what-it-checks.md), each planning about 3 730 s.
- Latest ≠ Newest: no dependency, no tool, no rule of a state crate.

## Considered Options

1. **The steps**: (a) the answers first with H-25's two stimuli, the third stimulus second; (b) three stimuli and three answers in one round.
2. **A channel's indirect drive among three**: (a) the largest of the other channels' counts; (b) their sum.
3. **The readouts**: (a) three of four places each, dealt from the places both stimuli reach; (b) three of six places each, dealt from every place that is not a stimulus's; (c) the two readouts kept and a third added from another part of the ring.
4. **The mapping at a flip**: (a) every stimulus's answer moves to the next readout; (b) two answers are swapped and the third readout is never an answer.
5. **The schedule and the arms**: (a) H-25's, 7 680 trials with three flips, from the assignment and from the mirrored assignment; (b) a longer one for a choice among three.
6. **The clauses**: (a) H-28's four, with clause 1 held per stimulus as well; (b) H-28's four as they stand.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The selection among three (Specified; brief 062 builds it)

- **The rule.** Each channel's direct drive is its own set's count. Its indirect drive is the largest of the other channels' counts, and its hyperdirect drive is zero. `compute_gating` decides each channel, unchanged.
- **What it gives.** A channel is selected when its count is above every other's. When the largest count is shared, no channel is selected, as at a tie between two.
- **With two channels** the largest of the others is the other, so the rule is the one in place and every run under two readouts is the run it was, bit for bit.
- **Where it lives.** In the runtime's task, as the two-channel composition does. No record, no rule of a state crate and nothing of the image changes.

### The mapping (Specified)

- **An answer for each stimulus**, a readout's index, in place of the flag. Under two readouts the flag's two values are the answers (0, 1) and (1, 0).
- **At a flip every stimulus's answer moves to the next readout**, the last to the first. With two readouts that is the flip in place.
- **H-29's two arms** are H-25's two starting assignments, each through H-25's three flips:
  - from the assignment: (0, 1), (1, 2), (2, 0), (0, 1);
  - from the mirrored assignment: (1, 0), (2, 1), (0, 2), (1, 0).

  The two arms hold all six ways of giving two stimuli different answers among three. At every flip one stimulus moves onto the readout the other is leaving, and the other onto the readout that was no one's answer.

### The readouts, derived (option 3(a))

- **The stimuli are ADR-0065's, unchanged**: places 0 and 11 of a period of twenty from rotation 0, 51 units each.
- **A place is within a stimulus's window** when it is at most eight places from it on the ring: places 1 to 8 and 12 to 19 for A, and 3 to 10 and 12 to 19 for B.
- **Fourteen places are within both**: 3 to 8 and 12 to 19. Three of them, 4, 14 and 19, are places the prior makes inhibitory.
- **Each readout is four of the fourteen, one of them inhibitory.** Every readout is then 204 units, 51 of them inhibitory, and each stimulus unit has four places of each readout in its window.
- **Six places are in no set**: 1, 2, 9, 10 and the two shared places the deal leaves.
- **Why not more places.** A readout that took a place only one stimulus reaches would be seen by the two stimuli unequally, or the three readouts would differ in size. Equal sizes and equal sight together allow only the shared places, and fourteen places hold three readouts of four.
- **The deal is the first that passes**, as ADR-0065's rotation is. The deals that satisfy the rule above are enumerated in an order written before any coupling is read. The deal taken is the first whose six stimulus–readout couplings, read as summed weights on the settled image the arms start from, are equal within ten per cent with none zero.

**What the readouts cost the instrument**, by the prior's rule:
- a stimulus's synapses into a readout: $51 \times (24 \times 4/16 + 8 \times 204/1024) \approx 387$, where the two-answer geometry has 795;
- a readout's units, and so its count from the background: 204 where it was 459.

A readout's response to a stimulus is about half what it was, and its background count four ninths. The calibration reads whether the readout still sees the stimulus.

### H-29 (Hypothesis; written before any run)

**On H-25's configuration, stimuli, schedule and arms, with three readouts and the mapping above:**
- **clause 1, the learning holds**: in both arms every mapping is learned. At least 80 of its last 128 trials are correct, and among them each stimulus selects its answer in more than half of its presentations.
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the six stimulus–readout couplings lies within 0.75 and 1.25 of the image's;
- **clause 4, the critic holds the expected reward**: for each stimulus and each mapping over its last 128 trials, the engine's mean value lies within a quarter of the reward of $(2p - 1)$ of it, with $p$ that stimulus's accuracy over those trials.

**Yes** when all four hold in both arms; otherwise **no**, with the clauses that failed.

**No prediction for the verdict.** The account predicts clause 1. What it does not hold is the instrument: whether readouts of 204 units resolve a learned answer from two others, and whether the couplings that takes stay under a bound read on readouts of 459.

**Predicted readings:**
- **(a) a later start.** The first mapping passes 40 of 64 later than H-25's sixth and fourth block, since a selection is right a third of the time at the start and not half.
- **(b) a value below zero first.** Before a mapping is learned each stimulus's value falls toward minus a third of the reward, $(2p - 1)$ at an accuracy of a third.
- **(c) elimination after a flip.** The old answer's readout is selected until its pairs are punished back toward the image's, as H-18 read. From there the selections split between the new answer and the third readout, and the third readout's pairs fall below the image's.

**Readings, no clause:**
- the block each mapping passes 40 of 64 in, beside H-25's;
- per mapping, the selections by readout for each stimulus (the answer, the old answer, the third) and the ties;
- after each flip, the stimulus that moves onto the other's old answer beside the one that moves onto the free readout;
- the six couplings' courses, and the value beside $(2p - 1)$ with its troughs;
- where the consolidation went and the inhibitory sum's course, beside H-25's;
- on the frozen block, each readout's count before and after the volley, and the selections by readout.

### H-29's stopping rule

1. **One round**: brief 062 builds the selection and the mapping, their tests and their ADR, then runs H-29. The order of the deals, the deal taken and H-29's constants are committed before the first rewarded run.
2. **A calibration** stops the round before any rewarded run, and is a finding, if any of these fails:
   - every pinned number of the tree holds, the two-answer runs among them;
   - a deal passes the readouts' rule;
   - on a frozen block of 64 trials from the settled image, the window after the volley holds more readout spikes than the window before the injection in at least 56, ADR-0065's mark.
3. **Yes**: the configuration learns and revises a choice among three. The next decision is the second step, an ADR on the third stimulus, with a less reliable reward on three answers, a reward delayed past a trial, another size and the operating regime beside it, named and not taken.
4. **No on clause 3**: a selection among three moves the network the learning stands on. The next decision is an ADR on what it reaches.
5. **Otherwise no on clause 1**: the configuration does not learn three answers on this instrument. The next decision is an ADR choosing between the selection, an exploration among its candidates, and the readout's resolution, with this round's readings of the ties and of where the wrong selections went as its need.
6. **Otherwise no on clause 2**: the answers are learned with couplings past the two-answer bound. The next decision is an ADR on whether the bound is the configuration's or the readout's size.
7. **Otherwise no on clause 4 alone**: the engine learns the choice without holding its expected reward. The next decision is an ADR on the critic's step at a chance of a third.
8. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: the first reading of the configuration on a choice among more than two, with one thing changed and H-25 beside it block for block.
- Good: the readouts are what a rule gives, and the deal is the first that passes it.
- Good: under two readouts nothing changes, and no rule of a state crate is touched.
- Bad: this is half of what the maintainers chose. Three stimuli is a second round, and its geometry gives a stimulus of 34 units.
- Bad: a third answer makes every readout smaller at this size, so the number of answers and the instrument's resolution change together. The calibration and the readings on the frozen block are there to tell them apart.
- Bad: with two stimuli one readout is no stimulus's answer in each mapping. Nine couplings that are all some stimulus's answer in some mapping is the second step.
- Neutral: files under `src/` change, so the dispatch's scope is `both` ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)). Two weekly tests of about H-25's cost, 1 817 and 1 837 s in the table, raise it from 44 765 s to about 48 400, about 4 030 s a shard of twelve.

## Alternatives considered and why rejected

- **Option 1(b), three stimuli and three answers at once**: the number of answers, the stimulus's size, the critic's step, the cancel and the schedule would move together, and H-25 would not stand beside the run. A yes would be worth as much; a no would not say which change it read.
- **Option 2(b), the sum of the others as the indirect drive**: a channel would be selected only when its count is above the two others together. Counts of 10, 6 and 6 would select nothing, so most trials would be ties before anything is learned, and under the drawn address a tie consolidates nothing.
- **Option 3(b), six places each**: eighteen places are no stimulus's, but four of them are within one stimulus's window only. The readouts would then differ in size or in what each stimulus sees of them.
- **Option 3(c), a third readout elsewhere on the ring**: it would be outside both stimuli's windows, reached only by the quarter of the synapses that go anywhere.
- **Option 4(b), a swap with the third readout never an answer**: the third readout would be a distractor only, and no mapping would ask a stimulus to move onto a readout that was no one's.
- **Option 5(b), a longer schedule**: it would be another schedule than H-25's. H-25's slowest reversal took 21 blocks of a mapping's 32; whether three answers fit is what clause 1 reads.
- **Option 6(b), clause 1 as it stands**: 80 of 128 is met by one stimulus answered every time and the other at chance, 64 and about 21.

## Confirmation

`briefs/062_three-answers.md` builds the selection and the mapping and runs H-29 once. Whitepaper 4.95.0 carries H-29 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
