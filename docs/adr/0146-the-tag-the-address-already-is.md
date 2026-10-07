---
status: accepted
date: 2026-10-07
depends-on: ADR-0145
decision-makers: VirtualCortex maintainers
---

# ADR-0146: The tag the address already is — after H-27, the attention-gated feedback H-27's stopping rule names is read as what the drawn address's target side already does, a mark on the units of the channel the engine's own selection chose that says which synapses the reward consolidates; a tag carried by membrane potential is not built, for what H-27 read of moving the readouts' activity; the second step ADR-0138 planned is closed, the learning configuration stays H-25's, and the hold and the released delivery stay in the task unset

## Context and Problem Statement

[ADR-0145](0145-the-gates-output-measured.md) read **H-27 no, on clauses 1 and 3**. By H-27's stopping rule, step 5, the next decision is an ADR on the attention-gated feedback, the fallback [ADR-0141](0141-the-readouts-own-competition.md) named. **The maintainers decided it on 2026-10-07: the second step closes here.**

**What the second step was for** ([ADR-0138](0138-the-address-drawn.md)). H-25 learns with the reward's address drawn by the engine. Its sources are the units the critic's window counted. Its targets are the readout the task's selection chose: an efference copy the task composes over the body's sets, not a mechanism of the network. The second step was to replace that target side with a neural form.

**What three rounds read:**
- **H-26** ([ADR-0142](0142-the-readouts-own-competition-measured.md)): releasing the targets to every unit costs about half the learning signal. Outside the readout window, the readout that lost fires as the winner does.
- **H-27's control** (ADR-0145): the release alone learns the first mapping and reverses once in six.
- **H-27** (ADR-0145): with the gate's output delivered, the losing channel is silenced through the pair rule's span.
  - The cost became a gain, and the learning signal was 1.02 to 1.59 of H-25's.
  - The learning was still slower than H-25's: two mappings unlearned from the mirrored assignment, and four reversals of six past the bound.
  - Beside it, the readouts fired a sixth less, the inhibitory sum fell to 0.78 of the image's and was still falling, the four couplings sank to 0.93 to 0.97 of the image's as a whole, and the selection was made from a margin more than a quarter smaller.

**The attention-gated feedback, as the literature states it** (Roelfsema and van Ooyen 2005; Rombouts, Bohte and Roelfsema 2015): once an action is chosen, a feedback from the chosen output marks the units that led to it, and a global reward signal then changes only the synapses that carry the mark. The mark is what makes a global signal specific. How the mark is carried is a matter of the model.

**What the drawn address's target side is** ([ADR-0139](0139-the-address-drawn-built.md)): after the engine's own selection, the units of the chosen channel are flagged as targets, and the fan-out consolidates under the reward's signal only the synapses whose target carries the flag and whose source the engine drew. That is the attention-gated tag in its abstract form. It differs from the literature's in one respect only: the flag is the executor's, and not a state of the unit's membrane.

## Decision Drivers

- **H-27's stopping rule**, step 5, and the maintainers' decision of 2026-10-07.
- **A measured cost of moving the readouts' activity**: ADR-0145's readings of the rates, the inhibitory sum, the couplings and the selection's margin under the hold.
- **What the engine has for a membrane-carried mark**: the apical compartment, whose potential at or above 0.5 at a somatic spike starts a plateau and a burst ([ADR-0018](0018-membrane-integration.md)) and couples into the soma on every tick.
- **The line's rule against a second attempt**: H-27's constants do not move after its run.
- Latest ≠ Newest: nothing is adopted.

## Considered Options

1. **The address's target side kept**, named as the attention-gated tag in its abstract form; the second step closed.
2. **A tag carried by the apical compartment**: a feedback into the chosen channel's apical compartments, and a plasticity gate that reads the target's apical potential.
3. **The hold tried again** on the readouts' excitatory units alone, after F-63.

## Decision Outcome

**Option 1.**

- **The learning configuration is H-25's**, as [ADR-0140](0140-the-address-drawn-measured.md) named it: the excitatory synapses under the reward's gate with the signed gate set, the inhibitory ones under a baseline of their own with the inhibitory rule's target at the settled network's rate, the engine's critic with its window, and the reward's address drawn — its sources the units the window counted, its targets the channel the engine's selection chose.
- **Its target side is read as the attention-gated tag**: a mark on the chosen channel's units, set from the engine's own choice, that says which synapses a reward reaches. The readout sets stay the body's interface.
- **The second step is closed.** `Hold` and `Delivery::Released` stay in the task, unset by default, with their tests and their four weekly arms. Nothing of the engine changes.
- **What stays Specified**: whitepaper §5.2.5's lateral competition, and a mark carried by a unit's own state.

## Consequences

- Good: the learning loop is named as it stands. Per trial the host gives the reward's sign; the state, the value, the error and both sides of the address are the engine's.
- Good: three rounds' readings stand for whoever reopens the target side: the release's cost, what silencing the loser does and does not do, and what it costs the network.
- Bad: the target side is not a mechanism of the network. The mark is the executor's flag, set by the task from the gate's choice, and a network with layers between its input and its output has no such flag for them.
- Bad: why a signal as large as H-25's learned more slowly under the hold was not derived (ADR-0145), and stays open.
- Neutral: the hold's four arms stay in the weekly job, which plans about 47 per cent of its bound.

## Alternatives considered and why rejected

- **Option 2, a tag carried by the apical compartment**: it would be the literature's form. But a potential high enough to be read as a mark is one that couples into the soma on every tick, and from 0.5 it turns the unit's next spike into a burst. ADR-0145 read what moving the readouts' activity by a sixth cost. An apical mark on the chosen channel would move it too, and toward the channel just chosen, which a reversal has to leave. It is a change to the membrane's part in the learning, for a mark the address already carries.
- **Option 3, the hold again on the excitatory units alone**: F-63 found that the hold reached more than ADR-0143 counted. But the schedule and the span were H-27's constants, and its stopping rule says none moves after a rewarded run and there is no second attempt. A hold redrawn after its result is the tuning the rule exists to refuse.

## Confirmation

Whitepaper 4.92.0 carries the decision under H-27 in §11.1, and §9's row. No file under `runtime/` or `crates/` changes. `npm run spec` holds the links, the rows and the version.
