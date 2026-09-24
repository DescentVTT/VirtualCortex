---
status: accepted
date: 2026-09-25
depends-on: ADR-0023
decision-makers: VirtualCortex maintainers
---

# ADR-0097: The active set, measured — the executor counts its turns; one message of 0.125 keeps a unit awake 2 503 ticks at the gain 1.75, so under ADR-0044's drive the executor serves 99.99 per cent of the reference network's units on every tick at 1 024 and at 4 096 units while 0.002 per cent of them fire; the drive's floor $1 - e^{-rD}$, written first, is read within two per cent at drives sixteen and 256 times sparser, where the network never fires; the budget Appendix A's three targets must fit together is written for per-tick service and for service on arrival, per-tick service reaching about 26 000 units in real time on 64 workers at that drive, or 43 million about 1 700 times slower than real time; finding F-49; no integration model chosen

## Context and Problem Statement

[ADR-0023](0023-executor.md) serves a unit "every tick from the one it is woken until it is at rest: every potential zero, no refractory or plateau window, the threshold at or below its base", and says "a unit that is at rest costs nothing". So what a tick costs turns on **how many units are not at rest in it**, and the tree had never counted that. Whitepaper §1.1's premise is that nervous tissue "is roughly 1–2 % active at any instant". [Appendix A](../WHITEPAPER.md#appendix-a-capacity-model) plans 43 000 000 units on 64 workers, and the tick is 10 µs ([ADR-0033](0033-tick-duration-in-the-header.md), §8.4). The embodiment period of T-4 is 1 ms, a hundred ticks, so the engine keeps that period only if it runs ticks in real time. Brief 042 argued from the rules that the reference network's drive ([ADR-0044](0044-reference-network.md), a message per unit about every 128 ticks) against a leak that takes about two thousand ticks to clear one message makes the executor's active set close to every unit, every tick. It asked for a measurement, a prediction written first, the bench's costs and a budget, and it forbade choosing an integration model. The maintainers prefer to keep integrating every awake unit every tick, so that a membrane dynamic with no closed form between two events stays admissible; the brief therefore asked the budget to say as fully **what per-tick service can reach** as what service on arrival would change.

## Decision Drivers

- The brief's standing directives: nothing in the engine moves but a counter, which changes no behaviour; no pinned number and not the determinism pin; no float, oracles included.
- [ADR-0010](0010-measured-or-target.md): the counts are the engine's on a stated network and are Measured in that sense on this tree; every cost is a developer machine's and not admissible, and any budget built from one is an estimate.
- Principle 1: where §1.1 or Appendix A disagree with what the tree reads, the disagreement is a finding, and neither is edited to fit.
- The next decision, the integration model, needs both sides of the budget. This one must not pre-empt it.

## Considered Options

For the count:

1. **An engine counter** where `delivered` is counted, per worker in the turns phase, published between ticks and summed by an accessor.
2. **A test-side replica**: between ticks, count the units whose gate is scheduled, which are exactly the units the next tick serves.
3. **Both.** The engine counts, and the replica holds the engine's count on every tick of every run.

For the reading: the four configurations the brief names; or those four, each at 1 024 units beside a **control** made of the same units armed and wired to nothing under the same drive, which separates the drive's own active set from the network's.

## Decision Outcome

**Option 3, and the reading with its controls.** The engine counter is the count a production budget will read (§8.11), it costs one add per turn, and it will serve the next round whichever model that round builds. The replica makes the count independent of the code that keeps it: in every run, on every tick, the turns the engine counted equal the units whose gate the tick before left scheduled. The controls cost three short runs, and they answer the one question the prediction leaves open: whether the network's own messages move the drive's active set.

