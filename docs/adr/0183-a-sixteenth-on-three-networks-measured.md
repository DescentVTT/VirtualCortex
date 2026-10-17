---
status: proposed
date: 2026-10-17
depends-on: ADR-0182
decision-makers: VirtualCortex maintainers
---

# ADR-0183: A sixteenth, on three networks, measured — H-39's protocol, written before any lead-in is run under the rate's rule on seed 23 or seed 24 and before any arm is run at a sixteenth: the rate's rule at a sixteenth on seeds 22, 23 and 24 at 1 024 units, each network under its own target; a derivation on seed 23 or seed 24 held by the rule, by its first run's tables and by its network's derivation with the gain held; an arm's ground given its network; the four predicted readings as rules over the tables, the alternation's count among them and held on H-38's arms first; the readings follow the run

## Context and Problem Statement

[ADR-0182](0182-a-sixteenth-on-three-networks.md) wrote H-39 before any run and predicts yes: under the rate's rule at a step of a sixteenth, on seeds 22, 23 and 24 at 1 024 units, each network under the target its own derivation returns with the gain held, clause 0 and H-25's four hold on every network. Brief 073 runs it once, and asks that the networks, the targets, the step, H-39's constants and the four predicted readings be committed before any lead-in is run under the rule on seed 23 or seed 24 and before any arm is run at a sixteenth.

**This ADR is written in two parts.** The protocol below is committed first. The readings are added after the run of each derivation and each arm, and nothing of the protocol moves then.

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

Added after the one run of each derivation and each arm.

### Consequences

- Good: the arms' ground carries a network and a settling, and every arm and derivation of the tree is the one it was.
- Good: a derivation on a network with no pin under the rule is held by more than its first run: what of it the settling does not reach is held to that network's pinned derivation with the gain held.
- Good: the alternation ADR-0181 read by a script is a rule in the tree, held on H-38's arms each week.
- Bad: `arm_on` dumps one line more on every ground.
- Neutral: six arms of thirty to forty minutes and two derivations of four to five join the weekly job, about 12 000 s.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `SIXTEENTH_SEEDS`, `SIXTEENTH_STEP`, `SIXTEENTH_TARGETS`, `SIXTEENTH_COUNTS`, `at_a_sixteenth`, `SIXTEENTH_PREDICTED`, `SIXTEENTH_READINGS_PREDICTED`, `OPPOSITE_BELOW`; `opposite_pairs`, `inhibitory_end`, `ends_lower`, `SixteenthArm`, `sixteenth_predicted`; `Sixteenth`, `sixteenth`, `sixteenth_step`; `settling_apart`, `first_ruled`, `sixteenth_pins`, `sixteenth_derivation`, `sixteenth_derivation_held`, `rate_pins_held`; `RuledSite::network`, `ruled_pins`, `ruled_stand`, `ruled_made`, `sixteenth_site`, `sixteenth_arm`; `GainPins::opposite`, `RULED_OPPOSITE`; the gate's test and `sixteenth_tables`; `SEEDED_HELD_RATE_READ`, `HELD_ENDS`, `RULED_ENDS`; the two derivations' and the six arms' weekly tests.
- Whitepaper §11.1 (H-39) and §9; `CHANGELOG.md`; `docs/adr/README.md`.
