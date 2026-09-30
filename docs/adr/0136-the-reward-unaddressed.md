---
status: accepted
date: 2026-09-30
depends-on: ADR-0135
decision-makers: VirtualCortex maintainers
---

# ADR-0136: The reward unaddressed — after H-23, the learning line removes the last label the host gives it: the dopamine term, delivered since H-14 only to the synapses from the stimulus the task presented onto the readout the engine selected, reaches every synapse again, as it did before ADR-0068, now under the engine's own critic with its window; nothing of the engine changes; H-24 written before any run, with H-23's learning clauses and a clause that the network the learning stands on holds; brief 057 runs it once

## Context and Problem Statement

[ADR-0135](0135-the-critics-window-measured.md) read **H-23 yes**. With the engine's critic counting only within the shortest synaptic delay after each reward, H-22's configuration:
- learned every mapping in both arms, with its couplings bounded;
- revised each reversal within H-21's speeds;
- came to hold each stimulus's expected reward.

By H-23's stopping rule, step 3, the configuration is named, and the next decision is an ADR choosing among the operating regime, another size, the rule held by the network reopened and a critic carried by a population. **The maintainers added a fifth, the reward's address, and took it on 2026-09-30.** [ADR-0111](0111-a-rule-held-by-the-network.md) set the precedent when it added a representation of the rule beside H-20's four.

**The address** (`runtime/cortex-runtime/src/task.rs`, [ADR-0068](0068-the-reward-addressed.md)):
- **What it is.** Since H-14 every learning run has used `Delivery::Addressed`. Between a trial's last tick and its reward, the task writes `exec.address(sources, targets)`: the sources are the units of the stimulus set it presented, and the targets are the units of the readout set the engine selected.
- **What it does.** Under it the dopamine term reaches those synapses alone. Every other excitatory synapse consolidates under the modulation baseline, which is zero in the learning configuration, so it never moves. H-14 to H-23 read exactly that, and H-20 to H-23 assert it (`punished_held`).
- **What it leaks.** The sources are a label only the host has: which stimulus it drew, and where that stimulus's units lie. H-22 and H-23 removed that label from the critic. The address is where the learning loop still takes it.

**The global delivery.** `Delivery::Global` calls `exec.address_all()`: every unit a source and a target, the rule before ADR-0068. Under the signed gate ([ADR-0094](0094-the-signed-gate-built.md)) every excitatory synapse then consolidates under $\mathrm{clamp}(b + d, -1, 1)$, with $b$ the baseline, zero, and $d$ the dopamine term.

The global delivery was last measured by [ADR-0066](0066-the-reward-path-measured-again.md), under a configuration that no longer exists:
- the prior unsettled;
- the modulation baseline 0.5;
- the raw reward, with no critic;
- the stimulus as two messages of 1.25, under which a unit fired more than once ([ADR-0074](0074-a-stimulus-that-fires-once.md), F-46, F-47);
- no signed gate, and the inhibitory rule at its default target under the same baseline.

It read 53 and 49 of the last 128 correct at 256 and 1 024 units, against the 80 asked for, every coupling falling alike. It has not been measured since the stimulus, the network, the gate, the inhibition and the critic changed.

**The account and the literature.** In reward-modulated plasticity the neuromodulator is global, and the eligibility trace assigns the credit: a synapse whose pairing preceded the reward holds the trace the reward consolidates (Izhikevich 2007; Florian 2007; Legenstein, Pecevski and Maass 2008). Frémaux, Sprekeler and Gerstner (2010) showed that such a rule learns only when the reward's expectation is taken from it. Otherwise every synapse's mean eligibility times the mean reward, an unsupervised bias, dominates what the rule learns. Removing that expectation is what a critic does. **The account predicts that with the engine's own critic the global delivery learns.**

**Where the engine's premise differs**, so that the account is a prediction to be measured, not a result to be assumed:
- **The trace's composition at this network.** The volley's pairing and the background's are summed in one trace (F-46). On the settled image the presented stimulus's summed eligibility onto the readouts was net potentiation on average, and positive after 53 of 64 trials ([ADR-0077](0077-the-background-side.md)). A global reward learns from how the trace onto the readout that answered differs from the trace onto the one that did not. The address never needed that difference, and no round has read it.
- **The whole network under the reward.** Every excitatory synapse now consolidates at every presynaptic spike under the dopamine term. When the critic is right the term is near zero on average, but not trial by trial. Over 7 680 trials its variance times each synapse's eligibility is a random walk the address kept out of the network. H-15 read what consolidation everywhere did at a baseline of 0.5: the couplings carried up by a third to a half ([ADR-0083](0083-plasticity-everywhere-measured.md)).
- **Integer arithmetic.** Each consolidation moves a rounded share of the trace, and a small term times a small trace rounds to nothing or to one LSB, now over every synapse of the network.
- **A fixed seeded prior** at 1 024 units, one drive and one gain.

So **no prediction is made for the verdict**. The literature's account predicts yes, and ADR-0066's reading was a no under a configuration that no longer exists. The round decides between them.

## Decision Drivers

