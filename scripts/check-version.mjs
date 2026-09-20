#!/usr/bin/env node
// The whitepaper declares its version and its date twice -- in the front matter and in the
// Document control table -- and a round that bumps one and not the other leaves the document
// disagreeing with itself: 4.15.0 in the front matter and 4.14.0 in the table stood for a day
// and a half in September 2026, found by a reader rather than by a check (F-43). This makes the
// two declarations a check, and, on a pull request that changes the document, holds the version
// to having moved: a living document whose content changes under a version that does not is a
// version that says nothing.
//
// Usage: node scripts/check-version.mjs [<whitepaper>] [--base <the base's copy of it>]
// The base file is what the branch is merging into, written out by CI with `git show`; without
// it only the agreement is checked, which is what a local `npm run spec` does.
// Zero dependencies. Exit 1 with the file and the line on failure, 0 otherwise.
// The exports are for scripts/check-version.test.mjs.

import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const SEMVER = /^(\d+)\.(\d+)\.(\d+)$/;

// The document's four declarations, each with the line it is on (1-based), or null.
export function declarations(text) {
  const lines = text.split(/\r?\n/);
  const find = (re) => {
    const i = lines.findIndex((l) => re.test(l));
    return i < 0 ? null : { value: re.exec(lines[i])[1].trim(), line: i + 1 };
  };
  const end = lines.indexOf("---", 1);
  const front = lines.slice(0, end < 0 ? 0 : end + 1).join("\n");
  const inFront = (re) => {
    const i = front.split("\n").findIndex((l) => re.test(l));
    return i < 0 ? null : { value: re.exec(front.split("\n")[i])[1].trim(), line: i + 1 };
  };
  return {
    frontVersion: inFront(/^version:\s*(.+)$/),
    frontDate: inFront(/^date:\s*(.+)$/),
    tableVersion: find(/^\|\s*Version\s*\|\s*([^|]+)\|/),
    tableDate: find(/^\|\s*Date\s*\|\s*([^|]+)\|/),
  };
}

// -1, 0 or 1 for two SemVer strings; null when either is not one.
export function compare(a, b) {
  const x = SEMVER.exec(a ?? "");
  const y = SEMVER.exec(b ?? "");
  if (x === null || y === null) return null;
  for (let i = 1; i <= 3; i += 1) {
    const d = Number(x[i]) - Number(y[i]);
    if (d !== 0) return d < 0 ? -1 : 1;
  }
  return 0;
}

// The front matter and the Document control table agree, and the version is a SemVer triple.
export function checkVersion(text, path = "docs/WHITEPAPER.md") {
  const d = declarations(text);
  const problems = [];
  for (const [name, found] of Object.entries(d)) {
    if (found === null) problems.push(`${path}: no ${name} declaration`);
  }
  if (problems.length > 0) return { problems, version: null };
  if (!SEMVER.test(d.frontVersion.value)) {
    problems.push(`${path}:${d.frontVersion.line}: version '${d.frontVersion.value}' is not MAJOR.MINOR.PATCH`);
  }
  if (d.frontVersion.value !== d.tableVersion.value) {
    problems.push(
      `${path}:${d.tableVersion.line}: the Document control table says version ${d.tableVersion.value} and ` +
        `the front matter (line ${d.frontVersion.line}) says ${d.frontVersion.value}; the document declares ` +
        `its version twice and both move together`,
    );
  }
  if (d.frontDate.value !== d.tableDate.value) {
    problems.push(
      `${path}:${d.tableDate.line}: the Document control table says date ${d.tableDate.value} and the front ` +
        `matter (line ${d.frontDate.line}) says ${d.frontDate.value}; the date moves with the version`,
    );
  }
  return { problems, version: d.frontVersion.value };
}

// A pull request that changes the document moves its version forward. Whitespace-only and
// no-op changes are changes: the comparison is the file's bytes, as git sees them.
export function checkBump(text, baseText, path = "docs/WHITEPAPER.md") {
  if (baseText === null || baseText === text) return { problems: [], changed: false };
  const here = checkVersion(text, path).version;
  const there = checkVersion(baseText, path).version;
  const order = compare(here, there);
  if (order === null) return { problems: [], changed: true };
  if (order > 0) return { problems: [], changed: true };
  return {
    changed: true,
    problems: [
      `${path}: this change edits the whitepaper but leaves its version at ${here} (the base is ${there}); ` +
        `a correction that changes no claim is a patch, new or changed content is a minor, and the date ` +
        `moves with it (CONTRIBUTING.md, ADR-0064)`,
    ],
  };
}

// `[<whitepaper>] [--base <file>]` in either order. `--base` takes the argument after it, and
// that argument is not the document: with no `--base` nothing is skipped, which is the local run.
export function parseArgs(argv) {
  const at = argv.indexOf("--base");
  const skip = at < 0 ? -1 : at + 1;
  return {
    base: at < 0 ? null : (argv[at + 1] ?? null),
    path: argv.filter((_, i) => i !== at && i !== skip)[0] ?? "docs/WHITEPAPER.md",
  };
}

// Run as a script, not when imported by the tests. Compared without case: Windows paths are.
const invoked =
  process.argv[1] !== undefined &&
  resolve(process.argv[1]).toLowerCase() === fileURLToPath(import.meta.url).toLowerCase();

if (invoked) {
  const { base: basePath, path } = parseArgs(process.argv.slice(2));
  if (!existsSync(path)) {
    console.error(`check-version: ${path} not found`);
    process.exit(1);
  }
  const text = readFileSync(path, "utf8");
  const { problems, version } = checkVersion(text, path);
  const base = basePath !== null && existsSync(basePath) ? readFileSync(basePath, "utf8") : null;
  const bump = checkBump(text, base, path);
  const all = problems.concat(bump.problems);
  for (const p of all) console.error(p);
  if (all.length > 0) {
    console.error(`check-version: ${all.length} problem(s) in ${path}`);
    process.exit(1);
  }
  const against =
    base === null
      ? "its two declarations agree"
      : bump.changed
        ? "its two declarations agree and the version moved with the change"
        : "its two declarations agree and the document is unchanged on this branch";
  console.log(`check-version: ${path} at ${version}, ${against}`);
}
