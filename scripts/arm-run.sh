#!/usr/bin/env bash
# One `#[ignore]`d test, run by a dispatch at `scope=arms` (ADR-0184): `arm-run.sh <test>` runs
# the test `--ignored --list` names <test>, whole-domain (`_exhaustive`) or paused (`_paused`,
# ADR-0127), with its output kept, so that a round's tables are written from a run the
# repository's host recorded and not from one on the developer machine.
#
# The test is found as the weekly shards find theirs: from `--list` at run time, never from a
# path written here (ADR-0073, F-44, F-45). A name that no binary lists, or that two list, fails
# the job before anything runs, since `--exact` would then run nothing or run the wrong test.
#
# The test runs in a process of its own (`<binary> --ignored --exact --nocapture <test>`), as a
# shard runs it but with its output not captured: the dumps an arm prints are what the round's
# tables are written from, and a test prints its dump before it asserts, so a failing arm still
# leaves its dump. It must exit 0 and report one test passed; otherwise the job fails, and the
# log is kept either way.
#
# Writes `arm.log` (the test's whole output), `arm.tsv` (the binary's path and stem, the test,
# its seconds, its exit status, the tests it reported passing) and `list.log`. Exits non-zero if
# the test was not found, failed or did not run.

set -u -o pipefail

name=${1:?usage: arm-run.sh <test>}

echo "the test: $name"

if ! cargo test --workspace --release --locked -- --ignored --list 2>&1 | tee list.log; then
  echo "the listing failed, so the test cannot be found"
  exit 1
fi

# The binaries that list the test, from the `Running ... (<path>)` line above each binary's list.
paths=$(awk -v want="$name: test" '
  /^[[:space:]]*Running[[:space:]]/ { p = $0; sub(/.*\(/, "", p); sub(/\)[[:space:]]*$/, "", p); next }
  /^[[:space:]]*Doc-tests[[:space:]]/ { p = ""; next }
  { sub(/\r$/, "") }
  $0 == want && p != "" { print p }
' list.log)
found=$(printf '%s' "$paths" | grep -c . || true)
if [ "$found" -ne 1 ]; then
  echo "the listing names this test in $found binaries, where one is required"
  printf '%s\n' "$paths"
  exit 1
fi
path=$paths
stem=${path##*/}
stem=${stem%-*}
echo "in $stem ($path)"

started=$SECONDS
"$path" --ignored --exact --nocapture "$name" < /dev/null > arm.log 2>&1
rc=$?
secs=$((SECONDS - started))
ran=$(awk '/^test result:/ { print $4; exit }' arm.log)
printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$path" "$stem" "$name" "$secs" "$rc" "${ran:-0}" > arm.tsv

tail -n 40 arm.log
echo "the test took $secs s, exit $rc, ${ran:-0} passed"
if [ "$rc" -ne 0 ] || [ "${ran:-0}" != "1" ]; then
  echo "the test failed or did not run"
  exit 1
fi
