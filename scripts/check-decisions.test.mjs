// Tests of scripts/check-decisions.mjs under node:test, which ships with Node (stable since 20;
// this repository requires 22), so the checker gains no dependency. Each case writes a docs
// tree of its own into a temporary folder and removes it afterwards.

import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

import { ROW, checkDecisions } from "./check-decisions.mjs";

const ADRS = ["0001-first.md", "0002-second.md"];

// A docs tree with the ADR files, a whitepaper and an index carrying the rows of `rows`.
function check({ whitepaper = ADRS, index = ADRS, extra = [] } = {}) {
  const dir = mkdtempSync(join(tmpdir(), "decisions-"));
  try {
    mkdirSync(join(dir, "adr"));
    for (const f of [...ADRS, ...extra]) writeFileSync(join(dir, "adr", f), "---\nstatus: accepted\n---\n");
    const table = (kind, files) => files.map((f) => `${ROW[kind](f)} a title |`).join("\n");
    writeFileSync(join(dir, "WHITEPAPER.md"), `# W\n\n| ID | Title |\n| :--- | :--- |\n${table("whitepaper", whitepaper)}\n`);
    writeFileSync(join(dir, "adr", "README.md"), `# I\n\n${table("index", index)}\n`);
    return checkDecisions(join(dir, "adr"), join(dir, "WHITEPAPER.md"), join(dir, "adr", "README.md"));
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

test("the rows are the documents' own link shapes", () => {
  assert.equal(ROW.whitepaper("0062-x.md"), "| [ADR-0062](adr/0062-x.md) |");
  assert.equal(ROW.index("0062-x.md"), "| [ADR-0062](0062-x.md) |");
});

test("every ADR with both rows passes", () => {
  assert.deepEqual(check(), { adrs: 2, problems: [] });
});

test("an ADR without its whitepaper row fails, and the message names the ADR and the row", () => {
  const { problems } = check({ whitepaper: ["0001-first.md"] });
  assert.equal(problems.length, 1);
  assert.match(problems[0], /WHITEPAPER\.md: no row for .*0002-second\.md \(a line beginning '\| \[ADR-0002\]\(adr\/0002-second\.md\) \|'\)/);
});

test("an ADR without its index row fails on its own", () => {
  const { problems } = check({ index: ["0002-second.md"] });
  assert.equal(problems.length, 1);
  assert.match(problems[0], /README\.md: no row for .*0001-first\.md/);
});

test("a new ADR file with no row anywhere is two problems, one per document", () => {
  const { adrs, problems } = check({ extra: ["0003-third.md"] });
  assert.equal(adrs, 3);
  assert.equal(problems.length, 2);
});

test("a row that only mentions the ADR mid-line does not count", () => {
  const dir = mkdtempSync(join(tmpdir(), "decisions-"));
  try {
    mkdirSync(join(dir, "adr"));
    writeFileSync(join(dir, "adr", "0001-first.md"), "");
    writeFileSync(join(dir, "WHITEPAPER.md"), "| F-1 | see [ADR-0001](adr/0001-first.md) | x |\n");
    writeFileSync(join(dir, "adr", "README.md"), "| [ADR-0001](0001-first.md) | t |\n");
    const { problems } = checkDecisions(join(dir, "adr"), join(dir, "WHITEPAPER.md"), join(dir, "adr", "README.md"));
    assert.equal(problems.length, 1);
    assert.match(problems[0], /WHITEPAPER\.md/);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});
