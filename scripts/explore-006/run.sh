#!/usr/bin/env bash
# Exploration 006, on `explore/006-where-an-arm-spends-its-time` alone (ADR-0193): where a heavy
# arm spends its time, and what the whole-program profiles of Cargo.toml buy it. The arms job runs
# it for a test named `<arch>/<mode>/<test>`, `<arch>` choosing the runner there (x86-64, the
# weekly shards' `ubuntu-24.04`, or aarch64, the arms' `ubuntu-24.04-arm`):
#
#   perf       the inhibition binary built in the release profile with line tables
#              (CARGO_PROFILE_RELEASE_DEBUG=line-tables-only, which adds debug information and
#              leaves the code as it is), the test run once under `perf record` at 49 Hz on the
#              cpu-clock with DWARF call chains of 8 KB, then perf's own reports and
#              classify.mjs's split of every sample by the innermost frame of this repository's
#              code: the engine's crates or the test crate.
#   profiles   every test binary of the workspace built in each of the four profiles in turn,
#              each build timed and its cargo report kept, then the test run in each profile in
#              turn, release, lto-fat, lto-thin, cgu1, three rounds: A B C D A B C D A B C D, on
#              one runner, each run in a process of its own.
#   <profile>  (release, lto-fat, lto-thin or cgu1) the inhibition binary alone built in that
#              profile and timed, then the test run once.
#
# Every run is timed by the clock around its process and by the test's own TIME lines. No run
# starts after 200 of the job's 240 minutes; one skipped so is written in runs.tsv as skipped.
#
# Writes explore-006/: machine.txt; builds.tsv (profile, what was built, ms, exit) and sizes.tsv;
# runs.tsv (round, profile, ms, exit, tests passed); times.tsv (round, profile, the TIME line's
# phase, ms); each build's log and cargo report, each run's log; and in `perf` perf's reports and
# classify.mjs's tables. Exits non-zero if a build failed or a run failed, did not run or was
# skipped.

set -u -o pipefail

spec=${1:?usage: run.sh <arch>/<mode>/<test>}
arch=${spec%%/*}
rest=${spec#*/}
mode=${rest%%/*}
name=${rest#*/}
out=explore-006
mkdir -p "$out"
: > "$out/builds.tsv"
: > "$out/sizes.tsv"
: > "$out/runs.tsv"
: > "$out/times.tsv"
status=0
budget=$((200 * 60))
profiles=(release lto-fat lto-thin cgu1)

case "$arch/$(uname -m)" in
  x86-64/x86_64 | aarch64/aarch64) ;;
  *) echo "asked for $arch, on $(uname -m)"; exit 1 ;;
esac

{
  echo "spec: $spec"
  echo "mode: $mode"
  echo "test: $name"
  echo "commit: $(git rev-parse HEAD)"
  uname -a
  echo "nproc: $(nproc)"
  lscpu
  free -m
  rustc --version --verbose
  cargo --version
} > "$out/machine.txt" 2>&1
cat "$out/machine.txt"

cargo fetch --locked > "$out/fetch.log" 2>&1 || { echo "cargo fetch failed"; tail "$out/fetch.log"; exit 1; }

# build <profile> <workspace|inhibition>: times the build of every test binary of the workspace or
# of the inhibition binary alone, keeps cargo's report, and sets `bin` to the inhibition binary.
build() {
  local profile=$1 scope=$2 args t0 rc ms
  case "$scope" in
    workspace) args=(--workspace) ;;
    inhibition) args=(-p menteion-runtime --test inhibition) ;;
  esac
  rm -rf target/cargo-timings
  t0=$(date +%s%N)
  cargo test --locked --profile "$profile" "${args[@]}" --no-run --timings \
    --message-format=json-render-diagnostics \
    > "$out/build-$profile-$scope.json" 2> "$out/build-$profile-$scope.log"
  rc=$?
  ms=$((($(date +%s%N) - t0) / 1000000))
  printf '%s\t%s\t%s\t%s\n' "$profile" "$scope" "$ms" "$rc" >> "$out/builds.tsv"
  if [ -f target/cargo-timings/cargo-timing.html ]; then
    cp target/cargo-timings/cargo-timing.html "$out/timings-$profile-$scope.html"
  fi
  tail -n 3 "$out/build-$profile-$scope.log"
  echo "== build $profile $scope: $ms ms, exit $rc"
  bin=$(jq -r 'select(.reason == "compiler-artifact" and .target.name == "inhibition" and .profile.test == true) | .executable // empty' \
    "$out/build-$profile-$scope.json" | tail -n 1)
  if [ "$rc" -ne 0 ] || [ -z "$bin" ]; then
    echo "no inhibition binary in $profile"
    status=1
    return 1
  fi
  printf '%s\t%s\t%s\n' "$profile" "$scope" "$(stat -c %s "$bin")" >> "$out/sizes.tsv"
}

