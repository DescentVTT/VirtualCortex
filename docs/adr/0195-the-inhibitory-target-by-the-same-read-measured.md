---
status: proposed
date: 2026-10-19
depends-on: ADR-0194
decision-makers: VirtualCortex maintainers
---

# ADR-0195: The inhibitory target by the same read, measured — H-41 is yes, as ADR-0192 predicted — on seeds 22, 23 and 24 at 1 024 units, at a sixteenth, with ADR-0194's flag set the read makes H-40's periods and the engine holds them for both rules from the 129th window; the inhibitory sum then stops falling, each lead-in settling at window 160 with 13.4 to 14.0 per cent more inhibition than its twin under H-40, and H-25's two arms on each network hold H-25's four clauses, 120 to 128 of each mapping's last 128 correct and each reversal passed in 9 to 16 blocks, 227 in all against H-40's 220; all three predicted readings held; the harness's twin check, tried first by a dispatch, failed at the read's last window on a reading taken at the period that window's close set, and was corrected before any derivation; H-41's stopping rule reaches step 3, and the next decision, the engine's default for an image no derivation has settled, is named and not taken; F-70

## Context and Problem Statement

[ADR-0192](0192-the-inhibitory-target-by-the-same-read.md) wrote H-41 before any run and predicts yes: with the engine's read setting the inhibitory rule's period beside the rate's at its end, on seeds 22, 23 and 24 at 1 024 units at a sixteenth, clause 0 and H-25's four hold on every network. [ADR-0194](0194-the-inhibitory-target-by-the-same-read-built.md) built the flag. Brief 075 runs H-41 once, its runs dispatched as [ADR-0184](0184-the-runs-off-the-developer-machine.md) and [ADR-0191](0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md) have them, and asks that H-41's clauses, its constants and its three predicted readings be committed before any lead-in is run with the flag set.

**This ADR is written in two parts.** The protocol below is committed first. The readings are added after the run of each derivation and each arm, and nothing of the protocol moves then.