- **The count** (`runtime/cortex-runtime/src/executor.rs`). `Worker::turns` is incremented after each `turn` in `phase_turns` and stored in `Shared::turns[id]` beside `delivered` at the end of `phase_deliveries`. `Executor::turns()` sums the workers' totals. The counter reads nothing the engine reads and writes nothing the engine writes: the determinism pin, the image format (16) and every pinned number of every earlier round are unchanged, and the whole-domain tests of the dispatch below reproduce them.
- **The ticks to rest** (`tests/active.rs`). An oracle was written from `membrane.rs`'s rule and committed (`7ebd9ba`) before the engine ran. It covers the three compartments' leak and coupling below the threshold, with the basal leak alone beside it, and it scales the message by the gain as the executor does (F-47). The engine was held to it on one armed unit with no synapse:

  | Gain | Message on arrival | Turns to rest (oracle = engine) | Basal alone | Soma's peak |
  | ---: | ---: | ---: | ---: | ---: |
  | 1.0 | 0.125 (8 192) | **2 210** (22.1 ms) | 2 209 | 0.0586 (3 842) |
  | 1.75 | 0.21875 (14 336) | **2 503** (25.0 ms) | 2 502 | 0.1025 (6 715) |

  The engine serves the unit on every tick from the one that integrates the message through the one after which it rests, and not after. The brief's "about 1 900" took the one-LSB tail as starting at 512. `|v| >> 9` is one from 1 023 down, so the tail is 1 023 ticks. The count is the basal leak's plus the tick that integrates the message: the soma, which follows the basal compartment, is back at zero by the same tick.
