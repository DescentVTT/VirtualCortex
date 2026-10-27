// Tests of scripts/gate-fingerprint.mjs under node:test (stable since Node 20; this repository
// requires 22). The listings are given in memory, as `git ls-tree -r -z` writes them, and one case
// runs the script on a repository made for it.

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

import { fingerprint, gateLines, readByGate } from "./gate-fingerprint.mjs";

const blob = (n) => n.toString(16).padStart(40, "0");
const entry = (path, n, mode = "100644") => `${mode} blob ${blob(n)}\t${path}`;
const listing = (...entries) => entries.map((e) => `${e}\0`).join("");

const SRC = "runtime/menteion-runtime/src/executor.rs";
const DOC = "docs/WHITEPAPER.md";
const TREE = [entry(SRC, 1), entry("Cargo.lock", 2), entry(".github/workflows/ci.yml", 3), entry(DOC, 4), entry("briefs/README.md", 5)];

test("the gate reads what a Rust build reads, the workflow and the line endings, and not the documents", () => {
  for (const p of [SRC, "crates/menteion-core/src/lib.rs", "testkit/prop.rs", ".cargo/mutants.toml", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "clippy.toml", ".github/workflows/ci.yml", ".gitattributes"]) {
    assert.ok(readByGate(p), p);
  }
  for (const p of [DOC, "briefs/README.md", "CHANGELOG.md", "README.md", "scripts/exhaustive-costs.tsv", "scripts/exhaustive-shard.sh", "package.json", "docs/explorations/005-x.md"]) {
    assert.ok(!readByGate(p), p);
  }
});

test("only the entries the gate reads are kept, a path with a space whole", () => {
  const spaced = entry("crates/menteion-core/src/a b.rs", 6);
  assert.deepEqual(gateLines(listing(...TREE, spaced)), [TREE[0], TREE[1], TREE[2], spaced]);
  assert.deepEqual(gateLines(""), []);
});

test("a change to a document, in the base or the merge, leaves the hash as it was", () => {
  const at = fingerprint(listing(...TREE), listing(...TREE));
  const doc = [...TREE.slice(0, 3), entry(DOC, 40), entry("briefs/new.md", 41)];
  assert.equal(fingerprint(listing(...doc), listing(...TREE)), at);
  assert.equal(fingerprint(listing(...TREE), listing(...doc)), at);
});

test("a blob, a mode or a path the gate reads that changes, in either tree, changes the hash", () => {
  const at = fingerprint(listing(...TREE), listing(...TREE));
  const changed = [
    [entry(SRC, 10), ...TREE.slice(1)],
    [entry(SRC, 1, "100755"), ...TREE.slice(1)],
    [entry("runtime/menteion-runtime/src/moved.rs", 1), ...TREE.slice(1)],
    [TREE[0], TREE[1], entry(".github/workflows/ci.yml", 30), ...TREE.slice(3)],
    [...TREE, entry("crates/menteion-core/tests/new.rs", 7)],
    TREE.slice(1),
  ];
  for (const t of changed) {
    assert.notEqual(fingerprint(listing(...t), listing(...TREE)), at, `base ${t.join(" ")}`);
    assert.notEqual(fingerprint(listing(...TREE), listing(...t)), at, `head ${t.join(" ")}`);
  }
});

test("the base and the merge cannot stand in for each other", () => {
  const other = [entry(SRC, 10), ...TREE.slice(1)];
  assert.notEqual(fingerprint(listing(...TREE), listing(...other)), fingerprint(listing(...other), listing(...TREE)));
  assert.match(fingerprint("", ""), /^[0-9a-f]{64}$/);
});

test("the script prints one hash for two commits, and fails without them", () => {
  const dir = mkdtempSync(join(tmpdir(), "gate-fingerprint-"));
  const script = join(process.cwd(), "scripts/gate-fingerprint.mjs");
  const git = (...args) => execFileSync("git", ["-c", "user.name=t", "-c", "user.email=t@t", ...args], { cwd: dir, encoding: "utf8" }).trim();
  const run = (...args) => execFileSync(process.execPath, [script, ...args], { cwd: dir, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] }).trim();
  try {
    git("init", "-q");
    mkdirSync(join(dir, "crates/menteion-core/src"), { recursive: true });
    writeFileSync(join(dir, "crates/menteion-core/src/lib.rs"), "pub fn a() {}\n");
    writeFileSync(join(dir, "README.md"), "one\n");
    git("add", ".");
    git("commit", "-q", "-m", "one");
    const one = run("HEAD", "HEAD");
    assert.match(one, /^[0-9a-f]{64}$/);
    writeFileSync(join(dir, "README.md"), "two\n");
    git("commit", "-q", "-am", "two");
    assert.equal(run("HEAD~1", "HEAD"), one, "a document changed, and the hash did not");
    writeFileSync(join(dir, "crates/menteion-core/src/lib.rs"), "pub fn b() {}\n");
    git("commit", "-q", "-am", "three");
    assert.notEqual(run("HEAD~1", "HEAD"), one, "a source changed, and so did the hash");
    assert.throws(() => run("HEAD"), (e) => e.status === 2);
    assert.throws(() => run("HEAD", "no-such-rev"), (e) => e.status === 2);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});
