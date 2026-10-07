---
status: accepted
date: 2026-10-07
depends-on: ADR-0142
decision-makers: VirtualCortex maintainers
---

# ADR-0143: The gate's output delivered — after H-26, the competition of the second step is taken as the basal ganglia's output reaching the network: the selection is made at the readout window's close, and the channel not selected is inhibited through the span in which the pair rule still pairs the volley, so that the eligibility onto the readout that lost is what the network's own activity leaves it; the address's targets are then released to every unit, its sources still drawn by the engine; unset every run the run it was; H-27 written before any run, with H-25's four clauses and the release alone as its control; brief 060 builds it and runs H-27 once

## Context and Problem Statement

[ADR-0142](0142-the-readouts-own-competition-measured.md) read **H-26 no, at the bar**. On H-25's runs an open-loop shadow of the released target side cost 0.486 to 0.602 of the learning signal, past half in six mappings of eight. By H-26's stopping rule, step 4, the next decision is an ADR on the lateral competition whitepaper §5.2.5 specifies, with that cost as its need and the attention-gated feedback as its fallback. **The maintainers took the gate's output delivered to the network on 2026-10-07**, with the release alone as the round's control.

**What ADR-0142 hands the build:**
- **The readout that lost fires as the winner does outside the window.** In the readout window, 500 ticks, it fires 0.57 to 0.62 of the selected readout's spikes; over the whole trial, 16 384 ticks, 0.95 to 0.97.
- **It carries the winner's eligibility by magnitude**, 0.93 to 1.06 of it, and a quarter to a half of it on net.
- **The release's first-order loss is about half.** The shadow's selected side delivered 1.01 to 1.15 of the run's signal, so its whole signal was 0.51 to 0.63 of the run's. ADR-0142 left that reading to this ADR.

**The rules a competition acts through**, read on 2026-10-07:
- **The pair rule** (`crates/cortex-core/src/dynamics/synapse.rs`, `step_stdp`) is nearest-neighbour. At a presynaptic spike $t$, with $p$ the block's previous presynaptic stamp and $q$ the target's last somatic spike:
  - the trace gains $A_+ (1 - 2^{-11})^{q - p}$ when $p < q \le t$;
  - it loses $A_- (1 - 2^{-11})^{t - q}$, scaled by the weight, when $q < t$.

  So a stimulus unit's synapse onto a readout unit is potentiated by how soon after its last volley that unit **last** fired. A unit that fired in the window and again later is read at the later spike.
- **The selection** (`runtime/cortex-runtime/src/task.rs`, `Readout::select`) counts the readouts' spikes in the window, 100 to 600 ticks after the trial's first, and is made after the trial's last tick. Nothing depends on it until the address is written, so it can be made at the window's close with the same counts and the same result.
- **The gate** (`crates/cortex-basal-ganglia`) computes each channel's net output to the thalamus, `gpi_snr_inhibition`, above zero for the channel not selected. Nothing reads it but the flag. §5.2.5 marks lateral competition *Specified*.
- **An inhibition into a set** exists in the task as ADR-0076's cancel: negative basal messages into a set's units between ticks, one message clamped at −2.0 and scaled by the gain (F-47). The basal compartment leaks over $2^9$ ticks and the soma over $2^{11}$ (`BASAL_LEAK_SHIFT`, `SOMA_LEAK_SHIFT`), so silencing a unit for thousands of ticks takes a schedule of messages, not one.

**The account and the literature.** In models of the basal ganglia the selected channel is released and the others stay under the output nuclei's inhibition, so the losing channels are quiet when the dopamine arrives and their synapses carry little eligibility (Gurney, Prescott and Redgrave 2001; Frank 2005). The account predicts that inhibiting the channel not selected makes a released target side affordable.

**Where the engine's premise differs**, so that two effects pull against each other:
- **The inhibition comes after the response.** The selection is read from the window, so the losing readout has already fired in it, about nine spikes a trial against the winner's fifteen.
- **The pair rule reads the last spike.** Silencing the losing readout after the window removes the background spikes that pair with the volley later. It also keeps the units that answered in the window from firing again, so their last spike stays the one nearest the volley, where the potentiation is largest.
- **The readouts are 357 units each, interleaved on one ring**, and the inhibitory rule holds every unit to a target rate ([ADR-0129](0129-a-target-the-network-fires-at-measured.md)). A readout silenced for part of each trial fires below it.

