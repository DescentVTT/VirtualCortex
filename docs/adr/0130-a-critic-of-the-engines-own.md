---
status: accepted
date: 2026-09-29
depends-on: ADR-0129
decision-makers: VirtualCortex maintainers
---

# ADR-0130: A critic of the engine's own — after H-21, the learning line takes the last of H-20's open questions it named: the expected reward the dopamine signal is measured against, held today by the task and indexed by the stimulus the task says it presented, is held by the engine instead as a value weight on every unit and read from the engine's own spikes since the last reward, the prediction error formed and the weights moved by the engine when a reward arrives; its constants a parameter of the image, unset every run the run it was; brief 055 builds it and runs H-22 once

## Context and Problem Statement

[ADR-0129](0129-a-target-the-network-fires-at-measured.md) read **H-21 yes**. With the inhibitory rule's target at the settled network's own rate, H-20's schedule kept its inhibitory sum at 0.848 of the settled image's or more, where H-20's fell to 0.07, and still learned every mapping with its couplings bounded. By H-21's stopping rule the learning configuration is named with the target. The next decision is an ADR choosing among the operating regime, another size and a critic of the engine's own. **The maintainers took the critic on 2026-09-29.**

**The critic as it is** (`runtime/cortex-runtime/src/task.rs`, [ADR-0106](0106-the-reward-prediction-error.md), [ADR-0107](0107-the-critic-built.md)):
- `Critic` holds one expected reward per stimulus, `expected_q16: [i32; 2]`, and a shift.
- At each trial `Task::trial` takes the outcome's reward $r$ and the expectation $V_s$ of the stimulus $s$ **the task presented**. It delivers $\delta = r - V_s$ to `Executor::reward`, and moves $V_s$ by $\delta \gg 5$.
- Its own documentation: *"The critic is the task's state, as the seed and the trial's index are: no record holds it and no image carries it."*

So the engine learns from a prediction error that the host computes, from a label the host knows: which stimulus it drew. That label is not the engine's to have. The engine sees only its own spikes. H-19 and H-20 read what the critic does for the learning — the advantage it delivers while the selection still errs ([ADR-0108](0108-the-reward-prediction-error-measured.md), [ADR-0109](0109-a-schedule-of-reversals.md), [ADR-0110](0110-a-schedule-of-reversals-measured.md)). They read it with a critic outside the engine.

**The account the choice rests on.** In the basal ganglia the expected reward that dopamine neurons compare the reward against is held by striatal and prefrontal populations and read from the cortex's own activity, not from a label (Schultz, Dayan and Montague 1997). The standard model of that computation is the actor–critic with linear function approximation (Sutton and Barto 2018): the value is a weighted sum of features of the state, the error is the reward less the value, and each weight moves by the error times its feature. In a spiking network the features are the neurons' activity (Potjans, Morrison and Diesmann 2009; Frémaux, Sprekeler and Gerstner 2013). The engine already has the features, its own spike train, and the modulator the error goes to.

## Decision Drivers

- **A measured need**: H-19 and H-20 showed the critic is what lets the configuration settle and revise, and it lives outside the engine.
- **The maintainers' choice** of the critic (2026-09-29).
- **The engine's own inputs only.** The critic reads nothing the host knows and the engine does not: no stimulus index, no geometry, no trial boundary but the rewards themselves.
- **Unset, bit for bit**, as ADR-0094, ADR-0114 and ADR-0123 built theirs: a parameter of the image, and every run without it the run it was.
- **ADR-0016's test.** A unit's weight onto the critic is a quantity of the unit, held in its record's last reserved bytes, not a crate.
- **Latest ≠ Newest**: a linear value function and the delta rule, in Q16.16 integers; no dependency.

## Considered Options

1. **Where the expectation lives**:
   - (a) the task's two expectations moved into the image, still indexed by the task's stimulus;
   - (b) a value weight on every unit, the value read from the units' own spikes;
   - (c) a population of critic units whose firing is the value, its synapses learning under the error by the three-factor rule.
2. **The features**: every unit's spikes since the previous reward; the stimulus sets' spikes; the readouts' spikes in the task's window.
3. **When**: at every reward the engine receives; or on a cadence of its own.

## Decision Outcome

**Options 1(b), 2 every unit's spikes since the previous reward, and 3 at every reward.**

### The critic (Specified; brief 055 builds it)

