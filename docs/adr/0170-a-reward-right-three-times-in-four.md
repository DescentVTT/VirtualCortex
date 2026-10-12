---
status: accepted
date: 2026-10-12
depends-on: ADR-0169
decision-makers: VirtualCortex maintainers
---

# ADR-0170: A reward right three times in four — H-34's stopping rule taken at a less reliable reward: H-28's task with the reward's sign the outcome's in three trials of four and its opposite in one, the coin two of the misleading coin's three bits so that every trial misleading at seven in eight is misleading here; H-25's configuration, network, schedule and arms; H-28's four clauses with the expected reward at a half; the naive account and H-28's reading give opposite answers, so no prediction for the verdict and three predicted readings; a no on the learning split beforehand between slower than the schedule and not revised; H-35; brief 069 builds the feedback and runs it once

## Context and Problem Statement

[ADR-0169](0169-twice-the-size-measured.md) read H-34 as yes: at twice the size, on two networks, the rules derived the learning configuration and it learned and revised as at 1 024 units. Its stopping rule, step 3, names an ADR choosing among a larger size still, the operating regime, a reward delayed past a trial and a less reliable reward. **The maintainers took a less reliable reward on 2026-10-12.**

**No sweep is owed.** Brief 068's dispatch ran the whole-domain shards alone ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)). The schedule's weekly run of 2026-10-12 had not started when this was written; its sweep is the next ADR's to record.

**What H-28 read at seven in eight** ([ADR-0149](0149-a-reward-right-seven-times-in-eight-measured.md)), on H-25's configuration, schedule and arms:
- **Yes on all four clauses**: every mapping learned, 107 to 124 of its last 128; the couplings bounded, the highest 1.187; the network held; the critic holding three quarters of $2p - 1$.
- **The reversals took 1.13 of H-25's on average**, where the naive scaling says four thirds. The slowest took 24 of its 32 blocks.
- **The second mapping was the weakest**, 113 and 107 of 128.
- **Why the slowdown was under the naive one**: the critic's value settles lower, so a true reward still delivers an error and moved the answer's pair 1.3 to 2.0 times what H-25's moved it.
- **What stands against it**: a misleading punishment took from the answer's pair 1.9 and 2.0 times what a true reward gave, with the gate's clamp passing on no more than −1 in 68 per cent of them. One correct trial in eight was punished, and they took back 27 and 28 per cent of what true rewards gave.
- **The learning signal** was 0.87 to 1.03 of H-25's, where the naive scaling says 0.75.

ADR-0149 handed the next decision this: *"A less reliable reward on this schedule is not ruled out by this reading"*, and *"Which of the two is the larger there is not read."*

**[ADR-0147](0147-a-reward-right-seven-times-in-eight.md)'s arithmetic at three in four.** With the reward true with probability $q = 3/4$:
- the expected reward of a stimulus answered correctly with probability $p$ is $(2p - 1)(2q - 1)$, **a half of $2p - 1$**;
- one correct trial in four is punished, and one wrong selection or tie in four rewarded;
- naively the learning signal is halved and a reversal takes twice as long.

ADR-0147 set three in four aside by that arithmetic, before H-28 was read: H-25's reversals of 14 to 21 blocks become 28 to 42, and the schedule gives a reversal 32 and asks 80 of its last 128 trials.

**Two accounts, which give opposite answers:**

| | What it says of a reversal at three in four | Of H-25's six, 14 to 21 blocks |
| :--- | :--- | :--- |
| The naive scaling | twice as long | 28 to 42 blocks: three of the six past the 32 a reversal is given, and a fourth at its last block |
| H-28's reading, carried in proportion | H-28 read 0.39 of the naive excess, 0.13 of a third; of the naive excess here, one whole, that is 1.39 | 19 to 29 blocks: inside the 32, the slowest at their edge |

Neither is a derivation of the engine. **What a misleading punishment takes back** is the third thing: at H-28's cost, 1.95 of a true reward's gain, one correct trial in four punished takes back 65 per cent of what the three true rewards give, where it took back 28. That cost should itself move, since the value settles at a half of $2p - 1$ and not three quarters, a true reward's error is larger and the punishment's is clamped; how far is not derived.

**The literature.** The probabilistic reversal task is usually run at four in five (Cools et al. 2002; Frank 2005), between the two shares read here. A learner driven by a prediction error holds it and reverses more slowly as the reward is less reliable. That is a prior for the engine, not a verdict: the gate's clamp, integer arithmetic and a dopamine signal that carries over from trial to trial are the engine's own.

## Decision Drivers

