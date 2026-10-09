---
status: accepted
date: 2026-10-09
depends-on: ADR-0161
decision-makers: VirtualCortex maintainers
---

# ADR-0162: An exploration the value gates — the decision three stopping rules name, taken after H-29, H-30 and H-31: with the task's exploration set, a trial's selection is drawn among the readout's channels with a probability equal to the part of the engine's own value that lies below zero, and is the gate's otherwise; no constant to choose, no rule of the engine changed, unset every run the run it was; the regime it settles in derived before any run; H-32 written on H-29's readouts, schedule and arms with no prediction for the verdict; a no on the learning stops the line among three answers; brief 066 builds it and runs H-32 once

## Context and Problem Statement

[ADR-0161](0161-the-span-a-choice-among-three-is-given-measured.md) read H-31 as no. Its stopping rule, step 5, names *"the ADR on an exploration that ADR-0153's step 5 and ADR-0156's steps 5 and 6 name, with three rounds' readings"*. The maintainers took it on 2026-10-09.

**No sweep is owed.** Brief 064's round changed no file under `src/` and dispatched the whole-domain shards alone ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)). The sweep before it is on record in [ADR-0160](0160-the-sweeps-tests-under-nextest.md).

**What three rounds read of a choice among three:**

| Round | The one change | What was read |
| :--- | :--- | :--- |
| H-29 ([ADR-0153](0153-three-answers-measured.md)) | three readouts | the first mapping learned as between two; a reversal slow, 21 to 31 of 32 blocks; seven mappings of eight learned |
| H-30 ([ADR-0156](0156-a-punishment-the-value-does-not-soften-measured.md)) | a punishment the value does not soften | the first reversal faster, 19 and 22 blocks; the two after it lost, the six couplings' sum sinking to 0.93 |
| H-31 (ADR-0161) | a mapping after a flip of 48 blocks | the first reversal's mapping learned; the second slower, 46 and 37 blocks; the third lost by one stimulus; six mappings of eight |

**The one thing all three read:** after a flip the gate goes on selecting the old answer, and the new answer is selected only once the old answer's pairs have been punished down to it.
- **In H-29** each stimulus first selected its new answer within 4 to 133 trials of the flip *"and then rarely chosen for ten to twenty blocks"*, while the old answer took 62 to 90 per cent of the wrong selections.
- **The punishment that unlearns the old answer is softened by the value**, to a fifth of the reward in H-31's slowest rows. Delivered whole, it wears every coupling down (H-30).
- **A mapping held for longer is harder to leave**: the old answer's coupling at the flip and the blocks it then leads rise together, with a rank correlation of 0.91 over twenty rows of two rounds (H-31).
- **The readouts were not what failed**: the largest count, the ties and the margin read in H-31 as in H-29 or better.

