// Reads every <dir>/runtime-*/outcomes.json under the directory given and prints, per shard and
// in total, the seconds cargo-mutants spent in the Build phase and in the Test phase, by outcome.
// Usage: node sweep-phases.mjs <dir>
import fs from "node:fs";
import path from "node:path";

const root = process.argv[2];
const shards = fs.readdirSync(root).filter((d) => d.startsWith("runtime-")).sort((a, b) => Number(a.split("-")[1]) - Number(b.split("-")[1]));

const total = { n: 0, build: 0, test: 0, byOutcome: {}, builds: [], tests: [] };
const firstBuilds = [];
const rows = [];
for (const s of shards) {
  const file = path.join(root, s, "outcomes.json");
  if (!fs.existsSync(file)) continue;
  const o = JSON.parse(fs.readFileSync(file, "utf8"));
  let b = 0, t = 0, n = 0;
  const per = {};
  let first = null;
  for (const x of o.outcomes) {
    if (!x.scenario || !x.scenario.Mutant) continue; // the baseline, if any
    n += 1;
    let bx = 0, tx = 0;
    for (const p of x.phase_results) {
      if (p.phase === "Build") bx += p.duration;
      else if (p.phase === "Test") tx += p.duration;
    }
    if (first === null) first = { name: x.scenario.Mutant.name, build: bx, summary: x.summary };
    b += bx; t += tx;
    const k = x.summary;
    per[k] ??= { n: 0, build: 0, test: 0 };
    per[k].n += 1; per[k].build += bx; per[k].test += tx;
    total.byOutcome[k] ??= { n: 0, build: 0, test: 0 };
    total.byOutcome[k].n += 1; total.byOutcome[k].build += bx; total.byOutcome[k].test += tx;
    if (k !== "Unviable") total.builds.push(bx);
    if (tx > 0) total.tests.push(tx);
  }
  total.n += n; total.build += b; total.test += t;
  firstBuilds.push({ shard: s, ...first });
  const span = (Date.parse(o.end_time) - Date.parse(o.start_time)) / 1000;
  rows.push({ s, n, b, t, span, per });
}

const f = (x) => x.toFixed(0);
const pct = (a, b) => ((100 * a) / b).toFixed(1);
console.log("shard\tmutants\tbuild s\ttest s\tbuild %\ttool wall s");
for (const r of rows) console.log(`${r.s}\t${r.n}\t${f(r.b)}\t${f(r.t)}\t${pct(r.b, r.b + r.t)}\t${f(r.span)}`);
console.log(`all\t${total.n}\t${f(total.build)}\t${f(total.test)}\t${pct(total.build, total.build + total.test)}`);
console.log("\nby outcome: n, build s, test s, build s a mutant, test s a mutant");
for (const [k, v] of Object.entries(total.byOutcome)) console.log(`${k}\t${v.n}\t${f(v.build)}\t${f(v.test)}\t${(v.build / v.n).toFixed(1)}\t${(v.test / v.n).toFixed(1)}`);
const q = (arr, p) => { const a = [...arr].sort((x, y) => x - y); return a[Math.min(a.length - 1, Math.floor(p * a.length))]; };
console.log(`\nbuild of a viable mutant: min ${f(q(total.builds, 0))} median ${f(q(total.builds, 0.5))} p90 ${f(q(total.builds, 0.9))} max ${f(q(total.builds, 0.999))}`);
console.log(`test phase: min ${f(q(total.tests, 0))} median ${f(q(total.tests, 0.5))} p90 ${f(q(total.tests, 0.9))} max ${f(q(total.tests, 0.999))}`);
console.log("\nfirst mutant of each shard (its build is the shard's cold build):");
for (const x of firstBuilds) console.log(`${x.shard}\t${f(x.build)} s\t${x.summary}\t${x.name}`);
