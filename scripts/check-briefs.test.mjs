// Tests of scripts/check-briefs.mjs under node:test, which ships with Node (stable since 20;
// this repository requires 22), so the checker gains no dependency. Each case writes a briefs
// directory of its own into a temporary folder and removes it afterwards.

import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

import { DOCTRINE, DOCTRINE_FROM, REQUIRED, checkBriefs } from "./check-briefs.mjs";

const WITH_DOCTRINE = `- **${DOCTRINE}** (§2.1): stable tools only.\n- Say what you did not do.`;
const WITHOUT_DOCTRINE = "- Say what you did not do.";

// A brief in the shape briefs/README.md asks for, with the parts a case varies.
function brief({ status = "proposed", directives = WITH_DOCTRINE, context = "Facts.", omit = [] } = {}) {
  const body = { "## Standing directives": directives, "## Context": context };
  const sections = REQUIRED.filter((h) => !omit.includes(h))
    .map((h) => `${h}\n\n${body[h] ?? "Text."}\n`)
    .join("\n");
  return `---\nstatus: ${status}\ndate: 2026-09-19\n---\n\n# Brief\n\n${sections}`;
}

// Writes `files` (name -> text; a name ending in `/` is a directory) and checks the folder.
function check(files) {
  const dir = mkdtempSync(join(tmpdir(), "briefs-"));
  try {
    for (const [name, text] of Object.entries(files)) {
      if (name.endsWith("/")) mkdirSync(join(dir, name));
      else writeFileSync(join(dir, name), text);
    }
    return checkBriefs(dir);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

test("the floor is brief 028", () => {
  assert.equal(DOCTRINE_FROM, 28);
  assert.equal(DOCTRINE, "Latest ≠ Newest");
});

test("a brief from the floor that restates the doctrine passes", () => {
  assert.deepEqual(check({ "028_a.md": brief() }), { live: 1, problems: [] });
});

test("a brief from the floor without the doctrine fails, and the message names it", () => {
  const { problems } = check({ "028_a.md": brief({ directives: WITHOUT_DOCTRINE }) });
  assert.equal(problems.length, 1);
  assert.match(problems[0], /028_a\.md: its standing directives must restate 'Latest ≠ Newest'/);
});

test("the doctrine outside the standing directives does not count", () => {
  const { problems } = check({
    "029_a.md": brief({ directives: WITHOUT_DOCTRINE, context: `${DOCTRINE} is §2.1.` }),
  });
  assert.equal(problems.length, 1);
});

test("a brief below the floor is not held to the doctrine", () => {
  assert.deepEqual(check({ "027_a.md": brief({ directives: WITHOUT_DOCTRINE }) }).problems, []);
});

test("a brief without the section is reported for the section and for the doctrine", () => {
  const { problems } = check({ "030_a.md": brief({ omit: ["## Standing directives"] }) });
  assert.deepEqual(
    problems.map((p) => p.replace(/^.*030_a\.md: /, "")),
    [
      "missing section '## Standing directives'",
      "its standing directives must restate 'Latest ≠ Newest' (CLAUDE.md principle 4, whitepaper §2.1)",
    ],
  );
});

test("a missing status and a missing section are still reported", () => {
  const { problems } = check({ "030_a.md": brief({ status: "archived", omit: ["## Report"] }) });
  assert.deepEqual(
    problems.map((p) => p.replace(/^.*030_a\.md: /, "")),
    ["live brief must declare 'status: proposed' in front matter", "missing section '## Report'"],
  );
});

test("only numbered files directly in the folder are live briefs", () => {
  const result = check({
    "README.md": "# Briefs",
    "archive/": "",
    "028_a.md": brief(),
  });
  assert.deepEqual(result, { live: 1, problems: [] });
});
