---
status: proposed
date: 2026-11-01
depends-on: ADR-0230
title: The couplings held in a running configuration, and M12 closed
kind: decision
summary: '`Config::holds_its_couplings()` is what an engine runs a settled image under: the default with a quarter''s exponent, the host setting the zero sum''s groups. The derivation stays H-42''s and an exponent is refused beside a read. No hypothesis is asked. M12 closes, and M11 opens with an exploration.'
decision-makers: Menteion maintainers
---

# ADR-0232: The couplings held in a running configuration, and M12 closed

**In brief**
- **Decision.** `Config::holds_its_couplings()` is `Config::reads_its_targets()` with `depression_exponent` at a quarter, and an engine runs a settled image under it, the host setting the zero sum's groups at each load. `Config::reads_its_targets()` does not change, and an exponent other than one is refused beside a read. M12 closes.
- **Why.** H-46's arms ran the two rules after H-42's derivation, never through it, so an image born at a quarter is a configuration no run has read. This puts in the tree what was measured, and no more. M12's exit test has read yes.
- **Left open.** Everything past sixteen schedules, the couplings' shape, and the inhibitory sum as anything but a reading. M11 opens with an exploration, and its decision ADR settles F-47's replay.

## Context and Problem Statement

[ADR-0230](0230-a-zero-sum-at-the-stimulus-measured.md) read H-46 as yes at step 3 of its stopping rule.
That step names the next decision: whether the two rules enter the engine's default, whether M12 closes, and then M11.
The maintainers took all three on 2026-11-01.

**What the default is** ([ADR-0196](0196-the-engines-default.md), `runtime/menteion-runtime/src/executor.rs`).
`Config::reads_its_targets()` is the engine's default for an image no derivation has settled.
ADR-0196 reads that a default acts only where an image is born, since a decoded engine takes its read, its flag, its step, its gain and both periods from the image's bytes.
ADR-0205's byte-for-byte route rested on that: the default was H-42's candidate, so nothing new had to be read.

**What H-46's arms ran** (`zero_from` and `zero_schedule` in `runtime/menteion-runtime/tests/inhibition.rs`):

| Stage | Exponent | Zero sum | Read by |
| :--- | :--- | :--- | :--- |
| The derivation: the lead-in, the read and the probe | 1 | unset | H-42 |
| Each schedule after it, on a decoded engine | 1/4 | set by the host at the load | H-46 |

- Neither rule is in the image. A decoded engine's exponent is its configuration's, and `Image::decode` outranks the configuration's read, flag, probe, baseline, gain and step.
- `Executor::set_zero_sum` takes the groups and their values from the host. Each schedule's engine set them again, every fraction at a half.
- Each group's value was its sum in H-42's image, the same value at every schedule. The harness reads it in `zero_groups`.

So `Config::reads_its_targets()` cannot take the exponent as ADR-0205 took the probe.
Set there, the derivation runs at a quarter: `depression_magnitude` shapes every excitatory depression.
No run has read that derivation, and ADR-0196's driver, *"the default is what was measured, and no more"*, would not hold.
And no configuration can name the zero sum's groups, since they are the body's stimulus sets.

