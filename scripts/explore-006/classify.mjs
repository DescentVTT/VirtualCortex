// Exploration 006, on its branch alone (ADR-0193): every sample of `perf script -F
// comm,tid,time,ip,sym,dso` (DWARF call chains, leaf first) read from stdin, split by whose code it
// was in. A sample's owner is the innermost frame of this repository's code: an engine crate
// (`menteion_*`, the runtime and the state crates) or the test crate (`inhibition`, its harness
// module included). A sample whose leaf is in std, alloc, libc or the kernel is its owner's, which
// called it. A sample in `SpinBarrier::wait` is the engine's, waiting. A sample with no frame of
// this repository's code is `outside` (libtest, the loader, a chain cut short).
//
// The thread is the sample's role: `worker`, a spawned worker of the executor (comm
// `menteion-worker`); `test`, the thread that runs the test and is the executor's worker 0 and its
// coordinator; `other`, any other.
//
// node classify.mjs <out dir> < perf-script output
// Writes <out>/classify-summary.txt, classify-owners.tsv (role, class, owner, samples, its own
// frame's, its callees' top leaf), classify-modules.tsv (role, class, module, samples),
// classify-inclusive.tsv (role, function of this repository, samples with it anywhere in the
// chain) and folded-<role>.txt (root first, `;`-joined, a count: a flame graph's input).
import fs from "node:fs";
import path from "node:path";
import readline from "node:readline";

const out = process.argv[2] ?? ".";
fs.mkdirSync(out, { recursive: true });

const clean = (sym) => sym.replace(/::h[0-9a-f]{16}$/, "").replace(/\+0x[0-9a-f]+$/, "");

