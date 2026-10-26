# Where the work stands

Every round rewrites this page whole, and it holds no history ([ADR-0197](adr/0197-one-place-for-each-kind-of-fact.md)).
[ADR-0220](adr/0220-the-cost-of-a-test-run-alone.md) wrote it on 2026-10-26, after brief 082 read H-45 ([ADR-0219](adr/0219-a-delay-the-engine-reads-measured.md)).

## The engine

- **It runs.** The records with their layout assertions, the update rules, synaptic fan-out with three-factor STDP, the executor on a pool of workers that each own a range of the unit arena, the `.cortex` image (format 25), the criticality controller, sleep with the episodic ledger and its replay, and the symbolic layer are Implemented ([§1.6](WHITEPAPER.md#16-implementation-status-at-a-glance)).
- **Its default** for an image no derivation has settled is `Config::reads_its_targets()`: `Config::default()` with the modulation at rest 0.5, the gain 1.75, the read of 128 windows, both targets by it, and the probe, so the rate's rule runs at the step the engine makes in step = 1/16's place ([ADR-0196](adr/0196-the-engines-default.md), [ADR-0205](adr/0205-the-probe-in-the-engines-default.md)).
- **An engine born under it** writes H-42's candidate's image byte for byte on seeds 22, 23 and 24 ([ADR-0206](adr/0206-the-probe-in-the-engines-default-built.md)). `Config::default()` stays the bare engine every pin runs.
- **It can keep its addresses** ([ADR-0208](adr/0208-a-reward-past-the-trial-its-address-kept-built.md)): with `Config::kept_addresses` set, each trial's close keeps the address, and a reward that names its trial's age consolidates under it.
- **It can read a delay it is not told** ([ADR-0217](adr/0217-a-delay-the-engine-reads-built.md)): with `Config::reads_its_delay` set, a critic per kept age scores its errors, and the engine moves once from age zero to the age that leads by 1/32 at 64 rewards running. Unset, told, or on time with no move, every run is the run it was.

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

**The learning configuration** is H-25's ([ADR-0146](adr/0146-the-tag-the-address-already-is.md)): the excitatory synapses under the reward's gate with the signed gate set, the inhibitory ones under a baseline of their own, a critic of the engine's own over the shortest synaptic delay after a reward, and the reward's address drawn by the engine from that window's counts onto the readout its selection chose.

Per trial the host gives the reward's sign; under a delay it names the trial's age, or, with the delay read, the engine reads it. The readout sets are the body's interface.

## Asked, read as no, and kept unset

- The rule held by the network is paused, with the facilitating class and the slow current ([ADR-0126](adr/0126-the-gate-raised-measured.md)); the address's neural target side is closed, with the hold and the released delivery (ADR-0146).
- The whole punishment is a parameter of the image no configuration sets (ADR-0155, ADR-0156); the exploration an option of the task no run sets (ADR-0163, ADR-0164).
- The controller's step under its estimate is zero in every run that learns (ADR-0077, ADR-0177).
- The rate's rule, the read of its target, the read's flag and the probe are each set in the default; no network but H-42's three has run under it.
- The kept addresses and the delay read are parameters of the configuration that no default sets; H-43 and H-45 ran them at `kept_addresses` 2.
- The speed line is closed, the sweep kept and the lanes reverted ([ADR-0105](adr/0105-the-speed-line-closed.md)).

## What does not exist

- Every subsystem's real dynamics beyond these rules.
- The lookahead ADR-0099 ordered, and a lever on the rest of the turn.
- The real-time mode's reach, measured on the reference platform.
- The `mmap` path, the shared-memory mappings, core pinning, the tool broker and standardising apart.
- A delay that changes during a run, a delay past two trials, and a mapping kept through a late move: the first is lost at age zero.
- The per-unit gain of §8.8, and a reserved byte in the modulator section.

## The next decision

H-45 read yes at step 3 of its stopping rule: the engine reads a delay of one or two trials it is not told.
1. Step 3's next decision: the re-read of the targets by evidence, at M12, with a step by its error's whiteness explored first.
2. What the weekly job holds: 219 tests at 113 minutes a shard, 47 % of its 240-minute bound since ADR-0220, and about 12 minutes more with each round of eighteen arms.
3. The schedule's sweep of 2026-10-26, recorded by the first ADR that merges after it ends. Brief 065 stays paused.
