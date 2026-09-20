// The timeout reader's own tests (node:test, no dependency): `npm run mutants:timeouts:test`.
// The fixtures are the shapes the real artifact has, cut down to what each case needs; the
// cases that matter are the three readings of one bound expiring -- a hang, a starved
// neighbour still finishing tests (F-42), and a run finishing other threads' tests while the
// test that would catch the mutant hangs, which is a hang and not F-42's kind.

import { deepStrictEqual, ok, strictEqual } from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

import { moduleOf, overlap, ownModuleHung, readLog, readTimeouts } from "./read-timeouts.mjs";

const MUTANT = "runtime/cortex-runtime/src/injector.rs:81:34: replace < with > in Injector::push";
const OTHER = "runtime/cortex-runtime/src/image.rs:564:13: delete field clauses in Image::decode";

// One `Apply` or `Revert` line as cargo-mutants' debug.log writes it.
const span = (at, slot, kind, name) =>
  `  ${at}s TRACE worker thread{build_dir="${slot}"}:mutant{name="short"}: cargo_mutants::mutant: ` +
  `/x/mutant.rs:297: ${kind} mutant self=Mutant { name: "${name}", source_file: SourceFile { } }`;

// A per-mutant log: the tests that warned, and the ones that reported afterwards.
const log = (warned, after) =>
  ["     Running `/tmp/x/target/debug/deps/cortex_runtime-ff09`", "", "running 4 tests"]
    .concat(warned.map((t) => `test ${t} has been running for over 60 seconds`))
    .concat(after.map((t) => `test ${t} ... ok`))
    .concat(["", "*** result: Timeout"])
    .join("\n");

// A sweep output directory: `timeouts` are the names in timeout.txt, `logs` their contents.
function sweep({ timeouts = [], spans = [], logs = {}, bound = 996, verdicts = {} } = {}) {
  const dir = mkdtempSync(join(tmpdir(), "mutants-out-"));
  mkdirSync(join(dir, "log"));
  writeFileSync(join(dir, "timeout.txt"), timeouts.map((t) => `${t}\n`).join(""));
  writeFileSync(
    join(dir, "debug.log"),
    [`  0.3s DEBUG cargo_mutants::lab: /x/lab.rs:88: timeouts=Timeouts { build: None, test: Some(${bound}s) }`]
      .concat(spans)
      .join("\n"),
  );
  const outcomes = Object.keys(logs).map((name, i) => {
    writeFileSync(join(dir, "log", `m${i}.log`), logs[name]);
    return { scenario: { Mutant: { name } }, summary: verdicts[name] ?? "Timeout", log_path: `log/m${i}.log` };
  });
  // A neighbour needs a verdict and no log of its own.
  for (const [name, summary] of Object.entries(verdicts)) {
    if (!(name in logs)) outcomes.push({ scenario: { Mutant: { name } }, summary });
  }
  writeFileSync(join(dir, "outcomes.json"), JSON.stringify({ outcomes }));
  return dir;
}

test("a directory with no sweep output is not a failure, because a job must not fail on it", () => {
  const { timeouts, lines } = readTimeouts(join(tmpdir(), "no-such-sweep-output-dir"));
  strictEqual(timeouts, 0);
  ok(lines[0].endsWith("no sweep output to read"));
});

test("a sweep with no timeout says so and reads nothing else", () => {
  const { timeouts, lines } = readTimeouts(sweep());
  strictEqual(timeouts, 0);
  ok(lines[0].endsWith("no timeout"));
});

test("the window and the slot come from debug.log's Apply and Revert pair", () => {
  const dir = sweep({
    timeouts: [MUTANT],
    spans: [span(1729.4, "/tmp/a.tmp", "Apply", MUTANT), span(2730.4, "/tmp/a.tmp", "Revert", MUTANT)],
    logs: { [MUTANT]: log(["injector::tests::ring"], []) },
  });
  const { entries, lines } = readTimeouts(dir);
  strictEqual(entries[0].span.start, 1729.4);
  strictEqual(entries[0].span.end, 2730.4);
  ok(lines.some((l) => l.includes("ran 1729s to 2730s (1001s) in slot /tmp/a.tmp")));
});

test("what ran beside it is named with its verdict and the seconds they shared", () => {
  const dir = sweep({
    timeouts: [MUTANT],
    spans: [
      span(1000, "/tmp/a.tmp", "Apply", MUTANT),
      span(2000, "/tmp/a.tmp", "Revert", MUTANT),
      span(1400, "/tmp/b.tmp", "Apply", OTHER),
      span(2400, "/tmp/b.tmp", "Revert", OTHER),
    ],
    logs: { [MUTANT]: log(["injector::tests::ring"], []) },
    verdicts: { [OTHER]: "Timeout" },
  });
  const { entries, lines } = readTimeouts(dir);
  deepStrictEqual(entries[0].beside, [{ name: OTHER, seconds: 600, verdict: "Timeout" }]);
  ok(lines.some((l) => l.includes("1 of which timed out; longest overlap 600s")));
});

