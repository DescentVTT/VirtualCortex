---
status: accepted
date: 2026-10-19
depends-on: ADR-0195
decision-makers: VirtualCortex maintainers
---

# ADR-0196: The engine's default for an image no derivation has settled — H-41's yes taken at ADR-0192's step 3, the maintainers' choices: a named configuration, `Config::reads_its_targets()`, with `Config::default()` left the bare engine every pin runs; in it what H-41 measured of the network's own regime and no more — the modulation at rest of every rewarded run, the gain 1.75 the calibration took on every network derived, the rate's rule at a sixteenth, the read of 128 windows and both targets by it; the gain an engine starts with a field of the configuration, the image's outranking it; no hypothesis, the default shown byte for byte H-41's candidate and every pin held by one dispatch; F-71; the step by the engine's own loop the next decision, then M10; brief 076 builds it

## Context and Problem Statement

[ADR-0195](0195-the-inhibitory-target-by-the-same-read-measured.md) read H-41 as yes: with the engine reading both of its network's targets, the configuration learns and revises on seeds 22, 23 and 24 at 1 024 units. [ADR-0192](0192-the-inhibitory-target-by-the-same-read.md)'s stopping rule, step 3, names the next decision: *"an ADR on the engine's default for an image no derivation has settled: the rate's rule at a sixteenth, the read of 128 windows, both targets by it. Then M10, a reward past the trial."* **The maintainers took it on 2026-10-19.**

**What "default" means in the tree today** (`runtime/cortex-runtime/src/executor.rs`, `runtime/cortex-runtime/src/image.rs`):
- `Config::default()` leaves every rule unset: the step 0, no rate target, no read, the read's flag clear, the inhibitory period 20 000 ticks, the modulation baseline 1.0. It is used 203 times in 22 files. Every use under `src/` is a test's; the harness's `config()` builds on it for five test binaries; T-1 spells every field out, so that a change to the default does not move its pin.
- **An engine built from an image takes its read, its flag, its step, its gain and both periods from the image's bytes**, and the configuration's values for them are outranked. A default therefore acts only where an image is born: `Executor::new`, the arenas synthesized from a prior, `Image::encode`.
- **The gain has no field in the configuration.** `Executor::new` starts it at 1.0 (`HomeostaticDrivePool::new`), and only an image sets it; the harness patches it into the image's record (`at_gain`).

**What every derivation does before the read** (`runtime/cortex-runtime/tests/inhibition.rs`, `derive_under`):
1. The rotation, by the prior's census.
2. The gain's calibration over the task's stimuli: 1.75, then 2.0, the first whose block is seen in at least 56 of its 64 trials with each readout rising. **On every network derived, 1.75 was calibrated**: seed 22 at 1 024 units, seeds 23 and 24 ([ADR-0167](0167-another-network-of-the-same-size-measured.md)), and seeds 22 and 23 at 2 048 units ([ADR-0169](0169-twice-the-size-measured.md)).
3. The candidate at that gain, its lead-in at the modulation at rest of every rewarded run since brief 027, 0.5 (`BASELINE_Q16`), and the read: the gain held for 128 windows, both targets from the mean of the last sixteen, then the rate's rule at a sixteenth.

So an image born at the defaults would read at a gain of 1.0, which no run has read at. A network whose mean count lies outside the read's range is refused, and its engine cannot be written as an image.

**What H-41 measured of the network's own regime** is therefore the modulation at 0.5, the gain at 1.75, the step at a sixteenth, the read of 128 windows and its flag set, on three networks at 1 024 units.

**F-71, found in writing this.** Whitepaper §8.8's row for synaptic scaling per unit, and §11.1's open item on it, give its bytes as `[52..54)` of the unit record. Since [ADR-0131](0131-the-critic-built.md) (format 19) those bytes are `value_weight`, as §5.2's table and `crates/cortex-core/src/dynamics/neuron.rs` say.

## Decision Drivers

- **The maintainers' choices** (2026-10-19): a named configuration with `Config::default()` untouched; the gain 1.75 in it; no hypothesis, the default built and pinned.
- **The default is what was measured, and no more**: each value with the round that read it.
- **Unset is the run it was**, bit for bit: no pinned number of the tree moves.
- **Latest ≠ Newest**: the engine's own rules, gathered under one name; nothing outside the tree is depended on.
- **The maintainers' order** (2026-10-19): the default; then the rate's rule's step made from the engine's own measured loop; a re-read of the targets by evidence is M12's, once a long run measures their drift.

## Considered Options

1. **Where the default lives**: (a) a named configuration, `Config::reads_its_targets()`, `Config::default()` untouched; (b) `Config::default()` itself.
2. **The gain**: (a) 1.75 in the default, through a new field of the configuration; (b) not in it, a born image at 1.0.
3. **What shows it**: (a) no hypothesis: the default built and shown byte for byte H-41's candidate, every pin held by one dispatch; (b) H-42, the default on a network not yet run, predicted yes.

## Decision Outcome

**Options 1(a), 2(a) and 3(a).**

### The gain an engine starts with (Specified; brief 076 builds it)

