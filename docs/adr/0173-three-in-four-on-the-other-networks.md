---
status: accepted
date: 2026-10-12
depends-on: ADR-0172
decision-makers: VirtualCortex maintainers
---

# ADR-0173: Three in four on the other networks — H-35's stopping rule taken with its need read first: the no at three in four turned on five trials of one mapping on one network, so before a rule of the engine is changed the same share is asked of the two other networks of the size, seeds 23 and 24, each under the configuration its rules derived, with the task's seed and so the misleading trials the same; H-35's four clauses on each; no prediction for the verdict, a forecast by H-35's own ratios and four predicted readings; nothing under `src/` changed; H-36; brief 070 runs it once

## Context and Problem Statement

[ADR-0172](0172-a-reward-right-three-times-in-four-measured.md) read H-35 as no. Its stopping rule, step 5(b), names *"an ADR on a single error's weight, the gate's clamp and the critic's step among its candidates, with this round's readings by kind as its need."* This is that ADR. **The maintainers chose on 2026-10-12 to read the need on the other networks before a rule of the engine is changed.**

**Two sweeps are running and neither had ended when this was written**: the one of brief 069's dispatch, run [38228081172](https://github.com/DescentVTT/VirtualCortex/actions/runs/38228081172), and the schedule's weekly on `main`, run [38234325824](https://github.com/DescentVTT/VirtualCortex/actions/runs/38234325824). By [ADR-0158](0158-the-sweep-off-the-next-decisions-path.md) the first ADR merged after each ends records it.

**How near the no stands.** On seed 22 at 1 024 units, under a reward right three times in four:
- seven mappings of eight were learned. The eighth read 75 of its last 128 where 80 are asked, and another read 80;
- five reversals of six crossed 40 of 64 inside their 32 blocks. The sixth had its best block at 38, in its 31st, with its counts still rising;
- the rule that splits a no read *not revised*, two trials of one block from *slower than the schedule*.

ADR-0172 says of it: *"the verdict turns on five trials of one mapping and the branch on two trials of one block. A second network or seed could read either the other way; none was run."*

**The candidates the stopping rule names, and what H-35 read of each:**

| Candidate | What was read | What the readings do not give |
| :--- | :--- | :--- |
| the gate's clamp | a true reward left the signal above the ceiling in 47 per cent of them, and the answer's pair gained a trial what it gained at seven in eight; a misleading punishment left it below the floor in 52 and 53 per cent, a misleading reward above the ceiling in 55 and 56 | a wider clamp passes on all three; which gains more is not derived |
| a single error's weight | a misleading punishment took 1.3 of what a true reward gave, and they took back 42 and 44 per cent; misleading rewards put back 41 and 43 per cent of what true punishments took from the old answer's pair | a smaller weight averages over more trials, on a schedule the learner is already slow for |
| the critic's step | the value held a half of $2p - 1$ within 0.22 of the reward in every cell | nothing says the critic's step is what failed |
| the span a mapping is given | the mapping that failed was rising at its end | H-31 read that a longer span is not neutral |

Each of the first two is a rule of the engine, and by these readings each could help or harm. Among three answers the line built four rounds on one boundary ([ADR-0165](0165-another-network-of-the-same-size.md)).

**What the other networks read under the true reward** ([ADR-0167](0167-another-network-of-the-same-size-measured.md)): on seeds 23 and 24 at 1 024 units, each under the configuration its rules derived, H-25's two arms learned every mapping, 115 to 126 of its last 128. Their reversals:

| Network | Blocks to 40 of 64 after each flip, the two arms | Mean |
| :--- | :--- | ---: |
| Seed 22 (H-25) | 19, 21, 16 and 20, 15, 14 | 17.5 |
| Seed 23 (H-33) | 15, 13, 13 and 16, 14, 12 | 13.8 |
| Seed 24 (H-33) | 21, 12, 15 and 17, 16, 14 | 15.8 |

**The task's seed is the same on every network**, 27. So the stimuli are the same trial for trial, and so are the 1 913 misleading trials at three in four, 363, 514, 515 and 521 by mapping. What differs is the network and the constants its rules derived.

## Decision Drivers

- **The maintainers' choice** (2026-10-12): the other networks first.
- **A measured need before a rule of the engine changes.** One network's edge is not yet a need.
- **One change from H-35**: the network. Seen from H-33, one change too: the reward's reliability.
- **The networks are in the tree**, with each derivation pinned and each arm under the true reward pinned beside it.
- **A round without a build.** No file under `src/` changes.
- **Predictions**: none for the verdict. What H-35's own ratios forecast is written, and the readings that are expected.
- Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **What is done first**: (a) the same share on the other networks; (b) a longer span on seed 22; (c) the gate's clamp widened; (d) a smaller weight on one error; (e) the critic's step.
2. **The networks**: (a) seeds 23 and 24 at 1 024 units; (b) those and the two at 2 048; (c) seeds not yet read.
3. **The clauses**: (a) H-35's four on each network; (b) one verdict pooled over the networks.
4. **The predicted reading of the value**: (a) restated here to leave out the cells near chance; (b) as ADR-0170 wrote it.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

### The runs (Specified; brief 070 builds them in the tests)

On each of the two networks, ADR-0044's generator at 1 024 units from seed 23 and from seed 24:
- the network derived again by its rules and held to ADR-0167's pins, and H-25's image of it;
- H-25's two arms on H-20's schedule under `Feedback::ThreeInFour`, the task's seed 27;
- the coin's count held to H-35's, 1 913 in all and 363, 514, 515 and 521 by mapping.

**Nothing under `src/` changes.** Under the true reward these arms are H-33's, so each run stands beside its own network's, block for block, and beside H-35's on seed 22.

### H-36 (Hypothesis; written before any run)

**On each of the two networks, with the configuration its rules derived, H-25's two arms under a reward right three times in four:**
- **clause 1, the learning holds**: every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the four couplings lies within 0.75 and 1.25 of the image's;
- **clause 4, the critic holds the expected reward**: for each stimulus and each mapping over its last 128 trials, the engine's mean value lies within a quarter of the reward of $(2p - 1) \cdot 1/2$ of it.

These are H-35's four, by H-35's rules.

**Yes** when all four hold on both networks, in both arms; otherwise **no**, with the network, the arm and the clause. On a network where clause 1 or 2 fails, H-35's split is read as H-35 read it: *slower than the schedule* where clause 2 holds, the first mapping is learned in both arms and each of the six reversals reaches a block with 40 of 64 correct inside its 32; *not revised* otherwise.

**No prediction for the verdict.** What H-35's own ratios forecast, carried to each network's reversals under the true reward:

| Network | At 1.42, H-35's mean | At 1.71, the slowest of H-35's that crossed | At twice, the naive scaling |
| :--- | :--- | :--- | :--- |
| Seed 23 | 17 to 23 blocks | 21 to 27 | 24 to 32 |
| Seed 24 | 17 to 30 | 21 to 36 | 24 to 42 |

At H-35's mean every reversal crosses inside its 32 blocks, one on seed 24 at their edge. A crossing is not a mapping learned: on seed 22 the mappings after a flip ended at 75 to 110 of 128.

**Four readings are predicted**; a miss is recorded as one:
1. **the first mapping is learned in all four arms**, at least 80 of its last 128;
2. **on each network the six reversals are slower than its own under the true reward**: the mean of their ratios is above 1.13, a reversal that does not cross counted at its 32 blocks;
3. **the value is nearer a half of $2p - 1$ than $2p - 1$**, for each stimulus in each mapping that clause 1 holds for and whose accuracy over those trials is at least three quarters. ADR-0170 wrote this reading without the last condition and it missed in one cell of fourteen, at an accuracy of 34 in 64, where the two are 0.03 of the reward apart;
4. **what the misleading trials undo is the share's and not the network's**: in each arm misleading punishments take back between a third and a half of what true rewards give the answer's pairs, and misleading rewards put back between a third and a half of what true punishments take from the pair a wrong selection reached. H-35 read 42 and 44, and 41 and 43 per cent.

**Readings, no clause**, each beside the network's own arms under the true reward and beside H-35's on seed 22:
- H-35's readings, each as H-35 computed it: the reversal speeds, the trials by outcome and by the reward received, what a reward of each kind moved, a true reward's error, how often a reward left the signal beyond the gate's bounds by kind, the learning signal, the value and its troughs, the couplings' separation and the inhibitory sum's course;
- **over the three networks**: the mappings learned of twenty-four, the reversals that crossed of eighteen, the mean ratio to the true reward's speeds, and each reading by kind as a range.

### H-36's stopping rule

1. **One round**: brief 070 builds the arms in the tests and runs H-36. No file under `src/` changes. The networks, H-36's constants and its predicted readings are committed before the first rewarded run at three in four on either network.
2. **A calibration** stops the round before any such run, and is a finding, if any of these fails:
   - every pinned number of the tree holds, H-33's arms on the two networks and H-35's on seed 22 among them;
   - on each network the derivation is the one ADR-0167 pinned, and the first block under the true feedback is H-33's;
   - the coin's count is H-35's, in all and by mapping.
3. **Yes**: the configuration holds a reward right three times in four on two networks of three, and seed 22's no stands at its edge. **No rule of the engine is changed for it.** The next decision is an ADR that states what the configuration holds of an unreliable reward, and chooses among a reward less reliable still, a reward delayed past a trial, the operating regime and a larger size, named and not taken.
4. **No on clause 3, on either network**: a misleading reward moves the network the learning stands on. The next decision is an ADR on what it reaches.
5. **Otherwise no on clause 1 or 2, on either network**: the no at three in four is not one network's. The next decision is the ADR H-35's step 5(b) named, on a single error's weight and the gate's clamp, with three networks' readings by kind as its need; where every network that fails reads *slower than the schedule*, the span a mapping is given is among its candidates.
6. **Otherwise no on clause 4 alone**: the next decision is an ADR on the critic's step under noise.
7. **No constant moves after a rewarded run, and there is no second attempt, no other network and no other share.**

## Consequences

- Good: the need is read before a rule of the engine is changed for it.
- Good: no build. The feedback, the networks and their derivations are in the tree.
- Good: whichever candidate is taken later is run on these networks; their arms at this share are then the ones it stands beside.
- Good: the same trials mislead on every network, so what differs between the runs is the network's.
- Bad: a yes does not undo seed 22's no. Two networks of three is a count, not a rate.
- Bad: a no may stand at an edge again. The rule written first is the rule.
- Bad: one share, one size and one generator.
- Bad: **four more arms in the weekly job**, each deriving its network first, about 1 880 s each. The table goes from 83 129 s to about 90 700, about 7 560 s summed a shard: 52 to 59 per cent of the bound at 1.79 to 2.00 of the summed seconds over the wall, which every shard of the last three runs read but one, and 67 at the 1.56 that one read. **Twelve shards do not hold another round's arms under the briefs' 60 per cent**, so the workflow's decision ADR-0168 named is due before the round after this one.
- Neutral: the round after this is a decision either way, and by step 3 it may be one without a build.

## Alternatives considered and why rejected

- **Option 1(b), a longer span on seed 22**: it asks the edge of one network a second question. H-31 read that a longer span is not neutral, and the answer would still be one network's.
- **Option 1(c), the gate's clamp widened**: a rule of the engine and a parameter of the image, for a need that turns on five trials. The readings say it passes on the misleading errors with the true ones.
- **Option 1(d), a smaller weight on one error**: a rule of the engine too, and the least defined of the candidates. It slows a learner the schedule is already short for.
- **Option 1(e), the critic's step**: the value held the expected reward in every cell.
- **Option 2(b), the networks at 2 048 units too**: four arms of about an hour each, half a shard's bound apiece. Their reversals under the true reward were the fastest read, 9 to 15 blocks, so they are the next networks to ask if these two disagree.
- **Option 2(c), seeds not yet read**: a network with no arm under the true reward to stand beside, and a derivation to run first.
- **Option 3(b), one pooled verdict**: a mapping that fails on one network would be averaged away, and the question is whether the no is one network's.
- **Option 4(b), the reading as ADR-0170 wrote it**: it would miss again wherever a learned mapping holds a stimulus near chance, which says nothing of the critic.

## Confirmation

`briefs/070_three-in-four-on-the-other-networks.md` runs H-36 once. Whitepaper 4.110.0 carries H-36 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
