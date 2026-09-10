---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0015
---

# ADR-0027: Vocal synthesis as motor output and computational humor — a `VocalFrame` with an integer source–filter renderer in `cortex-embodiment`, the benign-violation appraisal in `cortex-affect`, reward in `cortex-neuromod`, and the playful marker in `cortex-linguistic`

## Context and Problem Statement

The same directive asked for "neuromorphic vocal expression" (speech as continuous motor control in the 1 ms loop, a `no_std` Q16.16 formant generator with fundamental, three formants, jitter, shimmer and aspiration, and a voice that moves with the context) and "ubiquitous humor" (the benign-violation condition, incongruity above a threshold with no malicious threat; a dopamine release; a comedic rollout; witty rejoinders), with fields proposed in `AgentPerspectiveState` and acoustic parameters in "motor channel arrays". It asked that neither be confined to a game or a role, and that the veto gate of `cortex-ethics` remain the only boundary. Which of this is a rule, who owns each quantity, and what is claimed?

## Decision Drivers

- `cortex-embodiment` owns egress frames (`TorqueFrame`, [ADR-0015](0015-embodiment-frame-abi.md)); an audio device is an actuator behind the ring like a joint. A vocal frame is a second egress record under [ADR-0016](0016-thirty-two-crate-architecture.md)'s test: a named gap, one owner, a mechanism in §8.8, widths that fit, no boundary crossed.
- The source–filter model of speech (Fant, 1960; Klatt, 1980) is a small integer computation: an excitation and second-order resonators whose coefficients are $e^{-x}$ and $\cos\theta$, both series that Q16.16 holds to $3 \times 10^{-5}$ over the ranges a formant below a quarter of the sample rate and a bandwidth below a thirty-second of it need.
- Amusement is an appraisal of the moment (McGraw and Warren, 2010: a violation that is benign); appraisals of the body's moment are `cortex-affect`'s, not `cortex-agency`'s, whose record attributes actions to self or other.
- Determinism (§8.3): jitter, shimmer and noise come from a seeded generator, never from a random source.
- Rule L-6.

## Considered Options

1. The parameters in `TorqueFrame`'s reserved bytes and the renderer in the runtime; humor fields in `cortex-agency` as proposed.
2. **A `VocalFrame` record with a `VocalSynth` renderer in `cortex-embodiment`; the appraisal (`appraise_incongruity`, `mirth_q16`) in `cortex-affect`; `reward` and `decay_dopamine` in `cortex-neuromod`; `wander_at` in `cortex-imagination`; `PROSODY_PLAYFUL` and `mark_play` in `cortex-linguistic`, refused in a formal register; the voice shaped by the marker, the register and the valence.**

## Decision Outcome

Option 2. Image format version 8 (shared with [ADR-0026](0026-social-acumen-and-re-representation.md)); frame ABI version 2.

