// Tests of scripts/check-doc-shape.mjs under node:test (stable since Node 20; this repository
// requires 22). The tree and the base are Maps in memory, so no case but the last touches git or
// a file; the last runs the script against a base that does not exist.

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { test } from "node:test";

import { KINDS, LIMITS, blocks, cells, changelogEntries, checkShape, normalise } from "./check-doc-shape.mjs";

const words = (n) => "x".repeat(n);
const WP = "docs/WHITEPAPER.md";
const tree = (entries) => new Map(Object.entries(entries));
const shape = (head, base = {}) => checkShape(tree(head), tree(base));

// An ADR's text: its front matter's lines, then an H1 and a body.
function adr(front, h1 = "# ADR-0300: A short title", body = "**In brief**\n- **Decision.** One.\n") {
  return ["---", ...front, "---", "", h1, "", body].join("\n");
}
const SHAPED = ["status: proposed", "date: 2026-10-20", "title: A short title", "kind: decision", "summary: It decides one thing."];

test("the limits are ADR-0197's, and the kinds its five", () => {
  assert.deepEqual(LIMITS, { block: 1200, cell: 400, indexCell: 300, title: 80, summary: 300, statusLines: 60, changelogEntry: 400 });
  assert.deepEqual(KINDS, ["decision", "protocol", "built", "measured", "process"]);
});

test("a paragraph or a list item passes at 1 200 characters and fails at 1 201", () => {
  assert.deepEqual(shape({ [WP]: `${words(1200)}\n` }), []);
  assert.deepEqual(shape({ [WP]: `${words(1201)}\n` }), [`${WP}:1: a paragraph of 1201 characters, over 1200`]);
  assert.deepEqual(shape({ [WP]: `- ${words(1200)}\n` }), []);
  assert.deepEqual(shape({ [WP]: `text\n\n- [x] ${words(1201)}\n` }), [`${WP}:3: a list item of 1201 characters, over 1200`]);
});

test("a paragraph's lines are one block, measured joined by a space", () => {
  assert.deepEqual(shape({ "CLAUDE.md": `${words(600)}\n${words(599)}\n` }), []);
  assert.deepEqual(shape({ "CLAUDE.md": `${words(600)}\n${words(600)}\n` }), ["CLAUDE.md:1: a paragraph of 1201 characters, over 1200"]);
});

test("a table cell passes at 400 and fails at 401; the index's at 300 and 301", () => {
  const table = (n) => `| A | B |\n| :--- | :--- |\n| ${words(n)} | b |\n`;
  assert.deepEqual(shape({ "README.md": table(400) }), []);
  assert.deepEqual(shape({ "README.md": table(401) }), ["README.md:3: a table cell of 401 characters, over 400"]);
  assert.deepEqual(shape({ "docs/adr/README.md": table(300) }), []);
  assert.deepEqual(shape({ "docs/adr/README.md": table(301) }), ["docs/adr/README.md:3: a table cell of 301 characters, over 300"]);
});

test("docs/STATUS.md passes at 60 lines and fails at 61", () => {
  const lines = (n) => `${Array.from({ length: n }, (_, i) => `line ${i}`).join("\n\n").split("\n").slice(0, n).join("\n")}\n`;
  assert.deepEqual(shape({ "docs/STATUS.md": lines(60) }), []);
  assert.deepEqual(shape({ "docs/STATUS.md": lines(61) }), ["docs/STATUS.md: 61 lines, over 60"]);
});

test("a new changelog entry passes at 400 characters and fails at 401", () => {
  const log = (n) => `# Changelog\n\nPreamble ${words(2000)}\n\n## [Unreleased]\n\n### Added\n\n- ${words(n)}\n`;
  assert.deepEqual(shape({ "CHANGELOG.md": log(400) }), []);
  assert.deepEqual(shape({ "CHANGELOG.md": log(401) }), ["CHANGELOG.md:9: a new entry of 401 characters, over 400"]);
});