**The rules an exploration acts through**, read on 2026-10-09:
- **The selection** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`, `Readout::select`): the readout's counts over the task's window go to `cortex-basal-ganglia`'s gate, and the channel with the largest count alone is selected, none where it is shared. The selection is a function of the counts: it has no noise of its own (H-17).
- **The value** (`Executor::critic`, `Executor::features`, each unit's `value_weight`): the critic's value is its units' weights times their spikes within the critic's window, scaled. The window closes 100 ticks into a trial and the selection is read at the trial's end, so **the value the reward will be taken against is already determined when the selection is made**, and the task can read it from what the executor already exposes.
- **The address** (`Delivery::Drawn`): its targets are the units of the readout the selection chose.
- **The task's draw**: one `mix64` of the seed and the trial's index. Bit 0 is the stimulus, bit 32 the shuffled control's coin, and bits 48 to 50 the misleading coin.

**The account and the literature.**
- A learner that always takes its best-valued action learns nothing about the others. The standing remedy is to take another action some of the time (Sutton and Barto 2018).
- Exploration that rises as the expected outcome falls is the adaptive-gain account of the locus coeruleus (Aston-Jones and Cohen 2005). In a model of the basal ganglia, tonic dopamine sets how sharply the largest channel wins (Humphries, Khamassi and Gurney 2012).

The account predicts that a selection which sometimes leaves the largest count finds a new answer without first unlearning the old one.

**Where the engine's premise differs:**
- the noise is the task's seeded draw, not the network's activity, so a run stays a function of its seed;
- the probability is the value itself, with no temperature to set;
- a reward on a drawn selection consolidates the trace of a readout that did not win the count. H-13 read that rewarding an assigned readout's pairs raises them whatever was selected, on readouts of 459 units; on readouts of 204 it has not been read;
- integer arithmetic, one seeded prior at 1 024 units.

## Decision Drivers

- **Three measured rounds** that read the same thing, and three stopping rules that name this decision.
- **The maintainers' choice** (2026-10-09).
- **One change from H-29**: the selection. The image, the readouts, the deal, the schedule, the seed and the arms are H-29's.
- **Nothing to tune**: the probability is derived from the value and the reward.
- **Unset, bit for bit**, and no rule of the engine touched: the gate, the critic and the address are as they are.
- **A bounded commitment**: the fourth reading of a choice among three is the last without a new reading that names another.
- **Literature is a prior**: two predictions of a yes were wrong in a row (ADR-0154, ADR-0157). This ADR predicts no verdict.
- Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **What gates the exploration**: (a) the engine's value at the trial, where it is below zero; (b) a constant share of the trials; (c) the dopamine signal the last reward left; (d) the task's own count of recent outcomes.
2. **How much**: (a) a probability equal to the value's part below zero over the reward's magnitude; (b) a constant chosen by a calibration.
3. **What an explored selection is**: (a) one of the readout's channels with equal chance, whatever the counts; (b) one of the channels the gate did not select; (c) noise added to the counts.
4. **Where it lives**: (a) the task, beside the selection it composes; (b) the gate, in `cortex-basal-ganglia`; (c) the network, as a drive into the readouts.
5. **The schedule**: (a) H-25's, as H-29; (b) ADR-0157's longer one.
6. **The clauses**: (a) H-29's four; (b) H-30's five, with a reversal's time.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The rule (Specified; brief 066 builds it)

With the task's exploration set, at a trial's selection:
- **$V$ is the engine's own value**: its critic's value of the counts its window holds, the value the trial's reward is then taken against;
- **$p$ is the part of $V$ below zero, over the reward's magnitude $r$**: $p = \min(\max(-V, 0), r) / r$;
- **with probability $p$ the selection is one of the readout's channels, each with the same chance**, whatever the counts; otherwise it is the gate's;
- **the coin and the draw are bits of the trial's own draw** that the stimulus, the shuffled control and the misleading coin do not read.

Everything after the selection is as it is: a trial is correct when the selection is the stimulus's answer, the address's targets are the selected readout's units, and the reward's sign is the outcome's.

- **Unset, every run is the run it was**, bit for bit.
- **Set, nothing changes where the value is at or above zero**: the selection is the gate's.
- **Where it lives.** In the task, which composes the selection. It reads the value from what the executor exposes. No rule of the engine, no record, nothing of the image and no format changes. It is refused without the engine's critic.

### The regime it settles in, derived before any run

With the reward's magnitude 1, a stimulus answered correctly with probability $a$ has an expected reward of $2a - 1$, which the critic's value holds (H-23).

- **Until the value passes zero there is no exploration.** After a flip a learned stimulus's value stands near 0.9. A presentation moves it by a fortieth of the error ([ADR-0132](0132-a-critic-of-the-engines-own-measured.md)'s arithmetic), so punished at every presentation it passes zero after about 25, three quarters of a block.
- **While the gate still selects the old answer**, a drawn selection is right once in three, so $a = p/3$ and the value tends to $2p/3 - 1$. With $p = -V$ that settles at $p = 3/5$:
  - the value at −0.6 of the reward;
  - the new answer selected in one presentation of five, and rewarded with an error of 1.6, which the signed gate clamps at its ceiling;
  - the old answer selected in three of five, two by the gate and one by the draw, and punished with an error of −0.4;
  - the third readout in one of five, punished with −0.4.
- **Once the gate selects the new answer**, $a = 1 - 2p/3$ and the value tends to $1 - 4p/3$, which is above zero for any $p$ under three quarters. So $p$ falls to zero: the exploration switches itself off.

**Beside what H-29 and H-31 read of the same phase**: the new answer rarely chosen for ten to twenty blocks, where the regime gives it a fifth of the presentations from the second block; and the punishment of the old answer at a fifth to a third of the reward, where the regime holds it at two fifths, since the value settles at −0.6 and not near −0.9.

### H-32 (Hypothesis; written before any run)

**On H-29's configuration, readouts, schedule and arms, with the exploration set:**
- **clause 1, the learning holds**: in both arms every mapping is learned. At least 80 of its last 128 trials are correct, and among them each stimulus selects its answer in more than half of its presentations.
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end.
- **clause 3, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the six couplings lies within 0.75 and 1.25 of the image's.
- **clause 4, the critic holds the expected reward**: for each stimulus and each mapping over its last 128 trials, the engine's mean value lies within a quarter of the reward of $(2a - 1)$ of it, with $a$ that stimulus's accuracy over those trials.

These are H-29's four, by H-29's rules.

**Yes** when all four hold in both arms; otherwise **no**, with the clauses that failed.

**No prediction for the verdict.** The derivation above says what the rule does to the selections and to the value. It does not say how fast a reward on a drawn selection raises a readout of 204 units from its image's coupling past an old answer's at 1.2, nor whether the couplings then stay under 1.30.

**Predicted readings**, from the derivation:
- **(a) the new answer is selected early.** Over the second to the eighth block after each flip, each stimulus selects its new answer in more of its presentations than in H-29's same blocks.
- **(b) the value settles higher.** After a flip no stimulus's block mean value falls below −0.75 of the reward, where H-29's fell to −0.58 to −0.95.
- **(c) every reversal is faster than its own in H-29**: fewer blocks to 40 of 64 than 31, 31 and 21, and than 30, 23 and 23.
- **(d) the exploration switches itself off.** Over each learned mapping's last 128 trials fewer than one trial in ten is drawn.
- **(e) the couplings are not worn down.** The six couplings' sum stands at or above 0.99 of the image's at each mapping's end, where H-30's sank to 0.93.

**Readings, no clause:**
- per block, the trials drawn, the value at them, and the drawn selections by the readouts' roles and by outcome;
- what a reward on a drawn selection consolidated in the pair it reached, beside a reward on a selection of the gate's;
- the blocks the old answer leads, the old answer's coupling at each flip and at each mapping's end, beside H-29's;
- the six couplings' courses and their highest; the ties and the margin of the gate's own reading;
- the inhibitory sum's course, beside H-29's.

### H-32's stopping rule

1. **One round**: brief 066 builds the exploration in the task, its tests and its ADR, then runs H-32. The bits it reads and H-32's constants are committed before the first rewarded run.
2. **A calibration** stops the round before any rewarded run of the protocol, and is a finding, if either fails:
   - with the exploration unset, every pinned number of the tree holds, H-29's arms among them;
   - with it set, each arm is H-29's arm trial for trial up to the first trial at which the value is below zero and the coin draws, and differs from it there.
3. **Yes**: a selection that leaves the largest count is what the revision among three lacked. The next decision is an ADR on whether the exploration joins the learning configuration, with H-25's and H-28's two-answer arms read under it, and then on ADR-0151's second step. It is named and not taken.
4. **No on clause 3**: a reward on a drawn selection moves the network the learning stands on. The next decision is an ADR on what it reaches.
5. **Otherwise no on clause 1**: **the line stops among three answers.** Four rounds will have read no. The next decision is an ADR that states what the learning configuration holds of a choice among three — a first choice, and a revision that is slow and does not hold over three — and returns the line to the list H-28's stopping rule left: a less reliable reward, a reward delayed past a trial, another size and the operating regime. No further mechanism is tried on three answers without a reading that names it.
6. **Otherwise no on clause 2**: the answers are learned and revised with couplings past the bound. The next decision is an ADR on the bound at this readout's size.
7. **Otherwise no on clause 4 alone**: the next decision is an ADR on the critic's step under a selection that is sometimes drawn.
8. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: the decision three stopping rules name, in a form with nothing to tune and a regime derived before the run.
- Good: the gate, the critic, the address and the image are untouched, and unset nothing changes.
- Good: the line among three answers ends at this round whichever way it reads: a yes moves on, and a no on the learning stops it.
- Bad: the noise is the host's draw. An exploration the network makes itself is not attempted, as the address's neural target side was not kept ([ADR-0146](0146-the-tag-the-address-already-is.md)).
- Bad: one form. A draw among the channels the gate did not select, or noise on the counts, is not read.
- Bad: the two-answer configuration is not read under it in this round. A yes names that as the next decision's first question.
- Neutral: a file under `src/` changes, so the dispatch's scope is `both` ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)); the merge waits for the whole-domain shards and not for the sweep, whose outcome the first ADR that merges after it ends writes down (ADR-0158). Two weekly tests of about H-29's cost.

## Alternatives considered and why rejected

- **Option 1(b), a constant share of the trials**: a learned mapping would lose that share for good, and the share is a number to choose.
- **Option 1(c), the dopamine signal**: it is what the last trial left, and the last trial presented either stimulus. The value is the present stimulus's.
- **Option 1(d), the task's count of outcomes**: the host would again hold what the engine's critic already holds ([ADR-0138](0138-the-address-drawn.md)).
- **Option 2(b), a constant by calibration**: a second thing to fit, on the runs the hypothesis is read from.
- **Option 3(b), a draw among the channels the gate did not select**: where nothing is selected it has no meaning, and it makes the draw depend on the gate's reading. By the same derivation it settles at a probability of a half and gives the new answer a quarter of the presentations where the rule above gives a fifth, which is a matter of degree.
- **Option 3(c), noise on the counts**: its size against a count of five to ten spikes is a constant to choose.
- **Option 4(b), in the gate**: `compute_gating` is a rule of a state crate with no draw to read, and a state crate takes none from the host.
- **Option 4(c), a drive into the readouts**: it moves the readouts' activity and with it the traces, the counts and the inhibitory rule. H-27 read what moving the readouts' activity costs.
- **Option 5(b), the longer schedule**: H-31 read that the span is not what the revision lacks and that it deepens what the next revision undoes. H-29 on H-25's schedule is the run this one stands beside.
- **Option 6(b), a reversal's time as a clause**: 23 blocks is the two-answer bound. The time is read, as predicted reading (c).
- **Stopping the line among three now**: three rounds name the same missing thing, and none has tried it. Step 5 stops the line if this one reads no.

## Confirmation

`briefs/066_an-exploration-the-value-gates.md` builds the exploration and runs H-32 once. Whitepaper 4.102.0 carries H-32 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
