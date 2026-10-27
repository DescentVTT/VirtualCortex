// Exploration 008, on `explore/008-a-unit-served-on-arrival` alone (ADR-0193): reads one job's
// explore-008/ directory, as run.sh writes it, and prints the note's tables.
//
//   node scripts/explore-008/read.mjs <explore-008 directory>
//
// For each run: its derivation, its mean block over blocks 2 to the last, the whole arm they give
// (the derivation and 120 blocks at the mean block, exploration 006's estimate), the whole-arm
// runs' own seconds, and each build beside A in the same round. Then every run's ARENA lines held
// to the first A run's, block by block, and its DUMP pins; then B's counts: the units served a
// tick, the catch-up's steps a woken unit and a message, and the turns, served or caught up,
// beside A's.

import fs from 'node:fs';
import path from 'node:path';

const dir = process.argv[2];
if (!dir) {
  console.error('usage: read.mjs <explore-008 directory>');
  process.exit(1);
}

const runs = fs
  .readFileSync(path.join(dir, 'runs.tsv'), 'utf8')
  .trim()
  .split('\n')
  .filter(Boolean)
  .map((line) => {
    const [round, label, kind, ms, exit, passed] = line.split('\t');
    return { round: Number(round), label, kind, ms: Number(ms), exit, passed };
  });

function parseLog(run) {
  const file = path.join(dir, `run-${run.round}-${run.label}-${run.kind}.log`);
  if (!fs.existsSync(file)) return null;
  const text = fs.readFileSync(file, 'utf8');
  const lines = text.split('\n');
  const time = {};
  const blocks = [];
  const arena = [];
  const counts = {};
  const pins = [];
  for (const line of lines) {
    let m;
    if ((m = line.match(/^TIME (derivation|decoded|run|end) (\d+)/))) time[m[1]] = Number(m[2]);
    else if ((m = line.match(/^TIME block (\d+) (\d+)/))) blocks[Number(m[1])] = Number(m[2]);
    else if ((m = line.match(/^ARENA block (\d+) (0x[0-9a-f]+)/))) arena[Number(m[1])] = m[2];
    else if ((m = line.match(/^COUNT (turns|ticks) (\d+)/))) counts[m[1]] = BigInt(m[2]);
    else if ((m = line.match(/^COUNT mode (\w+)/))) counts.mode = m[1];
    else if ((m = line.match(/^COUNT (derivation|arm) ArrivalCounts \{(.*)\}/))) counts[m[1]] = parseCounts(m[2]);
    else if (/^DUMP .*(PIN|held bit for bit)/.test(line)) pins.push(line.replace(/^DUMP [^:]*?(PIN|held)/, '$1'));
  }
  return { time, blocks, arena, counts, pins, passed: /test result: ok\. 1 passed/.test(text) };
}

function parseCounts(body) {
  const out = {};
  for (const m of body.matchAll(/(\w+): (\[[^\]]*\]|\d+)/g)) {
    out[m[1]] = m[2].startsWith('[') ? JSON.parse(m[2]).map(BigInt) : BigInt(m[2]);
  }
  return out;
}

const logs = new Map(runs.map((r) => [r, parseLog(r)]));
const s = (ms) => (ms / 1000).toFixed(1);

function blockMean(log) {
  // Blocks 2 to the last: block 1 carries the lead-in.
  const deltas = [];
  for (let k = 2; k < log.blocks.length; k++) {
    if (log.blocks[k] !== undefined && log.blocks[k - 1] !== undefined) deltas.push(log.blocks[k] - log.blocks[k - 1]);
  }
  return deltas.reduce((a, b) => a + b, 0) / deltas.length;
}

console.log('## The 16-block runs\n');
console.log('| Round | Build | The run | Derivation | A block (2 to 16) | The whole arm (est.) | Beside A |');
console.log('| ---: | :--- | ---: | ---: | ---: | ---: | ---: |');
const sums = {};
for (const r of runs.filter((r) => r.kind === '16')) {
  const log = logs.get(r);
  if (!log || log.time.derivation === undefined) {
    console.log(`| ${r.round} | ${r.label} | ${r.exit} | | | | |`);
    continue;
  }
  const block = blockMean(log);
  const est = log.time.derivation + 120 * block;
  const a = runs.find((x) => x.kind === '16' && x.round === r.round && x.label === 'A');
  const aLog = a && logs.get(a);
  const aEst = aLog ? aLog.time.derivation + 120 * blockMean(aLog) : NaN;
  (sums[r.label] ??= []).push({ run: r.ms, derivation: log.time.derivation, block, est });
  console.log(
    `| ${r.round} | ${r.label} | ${s(r.ms)} s | ${s(log.time.derivation)} s | ${s(block)} s | ${s(est)} s | ${(est / aEst).toFixed(3)} |`,
  );
}
console.log('\n| Build | Derivation, mean | Block, mean | Whole arm (est.) | Beside A | Derivation beside A | Block beside A |');
console.log('| :--- | ---: | ---: | ---: | ---: | ---: | ---: |');
const mean = (xs, k) => xs.reduce((a, x) => a + x[k], 0) / xs.length;
for (const label of Object.keys(sums)) {
  const x = sums[label];
  const A = sums.A;
  console.log(
    `| ${label} | ${s(mean(x, 'derivation'))} s | ${s(mean(x, 'block'))} s | ${s(mean(x, 'est'))} s | ${(mean(x, 'est') / mean(A, 'est')).toFixed(3)} | ${(mean(x, 'derivation') / mean(A, 'derivation')).toFixed(3)} | ${(mean(x, 'block') / mean(A, 'block')).toFixed(3)} |`,
  );
}

