---
status: accepted
date: 2026-10-07
depends-on: ADR-0146
decision-makers: VirtualCortex maintainers
---

# ADR-0147: A reward right seven times in eight — with the learning configuration named, the line asks a task the engine has not been asked, the first of them a reward that is not always true: on H-25's configuration and schedule the reward's sign is the outcome's in seven trials of eight and its opposite in one, by a coin of the task's own draw; the share derived from the schedule before any run; a feedback of the task, unset every run the run it was; H-28 written before any run; brief 061 builds it and runs H-28 once

## Context and Problem Statement

[ADR-0146](0146-the-tag-the-address-already-is.md) closed the address's second step and named the learning configuration as H-25's. H-25's stopping rule, step 3, left a list of next decisions: the operating regime, another size, the rule held by the network reopened, a critic carried by a population, and a task the engine has not been asked. **The maintainers took a task the engine has not been asked on 2026-10-07, and chose the first: a reward that is right only most of the time**, the probabilistic reversal task.

**Why this task.** From H-13 to H-27 every reward the engine received was true: a correct selection was rewarded and a wrong one punished, every time. The line added a critic, a signed gate and an address under that condition. A reward that is sometimes wrong asks of the same machinery what it has never been asked:
- that the critic hold an **expected** reward and not a certain one;
- that one misleading reward not undo what many true ones taught;
- that a reversal be told from noise.

It is the standard task for a learner driven by a reward-prediction error (Cools et al. 2002; Frank 2005).

**The rules a misleading reward acts through**, read on 2026-10-07:
- **The critic** ([ADR-0131](0131-the-critic-built.md), [ADR-0134](0134-the-critics-window-built.md)) moves each counted unit's weight by the error. Its value on a stimulus comes to that stimulus's expected reward: about $(2p - 1)$ of the reward when the reward is always true ([ADR-0132](0132-a-critic-of-the-engines-own-measured.md), [ADR-0135](0135-the-critics-window-measured.md)), with $p$ the stimulus's accuracy.
- **The signed gate** ([ADR-0094](0094-the-signed-gate-built.md)) consolidates an addressed synapse under $\mathrm{clamp}(d, -1, 1)$ of the dopamine signal, the baseline being zero. A punishment moves the weight against its trace.
- **The task's draw** (`runtime/cortex-runtime/src/task.rs`): one `mix64` of the seed and the trial's index. Its bit 0 is the stimulus, and its bit 32 the shuffled control's coin.

**The arithmetic of a misleading reward**, written before any run. Let the reward be true with probability $q$:
- **The expected reward** of a stimulus answered correctly with probability $p$ is $(2p - 1)(2q - 1)$ of the reward. A critic that holds it expects less than the full reward on a correct trial, so a true reward still delivers an error above zero, of $1 - V$.
- **A misleading punishment** on a correct trial delivers $-1 - V$, below $-1$ for any value above zero. The gate clamps it at $-1$: the answer's pair moves against its whole trace, once.
- **Naively**, the learning signal is scaled by $(2q - 1)$ and a reversal takes $1 / (2q - 1)$ as long.
- **The clamp may change that.** Because the value stays further from the rails than under a certain reward, true rewards and true punishments keep delivering errors that the certain task's critic had spent. No round has read which is the larger.

**The account and the literature.** A learner driven by a prediction error tolerates a reward that is right most of the time and reverses when the contingency does, more slowly as the reward is less reliable (Frank 2005). The account predicts that the configuration learns and revises.

**Where the engine's premise differs:**
- the gate's clamp, which caps a single error's weight;
- integer arithmetic, and a dopamine signal that carries over from trial to trial (F-53);
- one seeded prior at 1 024 units.

## Decision Drivers

- **A measured need**: the configuration is named, and every reading of it is on a reward that never lies.
- **The maintainers' choice** (2026-10-07) of the task.
- **One change from H-25**: the reward's truth. The image, the critic, the window, the address, the schedule and the arms are H-25's.
- **The share is derived, not chosen**, from the schedule the comparison needs.
- **Unset, bit for bit**: a feedback of the task, beside the three it has.
- **The weekly job's budget**: about 47 per cent of the bound after ADR-0145.

## Considered Options

1. **The reward's reliability**: (a) seven in eight, derived below; (b) four in five, the literature's usual; (c) three in four; (d) several in one round.
2. **What is flipped**: (a) the reward's sign, whatever the outcome; (b) the reward of correct selections only.
3. **The coin**: (a) bits of the task's own draw that neither the stimulus nor the shuffled control reads; (b) a second seed.
4. **The schedule**: (a) H-25's, 7 680 trials with three flips; (b) a longer one for a slower learner.
5. **The clauses**:
   - (a) H-25's four;
   - (b) learned, bounded, the network held, and the critic holding the expected reward, with the reversal speeds as a reading.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(b).**

### The reliability, derived (option 1(a))

- **The schedule** gives each reversal 32 blocks. H-25's slowest took 21.
- **By the naive scaling** a reversal takes $1 / (2q - 1)$ as long, so the schedule holds the slowdown while $21 / (2q - 1) \le 32$, which is $q \ge 0.83$.
- **The least reliable reward the schedule holds**, among shares a coin of whole bits gives, is **seven in eight**: $2q - 1 = 3/4$, and 21 blocks become 28.

So H-25's schedule and arms are kept, and the comparison with H-25 is block for block. Four in five (option 1(b)) is below what the schedule holds by the same arithmetic.

