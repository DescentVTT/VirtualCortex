#!/usr/bin/env bash
# One shard of the whole-domain (`exhaustive`) tests: `exhaustive-shard.sh <k> <n>` runs every
# n-th test of the sorted list, starting at k (0-based). The unit is a test, not a binary
# (ADR-0084, amending ADR-0073): a binary's tests are spread over the shards, so no single file
# is a shard's floor.
#
# The tests are enumerated at run time from `--list`, never written down, because a list in
# this file and a list in the tree drift and that drift is F-44 and F-45. `--list` builds
# nothing it would not build anyway and runs no test.
#
# A binary's share of this shard runs in one process, its tests named as exact filters
# (`<binary> --ignored --exact <test> <test> …`), so libtest runs them side by side as it runs a
# whole binary, and no test pays a process of its own.
#
# Each binary is checked against the names this shard gave it: a binary that reports fewer tests
# than it was given is a shard that did not run its slice, and fails the job the way ADR-0058's
# completeness check fails a partial sweep. The union of the shards is the whole list by
# construction of the round robin.
#
# Writes `exhaustive-times-<k>.txt`: the binary, the tests of it this shard ran and the seconds
# they took (ADR-0071). Exits non-zero if a test failed or a binary ran less than its share.

set -u -o pipefail

k=${1:?usage: exhaustive-shard.sh <k> <n>}
n=${2:?usage: exhaustive-shard.sh <k> <n>}

echo "shard $k of $n"

# Every test `--list` names, as "<binary path>\t<test name>". A "Running … (<path>)" line opens
# a binary and each "<name>: test" line under it is one of its tests; a "Doc-tests" line opens a
# section that no binary of this script runs, so its lines are dropped. The byte order of the
# sort is the tree's, whatever the runner's locale.
#
# A listing that fails — a tree that does not build — or that names no test at all is a shard
# that ran nothing, and it fails the job rather than pass it: before ADR-0084 an empty list
# was a shard of no binaries, which exited 0 (F-48).
if ! cargo test --workspace --release --locked -- --ignored exhaustive --list 2>&1 | tee list.log; then
  echo "the listing failed, so this shard cannot know its slice"
  exit 1
fi
awk '
  /^[[:space:]]*Running/ { path = $0; sub(/^[^(]*\(/, "", path); sub(/\).*$/, "", path); next }
  /^[[:space:]]*Doc-tests/ { path = ""; next }
  path != "" && /: test$/ { name = $0; sub(/: test$/, "", name); printf "%s\t%s\n", path, name }
' list.log | LC_ALL=C sort > all.txt

echo "the tree holds $(wc -l < all.txt) exhaustive test(s) in $(cut -f1 all.txt | uniq | wc -l) binary/binaries"
if [ ! -s all.txt ]; then
  echo "the listing named no exhaustive test, where the tree holds them: nothing to shard"
  exit 1
fi

awk -v k="$k" -v n="$n" 'NR % n == k' all.txt > mine.txt
echo "this shard takes $(wc -l < mine.txt) of them:"
cat mine.txt

status=0
: > "exhaustive-times-$k.txt"
cut -f1 mine.txt | uniq > binaries.txt
while IFS= read -r path; do
  # This binary's share, compared in the shell rather than in awk, whose `-v` would read a
  # backslash in a path as an escape.
  names=()
  while IFS=$'\t' read -r p t; do
    if [ "$p" = "$path" ]; then names+=("$t"); fi
  done < mine.txt
  count=${#names[@]}
  name=${path##*/}
  name=${name##*\\}
  name=${name%.exe}
  name=${name%-*}
  started=$SECONDS
  "$path" --ignored --exact "${names[@]}" < /dev/null 2>&1 | tee "run-$name.log" || status=1
  elapsed=$((SECONDS - started))
  # What the binary itself reported having run, against the names this shard gave it.
  ran=$(awk '/^test result:/ { print $4; exit }' "run-$name.log")
  ran=${ran:-0}
  printf '%s\t%s\t%s\n' "$name" "$ran" "$elapsed" >> "exhaustive-times-$k.txt"
  if [ "$ran" -ne "$count" ]; then
    echo "$name ran $ran of the $count this shard gave it: this shard did not run its slice"
    status=1
  fi
done < binaries.txt

echo "== this shard"
cat "exhaustive-times-$k.txt"
exit "$status"
