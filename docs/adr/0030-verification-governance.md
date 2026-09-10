---
status: accepted
date: 2026-09-10
decision-makers: VirtualCortex maintainers
depends-on: ADR-0029
---

# ADR-0030: Verification governance — a mutation gate on the lines a change touches, a zero-dependency property kit with a lattice and a seeded walk, exhaustive tests where the domain allows, the tests in the engine's profile, and a determinism pin checked on two architectures; what is not adopted

## Context and Problem Statement

A directive asked for four additions to the way the workspace is verified: mutation testing (`cargo-mutants`), a red–green–invariant protocol with a discrete commit that proves a test failed before the rule made it pass, in-house property fuzzing over millions of inputs, and formal verification (`cargo-kani`) of the fixed-point routines. Its constraint is the workspace's own: no dependency of any kind in a state crate, nothing sub-1.0 or heavy in the runtime, nothing nightly as a gate (ADR-0005, ADR-0009, ADR-0023, ADR-0029; principle 4, Latest ≠ Newest).

The question underneath is the one quality goal 5 asks: which claims about the rules are checked by a tool in CI, and which are held by the reviewer's eye? Before this decision: every rule had tests, but nothing checked that the tests constrain the rule (a test that passes with the rule inverted is documentation, not a check); the tests ran in the debug profile only, though the engine runs in release without overflow checks; determinism across architectures (quality goal 1, target T-1) was asserted and never run; and the boundary tests were the ones a reviewer thought of.

## Decision Drivers

- Quality goal 5: a claim about a rule is checked by a tool that runs in CI.
- Principle 4: the tool has a stable specification and years of third-party production use; no nightly gate; no sub-1.0 dependency in a manifest.
- TC-2: a state crate declares no dependency of any kind, so a shared test helper cannot be a crate.
- Principle 5: a mechanical gate beats a protocol a reviewer has to remember.
- CI and the documented command set are one set.

## Considered Options

1. **Mutation testing on the changed lines as a blocking gate, the whole tree by hand; a property kit `include!`d into test modules; exhaustive tests where a domain is $2^{32}$ or smaller; the release profile in CI; a pinned arena hash checked on x86-64 and AArch64.** No manifest changes; one pinned external binary in CI.
2. The directive as written: mutation testing over the whole tree in CI with a percentage threshold; a mandated red commit per rule; property loops of $10^7$ iterations; Kani harnesses in CI.
3. Coverage (`cargo-llvm-cov`) with a percentage threshold.

## Decision Outcome

Option 1.