- **The maintainers' choice** (2026-10-12): a less reliable reward.
- **One change from H-28**: the coin's share. The image, the critic, the window, the address, the schedule, the arms and the network, seed 22 at 1 024 units, are H-25's, so the run stands beside H-28 and H-25 block for block.
- **The misleading trials are nested**: every trial that misled at seven in eight misleads here, and others beside them.
- **Unset, bit for bit**: a feedback of the task beside the four it has.
- **A no is to say which no**: slower than the schedule, or not revised.
- **Predictions**: ADR-0165 and ADR-0168 wrote none, and both read yes; ADR-0154's and ADR-0157's were wrong. Two accounts disagree here, so this ADR predicts no verdict, and writes the readings it does expect.
- **The weekly job's budget**: the table after ADR-0169 plans 46 to 51 per cent of a shard's bound.
- Latest ≠ Newest: one feedback of the task; nothing else is adopted.

## Considered Options

1. **The reliability**: (a) three in four; (b) five in eight; (c) thirteen in sixteen, by a fourth bit; (d) several in one round.
2. **The coin**: (a) two of the misleading coin's three bits, both zero; (b) bits no draw has read.
3. **The schedule**: (a) H-25's, as it is; (b) each mapping given twice the blocks, the naive scaling's; (c) both.
4. **The network**: (a) seed 22 at 1 024 units, H-28's; (b) one at 2 048 units; (c) several.
5. **The clauses**: (a) H-28's four, with the expected reward at a half; (b) those and a reversal's bound.
6. **A no on the learning**: (a) split by a reading written beforehand; (b) one branch.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The feedback (Specified; brief 069 builds it)

- **The rule.** The reward's sign is the outcome's in three trials of four and its opposite in one, whatever the outcome was: a correct selection is punished in one trial of four, and a wrong one or a tie is rewarded in one of four.
- **The coin** is bits 48 and 49 of the task's draw for the trial, two of the three `MISLEADING_BITS`. The reward is misleading when both are zero, whatever bit 50 is.
- **Nested.** Seven in eight misleads when bits 48, 49 and 50 are all zero. Every such trial misleads here, and so do the trials where bit 50 alone of the three is set.
- **Where it lives.** A feedback of the task beside `Answer`, `Shuffled`, `Withheld` and `SevenInEight`. Under the four, every run is the run it was, bit for bit, H-28's arms among them. Nothing of the engine, the image or the records changes.
- **What "correct" means under it**: the selection equals the stimulus's answer under the mapping in force, whatever reward the trial then received.

### H-35 (Hypothesis; written before any run)

**On H-25's configuration, schedule and arms, with the reward right three times in four:**
- **clause 1, the learning holds**: in both arms every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the four stimulus–readout couplings lies within 0.75 and 1.25 of the image's;
- **clause 4, the critic holds the expected reward**: for each stimulus and each mapping over its last 128 trials, the engine's mean value lies within a quarter of the reward of $(2p - 1) \cdot 1/2$ of it, with $p$ that stimulus's accuracy over those trials.

These are H-28's four, with the reliability's factor a half where it was three quarters.

**Yes** when all four hold in both arms; otherwise **no**, with the clauses that failed.

**No prediction for the verdict.** The naive scaling says no on clause 1, in the mappings after a flip. H-28's reading carried in proportion says yes, at the schedule's edge. The round tells them apart.

**Three readings are predicted**, each by both accounts; a miss is recorded as one:
1. **the first mapping is learned in both arms**, at least 80 of its last 128. It starts from the image with no answer to undo, H-28 learned it in 6 and 4 blocks as H-25 did, and it is given 24;
2. **the six reversals are slower than H-28's on average**: the mean of their ratios to H-25's is above 1.13, a reversal that does not cross counted at its 32 blocks;
3. **the value is nearer a half of $2p - 1$ than $2p - 1$**, for each stimulus in each mapping that clause 1 holds for.

**What clause 4 tells apart here.** At a full accuracy a half of $2p - 1$ is 0.5 and the band of a quarter ends at 0.75, so a value at $2p - 1$ fails it; at seven in eight the band held both. At an accuracy near a half both are near zero and the band holds either.

**Readings, no clause**, each beside H-28's and H-25's:
- the reversal speeds, beside twice H-25's and beside 1.39 of them;
- per mapping, the trials by outcome and by the reward received: true rewards, true punishments, misleading rewards and misleading punishments;
- what a misleading punishment moved in the answer's pair beside a true reward, and a misleading reward in the pair it reached beside a true punishment; the share of the true rewards' gain that misleading punishments took back;
- a true reward's error over a learned mapping's last 128 trials, and what it moved;
- how often a reward left the signal beyond the gate's bounds, by kind, and by how much;
- the learning signal over H-25's, beside the naive half;
- the value beside a half of $2p - 1$ and beside $2p - 1$, and its troughs after each flip;
- the trials that mislead here and did not at seven in eight, and the first block in which the run leaves H-28's;
- the couplings' separation and the inhibitory sum's course.

