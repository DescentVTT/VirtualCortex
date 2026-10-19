#!/usr/bin/env node
// The exploration lane (ADR-0193). An exploration asks a question with no prediction, runs on the
// repository's host, and is written down as a note in docs/explorations/, never as evidence. Two
// things hold that line:
//
// 1. A note says what it is and where it came from: its front matter `status: exploratory`, and in
//    its body the branch, the commit and at least one dispatch run of the repository's host.
// 2. An ADR cites a note only in its "## Context and Problem Statement", as the reason a question
//    is asked. A link to docs/explorations/ anywhere else in an ADR fails, since a verdict rests on
//    runs made after a protocol was committed, and a note's runs were made before any.
//
// Zero dependencies; runs the same way locally and in CI. Exit 1 with each file and the problem on
// failure, 0 otherwise. `checkNote` and `checkAdr` are exported for scripts/check-explorations.test.mjs.

import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

// A note's file name: three digits, a slug.
export const NOTE = /^[0-9]{3}-[a-z0-9-]+\.md$/;

// What a note's body names, each on a line of its own that starts with the label.
export const BRANCH = /^\*\*Branch\*\*:\s*`[^`]+`/m;
export const COMMIT = /^\*\*Commit\*\*:\s*`[0-9a-f]{7,40}`/m;
export const RUN = /^\*\*Runs?\*\*:.*actions\/runs\/[0-9]+/m;

// A link into the notes, from an ADR: relative or rooted, with or without `docs/`.
export const LINK = /\]\((?:\.\.\/explorations\/|(?:\.\.\/)*docs\/explorations\/)[^)]*\)/g;

// The front matter's lines, or null.
function frontMatter(text) {
  const lines = text.replace(/^﻿/, "").split(/\r?\n/);
  if (lines[0] !== "---") return null;
  const close = lines.indexOf("---", 1);
  return close === -1 ? null : lines.slice(1, close);
}

export function checkNote(name, text) {
  const problems = [];
  const fm = frontMatter(text);
  if (fm === null || !fm.some((l) => /^status:\s*exploratory\s*$/.test(l))) {
    problems.push(`${name}: its front matter does not say status: exploratory`);
  }
  if (!BRANCH.test(text)) problems.push(`${name}: names no **Branch**: \`...\``);
  if (!COMMIT.test(text)) problems.push(`${name}: names no **Commit**: \`<hash>\``);
  if (!RUN.test(text)) problems.push(`${name}: names no **Runs**: line with a run of the host`);
  return problems;
}

// The ADR's links into the notes that stand outside its Context section, by line.
export function checkAdr(name, text) {
  const problems = [];
  let section = "";
  text.split(/\r?\n/).forEach((line, i) => {
    if (line.startsWith("## ")) section = line.slice(3).trim();
    if (section === "Context and Problem Statement") return;
    for (const m of line.matchAll(LINK)) {
      problems.push(`${name}:${i + 1}: cites an exploration outside its Context: ${m[0]}`);
    }
  });
  return problems;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const problems = [];
  let notes = 0;
  let readme = false;
  for (const name of readdirSync("docs/explorations").sort()) {
    if (name === "README.md") {
      readme = true;
      continue;
    }
    if (!NOTE.test(name)) {
      problems.push(`docs/explorations/${name}: not a note's name (NNN-slug.md) nor the README`);
      continue;
    }
    notes += 1;
    problems.push(...checkNote(`docs/explorations/${name}`, readFileSync(join("docs/explorations", name), "utf8")));
  }
  if (!readme) problems.push("docs/explorations/README.md is missing: the lane's rules live there");
  let adrs = 0;
  for (const name of readdirSync("docs/adr").sort()) {
    if (!/^[0-9]{4}-.*\.md$/.test(name)) continue;
    adrs += 1;
    problems.push(...checkAdr(`docs/adr/${name}`, readFileSync(join("docs/adr", name), "utf8")));
  }
  if (adrs === 0) problems.push("no ADR in docs/adr, so nothing was checked");
  if (problems.length > 0) {
    for (const p of problems) console.error(p);
    console.error(`check-explorations: ${problems.length} problem(s)`);
    process.exit(1);
  }
  console.log(`check-explorations: ${notes} note(s), each exploratory with its branch, commit and runs; ${adrs} ADR(s) cite them only as context`);
}