test("a changelog entry counts the items under it, and the base's entries are not read", () => {
  const old = `- **Old.** ${words(3000)}\n  - its detail ${words(500)}\n`;
  const base = { "CHANGELOG.md": `## [Unreleased]\n\n### Added\n\n${old}` };
  assert.deepEqual(shape({ "CHANGELOG.md": `## [Unreleased]\n\n### Added\n\n- New, short.\n${old}` }, base), []);
  const grown = `## [Unreleased]\n\n### Added\n\n${old}  - one more line\n`;
  assert.equal(shape({ "CHANGELOG.md": grown }, base).length, 1);
  assert.equal(changelogEntries(`## [1]\n\n- a\n  - b\n- c\n`).length, 2);
});

test("a block unchanged from the base passes whatever its length", () => {
  const long = `${words(5000)}\n\n| ${words(900)} | b |\n`;
  assert.deepEqual(shape({ [WP]: long }, { [WP]: long }), []);
});

test("text moved verbatim to another document passes, its relative links rewritten for its place", () => {
  const sentence = `One sentence ([ADR-0079](adr/0079-x.md), [§11](#11-risks)) ${words(1500)}.`;
  const base = { [WP]: `${sentence} The rest of the paragraph stays.\n` };
  const moved = sentence.replace("](adr/", "](../adr/").replace("](#11-risks)", "](../WHITEPAPER.md#11-risks)");
  assert.deepEqual(shape({ "docs/research/H-13.md": `${moved}\n`, [WP]: "The rest of the paragraph stays.\n" }, base), []);
  const elsewhere = sentence.replace("](adr/", "](../other/");
  assert.equal(shape({ "docs/research/H-13.md": `${elsewhere}\n` }, base).length, 1);
});

test("a block that joins two of the base's, or adds to one, is new text", () => {
  const base = { [WP]: `${words(700)}\n\n${"y".repeat(700)}\n` };
  assert.equal(shape({ [WP]: `${words(700)} ${"y".repeat(700)}\n` }, base).length, 1);
  assert.equal(shape({ [WP]: `${words(700)} and more ${"z".repeat(600)}\n` }, base).length, 1);
});

test("code blocks, headings and directive comments are not read", () => {
  const text = [
    "```bash",
    words(5000),
    "```",
    `# ${words(2000)}`,
    `<!-- @assert-count target="crates" symbol="${words(3000)}" min="1" -->`,
    "<!--",
    words(3000),
    "-->",
    `short text <!-- ${words(3000)} -->`,
  ].join("\n");
  assert.deepEqual(shape({ [WP]: text }), []);
});

test("an archived brief and a document that is not living are not read", () => {
  assert.deepEqual(shape({ "briefs/archive/070_x.md": words(5000), "docs/explorations/001-x.md": words(5000) }), []);
  assert.equal(shape({ "briefs/078_x.md": words(1201) }).length, 1);
  assert.equal(shape({ "docs/zh-TW/README.md": "字".repeat(1201) }).length, 1);
});

test("an accepted ADR's body changed by one byte fails", () => {
  const path = "docs/adr/0100-x.md";
  const before = adr(["status: accepted", "date: 2026-09-01", ...SHAPED.slice(2)], "# ADR-0100: Old", "Body.\n");
  assert.deepEqual(shape({ [path]: before }, { [path]: before }), []);
  const after = before.replace("Body.", "Body!");
  assert.deepEqual(shape({ [path]: after }, { [path]: before }), [`${path}: the body of an accepted ADR differs from its base copy`]);
});

test("an accepted ADR's front matter changes only in its lifecycle's keys", () => {
  const path = "docs/adr/0100-x.md";
  const front = ["status: accepted", "date: 2026-09-01", "title: Old", "kind: decision", "summary: Old one."];
  const before = adr(front, "# ADR-0100: Old", "Body.\n");
  const superseded = adr(["status: superseded", ...front.slice(1), "superseded-by: ADR-0200"], "# ADR-0100: Old", "Body.\n");
  assert.deepEqual(shape({ [path]: superseded }, { [path]: before }), []);
  const retitled = before.replace("title: Old", "title: New");
  assert.deepEqual(shape({ [path]: retitled }, { [path]: before }), [
    `${path}: the front matter's title: changed on an accepted ADR, which only its lifecycle may change`,
  ]);
});

