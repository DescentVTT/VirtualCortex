---
status: proposed
date: 2026-09-30
depends-on: ADR-0139
decision-makers: VirtualCortex maintainers
---

# ADR-0140: The address drawn, measured — H-25 is yes: with the reward's sources drawn by the engine from its critic's window at every trial's end and its targets the readout the engine selected, nothing else of H-23's configuration changed and both oracles held at every trial, every mapping was learned in both arms, 116 to 124 of each mapping's last 128, no coupling past 1.30, the highest 1.191, each reversal passed 40 of 64 in 14 to 21 blocks, within 23, and the excitatory sum outside the four couplings stayed within 0.9999 and 1.0002 of the image's; the prediction held; the drawing named 99.66 per cent of the presented stimulus's units and about 1.6 others a trial, the reward's net in the answer's pairs was 13 to 16 per cent of the weight it moved there against H-24's 0.8 to 2.9, and outside the pairs 1.1 to 1.8 per cent of the answer's pairs' weight moved, onto the selected readout's units only; the learning loop no longer takes the stimulus's identity from the host; H-25's stopping rule at step 3, and the next decision, an ADR choosing among the operating regime, another size, the rule held by the network reopened, a critic carried by a population, the planned neural form of the target side and a task the engine has not been asked, named and not taken

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

### The order of the work, as the history holds it

