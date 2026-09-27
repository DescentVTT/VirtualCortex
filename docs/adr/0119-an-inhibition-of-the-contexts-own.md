---
status: accepted
date: 2026-09-27
depends-on: ADR-0117
decision-makers: VirtualCortex maintainers
---

# ADR-0119: An inhibition of the context's own — among the branches ADR-0117 named for a region no condition leaves robust, the maintainers take the context's own inhibition: every member of ADR-0117's assembly receives added inhibitory synapses from the network's inhibitory units, so that the inhibition it meets rises when the network it sits in does, and its recurrent weight is raised against it; brief 051 measures a grid of recurrent and inhibitory weights under ADR-0117's four conditions, nothing of the engine changed

## Context and Problem Statement

[ADR-0117](0117-the-region-measured.md) measured the region ADR-0116 asked for and read **no cell robust**.
- **The region is one weight wide.** Under set (ii), 64, 80 and 96 marked members hold at a quarter of Q1.15's range. A sixteenth lighter nothing holds; a sixteenth heavier every size ignites without a kick.
- **Two conditions tip the usable cells into igniting.** On the image H-20's arm leaves, the members' background rose 13 per cent and 64 and 96 units ignited in 2 and 4 rounds of 8. Under the task's stimulus every epoch, the rest of the network fired 24 per cent above the settled network and both sizes ignited in 4 rounds. **Neither condition kept a cell from holding; each pushed it over the igniting edge.**
- **Under another draw of the delays** 64 units stayed usable and 96 spilled.
- **A held context reaches both readouts alike** through the prior, 1.4 to 6.9 spikes a trial more on each.

ADR-0117 named the branches: the context's isolation from the task's stimulus, the inhibitory drain or an inhibition of the context's own, and the class's constants. **The maintainers took the context's own inhibition on 2026-09-27.** It is the one branch aimed at both failing conditions:
- isolation answers (d) only;
- the drain answers (c) only;
- the class's constants move the priming's contrast, which by the replica of [ADR-0113](0113-a-facilitating-class-of-synapses.md) rises from 1.60 to at most about 1.9 times the unkicked product at a release fraction of 13/256 and a depression of $2^{12}$ ticks.

**Why inhibition, by the account the choice rests on.** The unkicked assembly ignites when the drive's fluctuations bring enough members over threshold at once; a burst then primes it, and it holds. A condition that raises the members' excitatory input narrows the margin to that edge. Two properties of an added inhibition could widen it:
- **It scales the margin.** In attractor networks, a persistent state is stable against noise when strong recurrent excitation is held in check by inhibition (Amit and Brunel 1997). Raising the recurrent weight and the inhibition together keeps the primed-to-unkicked contrast and moves both edges away from what a small change of the drive can cross.
- **It tracks the network.** Inhibition driven by the network's own inhibitory units rises when the network does, so part of an extra drive is cancelled at the members themselves, as in a balanced network (van Vreeswijk and Sompolinsky 1996). The drained network and the task's stimulus raise the whole network's activity, the inhibitory units' with it.

Both are reasons for the choice, not a prediction: the arithmetic of brief 051 comes first, and the round may read that inhibition keeps the assembly from holding at all.

## Decision Drivers