### H-35's stopping rule

1. **One round**: brief 069 builds the feedback, its tests and its ADR, then runs H-35. The coin's bits and H-35's constants are committed before the first rewarded run.
2. **A calibration** stops the round before any rewarded run, and is a finding, if any of these fails:
   - every pinned number of the tree holds, H-28's two arms under `SevenInEight` among them;
   - H-25's first block reproduces under the true feedback;
   - over the run's 7 680 trials the coin by itself is misleading in between 1 760 and 2 080 of them, a share of one in four within a twelfth either way;
   - every trial of the 7 680 that is misleading at seven in eight is misleading here.
3. **Yes**: the configuration learns and revises under a reward right three times in four. The next decision is an ADR choosing among a reward less reliable still, a reward delayed past a trial, the operating regime and a larger size, named and not taken.
4. **No on clause 3**: a misleading reward moves the network the learning stands on. The next decision is an ADR on what it reaches.
5. **Otherwise no on clause 1 or 2**, told apart by a reading:
   - **(a) slower than the schedule**: clause 2 holds, the first mapping is learned in both arms, and each of the six reversals reaches a block with 40 of its 64 trials correct inside its 32. The configuration revises under this reward and the schedule does not hold the revision. The next decision is an ADR on the span a mapping is given under an unreliable reward, with what H-31 read of a longer span among three, and on what holds a learned mapping against the punishments it meets.
   - **(b) not revised**, in every other case. The configuration does not hold a reward wrong one time in four, and the boundary lies between this share and seven in eight. The next decision is an ADR on a single error's weight, the gate's clamp and the critic's step among its candidates, with this round's readings by kind as its need.
6. **Otherwise no on clause 4 alone**: the engine learns the task without holding its expected reward. The next decision is an ADR on the critic's step under noise.
7. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: the second share read. With seven in eight beside it the two give a slope, where one gave a point.
- Good: the two accounts are told apart by a run and not by argument.
- Good: the misleading trials are nested, so what the added ones did can be read against H-28 from the first block that differs.
- Good: a no says which no, by a rule written before the run.
- Bad: one share, one network and one schedule. A yes does not say where the boundary is, and a no says only that it lies above three in four.
- Bad: the schedule is at the edge by one account and past it by the other. A no of the first kind is as much the schedule's as the engine's, and the stopping rule says so.
- Bad: a build under `src/`, so the dispatch runs the sweep's thirteen jobs after the shards.
- Bad: two more arms in the weekly job. The table goes from 79 490 s to about 83 100, about 3 480 to 3 870 s of wall time a shard at the last run's ratios, 48 to 54 per cent of the bound.
- Neutral: the clamp's part is read from what the run does, as in ADR-0149, and not from a run without it.

## Alternatives considered and why rejected

- **Option 1(b), five in eight**: at H-28's cost of a misleading punishment, three correct trials in eight punished take back more than the five rewarded give. It is the step after this one, if this reads yes.
- **Option 1(c), thirteen in sixteen**: a fourth bit for a share between the two. H-28's reading already says seven in eight is not at the schedule's edge, and a smaller step tells the two accounts apart less.
- **Option 1(d), several in one round**: one hypothesis a round, its arms run once.
- **Option 2(b), bits no draw has read**: the misleading trials would be other trials than H-28's, and the run could not be read against it trial for trial.
- **Option 3(b), each mapping given twice the blocks**: the span is not neutral. H-31 read that a longer span bought the first reversal among three and slowed the ones after it, and H-20 read the couplings still rising at each flip, so a longer mapping moves clause 2 as well. It is two changes and twice the cost. **Option 3(c)** is four arms for the same.
- **Option 4(b), 2 048 units**: H-28 stands at 1 024 on seed 22, and an arm at twice the size is half a shard's bound. **Option 4(c)** is more arms before one has read.
- **Option 5(b), a reversal's bound as a clause**: H-25's 23 blocks were read under a reward that never lies. ADR-0147 made the speeds a reading for that reason, and they are one here.
- **Option 6(b), one branch for a no**: a mapping a few blocks short of its last 128 and a mapping never revised would send the line to the same ADR.

## Confirmation

`briefs/069_a-reward-right-three-times-in-four.md` builds the feedback and runs H-35 once. Whitepaper 4.108.0 carries H-35 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
