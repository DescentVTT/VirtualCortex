---
status: accepted
date: 2026-09-24
amends: ADR-0093
decision-makers: VirtualCortex maintainers
---

# ADR-0095: An image's pin moves with its format — ADR-0093's stopping rule amended before any run: the pinned numbers that stop the round when they move with the signed gate unset are the engine's and the records', not the header's version and the seal over it, which ADR-0093 itself orders moved; H-17's whole-image CRC re-pinned at format 16 and every other byte of that image held to the CRC H-17 read; nothing else of H-18 changes

## Context and Problem Statement

[ADR-0093](0093-the-punished-pair-consolidates-against-its-trace.md) orders two things that cannot both hold as written. Its rule *"moves the image format from 15 to 16"*; its decision drivers say *"Unset, every pinned number of the tree and the determinism pin stand, bit for bit"*, and step 2 of H-18's stopping rule says *"A calibration that does not reproduce, or a pinned number that moves with the signed gate unset, stops the round before any rewarded run; it is a finding, not an outcome."* Brief 041 repeats both. Brief 041 also gives the executing round the licence to amend H-18's stopping rule by an ADR committed before the first rewarded run when it finds a defect in it.

Building the mechanism ([ADR-0094](0094-the-signed-gate-built.md)), this round found the one pinned number of the tree that covers an image's whole bytes, header included: `REVERSAL_IMAGE_CRC_1024` in `runtime/cortex-runtime/tests/inhibition.rs`, the CRC-64 of the inhibited image both of H-17's arms decode, `0xd5579c31308388ad` ([ADR-0091](0091-the-assignment-reversed-measured.md)). The header's `[8..12)` is `FORMAT_VERSION` and its `[48..56)` a CRC-64 of the header, so moving the format moves two fields of every image and with them any pin over the whole bytes — whatever the signed gate is, and with nothing of the engine moved. Read literally, step 2 stops every round that moves the format, the one ADR-0093 orders included. No other pinned number of the tree covers a header: every other pin is a number a run produced (weights, sums, spikes, counts, readings and their hashes) or a section's bytes.

**What the image under format 16 is, read before this decision.** A probe built H-17's inhibited image as its arms build it — ADR-0077's settled engine, held to its tables step by step, its frozen image, the inhibitory baseline's flag and value written — from this round's tree with the signed gate unset (`crates/cortex-connectome` at format 16): its CRC-64 is `0xd2965219775c394a`, its header reads version 16, and it differs from the frozen image in the ten bytes H-17's test already allows (the section's CRC, the flag and the value's one byte that is not zero). CRC-64/XZ is affine in the message, so for two images of one length `crc(a) ⊕ crc(b)` is the register run from zero over `a ⊕ b`: with the format-16 header's 64 bytes from the probe, the header rewritten to version 15 and resealed, and 590 592 zero bytes after it, the format-16 image's CRC gives back **exactly `0xd5579c31308388ad`**, the CRC H-17 read (a script over the probe's header, the check value `crc64(b"123456789") = 0x995DC9BBDF1939FA` reproduced first). Every byte of the image but the header's version and seal is the image H-17 ran from.

## Decision Drivers

- H-18's stopping rule: a defect found in it before the first rewarded run is written as an ADR amending ADR-0093 and committed before that run; never after it.
- What step 2 protects: that H-18 differs from H-17 by one variable — that the engine with the signed gate unset is the engine H-17 ran, bit for bit, so that a difference in the runs is the gate's.
- Principle 6: a claim about the tree is made executable; the evidence that the image is H-17's but for its header belongs in the test that pins it, not only in this text.
- Principle 1: the tree and the document disagree here in the document's own words; the disagreement is resolved in the open, before any run, and not by reading the rule loosely after one.

## Considered Options

1. **Amend step 2's reading**: the numbers that stop the round are the engine's and the records' — every number a run produces and every byte of a section — and not the header's version and the seal over it; the whole-image pin is re-pinned at format 16, and the test holds the image with its header written back to 15 to the CRC H-17 read.
2. **Stop the round** at step 2 and record a finding.
3. **Re-pin silently**, as a consequence of the format the brief orders.
4. **Pin a CRC over the image without its header.**

## Decision Outcome

**Option 1.** H-18's stopping rule, step 2, as amended: *a calibration that does not reproduce, or a number of the engine that moves with the signed gate unset — any number a run produces, weights, traces, sums, spikes, counts, readings and their hashes, or any byte of a section of an image — stops the round before any rewarded run; it is a finding, not an outcome. A pin over an image's whole bytes moves with the format ADR-0093 orders, through the header's version and the seal over it, and is re-pinned in the round that moves the format, the rest of the image held to the pin before it.* ADR-0093's decision driver *"every pinned number of the tree and the determinism pin stand"* is read the same way.

In the tree, before any rewarded run:

- `REVERSAL_IMAGE_CRC_1024` is `0xd2965219775c394a`, the format-16 image's CRC-64.
- `REVERSAL_IMAGE_CRC_FORMAT_15_1024` is `0xd5579c31308388ad`, the CRC H-17 read, and each of H-17's arms asserts that its image with the header's version written back to 15 and the header resealed (`with_version`) has it — so the claim that nothing but the header moved is held in the weekly job, not only here.
- Every other pinned number of the tree stands with the signed gate unset: the debug suite passes, 634 tests and 51 ignored where main read 626 and 51 (the eight tests of ADR-0094), the determinism pin unmoved; the whole-domain tests, H-17's arms among them, run in the dispatch the measurement's ADR records.

**Nothing else of H-18 changes**: the network and the calibration, the constants, the two arms, 4 608 trials with the flip between the 1 536th and the 1 537th, the criterion's two clauses, the assertion, the readings, the absence of a verdict prediction, the three predicted readings and the other steps of the stopping rule are ADR-0093's as written.

### Consequences

- Good: step 2 says what it was written to protect — that the engine with the gate unset is H-17's engine — and a failure of it would be a finding about the engine rather than about a header field the same ADR moves.
- Good: the image H-17 ran from is still held bit for bit, in the test, through the CRC H-17 read.
- Neutral: one more constant and one helper in H-17's test; every future format change re-pins the one whole-image CRC the same way.
- Bad: ADR-0093's text stands with the sentence this decision reads narrowly; the whitepaper's H-18 names this decision beside it.

## Alternatives considered and why rejected

- **Stop the round** (option 2): the finding would be known before the round began to be the stopping rule's own, a finding of nothing, and it would block the mechanism ADR-0093 decided for a header field ADR-0093 decided to move.
- **Re-pin silently** (option 3): a pinned number would move without the rule that governs pinned numbers saying why, which is how a stopping rule stops meaning anything.
- **A CRC without the header** (option 4): changes what H-17 pinned rather than holding it; the header rewritten to 15 holds the same bytes H-17 read, and the format-16 pin holds the header as it is now.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `REVERSAL_IMAGE_CRC_1024`, `REVERSAL_IMAGE_CRC_FORMAT_15_1024`, `with_version`, and the assertion in `reversal_arm`, committed before H-18's constants.
- Whitepaper §11.1's H-18: its stopping rule's step 2 with this decision named; §9's row; the whitepaper's version moved in both declarations.
- The dispatched weekly job runs H-17's two arms with the new pin and the format-15 check; its run id is the measurement's evidence.
