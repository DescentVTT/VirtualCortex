#!/usr/bin/env bash
# One shard of the whole-domain (`exhaustive`) tests: `exhaustive-shard.sh <k> <n>` runs every
# n-th test binary that has one, starting at k (0-based).
#
# The binaries are enumerated at run time from `--list`, never written down, because a list in
# this file and a list in the tree drift and that drift is F-44 and F-45. `--list` builds
# nothing it would not build anyway and runs no test.
#
# Each binary is checked against what `--list` said it holds: a binary that reports fewer tests
# than the list counted is a shard that did not run its slice, and fails the job the way
# ADR-0058's completeness check fails a partial sweep. The union of the shards is the whole set
# by construction of the round robin.
#
# Writes `exhaustive-times-<k>.txt`: the binary, the tests it ran and the seconds it took
# (ADR-0071). Exits non-zero if a test failed or a binary ran less than its slice.

set -u -o pipefail

k=${1:?usage: exhaustive-shard.sh <k> <n>}
n=${2:?usage: exhaustive-shard.sh <k> <n>}

echo "shard $k of $n"

# The binaries and how many `exhaustive` tests each holds. `--list` prints a "Running … (<path>)"
# line per binary and an "N tests, M benchmarks" line after it; the binaries that hold none are
# left out here so that a shard's slice is of the binaries that cost something.
cargo test --workspace --release --locked -- --ignored exhaustive --list 2>&1 | tee list.log
awk '
  /^[[:space:]]*Running/ { path = $0; sub(/^[^(]*\(/, "", path); sub(/\).*$/, "", path); next }
  /^[0-9]+ tests?,/ { if ($1 + 0 > 0) printf "%s\t%s\n", path, $1 }
' list.log | sort > all.txt

total=$(awk -F'\t' '{ s += $2 } END { print s + 0 }' all.txt)
echo "the tree holds $total exhaustive test(s) in $(wc -l < all.txt) binary/binaries"

awk -v k="$k" -v n="$n" 'NR % n == k' all.txt > mine.txt
echo "this shard takes $(wc -l < mine.txt) of them:"
cut -f1 mine.txt

status=0
: > "exhaustive-times-$k.txt"
while IFS=$'\t' read -r path count; do
  name=$(basename "$path")
  name=${name%.exe}
  name=${name%-*}
  started=$SECONDS
  "$path" --ignored exhaustive 2>&1 | tee "run-$name.log" || status=1
  elapsed=$((SECONDS - started))
  # What the binary itself reported having run, against what the list said it holds.
  ran=$(awk '/^test result:/ { print $4; exit }' "run-$name.log")
  ran=${ran:-0}
  printf '%s\t%s\t%s\n' "$name" "$ran" "$elapsed" >> "exhaustive-times-$k.txt"
  if [ "$ran" -ne "$count" ]; then
    echo "$name ran $ran of the $count the list counted: this shard did not run its slice"
    status=1
  fi
done < mine.txt

echo "== this shard"
cat "exhaustive-times-$k.txt"
exit "$status"
