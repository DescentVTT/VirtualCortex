---
status: proposed
date: 2026-10-18
depends-on: ADR-0189
decision-makers: VirtualCortex maintainers
---

# ADR-0190: A target the engine reads, measured — H-40 is yes, as ADR-0188 predicted — on seeds 22, 23 and 24 at 1 024 units, at a sixteenth, the engine's read makes a target on every network, 2 330, 2 314 and 2 331 spikes a window, 1.0 to 2.8 per cent above the derived ones, each lead-in settling at window 160, and H-25's two arms on each hold H-25's four clauses, 116 to 128 of each mapping's last 128 correct and each reversal passed in 10 to 15 blocks, the eighteen in fewer blocks than under H-39; the count held at the read target with H-39's scatter; two of the three predicted readings held and the first missed, the live count still rising through the read as the inhibitory sum fell, seed 22's target above its derived one; H-40's stopping rule reaches step 3, and the next decision, the engine's default for an image no derivation has settled, is named and not taken

## Context and Problem Statement

[ADR-0188](0188-a-target-the-engine-reads.md) wrote H-40 before any run and predicts yes: with the engine holding its gain for 128 windows from its first and taking the mean count of the last sixteen as its target, then running the rate's rule at a sixteenth, on seeds 22, 23 and 24 at 1 024 units, clause 0 and H-25's four hold on every network. [ADR-0189](0189-a-target-the-engine-reads-built.md) built the read. Brief 074 runs H-40 once, its runs dispatched as [ADR-0184](0184-the-runs-off-the-developer-machine.md) has them, and asks that the span, H-40's clauses, its constants and its three predicted readings be committed before any lead-in is run with the read set.

**This ADR is written in two parts.** The protocol below is committed first. The readings are added after the run of each derivation and each arm, and nothing of the protocol moves then.

