---
status: accepted
date: 2026-09-28
depends-on: ADR-0120
decision-makers: VirtualCortex maintainers
---

# ADR-0122: A slow, voltage-gated current — after four rounds that read the same bottleneck, the maintainers take ADR-0111's other candidate for persistence: a unit may be marked to carry a slow current that integrates its excitatory synaptic input over tens of milliseconds and reaches the soma only as far as the soma is depolarised; its constants a parameter of the image, its rule beside `integrate`, which is untouched; unset, every run is the run it was; brief 052 builds it and measures ADR-0117's assembly with its members marked

## Context and Problem Statement

ADR-0111 asked whether an assembly of the network can hold a context, and named two candidates for persistence: synapses whose facilitation outlasts their depression, or a slower current. Four rounds have measured the first.
- **ADR-0112** ([brief 048](../../briefs/archive/048_an-assembly-that-holds.md)): with ADR-0019's short-term plasticity no assembly holds. A kick sets off one burst that drains the pool.
- **ADR-0115**: under ADR-0114's facilitating class one cell holds, by a train of bursts refreshed inside the class's primed window.
- **ADR-0117**: the usable region is one weight wide, and a drained network or the task's stimulus tips it into igniting.
- **ADR-0120**: an inhibition of the context's own, driven by the network's inhibitory units, moves the threshold and not the gap. At a quarter of the range nothing holds; from three eighths up every cell ignites; the heaviest inhibition makes the ignitions rarer and does not stop them.

**The four readings name one bottleneck.** In each, what separates a held assembly from a quiet one is the release its synapses make per spike: at most 1.38 to 1.60 times the unkicked release by the class's arithmetic, and 1.40 to 1.45 read over the held stretches. What starts a hold, a kick or a burst of the drive's fluctuations, then sustains it by the same regenerative bursts. So a change that acts on the held and the quiet state alike moves both edges together, as ADR-0120's inhibition did, and the gap between them stays as narrow as that ratio.

**A slow, voltage-gated current acts on the two states differently**, by two properties the fast synapses do not have (Lisman, Fellous and Wang 1998; Wang 1999):
- **It integrates rate, not spikes.** A current that sums a unit's excitatory synaptic input over tens of milliseconds follows how fast the unit's inputs fire. A held assembly's members fire each other several times faster than a quiet one's, so the current's contrast between the two states follows that rate ratio, not the per-spike release ratio. And its input is a steady depolarisation rather than a coincidence, so it can hold a member without regenerative bursts that drain the pool.
- **It is gated by the unit's own voltage.** The NMDA receptor's channel is blocked by magnesium at rest and unblocked by depolarisation (Jahr and Stevens 1990). A current gated that way barely reaches a soma the drive holds near its mean, so the quiet state's background input does not accumulate into an ignition. It reaches a soma already driven near threshold, which is the held state.

The maintainers chose this route on 2026-09-28. ADR-0120's own branches — a feedback inhibition, the context's isolation, the class's constants, the inhibitory drain — each act on both states or on one condition.

### What the engine has, read on 2026-09-28

- **The membrane** (`crates/cortex-core/src/dynamics/membrane.rs`, [ADR-0018](0018-membrane-integration.md)). The basal and apical compartments leak with $2^9$ and $2^{10}$ ticks and couple to the soma by a sixteenth of their difference a tick. The soma leaks with $2^{11}$ ticks and fires at an adapting threshold of 1.0, and a spike resets it to −0.25. **In the refractory window the compartments' inputs are dropped.** No compartment is slower than the soma, and none is gated by voltage.
- **The messages** (`crates/cortex-core/src/dynamics/synapse.rs`). Every message a unit receives carries its efficacy, whether it lands in the apical compartment, and whether a synapse sent it (`message_is_synaptic`). The drive's messages and the task's stimulus and cancel are injected, not synaptic. The executor's turn sums a unit's messages into its basal and apical inputs, scaled by the gain (`executor.rs`).
- **The record** (`DendriticSuperNeuron`). `[20..24)` is `_reserved_20`, "MUST be zero", and an image whose record holds anything else there is not at rest (`is_at_rest_image`). `flags` carries bits 0 to 2: burst mode, inhibitory, facilitating (ADR-0114).
- **The image.** The modulator section holds ADR-0114's class at `[32..36)`; `[36..64)` are reserved and refused unless zero.

