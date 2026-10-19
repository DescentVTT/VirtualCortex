---
status: proposed
date: 2026-10-19
depends-on: ADR-0196
decision-makers: VirtualCortex maintainers
---

# ADR-0198: The engine's default, built — ADR-0196's decision in the engine: `Config::synaptic_gain_q16`, the gain `Executor::new` starts an engine at, default 1.0, refused outside 0.25 to 4 with `ConfigError::GainOutOfRange`, the image's outranking it; `Config::reads_its_targets()`, `Config::default()` with ADR-0196's five values; an engine born under it writes H-41's candidate's image byte for byte on seeds 22, 23 and 24; `Config::default()` the engine it built, every pin held by one dispatch at `scope=both`; no record, no format, no hypothesis

## Context and Problem Statement

[ADR-0196](0196-the-engines-default.md) decided the engine's default for an image no derivation has settled and left its building to brief 076. Its decision:
- *"`Config::synaptic_gain_q16`, a `u32` in Q16.16, default `GAIN_ONE_Q16`: the gain `Executor::new` puts in the homeostasis record. `Config::default()` builds the engine it built."*
- *"Refused outside `GAIN_MIN_Q16` to `GAIN_MAX_Q16`, 0.25 to 4, the controller's own bounds."* And *"For an engine built from an image, the image's gain outranks it, as it outranks the step."*
- *"`Config::reads_its_targets()` is `Config::default()` with five fields set"*: the modulation at rest `0x8000`, the gain `0x0001_C000`, the step `0x1000`, the read of 128 windows and its flag.
- *"On each of seeds 22, 23 and 24 at 1 024 units, an engine built from `reads_its_targets()` with H-41's sizing, its arenas synthesized from the network's prior, writes the image H-41's candidate writes before its lead-in."*

Brief 076's empowerment names the choices: the names of the field, the default and the error, and where the refusal is made; whether `reads_its_targets()` is a `const fn`; how the byte-for-byte check is built, and whether H-41's `candidate_under` builds its configuration from `reads_its_targets()`.

What was read first (principle 2), on 2026-10-19, at `main` `5f7150e`:

1. **The configuration** (`runtime/cortex-runtime/src/executor.rs`). `Config` derives `Clone, Debug, PartialEq, Eq`; its `Default` sets the step 0, no rate target, no read, the flag clear, the inhibitory period 20 000 and the modulation at rest 1.0. `Executor::new` validates each field in turn and returns a `ConfigError`; it builds the homeostasis record as `HomeostaticDrivePool { control_step_q0_16, sleep_shift, ..HomeostaticDrivePool::new() }`, so the gain starts at `GAIN_ONE_Q16`, and the workers' gain word `shared.gain` at the same constant.
2. **The gain the workers read.** Before each tick's first barrier the coordinator stores the record's `synaptic_gain_q16` into `shared.gain`, and every turn scales its sums by that word after the barrier. The word's initial value is therefore never read.
3. **The loader** (`runtime/cortex-runtime/src/image.rs`, `Image::decode`). It builds the engine with `Executor::new(Config { …, rate_read_windows: 0, istdp_target_read: false, ..config })`, so every field it does not take from the image, the step among them, is the caller's and validated by `new`. The read and the flag are forced off because the image's are set after it, and a read beside the image's period would be refused. It then replaces the whole homeostasis record with the image's (`set_homeostasis`), which carries the step and the gain.
4. **The harness** (`runtime/cortex-runtime/tests/instrument/harness.rs`, `tests/inhibition.rs`). `config(units, workers, baseline)` builds on `..Config::default()`. `at_gain` synthesizes the prior into an engine built from a configuration, writes its image, lays the gain in the homeostasis record and reloads it under the same configuration. `candidate_under(p, units, gain, settling)` is `at_gain` on `config(units, 2, BASELINE_Q16)` with the settling's step, target, read and flag. It builds every settling's candidate (`GAIN_HELD`, `AT_AN_EIGHTH`, the rate's, the read's, both targets'), and every derivation of the tree runs through it.
5. **H-41's pins** (`tests/inhibition.rs`). `BOTH_RULES` holds each network's calibrated gain at `114688`, `0x0001_C000`, on seeds 22, 23 and 24. `UNDER_BOTH_TARGETS` is the step `0x1000`, no target, the read 128 and the flag set.
6. **What spells the configuration out whole.** T-1 (`tests/differential.rs`) and the contention test (`tests/contention.rs`) write every field, so that a change to `Config::default()` moves neither. Every other configuration in the tree, the benchmarks' included, is a struct update.

## Decision Drivers

- ADR-0196's decision and brief 076's standing directives: the five values and no other; `Config::default()` the engine it built; no record, no section, no format, no state crate; no pinned number moved, not even by a mask; no derivation, lead-in or arm; no dependency, no float, no `unsafe`.
- **One rule for the configuration and the loader** (principle 5): a gain past the bounds is refused where a step past its bound is.
- **The default is what H-41 ran**, shown on its images rather than asserted.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)).

