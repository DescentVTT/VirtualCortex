---
status: accepted
date: 2026-10-17
depends-on: ADR-0182
decision-makers: VirtualCortex maintainers
---

# ADR-0183: A sixteenth, on three networks, measured — H-39 is yes, as ADR-0182 predicted: under the rate's rule at a sixteenth on seeds 22, 23 and 24 at 1 024 units, each network under its own target, clause 0 holds on every network and H-25's two arms on each hold H-25's four clauses, 119 to 126 of each mapping's last 128 correct and each reversal passed in 11 to 19 blocks, the eighteen in fewer blocks than with the gain held; the count held at each network's target with the gain held's scatter; three of the four predicted readings held and the alternation's missed, a sixteenth halving an eighth's and not removing it; the inhibitory sum ends 0.10 to 0.13 lower than with the gain held on every network; H-39's stopping rule reaches step 3, and the next decision, on what the engine carries by default, is named and not taken

## Context and Problem Statement

[ADR-0182](0182-a-sixteenth-on-three-networks.md) wrote H-39 before any run and predicts yes: under the rate's rule at a step of a sixteenth, on seeds 22, 23 and 24 at 1 024 units, each network under the target its own derivation returns with the gain held, clause 0 and H-25's four hold on every network. Brief 073 runs it once, and asks that the networks, the targets, the step, H-39's constants and the four predicted readings be committed before any lead-in is run under the rule on seed 23 or seed 24 and before any arm is run at a sixteenth.

**This ADR is written in two parts.** The protocol below was committed first (`20a286b`, pushed 2026-10-17T11:55:49Z). The readings were added after the run of each derivation and each arm, and nothing of the protocol moved then.

**A sweep has ended that no ADR has recorded**: the one of brief 072's dispatch, run [38417864748](https://github.com/DescentVTT/VirtualCortex/actions/runs/38417864748), which [ADR-0181](0181-a-rate-the-controller-holds-measured.md) and ADR-0182 named as running. This ADR records it, in its evidence ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).

What was read first (principle 2), on 2026-10-17, against `main` at `2892b29`:

1. **The arms' ground under the rule** (`runtime/cortex-runtime/tests/inhibition.rs`). `RuledSite` was the instrument's prior at a step of H-38's scan: `ruled_made` ran that step's derivation (`rate_derivation`) and held it to its pinned tables (`rate_derivation_held`), and `ruled_stand` read the arm beside H-25's with the gain held. `derive_under(units, seed, settling, name)` takes a seed, and H-38 ran it on seed 22 alone.
2. **What a derivation reads of the step and the target.** The rotation is read from the prior's census, the gain from a frozen block on the prior's network, and the cancel from probes and frozen blocks on the prior's network at that gain: none of them reads the candidate the settling builds. So on seeds 23 and 24 each is the one H-33's derivation with the gain held pinned (`seeded_site`). The lead-in's first window is the gain held's in everything but the gain its regulation leaves, as `rate_derivation_held` holds on seed 22 at every step.
3. **The targets as counts**: 134 217 728 over 58 201, 59 510 and 59 179 ticks are 2 306.107, 2 255.381 and 2 268.033 spikes a window, 2 306, 2 255 and 2 268 to nearest; the engine makes the same count of each period (`Executor::rate_target_per_window`, held by the gate).
4. **The gain held's lead-ins against those counts**, by the rate's reading, before any lead-in under the rule on either network (`SEEDED_HELD_RATE_READ`): on seed 23, 106 windows of 2 117 to 2 459 spikes, 53 above 2 255, the last sixteen 2 265.6 a window, 0.47 per cent above the target; on seed 24, 99 windows of 2 103 to 2 487, 47 above 2 268, the last sixteen 2 281.7 a window, 0.60 per cent above. On seed 22 the last sixteen stand 2.7 per cent below its target (`HELD_RATE_READ`). The settled network's rule under the rate's rule reads the two gain held's lead-ins as settled at the windows their sums are, 106 and 99.
5. **The inhibitory sum at an arm's end, of its image's**, as the tree pins the arms (`HELD_ENDS`, `RULED_ENDS`): with the gain held 0.8479 and 0.8499 on seed 22 (H-25's), 0.9082 and 0.9102 on seed 23 and 0.8958 and 0.8968 on seed 24 (H-33's); under an eighth on seed 22 0.7818 and 0.7826 (H-38's).
6. **The alternation ADR-0181 read**, 750 and 758 pairs of 959, was counted by a script over its arms' dumped windows: a pair whose two windows' counts stand one above the target and one not. The rule below gives 750 and 758 over the same dumps, brief 072's second run of the arms.

## Decision Drivers

- ADR-0182's protocol and brief 073's standing directives: three networks, one step, each network's own target; the rule run as ADR-0180 built it; no rule that derives a constant changed so that a step holds; no constant moved after such a run, and no second attempt.
- **A derivation on a network with no pin is held by its rule and its first run's tables**, as ADR-0167 held one on a new seed, and by what of it does not read the settling, which is pinned already.
- **A check added to an arm is tried first on an arm the tree already pins, on a network whose first ticks hold a spike** (brief 073; ADR-0181's arms ran twice for a check that was not).
- **Every derivation and every arm of the tree is the one it was**, H-38's six steps and two arms among them.
- A heavy run is a weekly test; the gate grows by one test ([ADR-0061](0061-the-learning-runs-leave-the-gate.md)).

## Considered Options

1. **How an arm's ground carries a network and a settling**: (a) `RuledSite` given its network, its derivation H-38's step on seed 22 and H-39's on seeds 23 and 24; (b) a `Site` given a settling; (c) a fourth ground of `arm_on`.
2. **What holds a derivation under the rule on seeds 23 and 24**: (a) the rule run again, its first run's tables pinned, and what of it does not read the settling held to the network's derivation with the gain held; (b) the first run's tables alone.
3. **What an arm on seed 23 or seed 24 is read beside**: (a) that network's own two arms with the gain held, H-33's; (b) H-25's.
4. **How its first block is held**: (a) by its own pinned tables, as H-38's arms; (b) to the first block of the network's arm with the gain held.
5. **Predicted reading 3's count**: (a) ADR-0181's rule, written as a function and read beside every arm, held on H-38's two arms at 750 and 758; (b) read on H-39's arms alone.
6. **Predicted reading 4**: (a) each arm's sum at its end as a fraction of its own image's, beside the gain held's, compared by products; (b) in parts per ten thousand.
7. **Predicted readings 2 to 4 where a network has no arm**: (a) read over the arms that ran; (b) none.
8. **The local runs**: (a) the calibration; then the two derivations and seed 22's two arms side by side; then seeds 23's and 24's four arms side by side once their derivations' tables are pinned; (b) one after another.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a), 6(a), 7(a) and 8(a).**

### The protocol (written before any lead-in under the rule on seed 23 or seed 24 and before any arm at a sixteenth)

- **The networks** (`SIXTEENTH_SEEDS`): ADR-0044's generator at 1 024 units from seeds 22, 23 and 24.
- **The step** (`SIXTEENTH_STEP`): a sixteenth, `0x1000`, on every network.
- **The targets** (`SIXTEENTH_TARGETS`, `SIXTEENTH_COUNTS`): 58 201, 59 510 and 59 179 ticks, the periods the three derivations with the gain held return (`TARGET_PERIOD_1024`, `RULES_SEEDED`), as 2 306, 2 255 and 2 268 spikes a window.
- **The settling** (`at_a_sixteenth(target)`): H-38's at a sixteenth with the network's own target — the step and the target in the candidate's configuration and in every image written from it, the rate's rule on from the lead-in's first window, the settled network's rule over the excitatory sums and the gain's mean (`settled_in_mean`), within 320 windows. On seed 22 it is `under_rate(0x1000)`, so seed 22's derivation is H-38's step of a sixteenth, which brief 072 pinned.
- **Then H-25's rules**, as `derive_under` runs them; a rule that returns nothing stops the derivation there.
- **What holds a derivation on seed 23 or seed 24** (`sixteenth_derivation_held`, option 2(a)):
  - what it makes before the settled network — the rotation and the gain as their rules return them, the couplings at each rotation scanned, the prior's sums, the calibration's blocks, the volley's census and the synapses — and, where it reached its cancel, the span, the two least counts, the pick, the candidates' frozen blocks and their probes, are the network's derivation's with the gain held (`settling_apart` against `seeded_site`'s pins);
  - its first window is the gain held's in everything but the gain its regulation left, and that gain is the rate's rule's from the gain's rule's gain, the window's spikes, the network's count and the step (`first_ruled`);
  - every window's gain is the rate's rule's (`rate_read`'s `unruled`, by the oracle `rate_regulated`); no window holds more spikes than the train's capacity; no worker dropped an entry;
  - and the tables pinned from its one run, in `RatePins`'s shape (`rate_pins_held`, which `rate_derivation_held` now calls for seed 22's).
- **The arms** (`sixteenth_arm`, `sixteenth_site`): H-25's two on H-20's schedule on each network whose clause 0 holds, from the image its derivation wrote, under the true reward, the step and the target carried through every trial; each its own weekly test. An arm's calibration is its network's derivation run again and held as the derivation's own test holds it (`ruled_made`); on a network whose clause 0 does not hold there is no arm.
- **What an arm is read beside** (option 3(a), `ruled_stand`): on seed 22 H-25's two with the gain held, as H-38's are; on seed 23 or seed 24 that network's two with the gain held, H-33's, as `site_stand` gives them. Its first block is held by its own pinned tables (option 4(a)): the image is not the gain held's, so no first block of another arm is its own.
- **H-39's clauses** are ADR-0182's, by the rules the tree holds (`sixteenth`, which is `seeded` on each network): clause 0, `Rules::hold`; clauses 1 to 4, `drawn_of` against the image that network's derivation wrote — at least 80 of each mapping's last 128 trials correct; no coupling past 1.30 of its image's; each reversal passing 40 of 64 within 23 blocks of its flip; the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's. **Yes** when all five hold in both arms on all three networks; a network whose arms did not run is no yes.
- **The stopping rule's steps** (`sixteenth_step`): 3, yes; 4, a no on clause 0 on a network; otherwise 5, a no on seed 22; otherwise 6, a no on seed 23 or seed 24 alone.
- **The prediction**: yes (`SIXTEENTH_PREDICTED`).
- **The four predicted readings as rules** (`sixteenth_predicted`), each predicted to hold (`SIXTEENTH_READINGS_PREDICTED`):
  1. the step holds on seeds 23 and 24: `Rules::hold` on the second and the third network;
  2. through each arm the gain after every window is within five per cent of 1.75, 108 954 to 120 422 both counted (`gain_within`): no window of the arm's reading outside the band (`gain_readings`);
  3. in each arm fewer than 600 pairs of consecutive windows stand on opposite sides of the target (`opposite_pairs`, `OPPOSITE_BELOW`): of each window after the first, one of it and the window before holds more spikes than the network's count and the other does not, a window at the count standing with those below it; an arm's 960 windows make 959 pairs;
  4. in each arm the inhibitory sum at its end, as a fraction of its image's, is strictly below the same at the end of the same network's same arm with the gain held (`ends_lower`, the two products in `i128`; `HELD_ENDS`).

  The second to the fourth are read over the arms that ran (option 7(a)), and are none where none did.
