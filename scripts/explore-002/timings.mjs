// Reads cargo's --timings HTML reports out of build-cost artifacts.
// node timings.mjs <run dir> [unit regex]
// For each artifact (build-cost-<sha>-<profile>) prints steps.tsv and, for each step's report,
// the build's wall clock (the last unit's end) and each matching unit's seconds.
import fs from "node:fs";
import path from "node:path";

export function units(html) {
  const m = /const UNIT_DATA = (\[[\s\S]*?\n\]);/.exec(html);
  if (!m) throw new Error("no UNIT_DATA");
  return JSON.parse(m[1]).filter((u) => u.duration > 0);
}

export function label(u) {
  const t = u.target.trim();
  if (/\(test\)$/.test(t) && /"lib"/.test(t)) return `${u.name} lib (test)`;
  const mt = /^(\S+) "test"/.exec(t);
  if (mt) return `${u.name} test ${mt[1]}`;
  if (t === "") return `${u.name} lib`;
  return `${u.name} ${t}`;
}

if (process.argv[1] && process.argv[1].endsWith("timings.mjs")) {
  const root = process.argv[2];
  const re = new RegExp(process.argv[3] ?? "inhibition|assembly|instrument|cortex-runtime lib|criticality|reference|everywhere|image");
  for (const a of fs.readdirSync(root).sort()) {
    const dir = path.join(root, a);
    console.log(`\n## ${a}`);
    const steps = fs.readFileSync(path.join(dir, "steps.tsv"), "utf8").trim();
    console.log(steps);
    for (const f of fs.readdirSync(dir).filter((x) => x.endsWith(".html")).sort()) {
      const us = units(fs.readFileSync(path.join(dir, f), "utf8"));
      const wall = Math.max(...us.map((u) => u.start + u.duration));
      const sum = us.reduce((s, u) => s + u.duration, 0);
      const ws = us.filter((u) => !/^(criterion|clap|serde|regex|syn|proc-macro2|quote|plotters|ciborium|half|itertools|num-traits|aho-corasick|memchr|walkdir|same-file|tinytemplate|unicode-ident|anes|anstyle|autocfg|cast|cfg-if|crunchy|either|itoa|oorandom|ryu|winapi|windows|zerocopy|serde_core|serde_derive|serde_json|regex-automata|regex-syntax|clap_builder|clap_lex|ciborium-io|ciborium-ll|criterion-plot)$/.test(u.name));
      const wsum = ws.reduce((s, u) => s + u.duration, 0);
      console.log(`  ${f}: wall ${wall.toFixed(1)} s, units ${us.length}, sum ${sum.toFixed(1)} s, workspace sum ${wsum.toFixed(1)} s`);
      for (const u of us.filter((u) => re.test(label(u))).sort((x, y) => y.duration - x.duration)) {
        console.log(`    ${label(u).padEnd(44)} ${u.duration.toFixed(2).padStart(7)} s  start ${u.start.toFixed(1)}`);
      }
    }
  }
}
