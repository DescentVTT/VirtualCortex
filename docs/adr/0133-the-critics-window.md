---
status: accepted
date: 2026-09-30
depends-on: ADR-0132
decision-makers: VirtualCortex maintainers
---

# ADR-0133: The critic's window — after H-22, the learning line takes the window of H-22's stopping rule: the engine's critic counts a unit's spikes only within a window that opens at each reward and is as long as the shortest delay of any synapse the image carries, so that the value is read from what the network was given after its last outcome and not from what it did in response, the window a parameter of the image, unset the critic ADR-0131 built bit for bit; H-23 written before any run, with H-22's clause 3 replaced by a reversal clause and a prediction clause in the form F-61 gives; brief 056 builds it and runs H-23 once

## Context and Problem Statement

[ADR-0132](0132-a-critic-of-the-engines-own-measured.md) read **H-22 no, on clause 3 alone**. With [ADR-0131](0131-the-critic-built.md)'s critic, whose features are every unit's spikes since the previous reward, H-21's schedule learned every mapping in both arms with its couplings bounded. By H-22's stopping rule, step 5, the next decision is an ADR on the critic's scale or window. **The maintainers took the window on 2026-09-30.**

**What ADR-0132 read, and what it says about the need:**
- **The value is a stimulus's expected reward.** Per stimulus and mapping, the engine's mean value followed $2p - 1$ of the reward within about 0.15, and its value on the correct trials was its mean within 0.03. With two stimuli and two readouts, a linear value over the units cannot hold the conjunction of a stimulus and its answer, so the readouts' spikes did not make it an action's value.
- **F-61: clause 3 was an accuracy bound.** For a critic of a stimulus's expected reward the mean error on the correct trials is about $2(1 - p)$ of the reward, so clause 3 held where a mapping was learned at three quarters, 96 of 128. The one mapping that failed was learned at exactly 96. A critic's scale or window cannot move that: no linear critic of these features holds more than $2p - 1$ on a correct trial. So the step-5 decision is not about clause 3.
- **The need is the reversals.** After the first and third flips the engine's value fell to −0.92 to −1.03 of the reward, where H-21's task critic fell to −0.75 to −0.95. Those reversals took 22 to 30 blocks against H-21's 14 to 23, and fewer of the old answer's selections met a strong punishment: 262 to 425 per flip against H-21's 426 to 573. With the value near −1 a wrong selection delivers an error near zero, and the punishment that revises the old answer under the signed gate ([ADR-0093](0093-the-punished-pair-consolidates-against-its-trace.md)) fades while the old answer still wins.
- **Half the value sits on units both stimuli share.** Weighted by each group's spikes a trial, the presented stimulus's own units carried about a third of the value, the inhibitory units and the readouts about a half, and the other stimulus's units about a tenth. ADR-0132's account, a Hypothesis, is that the shared part is moved by every error of either stimulus, so after a flip, when both stimuli are answered wrongly, it falls twice as fast as either stimulus's own part.

**What a trial looks like to the critic** (`runtime/cortex-runtime/src/task.rs`, `tests/instrument/harness.rs`, read on 2026-09-30):
- The reward is delivered between the last tick of a trial and the first of the next, and the next stimulus is injected in the same gap. The stimulus set fires together about twelve ticks on (the task's module documentation), each of its 51 units once under ADR-0076's shape and cancel.
- The reference prior's local delay band is 100 to 300 ticks and its far band starts at 1 400 (the harness's `Prior`). No spike can cause another through a synapse sooner than the shortest delay. The readout window opens 100 ticks after the injection for that reason (`WINDOW`, [ADR-0065](0065-the-instrument-recalibrated.md)).
- A trial is $2^{14}$ ticks and holds about 351 spikes at H-21's rates: 51 the volley's, about 300 the background's and the responses', over some 970 units (ADR-0132).

So in the first hundred ticks after a reward a trial holds the new stimulus's volley and about two background spikes (1 024 units at about 2 Hz over one millisecond). The readouts' response and the rest of the background come later.

