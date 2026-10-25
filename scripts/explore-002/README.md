# Exploration 002's scripts

On `explore/002-what-the-pinned-tables-cost` alone ([ADR-0193](../../docs/adr/0193-an-exploration-lane.md)); the
note is `docs/explorations/002-what-the-pinned-tables-cost.md` on `main`. Node, no dependencies.

- `survey-tables.mjs <file.rs>`: a test file's top-level `const` and `static` items, and which are plain literals.
- `move-tables.mjs <tests dir> <stem> <v0|v1|v2>`: moves the plain literals of at least a kilobyte into
  `tests/data/<stem>.tables`. `v0` keeps the consts and adds the test that holds each table read to its const
  (commit `0807bb8`); `v1` replaces them by `LazyLock`s read from the file (`532d2b6`, with the two `_explore` copies of
  weekly tests added by hand). `4cac2e8` is `532d2b6` with the loader changed to `include_str!` by hand.
- `timings.mjs <dir>`: every unit of cargo's `--timings` reports in build-cost artifacts, one tree a job (runs
  40191314022 and 40192286352).
- `interleaved.mjs <dir>` and `per-binary.mjs <dir> <commit>`: the reports of run 40192831652, the trees built in turn
  on one runner; the note's tables 1 and 4.
- `sweep-phases.mjs <dir>` and `catchers.mjs <dir>`: a sweep's `mutants-out-runtime-*` artifacts, each in `<dir>/runtime-<n>/`;
  the note's section 2.