- **A measured need**: ADR-0117's readings, above.
- **The maintainers' choice** of the context's own inhibition (2026-09-27).
- **Nothing of the engine changes.** The inhibition is the test's own wiring, as the assembly's recurrent synapses are: blocks appended to an inhibitory unit's chain, holding synapses of that unit's sign (Dale's principle, [ADR-0049](0049-dale-principle-in-plasticity.md)).
- **One difference from ADR-0117.** Its substrate, geometry, class, protocol, measures, thresholds, conditions and rules stand; the added inhibition and the recurrent weights it is set against are the difference.
- **The weekly job's budget**, now five shards ([ADR-0118](0118-a-fifth-shard.md)).

## Considered Options

1. **Where the inhibition comes from**:
   - (a) the network's inhibitory units, onto the members, an input the network drives;
   - (b) a pool the members drive, onto the members, a feedback the context's own activity drives.
2. **Which inhibitory units**: those within the prior's window of each member, about three or four; or a number drawn over the ring.
3. **The grid**: the added inhibition alone at ADR-0117's weight; or the recurrent weight raised against it.

## Decision Outcome

**Options 1(a), 2 drawn over the ring, and 3 the two raised together.**

### The inhibition (the test's wiring; brief 051 builds it)

- **Sixteen sources a member.** Each member of the assembly receives sixteen added basal synapses, one from each of sixteen of the prior's inhibitory units, drawn without repetition over the ring by a seed written before any run. The weight is the cell's inhibitory weight, and the delays come from the prior's local band.
  - The prior's window holds only three or four inhibitory units of a member, too few to carry an input the network's activity sets.
  - Drawn over the ring, the sources fire as the whole network does, which is what (c) and (d) raise.
- **The sources' own firing is unchanged.** The added synapses leave the sources and reach only the members, so the task's readout places, where every inhibitory unit of the ring sits, fire as they did.
- **Chained after each source's own chain**, as ADR-0112's assembly synapses are chained after each member's, in blocks appended to the image. Every added synapse is frozen with the rest.

### The round (brief 051)

On ADR-0117's substrate: 64 members under set (ii) at ADR-0112's placement on ADR-0116's geometry, every weight frozen, ADR-0115's protocol and measures, and ADR-0117's four conditions:
- the core (the settled image, delay seed 48, no task);
- (b) the delays of seed 49;
- (c) the image H-20's arm leaves, frozen as ADR-0117 froze it;
- (d) the task's stimulus at every epoch's middle.

- **The grid**: recurrent weights 0.25, 0.375, 0.5 and 0.75 of Q1.15's range, and added inhibitory weights of a quarter, a half and the whole of the range (negative, by Dale's principle). That is twelve cells, and beside them ADR-0117's cell, 64 units at 0.25 with no added inhibition, reproduced bit for bit.
- **The backgrounds**: a background and a control per condition and inhibitory weight, the added inhibition wired and the recurrent synapses not. The inhibition changes what the members fire alone.
- **The arithmetic first**, at the background's rates:
  - the members' mean input from the prior's excitatory and inhibitory synapses;
  - the members' mean input from the assembly at the unkicked and at the primed product, at each recurrent weight;
  - the members' mean input from the added inhibition at each inhibitory weight;
  - under (c) and (d), how far the sources' rates rise, and how much of the members' extra excitatory input the added inhibition takes back, from ADR-0117's rates.
- **The rules**, ADR-0117's:
  - **robust**: usable under all four conditions;
  - **the cell for the second round**: of the robust cells, the one with the most usable neighbours in the core grid, one recurrent step and one inhibitory step either way; ties to the lighter recurrent weight, then the lighter inhibition;
  - **if none is robust**: what failed under which condition.
- **The next decision, named and not taken**:
  - if a cell is robust, ADR-0111's second round, a readout gated by the context, with ADR-0117's reading that the prior carries a held context to both readouts alike as its need;
  - if none is, the feedback inhibition (option 1(b)), the context's isolation, the class's constants, or the inhibitory drain, with this round's readings as the need.

## Consequences

- Good: one mechanism aimed at both conditions that failed, measured under the same conditions and rules as the reading it answers.
- Good: nothing of the engine changes, and every earlier reading stands.
- Bad: the context's inhibitory synapses are inhibitory synapses. Under the learning configuration's inhibitory baseline they would consolidate and drain as the prior's do. A round that runs the task with a context has to decide whether they learn, and that is a rule question this round does not answer; here they are frozen.
- Bad: the inhibition may keep the assembly from holding where it keeps it from igniting. The grid raises the recurrent weight against it for that reason, and the round may read no region at all.
- Neutral: the sources are readout places of the task's geometry. Their firing is unchanged, but a later round that reads the readouts reads units that also inhibit the context.

## Alternatives considered and why rejected

- **Option 1(b), a feedback pool**: an inhibition the members drive rises only once they fire. It cuts a burst short, the spontaneous one and the refreshing one alike, and does not answer an extra drive before it acts. It is named as the next branch should this one read nothing robust.
- **Option 2, the window's inhibitory units**: three or four sources a member, at the network's rate of about 1.7 Hz, carry too few events to stand for the network's activity.
- **Option 3, the inhibition alone at 0.25**: at ADR-0117's weight the assembly holds with less than a sixteenth to spare below it, so any inhibition would read as "never holding". The recurrent weight must rise with it.
- **The isolation, the drain or the class's constants first**: each answers one condition, or moves a contrast the replica bounds.

## Confirmation

`briefs/051_an-inhibition-of-the-contexts-own.md` measures the grid. Whitepaper 4.69.0 carries this decision in §11.1's question on a rule held by the network, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