# run_one <round> <label> <command...>: runs the test once in a process of its own, timed.
run_one() {
  local round=$1 label=$2 log t0 rc ms ran
  shift 2
  log="$out/run-$round-$label.log"
  if [ "$SECONDS" -gt "$budget" ]; then
    echo "== run $round $label: skipped, $SECONDS s of the job gone"
    printf '%s\t%s\t\tskipped\t0\n' "$round" "$label" >> "$out/runs.tsv"
    status=1
    return
  fi
  t0=$(date +%s%N)
  "$@" --ignored --exact --nocapture "$name" < /dev/null > "$log" 2>&1
  rc=$?
  ms=$((($(date +%s%N) - t0) / 1000000))
  ran=$(awk '/^test result:/ { print $4; exit }' "$log")
  printf '%s\t%s\t%s\t%s\t%s\n' "$round" "$label" "$ms" "$rc" "${ran:-0}" >> "$out/runs.tsv"
  awk -v r="$round" -v p="$label" '
    /^TIME / { if ($2 == "block") print r "\t" p "\tblock " $3 "\t" $4; else print r "\t" p "\t" $2 "\t" $3 }
  ' "$log" >> "$out/times.tsv"
  grep -E '^DUMP .*(PIN control|held bit for bit)' "$log" | cut -c 1-240
  grep -E '^TIME (derivation|run|end) ' "$log"
  tail -n 4 "$log"
  echo "== run $round $label: $ms ms, exit $rc, ${ran:-0} passed"
  if [ "$rc" -ne 0 ] || [ "${ran:-0}" != "1" ]; then
    status=1
  fi
}

case "$mode" in
  perf)
    sudo apt-get update -qq > "$out/apt.log" 2>&1
    sudo apt-get install -y -qq linux-tools-common "linux-tools-$(uname -r)" >> "$out/apt.log" 2>&1 \
      || sudo apt-get install -y -qq linux-tools-common linux-tools-azure >> "$out/apt.log" 2>&1 \
      || sudo apt-get install -y -qq linux-tools-common linux-tools-generic >> "$out/apt.log" 2>&1
    perf=/usr/lib/linux-tools/$(uname -r)/perf
    if [ ! -x "$perf" ]; then
      perf=$(ls /usr/lib/linux-tools/*/perf /usr/lib/linux-tools-*/perf 2> /dev/null | sort -V | tail -n 1)
    fi
    if [ -z "$perf" ] || [ ! -x "$perf" ]; then
      echo "no perf binary"
      tail -n 20 "$out/apt.log"
      exit 1
    fi
    sudo sysctl -w kernel.perf_event_paranoid=-1 kernel.kptr_restrict=0
    {
      echo "perf: $perf"
      "$perf" version --build-options
      echo "--- hardware counters, probed"
      "$perf" stat -e cycles,instructions,task-clock -- true
    } > "$out/perf-version.txt" 2>&1
    cat "$out/perf-version.txt"
    export CARGO_PROFILE_RELEASE_DEBUG=line-tables-only CARGO_PROFILE_RELEASE_STRIP=none
    build release inhibition || exit 1
    run_one 1 perf "$perf" record -q -e cpu-clock -F 49 --call-graph dwarf,8192 -o perf.data -- "$bin"
    ls -l perf.data
    started=$SECONDS
    "$perf" report -i perf.data --no-children --sort dso,sym --stdio -g none --percent-limit 0.1 \
      > "$out/report-self.txt" 2> "$out/report-self.err"
    "$perf" report -i perf.data --children --sort sym --stdio -g none --percent-limit 0.5 \
      > "$out/report-children.txt" 2> "$out/report-children.err"
    "$perf" report -i perf.data --no-children --sort comm,tid --stdio -g none \
      > "$out/report-threads.txt" 2> "$out/report-threads.err"
    echo "perf's three reports: $((SECONDS - started)) s"
    started=$SECONDS
    "$perf" script -i perf.data -F comm,tid,time,ip,sym,dso 2> "$out/script.err" \
      | node scripts/explore-006/classify.mjs "$out" || status=1
    echo "perf script and classify.mjs: $((SECONDS - started)) s"
    started=$SECONDS
    timeout 2400 "$perf" report -i perf.data --no-children --sort srcfile --stdio -g none --percent-limit 0.1 \
      > "$out/report-srcfile.txt" 2> "$out/report-srcfile.err"
    rc=$?
    echo "perf's report by source file: $((SECONDS - started)) s, exit $rc"
    head -n 60 "$out/report-self.txt"
    cat "$out/classify-summary.txt"
    ;;
  profiles)
    declare -A bins
    for p in "${profiles[@]}"; do
      build "$p" workspace && bins[$p]=$bin
    done
    for round in 1 2 3; do
      for p in "${profiles[@]}"; do
        if [ -n "${bins[$p]:-}" ]; then
          run_one "$round" "$p" "${bins[$p]}"
        fi
      done
    done
    ;;
  release | lto-fat | lto-thin | cgu1)
    build "$mode" inhibition || exit 1
    run_one 1 "$mode" "$bin"
    ;;
  *)
    echo "unknown mode $mode"
    exit 1
    ;;
esac

{
  echo "### Exploration 006: $spec, at $(git rev-parse --short HEAD)"
  echo
  echo "| Profile | Built | ms | Exit |"
  echo "| :--- | :--- | ---: | ---: |"
  awk -F'\t' '{ print "| " $1 " | " $2 " | " $3 " | " $4 " |" }' "$out/builds.tsv"
  echo
  echo "| Round | Run | ms | Exit | Passed |"
  echo "| ---: | :--- | ---: | ---: | ---: |"
  awk -F'\t' '{ print "| " $1 " | " $2 " | " $3 " | " $4 " | " $5 " |" }' "$out/runs.tsv"
} >> "${GITHUB_STEP_SUMMARY:-/dev/null}"

cat "$out/builds.tsv" "$out/runs.tsv"
exit "$status"