function crateOf(sym, dso) {
  if (/kernel\.kallsyms|^\[kernel/.test(dso)) return "kernel";
  if (/\/libc[.-]|\/libc\.so/.test(dso)) return "libc";
  if (/\/ld-linux|\/ld-[0-9]/.test(dso)) return "ld";
  if (/\[vdso\]/.test(dso)) return "vdso";
  const s = sym.replace(/^(?:<|&|mut |dyn |\*const |\*mut |\(|\[)+/, "");
  const m = /^([A-Za-z_][A-Za-z0-9_]*)(?:::|<|$)/.exec(s);
  return m ? m[1] : "?";
}

const isProject = (c) => c === "inhibition" || c.startsWith("menteion_");
const classOf = (c, sym) =>
  /SpinBarrier>?::wait/.test(sym) ? "engine-wait" : c.startsWith("menteion_") ? "engine" : "harness";

// The module of a function: its path without the last segment, generic arguments dropped.
function moduleOf(sym) {
  const s = sym.replace(/^<([^ >]+)(?:<[^>]*>)?(?: as [^>]*>)?>/, "$1").replace(/<[^<>]*>/g, "");
  const parts = s.split("::").filter((p) => !/^\{\{closure\}\}$/.test(p));
  return parts.length > 1 ? parts.slice(0, -1).join("::") : parts.join("::");
}

const samples = [];
let cur = null;
const rl = readline.createInterface({ input: process.stdin, crlfDelay: Infinity });
for await (const line of rl) {
  if (line.trim() === "") {
    if (cur) samples.push(cur);
    cur = null;
    continue;
  }
  if (!/^\s/.test(line)) {
    if (cur) samples.push(cur);
    const h = /^(.*?)\s+(\d+)\s+(\d+\.\d+):/.exec(line);
    cur = { comm: h ? h[1].trim() : line.trim(), tid: h ? h[2] : "?", frames: [] };
    continue;
  }
  const f = /^\s+([0-9a-f]+)\s+(.*?)\s+\((.*)\)\s*$/.exec(line);
  if (cur && f) {
    const sym = clean(f[2]);
    cur.frames.push({ sym, dso: f[3], crate: crateOf(sym, f[3]) });
  }
}
if (cur) samples.push(cur);

const ROOTS = /^(_start|start_thread|clone3?|__clone3?|thread_start|__libc_start_main|__libc_start_call_main)$/;
const count = (map, key, n = 1) => map.set(key, (map.get(key) ?? 0) + n);
const byRole = new Map();
const roleOfTid = new Map();
for (const s of samples) {
  if (/^menteion-worker/.test(s.comm)) s.role = "worker";
  else if (s.frames.some((f) => f.crate === "inhibition")) s.role = "test";
  else s.role = null;
  if (s.role) {
    const v = roleOfTid.get(s.tid) ?? new Map();
    count(v, s.role);
    roleOfTid.set(s.tid, v);
  }
}
const owners = new Map();
const modules = new Map();
const inclusive = new Map();
const folded = new Map();
const truncated = new Map();
const leaves = new Map();
for (const s of samples) {
  if (!s.role) {
    const v = roleOfTid.get(s.tid);
    s.role = v ? [...v.entries()].sort((a, b) => b[1] - a[1])[0][0] : "other";
  }
  count(byRole, s.role);
  const root = s.frames.at(-1);
  if (!root || !ROOTS.test(root.sym)) count(truncated, s.role);
  const k = s.frames.findIndex((f) => isProject(f.crate));
  let cls = "outside";
  let owner = s.frames[0] ? `${s.frames[0].crate}: ${s.frames[0].sym}` : "(no frame)";
  if (k >= 0) {
    const f = s.frames[k];
    cls = classOf(f.crate, f.sym);
    owner = f.sym;
    count(modules, `${s.role}\t${cls}\t${moduleOf(f.sym)}`);
  }
  const key = `${s.role}\t${cls}\t${owner}`;
  const o = owners.get(key) ?? { n: 0, self: 0, leaves: new Map() };
  o.n += 1;
  if (k === 0) o.self += 1;
  else if (s.frames[0]) count(o.leaves, `${s.frames[0].crate}: ${s.frames[0].sym}`);
  owners.set(key, o);
  if (s.frames[0]) count(leaves, `${s.role}\t${s.frames[0].crate}\t${s.frames[0].sym}`);
  const seen = new Set();
  for (const f of s.frames) {
    if (isProject(f.crate) && !seen.has(f.sym)) {
      seen.add(f.sym);
      count(inclusive, `${s.role}\t${f.sym}`);
    }
  }
  count(folded, `${s.role}\t${s.frames.map((f) => f.sym).reverse().join(";")}`);
}

const total = samples.length;
const pct = (n, d) => (d ? ((100 * n) / d).toFixed(1) : "0.0");
const lines = [];
lines.push(`samples: ${total}`);
for (const [role, n] of [...byRole.entries()].sort((a, b) => b[1] - a[1])) {
  lines.push(`\n${role}: ${n} samples (${pct(n, total)} % of all), ${truncated.get(role) ?? 0} with a chain not reaching a thread's root`);
  const cls = new Map();
  for (const [key, o] of owners) {
    const [r, c] = key.split("\t");
    if (r === role) count(cls, c, o.n);
  }
  for (const [c, m] of [...cls.entries()].sort((a, b) => b[1] - a[1])) {
    lines.push(`  ${c.padEnd(12)} ${String(m).padStart(8)}  ${pct(m, n).padStart(5)} %`);
  }
  const top = [...owners.entries()]
    .filter(([key]) => key.startsWith(`${role}\t`))
    .sort((a, b) => b[1].n - a[1].n)
    .slice(0, 25);
  lines.push(`  the owners, top 25 (samples, % of the thread's, own frame's %):`);
  for (const [key, o] of top) {
    const [, c, owner] = key.split("\t");
    lines.push(`    ${String(o.n).padStart(8)} ${pct(o.n, n).padStart(5)} % ${pct(o.self, o.n).padStart(5)} %  ${c.padEnd(11)} ${owner}`);
  }
}
const summary = lines.join("\n") + "\n";
fs.writeFileSync(path.join(out, "classify-summary.txt"), summary);

const tsv = (file, header, rows) =>
  fs.writeFileSync(path.join(out, file), [header, ...rows].join("\n") + "\n");
tsv(
  "classify-owners.tsv",
  "role\tclass\towner\tsamples\town frame\ttop callee leaf\tits samples",
  [...owners.entries()]
    .sort((a, b) => b[1].n - a[1].n)
    .map(([key, o]) => {
      const top = [...o.leaves.entries()].sort((a, b) => b[1] - a[1])[0];
      return `${key}\t${o.n}\t${o.self}\t${top ? top[0] : ""}\t${top ? top[1] : 0}`;
    }),
);
tsv(
  "classify-modules.tsv",
  "role\tclass\tmodule\tsamples",
  [...modules.entries()].sort((a, b) => b[1] - a[1]).map(([k, n]) => `${k}\t${n}`),
);
tsv(
  "classify-inclusive.tsv",
  "role\tfunction\tsamples",
  [...inclusive.entries()].sort((a, b) => b[1] - a[1]).map(([k, n]) => `${k}\t${n}`),
);
tsv(
  "classify-leaves.tsv",
  "role\tcrate\tleaf\tsamples",
  [...leaves.entries()].sort((a, b) => b[1] - a[1]).slice(0, 2000).map(([k, n]) => `${k}\t${n}`),
);
for (const role of byRole.keys()) {
  const rows = [...folded.entries()]
    .filter(([k]) => k.startsWith(`${role}\t`))
    .map(([k, n]) => `${k.slice(role.length + 1)} ${n}`);
  fs.writeFileSync(path.join(out, `folded-${role}.txt`), rows.join("\n") + "\n");
}
process.stdout.write(summary);
