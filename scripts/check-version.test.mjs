// The version checker's own tests (node:test, no dependency): `npm run spec:version:test`.
// The case that made the check is the one at the top: a round that bumped the front matter and
// left the Document control table behind (F-43).

import { deepStrictEqual, ok, strictEqual } from "node:assert/strict";
import { test } from "node:test";

import { checkBump, checkVersion, compare, declarations, parseArgs } from "./check-version.mjs";

// A whitepaper, cut to the declarations the check reads.
const paper = (frontVersion, frontDate, tableVersion = frontVersion, tableDate = frontDate, body = "body") =>
  [
    "---",
    "title: VirtualCortex Architecture Whitepaper",
    `version: ${frontVersion}`,
    "status: active",
    `date: ${frontDate}`,
    "---",
    "",
    "# VirtualCortex Architecture Whitepaper",
    "",
    "| Document control | |",
    "| :--- | :--- |",
    `| Version | ${tableVersion} |`,
    "| Status | Active (living document; amended by ADR) |",
    `| Date | ${tableDate} |`,
    "| License | Apache-2.0 OR MIT |",
    "",
    body,
  ].join("\n");

test("a document whose two declarations agree passes and reports its version", () => {
  const { problems, version } = checkVersion(paper("4.16.0", "2026-09-20"));
  deepStrictEqual(problems, []);
  strictEqual(version, "4.16.0");
});

test("the version bumped in the front matter and not in the table fails, with both lines", () => {
  const { problems } = checkVersion(paper("4.15.0", "2026-09-19", "4.14.0", "2026-09-19"));
  strictEqual(problems.length, 1);
  ok(problems[0].includes("table says version 4.14.0"));
  ok(problems[0].includes("the front matter (line 3) says 4.15.0"));
});

test("the date is held the same way as the version", () => {
  const { problems } = checkVersion(paper("4.16.0", "2026-09-20", "4.16.0", "2026-09-14"));
  strictEqual(problems.length, 1);
  ok(problems[0].includes("the date moves with the version"));
});

test("a version that is not MAJOR.MINOR.PATCH fails", () => {
  const { problems } = checkVersion(paper("4.16", "2026-09-20"));
  ok(problems.some((p) => p.includes("is not MAJOR.MINOR.PATCH")));
});

test("a missing declaration is named rather than passed over", () => {
  const without = paper("4.16.0", "2026-09-20").split("\n").filter((l) => !l.startsWith("| Version |")).join("\n");
  const { problems } = checkVersion(without);
  deepStrictEqual(problems, ["docs/WHITEPAPER.md: no tableVersion declaration"]);
});

test("a change to the document with the version left behind fails on a pull request", () => {
  const base = paper("4.16.0", "2026-09-20");
  const here = paper("4.16.0", "2026-09-20", "4.16.0", "2026-09-20", "a new finding");
  const { problems, changed } = checkBump(here, base);
  strictEqual(changed, true);
  strictEqual(problems.length, 1);
  ok(problems[0].includes("leaves its version at 4.16.0 (the base is 4.16.0)"));
});

test("a change that moves the version forward passes, by patch or by minor", () => {
  const base = paper("4.16.0", "2026-09-20");
  for (const next of ["4.16.1", "4.17.0", "5.0.0"]) {
    const { problems, changed } = checkBump(paper(next, "2026-09-21", next, "2026-09-21", "new"), base);
    deepStrictEqual(problems, [], next);
    strictEqual(changed, true);
  }
});

test("a version that moves backwards is not a bump", () => {
  const base = paper("4.16.0", "2026-09-20");
  const { problems } = checkBump(paper("4.15.0", "2026-09-21", "4.15.0", "2026-09-21", "new"), base);
  strictEqual(problems.length, 1);
});

test("a branch that does not touch the document needs no bump, and neither does a run with no base", () => {
  const base = paper("4.16.0", "2026-09-20");
  deepStrictEqual(checkBump(base, base), { problems: [], changed: false });
  deepStrictEqual(checkBump(base, null), { problems: [], changed: false });
});

test("compare orders the triple and refuses what is not one", () => {
  strictEqual(compare("4.17.0", "4.16.0"), 1);
  strictEqual(compare("4.16.0", "4.16.0"), 0);
  strictEqual(compare("4.9.0", "4.10.0"), -1);
  strictEqual(compare("4.16.1", "4.16.0"), 1);
  strictEqual(compare("v4.16.0", "4.16.0"), null);
  strictEqual(compare(null, "4.16.0"), null);
});

test("the declarations carry the line they are on, which is what the failure prints", () => {
  const d = declarations(paper("4.16.0", "2026-09-20"));
  strictEqual(d.frontVersion.line, 3);
  strictEqual(d.frontDate.line, 5);
  strictEqual(d.tableVersion.line, 12);
  strictEqual(d.tableDate.line, 14);
});

test("a Version row later in the document does not stand in for the front matter's", () => {
  // The front matter is read as the front matter, not as the first line that matches.
  const d = declarations(["| Version | 9.9.9 |", "---", "version: 4.16.0", "date: 2026-09-20", "---"].join("\n"));
  strictEqual(d.frontVersion, null);
});

test("the arguments are a document and an optional base, and neither eats the other", () => {
  // The first run of this script read the default document because a missing `--base` made the
  // filter skip argument zero.
  deepStrictEqual(parseArgs([]), { base: null, path: "docs/WHITEPAPER.md" });
  deepStrictEqual(parseArgs(["/tmp/a.md"]), { base: null, path: "/tmp/a.md" });
  deepStrictEqual(parseArgs(["--base", "/tmp/b.md"]), { base: "/tmp/b.md", path: "docs/WHITEPAPER.md" });
  deepStrictEqual(parseArgs(["/tmp/a.md", "--base", "/tmp/b.md"]), { base: "/tmp/b.md", path: "/tmp/a.md" });
  deepStrictEqual(parseArgs(["--base", "/tmp/b.md", "/tmp/a.md"]), { base: "/tmp/b.md", path: "/tmp/a.md" });
  deepStrictEqual(parseArgs(["--base"]), { base: null, path: "docs/WHITEPAPER.md" });
});