**M12** ([Appendix C](../WHITEPAPER.md#appendix-c-roadmap)) asks for the couplings and the inhibitory sum bounded over many schedules, and its exit test is a hypothesis written before any run reading yes.

| Asked of M12 | What read it | Result |
| :--- | :--- | :--- |
| The couplings bounded | H-46's clause 1 | 1.157 to 1.215 over sixteen schedules, on three networks |
| The inhibitory sum bounded | H-46's reading 4, not a clause | 0.486 to 0.522 of H-42's image's, moving at most 0.010 over schedules 9 to 16 |

[Exploration 013](../explorations/013-the-carried-sum-and-the-exponent.md) read the carried zero sum at an exponent of one out to 32 schedules, where it had not stopped, and a quarter's exponent to 16.

**M11** is next in ADR-0187's plan, and asks whether a learned mapping survives a sleep with replay better than one without.
No run of the task has slept: `runtime/menteion-runtime/src/task.rs` and `tests/inhibition.rs` tag no episode and read no sleep stage, and the harness's `config` sets `sleep_shift` to 0.
[F-47](../WHITEPAPER.md#11-risks-and-technical-debt) leaves a replay's arrival at the gain unread: its two messages of 1.25 reach a unit as 4.375 at a gain of 1.75.

## Decision Drivers

- **The maintainers' choices** (2026-11-01): a named configuration for a settled image, M12 closed now, and M11 explored first.
- **The default is what was measured, and no more** (ADR-0196).
- **Unset is the run it was**: `Config::default()` and `Config::reads_its_targets()` do not change, and no pin moves.
- **A structural boundary beats a reviewed one** (`CLAUDE.md` principle 5): nothing but a test should stand between a host and a derivation no run has read.
- **Latest ≠ Newest**: the engine's own rules under one more name, with nothing outside the tree depended on.

## Considered Options

1. **Where the rules go**:
   - (a) a second named configuration, for a settled image, with the groups the host's;
   - (b) nowhere, each round setting them, as the kept addresses and the delay read stand;
   - (c) the exponent in `Config::reads_its_targets()` and the groups in the image, format 27, under a new hypothesis on three networks.
2. **The held values**: (a) read once from the image the derivation wrote, and kept by the host; (b) read again from each image loaded.
3. **M12**: (a) closed now, the unread named; (b) closed after an exploration of schedules 17 to 32 at a quarter; (c) left open for a hypothesis over 32 schedules.
4. **M11's first step**: (a) an exploration, then the decision ADR; (b) the decision ADR at once.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).**

### The running configuration (Specified; brief 086 builds it)

`Config::holds_its_couplings()` is `Config::reads_its_targets()` with `depression_exponent: DepressionExponent::Quarter`; the name is brief 086's to change.
- An engine runs a settled image under it. `Image::decode` under it gives H-46's arm's engine, since the image outranks every other field it sets.
- `Config::reads_its_targets()` and `Config::default()` are unchanged, so every image is born as it was.
- No record, section or format changes, and the format stays 26.

### The boundary (Specified; brief 086 builds it)

- **An exponent other than one is refused beside a read.** `Executor::new` refuses it with a `ConfigError` of its own, so no engine is born under `Config::holds_its_couplings()`.
- **An image with a read or a probe under way is refused under such an exponent**, with the same error, so no derivation finishes at one.
- H-46's images are past their read and their probe, so no run of the tree meets either refusal.

### The host's part (Specified; brief 086 builds it)

- `Executor::coupling_sum(sources, targets)` reads, between ticks, the sum of the synapses from `sources` onto `targets`: what `zero_groups` reads, and what a host otherwise walks the chains for.
- The host reads each group's value once, from the image the derivation wrote, and keeps it.
- At each load it calls `Executor::set_zero_sum` with those values, every fraction at a half, as H-46's arms did.
- The groups stay the body's interface. A reloaded image needs them set again ([ADR-0226](0226-a-zero-sum-at-the-stimulus.md)).

### What shows it (Specified; no hypothesis)

- **H-46's arm under the name.** `zero_from` takes an arm's exponent from `Config::holds_its_couplings()`, and the six `…_exhaustive` short forms hold each arm's first row.
- **The value by the reader.** `zero_groups` reads through `coupling_sum`, and its assertion holds each value to the couplings H-42's derivation pinned on three networks.
- **The configuration, field by field**, and both refusals by tests over the lattice of `testkit/prop.rs`.
- **Every other pin held**, by one dispatch at `scope=exhaustive` with nothing excluded.
- The long runs are not run again. They read the same fields, and brief 086 adds no rule for them to read.

### M12 closes

Its exit test read yes, and its row in Appendix C is done.

**Named and not read:**
1. The inhibitory sum rests on a reading, which decides nothing. It stood at half its image's.
2. The couplings' shape has not settled. The bottom eighth's share rose about 0.02 a schedule to the sixteenth, to 0.46 to 0.52, and the rail's count rose 24 to 50 from schedule 8 to 16.
3. No run under both rules reads past sixteen schedules.
4. The zero sum holds a group's sum, not a coupling. One coupling can rise as its pair falls, so 1.157 to 1.215 is a reading of the run and not a bound by construction.

**A coupling past 1.30 in a later run** under `Config::holds_its_couplings()`, with its groups set, is a finding, and it reopens M12.

**Named and not taken:** the sliding threshold (UPG-02), the sign rule on the rate's step, and the re-read of the targets by evidence that ADR-0205 named.

### M11 is next

- An exploration reads it first: a night with replay against one without, on H-46's arm, beside a twin with the replay's drive divided by the gain.
- It writes no prediction and decides nothing.
- M11's decision ADR then settles F-47's replay before its hypothesis.

### The sweep

No sweep is left unrecorded: [ADR-0227](0227-the-pinned-tables-in-a-file-a-round.md) recorded the schedule's run of 2026-10-28.
The schedule's next runs on 2026-11-04 at 20:17Z, and the first ADR that merges after it ends records it.

## Consequences

- **Good**: a host has one name for what H-46 measured, and the tree holds no derivation it did not read.
- **Good**: M11 starts on bounded couplings, and each configuration names the round that read it.
- **Bad**: the default is now two names, one for birth and one for a settled image, and a host must use both.
- **Bad**: the host keeps two values across loads, since no image holds them.
- **Bad**: M12 closes with its shape unread. A later run that crosses 1.30 costs a reopening.
- **Neutral**: `Config::default()` stays the bare engine every pin runs, and no format moves.
- **Neutral**: `Config::holds_its_couplings()` has run only on H-42's three networks at 1 024 units.

## Alternatives considered and why rejected

- **Option 1(b), nowhere**: every later round would restate the two rules, and an engine set up by the name alone would still pass 1.30 by the third schedule.
- **Option 1(c), the derivation at a quarter**: about 48 jobs of 100 minutes and a format bump for a configuration with no reading behind it. ADR-0226 already rejected the groups in the image.
- **Option 2(b), each image's sum**: H-46 ran one value at every schedule, and each image's own sum would follow its rounding.
- **Option 3(b), an exploration first**: a note decides nothing, so the exit test would read as it does now, a day later.
- **Option 3(c), a hypothesis over 32 schedules**: at least 36 jobs of 100 minutes, and M11 waits on it.
- **Option 4(b), the decision ADR at once**: no run of the task has slept, so its prediction would have no baseline.

## Confirmation

`briefs/086_the-couplings-held.md` builds the configuration, the boundary and the reader, and holds every other pin by one dispatch.
`npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
