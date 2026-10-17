#!/usr/bin/env node
// Every archived brief carries the integrity hash `spec-brief archive` writes (ADR-0185).
//
// `spec-brief lint` holds an archived brief that carries a hash to it, and fails one changed since
// (`archive-freeze`); it holds nothing of one that carries none. spec-graph failed an archived
// brief with an open box up to 0.8.0 and reads it as a record from 0.9.0, so it no longer does.
// `spec-brief archive` refuses an open box and writes the hash. A brief moved into the archive by
// hand would pass both tools; this is the check it fails. With the hash present and right, an
// archived brief is the brief it was archived as.
//
// Zero dependencies; runs the same way locally and in CI. Exit 1 with each file and the problem on
// failure, 0 otherwise. An archive with no brief in it is a failure, since a check over nothing
// reads as a clean one. `checkArchive` is exported for scripts/check-archive.test.mjs.

import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

// A brief's file name, as `.spec-brief.json`'s `files` has it.
export const BRIEF = /^[0-9]{3}_.*\.md$/;

// The field and its form, as spec-brief writes it.
export const INTEGRITY = /^integrity:[ \t]*sha256-[0-9a-f]{64}[ \t]*$/;

// The front matter's lines: from a first line `---` to the next `---`, or null when the file opens
// with no such block or never closes it.
export function frontMatter(text) {
  const lines = text.replace(/^﻿/, "").split(/\r?\n/);
  if (lines[0] !== "---") return null;
  const close = lines.indexOf("---", 1);
  return close === -1 ? null : lines.slice(1, close);
}

export function checkArchive(dir) {
  const problems = [];
  let count = 0;
  for (const name of readdirSync(dir).sort()) {
    if (!BRIEF.test(name)) continue;
    count += 1;
    const lines = frontMatter(readFileSync(join(dir, name), "utf8"));
    if (lines === null) {
      problems.push(`${name}: no front matter, so no integrity hash`);
    } else if (!lines.some((line) => INTEGRITY.test(line))) {
      problems.push(
        `${name}: no integrity hash; archive a brief with \`spec-brief archive\`, which refuses an open box and writes one`,
      );
    }
  }
  if (count === 0) problems.push(`no archived brief in ${dir}, so nothing was checked`);
  return { count, problems };
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const dir = "briefs/archive";
  const { count, problems } = checkArchive(dir);
  if (problems.length > 0) {
    for (const p of problems) console.error(`${dir}/${p}`);
    console.error(`check-archive: ${problems.length} problem(s) in ${count} archived brief(s)`);
    process.exit(1);
  }
  console.log(`check-archive: ${count} archived brief(s), each carries its integrity hash`);
}
