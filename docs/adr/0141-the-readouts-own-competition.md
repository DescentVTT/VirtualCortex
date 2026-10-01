---
status: accepted
date: 2026-10-01
depends-on: ADR-0140
decision-makers: VirtualCortex maintainers
---

# ADR-0141: The readouts' own competition — after H-25, the second step ADR-0138 planned, a neural form of the target side, begins by measuring with nothing changed: on H-25's own runs, reproduced bit for bit, an open-loop shadow replays the stimulus–readout synapses as if the address's targets were released to every unit, so that what the release would cost, against the learning signal the run delivered, is read before anything is built; H-26 written before any run, its yes naming the release as the next round and its no the lateral competition `cortex-basal-ganglia` specifies, with the measured cost as its need; brief 059 runs it once

## Context and Problem Statement

[ADR-0140](0140-the-address-drawn-measured.md) read **H-25 yes**: with the reward's sources drawn by the engine from its critic's window and its targets the readout its own selection chose, every mapping was learned, revised and bounded as under the host's address. By H-25's stopping rule, step 3, the next decision is an ADR choosing among six, the second step [ADR-0138](0138-the-address-drawn.md) planned among them. **The maintainers took the second step on 2026-10-01, and chose to begin it by measuring with nothing changed.**

**What the second step replaces.** The address's targets are the readout the task's selection chose, an efference copy over the body's sets (ADR-0138). A neural form would let the network's own activity make the eligibility specific to the chosen readout, so that the targets could be released to every unit.

**The selection as built** (`runtime/cortex-runtime/src/task.rs`, `Readout::select`; `crates/cortex-basal-ganglia`):
- After a trial's ticks, each readout's spikes in the readout window are counted.
- Each channel's direct drive is its own count and its indirect drive the other's, and `compute_gating` selects the channel whose net output falls below zero: the larger count, none at a tie.
- Whitepaper §5.2.5 marks the gating rule *Implemented (linear form)* and **lateral competition and dopamine modulation *Specified* (§8.8)**.

So nothing in the network makes the losing readout fire less while the response is under way. The decision is a comparison after the fact.

**What a released target side would cost**, by the rules as they stand. With the targets every unit, the presented stimulus's drawn synapses onto the readout not selected consolidate under the same signal, in proportion to their eligibility. Both moves work against the answer:
- on a correct trial the signal is positive, and the other readout's coupling rises;
- on a wrong trial it is negative, and the answer's coupling falls.

So the cost is set by how much eligibility the readout that lost carries beside the one that won.

**What is already read.**
- H-24 delivered the reward to every synapse, sources and targets alike ([ADR-0137](0137-the-reward-unaddressed-measured.md)). The other pairs moved as much weight as the answer's, and the learning did not follow.
- H-24 cannot say what the target side alone costs, because its sources were global too and the outside moved five to six times as much.
- Under H-25's drawn address the readout that lost was never consolidated. So no round has read the target side's cost by itself, on a run that learns.

**The account and the literature.** In models of the basal ganglia the losing channels are suppressed by lateral and feedback inhibition while the winner is released, so the corticostriatal synapses onto the losers carry little eligibility when the dopamine arrives, and a global dopamine teaches the winner (Gurney, Prescott and Redgrave 2001; Frank 2005). The account predicts that without such a competition the released target side costs much of the learning signal.

**Where the engine's premise differs:**
- **The geometry.** The two readouts are interleaved sets on one ring, chosen by [ADR-0065](0065-the-instrument-recalibrated.md) so that each stimulus's couplings onto them are equal within ten per cent. Both respond to a volley through the same local wiring.
- **The selection** is read over a window after the fact, not settled by dynamics.
- **The arithmetic** is integer, over a fixed seeded prior.

So the cost may be larger than a model with separate channels predicts. It is measured, not assumed.

## Decision Drivers

- **The second step's need**, stated before anything is built: how specific the readouts' own activity already makes the eligibility, and so how large a gap a competition must close.
- **The maintainers' choice** (2026-10-01): measure first, with nothing changed.
- **Nothing of the engine changes**, and H-25's runs are reproduced bit for bit, so the reading is of the configuration H-25 named.
- **The measure must be the release's own**: what the stimulus's synapses onto the losing readout would consolidate under the run's signal, not a proxy read from spike counts alone.
- Latest ≠ Newest: no dependency, no tool, no rule.

## Considered Options

1. **What is read**:
   - (a) the readouts' spikes alone, the selected readout's against the other's in the window;
   - (b) the eligibility at each reward onto each readout;
   - (c) an open-loop shadow: the stimulus–readout synapses replayed from the run's own train and signal as if the targets were every unit, keeping their own traces and weights, never written back.
