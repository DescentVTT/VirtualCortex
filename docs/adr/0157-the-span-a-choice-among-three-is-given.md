---
status: accepted
date: 2026-10-09
depends-on: ADR-0156
decision-makers: VirtualCortex maintainers
---

# ADR-0157: The span a choice among three is given — after H-29 and H-30, the configuration left as it is and the schedule sized for three answers: a mapping after a flip is 48 blocks where it was 32, by the number of answers and by H-29's own slowest reversal; the whole punishment unset; no rule and no file under `src/` changed; each arm H-29's trial for trial up to H-29's second flip; H-31 written before any run; brief 064 runs it once

## Context and Problem Statement

[ADR-0156](0156-a-punishment-the-value-does-not-soften-measured.md) read H-30 as no, on clauses 1 and 5. Its stopping rule, step 5, names the decision [ADR-0153](0153-three-answers-measured.md)'s step 5 names: an ADR choosing between the selection, an exploration among its candidates and the readout's resolution, with both rounds' readings. **The maintainers took none of the three on 2026-10-09. They kept the configuration and took the schedule.**

**The sweep of that round's dispatch**, which the merge did not wait for ([ADR-0150](0150-a-round-waits-for-what-it-checks.md)): SWEEP-OUTCOME-PENDING

**What the two rounds read together:**

| | H-29, the punishment softened by the value | H-30, the punishment whole |
| :--- | :--- | :--- |
| The first mapping | learned, 118 and 119 of 128 | learned, 119 and 119 |
| The first reversal | 31 and 30 blocks; its mapping learned from the assignment, not from the mirrored | 19 and 22 blocks; learned in both |
| The second and third reversals | 31 and 21, 23 and 23 blocks; all four learned | none passed 40 of 64; none learned |
| The six couplings' sum | 1.008 to 1.053 of the image's | down to 0.933 |

- **The softened punishment is slow and holds.** Every mapping of H-29 after a flip was still improving when it ended, and seven of eight were learned.
- **The whole punishment is fast once and then costs the couplings** more than the rewards restore.
- **H-29's no was read on a schedule written for two answers.** [ADR-0151](0151-three-answers.md) kept H-25's schedule so that H-25 stood beside the run block for block, and H-29 failed it by three presentations of one stimulus in one mapping of eight.

**What neither round read:** whether the configuration as it is learns and revises a choice among three when a mapping lasts as long as that choice takes, and whether it stays where H-29 left it over the longer run. Two things were still moving when H-29 ended: every mapping after a flip, and the inhibitory sum, at 0.843 and 0.842 of the image's and falling.

**Why this was rejected twice and is taken now.** ADR-0151 rejected a longer schedule because it would be another schedule than H-25's. [ADR-0154](0154-a-punishment-the-value-does-not-soften.md) rejected it because *"it would likely pass, and nothing would have been changed"*. Both held while a faster revision was the open question. H-30 has read that question once: the one change aimed at the revision's speed lost the later reversals. What a choice among three takes with nothing changed is now the baseline the next step needs, and the third stimulus of ADR-0151's second step halves again what a stimulus is shown.

**The account and the literature.** A learner driven by a prediction error revises more slowly as the alternatives grow: with two, leaving the old answer finds the new one, and with three it finds it half the time (Frank 2005; Bogacz and Gurney 2007 on a choice among several). The account predicts that the configuration holds a choice among three given the span.

**Where the engine's premise differs:**
- the selection has no noise of its own (H-17), so a new answer is found only where the counts' own spread selects it;
- readouts of 204 units, whose counts lie within two spikes of one another in 40 to 65 per cent of the trials;
- a run half again as long on a network whose inhibitory sum was still falling.

## Decision Drivers

