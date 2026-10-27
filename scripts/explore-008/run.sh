#!/usr/bin/env bash
# Exploration 008, on `explore/008-a-unit-served-on-arrival` alone (ADR-0193): a unit served on
# arrival, timed against the per-tick service. The arms job runs it for a test named
# `<arch>/<mode>/<test>`, `<arch>` choosing the runner there (x86-64, the weekly shards'
# `ubuntu-24.04`, or aarch64, the arms' `ubuntu-24.04-arm`), as exploration 006's run.sh did.
#
#   ab   builds the inhibition binary twice in the release profile, each in a target directory of
#        its own: A from $A_COMMIT, whose engine is main's and whose tests are this branch's
#        timed workload, and B from HEAD, whose engine serves units on arrival when
#        MENTEION_EXPLORE_008 names a mode. Then <test>, the 16-block workload, in three rounds
#        of A, B under `integrate` and B under `quiet`, each run in a process of its own:
#        A B1 B2 A B1 B2 A B1 B2, on one runner. Then the whole arm, <test> without
#        `_over_16_blocks`, once in each of the three, in the same order.
#
# Every run is timed by the clock around its process and by the test's own TIME lines, and
# prints its ARENA and COUNT lines. No run starts after 200 of the job's 240 minutes; one skipped
# so is written in runs.tsv as skipped.
#
# Writes explore-008/: machine.txt; builds.tsv (build, ms, exit); runs.tsv (round, build, test,
# ms, exit, tests passed); each build's log and each run's log. Exits non-zero if a build failed
# or a run failed, did not run or was skipped.

set -u -o pipefail

# Build A: b455b14, the branch's timed workload on main's engine.
A_COMMIT=${A_COMMIT:-b455b147959f5c295561f36486a946f1e3610f9a}

spec=${1:?usage: run.sh <arch>/<mode>/<test>}
arch=${spec%%/*}
rest=${spec#*/}
mode=${rest%%/*}
name=${rest#*/}
whole=${name/_over_16_blocks/}
out=$PWD/explore-008
mkdir -p "$out"
: > "$out/builds.tsv"
: > "$out/runs.tsv"
status=0
budget=$((200 * 60))

case "$arch/$(uname -m)" in
  x86-64/x86_64 | aarch64/aarch64) ;;
  *) echo "asked for $arch, on $(uname -m)"; exit 1 ;;
esac
if [ "$mode" != ab ]; then
  echo "unknown mode $mode"
  exit 1
fi

{
  echo "spec: $spec"
  echo "16 blocks: $name"
  echo "whole: $whole"
  echo "A: $A_COMMIT"
  echo "B: $(git rev-parse HEAD)"
  uname -a
  echo "nproc: $(nproc)"
  lscpu
  free -m
  rustc --version --verbose
  cargo --version
} > "$out/machine.txt" 2>&1
cat "$out/machine.txt"

git fetch --no-tags --depth=50 origin "$A_COMMIT" > "$out/fetch-a.log" 2>&1 || true
rm -rf ../explore-008-a
if ! git worktree add --detach ../explore-008-a "$A_COMMIT" > "$out/worktree-a.log" 2>&1; then
  echo "build A's commit is not here"
  cat "$out/worktree-a.log" "$out/fetch-a.log"
  exit 1
fi

# build <label> <directory> <target directory>: the inhibition binary in release, timed; sets
# `bin` to it.
build() {
  local label=$1 dir=$2 target=$3 t0 rc ms
  t0=$(date +%s%N)
  (cd "$dir" && CARGO_TARGET_DIR=$target cargo test --locked --release -p menteion-runtime \
    --test inhibition --no-run --message-format=json-render-diagnostics) \
    > "$out/build-$label.json" 2> "$out/build-$label.log"
  rc=$?
  ms=$((($(date +%s%N) - t0) / 1000000))
  printf '%s\t%s\t%s\n' "$label" "$ms" "$rc" >> "$out/builds.tsv"
  tail -n 3 "$out/build-$label.log"
  echo "== build $label: $ms ms, exit $rc"
  bin=$(jq -r 'select(.reason == "compiler-artifact" and .target.name == "inhibition" and .profile.test == true) | .executable // empty' \
    "$out/build-$label.json" | tail -n 1)
  if [ "$rc" -ne 0 ] || [ -z "$bin" ]; then
    echo "no inhibition binary for $label"
    status=1
    return 1
  fi
}

build A "$PWD/../explore-008-a" "$PWD/target-a" || exit 1
bin_a=$bin
build B "$PWD" "$PWD/target" || exit 1
bin_b=$bin

# run_one <round> <label> <test>: one run in a process of its own, timed. B1 is B under
# `integrate`, B2 under `quiet`, A with no mode, which its engine does not read.
run_one() {
  local round=$1 label=$2 test=$3 log t0 rc ms ran bin env_mode kind
  kind=16
  [ "$test" = "$whole" ] && kind=whole
  case "$label" in
    A) bin=$bin_a; env_mode= ;;
    B1) bin=$bin_b; env_mode=integrate ;;
    B2) bin=$bin_b; env_mode=quiet ;;
  esac
  log="$out/run-$round-$label-$kind.log"
  if [ "$SECONDS" -gt "$budget" ]; then
    echo "== run $round $label $kind: skipped, $SECONDS s of the job gone"
    printf '%s\t%s\t%s\t\tskipped\t0\n' "$round" "$label" "$kind" >> "$out/runs.tsv"
    status=1
    return
  fi
  t0=$(date +%s%N)
  if [ -n "$env_mode" ]; then
    MENTEION_EXPLORE_008=$env_mode "$bin" --ignored --exact --nocapture "$test" < /dev/null > "$log" 2>&1
  else
    env -u MENTEION_EXPLORE_008 "$bin" --ignored --exact --nocapture "$test" < /dev/null > "$log" 2>&1
  fi
  rc=$?
  ms=$((($(date +%s%N) - t0) / 1000000))
  ran=$(awk '/^test result:/ { print $4; exit }' "$log")
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$round" "$label" "$kind" "$ms" "$rc" "${ran:-0}" >> "$out/runs.tsv"
  grep -E '^DUMP .*(PIN control|held bit for bit)' "$log" | cut -c 1-240
  grep -E '^TIME (derivation|run|end) |^COUNT (turns|ticks|mode) ' "$log"
  tail -n 3 "$log"
  echo "== run $round $label $kind: $ms ms, exit $rc, ${ran:-0} passed"
  if [ "$rc" -ne 0 ] || [ "${ran:-0}" != "1" ]; then
    status=1
  fi
}

for round in 1 2 3; do
  for label in A B1 B2; do
    run_one "$round" "$label" "$name"
  done
done
for label in A B1 B2; do
  run_one 1 "$label" "$whole"
done

{
  echo "### Exploration 008: $spec, B at $(git rev-parse --short HEAD), A at ${A_COMMIT:0:7}"
  echo
  echo "| Build | ms | Exit |"
  echo "| :--- | ---: | ---: |"
  awk -F'\t' '{ print "| " $1 " | " $2 " | " $3 " |" }' "$out/builds.tsv"
  echo
  echo "| Round | Build | Test | ms | Exit | Passed |"
  echo "| ---: | :--- | :--- | ---: | ---: | ---: |"
  awk -F'\t' '{ print "| " $1 " | " $2 " | " $3 " | " $4 " | " $5 " | " $6 " |" }' "$out/runs.tsv"
} >> "${GITHUB_STEP_SUMMARY:-/dev/null}"

cat "$out/builds.tsv" "$out/runs.tsv"
exit "$status"