- **A measured need**: after H-23 the address is the one label the learning loop still takes from the host.
- **The maintainers' choice** (2026-09-30), beside the four H-23's rule named.
- **Nothing of the engine changes.** `Delivery::Global` and `address_all` are the tree's; H-23's image, critic and window are kept.
- **One change at a time**: only the delivery moves from H-23's configuration.
- **The learning's measure stays H-23's**, and a clause is added for what the address kept still: the rest of the network.
- **The weekly job's budget**: after ADR-0135 the plan is about 34 per cent of the bound, and two arms of H-23's cost add about four points.

## Considered Options

1. **The delivery**:
   - (a) global, every unit a source and a target;
   - (b) the sources global and the targets the selected readout;
   - (c) an address the engine draws from its own activity, the units that fired within the critic's window for instance.
2. **The configuration**: (a) H-23's, unchanged but for the delivery; (b) H-23's with the baseline or the signed gate moved for the global delivery.
3. **The clauses**:
   - (a) H-23's four;
   - (b) H-23's learning and bounded couplings, a clause that the network holds, and the reversals and the prediction as readings.

## Decision Outcome

**Options 1(a), 2(a) and 3(b).**

### The configuration (nothing of the engine changes)

H-23's configuration and arms, the image H-23's arms decode, its critic at shift 9 and scale 2 and its window of 100 ticks, with the task's delivery `Delivery::Global` in place of `Delivery::Addressed`. The delivery is the task's and not the image's, so the image is H-23's bit for bit.

The selection stays the task's readout over its sets. The readout sets are the body's interface, what each answer is, and not a label of the learning. That the sets are the host's is stated, not removed.

### H-24 (Hypothesis; written before any run)

**On H-23's configuration with the reward delivered to every synapse:**
- **clause 1, the learning holds**: in both arms every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the network holds**: in both arms, at every block's end, the summed magnitude of every excitatory synapse outside the four stimulus–readout couplings lies within **0.75 and 1.25** of the image's.

**Yes** when all three hold in both arms; otherwise **no**, with the clauses that failed. No prediction is made for the verdict.

**Readings, no clause**:
- the four couplings per block, beside H-23's;
- outside the couplings, by the class of each synapse's source and target:
  - the excitatory sum per block;
  - the fraction of synapses moved from the image, the largest move, and the synapses at either rail;
- **where the reward's consolidation went** per block: the weight moved in the answer's pairs, in the other pairs and outside them;
- the population's rate by class per block;
- the reversal speeds, the value beside $2p - 1$ and the troughs after each flip, beside H-23's;
- the inhibitory sum's course.

### H-24's stopping rule

1. **One round**: brief 057 runs H-24 once. Nothing of the engine changes. H-24's constants and clauses are committed before the first rewarded run.
2. **A calibration** stops the round before any rewarded run, and is a finding, if any of these fails:
   - every pinned number of the tree holds;
   - H-23's first block, from H-23's image under the addressed delivery, reproduces H-23's tables.
3. **Yes**: the learning loop's credit assignment is the engine's own, and the configuration is named with the global delivery. The next decision is an ADR choosing among:
   - the operating regime;
   - another size;
   - the rule held by the network reopened;
   - a critic carried by a population;
   - a task the engine has not been asked, more answers than two or a reward delayed past a trial.

   It is named and not taken.
4. **No on clause 3**, whatever clauses 1 and 2 read: the global reward moves the network the learning stands on. The next decision is an ADR on bounding what the reward moves outside the pairs, with this round's readings as its need.
5. **Otherwise no on clause 1 or 2**: the address carried a credit assignment the eligibility does not. The next decision is an ADR on the eligibility's specificity under a global reward, with the readings of where the consolidation went as its need.
6. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: the one label the learning loop still takes from the host is asked about, with nothing of the engine changed.
- Good: H-23's configuration stands beside the round as its comparison, one variable apart.
- Bad: the assertion H-14 to H-23 held — no excitatory synapse outside the pairs moves — does not hold under a global delivery, by construction. Clause 3 and the readings stand in its place, and the rest of the network is no longer the image's bit for bit.
- Bad: the readout sets and the selection stay the host's. A yes removes the learning's label, not the body's interface.
- Neutral: two weekly tests of about H-23's cost, the plan to about 38 per cent of the bound; no source changes, so the dispatch's scope is `exhaustive` ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)).

## Alternatives considered and why rejected

- **Option 1(b), the sources global and the targets the selected readout**: the host would still say which synapses learn, through the readout's set. It is half the change, with no reading the whole one lacks.
- **Option 1(c), an address the engine draws from its own activity**: a new mechanism before the global delivery is measured under this configuration. It is named for a no on clause 1 or 2.
- **Option 2(b), the baseline or the gate moved**: two changes at once, and the global delivery would no longer be measured on the configuration H-23 named.
- **Option 3(a), H-23's four clauses**: a global delivery that learns more slowly would fail the reversal clause and hide whether it learns at all. The reversals are read beside H-23's; clause 1 already fails a mapping not learned within its span. And H-23's clauses have nothing for the rest of the network, which the address kept still.

## Confirmation

`briefs/057_the-reward-unaddressed.md` runs H-24 once. Whitepaper 4.83.0 carries H-24 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
