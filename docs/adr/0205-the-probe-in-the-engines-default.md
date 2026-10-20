---
status: proposed
date: 2026-10-21
title: The probe in the engine's default
kind: decision
summary: H-42 read yes, so Config::reads_its_targets() sets the probe and the engine's own step replaces step = 1/16 in the default. Brief 079 shows it byte for byte H-42's candidate, with no hypothesis. F-76, the gate's release step past its bound, is recorded and that bound raised.
depends-on: ADR-0204
decision-makers: VirtualCortex maintainers
---

# ADR-0205: The probe in the engine's default

**In brief**
- **Decision.** `Config::reads_its_targets()` sets `step_probe`. An engine born under the default reads its targets, probes its own loop, and runs the rate's rule at the step it made.
- **Why.** H-42 read yes with all four of its predicted readings, and ADR-0200's stopping rule names this as step 3's decision.
- **Left open.** The default on a network, a size or a generator H-42 did not run. M10 is next. The test crates' compile time, which F-76 records, is named and not taken.

## Context and Problem Statement

[ADR-0204](0204-a-step-the-engine-makes-from-its-own-loop-measured.md) read H-42 as yes on seeds 22, 23 and 24 at 1 024 units:

| Reading | Predicted | Read |
| :--- | :--- | :--- |
| S | 12 to 24 | 15.02 to 15.19 |
| step | about 1/60 | 1 078 to 1 091 in Q0.16, 1/61 to 1/60 |
| opposite pairs of 959 | fewer than 540 | 485 to 527, where step = 1/16 read 591 to 623 |
| the count against its target | within 1 % | 0.13 to 0.15 % above |
| the gain | within 5 % of 1.75 | 1.7032 to 1.7408 |

Both arms on each network held H-25's four clauses.

[ADR-0200](0200-a-step-the-engine-makes-from-its-own-loop.md)'s stopping rule, step 3: *"an ADR that puts the probe in `Config::reads_its_targets()` in step = 1/16's place. Then M10."*

**What the default is now** ([ADR-0196](0196-the-engines-default.md), built by [ADR-0198](0198-the-engines-default-built.md)):
- `Config::default()` with the modulation at rest 0.5, the gain 1.75, `control_step_q0_16` 0x1000, a read of 128 windows and the read's flag for the inhibitory target;
- a test that an engine born under it writes H-41's candidate's image, byte for byte, on H-41's three networks.

**What H-42 ran** ([ADR-0202](0202-a-step-the-engine-makes-from-its-own-loop-built.md)): `UNDER_THE_PROBE`, that default with `Config::step_probe` set.
- While the read and the probe run, the gain is held, so the configured step acts on no window before the probe writes its own.
- A refused probe ends the read refused, and its engine has no image.

## Decision Drivers

- **H-42's yes**, and ADR-0200's step 3.
- **The default is what was measured, and no more** (ADR-0196).
- **Unset is the run it was**: `Config::default()` does not change.
- **The order the maintainers set** (2026-10-19): this, then M10.

## Considered Options

1. **The configured step in the default**:
   - (a) left at 0x1000, a value no window of a default run reads;
   - (b) set to zero.
2. **What shows it**: (a) no hypothesis, the default byte for byte H-42's candidate and every other pin held by one dispatch; (b) H-43 on a network not yet run.

## Decision Outcome

**Options 1(a) and 2(a).**

### The default (Specified; brief 079 builds it)

`Config::reads_its_targets()` gains `step_probe: true`. Its other five values stay.
- The step stays 0x1000. With the probe set, no window reads it, and H-42's runs carried it.
- `Config::default()` and every other configuration of the tree are unchanged.

### What shows it (Specified; no hypothesis)

- **H-42's candidate, byte for byte.** On each of seeds 22, 23 and 24 at 1 024 units, an engine born under the default with H-42's sizing writes the image H-42's candidate writes before its lead-in.
- **The default's two tests re-pinned, by design.** ADR-0198's field-by-field test gains `step_probe`. Its byte-for-byte test moves from H-41's candidate to H-42's.
- **Every other pin held**, shown by one dispatch at `scope=both`, whose eighteen whole-domain shards are green.

### F-76: the gate's release step past its bound

Brief 078's pull request failed its gate once: the step *"Test in the release profile"* hit its 10 minutes on `9ac4475`, a commit that changed only documents.
A rerun passed in 448 s.
[ADR-0159](0159-the-gates-jobs-bounded.md) bounds each test step at three times the longest it took in twelve green runs, and its comment still read *"192 s at the longest"*.

| Step | Bound | Longest in the last twelve green runs |
| :--- | ---: | ---: |
| Test, the debug profile | 35 min | 1 188 s |
| Test in the release profile | 10 min | 516 s |

- **Recorded and resolved in part.** By ADR-0159's own rule, the release step's bound becomes 26 minutes, three times 516 s, in this pull request. Its comment names the figure.
- **The debug step** stands at 57 per cent of its bound, and is left.
- **Open.** The steps grow because the test crates grow: `runtime/cortex-runtime/tests/inhibition.rs` holds every round's pinned tables, in hundreds of thousands of lines. A decision on where pinned tables live is named here and not taken.

### The next decisions, named and not taken

1. **M10, a reward past the trial** ([Appendix C](../WHITEPAPER.md#appendix-c-roadmap)).
2. **A re-read of the targets by evidence**, at M12.
3. **Where the rounds' pinned tables live**, so that the gate's compile time stops growing with each round (F-76).

## Consequences

- Good: a network the engine is given reads its targets and makes its step, with no constant of its own in the rate's rule.
- Good: each value of the default names the round that read it.
- Bad: the default's lead-in is 16 windows longer than H-41's, and a network the probe refuses has no image.
- Neutral: one field of the default, no record and no format.
- Neutral: the release step may run 26 minutes before it is cut, where it was cut at 10.

## Alternatives considered and why rejected

- **Option 1(b), the step at zero**: no run has carried a zero step beside the probe, so the default would not be H-42's candidate.
- **Option 2(b), H-43**: the default is H-42's settings by construction. A network not yet run is a question of its own.

## Confirmation

`briefs/079_the-probe-in-the-engines-default.md` builds it and holds every other pin by one dispatch.
`npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
