// Tests of scripts/check-scope.mjs under node:test (stable since Node 20; this repository requires
// 22). The briefs are text in memory and the base a function, so no case touches git or a file.

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

import { parseConfig } from "@spec-family/spec-brief";

import { BRIEF, SCOPE_FROM, checkScope } from "./check-scope.mjs";

const config = parseConfig(readFileSync(".spec-brief.json", "utf8"), ".spec-brief.json");

// A brief's text with the front matter's scope lines as given (each a line, or absent).
function text(scope = ["affectedFiles: [docs/**]", "protectedFiles: [src/**]"], status = "proposed") {
  return ["---", `status: ${status}`, "date: 2026-10-19", ...scope, "---", "", "# Brief", ""].join("\n");
}

const live = (name, t = text()) => ({ path: `briefs/${name}`, name, phase: "active", text: t });
const archived = (name, t = text(undefined, "archived")) => ({ path: `briefs/archive/${name}`, name, phase: "archived", text: t });
const noBase = () => null;

test("the floor is brief 075, and the names are spec-brief's", () => {
  assert.equal(SCOPE_FROM, 75);
  assert.ok(BRIEF.test("075_a-scope.md"));
  assert.ok(!BRIEF.test("README.md"));
});

test("a brief from 075 that declares both passes, with no note", () => {
  assert.deepEqual(checkScope([live("075_a.md")], noBase, config), { problems: [], notes: [] });
});

test("a brief from 075 without affectedFiles, or with an empty one, fails by name", () => {
  for (const scope of [["protectedFiles: [src/**]"], ["affectedFiles: []", "protectedFiles: [src/**]"]]) {
    const { problems } = checkScope([live("075_a.md", text(scope))], noBase, config);
    assert.deepEqual(problems, ["briefs/075_a.md: declares no affectedFiles"]);
  }
});

test("a brief from 075 without protectedFiles fails, and one with neither fails twice", () => {
  assert.deepEqual(checkScope([live("076_b.md", text(["affectedFiles: [docs/**]"]))], noBase, config).problems, [
    "briefs/076_b.md: declares no protectedFiles",
  ]);
  assert.equal(checkScope([live("076_b.md", text([]))], noBase, config).problems.length, 2);
});

test("a brief before 075 is not held to a scope, live or archived", () => {
  const r = checkScope([live("074_a.md", text([])), archived("073_b.md", text([], "archived"))], noBase, config);
  assert.deepEqual(r, { problems: [], notes: [] });
});

test("an archived brief from 075 is held as a live one is", () => {
  const r = checkScope([archived("075_a.md", text(["affectedFiles: [docs/**]"], "archived"))], noBase, config);
  assert.deepEqual(r.problems, ["briefs/archive/075_a.md: declares no protectedFiles"]);
});

test("a protected pattern the base's brief holds and this one dropped fails, by pattern", () => {
  const base = (name) => (name === "075_a.md" ? text(["affectedFiles: [docs/**]", "protectedFiles: [src/**, ci.yml]"]) : null);
  const r = checkScope([archived("075_a.md")], base, config);
  assert.deepEqual(r.problems, ["briefs/archive/075_a.md: no longer protects ci.yml, which the base's brief protects"]);
});

test("protecting more than the base passes; widening affectedFiles passes and is noted", () => {
  const base = () => text(["affectedFiles: [docs/**]", "protectedFiles: [src/**]"]);
  const here = text(["affectedFiles: [docs/**, README.md]", "protectedFiles: [src/**, ci.yml]"]);
  assert.deepEqual(checkScope([live("075_a.md", here)], base, config), {
    problems: [],
    notes: ["briefs/075_a.md: affectedFiles widened by README.md since the base"],
  });
});

test("a block list reads as an inline one", () => {
  const block = text(["affectedFiles:", "  - docs/**", "protectedFiles:", "  - src/**"]);
  const base = () => text();
  assert.deepEqual(checkScope([live("075_a.md", block)], base, config), { problems: [], notes: [] });
});

test("a brief the base does not hold is compared with nothing", () => {
  assert.deepEqual(checkScope([live("077_new.md")], noBase, config), { problems: [], notes: [] });
});