- **Two measured rounds**: the softened punishment holds and is slow, and the whole one does not hold.
- **The maintainers' choice** (2026-10-09): the configuration kept, the schedule sized for three.
- **No mechanism added.** The exploration ADR-0153 and ADR-0156 name stays open, and is the next decision on a no.
- **The span is derived, not chosen**, two ways that agree.
- **Said plainly**: the span is derived from H-29's own reading. H-31 is another question than H-29's and leaves H-29's verdict as it is.
- **A round without a build.** No file under `src/` changes, so the dispatch is the whole-domain shards alone ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)).
- Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **The span of a mapping after a flip**: (a) 48 blocks; (b) 64, twice H-25's; (c) each mapping run until it is learned.
2. **The first mapping's span**: (a) H-25's 24 blocks; (b) 36.
3. **The number of reversals**: (a) three, as H-25; (b) one.
4. **The whole punishment**: (a) unset; (b) set, on the longer schedule too.
5. **The clauses**: (a) H-29's four, the speeds and the sums as readings; (b) H-30's five.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

### The span, derived (option 1(a))

- **By the number of answers**: H-25's 32 blocks for two answers, half again for three, is 48.
- **By H-29's reading**: H-25's slowest reversal took 21 of its mapping's 32 blocks. H-29's slowest took 31, and 31 times 32 over 21 is 47.2, so 48 gives the slowest reversal among three the room the slowest between two had.

Both give 48.

### The schedule (Specified; brief 064 builds it in the tests)

- **The first mapping is 24 blocks**, as in H-25 and H-29. It passed 40 of 64 in its fifth and its seventh block.
- **Each of the three mappings after a flip is 48 blocks.** The flips come before the trials of index 1 536, 4 608 and 7 680, and a run is 168 blocks, 10 752 trials.
- **The configuration is H-29's**: H-25's with ADR-0153's three readouts, the deal it took, the answer's feedback and the drawn address. The whole punishment of [ADR-0155](0155-a-punishment-the-value-does-not-soften-built.md) is unset.
- **The arms are H-29's two**, from the assignment and from the mirrored assignment, through (0, 1), (1, 2), (2, 0), (0, 1) and (1, 0), (2, 1), (0, 2), (1, 0).
- **Each arm is H-29's arm trial for trial for its first 3 584 trials**, up to H-29's second flip: the image, the seed, the first mapping and the first flip are the same, and the schedules first differ there. So the first reversal is already read, at 31 and 30 blocks.
- **Nothing under `src/` changes.** The schedule is the harness's.

### H-31 (Hypothesis; written before any run)

**On H-29's configuration, readouts and arms, with a mapping after a flip of 48 blocks:**
- **clause 1, the learning holds**: in both arms every mapping is learned. At least 80 of its last 128 trials are correct, and among them each stimulus selects its answer in more than half of its presentations.
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end.
- **clause 3, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the six couplings lies within 0.75 and 1.25 of the image's.
- **clause 4, the critic holds the expected reward**: for each stimulus and each mapping over its last 128 trials, the engine's mean value lies within a quarter of the reward of $(2p - 1)$ of it.

These are H-29's four, by H-29's rules, over each mapping's last 128 trials.

**Yes** when all four hold in both arms; otherwise **no**, with the clauses that failed.

**Predicted: yes.** The first reversal's mapping read 44 and 43 correct of 64 in its last two blocks from the assignment and 42 and 43 from the mirrored assignment, rising, with 16 blocks more to come. What the prediction does not hold: the second and third reversals leave a mapping held for 48 blocks, not 32, and a mapping learned for longer may be harder to leave. ADR-0154's prediction of a yes was wrong, and this one is made on less: no mechanism, and the same run for a third of its length.

**Predicted readings:**
- **(a) the mapping H-29 failed is learned**: from the mirrored assignment, the second mapping is learned by each stimulus over its last 128 trials.
- **(b) every reversal passes 40 of 64 within 35 blocks of its flip**, half again H-25's 23.
- **(c) the six couplings' sum stays at or above the image's** at every mapping's end, as in H-29, where H-30's sank.
- **(d) the inhibitory sum ends between 0.75 and 0.80 of the image's**, still falling. H-29's ended at 0.843 after 120 blocks.