- **Mutation testing** (`cargo-mutants` 27.1.0, pinned; a CLI that copies the tree and needs no manifest change; configured in `.cargo/mutants.toml`). The gate: on every pull request, `cargo mutants --in-diff` over the lines the request changes, and one surviving mutant fails the check. The threshold is therefore not a number chosen for no reason but zero, in the smallest scope that is still the change's own: a new rule merges with the test that fails when the rule is wrong, which is what the red–green protocol wanted to show, checked by a tool after the fact instead of by the order of commits. The whole tree is run by hand (`cargo mutants --workspace`, about 2 214 mutants, 66 minutes on a developer machine) when a round ends; this decision's run found 141 survivors, of which 45 are equivalent to the original by construction (a `|` on disjoint bit fields, a clamp compared at its own bound, a shift by zero, a bound the domain cannot reach) and are excluded by name with the reason in the configuration file, and 96 were gaps in the tests, each now closed by a test that names the boundary. Mutants the tool cannot make (a `saturating_add` swapped for `+`, the class of F-4) are the arithmetic lint's (ADR-0029), not this gate's.
- **The property kit** (`testkit/prop.rs`): a Knuth MMIX linear congruential generator and the lattices, the `i32`, `u32`, `i16`, `u8` and `i128` values a rule must survive (the extremes and their neighbours, zero and its neighbours, the powers of two around 1.0 in Q16.16, the wheel's ring lengths and horizon). A crate `include!`s the file into a `#[cfg(test)] mod prop`, so the state crates share one generator and one lattice with no dependency of any kind (TC-2 holds; `scripts/check-deps.mjs` still passes) and the kit compiles under every workspace lint. A property test enumerates the lattice exhaustively in pairs, then walks a seeded sequence; both are deterministic, so a failure is an input, not a flake. The walks are $10^5$ to $10^6$ steps, which the debug profile runs in well under a second per test; the directive's $10^7$ buys nothing a seeded $10^6$ and the lattice do not, and would slow every `cargo test` by seconds per crate.
- **Exhaustive tests** where the domain is $2^{32}$ or smaller: `synaptic_efficacy_q16` over every `(i16, u8, u8)`, a `#[test] #[ignore]` named `exhaustive_…`, run with `cargo test --release -- --ignored exhaustive` before a release and by the weekly job. Enumerating a whole domain on the compiled binary checks every input on that target; it needs no prover, no nightly, and runs on Windows. It is a test, not a proof: a proof would cover every target and every compiler.
- **The release profile**: `cargo test --workspace --release --locked` is a blocking CI step. The engine runs in release, where an overflow wraps instead of panicking; the tests hold there too, which is what saturating-by-name means.
- **The determinism pin** (`runtime/cortex-runtime/tests/differential.rs`): the 128-unit random network with STDP after 20 000 ticks on one worker hashes (CRC-64/XZ over the 64 image bytes of every unit, the synapse arena and the spike train) to a constant pinned in the test beside the spike count, and CI runs the test on `ubuntu-24.04` and `ubuntu-24.04-arm`. Target T-1 (quality goal 1) is checked on two architectures on every push; a deliberate change to the dynamics moves the pin, and the change that moves it says why.
- **Timeouts.** A mutant that makes a test hang (a loop bound turned into its opposite, a barrier that never releases) is a mutant the tests detected; the tool reports it as a timeout with its own exit code (3), and the CI step counts it as caught: the step fails when `missed.txt` is not empty or the tool itself failed. The bound is the tool's automatic one, five times the unmutated run.
- **The weekly job** runs the exhaustive tests and the whole-tree mutation run on a schedule; it blocks nothing, since there is no pull request to block, and its survivors are the next round's list.
- **The documented command set** gains the release test, the in-diff mutation run and the exhaustive tests; `CONTRIBUTING.md`'s definition of done gains "the mutation gate on the diff passes" and "a new rule carries a lattice test".

### Consequences

- Good: a change to a rule does not pass the gate unless a test catches every mutant the tool can make in the lines it changed; the tests hold in the profile the engine runs in; quality goal 1 is checked, not asserted; the boundary tests are the lattice's, not the reviewer's memory.
- Good: no manifest changed; the state crates are still free of dependencies; the mutation tool is one pinned binary CI builds once and caches.
- Bad: the mutation job adds two to five minutes to a pull request that changes many lines; a docs-only request finds no mutants and passes in the time it takes to build.
- Bad: an equivalent mutant needs an exclusion by name; the list in `.cargo/mutants.toml` is a maintenance item, and a wrong entry would hide a defect, which is why every entry names its function and its reason.
- Bad: the pin moves with any change to the dynamics; that is its purpose, and the cost is one line per deliberate change.
- Not adopted: a red commit per rule (a failing commit on `main` breaks bisection and proves less than the mutation gate proves mechanically); `cargo-kani` (0.x, no stability policy, no Windows, its own toolchain: it fails principle 4 as a gate; the routines it would prove are enumerated exhaustively or lattice-tested instead, and the arithmetic lint forbids the plain operation; revisit at a stable release with Windows support); Miri (nightly); `loom` for the deque and the ring (a dev-dependency with a heavy tree in the runtime; the contention and differential tests hold them today); coverage (a percentage is a Target nobody derives; mutation testing asks the question coverage cannot); `proptest` or `quickcheck` (dependencies in a state crate, TC-2).

## Alternatives considered and why rejected

- Option 2's whole-tree mutation run in CI: 66 minutes per push, and a percentage threshold either fails on an equivalent mutant or is set low enough to pass anything. The in-diff form has the right scope and a threshold of zero.
- Option 3: coverage counts lines a test executes, not lines a test constrains; the first mutant this decision's run found was in a line every test executed.
- A property-test crate under `testkit/` as a dev-dependency: a dependency of a state crate (TC-2, ADR-0029's manifest check would fail); the `include!` costs nothing and keeps the rule.
- Fuzzing with `cargo-fuzz`: nightly, libFuzzer, and coverage-guided search over byte inputs is the wrong shape for integer rules with two or three arguments; the lattice and the seeded walk are the shape.

## Confirmation

- `.github/workflows/ci.yml`: the `mutants` job on pull requests, the `arm64` job, the release-profile step; `.cargo/mutants.toml` with the named exclusions.
- `testkit/prop.rs`, and `mod prop` in `cortex-core` (membrane, plasticity, synapse, wheel), `cortex-arithmetic`, `cortex-cerebellum` and `cortex-embodiment` (the voice); `exhaustive_the_efficacy_is_bounded_by_one_for_every_input`.
- `the_random_network_hashes_to_the_pinned_value_on_every_architecture` in the runtime's differential tests; the pin.
- Whitepaper Appendix B rows V-6 (mutation), V-7 (determinism on two architectures) and the release-profile row; §10 T-1 and Appendix C M7 say what runs; §11 finding F-26 lists the survivors that became tests.
