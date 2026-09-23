// The whole-domain (`exhaustive`) tests dealt to the weekly shards by their cost (ADR-0092).
//
//   node scripts/exhaustive-costs.mjs plan <list.log> <costs.tsv> <k> <n>
//       Prints shard k's tests, one `<binary path>\t<test name>` to a line, the tests of
//       `--list` dealt longest first to the least-loaded of n shards. The planned load of every
//       shard goes to stderr. Exits 1 when the listing names no test.
//   node scripts/exhaustive-costs.mjs check [costs.tsv]
//       Fails on a line of the cost table that names no test function in the tree, or on a
//       malformed or repeated line (`npm run spec:costs`).
//   node scripts/exhaustive-costs.mjs from <dir> [--run <id>]
//       Prints a cost table from a weekly run's `exhaustive-tests-*.tsv` artifacts under <dir>.
//
// Which tests run is always what `--list` names (F-44, F-45, ADR-0073): the table decides the
// balance and never the coverage, so a stale or missing line costs a shard's margin and not a
// test. A test with no line is costed at DEFAULT_COST, which assumes a new test is a heavy one,
// because every round that has added a whole-domain test since ADR-0077 added one of five to
// twenty minutes.

import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

/** The seconds a test with no line in the table is assumed to take. */
export const DEFAULT_COST = 900;

/** A test binary's stem: its file name without the directory, `.exe` or cargo's hash. */
export function stemOf(path) {
  const base = path.split(/[\\/]/).pop() ?? path;
  return base.replace(/\.exe$/, "").replace(/-[0-9a-f]+$/, "");
}

/**
 * Every test `--list` names, as `{ path, stem, name }`. A `Running … (<path>)` line opens a
 * binary and each `<name>: test` line under it is one of its tests; a `Doc-tests` line opens a
 * section no binary of the shard runs, whose lines are dropped.
 */
export function parseList(text) {
  const tests = [];
  let path = null;
  for (const line of text.split(/\r?\n/)) {
    const running = /^\s*Running\b.*\(([^)]*)\)\s*$/.exec(line);
    if (running) {
      path = running[1];
      continue;
    }
    if (/^\s*Doc-tests\b/.test(line)) {
      path = null;
      continue;
    }
    const t = /^(\S.*): test$/.exec(line);
    if (t && path !== null) tests.push({ path, stem: stemOf(path), name: t[1] });
  }
  return tests;
}

const key = (stem, name) => `${stem}\t${name}`;

/**
 * The cost table: `<stem>\t<name>\t<seconds>` a line, `#` lines and blank lines ignored.
 * Returns `{ costs: Map<key, seconds>, lines: Map<key, line number> }`; throws on a malformed
 * or repeated line, naming it.
 */
export function parseCosts(text, file = "scripts/exhaustive-costs.tsv") {
  const costs = new Map();
  const lines = new Map();
  text.split(/\r?\n/).forEach((line, i) => {
    if (line.trim() === "" || line.startsWith("#")) return;
    const cells = line.split("\t");
    const seconds = Number(cells[2]);
    if (cells.length !== 3 || cells[0] === "" || cells[1] === "" || !Number.isInteger(seconds) || seconds < 0) {
      throw new Error(`${file}:${i + 1}: expected '<stem>\\t<test>\\t<whole seconds>', read '${line}'`);
    }
    const k = key(cells[0], cells[1]);
    if (costs.has(k)) throw new Error(`${file}:${i + 1}: ${cells[0]} ${cells[1]} is listed twice`);
    costs.set(k, seconds);
    lines.set(k, i + 1);
  });
  return { costs, lines };
}

const byteOrder = (a, b) => (a < b ? -1 : a > b ? 1 : 0);

/**
 * The tests dealt to n shards longest first, each to the shard with the least load so far, the
 * lowest index on a tie; the order of equal costs is the binary's stem, then the test's name, in
 * byte order, so every shard computes the same deal from the same listing and table. Returns
 * `{ shards: [{ tests, load }] }`, each shard's tests in the listing's order.
 */
export function plan(tests, costs, n, fallback = DEFAULT_COST) {
  if (!Number.isInteger(n) || n < 1) throw new Error(`the shard count must be a positive integer, read ${n}`);
  const costed = tests.map((t) => ({ ...t, cost: costs.get(key(t.stem, t.name)) ?? fallback }));
  costed.sort((a, b) => b.cost - a.cost || byteOrder(a.stem, b.stem) || byteOrder(a.name, b.name));
  const shards = Array.from({ length: n }, () => ({ tests: [], load: 0 }));
  for (const t of costed) {
    let best = 0;
    for (let s = 1; s < n; s += 1) if (shards[s].load < shards[best].load) best = s;
    shards[best].tests.push(t);
    shards[best].load += t.cost;
  }
  for (const s of shards) s.tests.sort((a, b) => byteOrder(a.stem, b.stem) || byteOrder(a.name, b.name));
  return { shards };
}

/** The `.rs` files under the given directories, `target` and `node_modules` skipped. */
function rustFiles(dirs) {
  const out = [];
  const walk = (dir) => {
    for (const entry of readdirSync(dir)) {
      if (entry === "target" || entry === "node_modules" || entry.startsWith(".")) continue;
      const p = join(dir, entry);
      if (statSync(p).isDirectory()) walk(p);
      else if (p.endsWith(".rs")) out.push(p);
    }
  };
  for (const d of dirs) walk(d);
  return out;
}