## Decision Drivers

- **A measured need**: four rounds that read one bottleneck, the per-spike release ratio.
- **The maintainers' choice** of the slow current (2026-09-28).
- **Unset, bit for bit**, as ADR-0094 and ADR-0114 built their mechanisms: the rule beside `integrate`, which is not touched; the mark a bit; the constants a parameter of the image.
- **ADR-0016's test.** A unit's slow potential is a quantity of the unit, held in its record's reserved bytes, not a crate.
- **The hot path.** `integrate` is the turn's work (ADR-0104, F-51). The slow current must cost an unmarked unit nothing it does not cost now.
- **One change at a time.** ADR-0117's substrate, geometry, conditions and rules stand; the mark and its constants are the difference.
- **Latest ≠ Newest.** A published account of persistent activity, as a rule in Q16.16 integers; no dependency.

## Considered Options

1. **What the slow current integrates**:
   - (a) every excitatory synaptic message the unit receives in its basal compartment, the drive and every injected message excluded;
   - (b) only the synapses of a marked type, a bit in each `SynapseBlock`;
   - (c) the basal potential itself, the drive included.
2. **How it reaches the soma**:
   - (a) as a current, the slow potential times a voltage gate, positive only;
   - (b) as a compartment coupled by the difference, gated.
3. **In the refractory window**: the slow potential takes its input or drops it.
4. **The gate**: a step at one voltage; piecewise linear between two voltages; a table.
5. **The short-term plasticity the measurement pairs it with**: ADR-0019's constants alone; ADR-0114's set (ii) alone; both, as two arms.

## Decision Outcome

**Options 1(a), 2(a), 3 takes its input, 4 piecewise linear, and 5 both arms.**

### The slow current (Specified; brief 052 builds it)

- **The mark.** A bit of `DendriticSuperNeuron::flags`, beside bits 0 to 2, which `integrate` and its sibling leave as they find them. A marked unit integrates under the slow rule; every other unit under `integrate`, bit for bit.
- **The state.** The slow potential $s$, Q16.16, in the record's `[20..24)`. It stays zero in every unmarked unit, so an image with no marked unit is the image it was. The record's field and whitepaper §5.2's table change, and the format moves from 17 to 18.
- **The constants**, a parameter of the image in the modulator section's reserved bytes, each refused outside what the rule resolves:
  - the slow leak's shift $k_s$, $\tau_s = 2^{k_s}$ ticks;
  - the input's shift $g$, the slow potential taking $2^{-g}$ of its input;
  - the gate's two voltages $V_{lo} < V_{hi}$, each in $(0, 1]$ of the threshold's base.

  Unset, no unit may be marked: the loader refuses a marked unit. Set with no unit marked, nothing reads the constants.
- **The input.** For a marked unit, the executor's turn also sums the positive efficacies of the synaptic messages landing in the basal compartment: the excitatory synapses' releases, never the drive's or any injected message. It scales that sum by the gain as it scales the basal input, and passes it to the slow rule. An unmarked unit's turn adds nothing: no word, no allocation and no syscall is added to the tick.
- **The rule**, beside `integrate`, which is not touched. Each tick, before the soma's update:
  - $s \leftarrow \operatorname{leak}(s, k_s) + \text{input} \cdot 2^{-g}$, saturating, the input taken in the refractory window too (option 3). The channel's opening is its input's; the soma's refractoriness is the soma's;
  - the gate: $\gamma(v) = \operatorname{clamp}\bigl((v - V_{lo})/(V_{hi} - V_{lo}), 0, 1\bigr)$, read on the soma before this tick's update;
  - the soma receives $\gamma(v_{\text{soma}}) \cdot s \cdot 2^{-4}$ beside its coupling to the basal and apical compartments. That is the coupling's own fraction, and only while $s > 0$.

  Everything else is `integrate`'s. With $s$ at zero and no input the rule is `integrate` bit for bit, which a property test holds over the lattice of `testkit/prop.rs`.
- **The image.** `FORMAT_VERSION` moves from 17 to 18. A pin of a whole image moves with the format number and nothing else, restated under a masked check as ADR-0095 did.

### The measurement (brief 052)

