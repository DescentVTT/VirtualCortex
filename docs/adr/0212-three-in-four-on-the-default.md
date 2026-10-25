---
status: accepted
date: 2026-10-25
title: A reward right three times in four, on the default
kind: decision
hypothesis: H-44
summary: H-35's question asked again under the engine's default on seeds 22, 23 and 24, beside a control whose image's step is zero, so that its gain is held through the arm. H-44 is predicted yes. The pinned tables stay where they are (F-81), and brief 080's sweep is recorded.
depends-on: ADR-0206
decision-makers: VirtualCortex maintainers
---

# ADR-0212: A reward right three times in four, on the default

**In brief**
- **Decision.** H-35's question is asked again under the engine's default: on seeds 22, 23 and 24, H-42's arms with the task's feedback `ThreeInFour`. Beside each runs a control whose image's step is zero, so that its gain is held through the arm. H-44 is predicted yes.
- **Why.** H-43's stopping rule named this next. Exploration 001 read the question yes on three networks under the default, where with the gain held it had read no on two. The default changes the network and the rule at once, and the control separates them.
- **Left open.** A delay the engine is not told, which comes next. The sweep's test phase, where exploration 002 read its time, is named and not taken.

## Context and Problem Statement

[ADR-0210](0210-a-reward-past-the-trial-its-address-kept-measured.md) read H-43 yes at step 3 of its stopping rule, which [ADR-0207](0207-a-reward-past-the-trial-its-address-kept.md) wrote before any run: *"the reward right three times in four asked again on the default, with a control at step zero. Then a delay the engine is not told."*

**What the share read before** ([H-35](../research/H-35.md), [H-36](../research/H-36.md)).
Under H-25's configuration, with the gain held at 1.75 and the coin `MISLEADING_IN_FOUR_BITS` misleading 1 913 of 7 680 trials:

| Network | Arm | Last 128 correct, by mapping | Read |
| :--- | :--- | :--- | :--- |
| Seed 22 | Assignment first | 115, 80, 110, 75 | no on clause 1 (H-35) |
| | Mirrored first | 117, 94, 110, 97 | |
| Seed 23 | Assignment first | 121, 115, 121, 119 | yes (H-36) |
| | Mirrored first | 121, 97, 117, 112 | |
| Seed 24 | Assignment first | 121, 71, 116, 76 | no on clause 1 (H-36) |
| | Mirrored first | 107, 119, 112, 112 | |

**What exploration 001 read under the default** ([its note](../explorations/001-the-old-noes-under-the-default.md), exploratory and not evidence).
The same coin, schedule and delivery ran on H-42's three networks, the reward given through the host:
- every mapping was learned in all six arms, the lowest at 90 of 128, and all four of H-35's clauses held;
- all 18 reversals reached 40 of 64 inside their 32 blocks, the slowest in 28, at 19.9 blocks on average, where the 16 that crossed with the gain held took 20.3;
- *"This run cannot say what made the difference."* The default changes the network it settles, with the targets it read, and the gain the rate's rule moves through every arm.

**The step in the image.** After the probe the rate's rule runs at the step the probe made, about 1/61 to 1/60 on these networks ([ADR-0204](0204-a-step-the-engine-makes-from-its-own-loop-measured.md)).
The step is the homeostasis record's `control_step_q0_16`, bytes 20 to 22 (`crates/cortex-homeostasis`), and the image carries it. At zero the rule leaves the gain.
An image with that field written zero is the default's network with its own gain held through the arm. The tests already write a section's bytes and re-seal it in this way.

**Where the pinned tables live, closed.** [ADR-0211](0211-the-round-shortened.md) left it to an exploration. Exploration 002 ([its note](../explorations/002-what-the-pinned-tables-cost.md)) read, exploratory and not evidence:
- the release test binaries built in 67.2 s with the tables as `const` items, and in 69.4 and 67.5 s with them in a data file; a cold `cargo check` was about 3 s shorter with the data file;
- brief 079's sweep spent 12.2 % of its runtime machine time building, 67.9 % on 164 slowly caught mutants and 14.9 % on 17 timeouts;
- the gate's release step, 493 s on #234's run, held 46 s of build.