- **`VocalFrame`** (64 B): `epoch`, `f0_hz_q16` (Q16.16 Hz), `formant_hz: [u16; 3]`, `bandwidth_hz: [u16; 3]`, `amplitude_q1_15`, `jitter_q0_8`, `shimmer_q0_8`, `aspiration_q0_8`, `voicing`, `sample_rate_hz`, `seed`, 28 reserved. `new(epoch)` is the neutral voice: 120 Hz, formants 500 / 1 500 / 2 500 Hz with bandwidths 60 / 90 / 120 Hz, amplitude 0.5, voiced, 16 kHz, seeded from the epoch. One frame per epoch, so the engine moves the voice at the period it moves the joints; the samples of an epoch are the renderer's.
- **`Resonator`**: $y_n = x_n + B y_{n-1} + C y_{n-2}$ with $B = 2 e^{-\pi\,\text{bw}/f_s}\cos(2\pi f/f_s)$, $C = -e^{-2\pi\,\text{bw}/f_s}$; $e^{-x}$ by $1 - x + x^2/2 - x^3/6$, $\cos\theta$ by the series to $\theta^8$, $\pi$ as 205 887; refused for a zero rate, a zero bandwidth (an undamped pole), a formant above a quarter of the rate (outside the cosine series' range) or a bandwidth above a thirty-second of it (outside the exponential's; the damping is then at most $\pi/32$, where the cubic is within $4 \times 10^{-6}$, and a wider pole would leave the unit circle). At 500 Hz, 50 Hz bandwidth, 16 kHz: $B$ = 127 298 (1.94241), $C$ = −64 262 (−0.98056), each within $10^{-5}$ of the real formula; an impulse rings at the formant (about a hundred zero crossings in 0.1 s) and decays.
- **`VocalSynth`**: from a frame, or refused (a negative amplitude; voiced without a fundamental or with a period longer than 65 535 samples; a resonator refused). `next_source` is a pulse at the start of each period (the period `fs / F0` in samples, the next one perturbed by jitter, the pulse's amplitude by shimmer, both from the seeded generator) plus aspiration noise; `next_sample` is the source through the three resonators; `render` fills a caller's slice. Saturating throughout; two renderers of the same frame produce the same samples.
- **`VocalFrame::shape(tone, formal, valence)`**: the fundamental moves by a quarter of the valence and by the tone's step (soften −1/16, suggest +1/16, reflect −1/8, topic shift +1/8, play +1/8), clamped to [50, 500] Hz; a negative valence adds breath (up to 64/256) and takes amplitude; soften and reflect quieten, a topic shift raises; play adds jitter and shimmer (16/256 each); a formal register halves both. The tone values are `cortex-linguistic`'s `PROSODY_*` markers restated. Any context may shape the voice; the only inputs are the marker, the register and the valence.
- **Appraisal** (`cortex-affect`): `appraise_incongruity(surprise, threat)`: the incongruity is benign when the threat is at most 0.25 and is then the surprise, else zero; `mirth_q16` moves toward it by $2^{-2}$ and at least one LSB, so it reaches zero exactly after a quiet while; `is_amused` at 0.25. The surprise is `cortex-predictive`'s error and the threat `cortex-salience`'s, joined by the runtime (R-15).
- **Reward** (`cortex-neuromod`): `reward(rpe)` adds to the dopamine signal, saturating; a quarter of the mirth is what R-15 sends; `decay_dopamine(shift)` returns it to rest exactly.
- **Play** (`cortex-linguistic`): `PROSODY_PLAYFUL` (5); `mark_play(mirth)` sets it and opens the particle slot; refused below the threshold, for a sealed frame, or in a formal register (`POLITENESS_FORMAL`). Humor is gated by the relationship, not by the context; tact (`apply_face`, ADR-0026) is applied after it and wins.
- **Rollout** (`cortex-imagination`): `wander_at(mirth)`.
- **Ethics.** Nothing here touches `cortex-ethics`. Every proposed motor or tool action, a vocal frame included, passes the veto gate before dispatch (§8.9), whatever marker the frame carries; "benign" is the salience threat's absence, and a proposal's harm is the gate's judgement, not the appraisal's.
- **Not added.** `benign_incongruity_score_q16` and `playful_irony_marker` in `cortex-agency`: the appraisal is affect's and the marker is the frame's. Parameters in `TorqueFrame`: a voice is not a joint.
- **Vocabulary.** The document says what the rules compute: a mirth average, a playful marker, a shaped voice. It does not say the engine is witty, has timing, or that anyone will laugh; that a listener finds the marked turn funny is hypothesis H-6 (§11.1), and that the shaped voice reads as the intended tone is part of it.

### Consequences

- Good: the renderer is a hundred and fifty lines of integer arithmetic with pinned coefficients and a ringing test; a whole epoch of samples is sixteen resonator steps per formant.
- Good: eleven new tests; no crate, no dependency, no `unsafe`, no float.
- Bad: three formants and an impulse source are a vowel-and-breath voice, not speech: consonants, the lexicon's phoneme sequence and the actuator's driver are Specified.
- Bad: the cosine series covers a quarter of the sample rate; a fourth formant at 16 kHz would need a longer series or a higher rate.
- Bad: the appraisal reads surprise and threat as numbers; the semantics of what was violated are the lexicon's, so the marker can fall on a turn nobody finds funny (H-6).

## Alternatives considered and why rejected

- **Option 1** puts acoustic parameters where torques live and the renderer where no test could hold it to the state crate's rules; and it puts an appraisal of the moment in a record that attributes actions.
- **A table-driven cosine** trades a 128-byte table for a series that the sample rate bounds anyway.
- **A "game mode" gate on play** is what the directive rejected and what the register replaces: the relationship gates humor, the context does not.

## Confirmation

Eleven tests: `cortex-embodiment` (the frame's layout and neutral voice; resonator coefficients against the real formulas and the refusals; an impulse rings and decays; a voiced source pulses every period and jitter and shimmer stay within bounds; an unvoiced frame is noise or silence and aspiration is bounded; rendering is deterministic per seed and refuses what it cannot render; `shape` moves the voice by tone, register and valence and clamps), `cortex-affect` (a surprise without threat is benign and raises mirth, a threat makes it nothing, mirth reaches zero; two bodies appraise alike), `cortex-neuromod` (reward saturates and decay reaches rest from both sides), `cortex-linguistic` (play needs mirth and a register below formal, and tact has the last word). `npx spec-guard` asserts `VocalFrame`, `fn next_sample`, `appraise_incongruity`, `mark_play` and `fn reward` exist.
