---
status: exploratory
date: 2026-10-25
---

# 003: A delay the engine is not told

**Branch**: `explore/003-a-delay-the-engine-is-not-told`
**Commit**: `4f64c1b` for the eight arms told and untold on seed 22, `87ba360` for the switch on seed 22, `7933286` for the switch on seeds 23 and 24
**Runs**: [40215359541](https://github.com/DescentVTT/VirtualCortex/actions/runs/40215359541) on `4f64c1b`, [40217289629](https://github.com/DescentVTT/VirtualCortex/actions/runs/40217289629) on `87ba360`, [40219023428](https://github.com/DescentVTT/VirtualCortex/actions/runs/40219023428) on `7933286`

[H-43](../research/H-43.md) read that a reward one or two trials late teaches when the engine keeps each trial's
address and the reward names the age of the trial it is for
([ADR-0207](../adr/0207-a-reward-past-the-trial-its-address-kept.md),
[ADR-0208](../adr/0208-a-reward-past-the-trial-its-address-kept-built.md)). The host named that age. ADR-0207 left
open a delay the engine is not told. This note asks three things:

- whether the engine could tell the true delay from the addresses it keeps (`Config::kept_addresses`);
- how many trials that takes;
- whether the network learns once it can.

It writes no prediction and decides nothing. **Every figure here is exploratory**, from one run of each test on the
repository's AArch64 host.

**In brief**
- **The kept counts name the delay early.** Each age had a critic of its own on the host. In all eight arms on seed
  22, told and untold, the true age's critic had the lowest score at each of the first 64 rewards scored, trials 65 to
  128. That held although in the untold arms the network learned at most one mapping of four. In all twenty arms, a
  run of 64 rewards with the true age lowest was complete by trial 235.
- **In those eight arms the true age was the lowest most of the time, never always.** Over a run it led at 55 to 68 %
  of rewards when the delay was told, and at 42 to 93 % when it was not, against a third by chance. Another age took
  the lead for up to 192 rewards at a time.
- **A switch driven by the scores found the delay in all twelve arms.** The host consolidated at age zero and moved to
  an age that led by a thirty-second of the reward for 64 rewards running. Its first move was to the true age in every
  arm on seeds 22, 23 and 24, by trial 309 in eight of them and at trials 1 448 to 4 269 in the other four. The
  network learned 43 mappings of 48, and seven arms of twelve held all four of H-43's clauses.
- **Both wrong moves were second moves.** In two arms a wrong age later led by more than the lead while a mapping was
  rising steeply, and the switch followed it. One of them moved back.

## What ran

**The network and the arm.** Every run starts from H-42's network of its seed at 1 024 units, the image an engine born
under `Config::reads_its_targets()` settles. Each test first runs that derivation again and holds it to H-42's pinned
tables (`probe_made`). Every run did so. Then comes H-43's arm: H-25's arm on H-20's schedule from that image, under the
drawn delivery, with the engine's critic and its window, two addresses kept, and the answer's reward. Trial $t$'s sign
comes at the end of trial $t + k$, for $k$ = 1 and 2. Both of H-25's arms ran, from the assignment and from the
mirrored one.

**The host's path.** Twenty `#[ignore]`d `…_explore` tests and one harness function, `untold_arm`, are appended to
`runtime/cortex-runtime/tests/inhibition.rs` on the branch. Nothing under `src/` changed, and nothing is pinned. The
task's feedback is withheld, so it closes each trial by drawing and gives no reward. After each trial the host:

1. keeps the trial's address (`Executor::keep_address`), as the task keeps it under the drawn delivery;
2. copies the counts the entry kept (`Executor::kept_at(0)`) into a ring of the last three trials, and holds the ring
   to every entry still live;
3. where trial $t - k$'s sign is due, reads each age's critic and the scores, then gives the engine the sign through
   `Executor::reward_kept` at the age it chooses.

A trial with no reward writes back the address the last reward wrote, or every unit before the first reward, as the
task's own path leaves it.

**The critics on the host.** There is one per age $a$ = 0, 1 and 2, the entries `kept_addresses` 2 holds. Each starts
from the engine's value weights at the arm's start. At each reward it predicts from the counts of trial $t - a$, its
own age's, and learns from them, by the harness's oracle of the engine's rule (`value_step`). Its error is read before
its weights move.
- An age's **score** is the mean magnitude of its errors over the last 64 rewards, as a fraction of the reward.
- Scores are compared once all three windows are full, from trial 65 on.
- At a reward, the **leader** is the age with the strictly lowest score, and its **lead** is how far below the next
  score it is.

**The three ways the host chose the age:**
- **Told**: every reward at age $k$. This is H-43's arm.
- **Untold**: every reward at age zero, exploration 001's case.
- **The switch**: age zero until an age other than the one in use has led at 64 consecutive rewards, each time by at
  least the switch's lead; then that age. Nothing latches it, so a later leader can move it again.

**What the path was held to.**
- Told, each arm is H-43's arm on its network bit for bit: its 120 blocks, its trace, every reward's value and its
  table. All four told arms held it.
- The host's critic at the age the engine consolidates under is the engine's own. Its value equalled the engine's at
  every reward of the told and untold arms, and of every switch arm before its first move.
- A switch arm runs its untold arm's path until it first moves. On seed 22 each first move came at the trial its
  untold arm put it at.

The untold path is not exploration 001's exactly. Here each close zeroes the counts, so the first reward reads trial
$k$'s counts alone; there it read the counts of trials 0 to $k$. The arms ended alike: from the assignment at one
trial's delay, 38, 68, 48 and 40 correct of each mapping's last 128 here, against 36, 68, 48 and 44 there.

## 1. Told: is the true age's critic the lowest?

| Delay | Arm | The true age led, of 7 615 rewards scored | By mapping | Mean score by age 0, 1, 2, by mapping |
| ---: | :--- | :--- | :--- | :--- |
| 1 | assignment | 4 727, 62 % | 42, 74, 54, 72 % | 0.453 0.445 0.454; 0.598 0.562 0.597; 0.553 0.543 0.552; 0.599 0.535 0.598 |
| 1 | mirrored | 5 213, 68 % | 77, 66, 69, 64 % | 0.488 0.459 0.488; 0.575 0.559 0.576; 0.573 0.553 0.573; 0.601 0.562 0.603 |
| 2 | assignment | 4 191, 55 % | 39, 54, 59, 63 % | 0.451 0.449 0.445; 0.607 0.608 0.602; 0.559 0.559 0.553; 0.577 0.578 0.540 |
| 2 | mirrored | 4 958, 65 % | 85, 63, 52, 66 % | 0.518 0.520 0.490; 0.623 0.622 0.590; 0.606 0.606 0.601; 0.603 0.606 0.575 |

Every told arm learned all four mappings and held H-43's four clauses, as H-43 read.

- **Lowest from the first reward scored.** In every told arm the true age led at each reward from trial 65 to 128.
- **Not always lowest after.** The last reward at which another age led was at trial 7 644 to 7 679, near the run's
  end. A wrong age led for at most 96 to 149 rewards running.
- **By a small margin.** Mapping by mapping, the true age's mean score sat 0.004 to 0.063 of the reward below the
  lower of the others'. In the first mapping from the assignment at two trials' delay it led at 39 % of rewards,
  barely more than another age. A reading: once a mapping is learned, every critic predicts a reward near its mean,
  and the scores close up.
- **Its sustained leads were larger than any wrong age's.** Over 64 rewards running, the true age held a lead of up to
  0.156 to 0.235 of the reward. A wrong age held at most 0.018.

## 2. Untold: do the scores still name the delay?

| Delay | Arm | Last 128 correct by mapping | The true age led, of 7 615 | By mapping | Mean score by age 0, 1, 2, by mapping |
| ---: | :--- | :--- | :--- | :--- | :--- |
| 1 | assignment | 38, 68, 48, 40 | 7 107, 93 % | 92, 87, 95, 98 % | 0.892 0.827 0.892; 0.979 0.927 0.978; 0.951 0.861 0.954; 0.983 0.773 0.984 |
| 1 | mirrored | 42, 45, 28, 75 | 4 012, 53 % | 46, 69, 68, 27 % | 0.936 0.930 0.935; 0.994 0.968 0.992; 0.929 0.904 0.925; 0.955 0.959 0.958 |
| 2 | assignment | 43, 66, 45, 85 | 3 199, 42 % | 72, 36, 27, 42 % | 0.947 0.948 0.926; 0.993 0.995 0.993; 0.930 0.931 0.933; 0.899 0.899 0.896 |
| 2 | mirrored | 50, 54, 37, 94 | 4 986, 65 % | 95, 70, 39, 66 % | 0.951 0.952 0.903; 0.985 0.987 0.959; 0.924 0.925 0.922; 0.932 0.933 0.915 |

With every reward consolidated at age zero the network learned no mapping at one trial's delay, and only the last of
four at two. That is exploration 001's reading again.

**The scores did not wait for the network.** In all four untold arms the true age led at each reward from trial 65 to
128, while the arms read 24 to 35 correct of 64 in blocks 1 and 2, about chance. Its lead was largest early. In the
first mapping it led at 46 to 95 % of rewards, its mean score 0.005 to 0.065 below the lower of the others'.

**Later it faded in two arms.**
- From the mirrored assignment at one trial's delay, age zero led more often than the true age in the last mapping.
- From the assignment at two trials' delay, age zero led more often in the second mapping and the third.
- Over 64 rewards running, a wrong age's lead reached at most 0.0275 of the reward, age zero's, against the true age's
  0.074 to 0.335.

**When a switch from age zero would first have moved**, read from each untold arm at six leads, the trial given:

| Delay | Arm | No lead | 1/64 | 1/32 | 1/16 | 1/8 | 1/4 |
| ---: | :--- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | assignment | 128 | 128 | 128 | 128 | 600 | 6 189 |
| 1 | mirrored | 128 | 252 | 2 565 | 3 372 | never | never |
| 2 | assignment | 128 | 128 | 129 | 582 | never | never |
| 2 | mirrored | 128 | 128 | 251 | 550 | never | never |

Up to its first move a switch arm is its untold arm, so this is when the switch would move. In all eight arms told and
untold, at every lead, the first age other than zero to complete the run was the true age.

**What carries it is not read here.** One reading fits: before the network learns, its accuracy differs by stimulus,
and only a critic that sees the rewarded trial's stimulus in its counts can follow that. The runs did not record
accuracy by stimulus, so this stays a guess.

## 3. The switch

**Its lead was chosen from the eight arms above, after they ran.** In none of them did an age other than the delay
hold a lead of more than 0.0275 of the reward over 64 rewards running. A thirty-second, 0.03125, is the smallest of the
six leads above that.
- So on seed 22 the lead is fitted to the runs it is read on. Seeds 23 and 24 ran it unchanged.
- The commit that set it (`87ba360`) gives the fourth arm's switch as trial 250. That trial was read at a lead of
  0.031, not at a thirty-second. At a thirty-second the switch is at 251, where the arm moved.

| Seed | Delay | Arm | Moved to (trial, age) | Signs lost | Last 128 correct by mapping | Learned | H-43's clauses | Reversals summed |
| ---: | ---: | :--- | :--- | ---: | :--- | ---: | :--- | ---: |
| 22 | 1 | assignment | (128, 1) | 1 | 118, 122, 125, 125 | 4 | all four | 36 |
| 22 | 1 | mirrored | (2 565, 1) | 1 | 42, 119, 119, 124 | 3 | not clause 1 | 35 |
| 22 | 2 | assignment | (129, 2), (471, 0), (4 318, 2) | 4 | 110, 61, 119, 120 | 3 | not clauses 1 and 3 | 55 |
| 22 | 2 | mirrored | (251, 2) | 2 | 120, 122, 123, 124 | 4 | all four | 44 |
| 23 | 1 | assignment | (2 144, 1) | 1 | 32, 123, 121, 126 | 3 | not clause 1 | 37 |
| 23 | 1 | mirrored | (1 448, 1) | 1 | 90, 125, 121, 127 | 4 | all four | 26 |
| 23 | 2 | assignment | (4 269, 2) | 2 | 38, 82, 120, 120 | 3 | not clause 1 | 31 |
| 23 | 2 | mirrored | (172, 2) | 2 | 127, 123, 122, 126 | 4 | all four | 36 |
| 24 | 1 | assignment | (145, 1) | 1 | 123, 126, 124, 125 | 4 | all four | 35 |
| 24 | 1 | mirrored | (225, 1) | 1 | 118, 118, 124, 126 | 4 | all four | 39 |
| 24 | 2 | assignment | (309, 2) | 2 | 124, 123, 126, 122 | 4 | all four | 40 |
| 24 | 2 | mirrored | (274, 2), (4 517, 1) | 2 | 121, 123, 118, 44 | 3 | not clauses 1 and 3 | 59 |

- A sign is lost where the age moved to has no live entry: the entries of the trials just before a move were spent at
  age zero.
- A mapping is learned at 80 or more of its last 128 correct, clause 1's bound. A reversal that never crossed counts
  32 blocks, as H-43's reading 2 counts it.
- Told, H-43's arms summed 33 to 46 on these networks. Untold, seed 22's learned 2 mappings of 16.

**Where the switch moved early and stayed, the network learned as if told.** That was six arms. In each, every mapping
was learned and all four of H-43's clauses held. Their reversals summed 35 to 44, against the told arms' 35 to 46 on
the same network and arm.

**Where it moved late, most mappings before the move were lost.** That was four arms, at trials 1 448 to 4 269:
- seed 22 from the mirrored assignment at one trial's delay;
- seed 23 from the assignment at one trial's delay and at two;
- seed 23 from the mirrored assignment at one trial's delay.

Of the five mappings that ran under age zero for most of their trials:

- Four mappings ran under age zero throughout. Three ended at 32 to 42. The fourth, seed 23's second from the
  assignment at two trials' delay, ended at 82.
- Seed 23's first mapping from the mirrored assignment ran under age zero for 1 448 of its 1 536 trials and ended at
  90. It had crossed 40 of 64 in its first block.

Every mapping a move came in, and every one after it, was learned.

**The two wrong moves.** Each was the arm's second move, and each came while a mapping was rising steeply:
- **Seed 22, from the assignment, two trials late.** Moved to age 2 at trial 129, then to age zero at 471, back to age
  2 at 4 318. In block 6, at 44 correct of 64 between 47 and 50, age zero led at all 64 rewards, its mean error
  magnitude 0.064 below age 2's. Under age zero the first mapping, already half learned, went on to 110 of 128. The
  second, all under age zero, ended at 61. The third, whose answers are the first's, crossed 40 of 64 in its eighth
  block, still under age zero, and ended at 119 after the move back.
- **Seed 24, from the mirrored assignment, two trials late.** Moved to age 2 at trial 274, then to age 1 at 4 517 and
  stayed. The third mapping was rising from 24 to 48 correct of 64 a block. In blocks 69 and 70, at 32 and 45,
  age 1 led at 49 and 64 of their rewards. The third mapping went on to 118. The fourth, all under age 1, ended at 44.

On seeds 23 and 24 a wrong age's largest lead over 64 rewards running was 0.0095 to 0.0396 of the reward, arm by arm.
It passed a thirty-second only where the switch moved wrongly. On seed 22's switch arms it reached 0.0338, in the arm
that moved wrongly.

## What it suggests

**The engine's kept counts can name the delay.** A critic per kept age, each learning only from its own age's counts,
put the true age lowest for 64 rewards running by trial 235 in every arm, on every network. In seed 22's untold arms
that held while the network learned nothing. What carries it is not read.

**What failed was the decision, not the reading.** A fixed lead with no latch followed a wrong age twice, both times a
second move while a mapping was rising steeply. A switch that moves once would have stayed at the true age in all
twelve arms; what the network would then learn is not read. Four first moves came late, at a lead fitted on seed 22.

**A formal question for a delay the engine is not told.** A decision ADR could ask the following, with its prediction
and its constants fixed before any run, on the three networks:
- the engine carries one value critic per kept age and consolidates under the age whose score leads;
- the rule's constants: the window, the run, the lead, and whether the choice latches;
- its control: a delay of zero, where the rule must stay at age zero.

This exploration's arms would be its baseline, measured again by its own protocol. Its cost, by arithmetic: three
value weights a unit where the engine holds one, 6 KiB at 1 024 units and `kept_addresses` 2, and three critic
readings a reward in place of one.

## What did not run

- **Questions 1 and 2 on seeds 23 and 24.** The rule for this exploration extends question 3 alone.
- **A delay of zero under the switch.** With no delay the true age is the one the switch starts at, so any move is a
  mistake. Whether the switch stays at age zero on time is the control a later round would need.
- Other leads, window lengths or runs for the switch, and a switch that moves once and stays.
- Delays beyond two, and more ages kept than the delay needs.
- Accuracy by stimulus, which would test the reading in section 2.

## The runs

| Test | Run | Commit | Seconds |
| :--- | :--- | :--- | ---: |
| `a_delay_told_read_by_age_one_trial_late_on_seed_22_from_the_assignment_explore` | [40215359541](https://github.com/DescentVTT/VirtualCortex/actions/runs/40215359541) | `4f64c1b` | 1129 |
| `a_delay_told_read_by_age_one_trial_late_on_seed_22_from_the_mirrored_assignment_explore` | [40215359541](https://github.com/DescentVTT/VirtualCortex/actions/runs/40215359541) | `4f64c1b` | 1134 |
| `a_delay_told_read_by_age_two_trials_late_on_seed_22_from_the_assignment_explore` | [40215359541](https://github.com/DescentVTT/VirtualCortex/actions/runs/40215359541) | `4f64c1b` | 1111 |
| `a_delay_told_read_by_age_two_trials_late_on_seed_22_from_the_mirrored_assignment_explore` | [40215359541](https://github.com/DescentVTT/VirtualCortex/actions/runs/40215359541) | `4f64c1b` | 1105 |
| `a_delay_untold_read_by_age_one_trial_late_on_seed_22_from_the_assignment_explore` | [40215359541](https://github.com/DescentVTT/VirtualCortex/actions/runs/40215359541) | `4f64c1b` | 1096 |
| `a_delay_untold_read_by_age_one_trial_late_on_seed_22_from_the_mirrored_assignment_explore` | [40215359541](https://github.com/DescentVTT/VirtualCortex/actions/runs/40215359541) | `4f64c1b` | 1077 |
| `a_delay_untold_read_by_age_two_trials_late_on_seed_22_from_the_assignment_explore` | [40215359541](https://github.com/DescentVTT/VirtualCortex/actions/runs/40215359541) | `4f64c1b` | 1089 |
| `a_delay_untold_read_by_age_two_trials_late_on_seed_22_from_the_mirrored_assignment_explore` | [40215359541](https://github.com/DescentVTT/VirtualCortex/actions/runs/40215359541) | `4f64c1b` | 1114 |
| `a_delay_untold_switched_by_age_one_trial_late_on_seed_22_from_the_assignment_explore` | [40217289629](https://github.com/DescentVTT/VirtualCortex/actions/runs/40217289629) | `87ba360` | 1077 |
| `a_delay_untold_switched_by_age_one_trial_late_on_seed_22_from_the_mirrored_assignment_explore` | [40217289629](https://github.com/DescentVTT/VirtualCortex/actions/runs/40217289629) | `87ba360` | 1107 |
| `a_delay_untold_switched_by_age_two_trials_late_on_seed_22_from_the_assignment_explore` | [40217289629](https://github.com/DescentVTT/VirtualCortex/actions/runs/40217289629) | `87ba360` | 1118 |
| `a_delay_untold_switched_by_age_two_trials_late_on_seed_22_from_the_mirrored_assignment_explore` | [40217289629](https://github.com/DescentVTT/VirtualCortex/actions/runs/40217289629) | `87ba360` | 1070 |
| `a_delay_untold_switched_by_age_one_trial_late_on_seed_23_from_the_assignment_explore` | [40219023428](https://github.com/DescentVTT/VirtualCortex/actions/runs/40219023428) | `7933286` | 1113 |
| `a_delay_untold_switched_by_age_one_trial_late_on_seed_23_from_the_mirrored_assignment_explore` | [40219023428](https://github.com/DescentVTT/VirtualCortex/actions/runs/40219023428) | `7933286` | 1113 |
| `a_delay_untold_switched_by_age_one_trial_late_on_seed_24_from_the_assignment_explore` | [40219023428](https://github.com/DescentVTT/VirtualCortex/actions/runs/40219023428) | `7933286` | 1123 |
| `a_delay_untold_switched_by_age_one_trial_late_on_seed_24_from_the_mirrored_assignment_explore` | [40219023428](https://github.com/DescentVTT/VirtualCortex/actions/runs/40219023428) | `7933286` | 1067 |
| `a_delay_untold_switched_by_age_two_trials_late_on_seed_23_from_the_assignment_explore` | [40219023428](https://github.com/DescentVTT/VirtualCortex/actions/runs/40219023428) | `7933286` | 1096 |
| `a_delay_untold_switched_by_age_two_trials_late_on_seed_23_from_the_mirrored_assignment_explore` | [40219023428](https://github.com/DescentVTT/VirtualCortex/actions/runs/40219023428) | `7933286` | 1088 |
| `a_delay_untold_switched_by_age_two_trials_late_on_seed_24_from_the_assignment_explore` | [40219023428](https://github.com/DescentVTT/VirtualCortex/actions/runs/40219023428) | `7933286` | 1083 |
| `a_delay_untold_switched_by_age_two_trials_late_on_seed_24_from_the_mirrored_assignment_explore` | [40219023428](https://github.com/DescentVTT/VirtualCortex/actions/runs/40219023428) | `7933286` | 1135 |
