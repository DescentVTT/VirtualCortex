---
status: accepted
date: 2026-10-18
decision-makers: VirtualCortex maintainers
---

# ADR-0187: The upgrade suite planned — an outside suite of twenty-nine proposals, UPG-01 to UPG-29 in seven phases, read against the tree: one already done, seven taken into the plan in an order and each to enter by a decision of its own and a measured round, four after the tool broker, seventeen deferred with what would bring each, and what no round adopts named; Latest ≠ Newest kept, its scope stated: it governs what the tree depends on, not what the engine does; Appendix C gains six planned milestones; nothing is built

## Context and Problem Statement

On 2026-10-17 the maintainers asked that the proposals of a suite written outside this tree be weighed and the fitting ones added to the plan, but not built. They also asked whether Latest ≠ Newest should go, since it might stand in the way of the upgrade.

**The suite** is eight documents with a roadmap. They propose twenty-nine upgrades, UPG-01 to UPG-29, as twenty-nine work packages in seven phases. Two things about it matter before any proposal is weighed:
- **It was written against an older tree:** [ADR-0101](0101-the-sweep-measured-again.md), the image at format 16, brief 044, whitepaper 4.53.0. This tree is at ADR-0186, format 22, brief 073 and 4.120.0.
- **It rates itself.** Its first stage is put at 90 to 95 per cent likely as engineering and 72 to 82 per cent as cognition, its fifth and sixth at 10 to 25 per cent, and literal superintelligence below 5 per cent. None of those figures was measured. This ADR reads the proposals and not the figures.

**What reading it against the tree finds.**
- **UPG-11, the ownership sweep and F-49, is done.** [ADR-0102](0102-the-sweep-timed-with-no-census.md) kept the sweep, [ADR-0104](0104-the-membrane-in-lanes.md) did not keep the lanes, [ADR-0105](0105-the-speed-line-closed.md) closed the speed line, and F-49 is resolved by [ADR-0097](0097-the-active-set-measured.md) and [ADR-0098](0098-the-integration-model.md).
- **Some of its layouts no longer hold.**
  - UPG-02 asks for `DendriticSuperNeuron`'s `[20..24)` as a BCM threshold. Those bytes are `v_slow`, the slow current of [ADR-0123](0123-the-slow-current-built.md), a different quantity.
  - The modulator section's `[32..36)`, which UPG-02 also asks for, is not among its reserved bytes. Those are `[26..28)`, `[39]`, `[54..56)` and `[60..64)` (`MODULATOR_RESERVED`).
- **One of its constants collides.** It numbers an `OBJECTIVE_EPISTEMIC_GAIN` as 2, which is `OBJECTIVE_REHYDRATIONS` (`crates/cortex-executive/src/lib.rs`).
- **One layout does fit.** UPG-01's flag `0x10` is free, since `flags` uses bits 0 to 3, and the apical fields it reads are there.
- **Two sections exist only as numbers.** `SECTION_LAMINAR` (38) and `SECTION_ROUTING` (39) have no record; the loader refuses both.

**Latest ≠ Newest.** Principle 4 admits a technology to the hot path, and the same test admits a dependency. It does not admit or refuse a mechanism. What the suite proposes that the doctrine would refuse is refused by another rule as well:
- RDMA and NVMe paging on the hot path, by the scope of §1.5 and by no syscalls on the hot path;
- performance figures not reproduced, by [ADR-0010](0010-measured-or-target.md);
- self-amendment beyond the registry, by [ADR-0031](0031-policy-amendment.md).

What limits how fast this tree moves is the measured round, one change a round. The sense that the doctrine stands in the way comes from the briefs. Each restates it among its standing directives and lets that sentence carry the round's scope, *"nothing else is adopted"*, which is a different rule.

## Decision Drivers

- **Literature is a prior, not a verdict**: a proposal enters as a candidate for a measured round, never as a result.
- **One change a round**, as every round since the measurement doctrine.
- **The repository wins over the document** (principle 1): what the suite says the tree holds is checked against the tree.
- **A rule says where it is enforced** (principle 5), and a principle says what it governs.

## Considered Options

1. The suite: (a) its proposals sorted into done, planned in an order, deferred with a condition, and not adopted; (b) its roadmap taken as the plan; (c) nothing taken.
2. Latest ≠ Newest: (a) kept, its scope stated; (b) removed; (c) kept as it is.

## Decision Outcome

**Options 1(a) and 2(a). Nothing is built by this ADR.**

### Done

- **UPG-11**, by ADR-0097, ADR-0098, ADR-0102, ADR-0104 and ADR-0105.

### Planned, in this order

Each item enters by a decision ADR of its own that writes a hypothesis before any run, and is then measured by one round. The UPG named is a candidate mechanism for that decision, not the decision.

