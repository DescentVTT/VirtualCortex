---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
---

# ADR-0010: Every performance figure is Measured or Target, never asserted

## Context and Problem Statement

Earlier revisions of the whitepaper stated figures such as "P99.99 tail latency under 35 nanoseconds", "more than 120 million spikes per second" and "cold boot of 86 billion nodes in under 100 milliseconds" as achievements, and the README said they were "tested on" a reference server. No benchmark existed in the repository. How are performance figures allowed into the documentation?

## Decision Drivers

- A number a reader cannot reproduce is marketing, and marketing in a specification poisons every number next to it.
- Some figures were physically impossible as stated (an 11 GB image cannot be read from NVMe in 100 ms).
- Targets are valuable: they tell implementers what the design must reach and how it will be checked.

## Considered Options

1. Remove all performance figures.
2. Keep figures with a general disclaimer.
3. **Two labels only. `Measured`: produced by a benchmark committed to this repository, run on the reference platform of whitepaper §7.1, reproducible from a committed command, with the commit hash recorded. `Target`: a goal with a stated measurement protocol and preconditions. A figure with neither label is a defect.**

## Decision Outcome

Option 3. Whitepaper §10.2 is the single table of figures; every row carries a Target and an empty Measured column until a benchmark exists. The withdrawn figures are named in that section so that their absence is visible rather than silent.

### Consequences

- Good: implementers have concrete goals; readers have honest ones.
- Good: the first benchmark (T-3, spike enqueue latency) is now a scheduled milestone rather than an implied fact.
- Bad: the project currently has no measured figures at all, which the document states in its first page.

## Confirmation

Reviewers reject any pull request that adds a number to §10.2's Measured column without a linked benchmark and command. A future `spec-guard` directive will assert the presence of `benches/` once the first benchmark lands.