**No sweep has ended unrecorded**: the last dispatched, run [38417864748](https://github.com/DescentVTT/VirtualCortex/actions/runs/38417864748)'s, [ADR-0183](0183-a-sixteenth-on-three-networks-measured.md) recorded, and no dispatch since ran one.

What was read first (principle 2), on 2026-10-18, against the build's commit `356d10d`:

1. **The read as built** (`runtime/cortex-runtime/src/executor.rs`). `Config::rate_read_windows` at 128 starts the read with 128 windows left. Each window's end holds the gain (`HomeostaticDrivePool::hold`), counts one window down and sums the windows with sixteen or fewer left, 113 to 128. At the 128th the engine holds the period of the mean and that period's count, the read none, the rate's rule from the 129th. A mean the rule refuses ends the read refused, the gain held from there on.
2. **The settled network's rule under the rule** (`tests/inhibition.rs`, `settled_in_mean`). It finds the first window at which ADR-0055's criterion holds over the excitatory sums and the gain's mean of sixteen windows is within 0.75 per cent of the sixteen before. **Read over a lead-in under the read whole, it would settle every network inside the read**, before the rule had moved the gain once: through the read the gain is one number, so its means agree from the 32nd window, and the sums settle where the gain held's do, at windows 96, 106 and 99.
3. **The gain held's lead-ins** (`BACKGROUND_LEAD_IN_1024[SETTLED]`, `LEAD_IN_SEEDED`): 96, 106 and 99 windows on seeds 22, 23 and 24, all inside the read's 128. Under the read the gain is held as there, and the estimate is stored by the slope as there, the saturation rule never reached (the greatest window of the three holds 2 487 spikes, the rule's count 32 768). So the read's windows are those lead-ins, window for window, as far as each ran, and nothing of the read's last sixteen is pinned anywhere.
4. **H-39 on the three networks** (`RATE_RULES[1]`, `SEEDED_RATE_RULES`): under the rule from the first window the lead-ins settled at windows 96, 105 and 93.
5. **The arms' tables** (`arm_on`). An arm under the true reward is held to its pinned tables after it has dumped them. Under ADR-0184 a run is a job that must exit 0 with one test passed, so an arm whose tables are not yet pinned would fail its own run at its first pin.

## Decision Drivers

- ADR-0188's protocol and brief 074's standing directives: three networks, one step, one span; the read as ADR-0189 built it; no rule that derives a constant changed so that a read holds; no constant moved after such a run, and no second attempt.
- **The settled network's rule reads the controller it settles**: windows in which no rule moved the gain cannot show that the rule holds it.
- **A derivation on a network with no pin is held by its rule and its first run's tables**, and by what of it the read does not reach, which is pinned already (ADR-0183's form).
- **A check of the harness is tried first by a dispatch**, before the run it guards (brief 074; ADR-0181's arms ran twice for a check that was not).
- **Every derivation and every arm of the tree is the one it was**, H-39's three derivations and six arms among them.
- **A run runs once and on the repository's host**, and its job exits 0 with its log kept (ADR-0184).
- A heavy run is a weekly test; the gate grows by one test ([ADR-0061](0061-the-learning-runs-leave-the-gate.md)).

## Considered Options

1. **The settled network's rule under the read**: (a) H-39's, over the windows after the read, from the sums and the gain the read's last window left, the window counted from the lead-in's first; (b) H-39's over the whole lead-in, at the first window past the read; (c) H-39's over the whole lead-in.
2. **A read that makes no target**: (a) no settled network, folded into the rule's window, so clause 0 is a no and the derivation stops; (b) a rule of its own beside the others.
3. **What holds a derivation under the read**: (a) its rules — what it makes before the settled network and its cancel the gain held's, the read's windows the gain held's lead-in as far as it ran, the target the hand rule's of the read's sixteen windows, every window after the read the rate's rule's — and its first run's tables where pinned; (b) its first run's tables alone.
4. **An arm before its one run**: (a) held by its rules alone, every table of it unpinned, so that its run exits 0; (b) held to empty tables, its run failing at its first pin with its dump kept.
5. **The dispatches**: (a) the calibration with the round's tests excluded, beside a trial of the harness; the three derivations; then the six arms, once their networks' derivations are pinned; (b) the derivations and the arms in one dispatch.
6. **Predicted reading 1's target**: (a) a window's count, the engine's beside the derivation's; (b) a period.
7. **What an arm is read beside**: (a) the same network's two arms under H-39; (b) its two with the gain held.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a) and 7(a).**

### The protocol (written before any lead-in with the read set)

- **The networks** (`READ_SEEDS`): ADR-0044's generator at 1 024 units from seeds 22, 23 and 24, H-39's.
- **The step** (`READ_STEP`): a sixteenth, `0x1000`, on every network.
- **The read** (`READ_SPAN`, `READ_LAST`): 128 windows from the lead-in's first, the last sixteen summed; no target in the configuration (ADR-0189 refuses one beside a read).
- **The settling** (`UNDER_THE_READ`): H-39's at a sixteenth with the read in the target's place — the step and the read in the candidate's configuration and in the image it reloads from; the lead-in's 128 windows of held gain first; then H-39's settled network's rule, the excitatory sums by ADR-0055's criterion and the gain's mean of sixteen windows, **read over the windows after the read**, from the excitatory sum and the gain the read's last window left, the window counted from the lead-in's first (option 1(a), `Settling::settled`); within 320 windows in all. The earliest window it can settle at is the 160th, both blocks of sixteen gains windows the rule moved the gain in. *Why not the whole lead-in* (options 1(b), 1(c)): read whole, the rule compares the gain the rule moved with the gain the read held, and settles a network at the 129th window, one window after the rule began, or inside the read before it began.
- **A network settles only where the read made a target the rule accepts** (option 2(a), `derive_under`): a read still under way or refused at the lead-in's end leaves no settled network, which is clause 0's first part, and the derivation stops. The derivation records what the read left — under way, refused, or none — and the period and the count the engine then held (`Derivation::read`). The image it writes carries that period at `[56..60)` and no read; the inhibitory rule's target stays the derivation's at `[20..24)`.
- **Then H-25's rules**, as `derive_under` runs them; a rule that returns nothing stops the derivation there.
- **What holds a derivation under the read** (option 3(a), `read_derivation_held`):
  - what it makes before the settled network, and its cancel where it reached it, are the network's derivation's with the gain held: on seed 22 `before_the_settling_held` and `cancel_held`, on seeds 23 and 24 `settling_apart` against H-33's pins;
  - **the read's windows are the gain held's lead-in, window for window** — every spike, both readouts, both sums, the units at the target, the gain and the estimate — as far as that lead-in ran, 96, 106 and 99 windows (`gain_held_lead_in`), and no window of the read moved the gain;
  - **what the engine made of the read is the hand rule's of its sixteen windows** (`read_target_by_hand`, ADR-0188's text in `i128` by doubling): the period and its count with the read ended, or, where the hand rule refuses, the read refused;
  - every window after the read is the rate's rule's against that count (`rate_read` over them, its `unruled`), and no window passed the train's capacity; no worker dropped an entry;
  - and where its one run is pinned, every table of it (`read_pins_held`), its images at format 23.
- **The arms** (`read_arm`, `read_site`): H-25's two on H-20's schedule on each network whose clause 0 holds, from the image its derivation wrote, under the true reward, the step and the read target carried through every trial; each its own weekly test. An arm's calibration is its network's derivation run again and held as the derivation's own test holds it, to its pinned tables (`read_made`); on a network whose clause 0 does not hold there is no arm.
- **What an arm is read beside** (option 7(a), `read_stand`): the same network's two arms under H-39 (`sixteenth_site`), so each network stands beside its own arms under H-39, as ADR-0188's readings ask.
- **H-40's clauses** are ADR-0188's, by the rules the tree holds, in H-39's form (`read_verdict`, which is `sixteenth`): clause 0, `Rules::hold` with the read folded into the settled window; clauses 1 to 4, `drawn_of` against the image that network's derivation wrote — at least 80 of each mapping's last 128 trials correct; no coupling past 1.30 of its image's; each reversal passing 40 of 64 within 23 blocks of its flip; the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's. **Yes** when all five hold in both arms on all three networks; a network whose arms did not run is no yes.
- **The stopping rule's steps** (`read_stop`): 3, yes; 4, a no on clause 0 on a network; otherwise 5.
- **The prediction**: yes (`READ_PREDICTED`).
- **The three predicted readings as rules** (`read_predicted`), each predicted to hold (`READ_READINGS_PREDICTED`):
  1. each network's read target within five per cent of its derived one, seed 22's below it: the read's count, the engine's, against `SIXTEENTH_COUNTS` — 2 306, 2 255 and 2 268 — twenty times their distance at most the derived count, and seed 22's strictly below (`read_target_within`, option 6(a)). A count below is a rate below, as ADR-0188 wrote it of seed 22's live count. A network whose read made no target does not hold it;
  2. in each of the six arms the mean count a window within one per cent of its read target: the windows' spikes summed against as many times the target, a hundred times their distance at most it (`count_within`, from the arm's `GainRead`);
  3. through each of the six arms the gain after every window within five per cent of 1.75, 108 954 to 120 422 both counted (`gain_within`, by the arm's `gain_readings`).

  The second and the third are read over the arms that ran, and are none where none did.
- **Readings, no clause**, as ADR-0188 lists them: each network beside its own arms under H-39 — each mapping's last 128, each reversal's blocks, the highest coupling, a window's count and its scatter, the gain's band and mean, the inhibitory sum's course, and the value and its troughs; per network the read's sixteen windows, the read target beside the derived one, the lead-in's window of settling beside H-39's, and the inhibitory target period beside H-39's; over the three networks the mappings learned of twenty-four, and the reversals' blocks beside H-39's.

### The harness (built before any such run)

- **`Settling` carries the read** (`Settling::read`, zero in every settling the tree held before), and `Settling::settled` reads the windows after it (`settled_over`, the rule as it was). `candidate_under` sets it in the candidate's configuration and holds the engine to it.
- **`RuledSite` says whether it is a read's network** (`RuledSite::read`): `ruled_pins`, `ruled_stand` and `ruled_made` take H-40's (`read_rate_pins`, `read_stand`, `read_made`) where it is set; every site the tree held before has it unset.
- **An arm before its one run** (option 4(a), `Ground::unpinned`): an arm of a read's network whose every table is empty and whose every reading is none is held by its rules alone — the calibration, the critic's step, the train, the window's count against the record's, every window's gain the rule's, no worker's entry dropped — and its dump is the run's record; its run exits 0. An arm of a read's network pinned in part is refused, and every other arm of the tree is held to its tables as it was.
- **The derivations** (`read_derivation`, `read_derivation_held`, `read_derived`), each a weekly test — `the_target_read_at_a_sixteenth_on_seed_{22,23,24}_at_1024_units_exhaustive` — dumped whole before anything of it is held, their tables empty until their one run (`READ_*`).
- **The six arms**, `the_address_drawn_under_the_target_read_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`, their tables empty until their one run (`READ_ARM_*`).
- **The trial of the harness**, `the_read_s_first_eight_windows_on_seed_22_are_the_gain_held_s_at_1024_units_exhaustive`: the candidate under the read on the instrument's prior run for eight windows, each held row for row to ADR-0077's lead-in with the gain held, the read under way with 120 windows left and nothing summed, no window moving the gain. It reads nothing of H-40 — no window of the read's last sixteen — and stays a weekly guard of the read's first windows.
- **The gate** (one test, `the_networks_of_h_40_the_read_s_settling_and_its_predicted_readings_rules`): the networks, the step, the span, the bound, the settling against H-39's, the arms and the prediction; the settled network's rule under the read over tables written by hand — a read whose sums drift past the tolerance settles nothing and the first settled window is the 160th, one short is none, a read whose windows would settle the whole-lead-in rule at its 32nd still settles at the 160th, a step of one per cent after the read settles at the 166th, a gain's mean apart by one per cent at the 165th; `read_target_by_hand` at the three derived counts, a half either side, no spike and a short read; `read_read` over a table by hand; the three predicted readings' rules at their edges; the verdict and the stopping rule's steps over tables written by hand; and, over H-40's pinned tables, each network's derivation by its rules, each arm's gain by block against its reading, and the verdict, its step and the three readings as written, none before every derivation and every arm of each network whose clause 0 holds is pinned (`read_tables`). It runs no lead-in.

### The dispatches (option 5(a))

1. **The calibration**: `scope=exhaustive` on the protocol's commit with this round's ten tests excluded. Every pinned number of the tree must hold at format 23 with the read unset, H-39's three derivations and six arms among them; if one does not, the round stops there as a finding (ADR-0188's stopping rule, step 2).
2. **The trial**, beside the calibration: `scope=arms` with the trial alone. It is a check of the harness, not a run of H-40.
3. **The derivations**: `scope=arms` with the three, once the calibration's shards are green and the trial passed. Their tables are pinned from their logs and committed.
4. **The arms**: `scope=arms` with the six, on the commit that pins their networks' derivations, which each arm's calibration holds. *Why not one dispatch* (option 5(b)): an arm's stand is its network's pinned image, and an arm whose calibration held nothing pinned would be a second derivation's run with no table to reproduce.
5. **The reproduction**: `scope=both` once the arms' tables are pinned (a file under `src/` changed, [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)).

