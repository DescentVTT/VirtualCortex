---
status: proposed
date: 2026-09-30
depends-on: ADR-0136
decision-makers: VirtualCortex maintainers
---

# ADR-0137: The reward unaddressed, measured — brief 057's protocol, committed before any rewarded run: H-23's image and arms with the task's delivery global and nothing else changed; the harness's run under a delivery, the addressed delivery's run bit for bit; the composer's four pairs consolidating under the signal, and a network's oracle replaying every excitatory synapse of the arena, held to the record at every trial; H-23's first block under the addressed delivery reproduced before it; H-24's three clauses as integer rules and the readings beside them

## Context and Problem Statement

[ADR-0136](0136-the-reward-unaddressed.md) took the reward's address and wrote **H-24** before any run: on H-23's configuration with the dopamine term delivered to every synapse (`Delivery::Global`), (1) every mapping is learned, at least 80 of its last 128 correct; (2) no stimulus–readout coupling passes 1.30 of its image's at any block's end; (3) at every block's end the summed magnitude of every excitatory synapse outside the four stimulus–readout couplings lies within 0.75 and 1.25 of the image's; all in both arms. No prediction is made for the verdict. Brief 057 runs H-24 once. This ADR is the round's protocol, committed before any rewarded run, and then its readings.

What was read before anything was written (principle 2, and brief 057's directive that the engine is read before a description of it is trusted, the brief's and ADR-0136's included):

1. **The delivery** (`runtime/cortex-runtime/src/task.rs`, `Task::trial`): between a trial's last tick and its reward the task writes the addressed set whatever the feedback — `exec.address_all()` under `Delivery::Global`, `exec.address(sources, targets)` under `Delivery::Addressed`. `Executor::address_all` sets every unit a source and a target, the state a new executor starts in, so the lead-in and the first trial are addressed alike under either delivery, under the signal at rest.
2. **The consolidation under it** (`runtime/cortex-runtime/src/executor.rs`, the fan-out and `Modulations::for_synapse`): a synapse is addressed when its source is an addressed source and its target an addressed target. An addressed excitatory synapse consolidates under the modulation from the signal — `clamp(baseline + dopamine, −1, 1)` with the signed gate set — and any other under the modulation at rest, `clamp(baseline, 0, 1)`, which is zero at the gate's baseline, so that it moves nothing. An inhibitory synapse consolidates under its own baseline, 0.5, whatever the addressing. So under the global delivery every excitatory synapse of the arena consolidates under the signal at every presynaptic spike, and the inhibitory ones as under H-23.
3. **The harness** (`runtime/cortex-runtime/tests/instrument/harness.rs`): `earned_run_valued` builds its task with `Delivery::Addressed` written in, asserts the addressed set at every trial to be the presented stimulus onto the selected readout, and feeds the composer the pair the last trial addressed. The composer replays the 3 188 synapses from a stimulus unit onto a readout unit and consolidates the addressed pair's under the course of the signal the last reward left, every other under the baseline; a replayed spike before its trial's start, the lead-in's, is never addressed there. Under the global delivery each of these is wrong: the set is every unit, the four pairs consolidate under the signal, and the lead-in's spikes are addressed.
4. **H-23's assertion** (`punished_held`, over `Reach`) — no excitatory synapse outside the four pairs moves — does not apply under the global delivery, by construction.
5. **The network outside the pairs** at 1 024 units (the instrument's network, whose wiring every image of it carries): 820 excitatory units, 26 240 excitatory synapses, 3 188 in the four pairs and 23 052 outside them. By the class of their source and their target in `CLASSES`'s order — an inhibitory unit, a stimulus unit, a readout unit, any other — the outside ones are 69 and 7 from a stimulus unit onto a stimulus unit and onto another; 4 857, 2 452, 15 456 and 83 from a readout unit; and 23, 9, 81 and 15 from the four units past the last whole period. Every inhibitory unit lies in a readout set, so a stimulus unit's synapse onto any readout unit is a pair's.
6. **The image's outside sum**: the settled image's excitatory sum, 218 243 354, which H-20's to H-23's images carry, less its four couplings, 26 347 838 — **191 895 516**.

## Decision Drivers

- Brief 057's standing directives: nothing of the engine changes and no file under `src/`; the delivery the one change from H-23, the image H-23's bit for bit; H-24's clauses and constants committed before the first rewarded run and not moved after it; no second attempt; no float; every loop ends by construction; the gate grows by at most one test; no shard of the weekly job past 60 per cent of its bound; no pinned number of an earlier round moves.
- Latest ≠ Newest: no dependency, no tool.
- The readings must be trustworthy where the round looks for the first time: the network outside the pairs, which no oracle has replayed and which the address kept still.

## Considered Options

1. **How the arms are built on H-23's harness**: (a) the harness's run under a delivery, `earned_run_delivered`, which `earned_run_valued` calls with the addressed delivery; (b) a copy of `earned_run_valued` with the delivery changed.
2. **What the oracle replays under the global delivery**: (a) the composer's four pairs alone, every one under the signal, the network outside them read from the record; (b) the four pairs, and a network's oracle replaying every excitatory synapse of the arena from the train, held to the record's traces, weights and stamps at every trial.
3. **The calibration's reproduction of H-23**: (a) H-23's two weekly tests alone; (b) those, and in each H-24 arm H-23's first block from H-23's image under the addressed delivery, held to H-23's tables, with the network's oracle beside it.
4. **Where each reading is read**: the network's cells from the record at every block's end; where the consolidation went from the network's oracle, held to the record.

## Decision Outcome

Options 1(a), 2(b), 3(b) and 4. Everything below is committed before any rewarded run.

### The harness (option 1(a))

- **`earned_run_delivered`** is `earned_run_valued` with the task's delivery a parameter, and `earned_run_valued` calls it with `Delivery::Addressed`, so every run before this round is the run it was: the same task, the same assertions, the same composer.
- **Under the global delivery** it asserts at every trial that every unit is a source and a target, and sets `Composer::global`: every replayed synapse consolidates under the course's signal, and a spike before its trial's start — only the first reading holds one, the lead-in's, before any reward — under the signal at rest, which the composer asserts. The composer's moves are read over the pair the last trial selected, the pair the addressed delivery would have addressed, so that they read as H-23's did.

### The network's oracle (option 2(b))

The four pairs are 3 188 of the 26 240 excitatory synapses; clause 3 reads the other 23 052, which no oracle has replayed. The network's oracle replays all 26 240:

- **Its state**: every excitatory unit's chain, block by block, with the block's stamp and each occupied slot's target, trace and weight as the record holds them at the run's start; every unit's last spike from the record; the executor's addressed set, every unit at the start.
- **Its rule, the harness's second writing** (`replay_block`): at a presynaptic spike, each slot's trace decayed by the ticks since the block's stamp; the pair rule against the target's last spike — the potentiation when the target fired after the stamp and not after the spike, the depression scaled by the magnitude when it fired before, neither without a spike on record; the consolidation under the baseline plus the signal where the addressed set holds the source and the target, and the baseline alone elsewhere, by `consolidated_signed`; then the stamp.
- **Its order**: tick by tick over the train since the last reading, every spiking unit's last spike written first, as the executor's integration precedes its fan-out, then each spiking excitatory unit's blocks; the signal from the course the last reward left, at rest before the first trial's start; the addressed set and the signal read again at every trial's end, as the delivery and the reward wrote them.
- **Its check**: at every trial the record's trace, weight and stamp of every excitatory block held to the oracle's, 26 240 synapses, in the calibration's block under the addressed delivery and in both arms under the global one.
- **The gate holds its rule to the engine's**: a block written by hand — traces of both signs, a weight at each rail — through six presynaptic spikes whose targets fired before, after and at the spike or never, under modulations below, at and above zero, with the signed gate set and unset, the oracle's traces, weights, stamp and moves the engine's `step_stdp_all` and `consolidate_signed` after every spike; and eight trials on the instrument's network under each delivery, both oracles held at every trial, the network outside the pairs unmoved under the addressed delivery and moved under the global one.

So what clause 3 reads is held to a second writing of the rule at every trial, and not only read from the record.

### H-24, restated as integer rules (ADR-0136's clauses and constants)

- **Clauses 1 and 2**: H-20's `scheduled`, unchanged — each mapping's last 128 trials at least 80 correct, and no coupling above 1.30 of its image's at any block's end.
- **Clause 3, the network holds** (`within_band`, `left_band`, `outside_of`, `image_outside`, `BAND_QUARTERS`, `BAND_DIVISOR`): at a block's end, the outside sum — the excitatory sum over the arena less the four couplings — holds when $3 \times \text{image} \le 4 \times \text{outside} \le 5 \times \text{image}$, the image's outside sum 191 895 516 above zero; so from 143 921 637 to 239 869 395. The clause holds in an arm when its run has 120 blocks and every block's end holds; `left_band` names the first block that did not, with its sum.
- **The verdict** (`Unaddressed`): `learning` (H-20's `Scheduled`), `left` and `held` per arm, and `yes` when all hold in both arms. `UNADDRESSED_PREDICTED` is none: no prediction.

### The readings, no clause (ADR-0136's)

- **The four couplings per block** beside H-23's, from the sight's table.
- **The network outside the couplings by cell** (`cells_of`, `Cells`, `CELL_SIZES_1024`), at every block's end from the record: the rows the source's class — a stimulus unit, a readout unit, another excitatory unit — and the columns the target's class in `CLASSES`'s order; per cell the weights summed, the synapses whose weight differs from the image's, the largest move from the image's weight with its sign, and the synapses at zero and at `i16::MAX`. The cells sum to the outside sum.
- **Where the consolidation went** (`Went`), per block by the network's oracle: the weight raised and the weight lowered in the answer's pairs — each stimulus onto its answer under the mapping in force at the trial the consolidation fell in — in the other two pairs, and outside the pairs. The baseline is zero, so every consolidation is the reward's.
- **The population's spikes by class** per block, from the train.
- **The reversal speeds** (`crossings`, `crossed_scheduled`, `first_new_scheduled`), **the value beside $2p - 1$** (`stimulus_values`, and H-23's clause 4 rule `holds_expected` as a reading) and **the troughs after each flip** (`troughs`), beside H-23's.
- **The inhibitory sum's course**, each block's as a fraction of the settled image's, beside H-23's.
- **H-23's other readings** by H-20's rules — the tallies, the settle measure, the highest coupling per mapping, the strong punishments, the moves by mapping, the blocks in which the stimulus fired once, the sums after — the engine's value and the weights by group, the spikes the window admitted, and the reach, which H-23 asserted and this round reads.

### The calibration, before any rewarded run (H-24's stopping rule, step 2)

- **In the tree**: the workspace's tests in the debug and the release profile and on the MSRV; every whole-domain test of the weekly job, H-23's two arms among them, each in a process of its own from the release build of the protocol's commit; the determinism pin with them.
- **In each arm's test**, before its first rewarded trial: the settled engine held to ADR-0077's tables step by step and H-20's image to its CRC (`signed_images`); a frozen block from the zero image held to ADR-0077's frozen run; H-21's, H-22's and H-23's images by their CRCs, H-23's `WINDOWED_IMAGE_CRC_1024`; and **H-23's first block from H-23's image under the addressed delivery** (`h23_first_block`), 64 trials held to H-23's pinned first block table by table — the sight, the composition, the earned block, the moves, the value, the weights by group and the spikes the window admitted — with the network's oracle held at every trial beside it, reading nothing moved outside the pairs and the pairs moved by what the composer consolidated.

A failure at any of them stops the round there as a finding.

### The gate

`the_clauses_of_h_24_the_network_s_oracle_and_the_readings_rules`, one test:
- the arms, the delivery and the constants; the image's outside sum and the couplings making the excitatory sum;
- clause 3 at its edges — 0.75 and 1.25 of an image hold, one LSB beyond does not, an image not a multiple of four rounds the band inward, no image holds nothing — and over blocks written by hand at the band's two ends and one LSB past; the verdict naming each clause and the arm, a run not whole holding no clause;
- the places over the instrument's network — the pairs the composer's 3 188 synapses, every other excitatory synapse in a cell of its source's row, the network's oracle replaying all 26 240 — and the cells against its own weights and against three written by hand, one at each rail and one moved;
- eight trials on that network under each delivery, and the oracle's replay of a block against the engine's rule, as above;
- the readings' rules over values written by hand.

### The weekly tests and their cost

- **Two weekly tests**, one per arm: `the_reward_unaddressed_from_the_assignment_at_1024_units_exhaustive` and `the_reward_unaddressed_from_the_mirrored_assignment_at_1024_units_exhaustive`.
- **Each is H-23's arm, a block more and the network's oracle**: H-23's arms took 1 362 and 1 845 s on the hosted runners; the first block adds a sixtieth of a run, and the oracle's replay and check a few per cent.
- **The plan stays inside the budget.** The table after ADR-0135 is 77 tests and 29 058 s, about 34 per cent of the bound at six shards. The two arms add about 3 300 s, and the plan rises to about 38 per cent, under the directive's 60.
- **The dispatch's scope**: the round changes no file under `src/`, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly is dispatched at `scope=exhaustive`.

### The order of the work

1. This commit: the harness's delivery, the network's oracle, the rules, the constants, the gate, the arms with empty pins, and this ADR's protocol, before any run.
2. The calibration above, before any rewarded run.
3. The two arms, once, stopping at their first empty pin; the tables written from the dumps, and a second run to reproduce them.
4. This ADR's readings and the documents; then the weekly dispatched on the branch at `scope=exhaustive` and the cost table regenerated from it.

## Consequences

- Good: only the delivery moves from H-23's configuration, so a difference from H-23 is the address's.
- Good: every excitatory synapse the global reward reaches is held to a second writing of the rule at every trial, so clause 3 and the readings outside the pairs rest on an oracle and not only on the record.
- Good: the addressed delivery's run is the harness's as it was, bit for bit, and H-23's first block reproduces under it before any rewarded run.
- Neutral: the whole run is pinned, 120 blocks per arm, as H-23's, with the network's cells per block beside it.
- Bad: one seed, one size, one schedule, as H-23's; the readout sets and the selection stay the host's (ADR-0136).

## Confirmation

- `runtime/cortex-runtime/tests/instrument/harness.rs`: `earned_run_delivered`, `earned_run_valued`, `Composer::global`.
- `runtime/cortex-runtime/tests/inhibition.rs`: `UNADDRESSED_ARMS`, `UNADDRESSED_DELIVERY`, `UNADDRESSED_PREDICTED`, `BAND_QUARTERS`, `BAND_DIVISOR`, `SOURCES`, `outside_of`, `image_outside`, `within_band`, `left_band`, `Unaddressed`, `unaddressed`, `Place`, `place_of`, `Cell`, `Cells`, `cells_of`, `cell_sizes`, `cells_sum`, `Went`, `Wired`, `Wiring`, `wiring_of`, `replay_block`, `Network`, `Watch`, `delivered_run`, `h23_first_block`, `unaddressed_arm`; the two weekly tests and the gate's test named above.
- Every pinned number of ADR-0065 to ADR-0135 unchanged, and the determinism pin; the image format 20.