**No sweep has ended unrecorded**: the last dispatched, run [39131009704](https://github.com/DescentVTT/VirtualCortex/actions/runs/39131009704)'s, [ADR-0191](0191-a-rounds-scope-eighteen-shards-and-the-reproduction-first.md) recorded, and no dispatch since ran one.

What was read first (principle 2), on 2026-10-19, against the build's commit `c92ad6d`:

1. **The flag as built** (`runtime/cortex-runtime/src/executor.rs`). `Config::istdp_target_read` beside `rate_read_windows` at 128 starts the read with the flag set. At the 128th window's close `read_step` gives the read's target where its period lies within the inhibitory rule's 100 to 1 000 000 ticks, and `read_close` sets the rate's period and count and then the inhibitory period to the same period, publishing its depression for the 129th window's first tick; a period outside ends the read refused. Either end spends the flag. Before the 128th window's close nothing differs from the read with the flag unset: `tests/criticality.rs` holds the two engines one at every window of a read.
2. **H-40's derivations, pinned** (`READ_LEAD_IN`, `READ_READ`, `READ_MADE`, ADR-0190). Each lead-in ran 160 windows. Under ADR-0194 the flag acts at the 128th window's close, so a derivation with the flag set runs H-40's first 128 windows, window for window, and its read makes H-40's target: periods 57 604, 58 002 and 57 579, each within the inhibitory rule's range. Its 129th window is the first under the network's own inhibitory target. **So every network has a twin pinned through the read's end**, which no derivation of the tree had before it ran.
3. **What the derivation writes into the image** (`derive_under`). With the flag unset, step 5 counts the settled image frozen over eight windows (`target_lead_in_at`, which asserts the image carries the default 20 000) and writes that period into the arms' image (`targeted_image`). With the flag set the settled image already carries the read's period at `[20..24)`, and ADR-0192 has the image carry it.
4. **H-40's inhibitory sums at its settling**, read from `READ_LEAD_IN` at window 160: 129 936 874, 130 948 477 and 130 184 727, the 129.9, 130.9 and 130.2 M ADR-0192's first predicted reading names.
5. **F-70, found in that reading.** ADR-0192's table heads its first column *"Inhibitory sum at window 96"* and gives 165.9, 160.7 and 164.0 M. `READ_LEAD_IN` reads 165.9 M at window 96 on seed 22, but 160.7 M on seed 23 at window 106 and 164.0 M on seed 24 at window 99, the windows where each network's gain held's lead-in settles; at window 96 seeds 23 and 24 read 165.8 and 165.6 M. ADR-0190's text, which the table restates, says *"where the gain held's lead-in settles"*. No clause, constant or predicted reading of H-41 reads that column.
6. **The arms' ground** (`RuledSite`, `arm_on`). An arm on a read's network whose tables are all empty is held by its rules alone (`Ground::unpinned`, ADR-0190), so its one run exits 0; the arm carries the image's periods, both of them, from its derivation's image.

## Decision Drivers

- ADR-0192's protocol and brief 075's standing directives: three networks, one step, one span, the flag as ADR-0194 built it; no rule that derives a constant changed so that a read or a settling holds; no constant moved after such a run, and no second attempt.
- **A derivation on a network with no pin is held by its rules and by what of it is pinned already**: here, everything up to the read's end, which is H-40's.
- **A check of the harness is tried first by a dispatch**, before the run it guards (ADR-0190).
- **Every derivation and every arm of the tree is the one it was**, H-40's three derivations and six arms among them, at format 24 with the flag unset.
- **A run runs once and on the repository's host**, its job exiting 0 with its log kept (ADR-0184).
- A heavy run is a weekly test; the gate grows by one test ([ADR-0061](0061-the-learning-runs-leave-the-gate.md)).

## Considered Options

1. **What holds a derivation with the flag set**: (a) its rules, and H-40's pinned derivation of the network through the read's end — its 128 windows, its read and its target — beside the read's period as the inhibitory one, then its first run's tables; (b) its rules and its first run's tables alone, as ADR-0190 held H-40's.
2. **The frozen copy's period**: (a) still counted, `Rules::period` the rule's over it, a reading beside the read's, which the image carries; (b) written into the image in the read's place, as before; (c) not counted.
3. **Predicted reading 1's sum**: (a) the lead-in's inhibitory sum at the window its own rule settled it at, against H-40's at window 160; (b) the settled image's, after the quiet run.
4. **What an arm is read beside**: (a) the same network's two arms under H-40; (b) its two under H-39.
5. **A trial of the harness**: (a) the read's 128 windows on seed 22 with the flag set, one window at a time, each H-40's, the inhibitory period set at the last and at none before; (b) none; (c) eight windows, as ADR-0190's trial.
6. **The dispatches**: (a) the calibration with the round's ten tests excluded, beside the trial; the three derivations; then the six arms on the commit that pins their networks' derivations; (b) the derivations and the arms in one.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The protocol (written before any lead-in with the flag set)

- **The networks** (`BOTH_SEEDS`): H-40's, ADR-0044's generator at 1 024 units from seeds 22, 23 and 24.
- **The step, the read and the bound** (`BOTH_STEP`, `BOTH_SPAN`, `BOTH_BOUND`): H-40's, a sixteenth, 128 windows with the last sixteen summed, 320 windows in all.
- **The settling** (`UNDER_BOTH_TARGETS`): H-40's (`UNDER_THE_READ`) with the read's flag for the inhibitory target set (`Settling::inhibitory`), in the candidate's configuration and in the image it reloads from, which carries the flag beside the read under way. H-40's settled network's rule, read over the windows after the read from the sums and the gain the read's last window left, within the bound: the earliest window it can settle at is the 160th.
- **A network settles only where the read made a period both rules accept** (`derive_under`, H-40's): a read refused at its end, by either rule, leaves no settled network, which is clause 0's first part, and the derivation stops. The derivation records what the read left and, under the flag, the inhibitory period the engine then held (`Derivation::istdp`).
- **The image the arms decode carries the read's period at `[20..24)`** (option 2(a)): the settled image already does, and `targeted_image` writes it again, a change of no byte. The target period's rule is still read over the settled image frozen for eight windows (`target_lead_in_under`, the image's period asserted), and `Rules::period` is what it returned: a reading beside the read's, *"the frozen copy's period beside the read's"*, and a rule clause 0 asks to give a value. *Why not the frozen copy's in the image* (option 2(b)): the arms would run under it, and from the 129th window of the lead-in to the arm's first trial the network would have been under one target and its arms under another, the inhibitory target of H-40's arms. *Why not drop the count* (option 2(c)): ADR-0192 asks for it as a reading.
- **Then H-25's rules**, as `derive_under` runs them; a rule that returns nothing stops the derivation there.
- **What holds a derivation with the flag set** (option 1(a), `both_derivation_held`):
  - what it makes before the settled network, and its cancel where it reached it, are the network's derivation's with the gain held, as H-40's are (`before_the_settling_held`, `cancel_held`, `settling_apart`);
  - **every window of the read is H-40's on the network, window for window**, as `READ_LEAD_IN` pins it — every spike, both readouts, both sums, the units at the target, the gain and the estimate — through the 127th, and the 128th in every field but the units at the target (`twin_apart`), and no window of the read moved the gain. The units at the target are a reading the harness takes after a window's close at the inhibitory period then in force (`spikes_and_at_target`), and the 128th window's close is where the flag sets that period to the read's; the network's own fields are H-40's in the 128th too;
  - **the read's sixteen windows and their target are H-40's** (`READ_READ`), and what the engine made of the read is H-40's (`READ_MADE`): the period and the count, the read ended;
  - **the inhibitory period the engine held after the lead-in is that period**, and the image carries it;
  - every window after the read is the rate's rule's against the read's count, and no window passed the train's capacity; no worker dropped an entry;
  - and where its one run is pinned, every table of it (`both_pins_held`), its images at format 24 and its inhibitory period.
  *Why not the first run's tables alone* (option 1(b)): the read's windows are pinned already, by H-40's run; holding a run to them before its own tables exist is the strongest check its one run can meet.
- **The arms** (`both_arm`, `both_site`): H-25's two on H-20's schedule on each network whose clause 0 holds, from the image its derivation wrote, under the true reward, the step, the read target and the inhibitory target carried through every trial; each its own weekly test. An arm's calibration is its network's derivation run again and held as the derivation's own test holds it (`both_made`); on a network whose clause 0 does not hold there is no arm.
- **What an arm is read beside** (option 4(a), `both_stand`): the same network's two arms under H-40 (`read_site`), so each network stands beside its own arms under H-40, as ADR-0192's readings ask: the twin differs by the inhibitory target from window 129 of the lead-in on. *Why not H-39's* (option 4(b)): those differ by the read as well.
- **H-41's clauses** are ADR-0192's, by the rules the tree holds, in H-40's form (`both_verdict`, which is `read_verdict`): clause 0, `Rules::hold` with the read folded into the settled window — the read gives a period both rules accept, the lead-in settles in its sums and its gain's mean, every later rule gives a value and the frozen block passes both measures; clauses 1 to 4, `drawn_of` against the image that network's derivation wrote — at least 80 of each mapping's last 128 trials correct; no coupling past 1.30 of its image's; each reversal passing 40 of 64 within 23 blocks of its flip; the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's. **Yes** when all five hold in both arms on all three networks; a network whose arms did not run is no yes.
- **The stopping rule's steps** (`both_stop`, numbered as ADR-0192's): 3, yes; 4, a no on clause 0 on a network; otherwise 5.
- **The prediction**: yes (`BOTH_PREDICTED`).
- **The three predicted readings as rules** (`both_predicted`), each predicted to hold (`BOTH_READINGS_PREDICTED`):
  1. each network's inhibitory sum at its lead-in's settling above H-40's: the lead-in's inhibitory sum at the window its own rule settled it at (`settled_inhibition`, option 3(a)), strictly above `BOTH_BESIDE_INHIBITION` — 129 936 874, 130 948 477 and 130 184 727, H-40's at window 160, which the gate holds to `READ_LEAD_IN` (`inhibition_above`). A network whose lead-in did not settle does not hold it. *Why not the settled image's* (option 3(b)): ADR-0192's numbers are H-40's lead-in's at its settling, and the quiet run after the lead-in is not a window of it;
  2. in each of the six arms the mean count a window within one per cent of its read target: the windows' spikes summed against as many times the target, a hundred times their distance at most it (`count_within`, H-40's);
  3. through each of the six arms the gain after every window within five per cent of 1.75, 108 954 to 120 422 both counted (`gain_within`, by the arm's `gain_readings`, H-40's).

  The second and the third are read over the arms that ran, and are none where none did.
- **Readings, no clause**, as ADR-0192 lists them: each network beside its own arms under H-40 — each mapping's last 128, each reversal's blocks, the highest coupling, a window's count and its scatter, the gain's band, the inhibitory sum's course, and the value and its troughs; per network the lead-in's inhibitory sum by window beside H-40's, its window of settling beside H-40's 160, and the frozen copy's period beside the read's; over the three networks the mappings learned of twenty-four, and the reversals' blocks beside H-40's.

### The harness (built before any such run)

- **`Settling` carries the flag** (`Settling::inhibitory`, unset in every settling the tree held before), and `candidate_under` sets it in the candidate's configuration and holds the engine to it, the inhibitory period the default at the start.
- **`derive_under` records the inhibitory period after the lead-in under the flag** (`Derivation::istdp`, none without it), holds the frozen image to it, counts the frozen copy under it (`target_lead_in_under`; `target_lead_in_at` calls it with the default) and writes it into the arms' image. With the flag unset every derivation is the one it was.
- **`RuledSite` says whether it is H-41's network** (`RuledSite::inhibitory`): `ruled_pins`, `ruled_stand` and `ruled_made` take H-41's (`both_rate_pins`, `both_stand`, `both_made`) where it is set; every site the tree held before has it unset. An H-41 arm before its run is H-40's case, `Ground::unpinned`, its `read` set.
- **The derivations** (`both_derivation`, `both_derivation_held`, `both_derived`), each a weekly test — `both_targets_read_at_a_sixteenth_on_seed_{22,23,24}_at_1024_units_exhaustive` — dumped whole before anything of it is held, their tables empty until their one run (`BOTH_*`).
- **The six arms**, `the_address_drawn_under_both_targets_read_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`, their tables empty until their one run (`BOTH_ARM_*`).
- **The trial of the harness** (option 5(a)), `the_flag_sets_both_targets_at_the_read_s_end_on_seed_22_at_1024_units_exhaustive`: the candidate under both targets on seed 22's prior, the read's 128 windows one at a time, each held row for row to H-40's pinned lead-in; after each of the first 127 the flag set and the inhibitory period 20 000, and after the 128th the flag spent, the read ended in H-40's target, 57 604 ticks and 2 330 spikes a window, and the inhibitory period 57 604. It runs no window after the read, so it reads nothing of H-41, and it stays a weekly guard of the flag on a network. *Why not eight windows* (option 5(c)): the flag acts at the 128th window and at no other; eight windows would try the image's round trip and nothing of the flag's act. *Why not none* (option 5(b)): a derivation runs once, and a check it fails on its one run is a run lost.
- **The gate** (one test, `the_networks_of_h_41_both_targets_by_the_read_and_its_predicted_readings_rules`): the networks, the step, the span, the bound, the settling against H-40's, the arms and the prediction; `BOTH_BESIDE_INHIBITION` against H-40's pinned lead-ins at their settling, window 160 on each; `settled_inhibition` and `inhibition_above` at their edges over tables by hand; the three predicted readings' rules; the verdict and the stopping rule's steps over tables written by hand; and, over H-41's pinned tables, each network's derivation by its rules — the read's windows H-40's, the read's reading and what the engine made of it H-40's, the inhibitory period the read's, every window after the read the rate's rule's — each arm's gain by block against its reading, and the verdict, its step and the three readings as written, none before every derivation and every arm of each network whose clause 0 holds is pinned (`both_tables`). It runs no lead-in.

### The dispatches (option 6(a))

1. **The calibration**: `scope=exhaustive` on this protocol's commit with this round's ten tests excluded. Every pinned number of the tree must hold at format 24 with the flag unset, H-40's three derivations and six arms among them; if one does not, the round stops there as a finding (ADR-0192's stopping rule, step 2).
2. **The trial**, beside the calibration: `scope=arms` with the trial alone. It is a check of the harness, not a run of H-41.
3. **The derivations**: `scope=arms` with the three, once the calibration's shards are green and the trial passed. Their tables are pinned from their logs and committed.
4. **The arms**: `scope=arms` with the six, on the commit that pins their networks' derivations, which each arm's calibration holds. *Why not one dispatch* (option 6(b)): an arm's stand is its network's pinned image, as ADR-0190 has it.
5. **The reproduction**: `scope=both` as soon as the arms' tables are pinned and pushed (a file under `src/` changed, [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md); ADR-0191), its readings and documents written while it runs.

