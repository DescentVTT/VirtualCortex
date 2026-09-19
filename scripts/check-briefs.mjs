#!/usr/bin/env node
// Every live brief carries its mandatory sections (briefs/README.md), and from brief 028 on its
// standing directives restate Latest ≠ Newest (CLAUDE.md principle 4, whitepaper §2.1; F-39).
// Zero dependencies; runs the same way locally and in CI. Exit 1 with a file and the problem on
// failure, 0 otherwise. `checkBriefs` is exported for scripts/check-briefs.test.mjs.

import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

export const REQUIRED = [
  "## Mission",
  "## Standing directives",
  "## Context",
  "## Deliverables",
  "## Not empowered",
  "## Architectural empowerment",
  "## Verification",
  "## Report",
];

// The doctrine every brief restates among its standing directives, and the first brief held to
// it. Briefs 004 to 020 carried it; 021 to 027 did not, because a brief is written by copying
// the newest one and this script checked the heading, not what it held (F-39). The floor is 028
// because brief 027 was being executed when the check landed, and a brief under execution is
// an input nobody edits; the archived briefs are frozen and not read here.
export const DOCTRINE = "Latest ≠ Newest";
export const DOCTRINE_FROM = 28;

// The lines of a `## ` section, from its heading to the next `## ` heading or the end.
function section(lines, heading) {
  const start = lines.findIndex((l) => l.trim() === heading);
  if (start < 0) return [];
  const rest = lines.slice(start + 1);
  const end = rest.findIndex((l) => l.startsWith("## "));
  return end < 0 ? rest : rest.slice(0, end);
}

// The live briefs of `dir` (the `NNN_*.md` files directly in it) and one line per problem.
export function checkBriefs(dir) {
  const live = readdirSync(dir)
    .filter((f) => /^\d{3}_.+\.md$/.test(f))
    .sort();
  const problems = [];
  for (const file of live) {
    const path = join(dir, file);
    const lines = readFileSync(path, "utf8").split(/\r?\n/);
    // Front matter: a leading `---` line, then key: value lines, then a closing `---` line.
    const close = lines[0] === "---" ? lines.indexOf("---", 1) : -1;
    const frontMatter = close > 0 ? lines.slice(1, close) : [];
    if (!frontMatter.some((l) => /^status:\s*proposed\s*$/.test(l))) {
      problems.push(`${path}: live brief must declare 'status: proposed' in front matter`);
    }
    for (const h of REQUIRED.filter((h) => !lines.some((l) => l.trim() === h))) {
      problems.push(`${path}: missing section '${h}'`);
    }
    const number = Number.parseInt(file.slice(0, 3), 10);
    const directives = section(lines, "## Standing directives");
    if (number >= DOCTRINE_FROM && !directives.some((l) => l.includes(DOCTRINE))) {
      problems.push(
        `${path}: its standing directives must restate '${DOCTRINE}' (CLAUDE.md principle 4, whitepaper §2.1)`,
      );
    }
  }
  return { live: live.length, problems };
}

// Run as a script, not when imported by the tests. Compared without case: Windows paths are.
const invoked =
  process.argv[1] !== undefined &&
  resolve(process.argv[1]).toLowerCase() === fileURLToPath(import.meta.url).toLowerCase();

if (invoked) {
  const { live, problems } = checkBriefs("briefs");
  for (const p of problems) console.error(p);
  if (problems.length > 0) {
    console.error(`check-briefs: ${problems.length} problem(s) in ${live} live brief(s)`);
    process.exit(1);
  }
  const from = String(DOCTRINE_FROM).padStart(3, "0");
  console.log(
    `check-briefs: ${live} live brief(s), all carry the ${REQUIRED.length} mandatory sections; those from ${from} restate ${DOCTRINE}`,
  );
}