test("an accepted ADR with no title on the base takes ADR-0197's four keys once, within their limits", () => {
  const path = "docs/adr/0100-x.md";
  const before = adr(["status: accepted", "date: 2026-09-01"], "# ADR-0100: A long old heading — and more", "Body.\n");
  const completed = adr(
    ["status: accepted", "date: 2026-09-01", "title: A long old heading", "kind: measured", "hypothesis: H-13", "summary: 'H-13 is yes: it holds.'"],
    "# ADR-0100: A long old heading — and more",
    "Body.\n",
  );
  assert.deepEqual(shape({ [path]: completed }, { [path]: before }), []);
  const wrong = completed.replace("kind: measured", "kind: result").replace("title: A long", `title: ${words(80)}`);
  assert.deepEqual(shape({ [path]: wrong }, { [path]: before }), [
    `${path}: the title is 92 characters, over 80`,
    `${path}: kind: result is not one of decision, protocol, built, measured, process`,
  ]);
  const extra = completed.replace("date: 2026-09-01", "date: 2026-10-20");
  assert.equal(shape({ [path]: extra }, { [path]: before }).length, 1);
});

test("a new ADR carries a title within 80, a summary within 300, a kind, its H1 and In brief", () => {
  const path = "docs/adr/0300-a-short-title.md";
  assert.deepEqual(shape({ [path]: adr(SHAPED) }), []);
  assert.deepEqual(shape({ [path]: adr(SHAPED.map((l) => (l.startsWith("summary") ? `summary: ${words(300)}` : l))) }), []);
  assert.deepEqual(shape({ [path]: adr(SHAPED.map((l) => (l.startsWith("summary") ? `summary: ${words(301)}` : l))) }), [
    `${path}: the summary is 301 characters, over 300`,
  ]);
  assert.deepEqual(shape({ [path]: adr(SHAPED, "# ADR-0300: Another title") }), [`${path}: the H1 is not '# ADR-0300: ' and the title`]);
  assert.deepEqual(shape({ [path]: adr(SHAPED, undefined, "Context first.\n") }), [`${path}: the body does not open with **In brief**`]);
  assert.equal(shape({ [path]: adr(SHAPED.slice(0, 2)) }).length, 3);
  assert.equal(shape({ [path]: adr(SHAPED, undefined, `**In brief**\n\n${words(1201)}\n`) }).length, 1);
});

test("an ADR proposed on the base is still a new one", () => {
  const path = "docs/adr/0300-a-short-title.md";
  const before = adr(SHAPED, "# ADR-0300: A short title", "**In brief**\n- Old.\n");
  assert.deepEqual(shape({ [path]: adr(SHAPED) }, { [path]: before }), []);
});

test("blocks: an item's continuation lines are the item, a paragraph under it after a blank line is its own", () => {
  const b = blocks("- [x] **H-1** one\n  two\n\n      **Its rule:** three\n- next\n");
  assert.deepEqual(b.map((x) => [x.kind, x.line, x.text]), [
    ["item", 1, "**H-1** one two"],
    ["paragraph", 4, "**Its rule:** three"],
    ["item", 5, "next"],
  ]);
});

test("cells split at the pipes that are not escaped, and the delimiter row is not a cell", () => {
  assert.deepEqual(cells("| a \\| b | `c` |"), ["a \\| b", "`c`"]);
  assert.deepEqual(blocks("| A | B |\n| :--- | ---: |\n| 1 | 2 |\n").map((x) => x.text), ["A", "B", "1", "2"]);
});

test("normalise resolves a relative link and an anchor to the root, and leaves a URL", () => {
  assert.equal(normalise("[a](../adr/x.md#s) [b](#c) [d](https://e.org/f)", "docs/research/H-1.md"), "[a](/docs/adr/x.md#s) [b](/docs/research/H-1.md#c) [d](https://e.org/f)");
});

test("a base git cannot read fails the check", () => {
  const run = spawnSync(process.execPath, ["scripts/check-doc-shape.mjs"], {
    env: { ...process.env, SPEC_SHAPE_BASE: "refs/heads/no-such-base-for-the-shape-check" },
    encoding: "utf8",
  });
  assert.equal(run.status, 1);
  assert.match(run.stderr, /is not a commit git can read/);
});
