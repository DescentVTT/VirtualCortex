---
status: proposed
date: 2026-10-28
title: A zero sum at the stimulus, and a gentler depression
kind: decision
hypothesis: H-46
summary: Two rules, unset by default. Each stimulus set's couplings to the readouts are held at their sum, scaled back after the consolidations that move them; the depression's exponent may be set to a quarter. H-46 asks both on three networks over sixteen schedules, beside the default, predicted yes.
depends-on: ADR-0055
decision-makers: Menteion maintainers
---

# ADR-0226: A zero sum at the stimulus, and a gentler depression

**In brief**
- **Decision.** Two rules, each unset by default. A **zero sum at the stimulus** holds each stimulus set's couplings to the readouts at a sum the host gives, scaling them back after the consolidations that move them, with each synapse's dropped fraction carried. The excitatory **depression's exponent** may be set to a quarter, where [ADR-0055](0055-a-weight-that-settles.md)'s is one. H-46 asks both together on H-42's three networks over sixteen schedules, beside the default, predicted yes.
- **Why.** M12's exit test asks for the couplings and the inhibitory sum bounded over many schedules; H-46 tests the couplings and reads the inhibitory sum. Under the default the couplings rose through every schedule read. Of the forms tried, the zero sum alone held them with every mapping learned, and a quarter's exponent gave back the default's speed.
- **Left open.** Runs past sixteen schedules; the default's adoption; F-86, until H-46's measured ADR.

## Context and Problem Statement

