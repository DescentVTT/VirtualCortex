#!/usr/bin/env node
// Every live brief carries its mandatory sections (briefs/README.md).
// Zero dependencies; runs the same way locally and in CI. Exit 1 with a
// file and the missing section on failure, 0 otherwise.

import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const DIR = "briefs";
const REQUIRED = [
  "## Mission",
  "## Standing directives",
  "## Context",
  "## Deliverables",
  "## Not empowered",
  "## Architectural empowerment",
  "## Verification",
  "## Report",
];

const live = readdirSync(DIR).filter((f) => /^\d{3}_.+\.md$/.test(f)).sort();
let failures = 0;

for (const file of live) {
  const path = join(DIR, file);
  const text = readFileSync(path, "utf8");
  const lines = text.split(/\r?\n/);
  const missing = REQUIRED.filter((h) => !lines.some((l) => l.trim() === h));
  // Front matter: a leading `---` line, then key: value lines, then a closing `---` line.
  const close = lines[0] === "---" ? lines.indexOf("---", 1) : -1;
  const frontMatter = close > 0 ? lines.slice(1, close) : [];
  if (!frontMatter.some((l) => /^status:\s*proposed\s*$/.test(l))) {
    console.error(`${path}: live brief must declare 'status: proposed' in front matter`);
    failures++;
  }
  for (const h of missing) {
    console.error(`${path}: missing section '${h}'`);
    failures++;
  }
}

if (failures > 0) {
  console.error(`check-briefs: ${failures} problem(s) in ${live.length} live brief(s)`);
  process.exit(1);
}
console.log(`check-briefs: ${live.length} live brief(s), all carry the ${REQUIRED.length} mandatory sections`);
