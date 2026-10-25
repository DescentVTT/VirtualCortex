#!/usr/bin/env bash
# Exploration 002, on `explore/002-what-the-pinned-tables-cost` alone: what the test crates cost to
# build on the hosted runner (ADR-0193; nothing here is in `main`'s workflow).
#
# `build-cost.sh <debug|release> [<commit> ...]` builds every test binary of the workspace from a
# clean target with `cargo test --workspace --no-run --timings` (and `--release` for release), then
# changes one file under runtime/cortex-runtime/src/ as cargo-mutants changes it for one mutant and
# builds again with the sweep's own command, `cargo test --no-run -p cortex-runtime`: the build a
# mutant of the runtime pays. The job runs it with CARGO_INCREMENTAL=0, as rust-cache sets it for
# every other job of this workflow, the sweep's among them. In debug it also times a cold
# `cargo check --workspace --all-targets`, the gate's Check and Clippy steps' front end.
#
# With commits named, each is checked out in turn on the same runner, in two rounds, the second in
# the reverse order and with a different mutant, every build from `cargo clean`: one runner's speed
# is then the same for every commit, where two runners of the same image differed by a quarter
# (run 40192286352). With none, the checked-out tree is built, twice.
#
# Where a tree has a test `the_moved_tables_parse_in_<binary>_explore` (the placements tried by the
# exploration), it is run once after the first cold build, in the same profile: what reading the
# tables at run time costs a test binary. So is `the_moved_tables_read_as_written_in_<binary>_explore`,
# where the tables are both in the source and in the data file: each read from the file is its
# const.
#
# Writes `build-cost/`: `<commit>-<step>-<round>.log` and `.html` (cargo's timing report),
# `steps.tsv` (commit, step, round, seconds, exit status, in the order run) and
# `sizes-<commit>.tsv` (each test binary's bytes).

set -u -o pipefail

profile=${1:?usage: build-cost.sh <debug|release> [<commit> ...]}
shift
case "$profile" in
  debug) flag=() ;;
  release) flag=(--release) ;;
  *) echo "usage: build-cost.sh <debug|release> [<commit> ...]"; exit 2 ;;
esac
commits=("$@")
checkout=1
if [ "${#commits[@]}" -eq 0 ]; then
  commits=("$(git rev-parse HEAD)")
  checkout=0
fi

out=build-cost
mkdir -p "$out"
: > "$out/steps.tsv"
status=0

step() {
  local commit=$1 label=$2 round=$3
  shift 3
  local name="$commit-$label-$round"
  rm -rf target/cargo-timings
  local started=$SECONDS
  "$@" > "$out/$name.log" 2>&1
  local rc=$?
  local secs=$((SECONDS - started))
  printf '%s\t%s\t%s\t%s\t%s\n' "$commit" "$label" "$round" "$secs" "$rc" >> "$out/steps.tsv"
  if [ -f target/cargo-timings/cargo-timing.html ]; then
    cp target/cargo-timings/cargo-timing.html "$out/$name.html"
  fi
  tail -n 3 "$out/$name.log"
  echo "== $name: ${secs} s, exit $rc"
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
  order=("${commits[@]}")
  if [ "$round" = 2 ]; then
    order=()
    for ((i = ${#commits[@]} - 1; i >= 0; i--)); do order+=("${commits[$i]}"); done
  fi
  for commit in "${order[@]}"; do
    if [ "$checkout" = 1 ]; then
      git checkout -q --detach "$commit" || { echo "cannot check out $commit"; exit 1; }
    fi
    c=$(git rev-parse --short=7 HEAD)
    echo "== round $round, $c"
    cargo clean
    if [ "$profile" = debug ]; then
      step "$c" check-cold "$round" cargo check --workspace --all-targets --locked --timings
    fi
    step "$c" test-cold "$round" cargo test --workspace --no-run --locked --timings "${flag[@]}"
    if [ "$round" = 1 ]; then
      find target/"$profile"/deps -maxdepth 1 -type f -executable -printf '%f\t%s\n' | sort > "$out/sizes-$c.tsv"
      if grep -q 'the_moved_tables_parse_in_' runtime/cortex-runtime/tests/*.rs; then
        step "$c" parse "$round" cargo test --workspace --locked "${flag[@]}" -- --ignored the_moved_tables_parse_in_ --nocapture
      fi
      if grep -q 'the_moved_tables_read_as_written_in_' runtime/cortex-runtime/tests/*.rs; then
        step "$c" read-as-written "$round" cargo test --workspace --locked "${flag[@]}" -- --ignored the_moved_tables_read_as_written_in_ --nocapture
      fi
    fi
    mutate "$round"
    step "$c" test-mutant "$round" cargo test --no-run -p cortex-runtime --locked --timings "${flag[@]}"
    git checkout -- runtime/cortex-runtime/src
  done
done

{
  echo "### Build cost, $profile (exploration 002)"
  echo
  echo "| Commit | Step | Round | Seconds | Exit |"
  echo "| :--- | :--- | ---: | ---: | ---: |"
  awk -F'\t' '{ print "| `" $1 "` | `" $2 "` | " $3 " | " $4 " | " $5 " |" }' "$out/steps.tsv"
} >> "${GITHUB_STEP_SUMMARY:-/dev/null}"
cat "$out/steps.tsv"
exit "$status"