1. **The build** (`d21aefd` on the branch, ADR-0139) and **the protocol** (`5d36964`, pushed at 13:03Z on 2026-09-30 and pull request #174 opened as a draft on it, before any run): the arms, the rules, the constants, the gate, the arms' empty pins and this ADR's protocol.
2. **The calibration**, before any rewarded run, from the release build of `5d36964`: the workspace's tests passed in the debug profile (686 passed, 119 ignored, on `d21aefd`), in the release profile (687 passed, 121 ignored) and on the MSRV (687 passed), and the check, Clippy, the documentation and the format exited 0; **every one of the 79 whole-domain tests of the weekly job passed**, each in a process of its own and each exit 0 with one test reported — 69 eight at a time from 13:08:59Z to 14:59:18Z, H-23's and H-24's four arms among them, and the ten of ADR-0097 and ADR-0102 one at a time to 15:01:37Z; the pull request's checks on `5d36964` passed, the determinism pin on AArch64 among them, and the mutation gate on the changed lines found 8 mutants in ADR-0139's lines, 6 caught and 2 unviable, no survivor.
3. **The arms**, once, side by side from 15:02:12Z. In each the calibration held: ADR-0077's settled engine and H-20's to H-23's images by their CRCs, and H-23's first block from H-23's image under the host's address reproduced table by table, the network's oracle beside it reading nothing moved outside the pairs. Both ran their 7 680 trials with the drawn sources held to the harness's critic oracle and both oracles held to the record at every trial, and stopped at the first empty table, in 1 308 and 1 313 s. The tables were written from those dumps (`b9df7ba`), and a second run side by side from 15:26:17Z reproduced every table and passed, in 1 339 and 1 343 s (on the developer machine, a ratio and not admissible).

No constant, clause or rule moved after the first rewarded trial, and there was no second attempt: the second run is the pinned tables' reproduction.

### The readings

**The verdict** (`DRAWN_1024`), by the rule committed first, over the pinned tables:

- **Clause 1 holds in both arms**: each mapping's last 128 trials, **122, 124, 119 and 116** correct from the assignment and **124, 118, 123 and 117** from the mirrored assignment, against the mark of 80. H-23's were 120, 121, 120, 115 and 124, 116, 116, 123; H-24's 79, 78, 64, 87 and 89, 67, 64, 83.
- **Clause 2 holds in both arms**: no coupling above 1.30 of its image's at any block's end. The highest per mapping, as a fraction of the image coupling:

  | Arm | First mapping | Second | Third | Fourth |
  | :--- | :--- | :--- | :--- | :--- |
  | Assignment first | 1.158, block 23, A→R0 | 1.147, block 55, A→R1 | 1.173, block 86, A→R0 | **1.181**, block 115, A→R1 |
  | Mirrored first | 1.174, block 23, A→R1 | 1.172, block 55, B→R1 | 1.184, block 87, B→R0 | **1.191**, block 119, A→R0 |

  H-23's highest were 1.200 and 1.198.
- **Clause 3 holds in both arms**: the three reversals passed 40 of 64 in **19, 21 and 16** blocks from the assignment and **20, 15 and 14** from the mirrored, against the bound of 23 — H-23's 19, 19, 16 and 20, 15, 14; H-24's 31, none, 20 and 24, none, 24. The first mappings crossed at the 6th and the 4th block, as H-23's.
- **Clause 4 holds in both arms**: at every block's end the excitatory sum outside the four couplings lay within **0.99992 and 1.00004** of the image's from the assignment (0.99992 at the end) and **0.99994 and 1.00015** from the mirrored (1.00014 at the end), against the band's 0.75 and 1.25; H-24's lay within 0.987 and 1.043.
- `Drawn { learning: Scheduled { learned: [[true; 4]; 2], bounded: [true; 2], over: [None; 2], yes: true }, revised: [[true; 3]; 2], left: [None; 2], held: [true; 2], yes: true }`. **H-25 is yes**, with its scope: this task, these two arms, this schedule of three flips over 7 680 trials, H-23's configuration at 1 024 units, the targets the task's efference copy.

**The account's prediction, against the reading.** ADR-0138 predicted yes, because the drawn sources are the presented stimulus's set but for about 1.6 background units a trial. The reading bears out both halves: the drawing named the presented set whole in nearly every trial, and about 1.6 other units a trial beside it; and the learning, the reversals, the couplings, the value and the network read as H-23's did under the host's address.

**The drawn sources beside the host's** (`DRAWN_SOURCED_1024`, per block and presented stimulus, by group):

| Arm | Presented units drawn, of the host's | Left out, a trial | Others drawn, a trial: inhibitory, other stimulus, readout 0, readout 1, the four others | All others, a trial |
| :--- | ---: | ---: | :--- | ---: |
| Assignment first | 390 364 of 391 680, **99.66 %** | 0.171 | 0.40, 0.09, 0.57, 0.56, 0.01 | **1.63** |
| Mirrored first | 390 359 of 391 680, **99.66 %** | 0.172 | 0.41, 0.09, 0.58, 0.56, 0.01 | **1.63** |

- The first trial's drawing named no source: its window closed in the lead-in, 500 ticks before the trial, as read before the protocol, so the first reward reached no synapse. Of the 1 316 and 1 321 presented units left out over the run, 51 are that trial's; the rest are about one unit in six trials whose volley fell outside the window or did not fire.
- The others are the window's background, ADR-0135's 1.6 spikes a trial counted as units: over a mapping 1.56 to 1.68 a trial, about equally from each readout's set, a quarter of them inhibitory units, whose synapses the inhibitory baseline consolidates whatever the address. The other stimulus's units were drawn at 0.085 a trial.

**Where the consolidation went** (`DRAWN_WENT_1024`, by the network's oracle, per mapping, the weight raised and the weight lowered summed with their signs):

| Arm, mapping | Answer's pairs: moved each way | net | Other pairs: moved | net | Outside: moved | net |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Assignment, 1 | 14 131 311 | +1 963 991 | 5 437 035 | −714 225 | 158 662 | −1 166 |
| Assignment, 2 | 18 403 870 | +2 442 666 | 17 441 278 | −2 582 144 | 292 129 | +331 |
| Assignment, 3 | 18 642 817 | +2 500 849 | 16 018 921 | −2 666 083 | 327 654 | +6 804 |
| Assignment, 4 | 19 294 323 | +3 040 807 | 15 242 173 | −2 465 583 | 329 155 | −20 991 |
| Mirrored, 1 | 14 942 421 | +2 272 937 | 4 791 040 | −465 352 | 161 944 | +12 142 |
| Mirrored, 2 | 17 817 154 | +2 516 134 | 16 636 093 | −2 461 151 | 260 108 | −9 848 |
| Mirrored, 3 | 17 066 803 | +2 490 753 | 13 289 658 | −2 320 682 | 259 957 | +13 699 |
| Mirrored, 4 | 19 162 536 | +2 707 446 | 14 500 470 | −2 427 736 | 268 435 | +10 489 |

The reward's net in the answer's pairs was **13.3 to 15.8 per cent** of the weight it moved there, against H-24's 0.8 to 2.9, and the other pairs lost on net in every mapping — the punishments of a wrong selection, as under the host's address. Outside the pairs **1.1 to 1.8 per cent** of the answer's pairs' weight moved, netting within 21 000 of zero, against H-24's five to six times the answer's.

**The network outside the pairs, by cell** (`DRAWN_CELLS_1024`): at the run's end 11 822 and 11 325 of the 23 052 synapses outside the pairs stood off their image weight (H-24: 23 045 and 23 048), and only where the drawing reaches: a readout unit's synapses onto readout units, 8 855 and 8 428 of 15 456, and onto the inhibitory units, which all lie in the readout sets, 2 897 and 2 832 of 4 857; the four other units' onto them, 70 and 65. No synapse of a stimulus unit outside the pairs moved, nor any synapse onto a stimulus unit or one of the four others. The largest single move from the image's weight was 1 197 and 1 139 of the width's 32 767 (H-24: 30 775 and 28 856).

**The couplings' separation**: each stimulus's coupling onto its answer less its coupling onto the other readout, each a fraction of its image coupling, at each mapping's end — **0.152 to 0.231** here against H-23's 0.134 to 0.244 and H-24's −0.013 to 0.148.

**The selection** split by stimulus in every mapping of both arms, as H-23's did: over each mapping's last 128 trials each stimulus selected its answer in 49 to 68 of its trials, with no lean to either readout (`DRAWN_EARNED_1024`).

**The value and its troughs**: each stimulus's mean value over each mapping's last 128 trials stood within **0.120** of the reward of $2p - 1$, within H-23's quarter in every mapping and stimulus of both arms; after each flip it fell to **−0.63 to −0.84** of the reward, against H-23's −0.58 to −0.91.

**The strong punishments per flip**, both stimuli summed: 554, 579 and 547 from the assignment and 498, 451 and 466 from the mirrored, against H-23's 543, 567, 554 and 509, 437, 441.

**The population's rate by class** (`DRAWN_SPIKES_1024`, the mean over the run): the inhibitory units 2.22 and 2.26 Hz, the stimulus units 4.46 and 4.46, the readout units 1.74 and 1.75 — H-21's under the host's address to within 0.01 Hz, where H-24's rose by three to four per cent.

**The inhibitory sum's course**: as H-23's, rising to 1.011 and 1.010 of the settled image's and falling from there, **0.848 and 0.850** at the end, against H-23's 0.849 and 0.848.

**Beside H-23, run by run.** The two runs share the image, the trials' stimuli and the first reward's selection; the weights part from H-23's in the first block, where the first reward reached no synapse here and the host's pair there, and the selections from the third and the fourth block. From there the runs are two samples of one configuration, the correct count per block H-23's in 23 and 34 of the 120 blocks.

**The oracles held** at every one of the 15 360 trials: the drawn sources to the units the harness's critic oracle counted from the train, the composer's traces, weights and signal over the pairs, the network's traces, weights and stamps over all 26 240 excitatory synapses, and the engine's value, error, weights and window to the harness's critic. The gate holds the tables to one another block by block: each coupling the one before plus what the composer consolidated, the pairs' moves by the network's oracle the composer's, the outside's moves the change in the cells' sum, the cells' sum the outside sum, and every block's drawn sources within the spikes the window admitted.

### The step of the stopping rule reached

**Step 3**: *"Yes: the learning loop no longer takes the stimulus's identity from the host, and the configuration is named with the drawn address."* The learning configuration is **H-23's with the drawn address**: the excitatory synapses under the reward's gate with the signed gate set, the inhibitory ones under a baseline of their own and the inhibitory rule's target at the settled network's rate, the engine's critic with its window, and the reward's sources the units that window counted, its targets the channel the engine's selection chose. Per trial the host gives the reward's sign and the efference copy of the engine's own choice. **The next decision is an ADR choosing among**:
- the operating regime;
- another size;
- the rule held by the network reopened;
- a critic carried by a population;
- the second step the maintainers planned: a neural form of the target side, a competition between the readouts or a feedback that tags the chosen channel, with this round's run as the reference it is measured against;
- a task the engine has not been asked.

It is named and not taken. Steps 4 and 5 did not arise; step 6 is kept.

## Consequences

- Good: only the address's sources move from H-23's configuration, so a difference from H-23 is the drawing's, and a difference from H-24 is the address's: at this size and on this task the sources the engine draws from its own window carry what the host's label carried.
- Good: per trial the learning loop takes from the host the reward's sign and the efference copy of the engine's own choice, and no label of the stimulus.
- Good: the drawn sources are held at every trial to a second count of the train, and every synapse they reach to a second writing of the rule, so what the drawing added outside the pairs — the synapses of the background units the window admitted onto the selected readout — is read and not inferred.
- Good: this run is the reference the maintainers' second step, a neural form of the target side, is measured against.
- Neutral: the whole run is pinned, 120 blocks per arm, with the drawn sources per block beside H-24's tables.
- Bad: the drawing rests on the window's fit to the task: the first trial, whose window closed in the harness's lead-in, drew nothing, and a host that presents its next stimulus later than one shortest delay after a reward would give the drawing the background alone (ADR-0133, ADR-0139).
- Bad: one seed, one size, one schedule, as H-23's; the readout sets and the selection stay the host's, and the targets an efference copy (ADR-0138).

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `DRAWN_ARMS`, `DRAWN_DELIVERY`, `DRAWN_PREDICTED`, `Drawn`, `drawn`, `Sourced`, `beside_host`, `sources_hash`, `Watch::sourced`, `drawn_arm`; the two weekly tests and the gate's test named above.
- `runtime/cortex-runtime/tests/instrument/harness.rs`: `earned_run_delivered`, `Composer::drawn` (ADR-0139).
- Every pinned number of ADR-0065 to ADR-0137 unchanged, and the determinism pin; the image format 20.
