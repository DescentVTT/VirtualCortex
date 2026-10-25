#!/usr/bin/env bash
# Exploration 002, on `explore/002-what-the-pinned-tables-cost` alone: what the test crates cost to
# build on the hosted runner (ADR-0193; nothing here is in `main`'s workflow).
#
# `build-cost.sh <debug|release>` builds every test binary of the workspace from a clean target
# with `cargo test --workspace --no-run --timings` (and `--release` for release), then changes one
# file under runtime/cortex-runtime/src/ as cargo-mutants changes it for one mutant and builds
# again: the build a mutant of the runtime pays. It does this twice, each time from `cargo clean`,
# with a different mutant the second time, so that each figure has two readings on one runner. In
# debug it also times a cold `cargo check --workspace --all-targets`, the gate's Check and Clippy
# steps' front end.
#
# Where the tree has the test `the_moved_tables_parse_explore` (the placements tried by the
# exploration), it is run once after the cold build, in the same profile: what reading the tables
# at run time costs a test binary.
#
# Writes `build-cost/`: `<step>.log`, `<step>.html` (cargo's timing report), `steps.tsv` (step,
# seconds, exit status) and `sizes.tsv` (each test binary's bytes).

set -u -o pipefail

profile=${1:?usage: build-cost.sh <debug|release>}
case "$profile" in
  debug) flag=() ;;
  release) flag=(--release) ;;
  *) echo "usage: build-cost.sh <debug|release>"; exit 2 ;;
esac

out=build-cost
mkdir -p "$out"
: > "$out/steps.tsv"
status=0

step() {
  local label=$1
  shift
  rm -rf target/cargo-timings
  local started=$SECONDS
  "$@" > "$out/$label.log" 2>&1
  local rc=$?
  local secs=$((SECONDS - started))
  printf '%s\t%s\t%s\n' "$label" "$secs" "$rc" >> "$out/steps.tsv"
  if [ -f target/cargo-timings/cargo-timing.html ]; then
    cp target/cargo-timings/cargo-timing.html "$out/$label.html"
  fi
  tail -n 3 "$out/$label.log"
  echo "== $label: ${secs} s, exit $rc"
  if [ "$rc" -ne 0 ]; then status=1; fi
}

# Two mutants the sweep of run 40097581635 built and caught, written as cargo-mutants writes them.
mutate() {
  local file
  case "$1" in
    1) file=runtime/cortex-runtime/src/branching.rs
       sed -i 's/^        if p\.tick < exec\.ticks() {$/        if p.tick > \/* ~ changed by cargo-mutants ~ *\/ exec.ticks() {/' "$file" ;;
    2) file=runtime/cortex-runtime/src/arena.rs
       sed -i '40s/^        self\.cells\.is_empty()$/        true \/* ~ changed by cargo-mutants ~ *\//' "$file" ;;
  esac
  if git diff --quiet -- "$file"; then
    echo "mutant $1 did not apply to $file"
    exit 1
  fi
  git diff --stat -- runtime/cortex-runtime/src
}

for round in 1 2; do
  cargo clean
  if [ "$profile" = debug ]; then
    step "check-cold-$round" cargo check --workspace --all-targets --locked --timings
  fi
  step "test-cold-$round" cargo test --workspace --no-run --locked --timings "${flag[@]}"
  if [ "$round" = 1 ]; then
    find target/"$profile"/deps -maxdepth 1 -type f -executable -printf '%f\t%s\n' | sort > "$out/sizes.tsv"
    if grep -q 'the_moved_tables_parse_explore' runtime/cortex-runtime/tests/*.rs; then
      step "parse-$round" cargo test --workspace --locked "${flag[@]}" -- --ignored --exact the_moved_tables_parse_explore --nocapture
    fi
  fi
  mutate "$round"
  step "test-incremental-$round" cargo test --workspace --no-run --locked --timings "${flag[@]}"
  git checkout -- runtime/cortex-runtime/src
done

{
  echo "### Build cost, $profile, at $(git rev-parse --short HEAD) (exploration 002)"
  echo
  echo "| Step | Seconds | Exit |"
  echo "| :--- | ---: | ---: |"
  awk -F'\t' '{ print "| `" $1 "` | " $2 " | " $3 " |" }' "$out/steps.tsv"
} >> "${GITHUB_STEP_SUMMARY:-/dev/null}"
cat "$out/steps.tsv"
exit "$status"