2. **The run**: (a) H-25's two arms, reproduced bit for bit, the shadow beside them; (b) a new rewarded run with the targets released.
3. **How the reading names the build**: (a) a rule written first, comparing the release's cost to the learning signal the run delivered; (b) readings only, the build chosen after.

## Decision Outcome

**Options 1(c) with (a) and (b) as readings, 2(a) and 3(a).**

- **Why the shadow** (option 1(c)). It is the release's own cost, computed by the composer's rule with only the address changed, so its arithmetic is the one the record has been held to at every trial since H-13. Spike counts (option 1(a)) and the eligibility at the reward (option 1(b)) are read beside it, since a competition would act on the first and must reduce the second.
- **Why H-25's run** (option 2(a)). A released run (option 2(b)) is a hypothesis of its own. It is the next round if this one says the release is affordable, and it would not say how far it is from affordable if it failed.
- **What the shadow cannot say.** A released run would diverge from H-25's, since its weights would differ and so would its selections. The shadow reads the release's cost on a trajectory that learned, its first-order cost, and the rule's yes leads to a released run rather than to a conclusion.

### H-26 (Hypothesis; written before any run)

**On H-25's runs, reproduced bit for bit, with the shadow beside them:**
- **The cost** of a mapping is the shadow's consolidation on the stimulus–readout pairs of the readout not selected, signed against the answer and summed over the mapping's trials: a rise of a stimulus's coupling onto the other readout counts as cost, and so does a fall of its coupling onto its answer.
- **The signal** of a mapping is the learning signal the run delivered: the answer pairs' net consolidation less the other pairs', summed over the mapping, as H-24's and H-25's readings of where the consolidation went define them.
- **H-26 holds** when, in both arms and in every mapping, the cost is at most half the signal.

**Yes** when it holds in every mapping of both arms; otherwise **no**, with the mappings that failed.

**Predicted: no.** The account predicts that without a competition the losing readout carries much of the eligibility. H-24 read the other pairs moving as much as the answer's, and the geometry makes the two readouts' couplings equal. The verdict is read by the rule whatever the prediction.

**Readings, no clause**, per block and mapping:
- the selected readout's spikes and the other's, in the readout window and over the trial;
- the eligibility at each reward onto each readout, from the drawn sources;
- the cost and the signal by block, the reversal blocks apart from the learned ones;
- the count margin the gating decided by.

### H-26's stopping rule

1. **One round**: brief 059 runs it once. Nothing of the engine changes. H-26's rule is committed before the run, and the only rewarded runs are H-25's own, reproduced.
2. **A calibration** stops the round, and is a finding, if any of these fails:
   - every pinned number of the tree holds;
   - H-25's two arms reproduce their pinned tables bit for bit with the shadow beside them;
   - the shadow, replaying under the drawn address, reproduces the composer's moves.
3. **Yes**: the readouts' own activity already makes the eligibility specific enough. The next decision is an ADR on the release as its own hypothesis, the targets every unit and nothing built.
4. **No**: the release costs more than half the learning signal. The next decision is an ADR on the lateral competition whitepaper §5.2.5 specifies, with this round's cost per block as its need and the attention-gated feedback named as its fallback.
5. **No constant moves, and there is no second attempt.**

## Consequences

- Good: the second step's build is chosen by a measured gap, not by a model's prediction, and on the configuration it will change.
- Good: nothing of the engine changes, H-25's pins stand, and the shadow is the composer's own rule with one address changed.
- Bad: the shadow is open-loop. It reads the release's cost on H-25's trajectory, and a released run would take another.
- Bad: a round that builds nothing. Its value is the need it measures for the next.
- Neutral: no file under `src/` changes, so the dispatch's scope is `exhaustive` ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)). The shadow either rides on H-25's two weekly tests, their pins untouched, or runs in two of its own, about four points of the bound; the plan stays near half of it.

## Alternatives considered and why rejected

- **Option 1(a), the spikes alone**: a competition acts on them, but the cost is in the eligibility and the signal. A readout that fires less may still carry a trace.
- **Option 1(b), the eligibility at the reward alone**: it misses where in the trial the consolidation falls, at each presynaptic spike under a signal that decays.
- **Option 2(b), a released run now**: a hypothesis without its need measured. A no would say the release fails, not by how much.
- **Option 3(b), readings only**: the build would be chosen after the reading, which is the tuning the line's rules exist to avoid.

## Confirmation

`briefs/059_the-readouts-own-competition.md` runs H-26 once. Whitepaper 4.87.0 carries H-26 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
