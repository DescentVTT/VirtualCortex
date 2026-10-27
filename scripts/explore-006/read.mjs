// Exploration 006, on its branch alone (ADR-0193): the note's tables, from the artifacts of the
// runs `gh run download <run> -D <dir>` saved, every run into the same <dir> or one each.
// node read.mjs <dir> [<dir> ...]
// For every job of run.sh (an artifact holding explore-006/machine.txt): its spec, the runner, the
// builds (wall clock, and the inhibition binary's own unit from cargo's report), and each run's
// clock and TIME phases: the derivation, the schedule (from the decode to the run's end) and the
// seconds a block over the schedule's blocks. For `profiles`, each profile's runs beside release's
// of the same round.
import fs from "node:fs";
import path from "node:path";

function jobs(root) {
  const found = [];
  const walk = (dir) => {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, e.name);
      if (e.isDirectory()) {
        if (e.name === "explore-006" && fs.existsSync(path.join(p, "machine.txt"))) found.push(p);
        else walk(p);
      }
    }
  };
  walk(root);
  return found;
}

const tsv = (file) =>
  fs.existsSync(file)
    ? fs.readFileSync(file, "utf8").split("\n").filter(Boolean).map((l) => l.split("\t"))
    : [];

export function units(html) {
  const m = /const UNIT_DATA = (\[[\s\S]*?\n\]);/.exec(html);
  if (!m) return [];
  return JSON.parse(m[1]);
}

const s1 = (ms) => (Number(ms) / 1000).toFixed(1);

for (const root of process.argv.slice(2)) {
  for (const dir of jobs(root).sort()) {
    const machine = fs.readFileSync(path.join(dir, "machine.txt"), "utf8");
    const spec = /^spec: (.*)$/m.exec(machine)?.[1] ?? "?";
    const cpu = /^Model name:\s*(.*)$/m.exec(machine)?.[1] ?? "?";
    const commit = /^commit: (.*)$/m.exec(machine)?.[1]?.slice(0, 7) ?? "?";
    console.log(`\n## ${spec}\n${dir}\ncommit ${commit}, ${cpu}, nproc ${/^nproc: (.*)$/m.exec(machine)?.[1]}`);
    for (const [profile, scope, ms, rc] of tsv(path.join(dir, "builds.tsv"))) {
      const html = path.join(dir, `timings-${profile}-${scope}.html`);
      let inh = "";
      if (fs.existsSync(html)) {
        const us = units(fs.readFileSync(html, "utf8"));
        const u = us.find((x) => x.name === "menteion-runtime" && /inhibition/.test(x.target));
        const lib = us.find((x) => x.name === "menteion-runtime" && x.target.trim() === "");
        const wall = us.length ? Math.max(...us.map((x) => x.start + x.duration)) : 0;
        inh = `; report's wall ${wall.toFixed(1)} s, ${us.length} units; inhibition ${u ? u.duration.toFixed(1) : "?"} s` +
          `${lib ? `, the runtime's library ${lib.duration.toFixed(1)} s` : ""}`;
      }
      console.log(`  build ${profile} ${scope}: ${s1(ms)} s, exit ${rc}${inh}`);
    }
    for (const [profile, scope, bytes] of tsv(path.join(dir, "sizes.tsv"))) {
      console.log(`  inhibition binary ${profile} (${scope}): ${(Number(bytes) / 1e6).toFixed(1)} MB`);
    }
    const times = tsv(path.join(dir, "times.tsv"));
    const runs = tsv(path.join(dir, "runs.tsv"));
    const phase = (round, label) => {
      const t = new Map(times.filter((x) => x[0] === round && x[1] === label).map((x) => [x[2], Number(x[3])]));
      const blocks = [...t.keys()].filter((k) => k.startsWith("block ")).length;
      const d = t.get("derivation");
      const sched = t.get("run") - t.get("decoded");
      return { d, decode: t.get("decoded") - d, sched, blocks, per: blocks ? sched / blocks : NaN, t };
    };
    const byRound = new Map();
    for (const [round, label, ms, rc, passed] of runs) {
      const p = phase(round, label);
      byRound.set(`${round}\t${label}`, { ms: Number(ms), p });
      console.log(
        `  run ${round} ${label.padEnd(8)}: ${s1(ms).padStart(7)} s, exit ${rc}, passed ${passed}; derivation ${s1(p.d)} s, decode ${s1(p.decode)} s, ` +
          `schedule ${s1(p.sched)} s over ${p.blocks} blocks, ${(p.per / 1000).toFixed(2)} s a block`,
      );
    }
    if (/\/profiles\//.test(spec)) {
      const labels = [...new Set(runs.map((r) => r[1]))];
      console.log("  beside release's run of the same round (run, derivation, schedule):");
      for (const label of labels.filter((l) => l !== "release")) {
        const rs = [...new Set(runs.map((r) => r[0]))]
          .map((round) => [byRound.get(`${round}\t${label}`), byRound.get(`${round}\trelease`)])
          .filter(([a, b]) => a && b && a.ms && b.ms);
        const fmt = (f) => rs.map(([a, b]) => (f(a) / f(b)).toFixed(3)).join(" ");
        console.log(`    ${label.padEnd(8)} ${fmt((x) => x.ms)} | ${fmt((x) => x.p.d)} | ${fmt((x) => x.p.sched)}`);
      }
    }
    // The block times of a whole run: the slowest and fastest block, and the first 16's mean.
    for (const [round, label] of runs) {
      const p = byRound.get(`${round}\t${label}`)?.p;
      if (!p || p.blocks < 120) continue;
      const at = (k) => p.t.get(`block ${k}`);
      const each = [];
      for (let k = 1; k <= p.blocks; k++) each.push(at(k) - (k === 1 ? p.t.get("decoded") : at(k - 1)));
      const mean = (a) => a.reduce((s, x) => s + x, 0) / a.length;
      console.log(
        `  ${label} blocks: first ${s1(each[0])} s (with the lead-in), 2..16 mean ${s1(mean(each.slice(1, 16)))} s, ` +
          `17..120 mean ${s1(mean(each.slice(16)))} s, min ${s1(Math.min(...each.slice(1)))} s, max ${s1(Math.max(...each.slice(1)))} s`,
      );
    }
    const summary = path.join(dir, "classify-summary.txt");
    if (fs.existsSync(summary)) console.log(fs.readFileSync(summary, "utf8"));
  }
}
