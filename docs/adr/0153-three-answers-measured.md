---
status: proposed
date: 2026-10-09
depends-on: ADR-0152
decision-makers: VirtualCortex maintainers
---

# ADR-0153: Three answers, measured — H-29 is no, on clause 1: on H-25's configuration, stimuli, schedule and arms with three readouts dealt by ADR-0151's rule, the 104th of 92 400 deals in an order committed before any coupling was read, the first mapping was learned as between two, 118 and 119 of its last 128, and seven of the eight mappings were learned by each stimulus; the mirrored arm's second was not, 85 of its last 128 correct but stimulus A in 30 of its 64 presentations; no coupling past 1.30, the highest 1.242 and 1.261, the network outside the six within a part in ten thousand of the image's, and each stimulus's mean value within 0.21 of the reward of 2p − 1; a reversal took 21 to 31 of a mapping's 32 blocks against H-25's 14 to 21, the old answer taking 62 to 90 per cent of the wrong selections and leading for 5 to 24 blocks while the value sat down to −0.95 of the reward and a punishment delivered a third to two thirds of it; nothing selected in 10 to 16 per cent of the trials; H-29's stopping rule at step 5, and the next decision, an ADR choosing between the selection, an exploration among its candidates and the readout's resolution, named and not taken

## Context and Problem Statement

[ADR-0151](0151-three-answers.md) wrote H-29 before any run: on H-25's configuration, stimuli, schedule and arms, with three readouts and an answer for each stimulus that moves to the next readout at a flip, does the engine learn every mapping by each stimulus, with every coupling bounded, the network outside the couplings held and the critic holding each stimulus's expected reward. [ADR-0152](0152-three-answers-built.md) built the selection and the mapping. This ADR is the measurement: the readouts' deal, H-29's protocol, its calibration, its one run and its reading.

Brief 062's empowerment names the choices the measurement makes: the order the deals are enumerated in, as long as it is committed before any coupling is read; how the harness's oracles and tallies read six couplings, and which oracle is held at every trial; how each reading is computed and pinned; and how the arms are dealt into tests.

What was read first (principle 2), on 2026-10-08:

1. **The prior's rule for a local synapse** (`crates/cortex-connectome/src/prior.rs`, `Prior::target` and the walk's `next`): the place is drawn from `1..=2W` with the same chance for each, and the delay from the local band whatever the place. So no place within a window is preferred by the rule.
2. **The two oracles of H-25's run**: the composer (`tests/instrument/harness.rs`, `Composer`, `advance`), whose every table is `[stimulus][readout]` over two readouts, and the network's oracle (`tests/inhibition.rs`, `Network`), which replays every excitatory synapse and reads the address from the executor. The second names a synapse's place by a rule that read two readouts (`place_of`) and nothing else of it counts readouts.
3. **The engine's critic's oracle** in `earned_run_held`: the counts, the value, the error, the weights and the window's opening, which read no readout.
4. **The blocks of every run before** (`Block`): a tuple of twelve fields shaped for two readouts, pinned in every arm since ADR-0066.
5. **The geometry's seam**: at 1 024 units the pattern is 51 periods and four units are in no set (F-65, below).

## Decision Drivers

- ADR-0151's H-29, its clauses, constants and stopping rule, and brief 062's standing directives: one change from H-25, the number of answers; the deal the first that passes in an order committed first; H-29's clauses and constants written before the first rewarded run; no second attempt.
- **Every pinned number of an earlier round holds.** The two-readout harness, its blocks and its oracles are not rewritten for the measurement.
- **An oracle at every trial**, as every learning run since ADR-0079 has had.
- Literature is a prior, not a verdict: ADR-0151 cites the account that predicts the learning and names where the engine's premise differs.
- The weekly job's budget: twelve shards, no shard past 60 per cent of its bound under the regenerated deal.
- Latest ≠ Newest (§2.1): nothing is adopted.

## Considered Options

1. **The order of the deals**: (a) the readouts numbered by their inhibitory place and the deals in lexicographic order of the places' numbers; (b) an order that prefers deals whose readouts are spread over both sides of each stimulus; (c) a seeded shuffle.
2. **The run's harness**: (a) a run of its own for a task of three readouts, beside the two-readout harness, which is not touched but for two entry points of the network's oracle; (b) the harness's run, blocks and oracles made generic over the number of readouts.
3. **The oracle held at every trial**: (a) the network's oracle, with the six pairs named among its synapses, and the critic's; (b) the composer written again for three readouts, beside it.
4. **The frozen block**: (a) ADR-0077's frozen run with the three readouts in the two's place, read by the sight's rule alone; (b) with the sign's rule too.
5. **The arms' tests**: (a) two weekly tests, each with its own calibration; (b) a third weekly test for the calibration.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).** It is written in the order the round ran, and the first section was committed before any coupling was read.

- **The order** (option 1(a)) is in the next section, with its reason. A spread-first order (option 1(b)) would be a preference with no rule of the prior behind it; a shuffle (option 1(c)) would add a seed to choose.
- **A run of its own** (option 2(a)): `answered_run`, about 330 lines, with tables shaped for three readouts (`AnsweredBlock`). A generic harness (option 2(b)) would re-type every pinned table of thirty rounds for one round's use; the second step of ADR-0151 can weigh it again with two rounds' shapes in hand.
- **The network's oracle** (option 3(a)): it replays all 26 240 excitatory synapses of the arena, the six couplings' among them, and holds each one's trace, weight and stamp to the record at every trial. A second composer (option 3(b)) would hold the same synapses a second time and add the split of a trace's terms by what paired them, which no clause or reading of H-29 asks for.
- **The frozen block** (option 4(a)) is read by the sight's rule, which is what ADR-0151's stopping rule names. The sign's rule (option 4(b)) is the composer's.
- **Two weekly tests** (option 5(a)): each arm holds its own calibration, as every arm since H-16 does, so that it can run alone in its shard.

