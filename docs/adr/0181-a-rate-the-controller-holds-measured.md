---
status: proposed
date: 2026-10-13
depends-on: ADR-0180
decision-makers: VirtualCortex maintainers
---

# ADR-0181: A rate the controller holds, measured — H-38's protocol, written before any lead-in is run under the rate's rule: six steps on seed 22 at 1 024 units with the controller holding 58 201 ticks a spike, the settled network's rule over the excitatory sums and the gain's mean of sixteen windows as an integer rule, the four predicted readings as rules over the tables, and H-25's two arms under the largest step that holds with the gain read window by window; the readings follow the run

## Context and Problem Statement

[ADR-0178](0178-a-rate-the-controller-holds.md) gave the controller a second rule and wrote H-38 before any run, predicting yes. [ADR-0180](0180-a-rate-the-controller-holds-built.md) built the rule. Brief 072 runs H-38 once, and asks that its clauses, its constants, the lead-in's bound and criteria and the four predicted readings be committed before any lead-in is run under the rate's rule.

**This ADR is written in two parts.** The protocol below is committed first. The readings are added after the one run of each step and each arm, and nothing of the protocol moves then.

**No sweep is owed.** ADR-0174 records the last that ended, and brief 071's dispatch ran the whole-domain shards alone.

What was read first (principle 2), on 2026-10-13:

1. **H-37's harness** (`runtime/cortex-runtime/tests/inhibition.rs`). `derive_under(units, seed, settling, name)` runs H-25's rules in ADR-0165's order under a `Settling`, which carried the controller's step, whether the settled network's rule reads each window's gain, and the bound. Its candidate's engine is built by `candidate_from` with the step in its configuration; the image `at_gain` reloads it from carries what the configuration held.
2. **No arm has run under a step.** ADR-0177's five steps stopped at the settled network's rule or at the frozen block. `arm_on` runs an arm on the instrument's network or on a `Site`, and its calibration holds a site's derivation to tables an earlier round pinned whole. It asserts the gain at the load and reads it nowhere after.
3. **A trial and a window.** A trial is 16 384 ticks and a window 131 072, so a block of sixty-four trials is eight windows. An image is written a quiet run after its lead-in's last window, so an arm's trials do not begin at a window's first tick: a window ends inside one trial in eight.
4. **The record's count of its open window** is its closed bins' spikes and its open bin's (`HomeostaticDrivePool::window_spikes`, `bin_activity`), which an image carries.
5. **The gain held's lead-in against H-38's target**, by the rule below, before any lead-in under the rate's rule: ninety-six windows of 2 132 to 2 462 spikes, thirty-two of them above 2 306, and 35 897 spikes over the last sixteen, 2 244 a window, 2.7 per cent below the target. The target is the settled image's rate frozen; the lead-in's last windows, its weights still moving, fire a little below it.

## Decision Drivers

- ADR-0178's protocol and brief 072's standing directives: one network, one target, six steps in their order, a bound of 320 windows; no rule that derives a constant changed so that a step holds; no constant moved after a lead-in is run under the rate's rule, and no second attempt.
- **The rule that reads a gain as settled must not read a swing as drift**, which is why ADR-0178 holds the gain to its mean; and it must be an integer rule with its edges written.
- **A run under the rule is held to the rule window by window**, in the lead-ins and in the arms, by an oracle that is not the record's code.
- **Every derivation and every arm of the tree is the one it was**: the harness that stands is given the target, and the calibration runs before any lead-in under the rule.
- A heavy run is a weekly test; the gate grows by one test for the measurement ([ADR-0061](0061-the-learning-runs-leave-the-gate.md)).

## Considered Options

