---
status: accepted
date: 2026-09-25
depends-on: ADR-0097
decision-makers: VirtualCortex maintainers
---

# ADR-0098: The integration model — every awake unit integrated every tick, as built, since the maintainers prefer it and ADR-0097's budget shows that service on arrival buys nothing without a catch-up the membrane does not have; Appendix A's three targets met in two modes: a real-time mode, with a body attached, whose unit count is what the budget allows at the input's density, and an offline mode, with none, whose unit count is what memory holds and whose run is slower than real time; F-49 resolved; five levers on the engine's speed named, and the three that change no rule taken first, before the learning line resumes

## Context and Problem Statement

[ADR-0097](0097-the-active-set-measured.md) counted the executor's turns on the reference network. One message of 0.125 keeps a unit awake 2 503 ticks (25 ms) at the gain 1.75, so under [ADR-0044](0044-reference-network.md)'s drive the executor serves 99.99 per cent of the units on every tick at 1 024 and at 4 096 units while 0.0018 per cent of them fire. It wrote the budget Appendix A's three targets must fit together — 43 000 000 units, the 10 µs tick in real time, and the input's density — and read that per-tick service at ADR-0044's density reaches about 25 600 units in real time on 64 workers, or runs 43 million about 1 700 times slower than real time with 275 TB/s of arena traffic (estimates at a developer machine's 25 ns a turn, not admissible). It opened F-49, because §1.1's premise and ADR-0023's "a unit that is at rest costs nothing" describe an executor whose cost follows activity and Appendix A states a plan the executor cannot run in real time, and it left two questions to this ADR: which integration model the engine keeps, and which of the three targets gives.

## Decision Drivers

