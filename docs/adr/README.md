# Architecture decision records

One file per decision, in [MADR](https://adr.github.io/madr/) format, numbered in order of acceptance. The `status:` front-matter field is read by `spec-graph`, which fails CI if a live document depends on a retired decision or if an open question is delegated to one.

To add a decision: copy the most recent file, take the next number, set `status: proposed`, open a pull request. A decision becomes `accepted` when merged with that status, `superseded` when a later ADR names it in `supersedes:`, and `rejected` if closed without adoption. Never renumber.

| ID | Title | Status |
| :--- | :--- | :--- |
| [ADR-0001](0001-64-byte-pod-records.md) | 64-byte cache-line POD records as the unit of state | accepted |
| [ADR-0002](0002-q16-16-fixed-point.md) | Q16.16 fixed-point arithmetic; no IEEE-754 on the hot path | accepted |
| [ADR-0003](0003-zero-allocation-hot-path.md) | Zero allocation and zero syscalls on the hot path | accepted |
| [ADR-0004](0004-two-tier-timing-wheel.md) | Two-tier timing wheel for axonal delay | accepted |
| [ADR-0005](0005-crate-per-subsystem.md) | One crate per subsystem; no dependencies among state crates | accepted |
| [ADR-0006](0006-virtual-actor-turn-invariant.md) | Virtual-actor turn invariant enforced by an atomic gate | accepted |
| [ADR-0007](0007-cortex-image-format.md) | The `.cortex` memory-mappable image format | accepted |
| [ADR-0008](0008-documentation-governance.md) | Documentation governance: arc42, MADR, BCP 14, executable assertions | accepted |
| [ADR-0009](0009-rust-edition-and-msrv.md) | Rust edition 2024 and a pinned MSRV | accepted |
| [ADR-0010](0010-measured-or-target.md) | Every performance figure is Measured or Target, never asserted | accepted |
| [ADR-0011](0011-epoch-based-reclamation.md) | Epoch-based reclamation for structural plasticity | accepted |
| [ADR-0012](0012-synaptic-weight-q1-15.md) | Sixteen-bit synaptic base weights are Q1.15 | accepted |
| [ADR-0013](0013-timing-wheel-geometry.md) | Timing wheel geometry: 256 × 10 µs fine, 256 × 100 µs coarse, fixed-capacity token lists (amends ADR-0004) | accepted |
| [ADR-0014](0014-benchmark-harness.md) | Benchmark harness: criterion 0.7, confined to a bench-only crate | accepted |
| [ADR-0015](0015-embodiment-frame-abi.md) | Embodiment frame ABI and single-producer single-consumer ring protocol | accepted |
| [ADR-0016](0016-thirty-two-crate-architecture.md) | Thirty-two state crates: fourteen subsystems admitted, three boundaries moved, and the admission test for the next one (amends ADR-0005) | proposed |