**Readings, no clause:**
- the block each mapping passes 40 of 64 in, and each stimulus's, beside H-29's;
- each mapping's correct trials by block from its 32nd to its 48th: what the added span bought;
- where the wrong selections went, the ties and the margin, by mapping;
- the six couplings' courses and their sum, and the value's troughs;
- the inhibitory sum's course, beside H-21's bar of a half.

### H-31's stopping rule

1. **One round**: brief 064 writes the schedule into the harness and runs H-31. No file under `src/` changes. The schedule and H-31's constants are committed before any trial past H-29's second flip is run.
2. **A calibration** stops the round before any such trial, and is a finding, if either fails:
   - every pinned number of the tree holds;
   - each arm is H-29's arm trial for trial for its first 3 584 trials.
3. **Yes**: the learning configuration holds a choice among three, given the span such a choice takes, a revision about half again as long as between two. The next decision is ADR-0151's second step, an ADR on the third stimulus, with this span as where its schedule starts. It is named and not taken.
4. **No on clause 3**: the longer run moves the network the learning stands on. The next decision is an ADR on what it reaches.
5. **Otherwise no on clause 1**: the configuration does not hold a choice among three even with the span. The next decision is the ADR on an exploration that ADR-0153's step 5 and ADR-0156's steps 5 and 6 name, with three rounds' readings.
6. **Otherwise no on clause 2**: the answers are learned with couplings past the bound. The next decision is an ADR on the bound at this readout's size.
7. **Otherwise no on clause 4 alone**: the next decision is an ADR on the critic's step.
8. **No constant moves after a trial past H-29's second flip, and there is no second attempt.**

## Consequences

- Good: the baseline the third stimulus needs, a choice among three with nothing changed, read on a span derived two ways.
- Good: no build. No rule, no record and no format changes, and the round's dispatch is the shards alone.
- Good: a third of each arm is H-29's bit for bit, which is the calibration.
- Bad: nothing is made faster. A revision among three stays half again as long as between two, and a third stimulus will lengthen it again.
- Bad: the span comes from H-29's own reading. A yes says the configuration learns given the span; it does not say H-29 was nearly a yes.
- Bad: the exploration both stopping rules name is deferred, not answered.
- Neutral: two weekly tests of 10 752 trials, about 1.4 times H-30's 1 798 and 1 729 s, raise the table from 50 038 s to about 55 000, about 4 580 s a shard of twelve. The longer of them becomes the floor no number of shards divides, about 2 500 s.

## Alternatives considered and why rejected

- **Option 1(b), 64 blocks**: twice H-25's is past both derivations, and each arm would be 216 blocks.
- **Option 1(c), each mapping run until learned**: a schedule the result writes. No table of it could be pinned before the run.
- **Option 2(b), a first mapping of 36 blocks**: the first mapping passes 40 of 64 by its seventh block and reads 118 and 119 of its last 128 at 24. At 24 the arm is H-29's until the second flip.
- **Option 3(b), one reversal**: H-30 lost its second and third reversals after gaining its first. The later reversals are where a run shows what it cannot keep.
- **Option 4(b), the whole punishment on the longer schedule**: H-30's later mappings were falling, not rising, and its couplings' sum had sunk. More blocks do not put that back.
- **Option 5(b), H-30's clause 5**: 23 blocks is the two-answer bound, and H-29 has read that the softened punishment does not meet it. Half again that bound is read, as predicted reading (b).
- **An exploration** (ADR-0153's step 5, ADR-0156's steps 5 and 6): the larger design, and the account's remedy for a revision that is slow. It is the next decision on a no, and stays open on a yes.
- **The readout's resolution at 4 096 units**: each arm about four times as long, and neither round's readings name the resolution as what the revision lacks.

## Confirmation

`briefs/064_the-span-a-choice-among-three-is-given.md` runs H-31 once. Whitepaper 4.99.0 carries H-31 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