/**
 * The table's lines that name no test function in the given sources: a test's function is the
 * last `::` segment of its name, and it must appear as `fn <segment>(` in some source.
 */
export function stale(costsText, sources, file = "scripts/exhaustive-costs.tsv") {
  const { costs, lines } = parseCosts(costsText, file);
  const problems = [];
  for (const k of costs.keys()) {
    const [stem, name] = k.split("\t");
    const leaf = name.split("::").pop();
    const fn = new RegExp(`\\bfn\\s+${leaf.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\s*[(<]`);
    if (!sources.some((src) => fn.test(src))) {
      problems.push(`${file}:${lines.get(k)}: ${stem} ${name} names no test function in the tree`);
    }
  }
  return problems;
}

/** A cost table from a weekly run's `exhaustive-tests-*.tsv` files, the passing tests only. */
export function fromArtifacts(texts, run) {
  const seconds = new Map();
  for (const text of texts) {
    for (const line of text.split(/\r?\n/)) {
      if (line.trim() === "") continue;
      const [, stem, name, secs, rc, ran] = line.split("\t");
      if (rc !== "0" || ran !== "1") continue;
      const k = key(stem, name);
      seconds.set(k, Math.max(seconds.get(k) ?? 0, Number(secs)));
    }
  }
  const rows = [...seconds.entries()].sort((a, b) => byteOrder(a[0], b[0]));
  const header = [
    "# The seconds each whole-domain (`exhaustive`) test took on the hosted runner, one test to a",
    "# line: the binary's stem, the test's name as `--list` prints it, the seconds. Read by",
    "# `scripts/exhaustive-shard.sh` to deal the tests to the weekly shards longest first",
    "# (ADR-0092). Advisory: which tests run is always what `--list` names, so a stale or missing",
    `# line changes the balance and never the coverage; a test with no line is costed at ${DEFAULT_COST} s.`,
    "# `npm run spec:costs` fails on a line that names no test in the tree. Regenerate from a",
    "# weekly run's `exhaustive-tests-*` artifacts: `node scripts/exhaustive-costs.mjs from <dir> --run <id>`.",
    `# Source: ${run ? `run ${run}` : "not recorded"}.`,
  ];
  return [...header, ...rows.map(([k, s]) => `${k}\t${s}`)].join("\n") + "\n";
}

function main(argv) {
  const [command, ...args] = argv;
  if (command === "plan") {
    const [listFile, costsFile, k, n] = args;
    const tests = parseList(readFileSync(listFile, "utf8"));
    if (tests.length === 0) {
      console.error("the listing named no exhaustive test, where the tree holds them: nothing to shard");
      return 1;
    }
    const { costs } = parseCosts(readFileSync(costsFile, "utf8"), costsFile);
    const { shards } = plan(tests, costs, Number(n));
    const mine = shards[Number(k)];
    if (mine === undefined) {
      console.error(`shard ${k} of ${n} does not exist`);
      return 1;
    }
    const known = tests.filter((t) => costs.has(key(t.stem, t.name))).length;
    console.error(`the tree holds ${tests.length} exhaustive test(s); the table costs ${known}, the rest at ${DEFAULT_COST} s`);
    shards.forEach((s, i) => console.error(`  shard ${i}: ${s.tests.length} test(s), planned ${s.load} s`));
    for (const t of mine.tests) console.log(`${t.path}\t${t.name}`);
    return 0;
  }
  if (command === "check") {
    const file = args[0] ?? "scripts/exhaustive-costs.tsv";
    const sources = rustFiles(["runtime", "crates"]).map((p) => readFileSync(p, "utf8"));
    let problems;
    try {
      problems = stale(readFileSync(file, "utf8"), sources, file);
    } catch (e) {
      problems = [e.message];
    }
    for (const p of problems) console.error(p);
    if (problems.length > 0) {
      console.error(`check-costs: ${problems.length} problem(s)`);
      return 1;
    }
    const { costs } = parseCosts(readFileSync(file, "utf8"), file);
    console.log(`check-costs: ${costs.size} line(s), each naming a test in the tree`);
    return 0;
  }
  if (command === "from") {
    const dir = args[0];
    const at = args.indexOf("--run");
    const run = at < 0 ? null : args[at + 1];
    const texts = [];
    const walk = (d) => {
      for (const entry of readdirSync(d)) {
        const p = join(d, entry);
        if (statSync(p).isDirectory()) walk(p);
        else if (/^exhaustive-tests-\d+\.tsv$/.test(entry)) texts.push(readFileSync(p, "utf8"));
      }
    };
    walk(dir);
    if (texts.length === 0) {
      console.error(`no exhaustive-tests-*.tsv under ${dir}`);
      return 1;
    }
    process.stdout.write(fromArtifacts(texts, run));
    return 0;
  }
  console.error("usage: exhaustive-costs.mjs plan <list.log> <costs.tsv> <k> <n> | check [costs.tsv] | from <dir> [--run <id>]");
  return 2;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) process.exitCode = main(process.argv.slice(2));
