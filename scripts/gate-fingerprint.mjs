#!/usr/bin/env node
// The tree the gate's Rust jobs test, as one hash (ADR-0223). A pull request's gate runs its four
// Rust jobs on every push, and a push that changes no file they read runs them for nothing: brief
// 082's last commit changed documents and the cost table, and the gate ran again for 51 minutes
// before its merge. The jobs test the pull request merged into its base, and the mutation gate
// mutates the lines between the two, so what they read is two trees: the base's and the merge's.
// Two runs on the same two trees, as far as these jobs read them, run the same jobs on the same
// input, and the second learns nothing the first did not.
//
//   node scripts/gate-fingerprint.mjs <base> <head>
//
// Prints the SHA-256 of what `git ls-tree -r -z` lists of each tree for the paths the jobs read: the
// paths a Rust build reads (`readByRust`, ADR-0215), the workflow, which holds the jobs' commands
// and the tools' pins, and `.gitattributes`, which decides the files' line endings. A mode, a blob
// or a path that changes changes the hash. `ci.yml`'s `changes` job reads it, and skips the Rust
// jobs when a green gate of this repository has already kept it. Exit 0 with the hash, 2 on a usage
// or git error. The exports are for scripts/gate-fingerprint.test.mjs.

import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { readByRust } from "./check-additive.mjs";

/** Whether one of the gate's Rust jobs reads the path. */
export function readByGate(path) {
  return readByRust(path) || path === ".github/workflows/ci.yml" || path === ".gitattributes";
}

/** The entries of `git ls-tree -r -z` output, each `<mode> <type> <object>\t<path>`, that the jobs read. */
export function gateLines(listing) {
  return listing
    .split("\0")
    .filter((entry) => entry.includes("\t") && readByGate(entry.slice(entry.indexOf("\t") + 1)));
}

/** The hash of the two listings, the base's first: neither tree can stand in for the other. */
export function fingerprint(baseListing, headListing) {
  const text = ["base", ...gateLines(baseListing), "head", ...gateLines(headListing), ""].join("\n");
  return createHash("sha256").update(text).digest("hex");
}

function git(args) {
  return execFileSync("git", args, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], maxBuffer: 1 << 30 });
}

function main(argv) {
  const [base, head] = argv;
  if (!base || !head) {
    console.error("usage: node scripts/gate-fingerprint.mjs <base> <head>");
    return 2;
  }
  let listings;
  try {
    listings = [base, head].map((rev) => git(["ls-tree", "-r", "-z", "--full-tree", rev]));
  } catch (e) {
    console.error(`gate-fingerprint: git cannot read ${base} and ${head}: ${e.message.split("\n")[0]}`);
    return 2;
  }
  console.log(fingerprint(...listings));
  return 0;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) process.exitCode = main(process.argv.slice(2));