- **Readings, no clause**, as ADR-0182 lists them: seed 22 at a sixteenth beside seed 22 at an eighth; each network beside its own arms with the gain held; per network the derivation's readings as brief 072's scan read them, the steepness among them; over the three networks the mappings learned of twenty-four, the reversals' blocks and the inhibitory sum's end.

### The harness (built before any such run)

- **`RuledSite` is given its network** (option 1(a)): `network`, its index in `SIXTEENTH_SEEDS`. On index 0 it is what it was, any step of H-38's scan, so H-38's arms (`ruled_site`) are the arms they were; on seeds 23 and 24 a sixteenth alone. `ruled_pins`, `ruled_stand` and `ruled_made` read the network. A `Site` given a settling (option 1(b)) would hold a derivation under the rule to the gain held's whole pinned derivation, which it is not; a fourth ground (option 1(c)) would be `Ground::Ruled` written twice.
- **The derivation on seeds 23 and 24** (`sixteenth_derivation`, `sixteenth_derivation_held`, `sixteenth_pins`), each a weekly test (`the_rate_s_rule_at_a_sixteenth_on_seed_{23,24}_at_1024_units_exhaustive`), dumped whole before anything of it is held, its tables empty until its one run (`SEEDED_RATE_*`). Seed 22's is H-38's weekly test of a sixteenth, unchanged.
- **The six arms**, `the_address_drawn_at_a_sixteenth_on_seed_{22,23,24}_from_the_{assignment,mirrored_assignment}_at_1024_units_exhaustive`, their tables empty until their one run (`SIXTEENTH_*`).
- **The alternation beside every arm** (option 5(a)): `arm_on` reads `opposite_pairs` over the arm's windows wherever the engine carries a target, dumps it on every ground, and holds it where a `RuledSite` pins it (`GainPins::opposite`). H-38's two arms pin it at 750 and 758 (`RULED_OPPOSITE`), ADR-0181's readings.
- **The gate** (one test, `the_networks_of_h_39_its_clauses_and_its_predicted_readings_rules`): the networks, the targets and the counts against the engine's own, the step, the settling, the arms and the prediction; `opposite_pairs` and `ends_lower` at their edges; the verdict and the stopping rule's steps over tables written by hand; the four predicted readings' rules at their edges; the checks a derivation on seed 23 or seed 24 is held by, over the tables the tree pins (below); the tables read before the run; and, over H-39's pinned tables, each network's derivation by its rules, each arm's gain by block against its reading, and the verdict, its step and the four readings as written, none before every derivation and every arm of each network whose clause 0 holds is pinned (`sixteenth_tables`). It runs no lead-in.
- **Tables read before the run**, functions of tables the tree pins: `SEEDED_HELD_RATE_READ`, `HELD_ENDS`, `RULED_ENDS` and `RULED_OPPOSITE` (the context's items 4 to 6).

### How a check added to an arm was tried before the runs

- **The checks added to `arm_on` run in the calibration on every arm it runs**, the eighteen the tree pins: the alternation is read in all eighteen and held in H-38's two, whose first 500 ticks hold two spikes each — the edge ADR-0181's watch missed — to the 750 and 758 that brief 072's dumps of them read by this rule before this commit.
- **The checks a derivation on seed 23 or seed 24 is held by are tried in the gate over the tables the tree pins**: `settling_apart` over each network's derivation with the gain held, given back from its pins (`pinned_derivation`), agrees with itself, names a census changed and a probe dropped, holds no cancel where the derivation stopped before one, and tells the two networks apart; `first_ruled` holds seed 22's pinned first windows at a sixteenth and at an eighth against the gain held's, refuses an eighth's as a sixteenth's and a sixteenth's read against another network's count, and holds each of seeds 23's and 24's gain held's first windows against themselves with the gain moved at their end by the rule.
- **The ground on seed 23 or seed 24** reads its own arms' tables beside from `site_stand`, the tables H-36's arms read beside on those networks every week.
- **What cannot be tried before** is the derivation under the rule on seed 23 or seed 24 itself: its one run is its trial, and a check of it that fails there costs that derivation's minutes and no arm's.

### The readings

Everything below was read on the developer machine on 2026-10-17, from one run of each derivation and each arm, after the protocol above was committed and pushed (`20a286b`, 11:55:49Z). A developer machine's seconds are not Measured figures ([ADR-0010](0010-measured-or-target.md)); the tables are integers of a deterministic engine, which the dispatched weekly reproduces.

**The calibration** (H-39's stopping rule, step 2). The tree's 128 whole-domain tests but this round's eight, each in a process of its own from the release build at `20a286b`, eight at a time, from 11:58:20Z to 16:45:33Z: 128 of 128 passed. So every pinned number of the tree holds, H-38's six steps and two arms among them, and seed 22's derivation at a sixteenth is the one brief 072 pinned. Within it the check this round added to an arm ran on every arm `arm_on` runs: the alternation was read in all eighteen, held in H-38's two at 750 and 758 — the arms whose first 500 ticks hold two spikes each — and read as none in the sixteen whose engine carries no target.

**The derivations**, seeds 23's and 24's, from 16:46:18Z, side by side with seed 22's two arms, 251 and 229 s. Each passed every check it is held by — everything before its settled network and its cancel the gain held's, its first window the gain held's with the gain moved at its end by the rule, every window's gain the rule's, every window inside the train — and stopped at its empty tables after everything was dumped; its tables were pinned from that run (`9d23424`). **Clause 0 holds on all three networks.**

| Network | Target, a window | Settled at window | Gain's band over the lead-in | Spikes a window | Last sixteen windows, a window | Windows above the target | Carried gain | Frozen block: seen of 64 | Gain after it | Inhibitory target period | Steepness: first step, table |
| :--- | ---: | ---: | :--- | :--- | ---: | ---: | :--- | ---: | :--- | ---: | :--- |
| Seed 22 | 2 306 | 96 | 1.7419 to 1.7599 | 2 067 to 2 507 | 2 303 | 46 of 96 | 1.7505 | 62 | 1.7259 | 58 188 | 15.0, 16.7 |
| Seed 23 | 2 255 | 105 | 1.7400 to 1.7561 | 1 991 to 2 486 | 2 253 | 53 of 105 | 1.7460 | 64 | 1.7264 | 59 672 | 21.4, 16.1 |
| Seed 24 | 2 268 | 93 | 1.7394 to 1.7571 | 1 993 to 2 521 | 2 267 | 45 of 93 | 1.7482 | 64 | 1.7221 | 59 280 | 16.0, 15.3 |

Seed 22's row is brief 072's pinned derivation, its first step's steepness ADR-0181's.

- **Every lead-in settles where its sums do**: at windows 105 and 93 on seeds 23 and 24, where with the gain held they settle at 106 and 99. The gain's mean's criterion holds from the thirty-second window on all three.
- **The count is held to its target.** Over the last sixteen windows of each lead-in it stands 0.14, 0.08 and 0.04 per cent below its target, where the gain held's last sixteen stand 2.7 per cent below on seed 22 and 0.47 and 0.60 per cent above on seeds 23 and 24. A window holds 1 991 to 2 521 spikes under the rule; with the gain held, 2 117 to 2 459 on seed 23 and 2 103 to 2 487 on seed 24.
- **The steepness is fifteen to twenty-one.** At the first step the gain leaves the first window 0.26 per cent lower on seed 23 and the second holds 5.6 per cent fewer spikes than the gain held's, 21.4 per cent of rate for one of gain; on seed 24, 0.60 per cent lower and 9.7 per cent fewer, 16.0. By the tables' slope the three read 16.7, 16.1 and 15.3. By ADR-0178's arithmetic a sixteenth's loop reaches its edge at a steepness of 32; at 21.4 a deviation of the gain stands at $-0.34$ of itself a window later.
- **With the weights frozen the count stays bounded.** The target's lead-in, the settled image frozen under the drive for eight windows, holds 2 141 to 2 393 spikes on seed 23 and 2 128 to 2 508 on seed 24, alternating without growing; with the gain held, 2 153 to 2 348 and 2 185 to 2 354. Seed 22's at a sixteenth holds 2 207 to 2 381, where an eighth's grew to 2 964 and 1 585.
- **Every later rule gives a value.** The cancel is the gain held's on each network, the frozen block sees the stimulus in 64 trials of 64 on both, and the inhibitory target periods derived under the rule, 59 672 and 59 280 ticks, stand 0.27 and 0.17 per cent above the gain held's, where seed 22's stands 0.02 per cent below.
- **No window comes near the saturation rule's count**: the greatest of the 294 holds 2 521 spikes of 32 768, and in every window the gain is the oracle's.

**The arms.** Seed 22's two from 16:46:18Z, from the protocol's build, 2 051 and 2 050 s; seeds 23's and 24's four from 16:52:31Z, from `9d23424`'s, 2 060 to 2 081 s. Each first ran its network's derivation again and held it to the tables pinned for it, then ran once, passed every check before its tables — the watch's count of the open window the record's at every trial, 960 windows, every window's gain the rule's from the network's own count, the critic's step the engine's, the train holding every trial — and stopped at its empty tables after everything was dumped. Its tables were pinned from that run (`ecd78cf`).

Seed 22, at a sixteenth beside an eighth and beside H-25's arms with the gain held:

| | A sixteenth, from the assignment | An eighth | The gain held | A sixteenth, from the mirrored | An eighth | The gain held |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Correct of each mapping's last 128 | 122, 121, 126, 123 | 122, 123, 118, 124 | 122, 124, 119, 116 | 124, 121, 119, 122 | 118, 118, 124, 119 | 124, 118, 123, 117 |
| First mapping passes 40 of 64 at block | 6 | 6 | 6 | 4 | 4 | 4 |
| Reversals, blocks from the flip | 15, 19, 16 | 18, 19, 16 | 19, 21, 16 | 19, 13, 14 | 15, 15, 15 | 20, 15, 14 |
| Highest coupling, of the image's | 1.203 | 1.191 | 1.181 | 1.206 | 1.220 | 1.191 |
| Spikes a window: mean, least to greatest, deviation | 2 306.7, 2 080 to 2 842, 75 | 2 306.1, 1 881 to 2 882, 127 | | 2 306.8, 2 074 to 2 846, 75 | 2 306.0, 1 880 to 2 876, 133 | |
| Gain: mean, band | 1.7206, 1.7096 to 1.7309 | 1.7215, 1.6993 to 1.7401 | 1.7500 | 1.7201, 1.7079 to 1.7316 | 1.7210, 1.6985 to 1.7407 | 1.7500 |
| Pairs on opposite sides of the target, of 959 | 597 | 750 | | 607 | 758 | |
| Inhibitory sum at the end, of its image's | 0.745 | 0.782 | 0.848 | 0.743 | 0.783 | 0.850 |

Seeds 23 and 24, each beside its own arms with the gain held, H-33's:

| | Seed 23, from the assignment | The gain held | Seed 23, from the mirrored | The gain held | Seed 24, from the assignment | The gain held | Seed 24, from the mirrored | The gain held |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| Correct of each mapping's last 128 | 124, 120, 125, 125 | 126, 124, 124, 123 | 121, 120, 122, 124 | 122, 115, 118, 126 | 121, 124, 124, 124 | 117, 123, 120, 115 | 124, 120, 120, 126 | 124, 120, 121, 123 |
| First mapping passes 40 of 64 at block | 3 | 4 | 3 | 3 | 1 | 1 | 7 | 8 |
| Reversals, blocks from the flip | 13, 13, 14 | 15, 13, 13 | 12, 13, 13 | 16, 14, 12 | 17, 14, 13 | 21, 12, 15 | 14, 11, 14 | 17, 16, 14 |
| Highest coupling, of the image's | 1.198 | 1.183 | 1.215 | 1.182 | 1.209 | 1.195 | 1.207 | 1.191 |
| Spikes a window: mean, least to greatest, deviation | 2 255.6, 2 027 to 2 646, 75 | 2 847.3, 2 618 to 3 091, 74 | 2 255.7, 2 013 to 2 646, 74 | 2 843.1, 2 632 to 3 078, 72 | 2 268.6, 2 023 to 2 604, 74 | 2 846.7, 2 641 to 3 105, 76 | 2 268.6, 2 032 to 2 602, 74 | 2 848.4, 2 661 to 3 097, 78 |
| Gain: mean, band | 1.7174, 1.7057 to 1.7272 | 1.7500 | 1.7167, 1.7051 to 1.7271 | 1.7500 | 1.7184, 1.7069 to 1.7321 | 1.7500 | 1.7186, 1.7065 to 1.7322 | 1.7500 |
| Pairs on opposite sides of the target, of 959 | 607 | | 619 | | 614 | | 608 | |
| Inhibitory sum at the end, of its image's | 0.782 | 0.908 | 0.780 | 0.910 | 0.776 | 0.896 | 0.776 | 0.897 |

In every arm the outside sum stays within 0.9999 and 1.0001 of its image's, and the stimulus fires once in each of the 120 blocks.

**Through each arm the controller reads:**
- **The count is the target's**, 0.03 per cent above it in every arm: 2 306.7 and 2 306.8 spikes a window on seed 22, 2 255.6 and 2 255.7 on seed 23, 2 268.6 twice on seed 24. With the gain held the task holds 2 843 to 2 848 a window on seeds 23 and 24, a quarter above their targets, and 2 814 to 2 838 on seed 22 (ADR-0181).
- **The gain stands 1.6 to 1.7 per cent below its image's**: means 1.7167 to 1.7206 against carried gains of 1.7460 to 1.7505, its band 1.7051 to 1.7322 over the six arms, 1.0 to 2.6 per cent below 1.75 after every window.
- **A window's count scatters as it does with the gain held**: a deviation of 74 or 75 spikes in all six arms, where the gain held's arms on seeds 23 and 24 read 72 to 78, and an eighth's on seed 22 127 and 133.
- **It alternates, half as strongly as an eighth.** Consecutive windows stand on opposite sides of the target in 597 to 619 pairs of 959. A window's count is correlated with the window's before it at $-0.36$ to $-0.39$; under an eighth at $-0.79$ and $-0.80$; with the gain held at $+0.16$ to $+0.30$ on seeds 23 and 24 and $+0.08$ on seed 22, and there a window's count crosses its arm's mean in 385 to 446 pairs of 959.
- **A reversal does not move it**: the gain stands at 1.7134 to 1.7232 at the eighteen flips, and over the twenty-three blocks after each within 1.7079 and 1.7316.
- **Every window's gain is the oracle's**, and the train's count of the open window is the record's own at every trial's end.

**The inhibitory sum falls further under the rule, on every network.** It ends at 0.743 to 0.782 of its image's, 0.10 to 0.13 below the same network's arm with the gain held in each of the six, and on seed 22 lower at a sixteenth, 0.745 and 0.743, than at an eighth, 0.782 and 0.783. Under the rule it falls from its first or second block on; with the gain held it rises first and peaks at the eleventh to the nineteenth block. ADR-0181's account stands, and this round did not vary it: the gain holds the population's count at the settled rate while the task has its own units fire more, so the rest fire below the rate the inhibitory rule drives each unit toward. What clause 2 and clause 4 read does not move: the highest coupling is 1.215 of its image's at most and the outside sum stays at its image's.

**Over the three networks**: every mapping of twenty-four learned, 119 to 126 of its last 128 correct, where the same networks with the gain held read 115 to 126; every reversal of eighteen passed, in 11 to 19 blocks, the eighteen summing to 257 blocks where with the gain held they sum to 283 — 96 against 105 on seed 22, 78 against 83 on seed 23 and 83 against 95 on seed 24; under an eighth seed 22's sum to 98.

The line `arm_on` dumps as the inhibitory course *beside the network's own under the true reward* reads, on a `RuledSite`, the gain held's arm's blocks over this arm's image's sum and not its own image's: on seed 22 it gives 0.852 and 0.854 at the end where the gain held's own fraction is 0.848 and 0.850, and on seeds 23 and 24 0.903 to 0.905 and 0.879 and 0.880 where it is 0.908 to 0.910 and 0.896 and 0.897. Predicted reading 4 reads each arm against its own image (`HELD_ENDS`), and the tables above do; the dumped line is a reading of no clause and is not changed after the runs.

### H-39's verdict

**Yes.** By `sixteenth` over the three networks' pinned derivations and the six arms' pinned blocks, each against the image its network's derivation wrote:
- **clause 0**, on every network: the lead-in settles, every later rule gives a value and the frozen block passes both measures;
- **clause 1**: every mapping learned, 119 to 126 of each mapping's last 128 where 80 are asked;
- **clause 2**: no coupling past 1.30 of its image's, the highest 1.215;
- **clause 3**: each reversal passes 40 of 64 within 23 blocks of its flip, in 11 to 19;
- **clause 4**: the outside sum within 0.75 and 1.25 of its image's, at 0.9999 to 1.0001.

**The prediction was yes, and it held.** The four predicted readings:

| | Predicted | Read |
| :--- | :--- | :--- |
| 1. The step holds on seeds 23 and 24 | holds | **held**: they settle at windows 105 and 93, the stimulus seen in 64 trials of 64 on each |
| 2. The gain within five per cent of 1.75 through each arm | 1.6625 to 1.8375 | **held**: 1.7051 to 1.7322 over 5 760 windows |
| 3. Fewer than 600 pairs of 959 on opposite sides of the target in each arm | below 600 | **missed**: 597, 607, 607, 619, 614 and 608 — one arm of six below 600 |
| 4. The inhibitory sum ending lower than with the gain held in each arm | lower | **held**: 0.743 to 0.782 against 0.848 to 0.910 |

**The missed reading.** ADR-0182's arithmetic gave the decay of a deviation of the gain: at a sixteenth $+0.06$ of itself a window later, so that a sixteenth would not alternate as an eighth does. What the reading counts is a window's spikes, and a window's count moves the gain the next window runs under by the step whatever the gain's deviation. With the gain held a window's count follows the one before it, at $+0.08$ to $+0.30$; under the rule the gain answers it, and the next window stands the other way, at $-0.36$ to $-0.39$ under a sixteenth and $-0.79$ and $-0.80$ under an eighth. A sixteenth halves the alternation and keeps the count's scatter at the gain held's; it does not remove the alternation. This is an account of the tables, and the round did not vary it: no other step, target or window was run.

**What ADR-0182 said would refute it, against what was read.**
- *A network so much steeper that a sixteenth is at its edge there*: the steepest first step reads 21.4, on seed 23, where a sixteenth's edge is 32; by the tables' slope the three read 15.3 to 16.7.
- *The inhibitory sum, falling further under the rule, moving the couplings*: it falls to 0.743 to 0.782 of its image's, and the couplings stay within 1.215 of theirs, the outside sum at its image's.
- *A sixteenth's slower answer letting a reversal run past twenty-three blocks*: every reversal passes in 11 to 19 blocks, and the eighteen take fewer blocks than with the gain held.

**What the readings do not say.**
- **Nothing of the other size**, the two networks at 2 048 units.
- **Nothing of a target for a network no derivation has settled**: each target here is the period its own derivation with the gain held returned.
- **Nothing of a longer run**: an arm is 960 windows, and the inhibitory sum is still falling at its end on every network.
- **Nothing of why the inhibitory sum falls further**, or of what the alternation's account would read under another step: neither was varied.

### The evidence

- **The dispatch's scope.** The diff changes no file under `src/`, `.cargo/mutants.toml` or the workflow, and removes no test from the gate: by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the weekly was dispatched on this round's branch at **`scope=exhaustive`**, run [39054186063](https://github.com/DescentVTT/VirtualCortex/actions/runs/39054186063) at `ecd78cf`, the arms' tables' commit, at 17:32:11Z. The round's commits, in pull request #210: `20a286b` the protocol, before any lead-in under the rule on seed 23 or 24 and any arm at a sixteenth; `9d23424` the two derivations' tables, from which seeds 23's and 24's arms were built; `ecd78cf` the six arms' tables and the verdict, its step and the readings as read; the commit that carries these readings and the documents; and the commit that carries this evidence, which changes no test.
- **Every whole-domain shard is green.** All 136 `exhaustive` tests passed on the hosted runners, each exit 0 with one test reported, 107 093 s summed. The 128 before this round reproduced their pinned numbers, H-38's six steps and two arms among them, the two holding the alternation at 750 and 758, and the sixteen arms with the gain held reading none. **This round's eight reproduced their tables there**: the derivations on seeds 23 and 24 in 236 and 216 s; the arms on seed 22 in 1 814 and 1 808 s, on seed 23 in 1 847 and 1 853 s and on seed 24 in 1 059 and 1 785 s, each after deriving its network again and holding the derivation to its tables.
- **The shards**, dealt by the table before this round, which did not know the eight and costed each at 900 s:

  | Shard | Job, of its bound | Its two heaviest tests (s) | Tests | Their seconds summed | Tests' wall time, of the bound |
  | ---: | ---: | :--- | ---: | ---: | ---: |
  | 0 | 92.0 min, 51 % | `the_address_drawn_on_seed_22_from_the_assignment_at_2048_units` 3 626; `the_address_drawn_at_a_sixteenth_on_seed_22_from_the_mirrored_assignment_at_1024_units` 1 808 | 9 | 10 837 | 5 437 s, 50 % |
  | 1 | 56.4 min, 31 % | `the_address_drawn_on_seed_23_from_the_assignment_at_2048_units` 2 139; `the_span_a_choice_among_three_is_given_from_the_assignment_at_1024_units` 1 175 | 11 | 6 200 | 3 320 s, 31 % |
  | 2 | 74.2 min, 41 % | `the_address_drawn_on_seed_22_from_the_mirrored_assignment_at_2048_units` 2 822; `a_reward_right_three_times_in_four_on_seed_24_from_the_mirrored_assignment_at_1024_units` 1 454 | 12 | 8 636 | 4 381 s, 41 % |
  | 3 | 75.0 min, 42 % | `the_address_drawn_on_seed_23_from_the_mirrored_assignment_at_2048_units` 2 840; `three_answers_from_the_mirrored_assignment_at_1024_units` 1 399 | 11 | 8 816 | 4 430 s, 41 % |
  | 4 | 70.8 min, 39 % | `the_span_a_choice_among_three_is_given_from_the_mirrored_assignment_at_1024_units` 2 101; `a_reward_right_three_times_in_four_on_seed_24_from_the_assignment_at_1024_units` 1 445 | 12 | 8 113 | 4 184 s, 39 % |
  | 5 | 73.2 min, 41 % | `the_address_drawn_from_the_mirrored_assignment_at_1024_units` 1 957; `a_reward_right_three_times_in_four_on_seed_23_from_the_assignment_at_1024_units` 1 861 | 11 | 8 610 | 4 311 s, 40 % |
  | 6 | 81.8 min, 45 % | `the_address_drawn_under_the_rate_s_rule_from_the_mirrored_at_1024_units` 1 801; `a_reward_right_three_times_in_four_from_the_mirrored_assignment_at_1024_units` 1 789 | 12 | 9 430 | 4 832 s, 45 % |
  | 7 | 67.0 min, 37 % | `a_reward_right_seven_times_in_eight_from_the_mirrored_assignment_at_1024_units` 1 794; `a_punishment_the_value_does_not_soften_from_the_mirrored_assignment_at_1024_units` 1 792 | 12 | 7 879 | 3 941 s, 36 % |
  | 8 | 66.7 min, 37 % | `an_exploration_the_value_gates_from_the_assignment_at_1024_units` 1 786; `the_schedule_of_reversals_from_the_assignment_at_1024_units` 1 782 | 11 | 7 722 | 3 925 s, 36 % |
  | 9 | 88.1 min, 49 % | `the_address_drawn_on_seed_23_from_the_assignment_at_1024_units` 1 880; `the_address_drawn_at_a_sixteenth_on_seed_23_from_the_mirrored_assignment_at_1024_units` 1 853 | 11 | 10 084 | 5 207 s, 48 % |
  | 10 | 94.1 min, 52 % | `three_answers_from_the_assignment_at_1024_units` 1 929; `the_address_drawn_on_seed_23_from_the_mirrored_assignment_at_1024_units` 1 843 | 12 | 10 897 | 5 570 s, 52 % |
  | 11 | 88.2 min, 49 % | `a_reward_right_three_times_in_four_on_seed_23_from_the_mirrored_assignment_at_1024_units` 1 862; `the_address_drawn_on_seed_24_from_the_assignment_at_1024_units` 1 852 | 12 | 9 869 | 5 210 s, 48 % |

  The percentages are of the job's bound, 180 minutes ([ADR-0179](0179-the-bounds-rule-restated.md)); each shard ran its tests two at a time, at 1.87 to 2.00 of their summed seconds over the wall. The twelve started within a second of each other, four seconds after the dispatch, and the last ended at 19:06:22Z, 1 h 34 min after it. **No shard's job passed 60 per cent of its bound**: the most is 94.1 minutes, 52 per cent, the slowest shard's job the tree has read, three minutes past the 91.0 of brief 072's dispatch; by ADR-0179's rule the bound stays at 180 minutes.
- **The cost table is regenerated from this run** (`node scripts/exhaustive-costs.mjs from <artifacts> --run 39054186063`): 136 lines, 107 093 s, where it held 128 and 101 602. The 128 before this round summed to 96 475 s here, a twentieth less than the last run read them at; this round's eight are 10 618 s of the rest. ADR-0092's deal plans each of the twelve shards at 8 923 to 8 926 s summed, about 4 550 s of wall time at this run's ratios: **42 per cent of the bound**. No shard is planned past 60.
- **The sweep of brief 072's dispatch, written down here** as [ADR-0158](0158-the-sweep-off-the-next-decisions-path.md) asks of the first ADR that merges after a sweep has ended. Run [38417864748](https://github.com/DescentVTT/VirtualCortex/actions/runs/38417864748), dispatched on brief 072's branch at `384738c` (`scope=both`), the tree with ADR-0180's rule and format 22: its thirteen jobs started at 16:28:40Z on 2026-10-13, four seconds after its last whole-domain shard, and the last ended at 19:15:28Z, green in all thirteen, after ADR-0182 had merged and before this ADR.
  - **3 883 mutants over the tree, 3 706 caught and none missed**: 2 416 caught in the state crates and 1 290 in the runtime, 154 unviable and 23 timeouts. Thirty-four more than the 3 849 of the schedule's sweep ADR-0174 records, the rule and the format ADR-0180 added under `src/`.
  - **Twenty-two of the timeouts** are in the places [ADR-0062](0062-the-first-complete-sweeps-list.md) reads as inherent: `cortex-core`'s three chain iterators, six; the injector, twelve; the barrier's wait, one; the workers' stop and run, three.
  - **The twenty-third is of F-42's kind**, by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence: `image.rs:826:13: delete field clauses from struct Config expression in Image::decode`, in `runtime-11`, 1 872 s against a bound of 1 848, 1 557 of them beside a hung mutant of the injector (`injector.rs:93:40: replace & with | in Injector::pop`) in the shard's other slot. Its log ends inside `reference`, four of its long tests running; the tests that catch it are `store`'s, the binary after it, which never ran. The schedule's sweep of run 38234325824 caught the same mutant, at `image.rs:805:13` before ADR-0180 moved the line, in `runtime-10`: `reference` passed under it in 173 s and `store` failed three tests on a malformed clause, 608 s into its test phase.
  - **Its twelve shards over the runtime took 1 h 09 m to 2 h 47 m each**, and the state crates' 22 m.
  - There is no survivor, so it leaves no finding.
- **The pull request's gate** on `20a286b`, on `9d23424` and on `5270893` is green in every job, the determinism pin on AArch64 and the MSRV among them; the gate on `ecd78cf` was cancelled by the push of `5270893`, which changes documents and whose tests are its. **The mutation gate on the changed lines** has nothing to mutate: no line under `src/` changed, and the job passes in 19 to 22 s.
- **On the developer machine**, at `ecd78cf`: the workspace's tests passed in the debug profile, in the release profile and on the MSRV, 753 passed and 176 ignored in each; the check, the format, Clippy, the documentation, the benchmarks and `npm run spec` exited 0; 136 whole-domain tests are listed; and `cargo mutants --in-diff` over the round's diff found no mutant to make. The derivations and the arms were not run a second time there: each ran once, each arm's calibration ran its network's derivation again, and the dispatch is their reproduction.

### The step of the stopping rule reached

**Step 3: yes.** The configuration learns and revises under the rate's rule at a sixteenth on the three networks of the size, each under its own target.

**The next decision, named and not taken**, is ADR-0182's: *"an ADR on what the engine carries by default — the rule, the step and a target's rule for a network no derivation has settled — and on the other size and the questions left, a reward delayed past a trial among them."* What this round's readings hand it:
- a sixteenth learns and revises on all three networks, every clause held, the reversals no slower than with the gain held, and holds each network's count at its target with the gain held's scatter;
- it halves an eighth's alternation and does not remove it, a window's count correlated with the one before at $-0.36$ to $-0.39$;
- the steepness reads fifteen to twenty-one on these networks, inside a sixteenth's loop;
- under the rule the inhibitory sum ends 0.10 to 0.13 lower than with the gain held, on every network, and lower at a sixteenth than at an eighth;
- each target here is the period the network's own derivation with the gain held returned, and the inhibitory target period a derivation under the rule returns stands within 0.3 per cent of it.

### Consequences

- Good: the arms' ground carries a network and a settling, and every arm and derivation of the tree is the one it was.
- Good: a derivation on a network with no pin under the rule is held by more than its first run: what of it the settling does not reach is held to that network's pinned derivation with the gain held.
- Good: the alternation ADR-0181 read by a script is a rule in the tree, held on H-38's arms each week.
- Good: H-38's yes is asked of the step a default would carry and of the three networks of the size, and holds on each.
- Bad: `arm_on` dumps one line more on every ground.
- Bad: a predicted reading was written from an arithmetic of the gain's deviation and read of the window's count, and missed; the account of the miss is the tables', not varied.
- Bad: the inhibitory sum's fall under the rule is read again on three networks and still not varied.
- Neutral: six arms of thirty to forty minutes and two derivations of four to five join the weekly job, about 12 000 s.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `SIXTEENTH_SEEDS`, `SIXTEENTH_STEP`, `SIXTEENTH_TARGETS`, `SIXTEENTH_COUNTS`, `at_a_sixteenth`, `SIXTEENTH_PREDICTED`, `SIXTEENTH_READINGS_PREDICTED`, `OPPOSITE_BELOW`; `opposite_pairs`, `inhibitory_end`, `ends_lower`, `SixteenthArm`, `sixteenth_predicted`; `Sixteenth`, `sixteenth`, `sixteenth_step`; `settling_apart`, `first_ruled`, `sixteenth_pins`, `sixteenth_derivation`, `sixteenth_derivation_held`, `rate_pins_held`; `RuledSite::network`, `ruled_pins`, `ruled_stand`, `ruled_made`, `sixteenth_site`, `sixteenth_arm`; `GainPins::opposite`, `RULED_OPPOSITE`; the gate's test and `sixteenth_tables`; `SEEDED_HELD_RATE_READ`, `HELD_ENDS`, `RULED_ENDS`; the two derivations' and the six arms' weekly tests.
- Whitepaper §5.2.16, §8.8, §11.1 (H-39 with its verdict and step) and §9; `CHANGELOG.md`; `CLAUDE.md`; `README.md`; `docs/zh-TW/README.md`; `docs/adr/README.md`.
