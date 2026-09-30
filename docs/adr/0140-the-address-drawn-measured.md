---
status: proposed
date: 2026-09-30
depends-on: ADR-0139
decision-makers: VirtualCortex maintainers
---

# ADR-0140: The address drawn, measured — brief 058's protocol, committed before any rewarded run: H-23's image and arms with the task's delivery drawn and nothing else changed; the drawn sources held at every trial to the units the harness's critic oracle counted, the composer's four pairs and a network's oracle over every excitatory synapse consolidating under the address the executor drew; H-23's first block under the host's address reproduced before it; H-25's four clauses as integer rules and the readings beside them, the drawn sources per trial beside the host's among them

## Context and Problem Statement

[ADR-0138](0138-the-address-drawn.md) had the engine draw the reward's address and wrote **H-25** before any run: on H-23's configuration with the address's sources drawn by the engine in place of the host's, (1) every mapping is learned, at least 80 of its last 128 correct; (2) no stimulus–readout coupling passes 1.30 of its image's at any block's end; (3) each of the three reversals passes 40 of 64 within 23 blocks of its flip; (4) at every block's end the summed magnitude of every excitatory synapse outside the four stimulus–readout couplings lies within 0.75 and 1.25 of the image's; all in both arms. The account's prediction, written first, is yes. [ADR-0139](0139-the-address-drawn-built.md) built the drawing. Brief 058 runs H-25 once. This ADR is the round's protocol, committed before any rewarded run, and then its readings.

What was read before anything was written (principle 2, and brief 058's directive that the engine is read before a description of it is trusted, the brief's and ADR-0138's included):

1. **The drawing** (`runtime/cortex-runtime/src/executor.rs`, `Executor::address_drawn`; `runtime/cortex-runtime/src/task.rs`, `Delivery::Drawn`): between a trial's last tick and its reward the task calls the executor with the selected readout's units, none at a tie, and the executor writes every unit a source whose critic count is not zero. The counts are the spikes the critic's window admitted since the previous reward, which zeroes them after the call.
2. **What the drawn address reaches** (`Modulations::for_synapse`, the fan-out): an excitatory synapse consolidates under the signal when its source is drawn and its target a unit of the selected readout's set; any other under the baseline, zero, which moves nothing; an inhibitory one under its own baseline whatever the address. So the reward reaches:
   - the host's pair, the presented stimulus's drawn units onto the selected readout;
   - the other stimulus's drawn units onto the selected readout, a pair the host's address never reached;
   - outside the pairs, the drawn readout units and the four units past the last whole period onto the selected readout's units, among which every inhibitory unit lies (ADR-0137's census).
3. **The window's first trial.** The harness runs a lead-in of 500 ticks (`LEAD_IN`) before the first trial, and the window opens at the load. It closes in the lead-in, so the first trial's window admits nothing: H-23's first block, which presents stimulus A at its first trial, admitted 1 677 spikes to A's units over 34 presentations, 57 short of 34 volleys of 51, and a probe of the train on the instrument's network read the first trial ending at tick 16 884, the lead-in's 500 ticks and the trial's 16 384 after the load. So the first drawing names no source and the first reward reaches no synapse. Every later window opens at a reward, at the next trial's first tick.
4. **H-23's window** ([ADR-0135](0135-the-critics-window-measured.md), `WINDOWED_ADMITTED_1024`): about 52.4 spikes a trial, 50.8 of them the presented stimulus's volley. The drawn sources are the units, not the spikes: a unit that fired twice in the window is one source.
5. **The harness** after ADR-0139: `earned_run_delivered` under the drawn delivery holds the executor's sources at every trial's end to the units its own critic oracle counted, and the targets to the selected readout's; the composer consolidates a replayed stimulus–readout synapse under the signal where its readout was selected and its source drawn (`Composer::drawn`). The network's oracle of brief 057 (`Network`) reads the executor's addressed set at every trial's end, so it replays the drawn address as it replayed the other two.
6. **H-24's arm** (`unaddressed_arm`) is H-23's configuration with a delivery changed, its calibration H-23's first block under the host's address (`h23_first_block`), its oracles both held at every trial, and its readings the cells, where the consolidation went and H-23's; `delivered_run` takes the delivery as a parameter.