1. **How the derivation takes the rule**: (a) the `Settling` carries the target and whether the rule reads the gain's mean, and `derive_under` builds its candidate under it; (b) a second derivation for the rate's rule.
2. **The gain's mean where the lead-in is shorter than two blocks of sixteen**: (a) both blocks whole windows of the lead-in, so no window below the thirty-second holds the criterion; (b) the prior's gain standing for the windows before the first.
3. **The arms' ground**: (a) a third ground of `arm_on`, the instrument's prior derived under the rate's rule, its calibration the step's own pinned derivation; (b) a `Site` given a settling.
4. **What an arm is read beside**: (a) H-25's two arms with the gain held, the tables the tree pins; (b) nothing.
5. **How the gain is read through an arm**: (a) at every trial's end, with each window's spikes from the train and the window's regulation held to the rule; (b) at each block's end.
6. **Whether every arm of the tree reads it**: (a) every arm run by `arm_on`, on every ground; (b) H-38's alone.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a), 5(a) and 6(a).**

### The protocol (written before any lead-in under the rate's rule)

- **The network**: ADR-0044's prior at 1 024 units from seed 22 (`SCAN_UNITS`, `PRIOR_SEED`).
- **The target**: 58 201 ticks (`RATE_TARGET_PERIOD`, which is `TARGET_PERIOD_1024`), 2 306 spikes a window at 1 024 units by ADR-0180's rule (`RATE_TARGET_COUNT`; the gate holds an engine's own count to it).
- **The steps**, in the order run: an eighth, a sixteenth, one in 32, one in 64, one in 128 and one in 256 (`RATE_STEPS`).
- **The settling** (`under_rate(step)`): the controller's step and the target in the candidate's configuration and so in every image written from it; the rate's rule on from the lead-in's first window; the bound 320 windows (`RATE_BOUND`).
- **The settled network's rule under the rate's rule** (`settled_in_mean`): the first window $w$, counted from one, at which both hold:
  - ADR-0055's criterion over the excitatory sums (`settled_in`), as it stands;
  - the gain's mean's criterion (`mean_settled_in`): with $A$ the gains after windows $w - 15$ to $w$ summed and $B$ the gains after windows $w - 31$ to $w - 16$ summed, $10\,000\,|A - B| < 75\,B$. The two means are over one count of windows, so their sums stand for them; the comparison is strict, as ADR-0055's is; a gain is the gain its window's regulation left.
  - **Both blocks are whole windows of the lead-in** (option 2(a)), so the rule gives no window below the thirty-second. ADR-0178 names *"the sixteen before"*; before the sixteenth window there are not sixteen, and a prior's gain standing for windows that did not run (option 2(b)) would hold a mean over one number sixteen times.
  - It returns nothing where no window of the 320 holds both.
- **Then H-25's rules**, as `derive_under` runs them: the quiet run, the cancel, the frozen block's two measures, the inhibitory rule's target period, the critic's window and constants. A rule that returns nothing stops the derivation there, and every rule after it returns nothing.
- **A step holds** when every rule gives a value and the frozen block passes both measures (`Rules::hold`): every stimulus unit firing once by ADR-0074's measure, and the stimulus seen in 56 trials of 64. **The step taken is the largest that holds**, the first in the scan's order (`ruled`).
- **The arms**: H-25's two on H-20's schedule from the taken step's image, under the true reward, the step and the target carried through every trial (`ruled_arm`).
- **H-38's clauses** are ADR-0178's, by the rules the tree holds: clause 0, `Rules::hold` at one step at least; clauses 1 to 4, `drawn_of` against the image the taken step's derivation wrote — at least 80 of each mapping's last 128 trials correct; no coupling past 1.30 of its image's; each reversal passing 40 of 64 within 23 blocks of its flip; the excitatory sum outside the four couplings within 0.75 and 1.25 of the image's. **Yes** when all five hold in both arms.
- **The stopping rule's steps** (`ruled_step`): 3, yes; 5, no step holds; otherwise 4.
- **The prediction**: yes (`RULED_PREDICTED`).
- **The four predicted readings as rules** (`rate_predicted`), each predicted to hold (`RATE_READINGS_PREDICTED`):
  1. the step of a sixteenth holds: `Rules::hold` at the scan's second step;
  2. no window of any of the six lead-ins holds 32 768 spikes or more, the saturation rule's count at 1 024 units;
  3. through each arm the gain after every window that ends inside it is within five per cent of 1.75: twenty times its distance from 114 688 at most 114 688, which is 108 954 to 120 422, both counted (`gain_within`);
  4. each arm's gains after those windows sum below as many times the gain its image carries, strictly (`mean_below`).

  The third and the fourth are read where arms run and are not read where none does.

