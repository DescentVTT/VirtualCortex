#!/usr/bin/env node
// The tables that list the decisions and the hypotheses, written from front matter (ADR-0197).
// Hand-written, the index and §9 drifted from the ADRs they list (F-40, F-41) and from each
// other, and a title that was the whole conclusion made both unreadable. Each ADR now carries a
// short `title:`, a `summary:`, a `kind:` and, where it has one, a `hypothesis:`; each report in
// docs/research/ its `hypothesis:`, `question:` and `verdict:`. This writes, between markers:
//
// - docs/adr/README.md's index: ID, title, kind, H-n, summary, status;
// - the whitepaper's §9: ID, title, status;
// - docs/research/README.md's table and the whitepaper's §11.1: H-n, question, verdict, the ADRs
//   whose `hypothesis:` names it, the report.
//
// Usage: node scripts/adr-index.mjs [--check]. Without `--check` it writes the tables; with it,
// it writes nothing and fails when a committed table is not what it would write, or when an ADR
// or a report lacks what its row needs. `spec:decisions` still asks for every ADR's row. Zero
// dependencies. The exports are for scripts/adr-index.test.mjs.

import { existsSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { frontKeys, scalar, splitFrontMatter } from "./check-doc-shape.mjs";

export const ADR_FILE = /^(\d{4})-.+\.md$/;
export const REPORT_FILE = /^H-(\d+)\.md$/;
export const VERDICTS = ["yes", "no", "open"];
export const QUESTION_LIMIT = 160;
export const CELL_LIMIT = 400;

/** The markers a table is written between, by its name. */
export const marker = (name, end = false) => `<!-- adr-index: ${end ? "end" : "begin"} ${name} -->`;

/** A front matter value as a list: an inline `[a, b]`, a scalar, or nothing. */
export function list(raw) {
  if (raw === undefined) return [];
  const v = raw.trim();
  if (v === "" || v === "null" || v === "~") return [];
  if (v.startsWith("[") && v.endsWith("]")) {
    return v
      .slice(1, -1)
      .split(",")
      .map((s) => scalar(s.trim()))
      .filter((s) => s !== "");
  }
  return [scalar(v)];
}

const escape = (text) => text.replace(/(?<!\\)\|/g, "\\|");

/** Each ADR's row data from its file name and text; problems for what a row needs and lacks. */
export function readAdrs(files) {
  const adrs = [];
  const problems = [];
  for (const [file, text] of files) {
    const m = ADR_FILE.exec(file);
    if (m === null) continue;
    const keys = frontKeys(splitFrontMatter(text).front);
    const adr = {
      id: `ADR-${m[1]}`,
      file,
      title: scalar(keys.get("title")),
      summary: scalar(keys.get("summary")),
      kind: scalar(keys.get("kind")),
      status: scalar(keys.get("status")),
      hypotheses: list(keys.get("hypothesis")),
    };
    for (const key of ["title", "summary", "kind", "status"]) {
      if (adr[key] === undefined || adr[key] === "") problems.push(`docs/adr/${file}: no ${key}: in the front matter`);
    }
    adrs.push(adr);
  }
  adrs.sort((a, b) => a.id.localeCompare(b.id));
  return { adrs, problems };
}

/** Each report's row data; problems for a missing key, a question past 160, a verdict not known. */
export function readReports(files) {
  const reports = [];
  const problems = [];
  for (const [file, text] of files) {
    const m = REPORT_FILE.exec(file);
    if (m === null) continue;
    const keys = frontKeys(splitFrontMatter(text).front);
    const report = {
      n: Number(m[1]),
      id: `H-${m[1]}`,
      file,
      hypothesis: scalar(keys.get("hypothesis")),
      question: scalar(keys.get("question")),
      verdict: scalar(keys.get("verdict")),
    };
    const at = `docs/research/${file}`;
    if (report.hypothesis !== report.id) problems.push(`${at}: hypothesis: is not ${report.id}`);
    if (report.question === undefined || report.question === "") problems.push(`${at}: no question: in the front matter`);
    else if ([...report.question].length > QUESTION_LIMIT) {
      problems.push(`${at}: the question is ${[...report.question].length} characters, over ${QUESTION_LIMIT}`);
    }
    if (!VERDICTS.includes(report.verdict)) problems.push(`${at}: verdict: ${report.verdict ?? "(none)"} is not one of ${VERDICTS.join(", ")}`);
    reports.push(report);
  }
  reports.sort((a, b) => a.n - b.n);
  return { reports, problems };
}

/** The index's rows, linked from docs/adr/. */
export function indexTable(adrs, reports) {
  const reported = new Set(reports.map((r) => r.id));
  const hyp = (h) => (reported.has(h) ? `[${h}](../research/${h}.md)` : h);
  return [
    "| ID | Title | Kind | H-n | Summary | Status |",
    "| :--- | :--- | :--- | :--- | :--- | :--- |",
    ...adrs.map(
      (a) =>
        `| [${a.id}](${a.file}) | ${escape(a.title)} | ${a.kind} | ${a.hypotheses.map(hyp).join(", ")} | ${escape(a.summary)} | ${a.status} |`,
    ),
  ];
}

/** §9's rows, linked from docs/. */
export function decisionsTable(adrs) {
  return [
    "| ID | Title | Status |",
    "| :--- | :--- | :--- |",
    ...adrs.map((a) => `| [${a.id}](adr/${a.file}) | ${escape(a.title)} | ${a.status} |`),
  ];
}

/**
 * The ADRs whose `hypothesis:` names a report's, linked with `prefix`. A cell past 400 characters
 * gives the first and the last and the count, the whole list being the report's.
 */
export function adrCell(adrs, id, prefix) {
  const named = adrs.filter((a) => a.hypotheses.includes(id));
  const link = (a) => `[${a.id}](${prefix}${a.file})`;
  const all = named.map(link).join(", ");
  if ([...all].length <= CELL_LIMIT || named.length < 2) return all;
  return `${link(named[0])} to ${link(named[named.length - 1])}, ${named.length} ADRs`;
}

/** The hypotheses' rows, linked from the research directory (`""`) or from docs/ (`"research/"`). */
export function hypothesesTable(reports, adrs, from) {
  const adrPrefix = from === "" ? "../adr/" : "adr/";
  return [
    "| H-n | Question | Verdict | ADRs | Report |",
    "| :--- | :--- | :--- | :--- | :--- |",
    ...reports.map(
      (r) =>
        `| ${r.id} | ${escape(r.question)} | ${r.verdict} | ${adrCell(adrs, r.id, adrPrefix)} | [${r.id}](${from}${r.file}) |`,
    ),
  ];
}

/** A document with the lines between a table's markers replaced; throws when the markers are not there once each. */
export function between(text, name, rows) {
  const lines = text.replace(/\r\n?/g, "\n").split("\n");
  const begin = lines.indexOf(marker(name));
  const end = lines.indexOf(marker(name, true));
  if (begin < 0 || end < begin || lines.lastIndexOf(marker(name)) !== begin || lines.lastIndexOf(marker(name, true)) !== end) {
    throw new Error(`the markers '${marker(name)}' and '${marker(name, true)}' are not there once each, in order`);
  }
  return [...lines.slice(0, begin + 1), ...rows, ...lines.slice(end)].join("\n");
}

/**
 * What each document should be: `{ path: text }` for the index, the whitepaper and the research
 * README, from the ADRs and the reports given; problems for what a row lacks or a missing marker.
 */
export function generate({ adrFiles, reportFiles, documents }) {
  const { adrs, problems } = readAdrs(adrFiles);
  const read = readReports(reportFiles);
  problems.push(...read.problems);
  const { reports } = read;
  const plan = [
    ["docs/adr/README.md", "index", indexTable(adrs, reports)],
    ["docs/WHITEPAPER.md", "decisions", decisionsTable(adrs)],
    ["docs/WHITEPAPER.md", "hypotheses", hypothesesTable(reports, adrs, "research/")],
    ["docs/research/README.md", "hypotheses", hypothesesTable(reports, adrs, "")],
  ];
  const out = new Map(documents);
  for (const [path, name, rows] of plan) {
    try {
      out.set(path, between(out.get(path) ?? "", name, rows));
    } catch (error) {
      problems.push(`${path}: ${error.message}`);
    }
  }
  return { documents: out, problems, adrs: adrs.length, reports: reports.length };
}

const invoked =
  process.argv[1] !== undefined &&
  resolve(process.argv[1]).toLowerCase() === fileURLToPath(import.meta.url).toLowerCase();

if (invoked) {
  const check = process.argv.includes("--check");
  const read = (dir, re) =>
    readdirSync(dir)
      .filter((f) => re.test(f))
      .map((f) => [f, readFileSync(`${dir}/${f}`, "utf8")]);
  const paths = ["docs/adr/README.md", "docs/WHITEPAPER.md", "docs/research/README.md"];
  const documents = new Map(paths.map((p) => [p, existsSync(p) ? readFileSync(p, "utf8").replace(/\r\n?/g, "\n") : ""]));
  const reportFiles = existsSync("docs/research") ? read("docs/research", REPORT_FILE) : [];
  const result = generate({ adrFiles: read("docs/adr", ADR_FILE), reportFiles, documents });
  const stale = paths.filter((p) => result.documents.get(p) !== documents.get(p));
  if (!check && result.problems.length === 0) for (const p of stale) writeFileSync(p, result.documents.get(p));
  const problems = [...result.problems, ...(check ? stale.map((p) => `${p}: a table is not what scripts/adr-index.mjs writes; run it`) : [])];
  for (const p of problems) console.error(p);
  if (problems.length > 0) {
    console.error(`adr-index: ${problems.length} problem(s)`);
    process.exit(1);
  }
  const did = check ? "match their front matter" : stale.length > 0 ? `written in ${stale.join(", ")}` : "already current";
  console.log(`adr-index: ${result.adrs} ADRs and ${result.reports} reports; the tables ${did}`);
}
