// Reads the interleaved build-cost artifacts (build-cost-<profile>-<runner>/) and prints, per
// profile, runner and commit, each step's wall clock and the units that matter, by round.
// node interleaved.mjs <run dir>
import fs from "node:fs";
import path from "node:path";
import { units, label } from "./timings.mjs";

const root = process.argv[2];
const names = { "5ee329b": "main's tree", "532d2b6": "data file", "4cac2e8": "include_str" };
const rows = [];
for (const a of fs.readdirSync(root).sort()) {
  const m = /^build-cost-(debug|release)-(\d)$/.exec(a);
  if (!m) continue;
  const [, profile, runner] = m;
  const dir = path.join(root, a);
  const steps = fs.readFileSync(path.join(dir, "steps.tsv"), "utf8").trim().split("\n").map((l) => l.split("\t"));
  for (const [commit, step, round, secs, rc] of steps) {
    const html = path.join(dir, `${commit}-${step}-${round}.html`);
    const r = { profile, runner, commit, step, round, secs: Number(secs), rc };
    if (fs.existsSync(html)) {
      const us = units(fs.readFileSync(html, "utf8"));
      r.wall = Math.max(...us.map((u) => u.start + u.duration));
      const get = (re) => us.filter((u) => re.test(label(u))).reduce((s, u) => s + u.duration, 0);
      r.inhibition = get(/^cortex-runtime test inhibition$/);
      r.assembly = get(/^cortex-runtime test assembly$/);
      r.instrument = get(/^cortex-runtime test instrument$/);
      r.libtest = get(/^cortex-runtime lib \(test\)$/);
      // The units no commit changes: every unit of the workspace but the two test binaries.
      r.others = us.filter((u) => !/^cortex-runtime test (inhibition|assembly)$/.test(label(u))).reduce((s, u) => s + u.duration, 0);
      r.sum = us.reduce((s, u) => s + u.duration, 0);
    }
    rows.push(r);
  }
}

const f = (x) => (x === undefined ? "" : x.toFixed(1));
console.log("profile\trunner\tcommit\tstep\tround\tsecs\twall\tinhibition\tassembly\tinstrument\tlib(test)\tothers\tsum");
for (const r of rows) {
  console.log([r.profile, r.runner, `${r.commit} ${names[r.commit] ?? ""}`, r.step, r.round, r.secs, f(r.wall), f(r.inhibition), f(r.assembly), f(r.instrument), f(r.libtest), f(r.others), f(r.sum)].join("\t"));
}

// Means over the two rounds and the two runners, per profile, step and commit.
console.log("\nmeans over rounds and runners (n), wall / inhibition / assembly / others");
const key = (r) => `${r.profile}\t${r.step}\t${r.commit}`;
const groups = new Map();
for (const r of rows) if (r.wall !== undefined) { if (!groups.has(key(r))) groups.set(key(r), []); groups.get(key(r)).push(r); }
const mean = (xs) => xs.reduce((s, x) => s + x, 0) / xs.length;
for (const [k, rs] of [...groups.entries()].sort()) {
  const [profile, step, commit] = k.split("\t");
  console.log(`${profile}\t${step}\t${commit} ${(names[commit] ?? "").padEnd(12)}\tn=${rs.length}\twall ${f(mean(rs.map((r) => r.wall)))} [${rs.map((r) => f(r.wall)).join(", ")}]\tinhibition ${f(mean(rs.map((r) => r.inhibition)))} [${rs.map((r) => f(r.inhibition)).join(", ")}]\tassembly ${f(mean(rs.map((r) => r.assembly)))}\tothers ${f(mean(rs.map((r) => r.others)))}`);
}
