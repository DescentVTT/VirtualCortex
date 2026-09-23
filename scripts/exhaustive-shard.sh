#!/usr/bin/env bash
# One shard of the whole-domain (`exhaustive`) tests: `exhaustive-shard.sh <k> <n>` runs the
# tests the deal gives shard k of n (0-based).
#
# The tests are enumerated at run time from `--list`, never written down, because a list in
# this file and a list in the tree drift and that drift is F-44 and F-45. `--list` builds
# nothing it would not build anyway and runs no test.
#
# The deal is by cost (ADR-0092, amending ADR-0073 and ADR-0084): `scripts/exhaustive-costs.mjs
# plan` deals the listed tests longest first to the least-loaded shard, each test costed by
# `scripts/exhaustive-costs.tsv` — the seconds a weekly run measured it at — and a test the
# table does not know as a heavy one. The table decides the balance and never the coverage:
# which tests run is what `--list` named.
#
# Each test runs in a process of its own (`<binary> --ignored --exact <test>`), so each test's
# seconds are its own, which is what the table is made from. No whole-domain test shares a
# fixture with another, so a process per test costs a process start and nothing more. Half as
# many run at once as the runner has cores: the heavy tests run an executor of two or four worker
# threads each, and at one process a core the first dispatch of ADR-0092 read them three times
# slower apiece and the shards' test time a fifth higher than ADR-0091's model.
#
# Each test is checked: it must exit 0 and report one test passed. A test that ran nothing — a
# name `--exact` did not match — or failed, or never ran, fails the job the way ADR-0058's
# completeness check fails a partial sweep; a listing that fails or names nothing fails it too
# (F-48).
#
# Writes `exhaustive-tests-<k>.tsv` (the binary's path and stem, the test, its seconds, its exit
# status, the tests it reported passing), `exhaustive-times-<k>.txt` (per binary: its tests in
# this shard and the sum of their seconds, ADR-0071's table) and `exhaustive-wall-<k>.txt` (the
# shard's wall clock over its tests). Exits non-zero if any test failed or did not run.

set -u -o pipefail

k=${1:?usage: exhaustive-shard.sh <k> <n>}
n=${2:?usage: exhaustive-shard.sh <k> <n>}

echo "shard $k of $n"

if ! cargo test --workspace --release --locked -- --ignored exhaustive --list 2>&1 | tee list.log; then
  echo "the listing failed, so this shard cannot know its slice"
  exit 1
fi

if ! node scripts/exhaustive-costs.mjs plan list.log scripts/exhaustive-costs.tsv "$k" "$n" > mine.txt; then
  echo "the deal failed, so this shard cannot know its slice"
  exit 1
fi
count=$(wc -l < mine.txt | tr -d ' ')
echo "this shard takes $count test(s):"
cat mine.txt

# One test in a process of its own, its result a line of results-<k>.tsv. It always returns 0:
# the result is the line, and the check below reads it.
run_one() {
  local path name stem log started rc secs ran
  IFS=$'\t' read -r path name <<< "$1"
  stem=${path##*/}
  stem=${stem##*\\}
  stem=${stem%.exe}
  stem=${stem%-*}
  log="run-${stem}--${name//::/__}.log"
  started=$SECONDS
  "$path" --ignored --exact "$name" < /dev/null > "$log" 2>&1
  rc=$?
  secs=$((SECONDS - started))
  ran=$(awk '/^test result:/ { print $4; exit }' "$log")
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$path" "$stem" "$name" "$secs" "$rc" "${ran:-0}" >> "results-$SHARD.tsv"
  return 0
}
export -f run_one
export SHARD="$k"

cores=$(nproc 2>/dev/null || echo 2)
parallel=$((cores / 2))
if [ "$parallel" -lt 1 ]; then parallel=1; fi
: > "results-$k.tsv"
started=$SECONDS
# `-d '\n'` so that a line is one argument and xargs reads no quote or backslash in it.
xargs -a mine.txt -d '\n' -P "$parallel" -n 1 bash -c 'run_one "$0"'
elapsed=$((SECONDS - started))
echo "$elapsed" > "exhaustive-wall-$k.txt"

# Every test the deal gave this shard must have passed, and passed as one test.
status=0
if ! awk -F'\t' '
  NR == FNR { want[$1 "\t" $2] = 1; next }
  { got[$1 "\t" $3] = ($5 == 0 && $6 == 1) }
  END {
    bad = 0
    for (t in want) if (!(t in got) || !got[t]) { print "did not pass as one test: " t; bad = 1 }
    exit bad
  }
' mine.txt "results-$k.tsv"; then
  status=1
fi

cut -f1-6 "results-$k.tsv" | sort -t$'\t' -k4,4nr > "exhaustive-tests-$k.tsv"
awk -F'\t' '{ c[$2] += 1; s[$2] += $4 } END { for (b in c) printf "%s\t%d\t%d\n", b, c[b], s[b] }' \
  "exhaustive-tests-$k.tsv" | sort > "exhaustive-times-$k.txt"

echo "== this shard: $count test(s) in ${elapsed} s of wall clock, $parallel at a time"
cut -f2-6 "exhaustive-tests-$k.tsv"
exit "$status"
