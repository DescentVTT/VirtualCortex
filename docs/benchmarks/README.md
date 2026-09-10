# Benchmarks

`benches/cortex-bench` measures the parts of whitepaper target **T-3** that exist ([ADR-0014](../adr/0014-benchmark-harness.md)). This file says how to run them, what makes a run **admissible**, and how a result is recorded. The rule it serves is [ADR-0010](../adr/0010-measured-or-target.md): a figure enters whitepaper §10.2's Measured column only from an admissible run.

## What is measured

| Benchmark | Subject | Whitepaper |
| :--- | :--- | :--- |
| `wheel/advance_empty` | `FlatTimingWheel::advance` on an empty production wheel | §5.2.1, R-2 |
| `wheel/schedule_then_advance/both_rings` | one `schedule` (delay uniform over 1..2559) plus one `advance`, steady state | R-1 step 1, ADR-0013 |
| `wheel/schedule_then_advance/fine_ring` | the same with delays in 1..255 | R-1 step 1 |
| `efficacy/single`, `efficacy/batch_1024` | `synaptic_efficacy_q16` | §8.1, ADR-0012 |
| `gating/compute_gating` | `BasalGangliaChannelState::compute_gating` | §5.2.5 |
| `ignition/step_ignition_x64` | 64 sub-threshold `step_ignition` calls on a fresh slot (divide by 64) | §5.2.8 |
| `mailbox/push_drain_x16` | sixteen `mailbox_push` calls into one unit then one `mailbox_drain` walking them (divide by 16) | R-1 steps 2–3, ADR-0017 |
| `gate/schedule_begin_end` | `try_schedule`, `begin_turn`, `end_turn` on an idle unit with an empty mailbox | R-1 step 3, ADR-0017 |
| `neuron/integrate` | one `integrate` tick under a pseudo-random drive that fires the unit now and then | R-1 step 5, ADR-0018 |
| `stp/step_stp` | one `step_stp` per presynaptic spike with a pseudo-random interval, both exponentiations included | §8.8, ADR-0019 |

Inputs come from `cortex_bench::Lcg` seeded with `Lcg::SEED`, so every run measures the same sequence. What is **not** measured: fan-out (R-1 step 6), which does not exist; T-8 throughput, which has no subject yet.

## Running

Anywhere, for a developer's own information (never admissible):

```bash
cargo bench -p cortex-bench --bench hot_path
```

Smoke mode, what CI runs (one iteration per benchmark, no timing asserted):

```bash
cargo bench -p cortex-bench --bench hot_path -- --test
```

## An admissible run

All of the following, or the file says `admissible: no`.

1. **Platform**: the reference platform of whitepaper §7.1 (64-core x86-64-v4 or ARMv9-A, 64 GB DDR5 ECC, Linux). Record CPU model, memory, kernel version and the full kernel command line.
2. **Isolation**: the benchmark runs on a core in `isolcpus` and `nohz_full`, pinned with `taskset -c <core>`; no other load on the socket.
3. **Fixed frequency**: turbo/boost disabled and the governor set to `performance` (`cpupower frequency-set -g performance`); record the resulting fixed frequency.
4. **Counters**: wrap the run in `perf stat -e cycles,instructions,cache-misses,branch-misses` and record the output.
5. **Samples**: `--warm-up-time 3 --measurement-time 10` at least; record criterion's reported sample count.
6. **Toolchain and tree**: record `rustc --version`, `criterion`'s version from `Cargo.lock`, and the commit hash; the working tree must be clean.
7. **Command**: the exact command line, copy-pasteable.

## Recording a result

One file per run: `docs/benchmarks/results/<YYYY-MM-DD>-<host>.md`, starting with:

```text
admissible: yes | no (reason)
commit: <hash>
toolchain: rustc <version>, criterion <version>
platform: <cpu>, <memory>, <os and kernel>
isolation: <isolcpus/nohz_full core, taskset> | none
frequency: <fixed MHz> | not fixed
command: <exact command>
```

followed by criterion's `time:` lines verbatim and, for admissible runs, the `perf stat` output. Results directories under `target/criterion/` are build output and are not committed.

## Promoting a figure

To move a number into whitepaper §10.2's Measured column: the results file must say `admissible: yes`; the table cell links to that file and names the benchmark; the commit hash in the file is the tree the number was produced from. A non-admissible file may be cited in prose only with the words "not admissible" beside the number. Reviewers reject anything else ([ADR-0010](../adr/0010-measured-or-target.md)).
