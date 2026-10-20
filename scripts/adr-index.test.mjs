// Tests of scripts/adr-index.mjs under node:test (stable since Node 20; this repository requires
// 22). The ADRs, the reports and the documents are text in memory, so no case touches a file.

import assert from "node:assert/strict";
import { test } from "node:test";

import { CELL_LIMIT, adrCell, between, generate, list, marker, readAdrs, readReports } from "./adr-index.mjs";

const adr = (front) => ["---", ...front, "---", "", "# ADR-0000: A long old heading", "", "Body."].join("\n");
const ADRS = [
  ["0078-the-rewards-direction.md", adr(["status: accepted", "title: The reward's direction", "kind: protocol", "hypothesis: H-13", "summary: 'H-13 written: a reward raises its response.'"])],
  ["0079-the-rewards-direction-measured.md", adr(["status: accepted", "title: The reward's direction, measured", "kind: measured", "hypothesis: [H-13]", "summary: H-13 is yes."])],
  ["0080-a-pipe.md", adr(["status: proposed", "title: A | pipe", "kind: decision", "summary: Nothing to read."])],
];
const report = (front) => ["---", ...front, "---", "", "# H-13", ""].join("\n");
const REPORTS = [["H-13.md", report(["hypothesis: H-13", "question: a reward raises the response it is delivered to", "verdict: yes"])]];
const empty = (name) => `before\n${marker(name)}\nold row\n${marker(name, true)}\nafter\n`;
const DOCUMENTS = new Map([
  ["docs/adr/README.md", empty("index")],
  ["docs/WHITEPAPER.md", `${empty("decisions")}${empty("hypotheses")}`],
  ["docs/research/README.md", empty("hypotheses")],
]);
const run = (adrFiles = ADRS, reportFiles = REPORTS) => generate({ adrFiles, reportFiles, documents: DOCUMENTS });

test("a front matter list is inline, a scalar or nothing", () => {
  assert.deepEqual(list("[H-12, 'H-13']"), ["H-12", "H-13"]);
  assert.deepEqual(list("H-13"), ["H-13"]);
  assert.deepEqual(list(undefined), []);
  assert.deepEqual(list("null"), []);
});

test("the index carries ID, title, kind, H-n, summary and status, a pipe escaped and a report linked", () => {
  const { documents, problems } = run();
  assert.deepEqual(problems, []);
  const index = documents.get("docs/adr/README.md").split("\n");
  assert.deepEqual(index.slice(1, 7), [
    marker("index"),
    "| ID | Title | Kind | H-n | Summary | Status |",
    "| :--- | :--- | :--- | :--- | :--- | :--- |",
    "| [ADR-0078](0078-the-rewards-direction.md) | The reward's direction | protocol | [H-13](../research/H-13.md) | H-13 written: a reward raises its response. | accepted |",
    "| [ADR-0079](0079-the-rewards-direction-measured.md) | The reward's direction, measured | measured | [H-13](../research/H-13.md) | H-13 is yes. | accepted |",
    "| [ADR-0080](0080-a-pipe.md) | A \\| pipe | decision |  | Nothing to read. | proposed |",
  ]);
  assert.equal(index[7], marker("index", true));
  assert.equal(index[0], "before");
  assert.equal(index[8], "after");
});

test("§9 carries ID, title and status, linked from docs/, as spec:decisions reads a row", () => {
  const wp = run().documents.get("docs/WHITEPAPER.md").split("\n");
  assert.ok(wp.includes("| [ADR-0078](adr/0078-the-rewards-direction.md) | The reward's direction | accepted |"));
  assert.ok(!wp.includes("old row"));
});

test("the hypotheses' table is written twice, each linked from where it stands", () => {
  const { documents } = run();
  const both = "[ADR-0078](ADR/0078-the-rewards-direction.md), [ADR-0079](ADR/0079-the-rewards-direction-measured.md)";
  assert.ok(
    documents
      .get("docs/research/README.md")
      .includes(`| H-13 | a reward raises the response it is delivered to | yes | ${both.replaceAll("ADR/", "../adr/")} | [H-13](H-13.md) |`),
  );
  assert.ok(
    documents
      .get("docs/WHITEPAPER.md")
      .includes(`| H-13 | a reward raises the response it is delivered to | yes | ${both.replaceAll("ADR/", "adr/")} | [H-13](research/H-13.md) |`),
  );
});

test("an ADR cell past 400 characters gives the first, the last and the count", () => {
  const many = Array.from({ length: 12 }, (_, i) => ({ id: `ADR-01${String(i).padStart(2, "0")}`, file: `01${String(i).padStart(2, "0")}-a-long-file-name-for-a-row.md`, hypotheses: ["H-12"] }));
  const cell = adrCell(many, "H-12", "../adr/");
  assert.ok([...cell].length <= CELL_LIMIT);
  assert.equal(cell, "[ADR-0100](../adr/0100-a-long-file-name-for-a-row.md) to [ADR-0111](../adr/0111-a-long-file-name-for-a-row.md), 12 ADRs");
  assert.equal(adrCell(many.slice(0, 2), "H-12", ""), "[ADR-0100](0100-a-long-file-name-for-a-row.md), [ADR-0101](0101-a-long-file-name-for-a-row.md)");
});

test("an ADR without a title, a summary, a kind or a status is a problem by name", () => {
  const { problems } = readAdrs([["0081-x.md", adr(["status: accepted"])]]);
  assert.deepEqual(problems, [
    "docs/adr/0081-x.md: no title: in the front matter",
    "docs/adr/0081-x.md: no summary: in the front matter",
    "docs/adr/0081-x.md: no kind: in the front matter",
  ]);
});

test("a report's question is at most 160 characters, its verdict yes, no or open, and its hypothesis its own", () => {
  const ok = readReports([["H-2.md", report(["hypothesis: H-2", `question: ${"q".repeat(160)}`, "verdict: open"])]]);
  assert.deepEqual(ok.problems, []);
  const bad = readReports([["H-2.md", report(["hypothesis: H-3", `question: ${"q".repeat(161)}`, "verdict: maybe"])]]);
  assert.deepEqual(bad.problems, [
    "docs/research/H-2.md: hypothesis: is not H-2",
    "docs/research/H-2.md: the question is 161 characters, over 160",
    "docs/research/H-2.md: verdict: maybe is not one of yes, no, open",
  ]);
});

test("the reports are ordered by number, not by name", () => {
  const files = [10, 9, 2].map((n) => [`H-${n}.md`, report([`hypothesis: H-${n}`, "question: q", "verdict: open"])]);
  assert.deepEqual(readReports(files).reports.map((r) => r.id), ["H-2", "H-9", "H-10"]);
});

test("a document without its markers, or with them twice, is a problem and is not written", () => {
  assert.throws(() => between("no markers\n", "index", []), /are not there once each/);
  assert.throws(() => between(`${empty("index")}${empty("index")}`, "index", []), /are not there once each/);
  const documents = new Map(DOCUMENTS);
  documents.set("docs/research/README.md", "nothing\n");
  const { problems } = generate({ adrFiles: ADRS, reportFiles: REPORTS, documents });
  assert.equal(problems.length, 1);
  assert.match(problems[0], /^docs\/research\/README\.md: the markers/);
});

test("writing twice is writing once", () => {
  const first = run().documents;
  const second = generate({ adrFiles: ADRS, reportFiles: REPORTS, documents: first }).documents;
  assert.deepEqual(second, first);
});