### The readouts' deals and their order (written before any coupling was read)

ADR-0151's rule: each readout is four of the fourteen places of a period that lie within both stimuli's windows, one of them a place the prior makes inhibitory, and the deal taken is the first, in an order written before any coupling is read, whose six stimulus–readout couplings on the settled image are equal within ten per cent with none zero.

**The places**, held to the rule by the gate (`the_clauses_of_h_29_the_deals_in_their_order_and_the_readings_rules`, `runtime/cortex-runtime/tests/inhibition.rs`):
- the fourteen within both windows are 3 to 8 and 12 to 19 (`SHARED_PLACES`);
- three of them are inhibitory, 4, 14 and 19 (`INHIBITORY_PLACES`), so each readout holds exactly one;
- eleven are free: 3, 5, 6, 7, 8, 12, 13, 15, 16, 17 and 18 (`FREE_PLACES`). Each readout takes three, nine in all, and two are left in no set.

**The deals.** The readouts are numbered by their inhibitory place: readout 0 holds place 4, readout 1 place 14 and readout 2 place 19. Two deals that differ by the readouts' names alone are then one deal. A deal is readout 0's three free places, then readout 1's three of the eight left, then readout 2's three of the five left: $165 \times 56 \times 10 = 92\,400$ deals (`DEALS`).

**The order** (`deals`). Each readout's three free places are written as an ascending triple. The deals are in lexicographic order of the three triples, readout 0's outermost, then readout 1's, then readout 2's, and triples compare by their first place, then their second, then their third.
- The first deal: readout 0 is places 3, 4, 5 and 6; readout 1 is 7, 8, 12 and 14; readout 2 is 13, 15, 16 and 19.
- The second moves readout 2's last place: 13, 15, 17 and 19.
- The last: readout 0 is 4, 16, 17 and 18; readout 1 is 12, 13, 14 and 15; readout 2 is 6, 7, 8 and 19.

**Why this order.** It is the order of the places' numbers and reads nothing of the network. By the prior's rule a local synapse lands on each of the sixteen places of its source's window with the same chance, and its delay is drawn from the local band whatever the distance, so no place within both windows is nearer a stimulus than another in what the rule gives it. An order that preferred spread places to neighbouring ones would be a choice with no rule behind it.

**The rule's reading** (`dealt`): the largest of the six couplings at most eleven tenths of the smallest, and the smallest above zero — ADR-0065's rule for the rotation (`balanced`), over six couplings where it read four. A deal's six couplings are sums of each stimulus's couplings into single places (`place_couplings`, `deal_couplings`), so the whole order is read from twenty-eight numbers of the image.

**What the gate holds before any coupling is read**: the three lists of places against the rule that derives them; the deals' number; the first, the second, the eleventh, the 561st and the last deal as the order states them; every one of the 92 400 a deal of the rule's — four places a readout, of the fourteen, one inhibitory place and the readout's own, no place twice — and after the one before it in the order; a deal's sets at 1 024 units, each readout 204 units, 51 of them inhibitory; the rule at its edges; the first deal that passes over tables written by hand; and, on the instrument's network before any settling, three deals' couplings from the couplings by place against the couplings summed over their sets — the test reads that the two are equal and nothing of which deal passes.

**One thing read of the geometry and not in ADR-0151 (F-65).** ADR-0151 wrote that each stimulus unit has four places of each readout in its window. One unit does not: unit 0, stimulus A's first, lies at the ring's seam, where the four units past the last whole period take the place of places 12 to 15 of a period before. It sees a readout's places among 3 to 8 and 16 to 19 only. The other 101 stimulus units see four of each readout. The two-answer geometry has the same seam (`the_geometry_holds_against_the_census_at_both_sizes` reads between four and eight units of a readout in a stimulus unit's window). It is one unit of 51, and the rule reads the couplings the image holds, seam included.

### The deal taken

Read once the order above was committed and pushed (commit `04d8ce0`): each stimulus's coupling into each of the fourteen places on H-25's image, the settled image the arms decode (`PLACE_COUPLINGS_1024`), in the places' order 3 to 8 and 12 to 19.

| Stimulus | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 |
| :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| A | 703 639 | 999 283 | 895 229 | 773 142 | 678 323 | 686 862 | 685 329 | 678 891 | 1 223 561 | 855 375 | 796 796 | 695 659 | 826 558 | 798 661 |
| B | 824 421 | 900 220 | 835 204 | 874 475 | 631 173 | 829 251 | 865 793 | 775 030 | 787 470 | 836 992 | 723 315 | 705 115 | 846 092 | 830 072 |

**The first deal that passes is the 104th** (`DEAL_INDEX_1024` 103, `DEAL_1024`): the 103 before it fail the rule, and 10 598 of the 92 400 pass it in all, about one in nine.