### How a check added to an arm was tried before the runs

- **`arm_on` gains one branch and no check**: `Ground::unpinned` is false for every ground the tree held before — the instrument's, every `Site`, and every `RuledSite` with the read unset — so the calibration runs every pinned arm of the tree, H-38's two and H-39's six among them, through the branch it took, which holds every table as it did.
- **The ground of a read's network** reads its arms beside from `sixteenth_site`, the tables H-39's arms are held to every week, and its derivation through `read_made`, which holds the derivation as its own test does.
- **The checks a derivation under the read is held by are tried before its run**: the settling's rule, `read_target_by_hand` and `read_read` in the gate over tables by hand; the read's windows against the gain held's lead-in on seed 22 by the trial's dispatch, eight windows; `settling_apart`, `before_the_settling_held` and `cancel_held` every week on H-39's derivations.
- **What cannot be tried before** is a derivation under the read itself, past its eighth window: its one run is its trial.

### The readings

Everything below was read from one run of each derivation and each arm, each a job of its own on the repository's host, `ubuntu-24.04-arm` (ADR-0184), after the protocol above was committed and pushed (`ddc7f2c`). A job's seconds are not Measured figures ([ADR-0010](0010-measured-or-target.md)); the tables are integers of a deterministic engine, which the dispatched weekly reproduces on x86-64.

