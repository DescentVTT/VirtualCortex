#!/usr/bin/env node
// What the weekly sweep's timeouts were doing when the bound cut them (F-42).
//
// A timeout is read by the triage rule of ADR-0062, whose fifth kind is a mutant that was not
// hung at all but passing slowly, starved by the spinning mutant in the shard's other slot
// (`--jobs 2`, ADR-0058). Telling the two apart needs three facts `timeout.txt` does not carry:
// when the mutant ran, what ran beside it, and whether its own test run was still finishing
// tests when it was cut. All three are already in the artifact the job keeps -- `debug.log`'s
// Apply/Revert pair per mutant, `outcomes.json`'s verdicts and the per-mutant log -- and
// reading them by hand is what made F-42 expensive. This prints them.
//
// It reports; it does not classify. The verdict stays ADR-0062's rule, applied by the round
// that reads the list.
//
// Usage: node scripts/read-timeouts.mjs <mutants.out directory>
// Zero dependencies. Exit 1 only on a usage error: a sweep with no artifact is not this
// script's failure, and this must never be what fails a weekly job.
// The exports are for scripts/read-timeouts.test.mjs.

import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

// `<seconds>s TRACE worker thread{build_dir="..."}:mutant{...}: ... Apply mutant self=Mutant { name: "..." }`
// The name inside `self=Mutant` carries the line and the column, so it is the name
// `timeout.txt` and `outcomes.json` use; the one in the `mutant{...}` span does not.
const SPAN =
  /^\s*([\d.]+)s\s+\S+\s+worker thread\{build_dir="([^"]+)"\}.*?(Apply|Revert) mutant self=Mutant \{ name: "([^"]+)"/;
// `timeouts=Timeouts { build: None, test: Some(996s) }`, logged once as the lab starts.
const BOUND = /timeouts=Timeouts \{[^}]*test: Some\((\d+)s\)/;
// cargo's warning for a test that has not returned, and the lines that say one did.
const WARNED = /^test (.+) has been running for over/;
const FINISHED = /^test (.+?) \.\.\. \w+/;
const RESULT = /^test result:/;
const BINARY = /^\s*Running (?:`)?(?:.*[/\\])?([^/\\`\s]+)/;

// Every mutant's window and slot, under the name `timeout.txt` uses.
function windows(debugLog) {
  const spans = new Map();
  for (const line of debugLog.split(/\r?\n/)) {
    const m = SPAN.exec(line);
    if (m === null) continue;
    const [, at, slot, kind, name] = m;
    const span = spans.get(name) ?? { slot, start: null, end: null };
    span.slot = slot;
    if (kind === "Apply") span.start = Number(at);
    else span.end = Number(at);
    spans.set(name, span);
  }
  return spans;
}

// The seconds two windows share; 0 when they do not overlap or either is incomplete.
export function overlap(a, b) {
  if (a?.start == null || a?.end == null || b?.start == null || b?.end == null) return 0;
  return Math.max(0, Math.min(a.end, b.end) - Math.max(a.start, b.start));
}

// What a mutant's own log says about the test run that was cut: the binary it was inside, the
// tests cargo had warned about that never reported, and whether any test reported after the
// last such warning -- a run still finishing tests is not a hung one. A bound under sixty
// seconds ends before cargo's warning, so there is none and `progressed` is null: unknown,
// not false.
export function readLog(text) {
  const lines = text.split(/\r?\n/);
  const warned = [];
  const finished = new Set();
  let binary = null;
  let lastWarned = -1;
  let lastFinished = -1;
  lines.forEach((line, i) => {
    const b = BINARY.exec(line);
    if (b !== null) binary = b[1];
    const w = WARNED.exec(line);
    if (w !== null) {
      warned.push(w[1]);
      lastWarned = i;
    }
    const f = FINISHED.exec(line);
    if (f !== null) {
      finished.add(f[1]);
      lastFinished = i;
    }
    if (RESULT.test(line)) lastFinished = i;
  });
  return {
    binary,
    unfinished: warned.filter((t) => !finished.has(t)),
    warnings: warned.length,
    progressed: lastWarned < 0 ? null : lastFinished > lastWarned,
  };
}

// The module a mutant's file defines (`.../src/injector.rs` -> `injector`), which is how a unit
// test of that module is named in the log (`injector::tests::...`). Null for a path with no file.
export function moduleOf(name) {
  const file = /^([^:]+):\d+:\d+:/.exec(name)?.[1];
  const base = file === undefined ? undefined : file.split("/").pop();
  return base === undefined || !base.endsWith(".rs") ? null : base.slice(0, -3);
}

// Whether a test of the mutant's own module is among the ones that hung. It is the fact that
// separates a starved neighbour from a hang: a run may be finishing other threads' tests while
// the very test that would catch the mutant never returns.
export function ownModuleHung(name, unfinished) {
  const module = moduleOf(name);
  return module === null ? false : unfinished.some((t) => t.startsWith(`${module}::`));
}

// One report per timeout of `dir`, and a summary. `lines` is what to print.
export function readTimeouts(dir) {
  if (!existsSync(dir)) {
    return { timeouts: 0, lines: [`${dir}: no sweep output to read`], entries: [] };
  }
  const read = (name) => (existsSync(join(dir, name)) ? readFileSync(join(dir, name), "utf8") : null);
  const names = (read("timeout.txt") ?? "").split(/\r?\n/).filter((l) => l.trim() !== "");
  if (names.length === 0) return { timeouts: 0, lines: [`${dir}: no timeout`], entries: [] };

  const debugLog = read("debug.log");
  const spans = debugLog === null ? new Map() : windows(debugLog);
  const bound = debugLog === null ? null : (BOUND.exec(debugLog)?.[1] ?? null);
  const verdicts = new Map();
  const logPaths = new Map();
  const outcomesJson = read("outcomes.json");
  if (outcomesJson !== null) {
    for (const o of JSON.parse(outcomesJson).outcomes ?? []) {
      const name = o.scenario?.Mutant?.name;
      if (name === undefined) continue;
      verdicts.set(name, typeof o.summary === "string" ? o.summary : Object.keys(o.summary ?? {})[0]);
      if (o.log_path !== undefined) logPaths.set(name, o.log_path);
    }
  }

  const lines = [`${dir}: ${names.length} timeout(s)${bound === null ? "" : ` at a ${bound}s bound`}`];
  const entries = [];
  for (const name of names) {
    const span = spans.get(name);
    const beside = [];
    for (const [other, otherSpan] of spans) {
      if (other === name) continue;
      const shared = overlap(span, otherSpan);
      if (shared > 0) {
        beside.push({ name: other, seconds: shared, verdict: verdicts.get(other) ?? "unknown" });
      }
    }
    beside.sort((a, b) => b.seconds - a.seconds);
    const besideTimeout = beside.filter((b) => b.verdict === "Timeout");
    const logPath = logPaths.get(name);
    const kept = logPath !== undefined && existsSync(join(dir, logPath));
    const log = kept ? readLog(readFileSync(join(dir, logPath), "utf8")) : null;
    const ownHung = log === null ? false : ownModuleHung(name, log.unfinished);
    entries.push({ name, span, beside, besideTimeout, log, ownHung });

    lines.push(`- ${name}`);
    lines.push(
      span?.start == null
        ? "    window: not in debug.log"
        : `    ran ${span.start.toFixed(0)}s to ${span.end.toFixed(0)}s (${(span.end - span.start).toFixed(0)}s) in slot ${span.slot}`,
    );
    lines.push(
      beside.length === 0
        ? "    beside: nothing (no overlapping mutant in debug.log)"
        : `    beside ${beside.length} mutant(s), ${besideTimeout.length} of which timed out; longest overlap ${beside[0].seconds.toFixed(0)}s with ${beside[0].name} (${beside[0].verdict})`,
    );
    if (log === null) lines.push("    its log: not kept");
    else {
      const progress =
        log.progressed === null
          ? "no long-running warning (the bound is under cargo's sixty seconds), so whether it progressed is unknown"
          : log.progressed
            ? "a test reported AFTER the last long-running warning: the run was still finishing tests"
            : "no test reported after the last long-running warning: the run had stopped finishing tests";
      lines.push(`    its log ends in ${log.binary ?? "an unnamed binary"}; ${progress}`);
      if (log.unfinished.length > 0) {
        const own = ownHung
          ? " -- one of them a test of the mutant's own module"
          : " -- none of them a test of the mutant's own module";
        lines.push(`    warned and never reported${own}: ${log.unfinished.join(", ")}`);
      }
    }
  }
  const besideATimeout = entries.filter((e) => e.besideTimeout.length > 0).length;
  const stalled = entries.filter((e) => e.log?.progressed === false || e.ownHung).length;
  const progressing = entries.filter((e) => e.log?.progressed === true && !e.ownHung).length;
  const unknown = entries.filter((e) => e.log == null || e.log.progressed === null).length;
  lines.push(
    `${names.length} timeout(s): ${besideATimeout} beside another timeout; ${stalled} stopped finishing ` +
      `tests or hung a test of their own module; ${progressing} still finishing tests with no test ` +
      `of their own module hung, which is F-42's kind; ${unknown} without the evidence to say.`,
  );
  return { timeouts: names.length, lines, entries };
}

// Run as a script, not when imported by the tests. Compared without case: Windows paths are.
const invoked =
  process.argv[1] !== undefined &&
  resolve(process.argv[1]).toLowerCase() === fileURLToPath(import.meta.url).toLowerCase();

if (invoked) {
  const dir = process.argv[2];
  if (dir === undefined) {
    console.error("usage: node scripts/read-timeouts.mjs <mutants.out directory>");
    process.exit(1);
  }
  for (const line of readTimeouts(dir).lines) console.log(line);
}
