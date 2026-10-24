---
status: exploratory
date: 2026-10-25
---

# 001: The old no's under the engine's default

**Branch**: `explore/001-the-old-noes-under-the-default`
**Commit**: `de72983` for the first eleven runs, `b0f9fc8` for the four on seeds 23 and 24
**Runs**: [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) on `de72983`, [40130108060](https://github.com/DescentVTT/VirtualCortex/actions/runs/40130108060) on `b0f9fc8`

Four questions were read as no, or never asked, under an earlier configuration. This note asks them again under the
engine's default, `Config::reads_its_targets()`: the read of the rate's target, both targets set by it, and the probe of
the step ([ADR-0196](../adr/0196-the-engines-default.md), [ADR-0205](../adr/0205-the-probe-in-the-engines-default.md)).
It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host.

## What ran

**The network.** Every run starts from H-42's network of seed 22 at 1 024 units: `derive_under(…, &UNDER_THE_PROBE)`,
the network an engine born under the default settles ([ADR-0206](../adr/0206-the-probe-in-the-engines-default-built.md)).
Before anything else, each test runs that derivation again and holds it to the tables H-42's one run pinned
(`probe_derivation_held`). Every run did so. The derivation reads its inhibitory target at 57 604 ticks, makes the step
1 078/65 536 (about 1/61), and leaves the gain at 1.747.

**The tests.** Fifteen `#[ignore]`d `…_explore` tests are appended to `runtime/cortex-runtime/tests/inhibition.rs` on
the branch. Nothing under `src/` changed. Two harness functions changed on the branch alone:

- `Derivation` keeps the frozen settled image (`zero`), so that the oracle can fork it.
- `answered_read` takes its image as an argument (`answered_read_on`).

Nothing is pinned. Each test asserts only what the harness already asserts, and prints the rest.

**The host's reward.** Questions 3 and 4 run H-25's arm through `run_on_scheduled_over` with the task's feedback set to
`Withheld`. After each trial, once the task has written that trial's address, the host calls `Executor::reward` itself.
Nothing of the engine changes; the host chooses only which sign it gives and when. The control is the same path with the
reward on time. On both arms the control reproduced H-42's pinned arm on the network in its blocks, its trace and its
values hash, so the path is the arm's.

## 1. The causal count inside the loop

**Then** ([ADR-0054](../adr/0054-the-causal-count-inside-the-loop.md)). The measurement ran on the prior, unsettled,
with the controller off, at gains 1.75, 2.0 and 2.25. It compared the in-loop ratio, descendants over spikes per window,
with the oracle's gross causal ratio from sixteen kicks at 1 024 units. The rule was: within 0.15 of the gross ratio at
every gain, and rising with the gain. On the lattice at 1 024 units the ratio read 0.343, 0.575 and 0.674 against gross
ratios of 0.529, 0.638 and 1.000. The rule failed, and ADR-0054 read the count as the input's density, not the
branching ratio.

**Now.** The same method was run on the frozen settled image at each gain, with the gain fixed and the step zero. A
fourth gain was added: the 1.747 the image carries. The method is copied from `tests/reference.rs`. On the prior at
1.75 the copy reproduced ADR-0054's pinned windows and its sixteen-kick attribution exactly. Beside ADR-0054's sixteen
kicks, 256 kicks were also run, each in its own fork.

| Network | Gain | In loop, windows 1 and 2 | Gross, 16 kicks | Gross, 256 kicks | Net, 256 kicks |
| :--- | ---: | :--- | ---: | ---: | ---: |
| prior | 1.75 | 0.343, 0.314 | 0.529 | 0.501 | 0.434 |
| prior | 2.00 | 0.575, 0.504 | 0.638 | 0.704 | 0.378 |
| prior | 2.25 | 0.674, 0.638 | 1.000 | 0.777 | 0.332 |
| settled | 1.747 | 0.329, 0.324 | 0.438 | 0.446 | 0.397 |
| settled | 1.75 | 0.330, 0.328 | 0.438 | 0.447 | 0.396 |
| settled | 2.00 | 0.495, 0.510 | 0.932 | 0.710 | 0.446 |
| settled | 2.25 | 0.636, 0.633 | 0.935 | 0.732 | 0.312 |

On the settled network windows 3 and 4, beyond ADR-0054's two, read within 0.04 of windows 1 and 2.

**What changed.** Nothing in kind. On the settled network the in-loop ratio is the same function of the gain as on the
prior: 0.33, 0.50 and 0.63 against 0.33, 0.54 and 0.66. It rises with the gain on both networks. Against 256 kicks it
stands 0.12, 0.21 and 0.10 below the gross ratio on the settled network, and 0.17, 0.16 and 0.12 below it on the prior.
At the default's own gain the gap is 0.12. Read by ADR-0054's rule with its sixteen kicks, the count fails at 2.0 and
2.25; with 256 kicks, at 2.0.

Two readings go beyond the question:

- **Sixteen kicks are too few for a margin of 0.15.** Sixteen and 256 kicks differ by 0.22 at 2.0 on the settled
  network and by 0.22 at 2.25 on the prior.
- **ADR-0054's text and its table disagree.** Its text says the rule held "on the lattice at 1.75 at every size". Its
  own table puts the lattice at 1 024 units 0.19 and 0.22 below the gross ratio there, and at 4 096 units 0.18 and
  0.22 below. The rule's outcome does not depend on it.

**Suggests.** No formal question. If a later round wants the count as a reading of $\sigma$, it would need an oracle of
more than sixteen kicks before any margin is read.

## 2. Three answers

**Then** ([ADR-0152](../adr/0152-three-answers-built.md), [ADR-0153](../adr/0153-three-answers-measured.md)). H-29
ran on the instrument's network under H-25's configuration, with the gain fixed at 1.75. The prompt for this exploration
described H-29's inhibitory target as the default's 5 Hz. The tree says otherwise: H-29, like H-25 and H-35, ran at
H-21's 58 201 ticks, 1.72 Hz, and `answered_arm` asserts it. Its three readouts were the 104th deal of 92 400, index
103. H-29 was no on clause 1: seven mappings of eight were learned by each stimulus, and the mirrored arm's second was
not. A reversal took 21 to 31 of a mapping's 32 blocks.

**Now.** H-29's arm ran on the default's network: its task, schedule and harness unchanged, with the network's critic
oracle and network oracle held at every trial. The readouts' rule was applied to this network's own image, and took
the first deal in the committed order (index 0, 5 242 of 92 400 passing, where H-29's image passed 10 598). The
readouts are therefore not H-29's sets. H-29's frozen block, which needs the instrument's settled clock, was not run.

| Arm | Last 128 correct by mapping | Learned by each | Blocks to 40 of 64 by mapping |
| :--- | :--- | :--- | :--- |
| assignment, default | 117, 118, 94, 122 | all four | 4, 23, 29, 22 |
| assignment, H-29 | 118, 87, 82, 109 | all four | 5, 31, 31, 21 |
| mirrored, default | 108, 117, 87, 117 | not the third: B 29 of 66 | 8, 21, 31, 17 |
| mirrored, H-29 | 119, 85, 99, 114 | not the second: A 30 of 64 | 7, 30, 23, 23 |

No coupling passed 1.30 of its image's, and the network outside the six couplings held. Each stimulus's mean value
stood within H-29's band in fifteen cells of sixteen; the miss is stimulus A in the mirrored arm's second mapping. On
the same network with two answers (H-42's arms) the reversals took 11 to 14 blocks.

**What changed.** The verdict's shape did not: seven mappings of eight were learned, and the one that failed moved from
the mirrored arm's second mapping to its third. The reversals were a little faster, 23.8 blocks on average against
H-29's 26.5, and still about twice the two-answer arms' on the same network.

**Suggests.** No formal question. A choice among three is slow on this network for the reason it was slow on the
instrument's.

## 3. A reward right three times in four

**Then** ([ADR-0171](../adr/0171-a-reward-right-three-times-in-four-built.md),
[ADR-0172](../adr/0172-a-reward-right-three-times-in-four-measured.md)). H-35 ran on the instrument's network with the
gain fixed at 1.75. It was no on clause 1: seven mappings of eight were learned, the fourth from the assignment at 75 of
128. Its third reversal from the assignment never reached 40 of 64 inside its 32 blocks; its best block was 38. On
seeds 23 and 24, H-36 ([ADR-0174](../adr/0174-three-in-four-on-the-other-networks-measured.md)) learned every mapping on
seed 23 and six of eight on seed 24.

**Now.** H-35's arm ran on each of H-42's three networks: the same coin (`Task::misleading_in_four_at`), schedule and
delivery, with the reward given through the host as above.

| Network | Arm | Last 128 correct, default | Last 128 correct, gain held | Blocks to 40 of 64, default | Blocks to 40 of 64, gain held |
| :--- | :--- | :--- | :--- | :--- | :--- |
| seed 22 | assignment | 120, 114, 122, 105 | 115, 80, 110, 75 | 5, 18, 16, 18 | 6, 26, 20, none |
| seed 22 | mirrored | 118, 107, 122, 90 | 117, 94, 110, 97 | 6, 25, 19, 28 | 6, 24, 19, 24 |
| seed 23 | assignment | 120, 121, 125, 123 | 121, 115, 121, 119 | 6, 17, 17, 15 | 7, 17, 19, 16 |
| seed 23 | mirrored | 124, 106, 125, 112 | 121, 97, 117, 112 | 3, 16, 15, 24 | 3, 21, 14, 21 |
| seed 24 | assignment | 124, 90, 127, 94 | 121, 71, 116, 76 | 1, 26, 12, 26 | 2, 32, 12, none |
| seed 24 | mirrored | 105, 102, 121, 92 | 107, 119, 112, 112 | 14, 23, 20, 24 | 11, 18, 16, 25 |

The "gain held" columns are H-35's arms on seed 22 and H-36's on seeds 23 and 24.

In all six arms no coupling passed 1.30 of its image's, and the network outside the couplings held. Each stimulus's
mean value stood within H-35's band, at a reliability's part of a half, in every cell. So all four of H-35's clauses
held in every arm. All 24 mappings were learned, the lowest at 90 of 128, and all 18 reversals reached 40 of 64 inside
their 32 blocks, the slowest in 28. With the gain held the three networks had learned 21 mappings of 24, the lowest at
71, and 16 reversals of 18 had crossed.

Counted in blocks, the reversals were about as fast as with the gain held: 19.9 on average against 20.3 for the 16 that
crossed then. What changed is that none ran out of the schedule. Against each network's own arms under the true reward,
which on these networks take 10 to 14 blocks, the reversals took 1.67 times as long on average (0.92 to 2.36). ADR-0174
read 1.38 over the three networks with the gain held: the default's true-reward arms are the faster ones.

**What changed.** The reading. Under the default, H-35's question reads as yes on all three networks, on every clause in
every arm. With the gain held it read no on seed 22 (H-35) and on seed 24 (H-36). The four weakest mappings, 71 to 80
of 128 with the gain held, are 90 to 94 under the default.

This run cannot say what made the difference. The default changes three things at once: the settled network, the gain
moved by the rate's rule at the probe's step during every arm, and the targets the engine read.

**Suggests a formal question.** A decision ADR could write H-35's question as a hypothesis on the default, with its own
prediction and protocol, on all three networks. A control arm would separate the network from the rule: the default's
settled image with the controller's step set to zero through the arm.

## 4. A reward one or two trials late

**Then.** This question was not asked before. It is a baseline for M10, a reward past the trial
([Appendix C](../WHITEPAPER.md#appendix-c-roadmap)). The facts it rests on, from the tree:

- A trial is 16 384 ticks.
- The eligibility trace decays with $\tau$ = 65 536 ticks, four trials (`ELIGIBILITY_TAU_SHIFT`). A trace that
  nothing consolidates keeps 0.778 of itself over one trial and 0.605 over two.
- A trial's reward is consolidated during the trial after it ([ADR-0090](../adr/0090-the-old-answer-held-from-the-trial-after-the-flip.md)).
- The drawn address takes its sources from the critic's counts since the last reward, which every reward zeroes, and its
  targets from the trial's selection ([ADR-0139](../adr/0139-the-address-drawn-built.md)).

**Now.** H-25's arm ran under H-20's schedule on the default's network of seed 22. The host gave the sign of trial $t$
at the end of trial $t + k$, for $k$ = 1 and 2, with no new mechanism. Nothing was given at the end of the first $k$
trials, and the last $k$ signs were never given. The address each reward reached was the one the task wrote at the end
of trial $t + k$. At each reward the run recorded the following:

- whether trial $t + k$ presented trial $t$'s stimulus and selected its readout, so that the address named the pair
  trial $t$ was rewarded for;
- whether the sign was the one trial $t + k$'s own outcome would have earned;
- where the drawn sources lay;
- trial $t$'s pair's summed trace at its own end and again at the reward, each decayed to the moment it was read.

| Delay | Arm | Last 128 correct by mapping | Learned (80 of 128) | Critic's mean value by mapping |
| :--- | :--- | :--- | :--- | :--- |
| 0 | assignment | 120, 120, 123, 123 | all four | 0.88, 0.89, 0.91, 0.97 |
| 0 | mirrored | 122, 120, 125, 125 | all four | 0.86, 0.88, 0.91, 0.93 |
| 1 | assignment | 36, 68, 48, 44 | none | −0.39, −0.05, −0.25, −0.26 |
| 1 | mirrored | 38, 45, 35, 63 | none | −0.40, −0.26, −0.48, −0.15 |
| 2 | assignment | 47, 80, 43, 79 | the second, at 80 | −0.24, 0.13, −0.30, 0.33 |
| 2 | mirrored | 45, 73, 34, 66 | none | −0.36, 0.14, −0.50, 0.04 |

With one or two trials of delay nothing was learned. With ties counted wrong, as the task counts them, chance at these
runs' ties is about 58 of 128. The first mapping fell during its 24 blocks in all four arms: 24 to 32 correct of 64 in
its first block, 17 to 25 in its last two. The first and the third mappings, which share the first mapping's answers,
ended below chance in every arm, at 34 to 48 of 128. The second and the fourth ended at 44 to 80. So the delayed sign
did more than leave the network unlearned: it moved the network away from the first mapping, in both arms, and that
preference carried through the flips.

What each reward reached, per mapping, the ranges over the four mappings of both arms:

| Delay | Address names the rewarded pair | Same stimulus | Sign is the addressed trial's own | Sources in the rewarded trial's set | Rewarded pair's trace at the reward over its own |
| ---: | :--- | :--- | :--- | :--- | :--- |
| 0 | 0.94 to 0.96 | 1.00 | 1.00 | 0.98 | 1.00 |
| 1 | 0.20 to 0.25 | 0.50 to 0.52 | 0.50 to 0.55 | 0.49 to 0.50 | 0.56 to 0.59 |
| 2 | 0.20 to 0.27 | 0.49 to 0.52 | 0.49 to 0.58 | 0.48 to 0.50 | 0.69 to 0.73 |

**What failed: the address, not the trace.** When the reward came, the rewarded trial's pair still held 56 to 59 per
cent of its trace after one trial and 69 to 73 per cent after two. Decay alone would leave 0.778 and 0.605. The run
does not separate decay from what later pairings of the same pair added and what consolidations in between spent. One
of those is certain from the code: at one trial's delay, the reward given at the rewarded trial's own end carried the
trial before's sign and was addressed to this very pair. What the run reads is that the trace was there when the
reward came.

The address went elsewhere. It named the rewarded trial's pair in 20 to 27 per cent of rewards. Half the time its
sources were the other stimulus's units: the counts are zeroed at every reward, so the sources are always the
delivering trial's own stimulus. The addressed pair's own trace was 1.23 to 1.62 times the rewarded pair's. So each
reward was consolidated mostly into the pairing of the trial it arrived at, under a sign that was that trial's own in
about half the rewards.

The critic's value did not follow each stimulus's accuracy, since it is read from the delivering trial's stimulus and
taken against another trial's sign. In the first mapping at one trial's delay from the assignment, A was correct in 13
of 61 presentations and B in 23 of 67, and their mean values were −0.36 and −0.41.

**Suggests a formal question for M10.** With no new mechanism, a reward one or two trials late does not just go
unlearned; it moves the network away from the mapping it is given first. The trace is long enough for one or two
trials. What has to cross the delay is the address: the sources the critic counted and the targets the trial selected.
A decision ADR for M10 could
ask whether an engine that keeps each trial's drawn address until its reward arrives, and consolidates under that
address, learns the task at $k$ = 1 and 2. This exploration's arms would be its baseline, measured again by its own
protocol. Why the network moves against the mapping, and not merely to chance, is not read here; it is a question of
its own.

## What did not run

- Questions 1, 2 and 4 on seeds 23 and 24. The rule for this exploration extended only the items whose reading looked
  different from the old one, and only question 3's did. Question 4 has no old reading; its seed 22 arms failed in all
  four mappings of all four arms.
- H-29's frozen block's sight, which reads the instrument's settled clock, on the default's network.
- Any delay under the reward right three times in four, and any delay beyond two trials.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `a_reward_on_time_by_the_host_on_seed_22_from_the_assignment_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 1131 |
| `a_reward_on_time_by_the_host_on_seed_22_from_the_mirrored_assignment_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 1064 |
| `a_reward_one_trial_late_on_seed_22_from_the_assignment_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 1092 |
| `a_reward_one_trial_late_on_seed_22_from_the_mirrored_assignment_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 1070 |
| `a_reward_two_trials_late_on_seed_22_from_the_assignment_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 1097 |
| `a_reward_two_trials_late_on_seed_22_from_the_mirrored_assignment_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 1133 |
| `the_causal_count_on_the_default_s_settled_network_on_seed_22_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 700 |
| `three_answers_on_the_default_s_network_on_seed_22_from_the_assignment_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 1132 |
| `three_answers_on_the_default_s_network_on_seed_22_from_the_mirrored_assignment_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 1086 |
| `three_in_four_on_the_default_s_network_on_seed_22_from_the_assignment_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 1041 |
| `three_in_four_on_the_default_s_network_on_seed_22_from_the_mirrored_assignment_explore` | [40116320992](https://github.com/DescentVTT/VirtualCortex/actions/runs/40116320992) | `de72983` | 1081 |
| `three_in_four_on_the_default_s_network_on_seed_23_from_the_assignment_explore` | [40130108060](https://github.com/DescentVTT/VirtualCortex/actions/runs/40130108060) | `b0f9fc8` | 1092 |
| `three_in_four_on_the_default_s_network_on_seed_23_from_the_mirrored_assignment_explore` | [40130108060](https://github.com/DescentVTT/VirtualCortex/actions/runs/40130108060) | `b0f9fc8` | 1106 |
| `three_in_four_on_the_default_s_network_on_seed_24_from_the_assignment_explore` | [40130108060](https://github.com/DescentVTT/VirtualCortex/actions/runs/40130108060) | `b0f9fc8` | 1131 |
| `three_in_four_on_the_default_s_network_on_seed_24_from_the_mirrored_assignment_explore` | [40130108060](https://github.com/DescentVTT/VirtualCortex/actions/runs/40130108060) | `b0f9fc8` | 1123 |