**The calibration** (H-40's stopping rule, step 2). Dispatch [39120143892](https://github.com/DescentVTT/VirtualCortex/actions/runs/39120143892) at `scope=exhaustive` on `ddc7f2c`, this round's ten tests excluded: the tree's other 136 whole-domain tests, each in a process of its own, 136 of 136 passed, the twelve shards' jobs 46 to 99 minutes. So every pinned number of the tree holds at format 23 with the read unset — H-38's six steps and two arms and H-39's derivations and six arms among them — and every arm the tree pins took the branch of `arm_on` it took before.

**The trial**, dispatch [39120156976](https://github.com/DescentVTT/VirtualCortex/actions/runs/39120156976) on `ddc7f2c`, beside the calibration: the candidate under the read on seed 22 ran eight windows, each ADR-0077's lead-in with the gain held row for row, the read at 120 windows left and nothing summed, in 7 s. It read nothing of H-40.

**The derivations**, dispatch [39127001541](https://github.com/DescentVTT/VirtualCortex/actions/runs/39127001541) on `ddc7f2c`, once the calibration's shards were green: 177, 186 and 156 s, each job passing (`arm-0` to `arm-2`). Each held every check it is held by — what it makes before the settled network and its cancel the gain held's; the read's windows the gain held's lead-in, window for window, through its 96, 106 and 99 windows; what the engine made of the read the hand rule's of its sixteen windows; every window after the read the rate's rule's; every window inside the train — and found its tables empty and said so after everything was dumped. Its tables were pinned from that run (`297def6`). **Clause 0 holds on all three networks.**

| Network | The read's sixteen windows, 113 to 128 | Read target, a window (period) | Derived, a window (period) | Read beside derived | Settled at window | H-39's, and the gain held's | Gain's band after the read | Spikes a window after the read | Carried gain | Frozen block: seen of 64 | Gain after it | Inhibitory target period, H-39's |
| :--- | :--- | ---: | ---: | ---: | ---: | :--- | :--- | :--- | :--- | ---: | :--- | :--- |
| Seed 22 | 2 181 to 2 437, 2 330.1 | 2 330 (57 604) | 2 306 (58 201) | 1.04 % above | 160 | 96, 96 | 1.7423 to 1.7561 | 2 160 to 2 463 | 1.7489 | 64 | 1.7198 | 57 512, 58 188 |
| Seed 23 | 2 224 to 2 392, 2 313.6 | 2 314 (58 002) | 2 255 (59 510) | 2.62 % above | 160 | 105, 106 | 1.7416 to 1.7559 | 2 121 to 2 448 | 1.7492 | 64 | 1.7199 | 58 046, 59 672 |
| Seed 24 | 2 212 to 2 476, 2 331.3 | 2 331 (57 579) | 2 268 (59 179) | 2.78 % above | 160 | 93, 99 | 1.7411 to 1.7527 | 2 182 to 2 464 | 1.7457 | 64 | 1.7164 | 57 890, 59 280 |

- **Each lead-in settles at window 160, the earliest the rule allows**: at the 32nd window after the read the sums of the sixteen before are within ADR-0055's tolerance and the gain's means within 0.75 per cent. The rule moves the gain little in those windows, 1.7411 to 1.7561 against the 1.75 it held.
- **The count drifts upward through the read.** With the gain held the live count's last sixteen windows stand 2 243.6, 2 265.6 and 2 281.7 a window at windows 81 to 96, 91 to 106 and 84 to 99, where the gain held's lead-ins settle. Through windows 113 to 128 they stand 2 330.1, 2 313.6 and 2 331.3, 3.9, 2.1 and 2.2 per cent higher. Under the rule after the read the count holds at the target, 2 330.6, 2 314.4 and 2 333.8 a window over windows 129 to 160.
- **The inhibitory sum is still falling.** On seed 22 it stands at 165.9 M where the gain held's lead-in settles, at window 96; at 148.3 M at the read's end; and at 129.9 M at window 160. On seeds 23 and 24 it falls from 160.7 M and 164.0 M to 148.8 M and 148.2 M at the read's end and to 130.9 M and 130.2 M at window 160. The excitatory sums settle, as ADR-0055's criterion reads them: 218.2 M, 215.8 M and 214.1 M on seed 22 at the same three windows. So each settled image carries a fifth less inhibition than H-39's (129.9 M, 130.9 M and 130.2 M where H-39's carry 165.2 M, 161.7 M and 167.2 M), and its frozen copy fires faster. The inhibitory target periods that copy gives are 1.2, 2.7 and 2.3 per cent shorter than H-39's.
- **Every later rule gives a value.** The cancel is the gain held's on each network, and the frozen block sees the stimulus in 64 trials of 64 on all three.
- **No window comes near the saturation rule's count**: the greatest of the 480 holds 2 487 spikes of 32 768, seed 24's first window, which is the gain held's.

**The arms**, dispatch [39128573344](https://github.com/DescentVTT/VirtualCortex/actions/runs/39128573344) on `297def6`, the commit that pins their networks' derivations: 1 071 to 1 120 s, each job passing (`arm-0` to `arm-5`). Each first ran its network's derivation again and held it to the tables pinned for it, then ran once. It passed every check before its tables — the watch's count of the open window the record's at every trial, 960 windows, every window's gain the rule's from the network's read count, the critic's step the engine's, the train holding every trial — and was held by its rules alone, its tables unpinned (`Ground::unpinned`). Its tables were pinned from that run, with the verdict, its step and the readings as read (`8342095`).

Each network beside its own arms under H-39 ([ADR-0183](0183-a-sixteenth-on-three-networks-measured.md)):

| | Seed 22, from the assignment | Under H-39 | Seed 22, from the mirrored | Under H-39 |
| :--- | :--- | :--- | :--- | :--- |
| Correct of each mapping's last 128 | 122, 127, 116, 125 | 122, 121, 126, 123 | 124, 119, 120, 123 | 124, 121, 119, 122 |
| First mapping passes 40 of 64 at block | 3 | 6 | 4 | 4 |
| Reversals, blocks from the flip | 11, 15, 12 | 15, 19, 16 | 13, 14, 12 | 19, 13, 14 |
| Highest coupling, of the image's | 1.232 | 1.203 | 1.239 | 1.206 |
| Spikes a window: mean, least to greatest, deviation | 2 330.8, 2 139 to 2 853, 78 | 2 306.7, 2 080 to 2 842, 75 | 2 330.8, 2 133 to 2 849, 78 | 2 306.8, 2 074 to 2 846, 75 |
| Gain: mean, band | 1.7151, 1.7031 to 1.7262 | 1.7206, 1.7096 to 1.7309 | 1.7154, 1.7034 to 1.7272 | 1.7201, 1.7079 to 1.7316 |
| Pairs on opposite sides of the target, of 959 | 597 | 597 | 607 | 607 |
| Inhibitory sum at the end, of its image's | 0.779 | 0.745 | 0.778 | 0.743 |

| | Seed 23, from the assignment | Under H-39 | Seed 23, from the mirrored | Under H-39 | Seed 24, from the assignment | Under H-39 | Seed 24, from the mirrored | Under H-39 |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Correct of each mapping's last 128 | 125, 125, 126, 127 | 124, 120, 125, 125 | 128, 125, 126, 127 | 121, 120, 122, 124 | 125, 126, 125, 125 | 121, 124, 124, 124 | 122, 124, 125, 123 | 124, 120, 120, 126 |
| First mapping passes 40 of 64 at block | 4 | 3 | 2 | 3 | 1 | 1 | 6 | 7 |
| Reversals, blocks from the flip | 13, 11, 12 | 13, 13, 14 | 10, 11, 11 | 12, 13, 13 | 13, 14, 10 | 17, 14, 13 | 12, 12, 14 | 14, 11, 14 |
| Highest coupling, of the image's | 1.205 | 1.198 | 1.198 | 1.215 | 1.221 | 1.209 | 1.213 | 1.207 |
| Spikes a window: mean, least to greatest, deviation | 2 314.8, 2 081 to 2 847, 78 | 2 255.6, 2 027 to 2 646, 75 | 2 314.7, 2 075 to 2 845, 78 | 2 255.7, 2 013 to 2 646, 74 | 2 331.7, 2 088 to 2 866, 81 | 2 268.6, 2 023 to 2 604, 74 | 2 331.7, 2 106 to 2 864, 80 | 2 268.6, 2 032 to 2 602, 74 |
| Gain: mean, band | 1.7137, 1.7013 to 1.7240 | 1.7174, 1.7057 to 1.7272 | 1.7137, 1.7025 to 1.7241 | 1.7167, 1.7051 to 1.7271 | 1.7134, 1.7012 to 1.7257 | 1.7184, 1.7069 to 1.7321 | 1.7136, 1.7007 to 1.7251 | 1.7186, 1.7065 to 1.7322 |
| Pairs on opposite sides of the target, of 959 | 604 | 607 | 601 | 619 | 621 | 614 | 619 | 608 |
| Inhibitory sum at the end, of its image's | 0.772 | 0.782 | 0.771 | 0.780 | 0.779 | 0.776 | 0.776 | 0.776 |

In every arm the outside sum stays within 0.9999 and 1.0001 of its image's, the stimulus fires once in each of the 120 blocks, and the value's troughs after the flips stand at $-0.58$ to $-0.80$ of a reward, where H-39's arms on the same networks read $-0.56$ to $-0.84$.

**Through each arm the controller reads:**
- **The count is the read target's**, 0.03 per cent above it in every arm: 2 330.8 spikes a window on seed 22, 2 314.8 and 2 314.7 on seed 23, 2 331.7 twice on seed 24 — 1.1, 2.7 and 2.8 per cent above the counts H-39 held the same networks to.
- **The gain stands 1.8 to 2.0 per cent below its image's**: means 1.7134 to 1.7154 against carried gains of 1.7457 to 1.7492, its band 1.7007 to 1.7272 over the six arms, 1.3 to 2.8 per cent below 1.75 after every window; under H-39, 1.7167 to 1.7206.
- **A window's count scatters as under H-39**: a deviation of 78 to 81 spikes, where H-39's read 74 or 75.
- **It alternates as under H-39**: consecutive windows stand on opposite sides of the target in 597 to 621 pairs of 959, where H-39's stood in 597 to 619.
- **Every window's gain is the oracle's**, and the train's count of the open window is the record's own at every trial's end.

**The inhibitory sum** ends at 0.771 to 0.779 of its image's, within 0.04 of the same network's arm under H-39 — above it on seed 22, below it on seed 23, at it on seed 24 — and from an image that carries a fifth less: 101.0 M to 101.4 M at the end of every arm, where H-39's arms end at 122.7 M to 129.7 M.

**Over the three networks**: every mapping of twenty-four learned, 116 to 128 of its last 128 correct, where H-39's read 119 to 126; every reversal of eighteen passed, in 10 to 15 blocks, the eighteen summing to 220 blocks where under H-39 they sum to 257 — 77 against 96 on seed 22, 68 against 78 on seed 23 and 75 against 83 on seed 24.

### H-40's verdict

**Yes.** By `read_verdict` over the three networks' pinned derivations and the six arms' pinned blocks, each against the image its network's derivation wrote:
- **clause 0**, on every network: the read makes a target the rule accepts, the lead-in settles after it in its sums and its gain's mean, every later rule gives a value and the frozen block passes both measures;
- **clause 1**: every mapping learned, 116 to 128 of each mapping's last 128 where 80 are asked;
- **clause 2**: no coupling past 1.30 of its image's, the highest 1.239;
- **clause 3**: each reversal passes 40 of 64 within 23 blocks of its flip, in 10 to 15;
- **clause 4**: the outside sum within 0.75 and 1.25 of its image's, at 0.9999 to 1.0001.

**The prediction was yes, and it held.** The three predicted readings:

| | Predicted | Read |
| :--- | :--- | :--- |
| 1. Each read target within five per cent of its derived one, seed 22's below it | within, below | **missed**: within, 1.04, 2.62 and 2.78 per cent above; seed 22's above, not below |
| 2. Each arm's mean count a window within one per cent of its read target | within 1 % | **held**: 0.028 to 0.034 per cent above in all six |
| 3. The gain within five per cent of 1.75 through each arm | 1.6625 to 1.8375 | **held**: 1.7007 to 1.7272 over 5 760 windows |

**The missed reading.** ADR-0188 read seed 22's live count with the gain held at windows 81 to 96, 2.7 per cent below its derived target, and predicted the read below it. The read takes windows 113 to 128, and through the thirty-two windows between the network kept moving: its inhibitory sum fell by a tenth and its count rose 3.9 per cent, to 1.0 per cent above. Seeds 23 and 24 rose the same way, by 2.1 and 2.2 per cent. The read reads the rate of a network still settling in its inhibition, as ADR-0188's first refutation named it — *"a network still drifting in its count at windows 113 to 128, so that its target is the drift's and not its rate's"* — and the target that drift gives served as a derived one did. This is an account of the tables, and the round did not vary it: no other span was run.

**What ADR-0188 said would refute it, against what was read.**
- *A network still drifting in its count at windows 113 to 128*: each was, by 2 to 4 per cent from where its gain held's lead-in settled; every clause held under the target it read.
- *A target a few per cent low that moves the gain far enough from 1.75 to change what the stimulus reaches*: the targets are a few per cent high, the gain stays within 2.8 per cent of 1.75, and the stimulus fires once in every block.
- *The 128 windows of held gain leaving the lead-in too few windows to settle within the bound*: each settles at window 160 of 320.

**What the readings do not say.**
- **Nothing of whether the read's span is right**: a span that ended where the inhibitory sum stops falling would read another target. No other span was run.
- **Nothing of the inhibitory rule's target by the same read**: it stayed the derivation's, from the frozen copy.
- **Nothing of the other size**, of a longer run, or of a network the derivation's own rules do not settle: every network here is one ADR-0044's generator makes and ADR-0055's criterion settles.
- **Nothing of what the engine should carry by default**: that is the decision this round names.

### H-40's stopping rule, reached

**Step 3**: a target the engine reads serves as a derived one did. **The next decision, named and not taken**: an ADR on the engine's default for an image no derivation has settled — the rule, the step, the read and its span, and the inhibitory rule's target by the same read — then M10, a reward past the trial ([ADR-0187](0187-the-upgrade-suite-planned.md)). Steps 4 and 5 did not arise. Step 6 kept: no constant moved after a lead-in with the read set, and no other span, step or network was run.

### Consequences

- Good: the settled network's rule under the read says that the rule holds the network, not that the read held it.
- Good: a derivation under the read is held through its read by tables earlier rounds pinned: every window of the gain held's lead-in on each network.
- Good: an arm's first run is a run the host passes, its log the record, so a round's arms need no second run to be pinned.
- Bad: `arm_on` gains a branch that holds nothing pinned; it is refused for an arm half pinned, and taken by no arm but a read's before its run.
- Neutral: three derivations, six arms and one trial of eight windows join the weekly job, about 11 600 s.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `READ_SEEDS`, `READ_STEP`, `READ_SPAN`, `READ_LAST`, `READ_BOUND`, `READ_NAMES`, `UNDER_THE_READ`, `READ_ARMS`, `READ_PREDICTED`, `READ_READINGS_PREDICTED`, `READ_TARGET_DIVISOR`, `READ_COUNT_DIVISOR`; `Settling::read`, `Settling::settled`, `settled_over`, `candidate_under`, `derive_under`, `Derivation::read`; `ReadMade`, `read_target_by_hand`, `ReadRead`, `read_read`, `read_target_within`, `count_within`, `ReadArm`, `read_arm_of`, `read_predicted`; `read_verdict`, `read_stop`; `gain_held_lead_in`, `read_rate_pins`, `read_derivation`, `read_derivation_held`, `read_pins_held`, `read_derived`; `RuledSite::read`, `read_site`, `read_stand`, `read_made`, `read_arm`, `Ground::unpinned`; the trial, the gate's test and `read_tables`; the three derivations' and the six arms' weekly tests.
- Whitepaper §11.1 (H-40) and §9; `CHANGELOG.md`; `docs/adr/README.md`.
