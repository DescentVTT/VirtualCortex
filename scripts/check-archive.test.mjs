// Tests of scripts/check-archive.mjs under node:test, which ships with Node (stable since 20;
// this repository requires 22), so the checker gains no dependency. Each case writes an archive
// directory of its own into a temporary folder and removes it afterwards.

import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

import { BRIEF, INTEGRITY, checkArchive, frontMatter } from "./check-archive.mjs";

const HASH = `sha256-${"0123456789abcdef".repeat(4)}`;

// An archived brief, its front matter holding `fields` in order.
function brief(fields = ["status: archived", "date: 2026-10-17", `integrity: ${HASH}`], eol = "\n") {
  return ["---", ...fields, "---", "", "> **Executed.**", "", "# Brief", ""].join(eol);
}

// Writes `files` (name -> text; a name ending in `/` is a directory) and checks the folder.
function check(files) {
  const dir = mkdtempSync(join(tmpdir(), "archive-"));
  try {
    for (const [name, text] of Object.entries(files)) {
      if (name.endsWith("/")) mkdirSync(join(dir, name));
      else writeFileSync(join(dir, name), text);
    }
    return checkArchive(dir);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

test("an archive whose every brief carries its hash passes, and counts them", () => {
  const r = check({ "001_a.md": brief(), "002_b.md": brief() });
  assert.deepEqual(r, { count: 2, problems: [] });
});

test("a brief with no hash fails, by name, and says how a brief is archived", () => {
  const r = check({ "001_a.md": brief(), "002_b.md": brief(["status: archived", "date: 2026-10-17"]) });
  assert.equal(r.count, 2);
  assert.equal(r.problems.length, 1);
  assert.match(r.problems[0], /^002_b\.md: no integrity hash; archive a brief with `spec-brief archive`/);
});

test("a brief with no front matter fails, and one whose front matter never closes", () => {
  const r = check({
    "001_a.md": "# Brief\n\nintegrity: " + HASH + "\n",
    "002_b.md": ["---", "status: archived", `integrity: ${HASH}`, "", "# Brief"].join("\n"),
  });
  assert.deepEqual(r.problems, [
    "001_a.md: no front matter, so no integrity hash",
    "002_b.md: no front matter, so no integrity hash",
  ]);
});

test("a hash in the body and not in the front matter does not count", () => {
  const text = brief(["status: archived"]) + `integrity: ${HASH}\n`;
  assert.equal(check({ "001_a.md": text }).problems.length, 1);
});

test("a hash not in spec-brief's form fails: short, upper case, another algorithm, or empty", () => {
  for (const value of [`sha256-${"0".repeat(63)}`, `sha256-${"A".repeat(64)}`, `sha512-${"0".repeat(64)}`, ""]) {
    const r = check({ "001_a.md": brief(["status: archived", `integrity: ${value}`]) });
    assert.equal(r.problems.length, 1, value);
  }
});

test("CRLF line endings and a byte-order mark read as the same brief", () => {
  const r = check({ "001_a.md": "﻿" + brief(undefined, "\r\n") });
  assert.deepEqual(r, { count: 1, problems: [] });
});

test("files that are not briefs are not read: the README, a .gitkeep, a directory, a short number", () => {
  const r = check({
    "001_a.md": brief(),
    "README.md": "no front matter",
    ".gitkeep": "",
    "sub/": "",
    "01_short.md": "no front matter",
    "001_a.txt": "no front matter",
  });
  assert.deepEqual(r, { count: 1, problems: [] });
});

test("an archive with no brief fails, since a check over nothing reads as a clean one", () => {
  const r = check({ "README.md": "x" });
  assert.equal(r.count, 0);
  assert.deepEqual(r.problems.length, 1);
  assert.match(r.problems[0], /^no archived brief in .*, so nothing was checked$/);
});

test("the forms the checker reads are spec-brief's", () => {
  assert.ok(BRIEF.test("073_a-sixteenth-on-three-networks.md"));
  assert.ok(!BRIEF.test("README.md"));
  assert.ok(INTEGRITY.test(`integrity: ${HASH}`));
  assert.ok(!INTEGRITY.test(`integrity: ${HASH} extra`));
  assert.deepEqual(frontMatter("---\na: 1\n---\nbody"), ["a: 1"]);
  assert.equal(frontMatter("body\n---\na: 1\n---\n"), null);
});
