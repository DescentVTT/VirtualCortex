---
status: accepted
date: 2026-09-29
depends-on: ADR-0124
decision-makers: VirtualCortex maintainers
---

# ADR-0125: The gate raised, and a stopping rule for the line — the maintainers take the first of ADR-0124's branches, the slow current's lower gate voltage raised above the band the quiet state's fluctuations reach, as the last try of the rule held by the network, measured on the backgrounds and the core grid with nothing of the engine changed; and they write before any run the rule that pauses the line: no kick that passes, no usable cell in the core, or no robust cell under the conditions, and the rule representation stops there

## Context and Problem Statement

[ADR-0124](0124-the-slow-current-measured.md) measured [ADR-0123](0123-the-slow-current-built.md)'s slow, voltage-gated current on ADR-0117's assembly, and **no cell ran**.
- **The backgrounds were not quiet.** Fed by the prior's synapses alone, the unwired members fired at 4 to 54 Hz where ADR-0117's fire at 1.6. The slow potential stayed below its critical level on average, but the gate's lower voltage, $V_{lo}$, sat at the soma's mean under the drive's mean input, 0.436 of the threshold. The gate therefore opened on every upward excursion, by 0.13 to 0.18 on average, and each open gate multiplied the excursion by $1 / (1 - s / s_c)$, shortening the distance a fluctuation must cover to the threshold from 0.564 to between 0.51 and 0.16. In the drive's tail, where the members fire, the rate climbed 2.5 to 33 times.
- **The kick's second clause failed at every arm and shift**, so by the protocol no cell ran. The maintainers stopped the round there.

ADR-0124 named four branches: the gate's voltages, the input's shift beyond the grid, the slow time constant, and the line paused.

**The maintainers take the gate's voltages on 2026-09-29, as the last try.** The rule held by the network has run five rounds:
- ADR-0112: no assembly held;
- ADR-0115: one cell under the facilitating class;
- ADR-0117: that region one weight wide and not robust;
- ADR-0120: an inhibition that moved the threshold and not the gap;
- ADR-0124: a slow current whose gate let the quiet state's fluctuations through.

What remains cheap is a parameter. ADR-0123 made the gate's voltages a parameter of the image, so raising $V_{lo}$ changes nothing of the engine, and the round is a measurement.

**Why raising the gate could help, and why it may not.**
- **Why it could.** A gate whose lower voltage lies above most of the quiet soma's excursions stays shut on them. It opens only when the soma is held near the threshold, which is what a held state does.
- **Why it may not.** Every spike of a quiet member still crosses the band between $V_{lo}$ and the threshold on its way up, and the near misses that peak inside that band are pushed over. In a regime where the rate is set by the fluctuations' tail, a small shortening of the last part of the path can still multiply the rate, as ADR-0124 read 2.5 times at a slow potential a tenth of the critical level. The round measures which of the two the network does.

## Decision Drivers

- **A measured need**: ADR-0124's backgrounds, and the gate open on the quiet state's excursions.
- **The maintainers' choice**: the gate raised, as the last try, with a rule that pauses the line written first.
- **Nothing of the engine changes.** $V_{lo}$ is a parameter of the image (ADR-0123), and every rule is the tree's.
- **The weekly job's budget.** The cost table regenerated from ADR-0124's dispatch is 100 tests and 42 699 s, about 50 per cent of the bound a shard at six shards. That leaves about 8 000 s under the briefs' 60 per cent, and condition (c) alone rebuilds H-20's arm at about 1 900 s. So this round measures the backgrounds and the core grid, and the conditions follow in a round of their own if the core gives them a cell.
- **Five rounds.** A line that has read no robust cell in five rounds needs a rule that says when it stops. The rule has to be written before the run that could trigger it.

## Considered Options

1. **What is raised**: (a) $V_{lo}$ alone, $V_{hi}$ at the threshold's base; (b) both; (c) the gate made a step.
2. **Where**: fixed fractions of the threshold; or fractions read from the quiet soma's distribution.
3. **What this round runs**: the backgrounds, the core grid and the conditions in one round; or the backgrounds and the core grid, with the conditions after.
4. **The stopping rule**: none; or one written before any run.

## Decision Outcome

**Options 1(a), 2 fixed fractions with the distribution read beside them, 3 the backgrounds and the core grid, and 4 a rule written first.**