### How the checks were tried before the runs

- **`arm_on` takes no new branch**: an H-41 arm before its run is H-40's case, `Ground::unpinned`, and every arm the tree pins is held to its tables as it was, which the calibration runs, H-40's six among them.
- **The checks a derivation with the flag set is held by are tried before its run**: the twin's — H-40's 128 windows, its read and its target, the inhibitory period set at the 128th — by the trial's dispatch on seed 22; `settling_apart`, `before_the_settling_held` and `cancel_held` every week on H-40's derivations; the predicted readings' rules and the verdict's in the gate over tables by hand.
- **The trial's first dispatch failed, and the check it failed was the harness's.** Dispatch [39190419222](https://github.com/DescentVTT/VirtualCortex/actions/runs/39190419222) on `9e86d1f` ran the trial in 113 s. Every check of the flag held, after each of the 128 windows: the flag set and the inhibitory period 20 000 through the 127th, and after the 128th the flag spent, the read ended in H-40's target, 57 604 ticks and 2 330 spikes a window, and both periods 57 604. Then it held its table to H-40's whole, and the 128th window parted in one field: the units at the target, 51 648 where H-40's read 25 664 (in parts of $2^{16}$). Every spike, both readouts, both sums, the gain and the estimate of that window, and every field of the 127 before it, are H-40's. The harness reads that field after the window's close at the engine's inhibitory period (`spikes_and_at_target`), and the close it follows is the one where the flag set the period to 57 604, so the 128th window's units at the target are read against the read's period where H-40's were read against 20 000. The protocol's check was wrong, not the network: a window's units at the target measure the network against a period, and the period moved at that window's end.
  - **The correction** (`twin_apart`): the read's windows held to H-40's whole through the 127th and the 128th in every field but the units at the target, in the trial, in `both_derivation_held` and in the gate's `both_tables`, the rule tried at its edges in the gate over H-40's pinned lead-in. It changes no clause, no constant and no predicted reading: none of them reads that field. It changes no test the calibration ran, since the calibration excluded this round's ten and the correction touches only H-41's functions, so no second calibration is owed (ADR-0191). The trial was dispatched again on the correction's commit, `36c9b09`, as [39194363408](https://github.com/DescentVTT/VirtualCortex/actions/runs/39194363408), before any derivation, and passed.
- **What cannot be tried before** is a derivation with the flag set past the read's end: its one run is its trial.

### F-70

**ADR-0192's table heads a column *"Inhibitory sum at window 96"*** where two of its three figures, seed 23's 160.7 M and seed 24's 164.0 M, are at windows 106 and 99, where those networks' gain held's lead-ins settle; at window 96 they read 165.8 and 165.6 M (`READ_LEAD_IN`). Recorded in §11 as resolved by this record. ADR-0192 is accepted and is not edited. No clause, constant or predicted reading reads the column.

### The readings

Everything below was read from one run of each derivation and each arm, each a job of its own on the repository's host, `ubuntu-24.04-arm` (ADR-0184), after the protocol above was committed and pushed (`9e86d1f`) and its harness corrected (`36c9b09`). A job's seconds are not Measured figures ([ADR-0010](0010-measured-or-target.md)); the tables are integers of a deterministic engine, which the dispatched weekly reproduces on x86-64.

**The calibration** (H-41's stopping rule, step 2). Dispatch [39190412104](https://github.com/DescentVTT/VirtualCortex/actions/runs/39190412104) at `scope=exhaustive` on `9e86d1f`, this round's ten tests excluded: the tree's other 146 whole-domain tests, each in a process of its own, 146 of 146 passed, the eighteen shards' jobs 34.8 to 82.4 minutes, at most 46 per cent of their bound. So every pinned number of the tree holds at format 24 with the flag unset — H-40's three derivations, its six arms and its trial among them — and every arm the tree pins took the branch of `arm_on` it took before.

**The trial**, after its correction: dispatch [39194363408](https://github.com/DescentVTT/VirtualCortex/actions/runs/39194363408) on `36c9b09`, beside the calibration, passed in 117 s. Every window of the read on seed 22 is H-40's (`twin_apart`), the flag set and the inhibitory period 20 000 after each of the first 127, and after the 128th the read ended in 57 604 ticks and 2 330 spikes a window, both rules' period 57 604, the flag spent. It read nothing of H-41.

**The derivations**, dispatch [39197018074](https://github.com/DescentVTT/VirtualCortex/actions/runs/39197018074) on `36c9b09`, once the calibration's shards were green and the trial passed: 179, 180 and 179 s, each job passing (`arm-0` to `arm-2`). Each held every check it is held by — what it makes before the settled network and its cancel the gain held's; the read's windows H-40's; the read's sixteen windows, its target and what the engine made of it H-40's; the inhibitory period after the lead-in the read's; every window after the read the rate's rule's; every window inside the train — and found its tables empty and said so after everything was dumped. Its tables were pinned from that run (`3b94bb8`). **Clause 0 holds on all three networks.**

| Network | Read target, a window (period), H-40's | Settled at window, H-40's | Inhibitory sum: window 128 | At the settling | H-40's at its settling | Change, window 128 to 160 (H-40's) | Spikes a window after the read (H-40's mean) | Gain's band after the read | Carried gain | Frozen block: seen of 64 | Gain after it | The frozen copy's period, beside the read's (H-40's copy) |
| :--- | :--- | ---: | ---: | ---: | ---: | :--- | :--- | :--- | :--- | ---: | :--- | :--- |
| Seed 22 | 2 330 (57 604) | 160, 160 | 148.3 M | 148.1 M | 129.9 M | −0.16 % (−12.40 %) | 2 160 to 2 458, 2 329.9 (2 330.6) | 1.7432 to 1.7561 | 1.7500 | 64 | 1.7212 | 57 447, 57 604 (57 512) |
| Seed 23 | 2 314 (58 002) | 160, 160 | 148.8 M | 148.5 M | 130.9 M | −0.21 % (−12.00 %) | 2 122 to 2 456, 2 313.4 (2 314.4) | 1.7424 to 1.7559 | 1.7507 | 64 | 1.7206 | 57 949, 58 002 (58 046) |
| Seed 24 | 2 331 (57 579) | 160, 160 | 148.2 M | 147.9 M | 130.2 M | −0.21 % (−12.16 %) | 2 171 to 2 475, 2 333.3 (2 333.8) | 1.7411 to 1.7534 | 1.7464 | 64 | 1.7176 | 57 896, 57 579 (57 890) |

- **Through the read the network is H-40's**, window for window, on every network: the same 128 windows, the same sixteen counts, the same target, 57 604, 58 002 and 57 579 ticks, which the inhibitory rule accepts and the engine then holds for both rules.
- **From the 129th window the inhibitory sum stops falling.** Under H-40 it fell 12.0 to 12.4 per cent between the read's end and the settling; under both targets it moves 0.16 to 0.21 per cent. So each settled image carries 13.4 to 14.0 per cent more inhibition than its twin's: 148.1 M, 148.5 M and 147.9 M where H-40's carry 129.9 M, 130.9 M and 130.2 M. The excitatory sums at the settling are H-40's to a part in a thousand, 214.1 M, 213.7 M and 214.3 M.
- **The count and the gain are H-40's, near enough.** Over windows 129 to 160 the count stands within a spike a window of its twin's mean, held at the read target by the rate's rule, and the gain moves within 1.7411 and 1.7561, each band H-40's to 0.0009. So the stimulus meets a network firing at the rate it fired at under H-40, with more inhibition.
- **Each lead-in settles at window 160**, the earliest the rule allows, as H-40's did. **Every later rule gives a value**: the cancel is the gain held's on each network, and the frozen block sees the stimulus in 64 trials of 64 on all three.
- **The frozen copy's period, a reading**, lies 0.27 and 0.09 per cent below the read's on seeds 22 and 23 and 0.55 per cent above it on seed 24. Each gives the inhibitory rule the depression the read's period gives, 23.

**The arms**, dispatch [39198215007](https://github.com/DescentVTT/VirtualCortex/actions/runs/39198215007) on `3b94bb8`, the commit that pins their networks' derivations: 1 053 to 1 117 s, each job passing (`arm-0` to `arm-5`). Each first ran its network's derivation again and held it to the tables pinned for it, then ran once. It passed every check before its tables — the watch's count of the open window the record's at every trial, 960 windows, every window's gain the rule's from the network's read count, the critic's step the engine's, the train holding every trial — and was held by its rules alone, its tables unpinned (`Ground::unpinned`). Its tables were pinned from that run, with the verdict, its step and the readings as read (`c1b8d55`).

Each network beside its own arms under H-40 ([ADR-0190](0190-a-target-the-engine-reads-measured.md)):

| | Seed 22, from the assignment | Under H-40 | Seed 22, from the mirrored | Under H-40 |
| :--- | :--- | :--- | :--- | :--- |
| Correct of each mapping's last 128 | 122, 126, 121, 126 | 122, 127, 116, 125 | 124, 121, 120, 123 | 124, 119, 120, 123 |
| First mapping passes 40 of 64 at block | 4 | 3 | 4 | 4 |
| Reversals, blocks from the flip | 11, 16, 13 | 11, 15, 12 | 14, 14, 14 | 13, 14, 12 |
| Highest coupling, of the image's | 1.215 | 1.232 | 1.212 | 1.239 |
| Spikes a window: mean, least to greatest, deviation | 2 330.8, 2 134 to 2 829, 78 | 2 330.8, 2 139 to 2 853, 78 | 2 330.8, 2 147 to 2 828, 76 | 2 330.8, 2 133 to 2 849, 78 |
| Gain: mean, band | 1.7158, 1.7038 to 1.7279 | 1.7151, 1.7031 to 1.7262 | 1.7164, 1.7039 to 1.7276 | 1.7154, 1.7034 to 1.7272 |
| Pairs on opposite sides of the target, of 959 | 591 | 597 | 597 | 607 |
| Inhibitory sum at the end, of its image's | 0.745 (110.4 M) | 0.779 (101.3 M) | 0.742 (109.8 M) | 0.778 (101.1 M) |

| | Seed 23, from the assignment | Under H-40 | Seed 23, from the mirrored | Under H-40 | Seed 24, from the assignment | Under H-40 | Seed 24, from the mirrored | Under H-40 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Correct of each mapping's last 128 | 124, 124, 128, 127 | 125, 125, 126, 127 | 128, 123, 125, 128 | 128, 125, 126, 127 | 125, 126, 128, 126 | 125, 126, 125, 125 | 122, 126, 126, 123 | 122, 124, 125, 123 |
| First mapping passes 40 of 64 at block | 4 | 4 | 2 | 2 | 1 | 1 | 6 | 6 |
| Reversals, blocks from the flip | 12, 11, 12 | 13, 11, 12 | 10, 11, 11 | 10, 11, 11 | 13, 16, 9 | 13, 14, 10 | 12, 13, 15 | 12, 12, 14 |
| Highest coupling, of the image's | 1.202 | 1.205 | 1.205 | 1.198 | 1.223 | 1.221 | 1.234 | 1.213 |
| Spikes a window: mean, least to greatest, deviation | 2 314.8, 2 081 to 2 856, 77 | 2 314.8, 2 081 to 2 847, 78 | 2 314.8, 2 062 to 2 850, 78 | 2 314.7, 2 075 to 2 845, 78 | 2 331.7, 2 074 to 2 838, 82 | 2 331.7, 2 088 to 2 866, 81 | 2 331.7, 2 083 to 2 838, 81 | 2 331.7, 2 106 to 2 864, 80 |
| Gain: mean, band | 1.7145, 1.7010 to 1.7251 | 1.7137, 1.7013 to 1.7240 | 1.7147, 1.7034 to 1.7254 | 1.7137, 1.7025 to 1.7241 | 1.7138, 1.7013 to 1.7261 | 1.7134, 1.7012 to 1.7257 | 1.7140, 1.6990 to 1.7258 | 1.7136, 1.7007 to 1.7251 |
| Pairs on opposite sides of the target, of 959 | 599 | 604 | 611 | 601 | 621 | 621 | 623 | 619 |
| Inhibitory sum at the end, of its image's | 0.745 (110.6 M) | 0.772 (101.1 M) | 0.743 (110.4 M) | 0.771 (101.0 M) | 0.749 (110.8 M) | 0.779 (101.4 M) | 0.745 (110.2 M) | 0.776 (101.0 M) |

In every arm the outside sum stays within 0.9999 and 1.0001 of its image's, the stimulus fires once in each of the 120 blocks, and the value's troughs after the flips stand at $-0.51$ to $-0.83$ of a reward, where H-40's arms on the same networks read $-0.58$ to $-0.80$.

**Through each arm:**
- **The count is the read target's**, 0.03 to 0.04 per cent above it in every arm, as under H-40, with H-40's scatter: a deviation of 76 to 82 spikes where H-40's read 78 to 81.
- **The gain stands 1.9 to 2.1 per cent below 1.75**: means 1.7138 to 1.7164, its band 1.6990 to 1.7279 over the six arms; under H-40, 1.7134 to 1.7154.
- **The controller alternates as under H-40**: consecutive windows stand on opposite sides of the target in 591 to 623 pairs of 959, where H-40's stood in 597 to 621.
- **The inhibitory sum falls again in the arms.** It ends at 0.742 to 0.749 of its image's, 109.8 M to 110.8 M, where H-40's arms end at 0.771 to 0.779, 101.0 M to 101.4 M. So it falls by a quarter through the task here and by 22 to 23 per cent under H-40, from an image that here carries 14 per cent more, and ends 8.6 to 9.4 per cent above its twin's. Under the same period the lead-in's sum held through its last 32 windows; the arms add the task, and what in the task moves it was not varied.
- **Every window's gain is the oracle's**, and the train's count of the open window is the record's own at every trial's end.

**Over the three networks**: every mapping of twenty-four learned, 120 to 128 of its last 128 correct, where H-40's read 116 to 128; every reversal of eighteen passed, in 9 to 16 blocks, the eighteen summing to 227 blocks where under H-40 they sum to 220 — 82 against 77 on seed 22, 67 against 68 on seed 23 and 78 against 75 on seed 24. Each first mapping passes 40 of 64 in the block its twin's does but on seed 22 from the assignment, a block later.

### H-41's verdict

**Yes.** By `both_verdict` over the three networks' pinned derivations and the six arms' pinned blocks, each against the image its network's derivation wrote:
- **clause 0**, on every network: the read makes a period both rules accept, the lead-in settles after it in its sums and its gain's mean, every later rule gives a value and the frozen block passes both measures;
- **clause 1**: every mapping learned, 120 to 128 of each mapping's last 128 where 80 are asked;
- **clause 2**: no coupling past 1.30 of its image's, the highest 1.234;
- **clause 3**: each reversal passes 40 of 64 within 23 blocks of its flip, in 9 to 16;
- **clause 4**: the outside sum within 0.75 and 1.25 of its image's, at 0.9999 to 1.0001.

**The prediction was yes, and it held. So did all three predicted readings:**

| | Predicted | Read |
| :--- | :--- | :--- |
| 1. Each network's inhibitory sum at its lead-in's settling above H-40's | above 129.9, 130.9 and 130.2 M | **held**: 148.1, 148.5 and 147.9 M, 13.4 to 14.0 per cent above |
| 2. Each arm's mean count a window within one per cent of its read target | within 1 % | **held**: 0.029 to 0.036 per cent above in all six |
| 3. The gain within five per cent of 1.75 through each arm | 1.6625 to 1.8375 | **held**: 1.6990 to 1.7279 over 5 760 windows |

**What ADR-0192 said would refute it, against what was read.**
- *More inhibition left in the settled image, enough that the stimulus no longer reaches what the frozen block's measure asks*: 14 per cent more is left, and the frozen block sees the stimulus in 64 trials of 64 on every network.
- *A lead-in whose inhibitory sums still move enough that it does not settle within the bound*: the inhibitory sum moves less than a quarter of a per cent after the read, and each lead-in settles at window 160 of 320.
- *Learning slower from a more inhibited start, a reversal past twenty-three blocks*: the slowest reversal takes 16 blocks. The eighteen take seven blocks more in all than H-40's, on two networks of three; no clause is near its bound.

**What the readings do not say.**
- **Nothing of whether the inhibition the task removes matters**: in the arms the inhibitory sum falls by a quarter under both targets, as under H-40, and no arm ran with that fall held.
- **Nothing of the read's span**: no other span was run, and the inhibitory rule's target is the rate's period the read made.
- **Nothing of the other size**, of a longer run, or of a network the derivation's own rules do not settle.
- **Nothing of what the engine should carry by default**: that is the decision this round names.

### H-41's stopping rule, reached

**Step 3**: the engine reads both of its network's targets for itself. **The next decision, named and not taken**: an ADR on the engine's default for an image no derivation has settled — the rate's rule at a sixteenth, the read of 128 windows, both targets by it — then M10, a reward past the trial ([ADR-0187](0187-the-upgrade-suite-planned.md)). Steps 4 and 5 did not arise. Step 6 kept: no constant moved after a lead-in with the flag set, and no other span, step or network was run.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `BOTH_SEEDS`, `BOTH_STEP`, `BOTH_SPAN`, `BOTH_BOUND`, `BOTH_NAMES`, `UNDER_BOTH_TARGETS`, `BOTH_ARMS`, `BOTH_PREDICTED`, `BOTH_READINGS_PREDICTED`, `BOTH_BESIDE_INHIBITION`; `Settling::inhibitory`, `candidate_under`, `derive_under`, `Derivation::istdp`, `target_lead_in_under`; `settled_inhibition`, `inhibition_above`, `twin_apart`, `both_predicted`, `both_verdict`, `both_stop`; `both_rate_pins`, `both_derivation`, `both_derivation_held`, `both_pins_held`, `both_derived`; `RuledSite::inhibitory`, `both_site`, `both_stand`, `both_made`, `both_arm`; the trial, the gate's test and `both_tables`; the three derivations' and the six arms' weekly tests.
- Whitepaper §11.1 (H-41), §11 (F-70) and §9; `CHANGELOG.md`; `docs/adr/README.md`.