## Considered Options

1. **The refusal**: (a) in `Executor::new`, beside the step's, so that a configuration given to the loader is refused as a step past its bound is; (b) `Image::decode` forces the configuration's gain to 1.0, as it forces the read off.
2. **`reads_its_targets()`**: (a) a `fn` struct-updated onto `Self::default()`; (b) a `const fn`, which would need the defaults spelled out a second time, since `Default::default` is not `const`.
3. **The byte-for-byte check**: (a) beside H-41's tests, comparing an engine born under the default with `candidate_under` as it is; (b) `candidate_under` rebuilt on `reads_its_targets()`.
4. **The workers' gain word**: (a) started at the configuration's gain; (b) left at 1.0.

## Decision Outcome

**Options 1(a), 2(a), 3(a) and 4(a).** Built in brief 076's first commit.

### The gain an engine starts with (Implemented)

- **`Config::synaptic_gain_q16: u32`**, Q16.16, default `GAIN_ONE_Q16`. `Executor::new` puts it in the homeostasis record and starts `shared.gain` at it. At 1.0 the record is `HomeostaticDrivePool::new()`'s with the step and the sleep shift, as it was.
- **`ConfigError::GainOutOfRange`**, after `ControlStepOutOfRange`: `Executor::new` refuses a gain outside `GAIN_MIN_Q16..=GAIN_MAX_Q16`, right after the step's refusal.
- **For an engine built from an image the image's gain outranks it**, by the record `set_homeostasis` lays whole. The loader passes the configuration's gain to `new` as it passes the step, so a gain past the bounds is refused at the load too. Option 1(b) was not taken: the read is forced off because the image's read is set after `new` and would be refused beside the image's period; the gain meets no such refusal.
- **The workers' word** starts at the configuration's gain. It is stored from the record before the first tick's first barrier, so no tick reads its initial value; it is started there so that the two never disagree.

### The default (Implemented)

`Config::reads_its_targets()`, a `fn`:

| Field | Value |
| :--- | :--- |
| `modulation_baseline_q16` | `0x8000` |
| `synaptic_gain_q16` | `0x0001_C000` |
| `control_step_q0_16` | `0x1000` |
| `rate_read_windows` | 128 |
| `istdp_target_read` | `true` |

and `..Self::default()` for every other field. The sizing is the caller's, by struct update. Option 2(b) was not taken: a second copy of the defaults is one more place to keep in step, and nothing in the tree needs the default in a `const`.

### Its tests

- **In `executor.rs`'s tests:**
  - the gain 1.0 by default, `Config::default()` building the record `HomeostaticDrivePool::new()` and the word 1.0; from 0.25 to 4, both included, the record's gain and the word the configuration's, the step and the sleep shift beside it the configuration's; one LSB past either bound, zero and the width refused;
  - `reads_its_targets()` field by field: the five values, each other than the bare engine's, and with those five put back, `Config::default()`;
  - an engine born under it, at 1 024 units, reading with 128 windows left and the sum 0, at the gain 1.75 and the step a sixteenth, its flag set, no target, the inhibitory period 20 000, the modulation 0.5 and the learning configuration unset; its image decoded under `Config::default()` is the same engine, written again byte for byte.