**The account and the literature.** In the temporal-difference actor–critic the critic values the state the agent is in, before its action and the action's consequences (Sutton and Barto 2018). In spiking actor–critics that state is a population given to the critic as its input, place cells for instance (Potjans, Morrison and Diesmann 2009; Frémaux, Sprekeler and Gerstner 2013). The account predicts that a critic of what the network was given, rather than what it did, holds each stimulus's value apart. It should then fall after a flip as the task's critic did, and revise as fast.

**Where the engine's premise differs from those models**, so that the account is a prediction to be measured, not a result to be assumed:
- **No state population is given.** The engine has only its spikes and its rewards, and naming the stimulus's units would be the host's geometry, which ADR-0130 excludes. The window selects the state by time from the one mark the engine has, the reward, not by a population.
- **The arithmetic** is Q16.16 integers with a floor, over a fixed seeded prior, under one drive and one gain.
- **The window is a constant.** Its fit to the task depends on the host presenting the next situation soon after a reward.

## Decision Drivers

- **A measured need**: two of H-22's three reversals slower than H-21's by 7 to 11 blocks, with a value trough deeper than the task critic's, and half the value on shared units.
- **The maintainers' choice** of the window (2026-09-30), within H-22's step 5.
- **The engine's own inputs only**, as ADR-0130: the window opens at the engine's reward and its length is a rule of the engine's own anatomy, never the task's timing.
- **One change at a time**: H-22's configuration, arms, shift and scale are kept, and only the window moves.
- **Unset, bit for bit**, as ADR-0094, ADR-0114, ADR-0123 and ADR-0131 built theirs.
- **F-61 read into the clauses**: the prediction clause states what a critic of a stimulus's expected reward can hold.
- Latest ≠ Newest: an integer comparison on the executor's clock; no dependency.

## Considered Options

1. **Where the window sits**:
   - (a) the first $W$ ticks after each reward;
   - (b) a kernel that weighs a spike by its distance from the reward, falling off with a time constant;
   - (c) the last $W$ ticks before each reward.
2. **Its length $W$**:
   - (a) the shortest delay of any synapse the image carries, read from the image;
   - (b) the prior's local band's longest delay;
   - (c) the plasticity rule's pairing window, $2^{11}$ ticks;
   - (d) a length read from the task's timing.
3. **The constants**: (a) H-22's shift 9 and scale 2 kept; (b) the arithmetic run again for the window's features.
4. **The clauses**: (a) H-22's three; (b) H-22's first two, a reversal clause for the need, and a prediction clause in F-61's form.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(b).**

### The window (Specified; brief 056 builds it)

- **The rule.** With the window set, a unit's spike counts toward its feature only when it falls within $W$ ticks after the previous reward the engine received; before the first reward, within $W$ ticks after the engine's start. Every other part of ADR-0131's critic is kept: the value, the error, the step, the weights written between ticks, and every unit's count zeroed at a reward.
- **Its length.** $W$ is the shortest delay of any synapse the image carries, read by a rule written before any run.
  - Within $W$ ticks after a reward no spike of the window can have caused another spike of the window through a synapse. So the window holds what the network was given, not what it made of it.
  - At the reference prior it is expected to be 100 ticks, the local band's floor. The rule, not that estimate, sets it, and the rule is pinned before any rewarded run.
- **The parameter.** $W$ is a parameter of the image beside ADR-0131's constants. Zero, or unset, means every spike since the previous reward, which is ADR-0131's critic bit for bit. The loader refuses a window the rule does not resolve. The image format moves from 19 to 20.
- **Where it acts.** The count is taken where ADR-0131 takes it, as the coordinator merges a tick's spikes, and the window is a comparison of the tick with the previous reward's. The critic unset, nothing of the tick changes.

### The constants (option 3(a))

H-22's shift 9 and scale 2 are kept. Brief 056 restates ADR-0132's arithmetic for the window's features before any run, and holds it by the gate:
- the volley's 51 spikes move the value of the same volley by $51/2\,048$ of the error, 0.80 of the task critic's thirty-second, as before;
- about two background spikes join them.

If the restated arithmetic shows the constants no longer resolve the rule, the round stops before any rewarded run and the finding is written. The constants are not moved.

### H-23 (Hypothesis; written before any run)

