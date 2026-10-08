---
status: proposed
date: 2026-10-08
depends-on: ADR-0152
decision-makers: VirtualCortex maintainers
---

# ADR-0153: Three answers, measured — the readouts dealt by ADR-0151's rule and H-29 run once

## Context and Problem Statement

[ADR-0151](0151-three-answers.md) wrote H-29 before any run: on H-25's configuration, stimuli, schedule and arms, with three readouts and an answer for each stimulus that moves to the next readout at a flip, does the engine learn every mapping by each stimulus, with every coupling bounded, the network outside the couplings held and the critic holding each stimulus's expected reward. [ADR-0152](0152-three-answers-built.md) built the selection and the mapping. This ADR is the measurement: the readouts' deal, H-29's protocol, its calibration, its one run and its reading.

It is written in the order the round ran. **This section and the next were committed before any coupling was read.**

## The readouts' deals and their order (written before any coupling was read)

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

**What the gate holds before any coupling is read**: the three lists of places against the rule that derives them; the deals' number; the first, the second, the eleventh, the 561st and the last deal as the order states them; every one of the 92 400 a deal of the rule's — four places a readout, of the fourteen, one inhibitory place and the readout's own, no place twice — and after the one before it in the order; a deal's sets at 1 024 units, each readout 204 units, 51 of them inhibitory; the rule at its edges; and the first deal that passes over tables written by hand.

**One thing read of the geometry and not in ADR-0151.** ADR-0151 wrote that each stimulus unit has four places of each readout in its window. One unit does not: unit 0, stimulus A's first, lies at the ring's seam, where the four units past the last whole period take the place of places 12 to 15 of a period before. It sees a readout's places among 3 to 8 and 16 to 19 only. The other 101 stimulus units see four of each readout. The two-answer geometry has the same seam (`the_geometry_holds_against_the_census_at_both_sizes` reads between four and eight units of a readout in a stimulus unit's window). It is one unit of 51, and the rule reads the couplings the image holds, seam included.

## The deal taken

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

## H-29's protocol (written before the first rewarded run)

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

## The calibration (H-29's stopping rule, step 2)

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

**Every other whole-domain test of the tree, before the arms**: recorded below, with the run.