test("a run that reported no test after the last warning had stopped finishing tests", () => {
  const dir = sweep({
    timeouts: [MUTANT],
    spans: [span(10, "/tmp/a.tmp", "Apply", MUTANT), span(1010, "/tmp/a.tmp", "Revert", MUTANT)],
    logs: { [MUTANT]: log(["injector::tests::ring", "task::tests::refusals"], []) },
  });
  const { lines } = readTimeouts(dir);
  ok(lines.some((l) => l.includes("had stopped finishing tests")));
  ok(lines.at(-1).includes("0 still finishing tests"));
});

test("a run still finishing tests with no test of its own module hung is F-42's kind", () => {
  const dir = sweep({
    timeouts: [OTHER],
    spans: [span(10, "/tmp/a.tmp", "Apply", OTHER), span(1010, "/tmp/a.tmp", "Revert", OTHER)],
    logs: { [OTHER]: log(["a_slow_wave_onset_compacts_the_arena"], ["a_waking_day_at_256_units"]) },
  });
  const { entries, lines } = readTimeouts(dir);
  strictEqual(entries[0].log.progressed, true);
  strictEqual(entries[0].ownHung, false);
  ok(lines.some((l) => l.includes("none of them a test of the mutant's own module")));
  ok(lines.at(-1).includes("1 still finishing tests with no test of their own module hung"));
});

test("a run finishing another thread's tests while its own module's test hangs is not F-42's kind", () => {
  // The real case: `Injector::pop` replaced, `deque`'s test finishes after the warning, and
  // `injector`'s own test -- the one that would catch it -- never returns.
  const name = "runtime/cortex-runtime/src/injector.rs:91:9: replace Injector::pop with Some((0, 1))";
  const dir = sweep({
    timeouts: [name],
    spans: [span(10, "/tmp/a.tmp", "Apply", name), span(1010, "/tmp/a.tmp", "Revert", name)],
    logs: { [name]: log(["deque::tests::thieves", "injector::tests::four_producers"], ["deque::tests::thieves"]) },
  });
  const { entries, lines } = readTimeouts(dir);
  strictEqual(entries[0].log.progressed, true);
  strictEqual(entries[0].ownHung, true);
  ok(lines.some((l) => l.includes("one of them a test of the mutant's own module")));
  ok(lines.at(-1).includes("1 stopped finishing tests or hung a test of their own module"));
  ok(lines.at(-1).includes("0 still finishing tests"));
});

test("a bound under cargo's sixty seconds leaves no warning, and the reading is unknown, not false", () => {
  const dir = sweep({
    bound: 33,
    timeouts: [MUTANT],
    spans: [span(10, "/tmp/a.tmp", "Apply", MUTANT), span(43, "/tmp/a.tmp", "Revert", MUTANT)],
    logs: { [MUTANT]: log([], ["injector::tests::ring"]) },
  });
  const { entries, lines } = readTimeouts(dir);
  strictEqual(entries[0].log.progressed, null);
  ok(lines[0].includes("at a 33s bound"));
  ok(lines.at(-1).includes("1 without the evidence to say"));
});

test("overlap is the shared seconds, and zero for windows that do not meet or are incomplete", () => {
  strictEqual(overlap({ start: 0, end: 10 }, { start: 5, end: 20 }), 5);
  strictEqual(overlap({ start: 0, end: 10 }, { start: 10, end: 20 }), 0);
  strictEqual(overlap({ start: 0, end: 10 }, { start: 11, end: 20 }), 0);
  strictEqual(overlap({ start: 0, end: null }, { start: 5, end: 20 }), 0);
  strictEqual(overlap(undefined, { start: 5, end: 20 }), 0);
});

test("the module is the mutant's file, and a mutant with no file has none", () => {
  strictEqual(moduleOf(MUTANT), "injector");
  strictEqual(moduleOf("crates/cortex-core/src/dynamics/synapse.rs:358:9: replace next"), "synapse");
  strictEqual(moduleOf("not a mutant name"), null);
  strictEqual(ownModuleHung(MUTANT, ["injector::tests::ring"]), true);
  strictEqual(ownModuleHung(MUTANT, ["deque::tests::thieves"]), false);
  strictEqual(ownModuleHung("not a mutant name", ["injector::tests::ring"]), false);
});

test("the log names the binary it was inside and the tests that warned and never reported", () => {
  const read = readLog(log(["a::b", "c::d"], ["a::b"]));
  strictEqual(read.binary, "cortex_runtime-ff09");
  deepStrictEqual(read.unfinished, ["c::d"]);
  strictEqual(read.warnings, 2);
});