So the account is a prediction to be measured, not a result to be assumed.

## Decision Drivers

- **A measured need**: ADR-0142's cost, its readings of where the losing readout's eligibility comes from, and its first-order reading of the release alone.
- **The maintainers' choice** (2026-10-07): the gate's output, with the release alone as the control.
- **The mechanism is the engine's own**: the gate's output is a field `cortex-basal-ganglia` already computes, delivered by a form the task already has.
- **Unset, bit for bit**, and nothing of the image or the records changes.
- **H-25 is the reference**: its clauses are the measure, so that the target side released is read against the target side addressed.
- **The weekly job's budget**: after ADR-0142 the plan is about 39 per cent of the bound.

## Considered Options

1. **The competition**:
   - (a) the gate's output delivered: the channel not selected inhibited after the window's close;
   - (b) lateral inhibitory wiring between the readouts, acting during the response;
   - (c) the attention-gated feedback, a tag on the selected readout's units.
2. **The span of the inhibition**:
   - (a) from the window's close until the pair rule's factor for a spike paired with the volley has fallen to its second time constant, tick $2^{12}$ of the trial;
   - (b) to the trial's end;
   - (c) a span fitted to the task's learning.
3. **The amount**: (a) the least schedule of messages that silences a unit through the span, derived from the membrane's rule before any run and checked on a frozen run; (b) an amount scaled by the gate's output.
4. **The arms**: (a) the released address with the gate's output, and the released address alone as the control, each from both assignments; (b) the gate's output alone.
5. **The clauses**: (a) H-25's four; (b) H-26's cost measured closed-loop as the clause.

## Decision Outcome

**Options 1(a), 2(a), 3(a), 4(a) and 5(a).**

### The gate's output (Specified; brief 060 builds it)

- **When the selection is made.** At the readout window's close, from the counts the selection reads today. The selection, and everything that depends on it, is what it was.
- **What is delivered.** Into every unit of the channel the gate did not select, negative basal messages by a schedule, from the window's close to tick $2^{12}$ of the trial, two time constants of the pair rule after the trial's first tick. Nothing is delivered at a tie, where no channel was selected.
- **The schedule** is derived before any run: the least messages a tick and the fewest ticks that keep a unit of the settled network from firing through the span under the drive, by an oracle of the membrane's rule, as [ADR-0076](0076-two-injections.md) derived its cancel.
- **It is checked on a frozen run before any rewarded one**: over a block of 64 trials with every weight frozen, the losing channel's spikes within the span are at most a tenth of the selected channel's in the same span. A schedule that fails the check stops the round as a finding.
- **Where it lives.** In the task, beside the cancel: a parameter of the task, unset by default. Unset, a trial is the trial it was, bit for bit. Nothing of the image or the records changes.

### The released address

A delivery beside the three: the sources drawn by the engine as `Delivery::Drawn` draws them, and every unit a target, at a tie as at any trial. It is refused without the critic's window, as the drawn delivery is.

### H-27 (Hypothesis; written before any run)

**On H-25's configuration, with the address's targets released to every unit and the gate's output delivered:**
- **clause 1, the learning holds**: in both arms every mapping is learned, at least 80 of its last 128 trials correct;
- **clause 2, the couplings stay bounded**: no stimulus–readout coupling passes 1.30 of its image's at any block's end;
- **clause 3, the engine revises**: each of the three reversals passes 40 of 64 within 23 blocks of its flip;
- **clause 4, the network holds**: at every block's end the summed magnitude of every excitatory synapse outside the four stimulus–readout couplings lies within 0.75 and 1.25 of the image's.

**Yes** when all four hold in both arms; otherwise **no**, with the clauses that failed.

**No prediction is made for the verdict.** The account predicts yes. The pair rule's last spike and the response already made in the window pull the other way, and no round has read which is the larger.

**The control: the released address alone**, from both assignments, read by the same four clauses. **Predicted: no**, on clause 1 or 3, since ADR-0142's first-order reading leaves it about half the signal and a reversal has 32 blocks.