- **The prediction, written first** (`7ebd9ba`, before any run of the network). A unit reached by messages at $r$ per tick, each keeping it awake $D$ ticks, is at rest only if no message arrived in the last $D$ ticks, so the fraction of ticks it is served is at least $1 - e^{-rD}$. That is the drive's floor, which the network's own messages move: its spikes raise the fraction, and inhibitory messages can shorten a unit's time to rest. The exponential is computed in integers, by the series of $e^{x}$ in `u128` at $2^{62}$ and its reciprocal, and is checked against $e^{-1}$, $e^{-2}$, $e^{-1/2}$ and $e^{-10}$ to six places. With $D$ = 2 503: (a) $rD$ = 19.55, **100 %**; (b) 1.222, **70.54 %**; (c) 0.0764, **7.354 %**; (d) 19.55, **100 %**. §1.1's 1–2 % would need $rD$ of 0.010 to 0.020: every message a unit receives, the drive's and the synapses', at 0.40 to 0.81 a second.
- **The active set** (`tests/active.rs`, four weekly `exhaustive` tests). The configuration is ADR-0044's prior at seed 22 under the instrument's configuration: the gain 1.75 held, the controller off, the modulation baseline zero (the calibration's, so no weight moves), two workers. Each run is read tick by tick: the turns, the messages delivered and those the drive sent, the spikes. It is pinned in rows: the first 512 ticks in eight rows of 64, the rest of a lead-in window of $2^{17}$ ticks, and four windows. The table gives the four windows, each 1.31 s:

  | Run | Units | Drive (messages a unit a second) | Prediction | Served per tick, four windows | Every unit served | Spikes (Hz a unit) | Synaptic messages a unit a second | Control, wired to nothing |
  | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
  | (a) | 1 024 | ADR-0044's, 781.25 | 100 % | **99.9914 to 99.9943 %** | 91.4 to 94.4 % of the ticks | 1.73 to 1.79 | 55.1 to 57.3 | 100.0000 %, every unit on every tick; fires 1.14 to 1.24 Hz |
  | (b) | 1 024 | sixteen times fewer, 48.83 | 70.54 % | **71.91 to 72.12 %** | none | 0 | 0 | the network's rows, bit for bit |
  | (c) | 1 024 | 256 times fewer, 3.052 | 7.354 % | **7.345 to 7.406 %** | none | 0 | 0 | the network's rows, bit for bit |
  | (d) | 4 096 | ADR-0044's, 781.25 | 100 % | **99.9935 to 99.9954 %** | 77.1 to 82.8 % of the ticks | 1.77 to 1.82 | 56.5 to 58.1 | not run |

  The readings are Measured on this tree. Under ADR-0044's drive the executor serves every unit but a few on every tick, at both sizes. The fewest in any tick after the lead-in is 1 021 of 1 024 and 4 092 of 4 096, while the units that fire in a tick are 1.76 Hz × 10 µs ≈ **0.0018 %** of them. Wired to nothing, the same units are served on every tick without exception. The network's own messages, 56 a unit a second beside the drive's 781, therefore lower the fraction slightly rather than raise it: an inhibitory message now and then returns a unit to rest. Under the sparser drives the network never fires, so its rows are its control's bit for bit, and the reading is the drive's alone. It sits 1.45 points (2.1 per cent) above the floor at (b), where messages overlap and a second message lengthens a unit's time awake, and within half a per cent of it at (c), one window 0.1 per cent below: the drive is a finite sample, and $1 - e^{-rD}$ is its expectation, not a bound on every window.
- **The costs, as the bench reports them** (`docs/benchmarks/results/2026-09-25-dancr-win11.md`: **not admissible**, a Windows developer laptop with no isolation and no fixed frequency; never written into §10.2):

  | Benchmark | Time | Read as |
  | :--- | ---: | :--- |
  | `gate/schedule_begin_end` | 9.79 ns | the turn's gate: schedule, begin, end |
  | `neuron/integrate` | 11.03 ns | the turn's integration |
  | `mailbox/push_drain_x16` | 75.2 ns ÷ 16 = 4.70 ns | one message pushed and drained |
  | `executor/push_to_turn` | 125.7 ns | one injected message through two ticks of the three phases on one worker |
  | `executor/idle_tick/{1,2,4}` | 44.5 ns, 543 ns, 18.9 µs; rerun at once 52.5 ns, 468 ns, 1.73 µs | a tick with every unit at rest: the barriers and the coordinator; four unpinned spinning threads on a desktop scheduler measure the scheduler |
  | `synapse/fan_out_x8`, `synapse/step_stdp` | 17.2 ns ÷ 8 = 2.15 ns; 129.6 ns a block | the fan-out a spike pays per synapse, and STDP per block |

  The runs themselves give the all-in figure the budget needs, their wall time over their turns (a developer machine's, printed and never pinned): **24 to 25 ns of one worker per turn** under (a), (b) and (d) in the run that reproduced the tables, up to 38 ns in an earlier run of the same tests; under (c) 32 to 41 ns, where the barriers are most of a tick. That is 12.7 µs a tick at 1 024 units and 52 µs at 4 096 on two workers, the reference network running 1.3 and 5.2 times slower than real time on this machine. The bench's components come to about 21 ns a turn (the gate and the integration) and 4.7 ns a message, which agrees with the runs.
- **The budget.** Write $N$ for the units, $W$ for the workers, $\tau$ for the tick, $f$ for the fraction of units served per tick, $m$ and $s$ for the messages and spikes a unit per tick, $k$ for the synapses a unit, $c_t$, $c_m$ and $c_s$ for one worker's cost of a turn, a message and a synapse's fan-out, $c_b(W)$ for the tick's barriers and coordinator, and $B$ for the memory's bandwidth. Real time is $T \le \tau$.

  *Per-tick service (as built):* $T = N\,(f\,c_t + m\,c_m + s\,k\,c_s)/W + c_b(W)$, and, once the arena outgrows the caches, $T \ge N f \cdot 64\ \text{B} / B$. The fraction is not a free parameter: $f \ge 1 - e^{-rD}$ with $D$ = 2 503 ticks for a message of 0.125 at 1.75. At the reference network's rates $m\,c_m$ and $s\,k\,c_s$ are a few per cent of $f\,c_t$, so $T \approx N f c_t / W + c_b$.

  *Service on arrival (not built, not chosen):* $T' = N\,(m\,(c_m + c_a) + s\,k\,c_s)/W + c_b'(W)$, and $T' \ge N m \cdot 64\ \text{B} / B$. Here $c_a$ is the cost of bringing a unit's state from its last event to the tick of this one. **For the membrane as written, $c_a$ grows with the gap.** Every leak moves by at least one LSB, the soma is coupled to two compartments, and the threshold decays by at least one LSB, so the rule is an integer map iterated per tick. Advancing a unit $\Delta$ ticks costs $\Delta$ integrations, which is per-tick service under another schedule. Service on arrival is cheaper only with a membrane whose state after $\Delta$ quiet ticks has a closed form: a change to §8.8 and to [ADR-0018](0018-membrane-integration.md), and the constraint the maintainers' preference keeps off. At equal cost per touch, each message costs per-tick service $(1 - e^{-rD})/r$ turns, at most $\min(D, 1/r)$, where it costs service on arrival one: as measured ($f/m$), 119 times more at (a), 1 474 at (b), 2 419 at (c).

  Appendix A's three targets are the units, 43 000 000; the tick in real time, 10 µs; and the input's density, ADR-0044's being the only one the tree runs. Estimated with $c_t$ = 25 ns (a developer machine's, not admissible), $W$ = 64 and the barriers taken as free, which favours both sides:

  | Input | $f$ (measured) | Per tick: largest $N$ at 10 µs | Per tick: slowdown at 43 M | Per tick: arena traffic at 43 M | On arrival at $c_m + c_a = c_t$: largest $N$ | On arrival: slowdown at 43 M | On arrival: traffic at 43 M |
  | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
  | (a) ADR-0044's, 837.5 messages a unit a second | 0.9999 | **25 600** | **1 680×** | 275 TB/s | 3.1 M | 14× | 2.3 TB/s |
  | (b) 48.8 a second | 0.720 | 35 600 | 1 210× | 198 TB/s | 52 M | 0.82× | 0.13 TB/s |
  | (c) 3.05 a second | 0.0738 | 347 000 | 124× | 20 TB/s | 840 M | 0.05× | 0.008 TB/s |
  | §1.1's 1–2 %, 0.40 to 0.81 a second (arithmetic, not run) | 0.01–0.02 | 1.3 M to 2.6 M | 17× to 34× | 2.8 to 5.5 TB/s | — | — | — |

  **What the arena alone implies.** Serving every unit every tick touches Appendix A's 2.75 GB arena once per tick: 275 TB/s at 10 µs, reading alone. That is at least 275 times what a DRAM system of the reference platform's class moves (§7.1: one socket of DDR5; its peak is well under $10^{12}$ bytes a second, a vendor's figure that this tree has not measured), whatever the compute costs. The arena is cache-resident only at the sizes the compute bound already allows (25 600 units are 1.6 MB).

  **What the relation says.** Under per-tick service, Appendix A's three targets cannot be met together, and which one gives is the next decision's question, stated here:
  - *Keep ADR-0044's density and the real-time tick:* about 2.6 × 10⁴ units on 64 workers.
  - *Keep 43 million units and that density:* each 10 µs tick takes about 17 ms of compute, 1 700 times real time, and at least 275 times real time on memory.
  - *Keep 43 million units and real time:* the fraction served must fall to 1/1 680, 0.06 per cent. That needs $rD \le 6 \times 10^{-4}$: every message a unit receives at no more than one every 42 seconds. The reference network's own synapses send 56 a second at the rate it fires, so no network that fires at that rate can meet it while one message keeps a unit awake 2 503 ticks. Even with no drive at all, those 56 messages alone would hold $f \ge 1 - e^{-1.41}$ = 76 % (arithmetic).

  So under per-tick service with this rest condition the density cannot be the target that gives: the unit count or the real-time tick must. Service on arrival changes the density at which the targets meet. With a catch-up that costs one integration, 43 million units run in real time up to about 60 messages a unit a second (on a network that does not fire; its fan-out adds to that), but not at ADR-0044's 837, where they run 14 times slower and need 2.3 TB/s. It needs a membrane with a closed form, which the present one is not.
- **The finding.** F-49, whitepaper §11. §1.1's premise and ADR-0023's "a unit that is at rest costs nothing" describe an executor whose cost follows activity. The executor as built serves 99.99 per cent of the reference network's units on every tick while 0.0018 per cent fire. Appendix A's unit count, the 10 µs tick in real time and ADR-0044's density cannot be met together by it. Neither §1.1 nor Appendix A is edited; the finding is open until the integration model's ADR disposes of it.
- **The gate** grows by one test, `the_turns_are_counted_one_message_keeps_a_unit_awake_as_the_oracle_says_and_the_prediction_is_as_written`. It runs the oracle, the exponential at its known values, the four predictions as committed, the counter at its edges on one armed unit at both gains (a tick with no unit awake serves none; one message serves the unit on every tick until it rests, and not after; the engine's count is the oracle's), and the first 512 ticks of (a), held to the first eight rows of its table. It takes 0.2 s in the debug profile.
- **The evidence.** This round's diff changes a file under `src/` (`executor.rs`), so by the first clause of [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the dispatch is `scope=both`, on this round's branch; its run, its jobs and the cost table regenerated from it (ADR-0092) are recorded here when it completes.

### Consequences

- Good: the tree now knows its active set, and knows it from the engine; the next round can read it under any model it builds.
- Good: the question the brief was written to settle is settled by counts, not arithmetic. Under ADR-0044's drive per-tick service is dense service, and the reason is $D$: one message keeps a unit awake 25 ms. It is not the network's recurrence, which slightly lowers the fraction.
- Good: the budget states per-tick service's reach as fully as service on arrival's, with the constraint that decides between them (a closed form of the membrane) named rather than assumed.
- Bad: every cost in the budget is a developer machine's. The barriers at 64 workers ($c_b(64)$, `executor/idle_tick` under the protocol, §11.1's lookahead question) and the memory bandwidth of the reference platform are unmeasured, and both only tighten the per-tick column.
- Bad: F-49 is open, and Appendix A still states a plan the executor cannot run in real time.
- Neutral: the four runs add about seventy seconds to the weekly job on a developer machine.

## Alternatives considered and why rejected

- **The replica alone** (option 2): no change under `src/` and no whole-tree sweep. But the count would then live only in a test, and the next round, whichever model it builds, would have to rebuild it.
- **The counter alone** (option 1): the brief's first form. Without the replica, "a counter changes no behaviour" would be the counter's own claim about itself; with the replica it is checked on every tick.
- **Choosing the model here**: excluded by the brief. The budget shows why it is a decision and not a reading: per-tick service keeps every membrane dynamic admissible and pays up to $\min(D, 1/r)$ turns per message, while service on arrival pays one but needs a closed form.
- **Relaxing the rest condition** (a unit counted at rest once every potential is inside the one-LSB tail, below 1 024 LSB, rather than at zero) would shorten $D$ by that tail, 1 023 of 2 503 ticks. $rD$ at ADR-0044's density would still be 11.6 and $f$ still 100 %. It is a rule change, not a count; it is named here for the next decision, not taken.
- **The instrument's configuration at 4 096 units rather than ADR-0051's**: the empowerment allowed either. ADR-0051's configuration at its lowest gain is the same prior, drive and gain with its own arena, store and sleep settings; the instrument's makes (a) and (d) differ in the size alone.

## Confirmation

`runtime/cortex-runtime/tests/active.rs`: the gate test above, and `the_active_set_under_the_reference_drive_at_1024_units_exhaustive`, `the_active_set_under_a_drive_sixteen_times_sparser_at_1024_units_exhaustive`, `the_active_set_under_a_drive_256_times_sparser_at_1024_units_exhaustive` and `the_active_set_under_the_reference_drive_at_4096_units_exhaustive`, each holding its thirteen rows, and its control's, to the pinned tables, and every tick's turns to the scheduled gates. The oracle and the predictions were committed in `7ebd9ba` before any run, and the tables in `94b87a5` after one run and a reproduction. The mutation gate on the diff is the pull request's `mutants` job; its reading is recorded with the dispatch's.