On ADR-0117's substrate: 64 members at ADR-0112's placement on ADR-0116's geometry, every weight frozen, ADR-0115's protocol and measures, and ADR-0117's four conditions and rules.
- **Two arms**: every member marked for the slow current, (A) under ADR-0019's short-term plasticity, and (B) marked facilitating under ADR-0114's set (ii) as well. (A) asks whether the slow current alone holds a context; (B) whether it widens the one cell ADR-0117 found.
- **The slow constants**: $\tau_s = 2^{13}$ ticks (82 ms, within the NMDA current's tens to a hundred milliseconds), $V_{lo}$ and $V_{hi}$ at the soma's standing under the drive's mean input and at the threshold's base, and three input shifts $g$ placed by the arithmetic before any run.
- **The grid**: recurrent weights of 0.125, 0.25, 0.375 and 0.5 × the three input shifts × the two arms, twenty-four cells, run first under the core condition.
- **The conditions**: every cell usable in the core is run under (b), (c) and (d), and **robust** is usable under all four, by ADR-0117's rule. If more than eight cells are usable in the core, the eight with the most usable neighbours are run, by ADR-0117's order. This rule is written before any run, so the conditions' cost follows the core's reading and not a choice made after it.
- **The arithmetic first**, at the backgrounds' rates:
  - the slow potential's steady level under the background's synaptic input, and at a sustained 5, 10 and 20 Hz of the assembly, at each weight, shift and arm;
  - the gate's opening at the drive's mean standing and near the threshold;
  - the current each delivers to the soma beside the gap from the mean standing to the threshold, 0.564;
  - so, **the contrast between the held and the quiet state's slow current**, beside the per-spike ratio of 1.4 to 1.6 the four rounds were bounded by.
- **The next decision, named and not taken**:
  - if a cell is robust, ADR-0111's second round, a readout gated by the context, with ADR-0117's reading that the prior carries a held context to both readouts alike as its need;
  - if none is, what the readings name: the gate's voltages, the slow time constant, or the line paused.

## Consequences

- Good: the one candidate ADR-0111 named and the four rounds did not try, aimed at the bottleneck they read: a contrast that follows the rate ratio and a gate that separates the held state from the quiet one.
- Good: unset, nothing changes, by construction and by a property test; the substrate, conditions and rules are ADR-0117's.
- Bad: a rule beside the membrane's, the engine's hottest code, a record field and a format bump. The unmarked path must be shown unchanged, and its cost is not measured by this round.
- Bad: the slow potential integrates every excitatory synaptic input of a marked unit, the prior's as well as the assembly's, so a member's background input charges it. The gate is what keeps that from mattering, and the arithmetic says how far it does.
- Bad: three rounds at most before a readout is gated, and one of them a build with a mutation sweep.
- Neutral: the slow current is a rule of the class of units marked, not of the task. Whether a context's units should carry it in a learning run is the round's after this.

## Alternatives considered and why rejected

- **Option 1(b), a synapse type**: a bit in every `SynapseBlock`, changing every block of every image, where the unit's mark and the synaptic bit the messages already carry suffice.
- **Option 1(c), the basal potential**: it carries the drive, whose mean holds the basal at 0.875. A slow copy of it would be a standing depolarisation in the quiet state, the opposite of the gate's purpose.
- **Option 2(b), a coupled compartment**: at $s = 0$ it would pull an open-gated soma toward zero, so the rule would not be `integrate` there.
- **Option 3, dropping the input in the refractory window**: a unit firing at 20 Hz is refractory for 4 per cent of the time, and at 100 Hz for 20. The channel would lose the input of the state it exists to hold.
- **Option 4, a step or a table**: a step gives the soma a jump at one voltage the fluctuations cross both ways; a table is constants the arithmetic cannot state as two numbers.
- **Option 5, one arm**: arm (A) alone would not say whether the current widens the one cell found, and arm (B) alone could not tell the current from the class.
- **ADR-0120's branches**: each acts on both states alike or on one condition.
- **Pausing the line**: the maintainers chose to try the candidate aimed at the bottleneck first.

## Confirmation

`briefs/052_a-slow-voltage-gated-current.md` builds the current and measures it. Whitepaper 4.72.0 carries this decision in §11.1's question on a rule held by the network, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