| Readout | Places | From A | From B | Synapses from A | Synapses from B |
| :--- | :--- | ---: | ---: | ---: | ---: |
| 0 | 3, 4, 5, 6 | 3 371 293 | 3 434 320 | 400 | 416 |
| 1 | 7, 12, 14, 18 | 3 413 771 | 3 130 528 | 404 | 367 |
| 2 | 8, 15, 16, 19 | 3 137 694 | 3 219 630 | 384 | 391 |

- The largest coupling is 1.097 of the smallest (`DEALT_COUPLINGS_1024`).
- The synapse counts are the prior's census of the same six (`DEALT_SYNAPSES_1024`), by the prior's walk and by the arena the walk fills: 367 to 416, where ADR-0151's arithmetic expected about 387 and the two-answer geometry counts 775 to 809.
- Six places of a period are in no set: 1, 2, 9, 10, 13 and 17.
- Readout 0 is four neighbouring places. That is what the order gives first, and the rule does not ask otherwise.

The deal is not chosen for what a frozen block or a run reads of it. The gate holds the deal to the rule over the pinned table, with every deal before it failing; each arm's calibration holds the table to the image.

### H-29's protocol (written before the first rewarded run)

**The arms** (`ANSWERED_ARMS`, `answered_arm`): H-25's two, each its own weekly test, `three_answers_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`.
- The image is H-25's, built link by link as H-25's and H-28's arms build it and asserted by its CRC (`WINDOWED_IMAGE_CRC_1024`).
- The task is H-25's with three readouts: ADR-0065's two stimuli with ADR-0076's cancel; the window, the trial, the seed and the reward's magnitude H-25's; the answer's feedback, the drawn delivery, no critic of the task's and no hold.
- The answers start at (0, 1) or (1, 0) and move on at H-20's three flips by `Task::flip`: (0, 1), (1, 2), (2, 0), (0, 1) from the assignment and (1, 0), (2, 1), (0, 2), (1, 0) from the mirrored assignment. The same trials present the same stimuli as in H-25, since the stimulus is the seed's draw.

**What is held at every trial** (`answered_run`):
- the task's answers to a hand rule (`answers_at`), the selection to a hand rule (`largest_alone`), and `correct` to the two;
- the engine's value, the error the modulator received, every unit's value weight and the window's opening to the critic's oracle (`value_step`), as H-25's run holds them;
- the address: its sources exactly the units the critic's oracle counted, its targets the selected readout's units, none where nothing was selected;
- **every excitatory synapse of the arena** — its trace, its weight and its block's stamp — to the network's oracle ([ADR-0137](0137-the-reward-unaddressed-measured.md)), replayed from the train under the address and the signal the last trial left.

