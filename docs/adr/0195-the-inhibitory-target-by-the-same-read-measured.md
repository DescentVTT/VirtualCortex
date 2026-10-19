---
status: proposed
date: 2026-10-19
depends-on: ADR-0194
decision-makers: VirtualCortex maintainers
---

# ADR-0195: The inhibitory target by the same read, measured — H-41's protocol, written before any lead-in is run with the flag set: on seeds 22, 23 and 24 at 1 024 units, at a sixteenth, H-40's read of 128 windows with ADR-0194's flag set, so that at the read's last window the inhibitory rule's period becomes the rate's; H-40's settled network's rule over the windows after the read, within 320; a derivation with no pin held by its rules and by H-40's pinned derivation through the read's end, its twin; the frozen copy's period a reading beside the read's, which the image carries; each arm beside the same network's arm under H-40; three predicted readings as rules over the tables, the first against H-40's pinned inhibitory sums at window 160; a trial of the read's 128 windows on seed 22 dispatched beside the calibration; F-70

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
  - **every window of the read is H-40's on the network, window for window**, as `READ_LEAD_IN` pins it — every spike, both readouts, both sums, the units at the target, the gain and the estimate — through the 128th, and no window of the read moved the gain;
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
- **What cannot be tried before** is a derivation with the flag set past the read's end: its one run is its trial.

### F-70

**ADR-0192's table heads a column *"Inhibitory sum at window 96"*** where two of its three figures, seed 23's 160.7 M and seed 24's 164.0 M, are at windows 106 and 99, where those networks' gain held's lead-ins settle; at window 96 they read 165.8 and 165.6 M (`READ_LEAD_IN`). Recorded in §11 as resolved by this record. ADR-0192 is accepted and is not edited. No clause, constant or predicted reading reads the column.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `BOTH_SEEDS`, `BOTH_STEP`, `BOTH_SPAN`, `BOTH_BOUND`, `BOTH_NAMES`, `UNDER_BOTH_TARGETS`, `BOTH_ARMS`, `BOTH_PREDICTED`, `BOTH_READINGS_PREDICTED`, `BOTH_BESIDE_INHIBITION`; `Settling::inhibitory`, `candidate_under`, `derive_under`, `Derivation::istdp`, `target_lead_in_under`; `settled_inhibition`, `inhibition_above`, `both_predicted`, `both_verdict`, `both_stop`; `both_rate_pins`, `both_derivation`, `both_derivation_held`, `both_pins_held`, `both_derived`; `RuledSite::inhibitory`, `both_site`, `both_stand`, `both_made`, `both_arm`; the trial, the gate's test and `both_tables`; the three derivations' and the six arms' weekly tests.
- Whitepaper §11.1 (H-41), §11 (F-70) and §9; `CHANGELOG.md`; `docs/adr/README.md`.