## Decision Drivers

- Brief 058's standing directives: the sources from the executor's own counts, never the task's stimulus; nothing of H-23's configuration changed but the address's sources; H-25's clauses and constants committed before the first rewarded run and not moved after it; no second attempt; no float; every loop ends by construction; the gate grows by at most one test; no shard of the weekly job past 60 per cent of its bound; no pinned number of an earlier round moves.
- Latest ≠ Newest: no dependency, no tool.
- The readings must say what the drawing added to the host's address, trial by trial, and where the reward went because of it.

## Considered Options

1. **How the arms are built**: (a) H-24's arm with the drawn delivery, `delivered_run` unchanged but for the sources read beside it; (b) H-23's arm with the delivery changed, no network's oracle.
2. **The drawn sources' reading**: (a) at every trial's end, the executor's sources by group, per block and presented stimulus, and every trial's by its hash; (b) the drawn units listed per trial.
3. **The calibration's reproduction of H-23**: (a) H-24's, H-23's first block from H-23's image under the host's address held to H-23's tables, with the network's oracle beside it; (b) a first block under the drawn delivery held to H-23's tables.

## Decision Outcome

Options 1(a), 2(a) and 3(a). Everything below is committed before any rewarded run.

### The arms (option 1(a))

- **`drawn_arm`** is `unaddressed_arm` with `DRAWN_DELIVERY`, `Delivery::Drawn`: H-23's image, arms, flips and critic, the task carrying no critic of its own; the engine held to H-23's configuration with `Executor::draws` true at the load.
- **Both oracles at every trial**: the composer's 3 188 pair synapses under the drawn pair, and the network's 26 240 excitatory synapses under the executor's addressed set, each held to the record's traces and weights, the network's stamps too; the drawn sources held to the harness's critic oracle's counts and the targets to the selected readout.
- **What is read beside H-24's** (`Watch::sourced`, option 2(a)): at every trial's end the executor's sources by group in `GROUPS`'s order — inhibitory, stimulus A's set, B's, readout 0's, readout 1's, any other — summed per block and presented stimulus (`Sourced`), with every trial's counts hashed (`sources_hash`). The same reading under the host's address is the presented stimulus's set and nothing else, and under the global one every unit. A list of units per trial (option 2(b)) is 7 680 lists an arm, where the groups say what H-25 asks: the host's units in and out and the others in.
- A first block under the drawn delivery (option 3(b)) could not be held to H-23's tables: the address differs from the first reward on.

### H-25, restated as integer rules (ADR-0138's clauses and constants)

- **Clauses 1 and 2**: H-20's `scheduled`, unchanged — each mapping's last 128 trials at least 80 correct (`REWARDED_MIN`), and no coupling above 1.30 of its image's at any block's end (`BOUND_PER_CENT`).
- **Clause 3**: H-23's `reversals_within` of `crossings` — each of the second, third and fourth mappings passes `CROSSING_MARK`, 40 of 64, within `REVERSAL_BLOCKS_MAX`, 23 blocks of its flip, the crossing block counted; a reversal that never crosses does not hold.
- **Clause 4**: H-24's `left_band` — at every block's end $3 \times \text{image} \le 4 \times \text{outside} \le 5 \times \text{image}$, the image's outside sum 191 895 516, over a run of 120 blocks.
- **The verdict** (`Drawn`, `drawn`): `learning`, `revised`, `left` and `held` per arm, and `yes` when all four hold in both. `DRAWN_PREDICTED` is yes, ADR-0138's, dumped beside the reading and never asserted.

### The readings, no clause (ADR-0138's)