- `Config::synaptic_gain_q16`, a `u32` in Q16.16, default `GAIN_ONE_Q16`: the gain `Executor::new` puts in the homeostasis record. `Config::default()` builds the engine it built.
- Refused outside `GAIN_MIN_Q16` to `GAIN_MAX_Q16`, 0.25 to 4, the controller's own bounds.
- For an engine built from an image, the image's gain outranks it, as it outranks the step.
- No record and no section changes. The image stays at format 24.

### The default (Specified; brief 076 builds it)

`Config::reads_its_targets()` is `Config::default()` with five fields set:

| Field | Value | Read by |
| :--- | :--- | :--- |
| `modulation_baseline_q16` | `0x8000`, 0.5 | every rewarded run's lead-in since brief 027 |
| `synaptic_gain_q16` | `0x0001_C000`, 1.75 | the calibration's first candidate, calibrated on all five networks derived |
| `control_step_q0_16` | `0x1000`, a sixteenth | H-39, H-40 and H-41 ([ADR-0183](0183-a-sixteenth-on-three-networks-measured.md), [ADR-0190](0190-a-target-the-engine-reads-measured.md), ADR-0195) |
| `rate_read_windows` | 128 | H-40 and H-41 |
| `istdp_target_read` | set | H-41 |

- **Every other field is the default's.**
  - The sizing (workers, units, blocks, nodes, the injector, the trace and the train) is the caller's, by struct update.
  - The rate's target stays 0, since the read makes it. The inhibitory period stays 20 000 until the read's end.
  - The learning configuration stays unset in it: the inhibitory baseline, the signed gate, the critic and its window, the class, the slow current and the whole punishment. The arms lay it in their images, as they do.
- **It is the default for an image no derivation has settled.** An engine built by `Executor::new` from it, its arenas synthesized, holds its gain at 1.75 for 128 windows. It then sets both targets from the mean of the last sixteen and runs the rate's rule at a sixteenth from the 129th window. `Config::default()` stays the bare engine, every rule unset, that every pin runs.
- **It is what H-41 read and no more**: three networks at 1 024 units. At 2 048 units the gain was calibrated at 1.75 (H-34) and the read was never run. No other generator, drive or size has been read under it.
- **A network whose mean count the read refuses is refused**, as the read refuses it now; the default does not repair it.

### What shows it (Specified; no hypothesis)

- **H-41's candidate, byte for byte.** On each of seeds 22, 23 and 24 at 1 024 units, an engine built from `reads_its_targets()` with H-41's sizing, its arenas synthesized from the network's prior, writes the image H-41's candidate writes before its lead-in.
- **Every pin held.** With the gain's field at its default, every pinned number of the tree holds. It is shown by one dispatch at `scope=both`, whose eighteen whole-domain shards are green.
- **There is nothing to predict.** The default is H-41's settings by construction, and nothing is run under it that H-41 did not run.

### F-71

**§8.8 and §11.1 place synaptic scaling per unit at `[52..54)` of the unit**, where `value_weight` has stood since ADR-0131. It is recorded in §11 and resolved by this record:
- §8.8's row and §11.1's item now say that the per-unit gain has no bytes yet, and that its layout is decided by its own ADR first, as Appendix C's M12 says of the bytes the sliding threshold asked for.
- No record and no rule change.

### The next decisions, named and not taken

1. **The rate's rule's step from the engine's own loop** (the maintainers' order, 2026-10-19).
   - Under an eighth the rate moved fifteen per cent for one per cent of gain (ADR-0181).
   - A sixteenth halved the count's alternation and did not remove it (ADR-0183).
   - The decision is an ADR on a step the engine makes from the steepness it measures, with a hypothesis that it removes the alternation. If it reads yes, that step takes the sixteenth's place in `reads_its_targets()`.
2. **Then M10**, a reward past the trial.
3. **A re-read of the targets by evidence is M12's**, once a long run measures their drift.

## Consequences

- Good: the engine has a default that reads its network's targets for itself, and a host has one configuration to start an image from.
- Good: every pin stays, and `Config::default()` is the bare engine it was.
- Good: each value of the default names the round that read it.
- Bad: two configurations to keep in step. A rule that joins the learning regime must say whether it joins the default.
- Bad: the gain is a designer's constant. The task's calibration picked 1.75, and a network on which it is not calibrated has a default that was not measured.
- Bad: the task's own steps (the rotation, the gain's calibration, the cancel and the critic's constants) are still derivations; the default does not touch them.
- Neutral: one field more in the configuration, no record, no format.

## Alternatives considered and why rejected

- **Option 1(b), `Config::default()` itself**:
  - 203 uses in 22 files, and every derivation and arm under the harness's `config()`, would move.
  - The read would be refused beside every test that sets a target (`RateReadBesideTarget`).
  - It is a re-pin of the tree for nothing measured.
- **Option 2(b), no gain**: a born image would read at 1.0, which no run has read at, and a quiet network may be refused.
- **Option 3(b), H-42**: the default is H-41's by construction. A network not yet run is a question of its own, a generalisation the maintainers did not take now.

## Confirmation

`briefs/076_the-engines-default.md` builds the gain's field and the default, shows the default byte for byte H-41's candidate, and holds every pin by one dispatch. Whitepaper 4.132.0 carries:
- §9's row;
- F-71 in §11;
- §8.8's per-unit row and §11.1's item corrected;
- §8.8's rate row naming the default as Specified;
- Appendix C's M10 after the step this record names.

`npm run spec` holds the links, the rows, the brief's contract and scope, and the version.