- **Over the lattice** (`gain_prop`): `U32_LATTICE`, both bounds and their neighbours, and a seeded walk of a thousand gains inside the bounds, edge-biased and across the width; an engine is built exactly where an oracle written from ADR-0196's text, 0.25 to 4 inclusive, accepts the gain.
- **In `tests/criticality.rs`**, on the recurrent network at an eighth:
  - an engine born at 1.5 by its configuration writes the image of one born at 1.0 with 1.5 laid in its record, byte for byte;
  - loaded under a configuration at 1.0 or at 1.75, that image's gain and step outrank the configuration's;
  - the born engine and the loaded one run a window alike, the gain moving;
  - a configuration at one LSB past either bound is refused at the load with `GainOutOfRange`.
- **T-1 and the contention test** spell the new field at `GAIN_ONE_Q16`. Neither pin moved.

### H-41's candidate, byte for byte

`the_engine_s_default_writes_h_41_s_candidate_byte_for_byte_on_its_three_networks`, beside H-41's tests in `tests/inhibition.rs` (option 3(a)):
- **The configuration.** H-41's sizing is `config(1024, 2, BASELINE_Q16)`'s: two workers, 1 024 units, the prior's blocks, $2^{16}$ nodes a worker, the injector's entries and the train's capacity, struct-updated onto `reads_its_targets()`. With its gain put back to 1.0 it equals the candidate's configuration, `UNDER_BOTH_TARGETS`'s step, target, read and flag on `config(1024, 2, BASELINE_Q16)`.
- **The gain.** On each network the gain H-41's derivation calibrated, `BOTH_RULES[n]`'s, is the default's, 1.75.
- **The images.** An engine built by `Executor::new` from that configuration, its arenas synthesized from the network's prior, writes the image `candidate_under(p, 1024, 1.75, UNDER_BOTH_TARGETS)` writes:

  | Network | Bytes | CRC-64 of both images |
  | :--- | ---: | :--- |
  | Seed 22 | 590 656 | `0xf3cddfb95c264715` |
  | Seed 23 | 590 656 | `0x7ec77b205b45f5a4` |
  | Seed 24 | 590 656 | `0xb9f74729bbb099e0` |

- **Its cost.** It runs no tick and is in the gate: 1.3 to 1.7 s in the debug profile on the developer machine (a ratio, not admissible). It is the gate's one test of [ADR-0061](0061-the-learning-runs-leave-the-gate.md)'s class from this round, the one that builds the reference network. The rule's own tests above are unit tests of milliseconds and one window on 64 units.
- **`candidate_under` is unchanged** (option 3(b) not taken). It builds every settling's candidate and every derivation's pins run through it, and the gain of each comes from its image. The check holds the two equal instead, and nothing is re-pinned.

### What the dispatch read

To be written from the dispatch at `scope=both` once its eighteen whole-domain shards end.

## Consequences

- Good: a host has one configuration to start an engine that reads its network's targets, and its image is H-41's candidate's.
- Good: the gain an engine starts with is a field of the configuration, refused by the controller's bounds, and no longer only an image's.
- Good: `Config::default()` is the engine it was; T-1 and every other pin hold.
- Bad: two configurations to keep in step, as ADR-0196 says. The field-by-field test fails when a field joins `Config` and the default without a decision.
- Neutral: one field and one error more; no record, no section, no format.

## Confirmation

- `cargo test --workspace --locked` runs the six tests above in the gate.
- The pull request's mutation gate meets the changed lines of `executor.rs`: the refusal, the record's field and `reads_its_targets()`.
- One dispatch at `scope=both` on the last commit under `src/` and `tests/` holds every pinned number of the tree.
- Whitepaper 4.134.0 carries §5.2's paragraph, §8.8's rate row naming the default Implemented, and §9's row.
