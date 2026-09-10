---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0002
---

# ADR-0018: Membrane integration — shift leaks with a one-LSB floor, difference coupling, a 2 ms refractory window, apical-gated plateaus, adaptive threshold

## Context and Problem Statement

`DendriticSuperNeuron` carried the fields of a two-compartment leaky integrate-and-fire unit (`v_soma`, `v_basal`, `v_apical`, `v_thresh`, `bac_plateau_ticks`, `refractory_ticks`, `last_soma_spike_tick`, `flags`) and no method updated any of them; whitepaper §8.8 gave the continuous equation and R-1 step 5 was Specified. Brief 011 asked for the discrete, integer, saturating update, with its fixed points and its stability at the 10 µs tick, and for the BAC coincidence and threshold adaptation to be Implemented or explicitly left Specified. Which discrete form, which constants, and what happens on the tick of a spike?

## Decision Drivers

- [ADR-0002](0002-q16-16-fixed-point.md): Q16.16, saturating, widened multiplication; no division on the hot path.
- Every decay by right shift stalls at $2^k - 1$ LSB unless it takes at least one LSB per step (the lesson recorded by [ADR-0016](0016-thirty-two-crate-architecture.md)); a potential must reach rest.
- §8.7: an image at rest holds zeros, so rest must be zero and the resting record must not fire.
- §8.4: `last_soma_spike_tick` is a `u32` that wraps; comparisons are wrapping differences.
- Axiom A3: the worker holding the turn is the only writer of the plain fields, so `integrate` takes `&mut self` and never touches the atomics (ADR-0017's domain).
- A test must pin every rule: the fire tick under constant drive, the fixed point under sub-threshold drive, the refractory count, saturation, the wrap, determinism, the plateau, the adaptation.

## Considered Options

1. A single-compartment leak on the soma with the dendrites as instantaneous inputs.
2. **Per-compartment shift leaks; the soma receives a $2^{-4}$ fraction of its potential difference to each compartment per tick (a conductance, so the soma never overshoots a compartment); a hard refractory window that drops inputs; a plateau gated by apical depolarisation at the spike; threshold adaptation toward a base.**
3. The continuous equation integrated with a fixed-point exponential table.

## Decision Outcome

Option 2, with potentials relative to rest (rest is 0).

- **Leaks.** Soma $2^{-11}$ per tick ($\tau_m \approx 20.5$ ms), basal $2^{-9}$ (5.1 ms), apical $2^{-10}$ (10.2 ms); each leak takes at least one LSB, so a displaced potential reaches exactly zero from either sign.
- **Inputs.** The basal and apical inputs of the tick are added to their compartments (saturating). In the refractory window they are dropped.
- **Coupling.** The soma gains $(v_b - v_s) \gg 4$ and $(v_a - v_s) \gg 4$ per tick. Fixed points under constant per-tick input $I_b$: $v_b^* = 2^9 I_b$; with no apical drive $v_s^* \approx v_b^*/2$, since the two coupling terms weigh the soma against each compartment equally and the leak is negligible beside them. Constant $I_b = 128$ LSB (0.002 per tick) settles the soma near 0.5 and never fires; $I_b = 512$ drives it through 1.0 after about $2^9 \ln 2 \approx 355$ ticks plus the coupling lag, which the test bounds to 300 to 600 ticks.
- **Threshold.** A somatic potential at or above `v_thresh` fires when `v_thresh` is positive. A threshold at or below zero is an unconfigured unit and never fires: the resting record fails closed.
- **Spike.** `last_soma_spike_tick` is stamped with the tick; the soma resets to −0.25; the refractory window is 200 ticks (2 ms); the threshold steps up by 0.02.
- **BAC plateau.** If the apical compartment is at or above 0.5 at the spike, a plateau of 200 ticks begins (or is refreshed), bit 0 of `flags` is set, the apical coupling is $2^{-2}$ for its duration, and the refractory window is 50 ticks, so the unit bursts at up to 200 Hz. The plateau counts down every tick and clears the flag at zero.
- **Adaptation.** `v_thresh` decays toward the base 1.0 by $2^{-12}$ of its excess per tick (at least one LSB), so it returns exactly to the base; a threshold below the base is left alone, since a lower configured threshold is a configuration, not an excess.
- **Wrap.** `ticks_since_spike(now)` is `now.wrapping_sub(last_soma_spike_tick)`.
- **Order within a tick.** Plateau countdown, threshold decay, refractory countdown, compartment update, soma update, threshold test, spike effects. The test that pins the refractory count depends on this order and states it.

### Consequences

- Good: eight integer operations per compartment per tick, no multiplication, no table; a spike is a few stores.
- Good: every rule has a fixed point the tests reach exactly, and a determinism test over 10⁵ ticks of a fixed pseudo-random trace.
- Bad: a one-LSB leak floor makes potentials below $2^{\text{shift}}$ LSB decay linearly rather than exponentially; at the magnitudes the model runs at (mV-scale potentials are thousands of LSB) the floor is invisible, and the alternative is a stall at rest.
- Bad: the coupling is not charge-conserving; the compartments drive the soma but the soma does not drain them. A conserving form is a change to the same method under a new ADR if a mechanism needs it.
- Bad: constants are `const` items of `cortex-core`, one unit type for the whole engine. Per-type constants are a record question ([ADR-0016](0016-thirty-two-crate-architecture.md)'s test).

## Alternatives considered and why rejected

- **Option 1** loses the apical compartment's role in BAC firing, which is the point of the two-compartment record.
- **Option 3** needs a table or a multiplication per compartment per tick for a gain the shift form does not need; the shift form's time constants are powers of two, which is a coarse but honest grid.
- **Inputs accumulated during the refractory window** would let a burst of input fire the unit the tick the window ends, which the hard window of §8.8 is meant to prevent.

## Confirmation

Ten tests in `cortex-core` (`dynamics/membrane.rs`): leak to exact rest from both signs in every compartment; sub-threshold fixed point below threshold; a supra-threshold fire tick in the predicted window with a monotone rise, then periodic firing outside the refractory window; refractory inputs dropped and the window ending on the stated tick; saturation at both extremes; the unconfigured unit is silent; the wrap; determinism over 10⁵ ticks; the plateau and burst; threshold adaptation to the base exactly. A benchmark `neuron/integrate` measures the tick. `npx spec-guard` asserts `fn integrate` exists in `cortex-core`.
