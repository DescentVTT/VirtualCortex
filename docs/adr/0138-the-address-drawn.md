---
status: accepted
date: 2026-09-30
depends-on: ADR-0137
decision-makers: VirtualCortex maintainers
---

# ADR-0138: The address drawn — after H-24, the eligibility's specificity under a global reward is taken as the engine drawing the address itself: the reward's sources are the units the critic's window counted since the previous reward, the state the engine was given, and its targets the output channel the engine's own selection chose, an efference copy; so the host no longer says which stimulus it drew; the drawing a call of the executor between ticks, refused without the critic's window, every other delivery the run it was bit for bit; H-25 written before any run, with H-23's clauses and H-24's clause on the network; brief 058 builds it and runs H-25 once

## Context and Problem Statement

[ADR-0137](0137-the-reward-unaddressed-measured.md) read **H-24 no, on clause 1**. With the dopamine term delivered to every synapse on H-23's configuration:
- the critic held each stimulus's expected reward, and the network outside the pairs held in sum;
- the learning did not follow: 64 to 89 of each mapping's last 128 correct, against H-23's 115 to 124.

By H-24's stopping rule, step 5, the next decision is an ADR on the eligibility's specificity under a global reward. **The maintainers took it on 2026-09-30 in two steps.** This round is the first: the engine's own address, [ADR-0136](0136-the-reward-unaddressed.md)'s option 1(c). The second is a neural form of the target side, a competition between the readouts or a feedback that tags the chosen channel. It is to replace the efference copy once this round has read what an address drawn from the engine's own state does.

**What H-24 read about the specificity:**
- **The answer's pairs moved as much as the others.** The reward's net in the answer's pairs was 0.8 to 2.9 per cent of the weight it moved there, and the other two pairs moved as much, the other way on net.
- **The network outside the pairs moved five to six times as much weight.** Nearly every one of the 23 052 synapses outside the pairs left its image weight, while each cell's sum stayed near the image's.
- **From the second mapping both stimuli selected readout 1 more often**, the readout onto which the image's couplings are the larger for both.

The pair each trial selected moved 0.75 to 0.99 of what H-23's did. What the global delivery added was the other three pairs and the network, under the same signal.

**Why the address narrowed both sides**, in the tree's own words (`runtime/cortex-runtime/src/task.rs`, `Delivery::Addressed`, [ADR-0068](0068-the-reward-addressed.md)): *"The presynaptic side narrows the set because the reward is consolidated at the next presynaptic spike, which is the next presentation of a stimulus: without the narrowing the reward of one trial would reach the other stimulus's synapses at half the trials."*
- The postsynaptic side is the readout the engine selected, so that the reward reaches the answer that was given and not the other.
- The address therefore carries two things. The **sources** are which stimulus the host drew, a label the engine does not have. The **targets** are which output channel the engine's own selection chose.

**What the engine already has for each side:**
- **The sources.** The critic's window ([ADR-0133](0133-the-critics-window.md), [ADR-0134](0134-the-critics-window-built.md)) counts each unit's spikes within the shortest synaptic delay after each reward. `Executor::features` holds those counts, in unit order, until the next reward zeroes them. At a trial's end they are the spikes of that trial's window. H-23 read them at about 52.4 a trial: 50.8 from the presented stimulus's own units, its volley, and about 1.6 from the rest ([ADR-0135](0135-the-critics-window-measured.md)). So the units with a count at a trial's end are the presented stimulus's set, give or take a background spike or two, drawn from the engine's own train with no label.
- **The targets.** The selection is `cortex-basal-ganglia`'s gating over the counts the readout reads from the engine's own train. The readout sets are the body's interface: which units drive which output, configured once, not a label per trial. The channel the gate chose is the engine's own action, and addressing its units is an efference copy, not the host's knowledge of the answer.

**The account and the literature.** A global neuromodulator assigns credit only as well as the eligibility is specific. Where it is not, models narrow it with what the agent knows of its own state and action:
- the critic's state representation (Potjans, Morrison and Diesmann 2009; Frémaux, Sprekeler and Gerstner 2013);
- a feedback from the chosen action that tags the synapses that led to it (Roelfsema and van Ooyen 2005; Rombouts, Bohte and Roelfsema 2015).

The account predicts that an address the engine draws from its own window and its own choice does what the host's address did.

**Where the engine's premise differs:**
- **The window's sources are the stimulus's set only approximately.** They admit background units, about 1.6 a trial, whose synapses onto the chosen readout the host's address never reached. A volley unit that did not fire within the window would be left out.
- **The window's fit rests on the host** presenting the next stimulus at the reward ([ADR-0133](0133-the-critics-window.md)).
- **The target side is the task's call**, the efference copy composed in the task over the body's sets, not a network mechanism. A neural form of it, a competition between the readouts or a feedback that tags the chosen channel, is the maintainers' second step and not this round's.

## Decision Drivers

- **A measured need**: H-24's readings of where the global reward's consolidation went, and the tree's own account of why the address narrowed each side.
- **The maintainers' choice** (2026-09-30): two steps, the drawn address first as the reference, a neural form of the target side second.
- **The engine's own inputs only** for the per-trial part: no stimulus index and no stimulus set from the host; the output channels are the body's interface, stated as such.
- **One change from H-23**: the sources drawn by the engine in place of the host's. The targets, the critic, its window, the image and every rule are H-23's.
- **Unset, bit for bit**: the global and the addressed deliveries run as they did.
- Latest ≠ Newest: a set of flags written from counts the executor already keeps; no dependency.