**Readings, no clause**, for each arm:
- H-26's cost and signal, now closed-loop: where the consolidation went on the selected side and on the side not selected, by the network's oracle;
- the losing channel's spikes within the span and outside it, beside the selected channel's;
- the eligibility at each reward onto each readout, by magnitude and on net, beside ADR-0142's;
- the messages delivered a trial;
- the population's rate by class and **the inhibitory sum's course**, since a readout silenced for part of each trial fires below the inhibitory rule's target;
- the couplings' separation, the reversal speeds, the value and the troughs, beside H-25's.

### H-27's stopping rule

1. **One round**: brief 060 builds the gate's output and the released delivery, their tests and their ADR, then runs H-27. The schedule, the span and H-27's constants are committed before the first rewarded run.
2. **A calibration** stops the round before any rewarded run, and is a finding, if any of these fails:
   - every pinned number of the tree holds with the gate's output unset;
   - H-25's first block reproduces under the drawn address;
   - the schedule passes its frozen check.
3. **Yes**: the address's target side is the network's own, and the configuration is named with the released address and the gate's output. If the control holds too, it is named with the release alone, the simpler. The next decision is an ADR choosing among the operating regime, another size, the rule held by the network reopened, a critic carried by a population and a task the engine has not been asked, named and not taken.
4. **No on clause 4**: the inhibition or the release moves the network the learning stands on. The next decision is an ADR on the inhibition's span and amount.
5. **Otherwise no on clause 1, 2 or 3**: the gate's output after the window does not make the eligibility specific enough. The next decision is an ADR on the attention-gated feedback, the fallback ADR-0141 named, with this round's readings as its need.
6. **No constant moves after a rewarded run, and there is no second attempt.**

## Consequences

- Good: the competition is a field the engine already computes, delivered by a form the task already has, and it acts on the span ADR-0142 read as the losing readout's share.
- Good: the control separates what the gate's output adds from what the release already keeps, which ADR-0142 could read only to first order.
- Good: unset, nothing changes; H-25 stands as the reference.
- Bad: the inhibition follows the response. What the losing readout fired in the window is not undone, and the pair rule's last spike may keep its potentiation.
- Bad: a readout silenced for about a fifth of every trial it loses fires below the inhibitory rule's target, and the rule will answer. The inhibitory sum's course is read for it and no clause holds it.
- Bad: it is the output nuclei's inhibition composed by the task over the body's channels, not lateral inhibition inside the network. §5.2.5's lateral competition stays Specified.
- Neutral: files under `src/` change, so the dispatch's scope is `both` ([ADR-0075](0075-the-dispatch-scope-follows-the-diff.md)); four weekly tests of about H-25's cost raise the plan from about 39 to about 48 per cent of the bound.

## Alternatives considered and why rejected

- **Option 1(b), lateral wiring between the readouts**: it would act during the response, which the gate's output cannot. But the readouts answer with nine to fifteen spikes among 357 units, and added inhibition moved a threshold rather than sharpening a contrast in [ADR-0120](0120-the-contexts-own-inhibition-measured.md). It is a change to the settled network every learning reading starts from.
- **Option 1(c), the attention-gated feedback**: the most controllable, and a new mechanism of plasticity. It is the fallback step 5 names.
- **Option 2(b), to the trial's end**: three quarters of each trial silenced for no pairing the rule still makes, and a larger pull on the inhibitory rule.
- **Option 2(c), a span fitted to the learning**: the tuning the line's rules exist to avoid.
- **Option 3(b), an amount scaled by the gate's output**: the output is a difference of counts, and a narrow win would leave the loser half silenced. Whether the loser is silenced is what the round asks.
- **Option 4(b), the gate's output alone**: without the control a yes would not say whether the gate's output was needed.
- **Option 5(b), the cost as the clause**: H-26's rule read a shadow. Closed-loop, the measure of the target side is whether the engine learns as H-25 did.

## Confirmation

`briefs/060_the-gates-output-delivered.md` builds it and runs H-27 once. Whitepaper 4.89.0 carries H-27 in §11.1 with its stopping rule, and §9's row. `npm run spec` holds the links, the rows, the brief's sections and the version.