- **The weights.** Each unit carries a value weight $w_i$, signed, in its record's `[52..54)`, the `_reserved` field today, "MUST be zero". It stays zero in every image written before the critic, and in every run without it. The record's field and whitepaper §5.2's table change, and the format moves from 18 to 19.
- **The features.** $c_i$, the unit's spikes since the previous reward the engine received, read from the executor's own train. The rewards alone mark the window: in the task one trial, $2^{14}$ ticks; for a host that rewards otherwise, whatever lies between its rewards.
- **At a reward $r$**, with the critic set:
  - the value $V = \sum_i w_i\, c_i$, widened and saturating;
  - the error $\delta = r - V$, saturating, is what the modulator receives in place of $r$;
  - every unit that fired moves its weight by $(\delta \cdot c_i) \gg k$, saturating within the weight's width. The step is the error's sign times the feature, the delta rule.

  With the critic unset, `Executor::reward` is what it is today, bit for bit.
- **The constants**, a parameter of the image: the set flag, the update's shift $k$, and the weight's scale (how much reward one unit of weight predicts per spike). The build chooses the widths by arithmetic it writes first. The loader refuses a shift or scale the rule does not resolve.
- **Where the rule sits.** The value, the error and the step are a pure rule over counts and weights, in a state crate beside the modulator, tested over the lattice. The executor composes it: at `reward` it reads the train since the last reward, forms the error, delivers it, and writes the weights between ticks, where no worker holds a record (axiom A3).
- **The task.** With the engine's critic set, the task delivers the outcome's reward and no critic of its own. A task that carries its own critic with the engine's also set is refused, since two critics would take the expectation twice.

### H-22 (Hypothesis; written before any run)

**On H-21's configuration — H-20's configuration, schedule and arms with the inhibitory rule's target at the settled network's rate — with the task's critic replaced by the engine's own:**
- **clause 1, the learning holds**: in both arms every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the critic predicts**: in both arms, over each mapping's last 128 trials, the mean error on the correct trials is at most half the reward's magnitude. The engine then expects at least half of what a correct answer brings.

**Yes** when all three hold in both arms; otherwise **no**, with the clauses that failed. No prediction is made for the verdict.

**Readings, no clause**:
- the engine's value on each trial beside the task critic's expectation for the same stimulus in H-21's arm;
- the weights by class of unit at every block's end, and which units carry the value;
- the reversal speeds beside H-20's and H-21's;
- the inhibitory sum's course.

### H-22's stopping rule

1. **One round**: brief 055 builds the critic, its ADR and tests, then runs H-22. The constants and the criterion are committed before the first rewarded run.
2. **A calibration** stops the round before any rewarded run, and is a finding, if with the critic unset any of these fails:
   - every pinned number of the tree holds;
   - the determinism pin does not move;
   - H-21's arms reproduce.
3. **Yes**: the configuration is named as one that learns with its own critic. The next decision is an ADR choosing among the operating regime, another size and the rule held by the network reopened on this configuration, named and not taken.
4. **No on clause 1 or 2**: the engine's critic does not stand in for the task's. The next decision is an ADR on its features or its rate, with this round's readings as its need.
5. **No on clause 3 alone**: the engine learns the task without learning to predict its reward. The next decision is an ADR on the critic's scale or window.
6. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: the engine's learning no longer depends on a label only the host has; every input the critic reads is the engine's own.
- Good: unset, nothing changes, by construction and by the build's tests; H-21's configuration stands beside it as the comparison.
- Bad: a record field and a format bump, and the weights written between ticks by the coordinator: a new write to the unit arena outside a turn, which the build must show respects axiom A3.
- Bad: every unit that fires between two rewards is a feature, background included. The value may learn from noise, and the shift has to make the step small enough for that noise to average out while the stimulus's volley still teaches.
- Neutral: the critic reads spikes, not a trial. A host that rewards at a different cadence gets a different window, which is what an engine's own critic should do.

## Alternatives considered and why rejected

- **Option 1(a), the task's expectations in the image**: the engine would hold the numbers and still take the index from the task. It is the engine's storage, not its critic.
- **Option 1(c), a population of critic units**: this is the fully neural form, but it is two mechanisms at once — a value carried by firing, and weights learning under the error by STDP pairings — and the second is the pairing rule's, not the delta rule the account names. It is named as the step after a yes.
- **Option 2, the stimulus sets or the readout window only**: the task's geometry and timing are the host's knowledge. Every unit and the rewards' own spacing are the engine's.
- **Option 3, a cadence of its own**: the error exists only where a reward arrives.

## Confirmation

`briefs/055_a-critic-of-the-engines-own.md` builds the critic and runs H-22 once. Whitepaper 4.79.0 carries H-22 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
