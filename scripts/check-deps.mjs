#!/usr/bin/env node
// TC-2 (whitepaper §2.2; ADR-0005, ADR-0014, ADR-0023, ADR-0029): state crates declare no
// dependencies of any kind; the runtime depends on state crates by path and on nothing
// else; the benchmark crate depends on workspace crates by path and carries exactly one
// dev-dependency, the harness. Read from the manifests, so it needs no cargo and runs the
// same way locally and in CI. Zero dependencies. Exit 1 with a file and the offending line
// on failure, 0 otherwise.

import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";

/** The entries of every `[…dependencies]` table in a manifest: {table, name, line, text}. */
function dependencyEntries(path) {
  const entries = [];
  let table = null;
  readFileSync(path, "utf8")
    .split(/\r?\n/)
    .forEach((raw, i) => {
      const line = raw.replace(/#.*$/, "").trim();
      if (line === "") return;
      const header = /^\[(.+)\]$/.exec(line);
      if (header) {
        table = /dependencies$/.test(header[1]) ? header[1] : null;
        return;
      }
      if (table === null) return;
      const name = /^([A-Za-z0-9_-]+)\s*=/.exec(line);
      entries.push({ table, name: name ? name[1] : line, line: i + 1, text: line });
    });
  return entries;
}

let failures = 0;
const fail = (path, entry, why) => {
  failures += 1;
  console.error(`check-deps: ${path}:${entry.line}: ${why}: ${entry.text}`);
};

// State crates: no dependency of any kind.
const stateCrates = readdirSync("crates")
  .filter((d) => existsSync(join("crates", d, "Cargo.toml")))
  .sort();
for (const crate of stateCrates) {
  const path = join("crates", crate, "Cargo.toml");
  for (const entry of dependencyEntries(path)) {
    fail(path, entry, "a state crate declares no dependencies (TC-2)");
  }
}

// The runtime: workspace crates by path, nothing else, normal dependencies only.
const runtime = join("runtime", "cortex-runtime", "Cargo.toml");
for (const entry of dependencyEntries(runtime)) {
  if (entry.table !== "dependencies") {
    fail(runtime, entry, "the runtime carries normal dependencies only (ADR-0023)");
  } else if (!/^cortex-[a-z-]+\s*=\s*\{\s*path\s*=/.test(entry.text)) {
    fail(runtime, entry, "the runtime depends on workspace crates by path and on nothing else (ADR-0023)");
  }
}

// The benchmark crate: workspace crates by path; exactly one dev-dependency, the harness.
const bench = join("benches", "cortex-bench", "Cargo.toml");
const dev = [];
for (const entry of dependencyEntries(bench)) {
  if (entry.table === "dependencies") {
    if (!/^cortex-[a-z-]+\s*=\s*\{\s*path\s*=/.test(entry.text)) {
      fail(bench, entry, "the benchmark crate's normal dependencies are workspace crates by path (ADR-0014)");
    }
  } else if (entry.table === "dev-dependencies") {
    dev.push(entry);
  } else {
    fail(bench, entry, "the benchmark crate carries normal and dev-dependencies only (ADR-0014)");
  }
}
if (dev.length !== 1 || dev[0].name !== "criterion" || !/=\s*\{\s*version\s*=\s*"=\d/.test(dev[0].text)) {
  for (const entry of dev.length ? dev : [{ line: 0, text: "(none)" }]) {
    fail(bench, entry, "the benchmark crate's one dev-dependency is criterion at an exact pin (ADR-0014)");
  }
}

if (failures > 0) {
  console.error(`check-deps: ${failures} violation(s)`);
  process.exit(1);
}
console.log(
  `check-deps: ${stateCrates.length} state crates declare no dependencies; the runtime and the benchmark crate depend on workspace crates by path (plus criterion, pinned)`,
);
