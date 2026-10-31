# Where the work stands

Every round rewrites this page whole, and it holds no history ([ADR-0197](adr/0197-one-place-for-each-kind-of-fact.md)).
[ADR-0230](adr/0230-a-zero-sum-at-the-stimulus-measured.md) wrote it on 2026-11-01, after brief 085.

## The engine

- **It runs.** The records with their layout assertions, the update rules, synaptic fan-out with three-factor STDP, the executor on a pool of workers that each own a range of the unit arena, the `.menteion` image (format 26, its magic `MENTEION`), the criticality controller, sleep with the episodic ledger and its replay, and the symbolic layer are Implemented ([§1.6](WHITEPAPER.md#16-implementation-status-at-a-glance)).
- **It serves a unit on arrival** ([ADR-0225](adr/0225-a-unit-served-on-arrival-built.md)): a unit that cannot fire with no input sleeps off the schedule and is caught up bit for bit at its next message or when the arena is read. Per-tick service stays the model and the tests' reference, `Executor::per_tick`. The weekly job's tests took 0.417 of their seconds before on x86-64, and the 16-block workload 0.74 on AArch64.
- **Its default** for an image no derivation has settled is `Config::reads_its_targets()`: `Config::default()` with the modulation at rest 0.5, the gain 1.75, the read of 128 windows, both targets by it, and the probe, so the rate's rule runs at the step the engine makes in step = 1/16's place ([ADR-0196](adr/0196-the-engines-default.md), [ADR-0205](adr/0205-the-probe-in-the-engines-default.md)). An engine born under it writes H-42's candidate's image byte for byte on seeds 22, 23 and 24 ([ADR-0206](adr/0206-the-probe-in-the-engines-default-built.md)). `Config::default()` stays the bare engine every pin runs.
- **It can keep its addresses and read a delay** ([ADR-0208](adr/0208-a-reward-past-the-trial-its-address-kept-built.md), [ADR-0217](adr/0217-a-delay-the-engine-reads-built.md)): with `Config::kept_addresses` set, each trial's close keeps the address, and a reward that names its trial's age consolidates under it. With `Config::reads_its_delay` set, a critic per kept age scores its errors, and the engine moves once from age zero to the age that leads by 1/32 at 64 rewards running. Unset, told, or on time with no move, every run is the run it was.
- **It can hold a stimulus's couplings** ([ADR-0228](adr/0228-a-zero-sum-at-the-stimulus-built.md)): `Executor::set_zero_sum` scales each group back to a value the host gives, each synapse's fraction carried, and `Config::depression_exponent` sets the excitatory depression's exponent. Unset, every run is the run it was, and neither is in the image.

## What it learns

On the reference network at 1 024 units, ADR-0044's prior from seed 22, unless a row says otherwise.
Every hypothesis, with its report, is in [docs/research/](research/README.md).

| Asked | Read | Hypotheses |
| :--- | :--- | :--- |
| The two-alternative task: learned, revised through three reversals with the couplings bounded, under a reward right 7 times in 8 | yes, under H-25's configuration | H-13 to H-28 |
| A choice among three answers | no: the first mapping is learned, not every revision, and the line stops | H-29 to H-32 |
| The task on seeds 23 and 24, and at 2 048 units | yes | H-33, H-34 |
| A reward right 3 times in 4, with the gain held at 1.75 on the earlier networks | no: 21 mappings of 24 learned over three networks | H-35, H-36 |
| The criticality controller's step | no: every step takes the network to 24.4 Hz a unit | H-37 |
| The gain moved by a rate the controller holds | yes, at step = 1/8 on seed 22 and at 1/16 on three networks | H-38, H-39 |
| A target the engine reads, then the inhibitory target by the same read | yes, on three networks | H-40, H-41 |
| A step the engine makes from its own loop | yes, on three networks: S = 15.0 to 15.2, step = 1/61 to 1/60 | [H-42](research/H-42.md) |
| A reward one or two trials late, its address kept | yes, on three networks under the default: 118 to 128 of 128 | [H-43](research/H-43.md) |
| A reward right 3 times in 4, on the default | yes, on three networks: 90 to 127 of 128, and with the step zero too | [H-44](research/H-44.md) |
| A delay of one or two trials the engine is not told | yes, on three networks: one move to the true age at trials 128 to 4 269, every later mapping 118 to 127 of 128; on time no move | [H-45](research/H-45.md) |
| The couplings over sixteen schedules, under a zero sum at the stimulus and a quarter's exponent | yes, on three networks: highest coupling 1.157 to 1.215, H-25's four clauses in every schedule, the default past 1.30 by the third and at 1.98 to 2.11 by the last | [H-46](research/H-46.md) |

**The learning configuration** is H-25's ([ADR-0146](adr/0146-the-tag-the-address-already-is.md)): the excitatory synapses under the reward's gate with the signed gate set, the inhibitory ones under a baseline of their own, a critic of the engine's own over the shortest synaptic delay after a reward, and the reward's address drawn by the engine from that window's counts onto the readout its selection chose.

Per trial the host gives the reward's sign; under a delay it names the trial's age, or, with the delay read, the engine reads it. The readout sets are the body's interface, and so, under the zero sum, are its groups.

## Asked, read as no, and kept unset

- The rule held by the network is paused, with the facilitating class and the slow current ([ADR-0126](adr/0126-the-gate-raised-measured.md)); the address's neural target side is closed, with the hold and the released delivery (ADR-0146).
- The whole punishment is a parameter of the image no configuration sets (ADR-0155, ADR-0156); the exploration an option of the task no run sets (ADR-0163, ADR-0164).
- The controller's step under its estimate is zero in every run that learns (ADR-0077, ADR-0177).
- The rate's rule, the read of its target, the read's flag and the probe are each set in the default; no network but H-42's three has run under it.
- The kept addresses, the delay read, the zero sum and the depression's exponent are in no default; H-43 and H-45 ran the first two at `kept_addresses` 2, and H-46 the last two.
- Of the speed line ([ADR-0105](adr/0105-the-speed-line-closed.md)), the sweep and service on arrival are kept, and the lanes reverted.

## What does not exist

- Every subsystem's real dynamics beyond these rules.
- The lookahead ADR-0099 ordered, a lever on the rest of the turn, and a catch-up in fewer steps than ticks.
- The `mmap` path, the shared-memory mappings, core pinning, the tool broker, standardising apart, and the real-time mode's reach, measured on the reference platform.
- A delay that changes during a run, a delay past two trials, and a mapping kept through a late move: the first is lost at age zero.
- The per-unit gain of §8.8, and a reserved byte in the modulator section.
- A run past sixteen schedules, and couplings whose shape has settled: under the zero sum their bottom eighth still rose about 0.02 a schedule at the sixteenth.

## The next decision

Brief 085 built the two rules, and H-46 read yes at step 3 with all four readings held ([ADR-0230](adr/0230-a-zero-sum-at-the-stimulus-measured.md)).
1. Whether the zero sum and a quarter's exponent enter the engine's default, whether M12 closes, and then M11.
2. A read of the arena on every tick catches every asleep unit up a tick, so a test that reads it so takes longer ([ADR-0225](adr/0225-a-unit-served-on-arrival-built.md)): what such a reading should pay is not decided.
