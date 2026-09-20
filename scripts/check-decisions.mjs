#!/usr/bin/env node
// Every ADR has its row in the whitepaper's table of decisions (§9) and in docs/adr/README.md
// (F-40, F-41: twice in two rounds an ADR was accepted with its index row and no §9 row, and the
// document's own list of decisions disagreed with the tree by one; a sentence in a finding did
// not prevent the second time, so this does). spec-guard cannot count inside a document it
// checks, which is why this is a script. Zero dependencies; runs the same way locally and in
// CI. Exit 1 with the ADR and the document that lacks its row on failure, 0 otherwise.
// `checkDecisions` is exported for scripts/check-decisions.test.mjs.

import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

// The row a document must carry for an ADR file, as its line begins: the whitepaper links from
// docs/ (`adr/NNNN-title.md`), the index from docs/adr/ (`NNNN-title.md`).
export const ROW = {
  whitepaper: (file) => `| [ADR-${file.slice(0, 4)}](adr/${file}) |`,
  index: (file) => `| [ADR-${file.slice(0, 4)}](${file}) |`,
};

// The ADR files of `adrDir` (`NNNN-*.md`), and one line per row a document lacks.
export function checkDecisions(adrDir, whitepaperPath, indexPath) {
  const adrs = readdirSync(adrDir)
    .filter((f) => /^\d{4}-.+\.md$/.test(f))
    .sort();
  const problems = [];
  const documents = [
    ["whitepaper", whitepaperPath],
    ["index", indexPath],
  ];
  for (const [kind, path] of documents) {
    if (!existsSync(path)) {
      problems.push(`${path}: not found`);
      continue;
    }
    const lines = readFileSync(path, "utf8").split(/\r?\n/);
    for (const file of adrs) {
      const row = ROW[kind](file);
      if (!lines.some((l) => l.startsWith(row))) {
        problems.push(`${path}: no row for ${join(adrDir, file)} (a line beginning '${row}')`);
      }
    }
  }
  return { adrs: adrs.length, problems };
}

// Run as a script, not when imported by the tests. Compared without case: Windows paths are.
const invoked =
  process.argv[1] !== undefined &&
  resolve(process.argv[1]).toLowerCase() === fileURLToPath(import.meta.url).toLowerCase();

if (invoked) {
  const { adrs, problems } = checkDecisions("docs/adr", "docs/WHITEPAPER.md", "docs/adr/README.md");
  for (const p of problems) console.error(p);
  if (problems.length > 0) {
    console.error(`check-decisions: ${problems.length} row(s) missing for ${adrs} ADR(s)`);
    process.exit(1);
  }
  console.log(`check-decisions: ${adrs} ADRs, each with its row in the whitepaper's §9 and in docs/adr/README.md`);
}
