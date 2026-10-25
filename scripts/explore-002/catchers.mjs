// Exploration 002: for a sweep's caught runtime mutants whose test phase ran past a minute, the
// binary that failed (the last one cargo ran) and the seconds the binaries that passed before it
// spent. node catchers.mjs <dir holding runtime-*/ from the mutants-out-runtime-* artifacts>
import fs from "node:fs";
import path from "node:path";

const root = process.argv[2];
const stem = (p) => path.basename(p).replace(/-[0-9a-f]{16}$/, "");
const catcher = {};
const before = {};
let n = 0;
for (const d of fs.readdirSync(root)) {
  const o = JSON.parse(fs.readFileSync(path.join(root, d, "outcomes.json"), "utf8"));
  for (const x of o.outcomes) {
    if (!x.scenario.Mutant || x.summary !== "CaughtMutant") continue;
    let t = 0;
    for (const p of x.phase_results) if (p.phase === "Test") t += p.duration;
    if (t <= 60) continue;
    n += 1;
    const log = fs.readFileSync(path.join(root, d, x.log_path), "utf8");
    const tail = log.slice(log.lastIndexOf("*** /"));
    let cur = null;
    for (const l of tail.split("\n")) {
      const m = /^\s+Running `([^`]+)`/.exec(l);
      if (m) {
        cur = stem(m[1]);
        continue;
      }
      const r = /^test result: ok\..*finished in ([0-9.]+)s/.exec(l);
      if (r && cur) before[cur] = (before[cur] ?? 0) + Number(r[1]);
    }
    const k = cur ?? "?";
    catcher[k] ??= { n: 0, t: 0 };
    catcher[k].n += 1;
    catcher[k].t += t;
  }
}
console.log(`${n} caught mutants with a test phase over 60 s; the binary that failed:`);
for (const [k, v] of Object.entries(catcher).sort((a, b) => b[1].t - a[1].t)) {
  console.log(`  ${k.padEnd(16)} ${String(v.n).padStart(4)}  ${(v.t / 3600).toFixed(1)} h of test phase`);
}
console.log("the binaries that passed before it, their seconds summed:");
for (const [k, v] of Object.entries(before).sort((a, b) => b[1] - a[1]).slice(0, 10)) {
  console.log(`  ${k.padEnd(16)} ${(v / 3600).toFixed(1)} h`);
}