### The harness (built before any lead-in under the rate's rule)

- **The settling carries the rule** (option 1(a)). `Settling` gains the target and whether the rule reads the gain's mean; `GAIN_HELD`, `AT_AN_EIGHTH` and `under_step` carry no target and read no mean. `derive_under` builds its candidate by `candidate_under`, which is `candidate_from` with the target in the configuration, and holds the image to carrying it. With the target unset the configuration is the one it was, field for field, so every derivation of the tree is the derivation it was. A second derivation (option 1(b)) would be H-25's rules written twice.
- **The rate's rule as an oracle** (`rate_regulated`): ADR-0180's rule in `i128`, each rounding to nearest by doubling and not by the record's shifts. The gate holds it to numbers written by hand and to `HomeostaticDrivePool::regulate_rate` over 400 windows fed bin by bin.
- **A lead-in's reading** (`RateRead`, `rate_read`): its length; the window each criterion first holds at, and both; the gain's band over the lead-in and over its last sixteen windows; the least and the greatest count, and the last sixteen windows' count; the windows above the target; the windows at the saturation rule's count; the windows whose gain is not the oracle's from the gain before, the window's count, the target and the step; the last window; the sums; and the steepness.
- **The steepness** (`steepness`): each window's count against the gain it ran under, by the least-squares slope at the two means, as the per cent of rate a per cent of gain moves. It is the table's slope: the lead-in's weights move under it, and the gain answers the count of the window before.
- **Each step a weekly test** (`rate_step`): the derivation dumped whole, then held — what it makes before the settled network and its cancel to seed 22's constants; its first window to the gain held's in everything but the gain; every window's gain to the oracle; the train's capacity; and the tables pinned from its one run (`RATE_*`).
- **The arms' ground** (option 3(a), `Ground::Ruled`, `RuledSite`): the instrument's prior under the rate's rule at the taken step. Its calibration is that step's derivation, run again and held to the step's pinned tables as the step's own test holds it (`ruled_made`); a step that does not hold has no arm. A `Site` given a settling (option 3(b)) would need a pinned derivation in a `Site`'s shape for a network whose derivation before the settled network is seed 22's own, and would run `own_first_block`, which holds an arm under another reward to the network's arm under the true one: here the arm is that arm.
- **Read beside H-25's** (option 4(a), `ruled_stand`): the tables `arm_on` reads an arm beside are, on this ground, H-25's two with the gain held. So each reversal's block, the value, the troughs, the signal, the strong punishments, the couplings' and the inhibitory sum's courses, and the first block that is not H-25's are dumped beside H-25's by the lines the tree already has.
- **The controller beside every arm** (options 5(a) and 6(a), `GainWatch`): at every trial's end the gain, and each window's spikes from the train, the window open at the load beginning with what the image's record held of it. Then, on every ground:
  - the train's count of the open window is the record's own at every trial's end, 7 680 times an arm;
  - 960 windows end inside an arm, eight in every block;
  - every window's gain is the rule's from the gain before it: the oracle's from the window's count where the engine carries a target, and the gain before where it carries none;
  - with no target the step is zero and the gain is the image's throughout.

  So every arm the calibration runs through `arm_on` shows the gain held at every window, and shows the count H-38's arms are read by to be the engine's own.
