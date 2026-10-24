---
status: proposed
date: 2026-10-24
depends-on: ADR-0205
title: The probe in the engine's default, built
kind: built
summary: '`Config::reads_its_targets()` sets `step_probe`, its other five values unchanged. An engine born under it writes H-42''s candidate''s image byte for byte on seeds 22, 23 and 24, and with the probe unset H-41''s. One dispatch at scope=both held every other pin.'
decision-makers: VirtualCortex maintainers
---

# ADR-0206: The probe in the engine's default, built

**In brief**
- **Decision.** `Config::reads_its_targets()` gains `step_probe: true` and nothing else, as [ADR-0205](0205-the-probe-in-the-engines-default.md) decided. An engine born under it writes H-42's candidate's image byte for byte on all three networks.
- **Why.** ADR-0205 took H-42's step 3 with no hypothesis. The default's tests and one dispatch are what show it.
- **Left open.** The default on a network, a size or a generator H-42 did not run. M10 is next.

## Context and Problem Statement

ADR-0205 decided that the engine's default sets the probe of the step, and left its building to brief 079.
Its decision, in its words: *"`Config::reads_its_targets()` gains `step_probe: true`. Its other five values stay."*
And: *"On each of seeds 22, 23 and 24 at 1 024 units, an engine born under the default with H-42's sizing writes the image H-42's candidate writes before its lead-in."*

What was read first (principle 2), on 2026-10-24 at `main` `8c48fc7`:
1. **The default** (`runtime/cortex-runtime/src/executor.rs`). `Config::reads_its_targets()` is `Config::default()` with the modulation at rest `0x8000`, the gain `0x0001_C000`, the step `0x1000`, the read of 128 windows and its flag.
2. **Three tests in `executor.rs` hold the default.**
   - `reads_its_targets_is_the_default_with_adr_0196_s_five_values_and_no_other`, field by field.
   - `an_engine_born_under_the_default_reads_at_1_75_with_its_flag_and_its_image_reads_back_alike`, an engine born under it and its image read back.
   - ADR-0202's `the_probe_is_unset_by_default_and_refused_without_a_read`, which asserted the default left the probe unset.
3. **The byte-for-byte test** (`tests/inhibition.rs`). `the_engine_s_default_writes_h_41_s_candidate_byte_for_byte_on_its_three_networks` compared the default at H-41's sizing with `candidate_under(…, &UNDER_BOTH_TARGETS)`.
4. **H-42's candidate.** `UNDER_THE_PROBE` is `UNDER_BOTH_TARGETS` with `probe` set. `PROBE_SEEDS` is `BOTH_SEEDS`, and `PROBE_RULES` pins the gain 114 688, 1.75, on each network. `candidate_under` already sets the probe's flag from the settling (ADR-0203).

Brief 079's empowerment names the choices: how the byte-for-byte test is moved, whether H-41's comparison is kept beside it with the probe unset, and the tests' names.

## Decision Drivers

- ADR-0205's decision and brief 079's standing directives: one field of the default, `Config::default()` the engine it built, no record, no format and no state crate.
- **No pinned number moves but the default's tests**, and no derivation, lead-in or arm runs.
- **The gate grows by no test** ([ADR-0061](0061-the-learning-runs-leave-the-gate.md)): the byte-for-byte test is moved, not added.
- **The default is H-42's candidate**, shown on its images rather than asserted.
- **Every mutant in the changed lines caught** ([ADR-0030](0030-verification-governance.md)).

## Considered Options

1. **The byte-for-byte test**:
   - (a) moved to H-42's candidate alone;
   - (b) moved, with H-41's comparison kept beside it, the probe put back unset.
2. **ADR-0202's assertion that the default leaves the probe unset**:
   - (a) inverted in place;
   - (b) removed there, the default's field-by-field test holding the probe.

## Decision Outcome

**Options 1(b) and 2(b).**
Built in `771494d`, brief 079's one commit under `src/` and `tests/`, in pull request [#229](https://github.com/DescentVTT/VirtualCortex/pull/229).

### The default (Implemented)

`Config::reads_its_targets()`, a `fn` struct-updated onto `Self::default()`:

| Field | Value | Since |
| :--- | :--- | :--- |
| `modulation_baseline_q16` | `0x8000` | ADR-0196 |
| `synaptic_gain_q16` | `0x0001_C000` | ADR-0196 |
| `control_step_q0_16` | `0x1000` | ADR-0196; no window reads it while the probe is set |
| `rate_read_windows` | 128 | ADR-0196 |
| `istdp_target_read` | `true` | ADR-0196 |
| `step_probe` | `true` | ADR-0205 |

`Config::default()` is unchanged, and so is every other configuration of the tree.

### Its tests

The brief named two of the default's tests. Four held a value of it, and each changes by the probe alone.
- **Field by field**, renamed `reads_its_targets_is_the_default_with_adr_0196_s_five_values_the_probe_and_no_other`.
  - The five values and the probe set, each other than the bare engine's.
  - With the six put back, `Config::default()`.
- **An engine born under it** at 1 024 units reads the probe's flag set beside the read under way. Its image, decoded under `Config::default()`, reads the flag back and is written again byte for byte. Every other reading is the one it was.
- **ADR-0202's test** keeps `Config::default()`'s probe unset and drops its line on the default (option 2(b)). The field-by-field test holds the probe the other way, so one test holds each value.
- **The byte-for-byte test** is renamed `the_engine_s_default_writes_h_42_s_candidate_byte_for_byte_on_its_three_networks` (option 1(b)), below.

### H-42's candidate, byte for byte

- **The configuration.** The default at H-42's sizing, `config(1024, 2, BASELINE_Q16)`'s, H-41's, is the candidate's configuration but for the gain: `UNDER_THE_PROBE`'s step, target, read, flag and probe.
- **The gain.** On each network the gain `PROBE_RULES[n]` pins, 1.75, is the default's.
- **The images.** An engine built by `Executor::new` from the default, its arenas synthesized from the network's prior, writes the image `candidate_under(p, 1024, 1.75, UNDER_THE_PROBE)` writes.
- **H-41's beside it** (option 1(b)). With the probe put back unset, the default is H-41's candidate's configuration but for the gain, and writes H-41's candidate's image. So the probe is the one thing the default gained.

| Network | Bytes | CRC-64, the default and H-42's candidate | CRC-64, the probe unset and H-41's candidate |
| :--- | ---: | :--- | :--- |
| Seed 22 | 590 656 | `0x34dfbafb1976b31d` | `0xb2d158b69dc0a3cf` |
| Seed 23 | 590 656 | `0xb9d51e621e1501ac` | `0x3fdbfc2f9aa3117e` |
| Seed 24 | 590 656 | `0x7ee5226bfee06de8` | `0xf8ebc0267a567d3a` |

- H-41's CRCs are not [ADR-0198](0198-the-engines-default-built.md)'s: its images were format 24, and these are format 25 since [ADR-0202](0202-a-step-the-engine-makes-from-its-own-loop-built.md). The header's version is in the checksum.
- Each pair of images differs in nine bytes: the modulator section's `[27]`, and that section's CRC-64 in the directory.
- **Its cost.** It runs no tick and is in the gate. It builds six images where it built three: 2.1 s in the debug profile on the developer machine, a ratio and not admissible.

### What the dispatch read

