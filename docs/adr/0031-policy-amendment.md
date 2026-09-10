---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0024
---

# ADR-0031: Self-amendment — a policy amendment on trial in two forks of the image, committed only when the engine behaved exactly as before and cost less; the engine never amends its own code; the amendment register; image format 10

## Context and Problem Statement

A directive dated 2026-09-10 asked for "verifiable self-code evolution and safe digital self-improvement": an architecture in which the engine "autonomously propose[s], verif[ies], and incorporate[s] modifications to its own code or subsystem rules in a verifiable closed loop, while preserving the fundamental physical and mathematical invariants of the system". It left the mechanism open and named four dimensions to decide: isolation (the worker's seccomp filter forbids `execve`, `fork` and `socket`; compilation and analysis would have to happen out of process), gates (the veto gate of `cortex-ethics`, the 64-byte layout, zero allocation on the hot path, bit-identical determinism), staging and persistence (how a validated change becomes runtime reality), and the interface (whether `cortex-tools` and its `ToolInvocationFrame` carry it).

The question underneath is what an engine made of integer rules over fixed 64-byte records can change about itself *such that the change is verified by a test the engine itself can run*, and what must stay with the maintainers and the repository's gates. A rule in this workspace is a Rust function; a run is defined by `(image, seed, input trace)` (§8.3); the tick loop allocates nothing and makes no system call (TC-5); a state crate declares no dependency (TC-2) and nothing sub-1.0 or nightly reaches the hot path (§2.1); the engine is not a general-purpose framework (§1.5). Within those, "modify its own code" and "modify its own rules' parameters" are different questions with different answers.

## Decision Drivers

- Quality goal 1 and §8.3: a run stays a function of the image, the seed and the trace. Whatever the engine changes about itself must be *in the image* or *in the trace*, or the pin of ADR-0030 means nothing.
- §8.9: the veto gate stands in front of every action the engine proposes, and its own parameters cannot be what the engine proposes to change.
- Principle 5: a gate the record keeps and the loader checks beats a protocol a reviewer remembers.
- TC-5, §7.3 and §8.10: no compiler, interpreter or code loader inside the engine process; no `execve`.
- The admission test of [ADR-0016](0016-thirty-two-crate-architecture.md): a new quantity is a field or a second record in the crate that owns it, never a new crate.
- The code lane's verifier already exists: [ADR-0029](0029-structural-enforcement.md) and [ADR-0030](0030-verification-governance.md) are the gates a change to a rule passes, and they run in the repository, not in the engine.

## Considered Options

1. **Run-time modification of the engine's own code**: an interpreter or a just-in-time compiler in the runtime, or `rustc` behind the broker with a hot reload of the result.
2. **Proposal only**: the engine emits proposals through the broker; the repository's gates and the maintainers verify and merge everything; nothing changes inside a running engine.
3. **Two lanes.** (a) A *parameter* of the engine's own policy, from a registry with bounds, is amended inside the engine through four gates kept on a 64-byte record: the bounds, the veto gate by id, a trial in two forks of the image that must behave identically, and a cost that must fall; committed between ticks, persisted in the image, replayed by the loader. (b) A *rule* is never amended by the engine: what it cannot commit leaves as a frame to an amendment register outside the engine, where the repository's gates are the verifier. The veto gate's parameters are in neither lane.

## Decision Outcome

Option 3.

### What the engine may amend: the registry

`cortex-executive` carries `REGISTRY`, the parameters the engine may amend by itself, each with an owner and a closed interval: today `PARAM_SWEEP_QUIET_TICKS` and `PARAM_SWEEP_BUDGET`, the clock sweep's quiet bound and budget (axiom A5, [ADR-0024](0024-cortex-image-and-clock-sweep.md)), both owned by the runtime's sweep and bounded to `[0, i32::MAX]`. A parameter a state crate's rule takes joins the registry when the runtime composes that rule, by an ADR that states the bounds; until then the rule's parameter is the caller's argument and is not amendable. `OWNER_VETO_GATE` is defined and never used: the test `the_registry_names_the_sweep_s_two_parameters_and_never_the_veto_gate` holds that no entry carries it, so the gate's harm threshold, forbidden mask and required level are structurally outside the engine's reach.

### The record: `PolicyAmendment`

The second record of `cortex-executive`, admitted under ADR-0016's test: the gap is a proposed change to the engine's own policy with its trial and its verdict; no Responsibility row owns it (`cortex-homeostasis` regulates set points and holds no proposal; `cortex-ethics` evaluates a proposal and does not hold what is proposed; `cortex-imagination` is goal-free rehearsal; `cortex-executive` is "a lookahead evaluated in an internal sandbox that never drives the motor channel", which is what a trial is); the mechanism is a §8.8 row; every width holds its format; §1.5 and §8.10 are moved explicitly below. Sixty-four bytes: the two behaviour hashes, the id (arena index + 1; zero is an empty slot; the veto gate's `proposal_action_id`), the proposed and committed ticks, the current and proposed values, the two costs, the trial's ticks, the parameter, the least gain that counts, the status, the reason, the objective, the gates passed, eight reserved bytes.

### The four gates, in order

1. **Bounds** (`propose`): the parameter is registered, the current and the proposed value are within its bounds, they differ, the objective is known; the first failure names the reason (`REJECT_UNKNOWN_PARAMETER`, `REJECT_OUT_OF_BOUNDS`, `REJECT_NO_CHANGE`, `REJECT_UNKNOWN_OBJECTIVE`). Every proposal leaves a record, a rejected one as the record of its rejection.
2. **Veto** (`admit`; `Executor::admit`): the veto gate of `cortex-ethics` is evaluated on this amendment (the runtime refuses a gate whose `proposal_action_id` is another's, `AmendError::WrongProposal`), and only a permitting verdict admits it; a gate not evaluated is closed ([ADR-0028](0028-edge-behaviour-audit.md)).
3. **Behaviour** (`record_trial`): the candidate fork's behaviour hash equals the baseline's, or the amendment is rejected as `REJECT_BEHAVIOUR_CHANGED`. This is the rule that makes the loop verifiable: *the engine may change what it costs, never what it does*. Equality of two hashes is decidable by the engine; "better behaviour" is not a test, and a change to behaviour is the maintainers' lane.
4. **Gain** (`record_trial`): the objective's cost fell by at least `min_gain`, and by at least one, or `REJECT_NO_GAIN`. Two objectives exist, both counts the runtime already keeps: `OBJECTIVE_RESIDENT_UNITS` (units still resident when the trial ends; memory) and `OBJECTIVE_REHYDRATIONS` (evictions that were wrong; churn). A zero-tick trial is `REJECT_EMPTY_TRIAL`.

A zero id, an empty slot's, is rejected before the bounds gate (`REJECT_NO_ID`). `may_commit` is true only for a trialled record whose `gates` byte is `GATES_ALL`; `commit` stamps the tick. `is_well_formed` holds that a record's bytes are ones the state machine could have produced: the gates exactly those its status and reason imply, the values within bounds where a gate says they were, the hashes equal and the gain sufficient where a verdict says they were, the reserved bytes zero. The loader refuses anything else (`ImageError::MalformedAmendment`), so an image cannot smuggle a committed amendment past a gate. Every rule is lattice-tested (`mod prop`: every pair of the `i32` lattice against every parameter and objective; a seeded walk of twenty thousand histories through the whole machine, each record well-formed and byte-round-tripped), and the mutation gate on the diff passed in CI (its first run caught seven mutants in the trial that no test constrained; the second commit's tests catch them).

### The trial: two forks of the image

`cortex_runtime::trial::run(exec, index, trial)` takes the live executor's image, written at a quiescent point after its last commit, decodes it twice (`Image::decode`, the only copy of the state the runtime knows how to make, which validates the bytes on the way), refuses an image whose policy is not the value the amendment started from (`ImageError::StaleBaseline`: an image written before a later commit would measure the gain against a baseline that is not the live one), attaches a write-ahead log to each fork, puts the proposed value into the candidate's policy, runs both for the same ticks with the same injections and the same sweep cadence, requires each to end at a quiescent point (`NotQuiescent` otherwise: a pending mailbox names the pool node the delivering worker chose, which work-stealing decides, so a non-quiescent fork's bytes are not a function of behaviour) and to have traced every spike (`NoTrace` when the trace capacity is zero or a spike was dropped), and hashes each (CRC-64/XZ over every unit's 64 image bytes, an evicted one's read from its log, every synapse block, the spike train in `(tick, unit)` order and the delivered count), counting resident units, evictions and re-hydrations. The verdict is written into the live arena by the trial and by nothing else: `Executor::record_trial` is crate-private, so a caller cannot record a trial that did not run. It runs outside the tick loop, where the writer runs: its allocations and its two log files are outside TC-5's scope, like `Image::write`'s. For the sweep's parameters the behaviour gate is a re-run of ADR-0024's transparency ("evict, spike, re-hydrate bit for bit") on the live image: the runtime tests show a shorter quiet bound freeing memory with the hashes equal, and a bound that evicts active tissue costing re-hydrations and being rejected for no gain.

### The commit and the live policy

`Executor::commit` runs between ticks (the `&mut` excludes a concurrent tick); it refuses an amendment that is not committable; refuses as `AmendError::Superseded` one for whose parameter a later proposal was committed first, so that commits to one parameter keep the arena's order and the loader's replay in that order reproduces the live policy; and refuses as `AmendError::Stale` one whose starting value is no longer the live value, since another commit came between and the trial did not test this change on top of it. The policy (`Policy { sweep_quiet_ticks, sweep_budget }`, default 10 000 ticks and 1 024 units) is read by `sweep_by_policy` and changed by nothing else; `Config::amendments` sizes the arena, and zero leaves the engine unable to propose.

### Persistence: image format 10

The arena is a section, `SECTION_AMENDMENT` (kind 41, the record's Appendix A row), written when it is not empty. The loader decodes every record, refuses one that is not well-formed, out of order (the id is not its index + 1) or, for a committed one, whose starting value is not the policy's at that point in the replay; it derives the policy by replaying the committed records in order. The policy is therefore in the image and nowhere else: `(image, seed, trace)` still defines a run, the arena is the log of how the policy came to be, and `Image::encode` of a loaded image is byte-identical to what was loaded. `CortexFileHeader::FORMAT_VERSION` is 10 (a version-9 loader would refuse the section; no record moved).

### The code lane: the amendment register

`cortex-tools` names `TOOL_CATEGORY_AMENDMENT_REGISTER` (0x0006) with `ACTION_FILE_PROPOSAL` (an amendment the engine may not commit itself, a change to a rule rather than to a registered parameter, for the repository's gates and its maintainers) and `ACTION_RECORD_COMMIT` (a committed parameter amendment into the operator's register, an audit trail the engine cannot rewrite); `param_hash` names the amendment record. The broker and its register are Specified, like the prover and the document engine; whether the register is a file, an issue or a pull request is the broker's configuration and names no product. The verifier for a proposed rule is the repository: the lints of ADR-0029, the mutation gate of ADR-0030, spec-guard, review, and a merge by a maintainer. The engine never applies a rule change, and there is no path by which it could: no compiler, no interpreter, no loader of code in the process (§7.3, §8.10).

### Boundaries moved

- §1.5: the engine may change its own policy, within the registry, by itself. It may not change its code; a proposed rule leaves through the broker.
- §8.10: a new bullet states the two lanes, the behaviour gate, the loader's refusal and the registry's exclusion of the veto gate.

### What needs no per-amendment gate, and why

The directive's other gates hold by construction and are not re-checked per amendment: the layout (no record changes; the amendment is a value in a preallocated slot), allocation on the hot path (the arena is sized at `Executor::new`; the trial is not the hot path), determinism across workers (the executor's property, checked by the differential test and the pin of ADR-0030 on every push, not by each trial), and the dependency discipline (no state crate gained a dependency; the runtime gained `cortex-executive` and `cortex-ethics` by path, as ADR-0023 intends).

### What is not claimed

The record is a mechanism: a proposal, four gates, a verdict. The document does not call it self-improvement, learning or evolution, and makes no claim that the policy the engine converges to is good: the objective is two counts, the gain is a difference, and whether a lower resident count or fewer re-hydrations is what an operator wants is the operator's proposal. Nothing here changes what the engine does; the behaviour gate is the proof of that, per amendment, and it is a test, not a proof about every possible amendment.

### Consequences

- Good: the engine has one thing it can change about itself, and every such change is recorded, gated four times, tested in two forks of the live image (the trial refusing a stale image, an untraced spike train or a fork that did not run to quiescence), committed only when the fork behaved identically and in the arena's order, persisted, replayed on load and refused by the loader when forged. The veto gate is consulted by id and cannot be amended. A run is still `(image, seed, trace)`.
- Good: no new crate, no new dependency in a state crate, no new `unsafe`; the trial reuses the writer, the loader and the log.
- Bad: two forks of the image cost two decodes and two runs; a trial of a large image is minutes, which is why it runs where the writer runs and not in the loop.
- Bad: the registry holds two parameters, both the sweep's; every other rule's parameters wait for the runtime to compose their crates. The mechanism is general; its coverage is not.
- Bad: format version 10 makes version-9 images unreadable, as every bump does.

## Alternatives considered and why rejected

- Option 1: an interpreter or JIT is a program in the image and a general-purpose framework in the tick loop (§1.5); `rustc` behind the broker with a hot reload changes the code a run depends on, so `(image, seed, trace)` no longer defines it (§8.3) and the pin of ADR-0030 becomes meaningless; and the engine cannot verify a compiler's output with any test it can run. Every candidate dependency (a bytecode VM, a WebAssembly runtime, a JIT) fails §2.1 for the hot path and TC-2 for a state crate.
- Option 2 alone: it gives up the one lane the engine can verify by itself, and it is the status quo under another name.
- A new crate (`cortex-evolution`, `cortex-metaplasticity`): the quantity has an owner, `cortex-executive`, and ADR-0016's test says a quantity with an owner is a field or a second record there.
- A behaviour-changing objective ("more spikes", "faster convergence"): which behaviour is better is not decidable by a test the engine runs, and a loop that accepts a behaviour change on a score it chose is the loop the directive asks to be made safe. Equality of the hashes is the boundary.
- The policy in the header: a second source of truth beside the amendments; the arena is the log and the loader replays it.
- A commit inside the tick: a parameter read inside a tick would need the quiescent point; the sweep reads its parameters between ticks, and no registry entry is read inside one.
- An amendment ring for the broker: the arena is the engine's record and the register is the broker's; a frame names a record by hash, as the prover names a conjecture.

## Confirmation

- `crates/cortex-executive/src/lib.rs`: `PolicyAmendment`, the constants, `REGISTRY`, `spec_of`, `bounds_reason`; sixteen unit tests (one of them the layout's) and two property tests.
- `crates/cortex-tools/src/lib.rs`: the third category and its two actions in `is_known_action` and its test.
- `crates/cortex-connectome/src/lib.rs`: `SECTION_AMENDMENT` (41), `FORMAT_VERSION` (10) and the test that pins it.
- `runtime/cortex-runtime/src/{executor,image,trial}.rs`: `Policy`, `AmendError`, `Executor::{policy, amendments, amendment_room, propose, admit, commit, sweep_by_policy}`, the section in `Image::{encode, decode}`, `trial::run` (the one writer of a verdict); `runtime/cortex-runtime/tests/amendment.rs`: the closed loop end to end, the rejection for no gain, the sweep cadence, the trial's refusals (a stale image, no trace, a dropped spike, a fork mid-flight, a refused injection), the superseded and the stale commit, persistence and the loader's refusals.
- Whitepaper 4.4.0: §1.2 FR-10, §1.5, §1.6, §3.2, §5.2.2, §5.2.10, §5.2.21, §5.2.28, §6.16 (R-16), §8.6, §8.7, §8.8, §8.10, §8.18, §9, §11.1, §12, Appendix A row 41, Appendix C M9; the directives that hold the record, the gates and the registry's exclusion.
- The mutation gate on the diff (ADR-0030) passed with no survivor outside `.cargo/mutants.toml`.