| Order | Milestone | What it asks | Candidate from the suite | What comes first |
| :---: | :--- | :--- | :--- | :--- |
| 1 | M10 A reward past the trial | Does the configuration learn when the reward arrives trials after the choice? | UPG-03, options and a critic over several timescales | the default [ADR-0183](0183-a-sixteenth-on-three-networks-measured.md) named |
| 2 | M11 Learning that sleep keeps | Does a learned mapping survive a sleep with replay better than without one? | UPG-04, chained episodes compiled in sleep | M10 or its no |
| 3 | M12 Couplings over long runs | Do the couplings and the inhibitory sum stay bounded over many schedules? | UPG-02, a sliding threshold | a layout decision: the bytes it asked for are taken |
| 4 | M13 Lookahead | Does a choice made by an imagined rollout beat the reflexive one on a task that needs it? | UPG-08, a counterfactual rollout; [ADR-0099](0099-the-engines-speed.md) ordered the lookahead | M11 |
| 5 | M14 Synthesis in sleep | Does a REM pass propose a term that shortens the ledger's description? | UPG-09, combinators under minimum description length | M11 |
| 6 | M8, its remainder | The tool ring and broker | — | none |
| 7 | M15 Language that says what it does not know | Does a frame's confidence decide whether an answer is given? | UPG-05 and UPG-06 | M8's lexicon and broker |

### After the tool broker

- **UPG-07, UPG-10, UPG-15 and UPG-16** go through the broker, and wait for it.

### Deferred, with what would bring each

- **UPG-01, apical credit:** a task with a hidden layer. The reference task is learned by a three-factor rule with the engine's critic, and has no layer to assign credit through.
- **UPG-12, the RDMA barrier:** a decision that the engine runs across nodes. §1.5 scopes it to one node, with scale-out a Specified concern of the data plane.
- **UPG-13, the image's meta-evolution:** only as entries in `REGISTRY`, each its own ADR; the veto gate's parameters never ([ADR-0031](0031-policy-amendment.md)).
- **UPG-14, column paging:** the `mmap` path, with no syscall on the hot path.
- **UPG-17, swarm distillation:** more than one instance, which a single-node engine does not have.
- **UPG-18 to UPG-29, the suite's sixth and seventh phases**, which it marks optional: each waits for the subsystem it extends to have dynamics beyond its placeholder rule (M5) and a task that would measure it. UPG-24's laminar and routing records are the candidate if this tree asks the question of another generator. Any part of UPG-21 or UPG-22 that is a gate the engine sets on its own amendments is not adopted, below.

### Not adopted, by any round

- **Claims:** "zero hallucination", "surpasses LLMs", superintelligence, and the suite's probability figures.
- **Performance figures not reproduced on the reference platform**, the suite's and ADR-0100's developer-machine figures among them.
- **On the hot path:** GPUs, AVX-512 lanes, RDMA and NVMe paging.
- **The engine changing its own behaviour beyond `REGISTRY`'s parameters:** the suite's "Lane B", a `GATE_EPISTEMIC` the engine sets, a constitution it co-designs.
- **A loop that does not end by construction.**

### Latest ≠ Newest, its scope stated

- **It is kept.** It governs what the tree depends on, which is a technology or a dependency on the hot path or in the engine's build, and a performance claim, which must be reproduced.
- **It does not govern what the engine does.** A mechanism, such as a learning rule, a critic, a sleep phase or a lookahead, enters by an ADR and a measured round.
- **The project's own documentation tools** are admitted by [ADR-0008](0008-documentation-governance.md) and [ADR-0185](0185-the-spec-tools-upgraded-and-spec-brief.md), not by this test.
- **A brief's "nothing else is adopted" is that round's scope**, written in its `## Not empowered`. The Latest ≠ Newest directive in its `## Standing directives` stays, as `.spec-brief.json` requires, applied to what the round could be tempted to depend on.
- `CLAUDE.md`'s principle 4, whitepaper §2.1 and `briefs/README.md` say so in this pull request.

## Consequences

- Good: the suite's proposals have a place each, and the next decisions have an order.
- Good: the doctrine says what it governs, so it is not read as a brake on what it never governed.
- Bad: seven milestones of measured rounds is many rounds. The order is a plan, and each round's verdict may reorder it.
- Bad: the suite's layouts and constants are not trusted, so each round that takes a candidate derives its layout again.
- Neutral: the suite stays outside the tree. This ADR cites it by its identifiers and does not copy it.

## Alternatives considered and why rejected

- **Option 1(b), the suite's roadmap as the plan**: its baseline is twenty-one image formats and eighty ADRs behind, its layouts in part no longer hold, and its phases are not measured rounds.
- **Option 1(c), nothing taken**: several of its proposals name the questions this tree's own ADRs already left open, a delayed reward and the lookahead among them.
- **Option 2(b), removing the doctrine**: the proposals it would refuse are refused by other rules, and what it does guard, the dependencies and the claims, would lose its guard.
- **Option 2(c), keeping it as written**: it would go on being read as the round's scope.

## Confirmation

- Whitepaper Appendix C: M10 to M15, marked planned by this ADR. §2.1: what the rule does not govern. Section 9's row.
- `CLAUDE.md`'s principle 4; `briefs/README.md`'s standing-directives row; the index; `CHANGELOG.md`.
- No file under `crates/`, `runtime/`, `benches/` or `scripts/` changes.