- **The dispatch.** Run [40097581635](https://github.com/DescentVTT/VirtualCortex/actions/runs/40097581635) at `scope=both` on `771494d`, at 13:18:40Z on 2026-10-24, once that commit was pushed. A file under `src/` changed, so by [ADR-0075](0075-the-dispatch-scope-follows-the-diff.md) the scope is `both`.
- **No commit under `src/` or `tests/` follows it.** The commits after it change the documents, the brief and the cost table.
- **Every whole-domain shard is green, and every pinned number held** on x86-64. All 166 `exhaustive` tests passed, each exit 0 with one test reported, none excluded, 143 646 s summed.
- The round added no whole-domain test and changed none. So these are the tree's 166 of brief 078's reproduction, H-42's derivations and arms among them, each holding its tables.

| Shard | Job | Of its bound | Tests | Their seconds summed | Tests' wall time |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 85.5 min | 48 % | 8 | 9 656 | 5 044 s |
| 1 | 80.2 min | 45 % | 8 | 9 430 | 4 720 s |
| 2 | 69.5 min | 39 % | 10 | 8 155 | 4 079 s |
| 3 | 57.8 min | 32 % | 10 | 6 779 | 3 401 s |
| 4 | 72.8 min | 40 % | 10 | 8 455 | 4 284 s |
| 5 | 71.3 min | 40 % | 9 | 8 287 | 4 192 s |
| 6 | 72.8 min | 40 % | 9 | 8 390 | 4 272 s |
| 7 | 77.4 min | 43 % | 8 | 8 857 | 4 560 s |
| 8 | 74.3 min | 41 % | 10 | 8 721 | 4 365 s |
| 9 | 65.0 min | 36 % | 10 | 6 932 | 3 812 s |
| 10 | 46.6 min | 26 % | 10 | 5 396 | 2 726 s |
| 11 | 71.5 min | 40 % | 10 | 8 180 | 4 206 s |
| 12 | 65.3 min | 36 % | 9 | 7 595 | 3 826 s |
| 13 | 42.7 min | 24 % | 9 | 4 940 | 2 502 s |
| 14 | 64.8 min | 36 % | 10 | 7 577 | 3 806 s |
| 15 | 74.5 min | 41 % | 8 | 8 457 | 4 385 s |
| 16 | 68.8 min | 38 % | 9 | 7 935 | 4 043 s |
| 17 | 86.0 min | 48 % | 9 | 9 904 | 5 078 s |

- The bound is 180 minutes a job ([ADR-0179](0179-the-bounds-rule-restated.md)). **No job passed 60 % of it**: the longest, shard 17's, took 86.0 minutes, 48 %.
- Sixteen shards started at 13:18:43Z to 13:20:01Z, and shards 13 and 15 waited for runners until 13:46:55Z and 13:56:50Z. The last ended at 15:11:19Z, 1 h 53 min after the dispatch.
- **The cost table** is regenerated from this run's artifacts: 166 lines, 143 646 s, where brief 078's reproduction read 130 054 s.
- **The pull request's gate** on `771494d`, run 40097564802, was green in its five checks.
- **The sweep of this dispatch had not ended** when this was written, at 15:15Z: its thirteen jobs were queued behind the shards ([ADR-0166](0166-the-sweep-after-the-shards.md)). The first ADR merged after it ends records it ([ADR-0158](0158-the-sweep-off-the-next-decisions-path.md)).

### The schedule's sweep of 2026-10-19, recorded here

The schedule's weekly run on `main`, run 39222543243 at `417af36`, ended green at 16:51Z on 2026-10-19. No ADR merged since has recorded its sweep, so this one does.

| Outcome | Mutants |
| :--- | ---: |
| Caught | 3 736 |
| Missed | 0 |
| Unviable | 157 |
| Timeouts | 23 |
| All | 3 916 |

- 22 timeouts are ADR-0199's inherent ones, by [ADR-0062](0062-the-first-complete-sweeps-list.md): `cortex-core`'s three chain iterators, six; the injector, twelve; the barrier's wait, one; the workers' stop and run, three.
- `image.rs:1132:24`, *"delete ! in Image::decode"*, ran 1 939 s beside an injector mutant that timed out. It had stopped with none of its unreported tests its own module's. ADR-0204 read the same case as F-42's kind, by [ADR-0063](0063-the-sweep-reads-its-own-timeouts.md)'s evidence, and so does this.
- No survivor, so no finding.

## Consequences

- Good: an engine born under the default makes its own step from its own loop, and the rate's rule carries no constant the designer chose.
- Good: the default's image is H-42's candidate's on each of its networks, so H-42's runs are the default's runs there.
- Bad: the byte-for-byte test builds twice the images it built.
- Neutral: one line of the default; no record, no section and no format.

## Confirmation

- `cargo test --workspace --locked` runs the four tests above in the gate.
- The pull request's mutation gate on `771494d`, run 40097564802, made 2 mutants in the changed lines of `executor.rs` and caught both.
- One dispatch at `scope=both` on `771494d`, the last commit under `src/` and `tests/`, holds every other pinned number of the tree.
- `scripts/exhaustive-costs.tsv` is regenerated from that dispatch.