### The feedback (Specified; brief 061 builds it)

- **The rule.** The reward's sign is the outcome's in seven trials of eight and its opposite in one, whatever the outcome was: a correct selection is punished in one trial of eight, and a wrong one or a tie is rewarded in one of eight.
- **The coin** is three bits of the task's draw for the trial that neither the stimulus's bit nor the shuffled control's reads. The reward is misleading when all three are zero. A run is a function of its seed, as every run is.
- **Where it lives.** A feedback of the task beside `Answer`, `Shuffled` and `Withheld`. Under the three, every run is the run it was, bit for bit. Nothing of the engine, the image or the records changes.
- **What "correct" means under it**: the selection equals the stimulus's answer under the mapping in force, whatever reward the trial then received.

### H-28 (Hypothesis; written before any run)

**On H-25's configuration, schedule and arms, with the reward right seven times in eight:**
- **clause 1, the learning holds**: in both arms every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the four stimulus–readout couplings lies within 0.75 and 1.25 of the image's;
- **clause 4, the critic holds the expected reward**: for each stimulus and each mapping over its last 128 trials, the engine's mean value lies within a quarter of the reward of $(2p - 1) \cdot 3/4$ of it, with $p$ that stimulus's accuracy over those trials.

**Yes** when all four hold in both arms; otherwise **no**, with the clauses that failed.

**Predicted: yes.** The account predicts it, and the reliability was chosen so that the naive slowdown fits the schedule. The clamp's part is not predicted.

**Readings, no clause**:
- the reversal speeds beside H-25's, and beside four thirds of them;
- per mapping, the trials by outcome and by the reward received: true rewards, true punishments, misleading rewards and misleading punishments;
- what a misleading punishment moved in the answer's pair, and a misleading reward in the pair it reached, beside the true ones;
- the value beside $(2p - 1) \cdot 3/4$ and beside $(2p - 1)$, and its troughs after each flip;
- where the consolidation went, the couplings' separation and the inhibitory sum's course, beside H-25's.

**What clause 4 does not tell apart.** At a high accuracy the band of a quarter holds a value near $(2p - 1)$ as well as one near three quarters of it. The reading beside it says which the value is nearer.

### H-28's stopping rule

1. **One round**: brief 061 builds the feedback, its tests and its ADR, then runs H-28. The coin's bits and H-28's constants are committed before the first rewarded run.
2. **A calibration** stops the round before any rewarded run, and is a finding, if any of these fails:
   - every pinned number of the tree holds;
   - H-25's first block reproduces under the true feedback;
   - over the run's 7 680 trials, the coin by itself is misleading in between 880 and 1 040 of them, a share of one in eight within a twelfth either way.
3. **Yes**: the configuration learns and revises under a reward right seven times in eight. The next decision is an ADR choosing among a less reliable reward, more answers than two, a reward delayed past a trial, another size and the operating regime, named and not taken.
4. **No on clause 3**: a misleading reward moves the network the learning stands on. The next decision is an ADR on what it reaches.
5. **Otherwise no on clause 1 or 2**: the configuration does not hold a reward that is sometimes wrong. The next decision is an ADR on a single error's weight, the gate's clamp and the critic's step among its candidates, with this round's readings of what a misleading reward moved as its need.
6. **Otherwise no on clause 4 alone**: the engine learns the task without holding its expected reward. The next decision is an ADR on the critic's step under noise.
7. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: the first reading of the configuration under a reward that is not always true, with one thing changed and H-25 beside it block for block.
- Good: the reliability is what the schedule holds, by an arithmetic written before the run.
- Good: unset, nothing changes, and no rule of the engine is touched.
- Bad: one reliability. Whether seven in eight is near the edge or far from it is not read; a yes names a less reliable reward among the next decisions.
- Bad: the naive scaling the reliability was derived from may not be the engine's, since the clamp caps a single error. The reversal speeds are read for it, not held by a clause.
- Bad: clause 4's band does not tell an expected reward from an accuracy where the accuracy is high.
- Neutral: files under `src/` change, so the dispatch's scope is `both` ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)); two weekly tests of about H-25's cost raise the plan from about 47 to about 51 per cent of the bound.

## Alternatives considered and why rejected

- **Option 1(b), four in five**: the usual figure, and by the naive scaling 21 blocks become 35, past the schedule's 32. A no there would not say whether the engine fails the task or the schedule is short.
- **Option 1(c), three in four**: 21 blocks become 42.
- **Option 1(d), several reliabilities**: one change at a time, and each is two arms of the weekly job.
- **Option 2(b), correct selections only**: half the task. A wrong selection that is sometimes rewarded is what makes a reversal hard to tell from noise.
- **Option 3(b), a second seed**: the task's draw has the bits, and a run stays a function of one seed.
- **Option 4(b), a longer schedule**: it would be another schedule than H-25's, and the comparison would lose its reference. It is the next step if the reliability falls.
- **Option 5(a), H-25's four clauses**: H-25's reversal bound of 23 blocks is the task critic's speed under a true reward. Under this one the naive scaling alone puts a reversal at 28. The speeds are read, and clause 1 already fails a mapping that is not learned within its span.

## Confirmation

`briefs/061_a-reward-right-seven-times-in-eight.md` builds the feedback and runs H-28 once. Whitepaper 4.92.0 carries H-28 in §11.1 with its stopping rule, and §9's rows. `npm run spec` holds the links, the rows, the brief's sections and the version.
