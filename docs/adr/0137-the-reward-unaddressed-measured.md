---
status: accepted
date: 2026-09-30
depends-on: ADR-0136
decision-makers: VirtualCortex maintainers
---

# ADR-0137: The reward unaddressed, measured — H-24 is no, on clause 1: with the dopamine term delivered to every synapse on H-23's configuration, nothing else changed and a network's oracle holding every excitatory synapse of the arena to the rule at every trial, the engine learned the first mapping in part and did not learn the reversals — 79, 78, 64 and 87 of each mapping's last 128 from the assignment and 89, 67, 64 and 83 from the mirrored, against the 80 asked and H-23's 115 to 124 — while every coupling stayed bounded, the highest 1.173 of its image's, and the network held, the excitatory sum outside the four couplings within 0.987 and 1.043 of the image's though every synapse outside them moved; the reward's net in the answer's pairs was 0.8 to 2.9 per cent of the weight it moved there, the other pairs moved as much, and from the second mapping both stimuli selected readout 1 more often than readout 0; the critic held each stimulus's expected reward throughout; H-24's stopping rule at step 5, and the next decision, an ADR on the eligibility's specificity under a global reward, named and not taken

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

### The order of the work, as the history holds it

1. **The protocol** (`dfea8dc` on `main`; `3fcee54` on the branch before the rebase that merged pull request #171, pushed at 08:11:18Z on 2026-09-30 and pull request #171 opened as a draft at 08:11:35Z, before any run): the harness's delivery, the network's oracle, the rules, the constants, the gate, the arms with empty pins, and this ADR's protocol.
2. **The calibration**, before any rewarded run, from the release build of `3fcee54` (`dfea8dc`): the workspace's tests passed in the debug profile and in the release profile (684 passed, 119 ignored) and on the MSRV (684 passed), and the check, Clippy, the documentation, the benchmarks' single pass and the format exited 0; **every one of the 77 whole-domain tests of the weekly job passed**, each in a process of its own and each exit 0 with one test reported — 67 eight at a time from 08:13:43Z to 09:20:23Z, H-23's two arms among them in 1 853 and 1 782 s under that load, and the ten of ADR-0097 and ADR-0102 one at a time to 09:22:09Z; the pull request's checks on `3fcee54` (`dfea8dc`) passed, the determinism pin on AArch64 among them, and the mutation gate found no mutant in a diff that changes no source.
3. **The arms**, once, side by side from 09:22:38Z. In each the calibration held: ADR-0077's settled engine and H-20's to H-23's images by their CRCs, and H-23's first block from H-23's image under the addressed delivery reproduced table by table, the network's oracle beside it reading nothing moved outside the pairs. Both ran their 7 680 trials with both oracles held to the record at every trial and stopped at the first empty table at 09:44:21Z, 1 303 s. The tables were written from those dumps (`5d9489e`; `5b316f8`), and a second run side by side from 09:47:44Z reproduced every table and passed, in 1 291 and 1 288 s (on the developer machine, a ratio and not admissible).

No constant, clause or rule moved after the first rewarded trial, and there was no second attempt: the second run is the pinned tables' reproduction.

### The readings

**The verdict** (`UNADDRESSED_1024`), by the rule committed first, over the pinned tables:

- **Clause 1 fails in both arms**: each mapping's last 128 trials, **79, 78, 64 and 87** correct from the assignment and **89, 67, 64 and 83** from the mirrored assignment, against the mark of 80 — the first three mappings unlearned in the first arm and the second and third in the second. H-23's were 120, 121, 120, 115 and 124, 116, 116, 123.
- **Clause 2 holds in both arms**: no coupling above 1.30 of its image's at any block's end. The highest per mapping, as a fraction of the image coupling:

  | Arm | First mapping | Second | Third | Fourth |
  | :--- | :--- | :--- | :--- | :--- |
  | Assignment first | 1.087, block 23, A→R0 | 1.077, block 55, A→R1 | 1.071, block 63, A→R1 | **1.173**, block 119, A→R1 |
  | Mirrored first | 1.117, block 22, A→R1 | 1.108, block 24, A→R1 | 1.080, block 56, B→R1 | **1.121**, block 119, B→R1 |

- **Clause 3 holds in both arms**: at every block's end the excitatory sum outside the four couplings lay within the band, **0.987 to 1.043** of the image's from the assignment (the lowest at block 22, the highest at block 72, 1.012 at the end) and **0.987 to 1.019** from the mirrored assignment (block 23 and block 61, 0.990 at the end), against the band's 0.75 and 1.25.
- `Unaddressed { learning: Scheduled { learned: [[false, false, false, true], [true, false, false, true]], bounded: [true; 2], over: [None; 2], yes: false }, left: [None; 2], held: [true; 2], yes: false }`. **H-24 is no, on clause 1**, with its scope: this task, these two arms, this schedule of three flips over 7 680 trials, H-23's configuration at 1 024 units.

**The account and ADR-0066, against the reading.** The literature's account predicted yes: with a critic taking the reward's expectation from it, a global reward learns, the eligibility trace assigning the credit. Two of its conditions held here. The critic held its expectation: each stimulus's mean value over each mapping's last 128 trials stood within 0.195 of the reward of $2p - 1$, within the quarter H-23's clause 4 allows in every mapping and stimulus of both arms. And the unsupervised part the account names did not move the network in sum: clause 3 held with the sum within five per cent of the image's. The learning still did not follow. ADR-0066's no was read under a configuration that no longer exists, with every coupling falling alike; this no is on H-23's configuration, and it is not a no of the same shape: the couplings did not fall, the first mapping reached 79 and 89 of 128, crossing 40 of 64 at the 11th and the 19th block against H-23's 6th and 4th, and the fourth came back over the mark, 87 and 83, while the second and third did not, 78 and 67, 64 and 64.

**Where the consolidation went** (`UNADDRESSED_WENT_1024`, by the network's oracle, per mapping, the weight raised and the weight lowered summed with their signs):

| Arm, mapping | Answer's pairs: moved each way | net | Other pairs: moved | net | Outside: moved | net |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| Assignment, 1 | 39 980 216 | +1 025 494 | 37 047 475 | −309 305 | 209 084 147 | −2 269 429 |
| Assignment, 2 | 51 658 511 | +716 297 | 53 846 281 | −1 190 677 | 287 695 749 | +2 505 431 |
| Assignment, 3 | 59 127 456 | +496 566 | 58 032 752 | −768 048 | 347 739 279 | +4 465 331 |
| Assignment, 4 | 56 660 129 | +1 223 781 | 56 317 425 | −657 075 | 327 048 874 | −2 358 930 |
| Mirrored, 1 | 39 629 622 | +1 164 756 | 36 693 531 | −295 367 | 204 477 814 | −2 441 766 |
| Mirrored, 2 | 51 882 988 | +917 732 | 54 288 659 | −1 002 199 | 293 025 936 | +3 740 430 |
| Mirrored, 3 | 52 630 151 | +459 785 | 52 146 511 | −781 815 | 287 370 404 | +44 392 |
| Mirrored, 4 | 55 184 392 | +1 083 196 | 54 286 903 | −724 885 | 311 720 123 | −3 196 347 |

The answer's pairs gained on net and the other pairs lost, in every mapping of both arms, so the reward's sign reached the pairs; but the net was **0.8 to 2.9 per cent** of the weight the consolidation moved there, and the other pairs moved as much. Outside the pairs five to six times as much weight moved as in the answer's pairs, netting within 1.3 per cent of it. The pair each trial selected — the one the addressed delivery would have reached — moved 0.75 to 0.99 of what H-23's did (`PUNISHED_MOVES_UNADDRESSED_1024`, `REWARDED_MOVES_UNADDRESSED_1024`): what the global delivery adds is the other three pairs and the network under the same signal.

**The couplings' separation**: each stimulus's coupling onto its answer less its coupling onto the other readout, each a fraction of its image coupling, at each mapping's end — **−0.013 to 0.148** here against H-23's 0.134 to 0.244.

**The selection leaned to one readout.** Over each mapping's last 128 trials, by stimulus, `[R0, R1, tie]` (`UNADDRESSED_EARNED_1024`):

| Arm | First mapping | Second | Third | Fourth |
| :--- | :--- | :--- | :--- | :--- |
| Assignment first | A [38, 18, 5], B [17, 41, 9] | A [11, 49, 4], B [29, 31, 4] | A [21, 37, 4], B [19, 43, 4] | A [0, 53, 1], B [34, 36, 4] |
| Mirrored first | A [10, 49, 2], B [40, 19, 8] | A [19, 36, 9], B [10, 48, 6] | A [14, 46, 2], B [18, 43, 5] | A [19, 25, 10], B [10, 64, 0] |

In the first mapping each stimulus selected its answer in most trials. From the second on, in every mapping of both arms, both stimuli selected readout 1 more often than readout 0, so that the stimulus whose answer was readout 1 read 0.65 to 0.98 correct and the other 0.27 to 0.46. H-23's selection split by stimulus in every mapping: 51 to 72 of each stimulus's trials on its answer. Readout 1 is where the image's couplings are the larger for both stimuli (6 698 611 and 6 815 470 against 6 249 552 and 6 584 205), and where H-15's consolidation everywhere drifted ([ADR-0083](0083-plasticity-everywhere-measured.md)).

**The network outside the pairs, by cell** (`UNADDRESSED_CELLS_1024`): every synapse outside the pairs moved from the image — by the 24th block at least 99.96 per cent of every cell had moved, and 23 045 and 23 048 of the 23 052 stood off their image weight at the run's end — while every cell's sum stayed near the image's. At the run's end, as fractions of the image's cell sums, from the assignment and from the mirrored assignment:
- the readout units' synapses: onto readout units, 15 456 synapses, **1.010 and 0.995**; onto inhibitory units, 4 857, **1.011 and 1.002**; onto stimulus units, 2 452, 1.046 and 0.937; onto the other four units, 83, 0.912 and 1.058;
- the stimulus units' synapses outside the pairs: onto stimulus units, 69, 0.918 and 0.871; onto the other units, 7, 0.628 and 1.028;
- the four other units' 128 synapses: 0.689 to 1.160.

The largest single move from the image's weight reached 30 775 and 28 856 of the width's 32 767, both from a readout unit onto a readout unit; a few synapses sat at a rail at a block's end — at most 10 at zero and 17 at the top from the assignment, 4 and 5 from the mirrored. The small cells drifted furthest: the variance ADR-0136 named is a random walk of each synapse, and a cell's sum averages it out as the cell grows.

**The population's rate by class** (`UNADDRESSED_SPIKES_1024`, the mean over the run): the inhibitory units 2.29 and 2.32 Hz, the stimulus units 4.60 and 4.54, the readout units 1.80 and 1.78, against H-21's 2.21 and 2.26, 4.46 and 4.46, 1.74 and 1.75 under the addressed delivery (`TARGET_SPIKES_1024`), rising by the run's end to 2.45, 4.63 and 1.87 from the assignment.

**The reversals** (`CROSSINGS_UNADDRESSED_1024`, the crossing block counted): from the assignment 11, then **31, none and 20** for the three reversals; from the mirrored 19, then **24, none and 24**, against H-23's 6, 19, 19, 16 and 4, 20, 15, 14. The third mapping never passed 40 of 64 in either arm.

**The value and its troughs**: the engine's value followed each stimulus's own accuracy, so the troughs after each flip were shallower than H-23's, −0.10 to −0.71 of the reward against −0.58 to −0.91: a stimulus that stayed near chance had little expectation to lose.

**The strong punishments per flip**, both stimuli summed: 697, 678 and 544 from the assignment and 700, 629 and 582 from the mirrored, against H-23's 543, 567, 554 and 509, 437, 441.

**The inhibitory sum's course**: as H-23's, rising to 1.009 and 1.010 of the settled image's in the first dozen blocks and falling from there, 0.872 and 0.867 at the end, against H-23's 0.849 and 0.848.

**The oracles held** at every one of the 15 360 trials: the composer's to the record's traces, weights and signal over the pairs, the network's to the record's traces, weights and stamps over all 26 240 excitatory synapses, and the engine's value, error, weights and window to the harness's critic. The gate holds the tables to one another block by block: each coupling the one before plus what the composer consolidated, the pairs' moves by the network's oracle the composer's, the outside's moves the change in the cells' sum, the cells' sum the outside sum.

### The evidence

- **The dispatch's scope.** The diff changes no file under `src/` — the shared harness and `tests/inhibition.rs`, the documents and the cost table — so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=exhaustive`**, as brief 057 asks: run [36700851135](https://github.com/DescentVTT/VirtualCortex/actions/runs/36700851135) at `7e806a4` (`9bc8562` on `main`), the readings' commit, whose tests are those of `5b316f8` (`5d9489e`), the pinned tables. The round's commits as `main` holds them, beside the branch's: `dfea8dc` (`3fcee54`) the protocol, before any run; `5d9489e` (`5b316f8`) the arms' tables and the gate's checks over them; `9bc8562` (`7e806a4`) this ADR's readings and the documents; `16c7ff7` (`1899745`) this section, the cost table and the brief's archive. The mutation sweep did not run: with no source changed it could make no mutant the tree's last sweep did not.
- **Every exhaustive job is green.** All seventy-nine `exhaustive` tests passed, each exit 0 with one test reported: the seventy-seven before this round reproduced their pinned numbers on the hosted runners, H-23's arms among them in 1 317 and 1 790 s, and H-24's two arms reproduced their tables there, in **1 809 s from the assignment and 1 907 s from the mirrored**.
- **The shards**, dealt by the table before this round, which did not know H-24's arms and costed each at 900 s:

  | Shard | Job | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 49.1 min | H-22 from the mirrored 1 803; H-20 from the assignment 1 550 | 13 | 5 393 | 2 889 s, 40 % |
  | 1 | 48.1 min | H-23 from the mirrored 1 790; H-19 from the assignment 1 280 | 14 | 5 575 | 2 820 s, 39 % |
  | 2 | 59.3 min | H-24 from the mirrored 1 907; H-21 from the mirrored 1 858 | 14 | 6 740 | 3 499 s, 49 % |
  | 3 | 50.5 min | H-21 from the assignment 1 810; H-18 from the assignment 1 148 | 12 | 5 859 | 2 968 s, 41 % |
  | 4 | 54.3 min | H-20 from the mirrored 1 940; H-24 from the assignment 1 809 | 13 | 6 306 | 3 193 s, 44 % |
  | 5 | 42.1 min | H-23 from the assignment 1 317; H-22 from the assignment 1 257 | 13 | 4 827 | 2 474 s, 34 % |

  The percentages are of the job's bound, 120 minutes; every shard ran its tests two at a time, at 1.87 to 1.98 of their summed seconds over the wall.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 36700851135`): 79 lines, 34 700 s, the seventy-seven earlier tests at 1.066 of the table before, so these runners were slower than ADR-0135's. ADR-0092's deal plans each of the six shards at 5 782 to 5 785 s summed, about 2 975 s of wall time at the run's ratio, **about 41 per cent of the bound**, inside the brief's 60; the 38 per cent planned above did not know the slower runners.
- **The pull request's gate** on `3fcee54` (`dfea8dc`), on `7e806a4` (`9bc8562`) and on `1899745` (`16c7ff7`) is green in every job, the determinism pin on AArch64 among them; the mutation gate on the changed lines found no mutant, the diff changing no source.

### The step of the stopping rule reached

**Step 5**: clause 3 held, so step 4 does not arise, and clause 1 failed — *"the address carried a credit assignment the eligibility does not. The next decision is an ADR on the eligibility's specificity under a global reward, with the readings of where the consolidation went as its need."* The readings that need names: the reward's net reached the answer's pairs at 0.8 to 2.9 per cent of the weight it moved there, the other pairs moved as much, and from the second mapping the selection leaned to the readout onto which both stimuli's couplings were the larger. **The next decision is an ADR on the eligibility's specificity under a global reward.** It is named and not taken. Step 6 is kept.

## Consequences

- Good: only the delivery moves from H-23's configuration, so the difference from H-23 is the address's: at this size and on this task the address carried the credit assignment the learning needed, while the critic and the network held without it.
- Good: every excitatory synapse the global reward reaches is held to a second writing of the rule at every trial, so clause 3 and the readings outside the pairs rest on an oracle and not only on the record.
- Good: the addressed delivery's run is the harness's as it was, bit for bit, and H-23's first block reproduces under it before any rewarded run.
- Good: where the reward's consolidation went is read per block, pair by pair and cell by cell, so the next decision starts from a measured need.
- Neutral: the whole run is pinned, 120 blocks per arm, as H-23's, with the network's cells per block beside it.
- Bad: the learning configuration H-23 named keeps the addressed delivery, the one label the learning loop still takes from the host.
- Bad: one seed, one size, one schedule, as H-23's; the readout sets and the selection stay the host's (ADR-0136).

## Confirmation

- `runtime/cortex-runtime/tests/instrument/harness.rs`: `earned_run_delivered`, `earned_run_valued`, `Composer::global`.
- `runtime/cortex-runtime/tests/inhibition.rs`: `UNADDRESSED_ARMS`, `UNADDRESSED_DELIVERY`, `UNADDRESSED_PREDICTED`, `BAND_QUARTERS`, `BAND_DIVISOR`, `SOURCES`, `outside_of`, `image_outside`, `within_band`, `left_band`, `Unaddressed`, `unaddressed`, `Place`, `place_of`, `Cell`, `Cells`, `cells_of`, `cell_sizes`, `cells_sum`, `Went`, `Wired`, `Wiring`, `wiring_of`, `replay_block`, `Network`, `Watch`, `delivered_run`, `h23_first_block`, `unaddressed_arm`; `UNADDRESSED_BLOCKS_1024` to `SUMS_AFTER_UNADDRESSED_1024`, `IMAGE_CELLS_1024`, `CELL_SIZES_1024` and the verdict `UNADDRESSED_1024`; the two weekly tests and the gate's test named above.
- Every pinned number of ADR-0065 to ADR-0135 unchanged, and the determinism pin; the image format 20.