- **The maintainers prefer per-tick service**, recorded before brief 042 ran. A neuron's membrane integrates continuously; what is sparse in nervous tissue is the spiking. Per-tick service keeps every membrane dynamic admissible, including one with no closed form between two events.
- **ADR-0097's budget.** For the membrane as written the tree has no way to advance a unit over $\Delta$ quiet ticks but to iterate it: every leak moves by at least one LSB, the soma is coupled to two compartments and the threshold decays by at least one LSB. So service on arrival is per-tick service under another schedule until a catch-up cheaper than $\Delta$ integrations exists. Under per-tick service the input's density cannot be the target that gives: at the rate the reference network fires, its own synapses alone would hold the fraction served at 76 per cent or more with no drive at all (ADR-0097's arithmetic).
- **Quality goal 1 and §8.3.** A run is `(image, seed, trace)`, and every arena after it is the same bit for bit on either architecture and at any pace. How fast the wall clock runs changes no result; only a body in the loop needs the wall clock kept (quality goal 4, T-4, T-5).
- **ADR-0010.** Every cost in the budget is a developer machine's. A figure this ADR writes is a Target until it is measured on the reference platform of §7.1 under the protocol of `docs/benchmarks/README.md`, and the tree has never had an admissible run.
- **Principle 1.** F-49 recorded the disagreement and left §1.1 and Appendix A as they stood. Once the decision is taken, the documents say what the tree does.
- **ADR-0078's lesson.** A mechanism waits for a measured need. Nothing in this decision needs code; the need for speed is ADR-0097's reading, in both modes.
- **The maintainers chose to take the engine's speed before the learning line resumes.** Every later round's runs and the weekly job get faster, and a lever that changes no rule has the tree's pins as its acceptance.

## Considered Options

For the model:

1. **Per-tick service, as built** ([ADR-0023](0023-executor.md)): every unit a message has reached is served on every tick until it is at rest.
2. **Service on arrival with a membrane that has a closed form** between events: a change to §8.8 and to [ADR-0018](0018-membrane-integration.md).
3. **Service on arrival with a faster catch-up** for the membrane as written, iterating it in fewer steps than ticks.

For the target that gives:

- **A.** The unit count: the engine always runs in real time, at what the budget allows.
- **B.** Real time: the engine always plans 43 000 000 units and runs slower than the wall clock.
- **C.** Two modes: real time where a body is attached, the unit count where none is.
- **D.** The input's density.
- **E.** The tick's resolution.

## Decision Outcome

**Option 1 and C.**

- **The model is per-tick service, as built.** No rule moves. Option 2 changes what a unit computes to make it cheaper to skip, against the maintainers' preference. Option 3 has no candidate: none is known that holds the three coupled compartments and the one-LSB floors bit for bit, and without one, service on arrival costs what per-tick service costs.
- **The real-time mode.** A body is attached, and the embodiment loop keeps its period: each epoch's hundred ticks complete within that millisecond of wall clock (quality goal 4, T-4, T-5). The unit count is not a free choice. It is what ADR-0097's budget allows at the input's density, the body's messages and the network's own. At ADR-0044's density that is a **Target of about $2.6 \times 10^4$ units on 64 workers** (the estimate above), about 3.6 × 10⁴ under ADR-0097's drive sixteen times sparser and 3.5 × 10⁵ under its drive 256 times sparser, where the network never fired. A network that fires at the reference network's rate stays below about 3.4 × 10⁴ whatever its drive, because its own messages alone would keep the fraction served at 76 per cent or more (ADR-0097's arithmetic). The real-time mode needs the embodiment loop, which is Specified; this ADR builds nothing for it.
- **The offline mode.** No body is attached, and the engine runs its ticks as fast as the machine serves them. The unit count is what one server's memory holds: Appendix A's plan, 43 000 000 units in about 15.9 GB of Tier 1, stays the plan **for this mode**. Its time is what the compute costs: at 43 million units and ADR-0044's density, about 1 700 times slower than real time, which is a simulated second in about half an hour (estimates, not admissible). Every run the tree has made is an offline run, and the learning line runs in this mode.
- **The mode is not a parameter.** Neither the image nor the engine carries it; it is what a deployment asks of the wall clock. A trace recorded in the real-time mode replays offline to the same arenas bit for bit (§8.3).
- **What the documents now say.** §1.1 keeps its sentence about the tissue and gains one on what is sparse there and what this engine's cost follows. Quality goal 4 holds in the real-time mode. Appendix A's table is not edited: the paragraph under it says the table is the offline mode's plan and states the real-time mode's Target. F-49 is resolved by this ADR, and §11.1's question on the integration model is decided. A new open question in §11.1 carries the engine's speed in either mode.
- **The levers on the engine's speed, named.** Each raises the real-time mode's reach and shortens every offline run, the weekly job's included. The first three change no rule, so each can be held to every pinned number and the determinism pin bit for bit; the last two move every pinned number.
  1. *The lookahead.* This is §11.1's question on synchronising once per minimum axonal delay (Chandy and Misra 1979; the communication interval of Morrison et al. 2005). The reference prior's shortest delay is 100 ticks (1 ms), so within a window of 100 ticks no unit needs a message sent inside that window. A worker could then advance each of its units the whole window from one load of its state, and synchronise once per window instead of three times per tick. It changes no tick and no delivery: every unit is still integrated on every tick, and every message still arrives at the tick it arrives at now; only the order in which the machine does the work moves. A path with no delay — an injected message, the modulation, the gain — must be known a window ahead, or the window shrinks to it; with a body attached, its input is taken at a window's start, as the embodiment's 1 ms period already takes it. That removes most of the arena's traffic, the bound ADR-0097 put at 275 TB/s for 43 million units. It needs units owned by workers. At the sizes the tree runs now, which fit the caches, it buys little; it is the lever for the sizes Appendix A plans.
  2. *A sweep without the gate.* When the fraction served is near one, a worker could serve every unit of a contiguous range without the deque and without the gate's schedule, begin and end. In ADR-0097's bench the gate took 9.79 ns and the integration 11.03 ns of a turn (a developer machine's, not admissible). A change to ADR-0023.
  3. *The layout of the integrated fields.* A sweep that reads only the fields `integrate` reads, a field to an array, moves less memory per unit than the 64-byte record and admits the vector units. [ADR-0001](0001-64-byte-pod-records.md) weighed a struct of arrays and kept it "available inside a subsystem where SIMD gathers need it", the 64-byte record staying the interchange unit and the image's layout; quality goal 2 says a unit's state is one cache line. Whether a working layout beside the record fits both is that layout's ADR's question.
  4. *The rest condition*, as ADR-0097 named it: a unit counted at rest inside the one-LSB tail. This shortens $D$ by 1 023 of 2 503 ticks; it leaves ADR-0044's density at 100 per cent and helps a sparse input. A rule change.
  5. *The tick's resolution* (option E). `TICK_NS` is a constant of `cortex-core`, and the loader refuses an image whose tick differs from it ([ADR-0033](0033-tick-duration-in-the-header.md)). A tick ten times longer runs ten times fewer ticks a second, but every time constant, every `*_ticks` field, the wheel's geometry and every pinned number move with it. A rule change across §8.4.
- **The order.** The three levers that change no rule come first, before the learning line resumes. Their order, their acceptance and the measure of their gain are the next ADR's. The two rule changes are not taken. The learning line then resumes from H-18's named next decision ([ADR-0096](0096-the-punished-pair-measured.md)): the reward-prediction error, a schedule of reversals, the operating regime or another size, all in the offline mode.

### Consequences

- Good: F-49 is resolved and the documents say what the tree does. Appendix A plans what one server holds, and quality goal 4 applies where a body needs it.
- Good: no rule, record, constant or pinned number moves, and the membrane stays free to take dynamics with no closed form.
- Good: results do not depend on the mode, so an offline run is as good a reading as a real-time one.
- Good: the levers taken first change no rule, so the tree's pins are their acceptance and no reading of any earlier round can move.
- Bad: the real-time mode's reach is small. It is about $10^4$ units at the reference network's rates, against Appendix A's $4.3 \times 10^7$, and it is an estimate until measured on the reference platform.
- Bad: at Appendix A's count the offline mode is slow. A simulated second takes about half an hour at ADR-0044's density, and a simulated day about four and a half years (estimates). The 43 million is a plan of memory, not of experiments; the experiments the tree runs are at $10^3$ to $10^4$ units, which a developer machine runs 1.3 and 5.2 times slower than real time on two workers at 1 024 and 4 096 units (ADR-0097).
- Neutral: T-8's target, "sustained random spiking at 2 % activity", counts delivered spikes and is left as it stands.

## Alternatives considered and why rejected

- **A, the unit count always gives**: it binds every run to the wall clock, though only a body needs it. Learning experiments larger than the real-time reach would be ruled out for no gain in any result.
- **B, real time always gives**: it abandons quality goal 4, which a body attached to the engine needs.
- **D, the input's density gives**: excluded by ADR-0097. Under per-tick service the network's own messages hold the fraction served near one at the rate it fires, whatever the drive.
- **E, the tick's resolution gives**: named above as a lever, not taken. It is a rule change that moves every pinned number, and the levers that change no rule come first.
- **Options 2 and 3 for the model**: above.

## Confirmation

No code, test, constant or pinned number changes. Whitepaper 4.50.0 carries the decision: the sentence added to §1.1, quality goal 4, the paragraph under Appendix A's table, F-49 resolved in §11, §11.1's question on the integration model decided and the engine's speed added as an open question, and this ADR's row in §9. `npm run spec` holds the links, the rows and the version.
