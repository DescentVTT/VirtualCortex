// Tests of scripts/check-explorations.mjs under node:test (stable since Node 20; this repository
// requires 22). Every case is text in memory.

import assert from "node:assert/strict";
import { test } from "node:test";

import { LINK, NOTE, checkAdr, checkNote } from "./check-explorations.mjs";

const NOTE_TEXT = [
  "---",
  "status: exploratory",
  "date: 2026-10-19",
  "---",
  "",
  "# 001: A scan",
  "",
  "**Branch**: `explore/001-a-scan`",
  "**Commit**: `0123abc`",
  "**Runs**: [39153149745](https://github.com/DescentVTT/VirtualCortex/actions/runs/39153149745)",
  "",
].join("\n");

test("a note's name is three digits and a slug", () => {
  assert.ok(NOTE.test("001-a-scan.md"));
  assert.ok(!NOTE.test("README.md"));
  assert.ok(!NOTE.test("1-a.md"));
  assert.ok(!NOTE.test("001_A.md"));
});

test("a note with its status, branch, commit and run passes", () => {
  assert.deepEqual(checkNote("n.md", NOTE_TEXT), []);
});

test("a note not marked exploratory fails, or with no front matter", () => {
  assert.deepEqual(checkNote("n.md", NOTE_TEXT.replace("status: exploratory", "status: accepted")), [
    "n.md: its front matter does not say status: exploratory",
  ]);
  assert.equal(checkNote("n.md", NOTE_TEXT.replace(/^---[\s\S]*?---\n/, "")).length, 1);
});

test("a note that names no branch, no commit or no run fails once for each", () => {
  const bare = NOTE_TEXT.split("\n").filter((l) => !l.startsWith("**")).join("\n");
  assert.deepEqual(checkNote("n.md", bare), [
    "n.md: names no **Branch**: `...`",
    "n.md: names no **Commit**: `<hash>`",
    "n.md: names no **Runs**: line with a run of the host",
  ]);
});

test("a run line must link a run of the host, and a commit must be a hash", () => {
  assert.equal(checkNote("n.md", NOTE_TEXT.replace(/actions\/runs\/[0-9]+/, "actions")).length, 1);
  assert.equal(checkNote("n.md", NOTE_TEXT.replace("`0123abc`", "`main`")).length, 1);
});

test("an ADR that cites a note in its Context passes", () => {
  const adr = ["# ADR-0200", "", "## Context and Problem Statement", "", "An exploration ([001](../explorations/001-a-scan.md)) asked it.", "", "## Decision Outcome", "", "Yes."].join("\n");
  assert.deepEqual(checkAdr("a.md", adr), []);
});

test("an ADR that cites a note anywhere else fails, by line, in either link form", () => {
  const adr = [
    "# ADR-0200",
    "## Context and Problem Statement",
    "Fine here: [001](../explorations/001-a-scan.md).",
    "## Decision Outcome",
    "H-50 is yes, as [001](../explorations/001-a-scan.md) showed.",
    "## Consequences",
    "See [it](../../docs/explorations/001-a-scan.md).",
  ].join("\n");
  assert.deepEqual(checkAdr("a.md", adr), [
    "a.md:5: cites an exploration outside its Context: ](../explorations/001-a-scan.md)",
    "a.md:7: cites an exploration outside its Context: ](../../docs/explorations/001-a-scan.md)",
  ]);
});

test("a link into the title, above any section, fails as outside the Context", () => {
  assert.equal(checkAdr("a.md", "# ADR-0200: as [001](../explorations/001-a-scan.md) read\n").length, 1);
});

test("links elsewhere are not the lane's", () => {
  assert.deepEqual("[x](0190-a.md) [y](../WHITEPAPER.md)".match(LINK), null);
});