- **The drawn sources per trial beside the host's** (`Sourced`, `beside_host`): per block and stimulus, the presented stimulus's units drawn, its units left out — its set's 51 times its presentations less those drawn — and the units of every other group drawn.
- **The synapses outside the pairs that moved, by H-24's cells** (`cells_of`), and **where the consolidation went, by H-24's measure** (`Went`): the answer's pairs, the other pairs and outside, by the network's oracle per block.
- **The couplings' separation, the reversal speeds, the value beside $2p - 1$ and the troughs**, beside H-23's; **the inhibitory sum's course** beside H-23's.
- **H-23's and H-24's other readings** by their rules — the tallies, the settle measure, the highest coupling per mapping, the strong punishments, the moves by mapping over the pair presented onto the pair selected, the blocks in which the stimulus fired once, the sums after, the engine's value and weights by group, the spikes the window admitted and by class, and the reach.

### The calibration, before any rewarded run (H-25's stopping rule, step 2)

- **In the tree**: the workspace's tests in the debug and the release profile and on the MSRV; every whole-domain test of the weekly job, H-23's and H-24's arms among them, each in a process of its own from the release build of the protocol's commit; the determinism pin with them.
- **In each arm's test**, before its first rewarded trial: ADR-0077's settled engine, H-20's to H-23's images by their CRCs, a frozen block held to ADR-0077's frozen run, and **H-23's first block from H-23's image under the host's address** (`h23_first_block`), table by table, the network's oracle beside it.

A failure at any of them stops the round there as a finding.

### The gate

`the_clauses_of_h_25_the_drawn_sources_and_the_readings_rules`, one test:
- the arms, the delivery, the prediction and the constants, H-23's clauses 1 to 3 and H-24's clause 3;
- the verdict over blocks written by hand, all four holding and each failing alone, named: a reversal passing the mark in its 23rd block holds and in its 24th does not, the first mapping's crossing reads nothing, a mapping unlearned, a coupling past the bound, the band's two ends holding and one LSB past it not, a run not whole;
- the readings' rules over values written by hand: the drawn sources beside the host's, the host's own address reading no unit left out and none other, and the sources' hash;
- eight trials on the instrument's network under the drawn delivery, its image flagged as the arms' are and carrying the critic with its window by its rule: the drawn sources held at every trial to the counts, both oracles to the record, and every trial's sources by group within the spikes the window admitted, a group drawn from exactly when the window admitted a spike of it.

### The weekly tests and their cost

- **Two weekly tests**, one per arm: `the_address_drawn_from_the_assignment_at_1024_units_exhaustive` and `the_address_drawn_from_the_mirrored_assignment_at_1024_units_exhaustive`.
- **Each is H-24's arm with the delivery drawn**: H-24's arms took 1 809 and 1 907 s on the hosted runners. The table after ADR-0137 is 79 tests and 34 700 s, about 41 per cent of the bound at six shards; the two arms add about 3 700 s and the plan rises to about 45 per cent, under the directive's 60.
- **The dispatch's scope**: ADR-0139 changed files under `src/`, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly is dispatched at `scope=both`.

### The order of the work

1. ADR-0139's build; then this commit: the arms, the rules, the constants, the gate, the arms' empty pins and this ADR's protocol, before any run.
2. The calibration above, before any rewarded run.
3. The two arms, once, stopping at their first empty pin; the tables written from the dumps, and a second run to reproduce them.
4. This ADR's readings and the documents; then the weekly dispatched on the branch at `scope=both` and the cost table regenerated from it.

## Consequences

- Good: only the address's sources move from H-23's configuration, so a difference from H-23 is the drawing's, and a difference from H-24 is the address's.
- Good: the drawn sources are held at every trial to a second count of the train, and every synapse they reach to a second writing of the rule.
- Neutral: the whole run is pinned, 120 blocks per arm, with the drawn sources per block beside H-24's tables.
- Bad: one seed, one size, one schedule, as H-23's; the readout sets and the selection stay the host's, and the targets an efference copy (ADR-0138).

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `DRAWN_ARMS`, `DRAWN_DELIVERY`, `DRAWN_PREDICTED`, `Drawn`, `drawn`, `Sourced`, `beside_host`, `sources_hash`, `Watch::sourced`, `drawn_arm`; the two weekly tests and the gate's test named above.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `earned_run_delivered`, `Composer::drawn` (ADR-0139).
- Every pinned number of ADR-0065 to ADR-0137 unchanged, and the determinism pin; the image format 20.