[Appendix C](../WHITEPAPER.md#appendix-c-roadmap)'s M12 asks for the couplings and the inhibitory sum bounded over many schedules. Explorations 004, 005, 007 and 009 to 013 read it at 1 024 units, from H-42's image, in H-25's two arms; 004 on seeds 22, 23 and 24, the rest on seed 22. They are exploratory, not evidence. Every coupling below is against H-42's image on its network:
- **Under the default** ([004](../explorations/004-long-runs-under-the-default.md), [005](../explorations/005-eight-schedules-under-the-default.md), [007](../explorations/007-sixteen-schedules-and-a-soft-bound.md)) H-25's clause 2 failed from the second or third schedule in all six arms on the three networks. On seed 22 the highest coupling rose about 0.05 a schedule, to 1.66 by the eighth and 2.07 by the sixteenth. Every mapping was learned. The inhibitory sum stood near 0.55 from the fourth schedule and rose to 0.59 by the sixteenth.
- **Bounds on one synapse at a time** ([007](../explorations/007-sixteen-schedules-and-a-soft-bound.md), [009](../explorations/009-a-bound-at-consolidation.md)), four forms: each lost learning or held nothing. The couplings' rise was the rewards' surplus over the punishments'.
- **A readout's sum held** ([010](../explorations/010-a-readouts-sum-held.md)) held the sums and not the couplings, which took a larger share of each readout's input.
- **A zero sum at the stimulus** ([011](../explorations/011-zero-sum-at-the-stimulus.md)): each stimulus set's couplings to the readouts held at their sum in H-42's image, at every schedule. Both forms held the highest coupling at 1.13 to 1.20 over sixteen schedules, with every mapping learned and the inhibitory sum at 0.51 to 0.54. Rounded down, the couplings split into a few strong synapses and many near zero.
- **The rounding** ([012](../explorations/012-the-zero-sums-rounding.md)): the split was the rounding's; rounded down moved 113 to 170 rails a schedule from the small synapses to the large. Rounded to the nearest, or with each synapse's fraction carried, the bound and the learning held and nothing split, but learning was slower than the default's.
- **The exponent** ([013](../explorations/013-the-carried-sum-and-the-exponent.md)): under the carried zero sum, a depression scaled by $(M / M_{ref})^{1/4}$ split the couplings by the rule itself. Its reversals took the default's blocks schedule by schedule, the highest coupling 1.13 to 1.19, the inhibitory sum 0.51 to 0.52, the excitatory arena within 0.9994 to 1.0005 of the image's. A reading, not measured: a schedule's reversals tracked how far its couplings had split.

**What the engine holds today**: ADR-0055's depression, *"`amount × magnitude / 2^13`"*, in `menteion-core`; the consolidations of `SynapseBlock::consolidate` and `consolidate_signed`; the task's readout sets, which the host gives. No group's sum is held.

## Decision Drivers

- **The maintainers' choices** (2026-10-28): explore before deciding; the carried zero sum and a quarter's exponent; the groups named by the host; the long runs paused with a short form in the weekly job; controls on all three networks.
- **M12's exit test** is a hypothesis written before any run reading yes.
- **Latest ≠ Newest**: integer rules of the engine's own; the exponent is the $\mu$ of Gütig, Aharonov, Rotter and Sompolinsky 2003, which ADR-0055 cited, read as a prior; no dependency.
- **Principle 5**: each rule's equality with what it replaces, unset, is held by property tests over the lattice.
- **The weekly job**: its cost table sums 95 604 s, and an eight-schedule job took 5 517 to 6 081 s on AArch64 in exploration 013.

## Considered Options

1. **The zero sum's rounding**: (a) each synapse's fraction carried; (b) to the nearest; (c) down.
2. **The exponent**: (a) a quarter; (b) a half; (c) one, ADR-0055's.
3. **The groups**: (a) named by the host; (b) a section of the image.
4. **The long runs after the round**: (a) paused, a short form of each arm in the weekly job; (b) all weekly; (c) paused only.
5. **The controls**: (a) the default on all three networks; (b) on seeds 23 and 24.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

### The zero sum (Specified; brief 085 builds it)

- **Set by the host** (`Executor::set_zero_sum`; the name is brief 085's to change): each group is a stimulus set's excitatory units, its targets the readout units, its held value the host's. H-46 gives each group its sum in H-42's image on that network, the same value at every schedule.
- **After a tick's consolidations**, each group whose sum moved is scaled back to its held value: each of its synapses' weights times the held value over the sum, plus the fraction that synapse's last scaling dropped, rounded down, no weight past the rail.
- **The carried fractions** start at a half and are set to a half again whenever the groups are set.
- **Unset, every run is the run it was**, bit for bit. Neither the groups nor the fractions are in the image, and the format stays 26.

### The exponent (Specified; brief 085 builds it)

- **A configuration's exponent** (`Config`; the name is brief 085's): one, ADR-0055's and the default, a half, or a quarter. The excitatory depression scales with $(M / M_{ref})^{\mu}$, the fractional powers by integer square roots.
- **At one it is ADR-0055's rule** bit for bit. It applies to every excitatory synapse's depression. It is not in the image.

### The runs (Specified; brief 085 builds them in the tests)

On each of seeds 22, 23 and 24 at 1 024 units, from the image an engine born under `Config::reads_its_targets()` settles (H-42's derivations, pinned), in H-25's two arms on H-20's schedule, sixteen schedules back to back:
- **the arms**: the carried zero sum and a quarter's exponent; six;
- **the controls**: the default, both rules unset; six;
- **each in two jobs of eight schedules**, the eighth image carried with its CRC-64, as exploration 007 carried it. The twelve images, about 7 MB, enter the tree.

**The second halves** run on the commit that adds the twelve eighth images and nothing else that runs, after the calibration is green and the first halves are read: a departure from [briefs/README.md](../../briefs/README.md#evidencing-a-round)'s arms on the protocol's commit, which no run of sixteen schedules within a job's bound can avoid.

**The tests' names.** The twelve long runs are `…_paused` from the protocol's commit ([ADR-0127](0127-the-paused-line-leaves-the-weekly.md)), so the weekly job never lists them and the arms job runs them by name. A short form of each arm, its first schedule, is a weekly `…_exhaustive` test. A control's first schedule is H-42's arm, already weekly, and gains no short form.

**The reproduction.** The short forms are reproduced at `scope=exhaustive` on x86-64. The long runs, which that scope cannot list, are dispatched again at `scope=arms` on the reproduction's commit, every pinned table held: on AArch64 again, so their schedules 2 to 16 run on one architecture only, and the long runs run twice where a round's arms run once.

### H-46 (Hypothesis; written before any run)

The text, with its clauses, readings and stopping rule, is [H-46's report](../research/H-46.md).
- **Clause 1**: in each of the six arms, H-25's four clauses hold in every one of the sixteen schedules, each of a schedule's three reversals among them, every coupling read against H-42's image on its network.
- **Clause 2**: in each of the six controls, some coupling passes 1.30 of H-42's image's before the sixteenth schedule ends.
- **Yes** when both hold; otherwise **no**, with the network, the arm, the schedule and the clause.

**Predicted: yes.** The zero sum held the bound on seed 22 in every form read, and the default passed it by the third schedule on all three networks.

**Four readings are predicted:**
1. each arm's reversals over schedules 9 to 16, summed, at most 1.25 times its control's;
2. each arm's highest coupling at most 1.25 of H-42's image's;
3. in every arm and control, a schedule that starts with half or more of its couplings' synapses in the bottom eighth of the width sums its reversals at 16 blocks or fewer; a run with no such schedule does not read it;
4. each arm's inhibitory sum within 0.45 and 0.65 of H-42's image's at every block's end of schedules 9 to 16.

**What would refute it:**
- a coupling passing 1.30 under the zero sum, on a network the forms were not read on;
- a mapping lost to a quarter's exponent where a network's couplings split early;
- a control that holds the bound without the zero sum.

### H-46's stopping rule

1. **One round**: brief 085 builds both rules with their tests and ADR, then runs H-46 once. Its clauses, constants and readings are committed before any arm runs.
2. **A calibration stops the round**, and is a finding, if a pinned number moves with both rules unset, or if a control's first schedule is not H-42's arm on its network bit for bit.
3. **Yes**: the couplings are bounded over sixteen schedules on three networks, M12's exit test for them; reading 4 reads the inhibitory sum. The next decision is whether the two rules enter the engine's default, whether M12 closes, and then M11.
4. **No on clause 1 by a coupling**: an ADR on the zero sum's groups.
5. **No on clause 1 by learning**, a mapping or a reversal: an ADR on the exponent.
6. **No on clause 1 by the excitatory sum outside the couplings**: an ADR on what the exponent does to the rest of the arena.
7. **No on clause 2**: an ADR on what the default holds on that network.
8. **Where several steps apply**, the first in this order is taken.
9. **No constant moves after such a run.** There is no second attempt, and no other exponent, rounding, arm or network.

### F-86, and the sweep

- **F-86 stays open.** [ADR-0224](0224-a-unit-served-on-arrival.md) named M12's decision to resolve it. H-46's measured ADR resolves it with the round's reading of the synapses at the rail.
- **The schedule's run of 2026-10-28 at 20:17Z** has not begun when this is written. The first ADR that merges after it ends records it.

## Consequences

- **Good**: the couplings are bounded by a rule on the stimulus's own synapses, with no cap on a single synapse and, at a quarter's exponent, the default's speed in the explorations.
- **Good**: unset, both rules are the engine as it is, and no format moves.
- **Bad**: ADR-0055's depression gains a setting, and the zero sum keeps a carried fraction a synapse in its groups, outside the image. A reloaded image needs its groups set again.
- **Bad**: the round runs twenty-four jobs of about 100 minutes on AArch64, and the same again to reproduce the long runs. Seven megabytes of images enter the tree.
- **Bad**: the weekly job gains six short forms of a schedule each, on x86-64 about as long as H-42's arms there, 526 to 980 s each.
- **Neutral**: the long runs stay in the tree, runnable by name.

## Alternatives considered and why rejected

- **Option 1(b), to the nearest**: a synapse whose share of a scaling is under half an LSB keeps its weight, so the largest pay every round.
- **Option 1(c), down**: the rounding, not the rule, chose which synapses paid; why that made learning fast was not read.
- **Option 2(b), a half**: its reversals summed 13 to 18 blocks over schedules 9 to 16, the default's 12 to 16.
- **Option 2(c), one**: 16 to 24 over the same schedules.
- **Option 3(b), the image**: a format change for what the host already gives, the task's sets.
- **Option 4(b), all weekly**: about two and a half times the weekly job's machine time. **Option 4(c)**: no pin of either rule held weekly.
- **Option 5(b)**: a comparison on seed 22 would rest on an exploration's numbers, which are not evidence.
- **The sliding threshold** (UPG-02), Appendix C's named candidate: its bytes are `v_slow`'s, its layout is undecided ([ADR-0187](0187-the-upgrade-suite-planned.md)), and no exploration read it.

## Confirmation

`briefs/085_a-zero-sum-at-the-stimulus.md` builds both rules and runs H-46 once.
[H-46's report](../research/H-46.md) holds the hypothesis as written before any run.
`npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