- **An arm's reading of the controller** (`GainRead`, `gain_blocks`): the band; the first window outside predicted reading 3's band; the gains summed beside as many times the image's; the counts beside as many targets; the windows above the target; the windows not the rule's; per reversal the gain at the flip and the lowest and the highest over its twenty-three blocks; and by block the lowest, the highest and the last gain and the block's count. Pinned for H-38's arms.
- **The gate** (one test, `the_settled_network_s_rule_under_the_rate_s_rule_clause_0_of_h_38_and_the_predicted_readings_rules`): the constants; the oracle; the gain's mean's criterion at its edges — below the thirty-second window, exactly 0.75 per cent either way and one part inside, against the reference's own sum, a swing, a rise that stops — and the settled network's rule waiting for the sums; clause 0, the verdict and the stopping rule's steps over tables written by hand; the four readings' rules at their edges; the lead-in's reading, the steepness, the watch, the gain by block and the arm's reading over tables written by hand. It runs no lead-in.
- **Two tables read before the run**, functions of lead-ins the tree already pins:
  - the settled network's rule under the rate's rule over them (`RATE_BESIDE_SETTLED`): the gain held's at its ninety-sixth window, where its sums settle; ADR-0077's at an eighth at its thirty-fifth; H-37's at a sixteenth, one in 32 and one in 64 at their forty-fifth, forty-fourth and fifty-sixth; H-37's at one in 128 and one in 256 at none of the 71 and 114 windows they ran. So the mean reads as settled the three lead-ins ADR-0177's rule read as never settling, which is what it was written to do; their frozen blocks were not run.
  - the gain held's lead-in by the rate's reading (`HELD_RATE_READ`), item 5 of the context.

### The readings

Added after the one run of each step and each arm.

### Consequences

- Good: the criterion that reads the gain as settled is an integer rule with its edges in the gate, and what it reads of seven pinned lead-ins is known before any lead-in under the rule.
- Good: a run under the rule is held to the rule in every window of every lead-in and every arm, by an oracle written from ADR-0180's text.
- Good: every arm the tree runs through `arm_on` now shows its gain held and its windows' counts the engine's own, each week.
- Bad: `arm_on` reads more on every ground, and its dumps grow by the windows of the arm.
- Bad: the steepness is one number for a lead-in whose weights and gain both move.

## Alternatives considered and why rejected

- **A second derivation** (option 1(b)), **the prior's gain for windows that did not run** (option 2(b)), **a `Site` given a settling** (option 3(b)): see above.
- **Read beside nothing** (option 4(b)): ADR-0178's readings ask for each of H-25's readings beside H-25's with the gain held, and the lines exist.
- **The gain at each block's end** (option 5(b)): predicted reading 3 is of the gain after every window, and a block's end is one window in eight.
- **H-38's arms alone** (option 6(b)): the watch's count would then be checked against the record only in the two arms it is needed for, and the gain held would stay asserted at the load alone.

## Confirmation

- `runtime/cortex-runtime/tests/inhibition.rs`: `Settling`, `candidate_under`, `derive_under`; `RATE_STEPS`, `RATE_TARGET_PERIOD`, `RATE_TARGET_COUNT`, `RATE_BOUND`, `under_rate`; `rate_regulated`, `mean_settled_in`, `settled_in_mean`, `steepness`, `RateRead`, `rate_read`; `GainWatch`, `windows_of`, `gain_blocks`, `GainRead`, `gain_read`, `gain_within`, `gains_within`, `mean_below`; `Ruled`, `ruled`, `ruled_step`, `rate_predicted`, `RULED_PREDICTED`, `RATE_READINGS_PREDICTED`; `Ground::Ruled`, `RuledSite`, `ruled_stand`, `ruled_made`, `ruled_arm`; `rate_step`; the six tests `the_rate_s_rule_at_a_step_of_…_at_1024_units_exhaustive` and the two `the_address_drawn_under_the_rate_s_rule_from_the_…_at_1024_units_exhaustive`; the gate's test; `RATE_BESIDE_SETTLED`, `HELD_RATE_READ`.
- Whitepaper §11.1 (H-38), §9; `CHANGELOG.md`.