console.log('\n## The whole arm, once each\n');
console.log('| Build | The run | Derivation | The blocks | Beside A |');
console.log('| :--- | ---: | ---: | ---: | ---: |');
const wholeA = runs.find((r) => r.kind === 'whole' && r.label === 'A');
for (const r of runs.filter((r) => r.kind === 'whole')) {
  const log = logs.get(r);
  if (!log || log.time.derivation === undefined) {
    console.log(`| ${r.label} | ${r.exit} | | | |`);
    continue;
  }
  console.log(
    `| ${r.label} | ${s(r.ms)} s | ${s(log.time.derivation)} s | ${s(log.time.run - log.time.derivation)} s | ${wholeA ? (r.ms / wholeA.ms).toFixed(3) : ''} |`,
  );
}

console.log('\n## Bit for bit\n');
for (const kind of ['16', 'whole']) {
  const ref = runs.find((r) => r.kind === kind && r.label === 'A');
  const refLog = ref && logs.get(ref);
  if (!refLog) continue;
  for (const r of runs.filter((r) => r.kind === kind)) {
    const log = logs.get(r);
    if (!log) continue;
    const n = Math.max(log.arena.length, refLog.arena.length) - 1;
    let same = 0;
    let first = null;
    for (let k = 1; k <= n; k++) {
      if (log.arena[k] !== undefined && log.arena[k] === refLog.arena[k]) same++;
      else if (first === null) first = k;
    }
    const pinsSame = JSON.stringify(log.pins) === JSON.stringify(refLog.pins);
    console.log(
      `- ${kind} round ${r.round} ${r.label}: passed ${log.passed}; ARENA ${same} of ${n} blocks as A's round 1${first === null ? '' : `, first differing block ${first}`}; DUMP pins ${pinsSame ? 'as A' : 'DIFFER'}; turns ${log.counts.turns ?? '?'}`,
    );
  }
}

console.log('\n## What B counted\n');
for (const r of runs.filter((r) => r.label !== 'A')) {
  const log = logs.get(r);
  const c = log?.counts.arm;
  if (!c) continue;
  const a = runs.find((x) => x.kind === r.kind && x.round === r.round && x.label === 'A');
  const aTurns = a && logs.get(a)?.counts.turns;
  const logical = c.served + c.caught_up + c.settled_steps;
  const ratio = (n, d) => (d === 0n ? 'n/a' : (Number(n) / Number(d)).toFixed(2));
  console.log(
    `- ${r.kind} round ${r.round} ${r.label} (${log.counts.mode}): ticks ${c.ticks}; served ${c.served} (${ratio(c.served, c.ticks)} a tick, ${c.served_min} to ${c.served_max}); ` +
      `slept ${c.slept}, woken ${c.woken} with ${c.woken_messages} messages after ${c.gap} ticks asleep; caught up ${c.caught_up} steps ` +
      `(${ratio(c.caught_up, c.woken)} a woken unit, ${ratio(c.caught_up, c.woken_messages)} a message); settles ${c.settles} of ${c.settled_units} units, ${c.settled_steps} steps; ` +
      `served or caught up ${logical}, A's turns ${aTurns ?? '?'}${aTurns !== undefined ? (logical === aTurns ? ' (equal)' : ' (DIFFER)') : ''}`,
  );
  console.log(`  steps a woken unit, by 0, 1, 2-3, 4-7, ...: ${c.steps_hist.join(' ')}`);
  console.log(`  ticks by units served, 32nds of the units: ${c.served_hist.join(' ')}`);
  const d = log.counts.derivation;
  if (d) {
    console.log(
      `  derivation: ticks ${d.ticks}, served ${d.served}, caught up ${d.caught_up}, settled ${d.settled_steps}, woken ${d.woken} with ${d.woken_messages} messages`,
    );
  }
}