## Considered Options

1. **The sources**:
   - (a) the units with a nonzero window count at the trial's end;
   - (b) the units with any spike since the previous reward, the whole-interval count;
   - (c) the units whose window count is at least some threshold.
2. **The targets**:
   - (a) the output channel the selection chose, as the addressed delivery's targets;
   - (b) every unit;
   - (c) the units that fired most within a window of the engine's own.
3. **Where the drawing lives**: (a) a call of the executor that writes the sources from its own counts and the targets as given; (b) the task reading `Executor::features` and passing them to `address`.
4. **The clauses**: (a) H-23's four; (b) H-23's clauses 1 to 3 and H-24's clause 3, the value as a reading.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(b).**

### The drawing (Specified; brief 058 builds it)

- **The call.** Between ticks, the executor sets the address's sources to the units whose critic count is not zero, and its targets to the units the caller gives. It is refused while the critic or its window is unset: without the window the counts are every spike since the reward, the whole network, which is H-24's reach again.
- **The task.** A delivery beside the two, which calls it with the selected readout's units, and with no targets at a tie, as the addressed delivery does. The global and the addressed deliveries are untouched, and every run under them is the run it was.
- **When.** The task writes the address between a trial's last tick and its reward, so the counts it draws from are that trial's window. Nothing of the image or the records changes, and no format moves: the address is an input between ticks, as the reward is.
- **Where it lives** (option 3(a)). In the executor, so that the per-trial part of the address is written from the engine's own state by the engine. The task gives only the body's channel its own selection chose.

### H-25 (Hypothesis; written before any run)

**On H-23's configuration with the address's sources drawn by the engine in place of the host's:**
- **clause 1, the learning holds**: in both arms every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the engine revises**: in both arms each of the three reversals passes 40 of 64 within 23 blocks of its flip, as H-23's clause 3;
- **clause 4, the network holds**: in both arms, at every block's end, the summed magnitude of every excitatory synapse outside the four stimulus–readout couplings lies within 0.75 and 1.25 of the image's, as H-24's clause 3.

**Yes** when all four hold in both arms; otherwise **no**, with the clauses that failed.

**The account's prediction, written first**: yes. The drawn sources are the presented stimulus's set but for about 1.6 background units a trial, so the reward reaches what the host's address reached, and a little more.

**Readings, no clause**:
- the drawn sources per trial beside the host's: the presented stimulus's units in and out, and the other units in, by class;
- the synapses outside the pairs that moved, and where, by H-24's cells;
- where the consolidation went, by H-24's measure;
- the couplings' separation, the reversal speeds, the value beside $2p - 1$ and the troughs, beside H-23's;
- the inhibitory sum's course.

### H-25's stopping rule

1. **One round**: brief 058 builds the drawing, its tests and its ADR, then runs H-25. H-25's constants and clauses are committed before the first rewarded run.
2. **A calibration** stops the round before any rewarded run, and is a finding, if any of these fails:
   - every pinned number of the tree holds, the determinism pin among them;
   - H-23's first block reproduces under the host's address.
3. **Yes**: the learning loop no longer takes the stimulus's identity from the host, and the configuration is named with the drawn address. The next decision is an ADR choosing among:
   - the operating regime;
   - another size;
   - the rule held by the network reopened;
   - a critic carried by a population;
   - the second step the maintainers planned: a neural form of the target side, a competition between the readouts or a feedback that tags the chosen channel, with this round's run as the reference it is measured against;
   - a task the engine has not been asked.

   It is named and not taken.
4. **No on clause 4**: the window's background units move the network the learning stands on. The next decision is an ADR on what the window admits, with this round's readings as its need.
5. **Otherwise no on clause 1, 2 or 3**: the host's sources carried what the window's do not. The next decision is an ADR with the drawn and the host's sources side by side as its need.
6. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: per trial, the host gives the reward's sign and nothing else the learning reads. The stimulus's identity, which the address took from the host since H-14, is drawn from the engine's own spikes.
- Good: one change from H-23, whose configuration stands beside the round, and the two deliveries before it untouched.
- Bad: the drawing needs the critic's window, and inherits its fit: a host that presents its next stimulus later than one shortest delay after its reward gives the drawing the background alone.
- Bad: the target side is an efference copy the task composes over the body's sets, not a mechanism of the network. The round says so, and the neural forms are the second step.
- Neutral: files under `src/` change, so the dispatch's scope is `both` ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)); two weekly tests of H-23's cost raise the plan from about 41 to about 45 per cent of the bound.

## Alternatives considered and why rejected

- **Option 1(b), every spike since the reward**: the whole network, which is H-24's reach.
- **Option 1(c), a threshold on the count**: in the window every volley unit fires once, so a threshold above one would drop the stimulus with the background.
- **Option 2(b), every unit as a target**: the other readout's pairs moved as much as the answer's under H-24. The target side is where the answer is told from the other.
- **Option 2(c), targets drawn from activity**: the readouts' window is the task's timing, and the channel the gate chose is already the engine's own.
- **Option 3(b), the task passing the counts**: the per-trial part of the address would still be written by the host's code, from the engine's state rather than the host's label. The call keeps it in the engine.
- **Option 4(a), H-23's four clauses**: the drawn sources reach background units' synapses the host's address never did, so the network needs H-24's clause. The value's clause is kept as a reading, since the critic does not change.

## Confirmation

`briefs/058_the-address-drawn.md` builds the drawing and runs H-25 once. Whitepaper 4.85.0 carries H-25 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
