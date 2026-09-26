---
status: accepted
date: 2026-09-27
depends-on: ADR-0110
decision-makers: VirtualCortex maintainers
---

# ADR-0111: A rule held by the network — beside the four questions H-20's stopping rule named, the maintainers add a representation of the rule in force and take it: a context the network holds by its own activity, switched by an error, under which each mapping is kept apart; staged in three rounds — first, measured and nothing changed, whether an assembly of the reference network holds its activity after a kick and lets it go on a signal; then a readout gated by the context; then the schedule of reversals asked again with it; brief 048 measures the first

## Context and Problem Statement

[ADR-0110](0110-a-schedule-of-reversals-measured.md) read H-20 as yes. Through three reversals over 7 680 trials, H-19's configuration learned every mapping, 113 to 125 of each mapping's last 128 trials in both arms, and no coupling passed 1.30 of its image's.

It also read **no learning set**. Each reversal passed 40 of 64 in the 13th to the 23rd block after its flip, and from the first reversal to the third there was no steady speed-up:
- from the assignment: 19, then 21, then 16 blocks;
- from the mirrored assignment: 23, then 13, then 18 blocks.

Every reversal is a relearning. The old answer's pairs are punished down and the new answer's raised, because one set of couplings holds whichever mapping is in force.

Animals trained on serial reversals come to reverse in a few trials. The account that has held up is that the prefrontal cortex holds the rule in force, a task set, as persistent activity. Each mapping is learned under its own context, and a reversal becomes a switch of context, driven by the errors that follow a change, rather than a relearning (Rougier, Noelle, Braver, Cohen and O'Reilly 2005; Collins and Frank 2013).

H-20's stopping rule named the next decision as an ADR choosing among the operating regime, another size, the inhibitory drain and a critic of the engine's own. The maintainers add a representation of the rule in force, with H-20's reversal speeds as its need, and take it.

**Whether this network can hold a context at all is not known.** Its rules give reasons to doubt it, read on 2026-09-27:
- **Short-term plasticity is depression-dominated.** `crates/cortex-core/src/dynamics/plasticity.rs` sets the baseline release $U$ to 51/256, about 0.2. Facilitation recovers with $\tau_f = 2^{14}$ ticks, about 164 ms; the vesicle pool with $\tau_d = 2^{15}$, about 328 ms. A synaptic theory of working memory holds activity by facilitation that outlasts depression, $\tau_f \gg \tau_d$ (Mongillo, Barak and Tsodyks 2008). Here the order is the other way round.
- **The threshold adapts.** Each spike raises it by 0.02, decaying with $2^{12}$ ticks, about 41 ms (`THRESHOLD_STEP`, `THRESHOLD_DECAY_SHIFT`), and the refractory window is 2 ms.
- **The prior was not built to hold anything.** It is a ring lattice with a local window of eight and a quarter rewired, 32 synapses a unit, excitatory weights of 6 000 to 12 000 in Q1.15 (0.18 to 0.37), every fifth unit inhibitory. Over a learning run the inhibitory sum falls, and in H-20 it levelled at about 0.07 of the settled image's (ADR-0110), so the network a context meets late in a run is not the one it meets at the start.

## Decision Drivers

- **A measured need.** ADR-0110's reversal speeds, and no learning set.
- **The maintainers' aim** that the engine work as nervous tissue does ([ADR-0098](0098-the-integration-model.md)): a rule held as activity, as the prefrontal cortex holds one.
- **ADR-0078.** A mechanism waits for a measured need, and its first question is whether its substrate exists. A context built on a network that cannot hold activity would be a mechanism with nothing under it.
- **One change at a time.** A context, a gated readout and a switch are three mechanisms. Each is its own round, and the first changes nothing.

## Considered Options

1. **The rule held in the spiking network**: an assembly that holds its activity once kicked, switched by an error, and readouts that learn under it.
2. **The rule held in the symbolic layer**: the clause store and the executive (`cortex-reasoning`, `cortex-executive`) hold the mapping as a clause, and the task reads it. The bridge from the spike train to a clause is the discovery loop's, and nothing yet crosses back into the selection.
3. **One of H-20's named four first**: the operating regime, another size, the inhibitory drain, a critic of the engine's own.

## Decision Outcome

**Option 1, in three rounds, each decided by the one before.**

1. **Measure the substrate, and change nothing** (brief 048). On the learning line's settled image, with every weight frozen, do the following for a grid of assemblies of the network's excitatory units, sizes and recurrent weights written before any run:
   - wire each assembly as the test's own network (added recurrent synapses among its units, as ADR-0097's controls were wired);
   - measure whether it **holds** its activity after a kick;
   - measure whether it **ignites** without one;
   - measure whether it **lets go** on a signal;
   - read what an active assembly does to the rest of the network.

   No rule changes. The round writes an ADR with the region where an assembly holds and lets go, if one exists, and names the next decision without taking it.
2. **A readout gated by the context** (a later round, if the substrate holds): each readout's input conjunctive in stimulus and context, so that a mapping learned under one context is not the mapping under the other. Its design is that round's ADR.
3. **The schedule of reversals asked again** (a later round): H-20's schedule with a context and a switch on errors, asking whether a reversal becomes a switch — the first reversal a relearning, the later ones a few trials.

If no assembly in the grid holds without running the network away, the next decision is an ADR on a mechanism of persistence. Candidates are synapses whose facilitation outlasts their depression for the context's units, or a slower current. Those are rule changes, each with its own need.

H-20's other four are named and not taken: the operating regime, another size, the inhibitory drain (which ADR-0110 read as levelling at about 0.07 and 0.08 of the image's), and a critic of the engine's own.

### Consequences

- Good: the next question is the one H-20's readings raise, and its first round asks whether the network can carry the answer before anything is built on it.
- Good: the first round changes no rule, so every earlier reading stands and its tests are the tree's as they are.
- Bad: three rounds before a reversal is asked again, and the first may read that nothing holds, which moves the line to a rule change.
- Bad: the symbolic layer, which already holds clauses, is left beside the spiking network rather than joined to it.
- Neutral: the learning line's configuration is ADR-0110's, unchanged.

## Alternatives considered and why rejected

- **Option 2, the symbolic layer**: nothing crosses from a clause back into the selection. Building that crossing is a larger step than the substrate question, and would not be what nervous tissue does.
- **Option 3, one of the four first**: none of them answers the reading H-20 left, a reversal that is always a relearning.
- **Building the context and the gated readout at once**: two mechanisms with no measurement under either. A no could not say which failed.

## Confirmation

`briefs/048_an-assembly-that-holds.md` is the first round. Whitepaper 4.62.0 carries H-20's step 3 taken, §11.1's open question on a rule held by the network, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
