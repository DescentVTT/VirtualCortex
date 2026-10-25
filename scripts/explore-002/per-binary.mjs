// Per test binary of the runtime, mean seconds over the interleaved run's four builds, by commit,
// profile and step. node per-binary.mjs <il dir> <commit>
import fs from "node:fs";
import path from "node:path";
import { units, label } from "./timings.mjs";
const [root, want] = process.argv.slice(2);
const acc = {};
const walls = {};
for (const a of fs.readdirSync(root)) {
  const m = /^build-cost-(debug|release)-(\d)$/.exec(a);
  if (!m) continue;
  for (const f of fs.readdirSync(path.join(root, a))) {
    const n = new RegExp("^" + want + "-(check-cold|test-cold|test-mutant)-([0-9])[.]html$").exec(f);
    if (!n) continue;
    const col = `${m[1]} ${n[1]}`;
    const us = units(fs.readFileSync(path.join(root, a, f), "utf8"));
    (walls[col] ??= []).push(Math.max(...us.map((u) => u.start + u.duration)));
    for (const u of us) {
      const l = label(u);
      let k = null;
      if (/^cortex-runtime (test \S+|lib \(test\)|lib)$/.test(l)) k = l.replace("cortex-runtime ", "");
      else if (/^cortex-/.test(l)) k = "the 33 other workspace crates";
      else k = "third-party crates (criterion's)";
      ((acc[k] ??= {})[col] ??= []).push(u.duration);
    }
  }
}
const cols = ["debug check-cold", "debug test-cold", "debug test-mutant", "release test-cold", "release test-mutant"];
const mean = (xs) => (xs && xs.length ? xs.reduce((s, x) => s + x, 0) / 4 : 0);
console.log(["unit", ...cols].join("\t"));
const rows = Object.entries(acc).sort((a, b) => mean(b[1]["release test-cold"]) - mean(a[1]["release test-cold"]));
for (const [k, v] of rows) console.log([k, ...cols.map((c) => mean(v[c]).toFixed(1))].join("\t"));
console.log(["wall", ...cols.map((c) => `${mean(walls[c]).toFixed(1)} [${(walls[c] ?? []).map((x) => x.toFixed(0)).join(",")}]`)].join("\t"));