**F-81.** [F-76 and F-78](../WHITEPAPER.md#11-risks-and-technical-debt) name the rounds' pinned tables, growing the test crates' compile time, as the cause of the release step's growth and of the sweep's length. The tables cost little of either.

**The sweep of brief 080's dispatch, recorded here** ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).
Dispatch [40171928796](https://github.com/DescentVTT/VirtualCortex/actions/runs/40171928796) on `4bc7f7e` ended its seventeen sweep jobs at 12:50Z on 2026-10-25, all green. The runtime's sixteen took 2 h 15 min to 4 h 01 min, under their bound of 330 minutes.

| Mutants | Count |
| :--- | ---: |
| Listed | 4 031 |
| Tested | 4 031 |
| Caught | 3 843: 2 418 in the state crates, 1 425 in the runtime |
| Missed | 0 |
| Unviable | 160 |
| Timeouts | 28 |

- 22 timeouts are where [ADR-0062](0062-the-first-complete-sweeps-list.md) reads them as inherent: `cortex-core`'s three chain iterators, six; the injector, twelve; the barrier, one; the workers' stop and run, three.
- 6 ran beside an injector timeout: four in `Image::decode`'s loading, and two in the store's search, `store.rs:148:9` and `store.rs:259:12`. Three were still finishing tests, F-42's kind by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence, and three had stopped finishing tests beside the hang.
- There is no survivor.

## Decision Drivers

- **The maintainers' choices** (2026-10-25): this question next, as H-43's stopping rule named it; the pinned tables left where they are.
- **What the exploration could not separate**: the network the default settles, and the rule that moves the gain through the arm.
- **A control on the same network, one field apart**, separates them with nothing added to the engine.
- **Latest ≠ Newest**: no mechanism, crate or tool is added; one field of an image is written zero by a test.

## Considered Options

1. **The control**: (a) the default's image with its step zero; (b) none, the default's arms alone; (c) an engine born under the default with the probe unset.
2. **The networks**: (a) seeds 22, 23 and 24; (b) seeds 22 and 24, where the share read no.

## Decision Outcome

**Options 1(a) and 2(a).**

### The runs (Specified; brief 081 builds them in the tests)

On each of seeds 22, 23 and 24 at 1 024 units, from the image an engine born under `Config::reads_its_targets()` settles (H-42's derivations, pinned):
- **the default's arms**: H-42's two arms, H-25's on H-20's schedule, with the task's feedback `Feedback::ThreeInFour`, H-35's coin;
- **the control's arms**: the same from the image with the homeostasis record's step written zero and the image re-sealed, every other byte the image's;
- twelve arms in all. No file under `src/` is expected to change.

### H-44 (Hypothesis; written before any run)

The text, with its clauses, readings and stopping rule, is [H-44's report](../research/H-44.md).
- **Clauses 1 to 4**: H-35's, in each of the default's six arms.
- **Yes** when all four hold in all six; otherwise **no**, with the network, the arm and the clause.
- **The control** is read by the same four clauses, by network, arm and clause. It splits the reading of a yes, and is not the verdict.

**Predicted: yes.** The exploration read every clause in every arm under the default. Its runs are not evidence, and this round measures them by its own protocol.

**Three readings are predicted:**
1. the control is no on clause 1: a mapping below 80 of its last 128 in at least one of its six arms;
2. on each network and arm, the default's lowest mapping stands above its control's;
3. all 18 of the default's reversals reach 40 of 64 inside their 32 blocks.

The first two say that the rule through the arm, not the network alone, holds the share.
With the gain held, H-35 and H-36 read their four weakest mappings at 71 to 80 of 128, and the exploration under the default read them at 90 to 94.

**What would refute it:**
- the rule taking back what the couplings learned: as they grow the rate rises, and the rule lowers the gain;
- the task's own feedback giving runs other than the host's path the exploration took;
- seed 24's two mappings that fell short with the gain held falling short again.

### H-44's stopping rule

1. **One round**: brief 081 builds the control's image and runs H-44 once. Its clauses, constants and readings are committed before any arm runs at three in four.
2. **A calibration stops the round**, and is a finding, if:
   - a pinned number moves;
   - a network's derivation is not H-42's;
   - a default arm's first block under the true feedback is not its H-42 arm's;
   - the coin's count is not H-35's, 1 913 of 7 680;
   - the control's image differs from the default's but in the step's two bytes and the image's checksums, or its gain moves in an arm.
3. **Yes**: the default holds a reward right three times in four on three networks. The next decision is a delay the engine is not told. The control splits what the yes is read as:
   - (a) the control no: what holds the share is the rule moving the gain through the arm;
   - (b) the control yes: what holds it is the network the default settles, and the rule is not needed for this share.
4. **No on clause 3** on any network: an ADR on what a misleading reward reaches.
5. **Otherwise no on clause 1 or 2**: the ADR H-35's step 5(b) named, on a single error's weight and the gate's clamp, with the default's readings and the control's as its need.
6. **Otherwise no on clause 4 alone**: an ADR on the critic's step under noise.
7. **No constant moves after such a run.** There is no second attempt, and no other share, step, arm or network.

### F-81, and the tables

- **F-81**, recorded and resolved by this record: what grows the release step and the sweep is their tests, by exploration 002's reading, not the tables' build.
- **The tables stay in the test sources.** Moving them would buy about 3 s of `cargo check`, at the cost of a reader behind every table and a parser of the moved text.
- F-76 and F-78 are resolved with it: the release step's bound and the sixteen shards stand, and their cause is F-81's.

## Consequences

- Good: the question H-35 and H-36 left at no is asked on the configuration the engine now starts with.
- Good: the control's image is one field apart from the default's, so a split of the yes needs no other mechanism.
- Bad: twelve arms more in the weekly job, about 18 000 s on x86-64 at H-43's 1 100 to 1 900 s an arm.
- Bad: the control is a test's construction. No engine settles to a step of zero under the default.
- Neutral: no record, no section and no format changes.

## Alternatives considered and why rejected

- **Option 1(b), no control**: the exploration already read the default's arms. Without a control, the round could not say what holds the share.
- **Option 1(c), the probe unset**: an engine born so runs the rule at a sixteenth from H-41's network. It separates two steps on two networks, not the rule from the network.
- **Option 2(b), two networks**: the control's reading needs seed 23 too, where the share held with the gain held.

## Confirmation

`briefs/081_three-in-four-on-the-default.md` builds the control's image and runs H-44 once.
[H-44's report](../research/H-44.md) holds the hypothesis as written before any run.
`npm run spec` holds the links, the tables, the brief's contract and scope, and the version.