**Which oracle is held, and which is not** (brief 062's empowerment). H-25's run held two: the composer, which replays the pair rule over the four stimulus–readout couplings' synapses and splits each term by what paired it, and the network's oracle, which replays every excitatory synapse. The composer is written for two readouts in every table it returns. H-29's run holds the network's oracle alone: the six couplings' synapses are among the 26 240 it replays, each named by its pair (`answered_place`), so the weights and the traces the composer would hold are held, and what each trial consolidated into each pair is read from it (`by_pair`). What is not read is the composer's split of a trace's terms into the volley's and the background's, and its census of the volley's ticks; no clause and no reading of H-29 asks for either.

**The tables** (`AnsweredBlock`, one row a block): each stimulus's presentations and correct trials; its selections by readout and its ties; the readouts' spikes after the volley and before the injection; the sight's count; the volley; the engine's values and the errors delivered; the arena's sums; the signal; the six couplings; where the consolidation went, by the answer's pairs, the other four and outside, and by pair; and the drawn sources. Every trial's reading is hashed.

**The clauses**, as integer rules (`Answered`, `answered`), with the constants ADR-0151 wrote:
1. **the learning holds** (`learned_by_each`): per mapping, over its last 128 trials, at least 80 correct (`REWARDED_MIN`), and each stimulus correct in more than half of its presentations, `2 × correct > presentations` (`EACH_TIMES`);
2. **the couplings stay bounded** (`answered_over`): a run of 120 blocks with none of the six above 1.30 of its image's at any block's end, `coupling × 100 > image × 130` (`BOUND_PER_CENT`);
3. **the network holds** (`answered_left`): at every block's end the excitatory sum less the six couplings within 0.75 and 1.25 of the image's, `3 × image ≤ 4 × outside ≤ 5 × image` (`BAND_QUARTERS`);
4. **the critic holds the expected reward** (H-23's `holds_expected`): per mapping and stimulus over the last 128 trials, `|V − (2c − n) r| × 4 ≤ r n`, with $n$ the stimulus's trials, $c$ its correct ones and $V$ its values summed.

**Yes** when all four hold in both arms. The step of the stopping rule a verdict reaches is a rule too (`answered_step`): 3 for a yes; 4 for a no on clause 3; otherwise 5 for a no on clause 1; otherwise 6 for a no on clause 2; otherwise 7.

**No prediction for the verdict** (`ANSWERED_PREDICTED`).

**The predicted readings**, as rules, never asserted:
- **(a) a later start** (`later_start`): the first mapping passes 40 of 64 in a later block than H-25's sixth and fourth; a mapping that never passes it counts as later.
- **(b) a value below zero first** (`below_zero_first`): per stimulus, some block before the first mapping's crossing block holds a mean value below zero. The troughs are read beside it, against minus a third of the reward.
- **(c) elimination after a flip** (`eliminated`): per flip and stimulus, the old answer is selected more often than the new answer and than the third readout in the block after the flip (`old_held` at least one), and the stimulus's coupling into the third readout stands below the image's at some block's end of the mapping.

**The readings, no clause** (`AnsweredRead`):
- the block each mapping passes 40 of 64 in, beside H-25's (`answered_crossings`);
- per mapping, each stimulus's selections by the readouts' roles to it — its answer, the readout before it, which was its answer until the flip, and the readout after it, the third — and its ties (`by_role`);
- after each flip, the stimulus that moves onto the readout the other is leaving (`onto_the_other_s`: A from the assignment, B from the mirrored assignment) beside the one that moves onto the free readout: the block in which each selects its new answer in more than half of its presentations (`each_crossed`), its first new selection (`first_new_answered`) and how long its old answer held (`old_held`);
- the six couplings' courses by role — lowest, highest and last of each mapping (`course_by_role`) — and the highest of the six (`answered_highest`);
- the value beside $2p - 1$, with its trough in each mapping (`answered_troughs`);
- where the consolidation went, by mapping (`went_by_mapping`), and the inhibitory sum's and the outside sum's courses (`low_high_last`), beside H-25's;
- on the frozen block, each readout's count before and after the volley and the selections by readout, beside the two-answer geometry's frozen block.

**The gate** (`the_clauses_of_h_29_the_deals_in_their_order_and_the_readings_rules`), one test: the deals' order as above; the mapping's hand rule over the schedule, with the two arms holding all six ordered pairs of different answers; the deal taken held to the rule over the pinned table; the four clauses at their edges, the verdict and its step over tables written by hand; every reading's rule over tables written by hand; and eight trials of the task with three readouts on the instrument's network under H-25's delivery, both oracles held at every trial, beside four with the reward withheld on the same network frozen.

### The calibration (H-29's stopping rule, step 2)

**Before the protocol was committed**, one arm was run to the end of its calibration, where it stops with no pin for the frozen block. In order:
- the settled engine held to [ADR-0077](0077-the-background-side.md) step by step, its images, H-20's to H-23's images by their CRCs and a frozen block of the two-readout task held to ADR-0077's frozen run;
- **H-25's first block under two readouts reproduced** table by table (`drawn_first_block`), through `Task::flip`'s schedule and the generic readout;
- **the deal**: the couplings by place read from the image, the first deal that passes the 104th, its couplings and its census as above;
- **the frozen block with three readouts** (`answered_frozen`, `FROZEN_ANSWERED_1024`), read by the sight's rule.

| The frozen block, 64 trials | Three readouts of 204 | Two readouts of 459 (ADR-0077) |
| :--- | :--- | :--- |
| Trials in which the window after the volley held more readout spikes than the window before the injection | **62** | 62 |
| Readout spikes after stimulus A's volleys, by readout | 189, 170, 149 | 330, 323 |
| Readout spikes after stimulus B's volleys, by readout | 134, 171, 150 | 315, 314 |
| Readout spikes before the injections, by readout | 109, 115, 114 | 258, 256 |

**The sight passes**: 62 of 64, at least 56 asked. The round goes on.

Read beside it, no gate:
- **What the smaller readouts cost.** A readout fires 4.4 to 5.7 spikes a trial after the volley of the stimulus presented, where a two-answer readout fired 9.5 to 10.5, and 1.7 to 1.8 a trial before the injection, where it fired 4.0. Above its background that is 2.6 to 3.9 spikes a trial where it was 5.5 to 6.5: about half, and the background four ninths, as ADR-0151's arithmetic gave. The three readouts' mean counts after a volley lie within about a spike of one another, so a selection is made on differences of single spikes.
- **The frozen selections are not at chance.** With nothing learned, stimulus A selected readout 0 in 16 of its 34 trials, readout 1 in 7 and readout 2 in 4, with 7 ties; stimulus B selected readout 1 in 15 of its 30, the others in 5 each, with 5 ties. Under the assignment's answers that is 31 correct of 64, where a choice among three at chance gives 21.
- **The couplings do not order the counts.** B's coupling into readout 0 is the largest of the six and readout 0 fired least after B's volleys; B's coupling into readout 1 is the smallest and readout 1 fired most. The rule balances summed weights, and what a readout fires is also its own inhibitory place, its units' delays and the synapses among the readouts.
- **Ties**: 12 trials of 64 selected nothing.

The frozen block was read before any rewarded run and is pinned with the protocol. The arms' starting assignments are H-25's and are not fitted to it: the assignment starts where the frozen network already leans, the mirrored one against it, and from the first flip on every mapping asks for something the frozen network does not give.

**Every other whole-domain test of the tree** was then run, before the arms: all 87 passed. The order of the work, below, has the times.

### The weekly tests and their cost

- **Two weekly tests**: `three_answers_from_the_{assignment, mirrored_assignment}_at_1024_units_exhaustive`.
- **Each is H-25's arm** without the composer and its shadows, with the calibration above in front of it. On the developer machine each took 1 217 s side by side with the other, its calibration included (a ratio, not admissible). H-25's arms are 1 817 and 1 837 s in the table.
- **The dispatch's scope**: ADR-0152 changed files under `src/`, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly is dispatched at `scope=both`.

### The order of the work, as the history holds it

Times are UTC on 2026-10-08. The hashes are the branch's, in pull request [DescentVTT/VirtualCortex#188](https://github.com/DescentVTT/VirtualCortex/pull/188) ([ADR-0150](0150-a-round-waits-for-what-it-checks.md)).

1. **The build** (`70d04a3`, 12:52:00): ADR-0152, with its tests.
2. **The order before any coupling** (`04d8ce0`, 13:00:15, pushed with the build): the places, the 92 400 deals in their order, the rule's reading and the gate's test over them. Nothing of the network was read by it.
3. **The couplings by place, read once**, on H-25's image, by a probe that is not kept: the twenty-eight numbers above, and the first deal that passes, the 104th.
4. **The rules before their result.** The run, the clauses, the step, the readings' rules, the arms and the gate's test were written with every table of an arm empty. One arm was then run to the end of its calibration, 13:14 to 13:18, where it stopped at the frozen block's empty pin: H-25's first block reproduced under two readouts, the deal held, and the sight read 62.
5. **The protocol committed and pushed** (`761f53f`, 13:22:37, pushed at 13:22:43), with the frozen block pinned, before any rewarded trial of an arm. The pull request was opened as a draft at 15:07:26 on `98360b5`, the build's documents; its gate passed there in every job, the mutation gate on the changed lines with 20 mutants, 16 caught and 4 unviable, none missed.
6. **The calibration's last part.** **Every one of the 87 whole-domain tests of the weekly job before this round passed**, 13:22:54 to 16:07:18, from a release build of the protocol's sources: the ten of `active`, four of `assembly`, one of `everywhere`, twenty-five of `inhibition`, twenty-eight of `instrument`, eleven of `learning`, seven of `reference` and `cortex-core`'s one. Every two-answer run goes through `Task::flip` and the generic readout, H-25's two arms and H-28's among them.
7. **The arms**, once, side by side from 16:08:16 to 16:28:48, from a release build of the tree at `98360b5`, whose Rust sources are `761f53f`'s. In each the calibration held, the frozen block the pinned one. Each ran its 7 680 trials with both oracles held at every trial and stopped at the first empty table. The tables were written from those dumps (`d0f60bd`), with the gate's checks over them and the verdict's constants, which are the rule's own output.

No constant, clause or rule moved after the first rewarded trial of an arm, and there was no second attempt. The arms were not run a second time on the developer machine: the dispatch is their reproduction (ADR-0150).

### The readings

**H-29 is no, on clause 1.** In the mirrored arm's second mapping one stimulus did not select its answer in more than half of its presentations. Every other mapping of both arms was learned by each stimulus, and clauses 2, 3 and 4 held in both arms. ADR-0151 wrote no prediction for the verdict.

#### Clause 1, the learning

Each mapping's last 128 trials: the correct ones, and each stimulus's correct ones of its presentations. H-25's count for the same trials is beside it.

| Arm | Mapping | Answers of A, B | Correct of 128 | A | B | Learned | H-25 |
| :--- | :--- | :--- | ---: | ---: | ---: | :--- | ---: |
| assignment | first | 0, 1 | 118 | 59 of 61 | 59 of 67 | yes | 122 |
| | second | 1, 2 | 87 | 45 of 64 | 42 of 64 | yes | 124 |
| | third | 2, 0 | 82 | 37 of 62 | 45 of 66 | yes | 119 |
| | fourth | 0, 1 | 109 | 54 of 54 | 55 of 74 | yes | 116 |
| mirrored | first | 1, 0 | 119 | 60 of 61 | 59 of 67 | yes | 124 |
| | second | 2, 1 | 85 | **30 of 64** | 55 of 64 | **no** | 118 |
| | third | 0, 2 | 99 | 58 of 62 | 41 of 66 | yes | 123 |
| | fourth | 1, 0 | 114 | 54 of 54 | 60 of 74 | yes | 117 |

- **The first mapping is learned as between two**: 118 and 119 of 128, where H-25 read 122 and 124.
- **The mapping that fails passes the count.** 85 of 128 is past the 80 asked. It fails the part ADR-0151 added to clause 1, each stimulus in more than half of its presentations: A was right in 30 of 64, three short. That is the case the part was written for, one stimulus answered and the other not.
- **Two mappings pass narrowly**: the assignment's third at 82 of 128 with A at 37 of 62, and its second at 87.
- **Every mapping after a flip was still improving when it ended.** The correct trials of a mapping's last two blocks were 44 and 43, 43 and 39, 53 and 56 from the assignment, and 42 and 43, 45 and 54, 56 and 58 from the mirrored assignment.

#### Clause 2, the couplings

No coupling passed 1.30 of its image's. The highest of each mapping:

| Arm | First | Second | Third | Fourth |
| :--- | ---: | ---: | ---: | ---: |
| assignment | 1.242 | 1.205 | 1.147 | 1.227 |
| mirrored | 1.261 | 1.223 | 1.202 | 1.233 |

Each is one of stimulus A's: its coupling into its answer's readout at or near a mapping's end, or into its old answer's readout in the first block after a flip. H-25's highest was 1.191. A coupling here is half the synapses, and the same learning carries it further from its image.

#### Clause 3, the network outside the couplings

The excitatory sum outside the six couplings stayed between 0.9999 and 1.0000 of the image's at every block's end in both arms. By the network's oracle at most 9 thousand of weight a mapping moved there on net, where the pairs moved millions.

#### Clause 4, the critic

Each stimulus's mean value over a mapping's last 128 trials, beside $2p - 1$ of the reward, in units of the reward:

| Arm | Mapping | A: $p$ | A: value | A: $2p - 1$ | B: $p$ | B: value | B: $2p - 1$ |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| assignment | first | 0.967 | 0.946 | 0.934 | 0.881 | 0.790 | 0.761 |
| | second | 0.703 | 0.433 | 0.406 | 0.656 | 0.133 | 0.312 |
| | third | 0.597 | 0.114 | 0.194 | 0.682 | 0.251 | 0.364 |
| | fourth | 1.000 | 0.959 | 1.000 | 0.743 | 0.308 | 0.486 |
| mirrored | first | 0.984 | 0.970 | 0.967 | 0.881 | 0.729 | 0.761 |
| | second | 0.469 | −0.233 | −0.062 | 0.859 | 0.707 | 0.719 |
| | third | 0.935 | 0.902 | 0.871 | 0.621 | 0.037 | 0.242 |
| | fourth | 1.000 | 0.872 | 1.000 | 0.811 | 0.540 | 0.622 |

All sixteen lie within a quarter of the reward; the widest is 0.206. Where the accuracy was still rising the value is below $2p - 1$, by up to a fifth of the reward: the value follows the accuracy from below.

#### The predicted readings

- **(a) A later start: held from the mirrored assignment and not from the assignment.** The first mapping passed 40 of 64 in the fifth block from the assignment, where H-25's passed in the sixth, and in the seventh from the mirrored assignment, where H-25's passed in the fourth. ADR-0151 reasoned from a chance of a third. The frozen block reads why one arm did not start there: with nothing learned the network already gives the assignment's answers in 31 trials of 64. The first block was 32 correct from the assignment and 12 from the mirrored assignment.
- **(b) A value below zero first: held**, for both stimuli in both arms. Before the first mapping's crossing the value's lowest block mean was −0.12 and −0.31 of the reward from the assignment and −0.31 and −0.40 from the mirrored assignment, where minus a third was predicted.
- **(c) Elimination after a flip: held at all twelve**, each flip and stimulus of both arms. The old answer led the new answer and the third readout for 5 to 24 blocks from the flip, and the stimulus's coupling into the third readout fell to between 0.893 and 0.972 of the image's.

#### The reversals

The blocks each mapping took to pass 40 of 64, beside H-25's:

| Arm | First | Second | Third | Fourth |
| :--- | ---: | ---: | ---: | ---: |
| assignment | 5 | 31 | 31 | 21 |
| H-25's | 6 | 19 | 21 | 16 |
| mirrored | 7 | 30 | 23 | 23 |
| H-25's | 4 | 20 | 15 | 14 |

**A reversal took 21 to 31 of a mapping's 32 blocks, 1.3 to 1.6 times H-25's.** That is where the schedule runs out: the mapping that failed passed 40 of 64 in its 30th block.

#### Where the selections went

Each stimulus's selections over a mapping, in per cent of its trials, by the readouts' roles to it: its answer; the readout before it, which was its answer until the flip; the readout after it, the third; and none.

| Arm | Mapping | A: answer | A: old | A: third | A: none | B: answer | B: old | B: third | B: none |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| assignment | first | 77.5 | 5.1 | 7.9 | 9.6 | 75.0 | 7.9 | 6.4 | 10.7 |
| | second | 22.4 | 60.1 | 6.5 | 11.0 | 24.3 | 53.1 | 8.7 | 13.9 |
| | third | 20.5 | 46.3 | 17.4 | 15.8 | 27.4 | 40.2 | 16.3 | 16.1 |
| | fourth | 61.6 | 18.8 | 8.4 | 11.2 | 30.7 | 35.9 | 18.7 | 14.7 |
| mirrored | first | 79.3 | 8.4 | 5.6 | 6.7 | 63.1 | 10.2 | 13.7 | 13.0 |
| | second | 15.1 | 61.2 | 11.2 | 12.4 | 48.4 | 33.7 | 7.1 | 10.7 |
| | third | 58.5 | 18.6 | 11.3 | 11.5 | 25.9 | 38.8 | 19.2 | 16.1 |
| | fourth | 43.0 | 39.2 | 8.1 | 9.7 | 36.2 | 35.5 | 13.4 | 15.0 |

In the first mapping there is no old answer, and the second and third columns are the two readouts that are not the answer.

- **The wrong selections went to the old answer**: 62 to 90 per cent of a stimulus's wrong selections after a flip, the third readout the rest.
- **The third readout was selected in 6 to 19 per cent of the trials.** It did not take the old answer's place: the selections left the old answer for the new one as its pairs rose.
- **Nothing was selected in 10 to 16 per cent of a mapping's trials**, where H-25's two readouts tied in 4.4 to 7.0.

#### After a flip, by stimulus

The block from the flip in which each stimulus first selected its new answer in more than half of its presentations:

| Arm | Flip | The stimulus that moves onto the other's old answer | Block | The stimulus that moves onto the free readout | Block |
| :--- | :--- | :--- | ---: | :--- | ---: |
| assignment | first | A, onto readout 1 | 28 | B, onto readout 2 | 25 |
| | second | A, onto readout 2 | 30 | B, onto readout 0 | 29 |
| | third | A, onto readout 0 | 14 | B, onto readout 1 | 25 |
| mirrored | first | B, onto readout 1 | 18 | A, onto readout 2 | never |
| | second | B, onto readout 2 | 30 | A, onto readout 0 | 14 |
| | third | B, onto readout 0 | 23 | A, onto readout 1 | 20 |

- **The role does not order them.** Onto the other's old answer took 14 to 30 blocks; onto the free readout took 14 to 29, and once more than the mapping held.
- **The readout does.** Stimulus A moved onto readout 0 in 14 blocks both times, and onto readout 2 in 30 and never. The frozen block read the same order before anything was learned: A selected readout 0 in 16 of 34 trials and readout 2 in 4.
- **The mapping that failed is the one that asks A for readout 2 straight after A's first answer.** The assignment's third mapping asks the same of A after one reversal and passed at 37 of 62.
- Each stimulus first selected its new answer within 4 to 133 trials of the flip.

#### The couplings at each mapping's end

As fractions of the image's, into readouts 0, 1 and 2, the answer's in bold:

| Arm | Mapping | From A | From B |
| :--- | :--- | :--- | :--- |
| assignment | first | **1.241**, 0.959, 0.969 | 0.972, **1.213**, 0.968 |
| | second | 0.986, **1.167**, 0.960 | 0.940, 0.972, **1.157** |
| | third | 0.951, 0.965, **1.119** | **1.120**, 0.915, 0.975 |
| | fourth | **1.227**, 0.919, 0.994 | 0.960, **1.150**, 0.893 |
| mirrored | first | 0.967, **1.261**, 0.965 | **1.191**, 0.945, 0.967 |
| | second | 0.941, 1.010, **1.098** | 0.978, **1.176**, 0.941 |
| | third | **1.202**, 0.965, 0.995 | 0.927, 0.945, **1.136** |
| | fourth | 0.956, **1.233**, 0.972 | **1.207**, 0.904, 0.993 |

At the end of the mapping that failed, A's coupling into its old answer was still above the image's, at 1.010, and into its new answer at 1.098. In every other mapping the old answer's coupling ended below the image's.

#### The value and the punishment after a flip

- **The value's troughs after a flip** were −0.58 to −0.95 of the reward, where H-25's were −0.63 to −0.84.
- **It stayed down longer.** A stimulus's block mean was below minus half the reward in 5 to 24 blocks of a mapping after a flip, where H-25's was in 3 to 15.
- **So a punishment of the old answer delivered little.** The error the modulator receives is the reward less the value. Over a mapping after a flip the trials that selected the old answer delivered a mean error of −0.31 to −0.67 of the reward, and in the slowest reversals most of them less than a quarter of it:

| Arm | Flip, stimulus | Trials that selected the old answer | Their mean error | Under a quarter of the reward | In blocks 9 to 16 |
| :--- | :--- | ---: | ---: | ---: | ---: |
| assignment | first, A | 604 | −0.33 | 66 % | −0.17 |
| | first, B | 554 | −0.38 | 53 % | −0.20 |
| | second, A | 466 | −0.34 | 53 % | −0.20 |
| | second, B | 419 | −0.46 | 18 % | −0.33 |
| | third, A | 192 | −0.67 | 0 % | −0.71 |
| | third, B | 368 | −0.51 | 6 % | −0.44 |
| mirrored | first, A | 615 | −0.31 | 59 % | −0.13 |
| | first, B | 352 | −0.63 | 3 % | −0.49 |
| | second, A | 187 | −0.65 | 1 % | −0.69 |
| | second, B | 404 | −0.48 | 27 % | −0.38 |
| | third, A | 401 | −0.50 | 39 % | −0.34 |
| | third, B | 364 | −0.57 | 7 % | −0.41 |

  The fastest reversals, A onto readout 0 in 14 blocks, are the two rows whose punishments stayed at two thirds of the reward. The row of the mapping that failed is the one whose punishments were smallest in blocks 9 to 16, an eighth of the reward.
- **What this reads, and what it does not.** Between two readouts a punished old answer is replaced by the answer, so the accuracy and the value rise together. Among three the selections that leave the old answer also go to the third readout and to nothing, the accuracy stays low, the value settles near $2p - 1$ of a low $p$, and each punishment is smaller. That is an account of these twelve rows. No run of this round varied the critic, so it is a reading and not a measured cause.

#### The readout's resolution

- **Nothing was selected** in 9.8 to 16.0 per cent of a mapping's trials; H-25's read 4.4 to 7.0.
- **The margin** between the largest count and the next was at most two spikes, a tie among them, in 40 to 65 per cent of a mapping's trials; [ADR-0142](0142-the-readouts-own-competition-measured.md) read 23 to 31 on H-25's runs. The largest count averaged 7.3 to 9.1 spikes.
- **At the first mapping's end** the answer's readout fired 8.9 to 12.8 spikes a trial after the volley and the two others 4.2 to 5.0. At the end of a mapping after a flip the answer's fired 5.6 to 11.5 and the others 3.8 to 5.6. Before the injection each fired 1.6 to 1.9. The sight held through the run: the window after the volley held more than the window before in 58 to 64 trials of every block.

#### Where the consolidation went, and the inhibitory sum

- **The learning signal**, the answer's pairs' net less the other four pairs' net by the network's oracle, was 1.9 to 3.4 million a mapping, 0.52 to 0.75 of H-25's on couplings half the size.
- **Outside the pairs** the weight moved was 0.04 to 0.08 million raised and as much lowered a mapping; by the address it is onto the selected readout's units.
- **The drawn sources** were H-25's: 99.66 per cent of the presented stimulus's units and 1.62 others a trial.
- **The inhibitory sum** rose to 1.010 and 1.008 of the image's and ended at 0.843 and 0.842, still falling; H-25's ended at 0.848 and 0.850.

### The evidence

*This section is written when the dispatch's whole-domain shards have ended.*

### The step of the stopping rule reached

**Step 5**: *"Otherwise no on clause 1: the configuration does not learn three answers on this instrument. The next decision is an ADR choosing between the selection, an exploration among its candidates, and the readout's resolution, with this round's readings of the ties and of where the wrong selections went as its need."* **The next decision is that ADR.** It is named and not taken. What this round hands it:

- **What failed is the revision inside the schedule, not the first choice.** The first mapping was learned as between two. Seven of eight mappings passed, two narrowly; the eighth had one stimulus at 30 of 64 and rising when its 32 blocks ended.
- **The ties**: nothing selected in 10 to 16 per cent of the trials, twice to three times H-25's, and a margin of at most two spikes in half of them.
- **Where the wrong selections went**: to the old answer, 62 to 90 per cent of them. The third readout took 6 to 19 per cent of the trials and its pairs fell below the image's as predicted. An exploration among the candidates would have a new answer to find in two readouts and not one; this round read that it is found within 4 to 133 trials and then rarely chosen for ten to twenty blocks.
- **A reading the rule's list does not name**: the punishment of the old answer shrinks as the value falls, to an eighth to a third of the reward in the slowest reversals. Step 7 names the critic's step, for a no on clause 4 alone. The ADR that step 5 names can weigh the critic beside its three candidates; this ADR does not.
- **The instrument's own lean**: the frozen network orders the readouts for each stimulus before anything is learned, and the reversals followed that order, onto readout 0 in 14 blocks and onto readout 2 in 30 or more for stimulus A. The readouts' rule balances summed weights and does not read it.
- **The schedule**: ADR-0151 kept H-25's so that H-25 stands beside the run, and rejected a longer one. Every mapping after a flip was still improving at its end.

Steps 3, 4, 6 and 7 did not arise; step 8 is kept.

## Consequences

- Good: the first reading of the configuration on a choice among more than two, with one thing changed and H-25 beside it block for block. It learns a first choice among three as it learns one between two.
- Good: the no is located. It is one stimulus in one mapping, on the part of clause 1 that asks each stimulus, and the readings say what was slow: the old answer's hold, under punishments the critic's value had shrunk.
- Good: the readouts are a rule's, the deal the first that passes it in an order committed before any coupling was read, and the frozen block read the instrument before any rewarded run.
- Good: the run holds every excitatory synapse of the arena to an oracle at every trial, with the six pairs named among them, and the gate holds the pinned tables to one another block by block.
- Bad: H-29 is no. The configuration is not named for a choice among three, and the second step of ADR-0151, the third stimulus, has no yes to stand on.
- Bad: the number of answers and the readouts' size changed together, as ADR-0151 said they would. The readings tell some of it apart, the first mapping learned on readouts of 204 units among them, and not all.
- Bad: the composer's split of a trace's terms was not read for three readouts.
- Neutral: two weekly tests. The cost table is regenerated from this round's dispatch.
- Neutral: the frozen network's lean toward the assignment's answers was read before the arms and the arms were not changed for it; ADR-0151 fixed them.

## Alternatives considered and why rejected

- **An order that prefers spread deals** (option 1(b)) **or a shuffle** (option 1(c)): see above.
- **The harness made generic** (option 2(b)): see above.
- **A second composer** (option 3(b)): see above.
- **The sign's rule on the frozen block** (option 4(b)): see above.
- **A third weekly test for the calibration** (option 5(b)): each arm would then depend on another test's having run.
- **Reading the verdict by the count alone.** Clause 1's count is met in all eight mappings. ADR-0151 wrote the part that asks each stimulus before any run, with this case as its reason, and the verdict is read by the clause as written.
- **A second run with a longer mapping, or with the arms started elsewhere.** It would be a second attempt and another schedule than the one H-29 names. Whether to ask it is the next decision's.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `SHARED_PLACES`, `INHIBITORY_PLACES`, `FREE_PLACES`, `deals`, `dealt`, `first_dealt`, `place_couplings`, `deal_couplings`, `answered_sets`, `answers_at`, `largest_alone`, `AnsweredBlock`, `answered_run`, `Answered`, `answered`, `answered_step`, `AnsweredRead`, `answered_arm`, the two weekly tests and the gate's test named above; `Network::placed` and `Network::replay_each`.
- The pins: `PLACE_COUPLINGS_1024`, `DEAL_INDEX_1024`, `DEAL_1024`, `DEALS_PASSING_1024`, `DEALT_COUPLINGS_1024`, `DEALT_SYNAPSES_1024`, `FROZEN_ANSWERED_1024`, the arms' `ANSWERED_*_1024`, the verdict `ANSWERED_1024` and its step `ANSWERED_STEP_1024`, and the gate's checks over them.
- Whitepaper §6.5, §9, §11 (F-65) and §11.1; `CHANGELOG.md`; `CLAUDE.md`; `README.md`; `docs/zh-TW/README.md`.