**On H-22's configuration — H-21's configuration, schedule and arms with the engine's critic at shift 9 and scale 2 — with the critic's window set by its rule:**
- **clause 1, the learning holds**: in both arms every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the engine revises as fast as the task's critic let it**: in both arms each of the three reversals passes 40 of 64 within **23 blocks** of its flip, the crossing block counted. This is the slowest reversal H-20 and H-21 read with the task's critic;
- **clause 4, the critic holds a stimulus's expected reward** (F-61's form): in both arms, for each stimulus and each mapping over its last 128 trials, the engine's mean value on that stimulus's trials lies within a quarter of the reward of $(2p - 1)$ of it, with $p$ that stimulus's accuracy over those trials.

**Yes** when all four hold in both arms; otherwise **no**, with the clauses that failed.

**The account's prediction, written first**: clause 3 holds. The value then rests on each stimulus's own units and falls after a flip as the task's critic's did. The verdict is read by the clauses whatever the prediction.

**Readings, no clause**:
- the window's length as the rule read it, and the spikes per trial it admits, by group;
- the value per stimulus and block beside $2p - 1$, H-21's task critic and H-22's engine critic;
- the value's trough after each flip and the strong punishments per flip (ADR-0132's measure) beside H-21's and H-22's;
- the weights by group at every block's end;
- the reversal speeds beside H-20's, H-21's and H-22's;
- the inhibitory sum's course.

### H-23's stopping rule

1. **One round**: brief 056 builds the window, its ADR and tests, then runs H-23. The window's rule, its length and H-23's constants are committed before the first rewarded run.
2. **A calibration** stops the round before any rewarded run, and is a finding, if with the window unset any of these fails:
   - every pinned number of the tree holds, H-22's arms among them;
   - the determinism pin does not move.
3. **Yes**: the engine's own critic is named as one that learns, revises as fast as the task's critic let it, and holds a stimulus's expected reward. The next decision is an ADR choosing among the operating regime, another size, the rule held by the network reopened on this configuration, and a critic carried by a population (ADR-0130's option 1(c)), named and not taken.
4. **No on clause 1 or 2**: the window costs the learning. The next decision is an ADR weighing the window against it, with this round's readings as its need.
5. **Otherwise no on clause 3**: the shared units were not the reversals' cause, or not alone. The next decision is an ADR on how a punishment is delivered while the value stands near −1, with this round's readings as its need.
6. **Otherwise no on clause 4 alone**: the window's value does not hold a stimulus's expected reward. The next decision is an ADR on the window's length.
7. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: the need is met at the cause ADR-0132's readings point to. The value is read from what the network was given, the state a temporal-difference critic values, and every input is still the engine's own.
- Good: F-61 is carried into the clauses. The prediction clause asks what a critic of a stimulus's expected reward can hold, and the reversal clause asks for the need itself.
- Good: unset, nothing changes, and H-22's arms stand beside the round as its comparison.
- Bad: the window's fit depends on the host. A host that presents its next situation later than one shortest delay after its reward gives the critic background alone. The critic then learns only the mean reward, the baseline that reward-modulated plasticity needs at the least (Frémaux, Sprekeler and Gerstner 2010). That is a degraded critic, not a broken one.
- Bad: a format bump for one parameter.
- Neutral: two weekly tests of about H-22's cost; after ADR-0132 the plan is about a third of the bound, and the two raise it to about 37 per cent.

## Alternatives considered and why rejected

- **Option 1(b), a kernel falling off from the reward**: it keeps every spike, the shared units' included, at a lower weight, so it only shrinks the part the window removes; and it adds a time constant to choose.
- **Option 1(c), the ticks before the reward**: they hold the response and the background, the part the need points away from.
- **Option 2(b), the local band's longest delay**: within it the volley's own consequences land, the readouts' response among them.
- **Option 2(c), the pairing window**: at $2^{11}$ ticks it holds the response and some forty background spikes.
- **Option 2(d), the task's timing**: the host's knowledge, which ADR-0130 excludes.
- **Option 3(b), the arithmetic run again for new constants**: two changes at once. The kept constants still place the volley's step below the task critic's.
- **Option 4(a), H-22's clauses**: F-61 shows clause 3 was an accuracy bound. Kept as written, it would ask again what ADR-0132 already answered.

## Confirmation

`briefs/056_the-critics-window.md` builds the window and runs H-23 once. Whitepaper 4.81.0 carries H-23 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