### The gate (the image's parameter; nothing of the engine changes)

- **$V_{lo}$ at 0.7, 0.8 and 0.9 of the threshold's base**, $V_{hi}$ at the base, 1.0. Every other slow constant is ADR-0124's: $\tau_s = 2^{13}$ ticks and input shifts of 0, 1 and 2.
  - These are fractions of the threshold rather than readings of the soma, so they do not move with the arm or the shift.
  - Beside them the round reads, before any run with the slow current, the quiet soma's distribution on ADR-0117's background: the fraction of the members' ticks above each of 0.5 to 0.95 of the threshold. That places the three voltages against the band the fluctuations reach.
- **The arms are ADR-0124's**: (A) the slow current under ADR-0019's short-term plasticity, and (B) the slow current with ADR-0114's set (ii).

### The round (brief 053): the backgrounds, then the core grid

- **The backgrounds and controls** for every arm, shift and $V_{lo}$: eighteen pairs, as ADR-0124's (the background marked and unwired, the drive alone; the control grown, marked and unwired, with the kick and the release). Each kick is read on the engine by ADR-0112's measure.
- **The cells**, only where the kick passes, as ADR-0124's protocol requires: recurrent weights of 0.125, 0.25, 0.375 and 0.5.
  - If more than four of the eighteen pass, the four whose backgrounds' members fire slowest run.
  - Ties go to the higher $V_{lo}$, then the larger shift, then arm (A).
  - So at most sixteen cells run.
- **ADR-0117's protocol, measures, thresholds and rules** stand: holds, ignites, lets go, spills and usable, the core condition only.
- **The arithmetic first**, with the slow rule itself:
  - the critical slow potential at each $V_{lo}$;
  - the gate's opening at the drive's mean standing and in the band between $V_{lo}$ and the threshold;
  - the held and the quiet states' slow potentials, and their contrast, at each arm, shift and weight, as ADR-0124's.

### The rule that pauses the line (written before any run)

1. **If no kick passes** at any arm, shift and $V_{lo}$, the gate cannot keep the quiet state quiet at any voltage tried, and **the line pauses**.
2. **If cells run and none is usable in the core**, **the line pauses**.
3. **If some are usable**, the next round runs ADR-0117's conditions (b), (c) and (d) on them, at most eight, as ADR-0124's rule picks them, with its own budget. If none is robust there, **the line pauses**.
4. **If a cell is robust**, the next decision is ADR-0111's second round, a readout gated by the context, with ADR-0117's reading that the prior carries a held context to both readouts alike as its need.

**The line paused** means an ADR that records it:
- what was built stays, unset bit for bit: the facilitating class (ADR-0114) and the slow current (ADR-0123);
- the readings of the five or six rounds are its record;
- the next decision returns to H-20's four open questions (the operating regime, another size, the inhibitory drain, a critic of the engine's own) or ADR-0111's option 2, the rule held in the symbolic layer, named and not taken.

## Consequences

- Good: the last try costs a parameter and a measurement, and every earlier reading stands.
- Good: the line has a rule that ends it, written before the run that can trigger it.
- Bad: the conditions wait a round, so a usable cell in this round is not yet a robust one.
- Bad: the gate may still let the near misses through; the round may read no kick that passes.
- Neutral: whatever the verdict, the two mechanisms built for the line stay in the engine, unset, for a later need.

## Alternatives considered and why rejected

- **Option 1(b), $V_{hi}$ raised too**: above the threshold the soma fires and resets, so a gate that opens past the threshold's base opens on nothing the soma holds.
- **Option 1(c), a step**: ADR-0122 rejected it; the fluctuations cross one voltage both ways.
- **Option 2, voltages read from the distribution**: they would move with every background, where fixed fractions compare across arms and shifts. The distribution is read beside them.
- **Option 3, the conditions in this round**: the budget, above.
- **No stopping rule**: a line that ends only when someone decides it has ended is decided after a run.
- **ADR-0124's other branches**: a larger input shift meets the staircase (F-57) and starves the held state as well; a different slow time constant is not what the backgrounds point at.

## Confirmation

`briefs/053_the-gate-raised.md` measures the backgrounds and the core grid. Whitepaper 4.74.0 carries this decision and the stopping rule in §11.1's question on a rule held by the network, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
