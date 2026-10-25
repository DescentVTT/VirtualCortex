// Tests of scripts/check-additive.mjs under node:test (stable since Node 20; this repository requires
// 22). The changes, the base's text and the names it holds are given in memory, so no case touches
// git or a file.

import assert from "node:assert/strict";
import { test } from "node:test";

import { checkAdditive, hunks, implType, inserted, insertsAtTop, readByRust, scan, underTests, wordsIn } from "./check-additive.mjs";

const TESTS = "runtime/cortex-runtime/tests/inhibition.rs";

// A `-U0` diff of one hunk inserting `lines` after base line `after`.
const insertion = (after, lines) => [`diff --git a/${TESTS} b/${TESTS}`, `@@ -${after},0 +${after + 1},${lines.length} @@`, ...lines.map((l) => `+${l}`)].join("\n");

const BASE = ["use super::*;", "", "fn existing() -> u32 {", "    let x = 1;", "    x", "}", "", "const TABLE: &[u32] = &[", "    1,", "    2,", "];", ""].join("\n");

const none = () => new Set();

test("a Rust build reads the crates, the runtime, the benches, the kit and the manifests, and not the documents", () => {
  for (const p of ["runtime/cortex-runtime/src/executor.rs", "crates/cortex-core/src/lib.rs", "benches/cortex-bench/benches/hot_path.rs", "testkit/prop.rs", ".cargo/mutants.toml", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "elsewhere/x.rs"]) {
    assert.ok(readByRust(p), p);
  }
  for (const p of ["docs/WHITEPAPER.md", "briefs/README.md", "scripts/exhaustive-costs.tsv", ".github/workflows/ci.yml", "package.json", "CHANGELOG.md"]) {
    assert.ok(!readByRust(p), p);
  }
  assert.ok(underTests(TESTS));
  assert.ok(underTests("crates/cortex-core/tests/x.rs"));
  assert.ok(!underTests("runtime/cortex-runtime/src/task.rs"));
  assert.ok(!underTests("testkit/prop.rs"));
});

test("the scan counts all three brackets, and none inside a comment, a string or a character", () => {
  const text = [
    "fn f() { // a { here",
    '    let s = "}}]] {";',
    "    let r = r#\"a \"} quote\"#;",
    "    let c = '{';",
    "    let l: &'a u8 = x;",
    "    /* a /* nested { */ } */",
    "    g([1, (2)]",
    "}",
  ].join("\n");
  const { depth, last } = scan(text);
  assert.deepEqual(depth, [1, 1, 1, 1, 1, 1, 2, 1]);
  assert.equal(last[0], "{");
  assert.equal(last[1], ";");
  assert.equal(last[5], "");
  assert.equal(last[6], "]");
});

test("a string or a raw string over several lines holds its brackets to the end", () => {
  const { depth } = scan(['const S: &str = "a {', "b }", '";', "const R: &str = r\"x [", '";'].join("\n"));
  assert.deepEqual(depth, [0, 0, 0, 0, 0]);
});

test("the hunks are read with their counts, an omitted count being one", () => {
  const got = hunks(["@@ -10,0 +11,2 @@", "+a", "+b", "@@ -20 +22 @@", "-old", "+new", "@@ -30,3 +32,0 @@", "-x", "-y", "-z"].join("\n"));
  assert.deepEqual(got, [
    { oldStart: 10, oldCount: 0, added: ["a", "b"] },
    { oldStart: 20, oldCount: 1, added: ["new"] },
    { oldStart: 30, oldCount: 3, added: [] },
  ]);
});

test("an insertion is at the top level after a whole item, never inside one or after an attribute", () => {
  const base = scan(BASE);
  assert.ok(insertsAtTop(base, 0), "the file's start");
  assert.ok(insertsAtTop(base, 1), "after a use");
  assert.ok(insertsAtTop(base, 6), "after a function's brace");
  assert.ok(insertsAtTop(base, 7), "after a blank line after one");
  assert.ok(insertsAtTop(base, 11), "after a table's `];`");
  assert.ok(!insertsAtTop(base, 4), "inside a function");
  assert.ok(!insertsAtTop(base, 9), "between a table's rows");
  const attr = scan(["fn f() {}", "#[test]", "fn t() {}", ""].join("\n"));
  assert.ok(!insertsAtTop(attr, 2), "between an attribute and its item");
  const open = scan(["const X: u32 =", "", "    5;", ""].join("\n"));
  assert.ok(!insertsAtTop(open, 2), "after a blank line inside an item");
  const commented = scan(["fn f() {", "}", "// a note", "", ""].join("\n"));
  assert.ok(insertsAtTop(commented, 4), "comments and blank lines after a whole item");
});

test("an insertion of whole items gives the names it puts at the top level", () => {
  const got = inserted([
    "/// A table.",
    "const NEW_TABLE: &[u32] = &[",
    "    1,",
    "];",
    "",
    "#[test]",
    "#[ignore]",
    "fn a_new_test_exhaustive() {",
    "    fn inner() {}",
    "    let _ = NEW_TABLE;",
    "}",
    "const _: () = assert!(true);",
    "pub(crate) const fn helper() -> u32 { 0 }",
    "macro_rules! made { () => {}; }",
    "struct Fresh { x: u32 }",
  ]);
  assert.deepEqual(got.problems, []);
  assert.deepEqual(got.names, ["NEW_TABLE", "a_new_test_exhaustive", "helper", "made", "Fresh"]);
  assert.equal(got.uses, 0);
  assert.deepEqual(got.impls, []);
});

test("an insertion that is not whole items is a problem", () => {
  assert.match(inserted(["fn f() {", "    1"]).problems.join(), /balance/);
  assert.match(inserted(["}", "fn f() {}"]).problems.join(), /close/);
  assert.match(inserted(["    .map(|x| x)", "    .sum();"]).problems.join(), /first line/);
  assert.match(inserted(["#[test]"]).problems.join(), /last line/);
  assert.match(inserted(["const X: u32 = 1 +"]).problems.join(), /last line/);
});

test("a use and an impl are counted, with the type an impl is for", () => {
  const got = inserted(["use crate::harness::thing;", "impl Display for Block {", "    fn fmt(&self) {}", "}", "impl<T: Copy> Holder<T> {}"]);
  assert.equal(got.uses, 1);
  assert.deepEqual(got.impls, ["Block", "Holder"]);
  assert.equal(implType("impl<'a> From<&'a Image> for crate::Kept<'a> {"), "Kept");
  assert.equal(implType("unsafe impl Send for &mut Ring {}"), "Ring");
  assert.equal(implType("impl Fresh {"), "Fresh");
});

test("a word is a whole word: a name inside a longer one is not held", () => {
  assert.deepEqual([...wordsIn("let control_image_2 = 1; fn other() {}", ["control_image", "other"])], ["other"]);
});

test("a change of documents alone only adds, and reads no Rust", () => {
  const got = checkAdditive([{ status: "M", path: "docs/WHITEPAPER.md", diff: "" }], () => "", none);
  assert.deepEqual(got, { additive: true, problems: [], files: 0 });
});

test("whole items under new names inserted into a test file only add", () => {
  const diff = insertion(11, ["", "const NEW_ROWS: &[u32] = &[3];", "", "#[test]", "fn new_rows_hold() {", "    assert_eq!(NEW_ROWS[0], 3);", "}"]);
  const asked = [];
  const got = checkAdditive([{ status: "M", path: TESTS, diff }], () => BASE, (names) => (asked.push(...names), new Set()));
  assert.deepEqual(got, { additive: true, problems: [], files: 1 });
  assert.deepEqual(asked, ["NEW_ROWS", "new_rows_hold"]);
});

test("a new file under a package's tests/ only adds, whatever it holds", () => {
  const got = checkAdditive([{ status: "A", path: "runtime/cortex-runtime/tests/data/new.tables", diff: "" }, { status: "A", path: "runtime/cortex-runtime/tests/fresh.rs", diff: "" }], () => "", none);
  assert.equal(got.additive, true);
  assert.equal(got.files, 2);
});

test("a change under src/, to a manifest or to the kit does not only add", () => {
  for (const path of ["runtime/cortex-runtime/src/executor.rs", "Cargo.lock", "testkit/prop.rs", "crates/cortex-core/src/lib.rs"]) {
    const got = checkAdditive([{ status: "M", path, diff: "" }], () => "", none);
    assert.equal(got.additive, false, path);
    assert.match(got.problems[0], /outside a package's tests/);
  }
});

test("a line of a test file removed or changed does not only add", () => {
  const diff = [`@@ -4 +4 @@`, "-    let x = 1;", "+    let x = 2;"].join("\n");
  const got = checkAdditive([{ status: "M", path: TESTS, diff }], () => BASE, none);
  assert.equal(got.additive, false);
  assert.match(got.problems[0], /4: 1 line\(s\) of the base removed or changed/);
});

test("an insertion inside a function or a table does not only add", () => {
  for (const after of [4, 9]) {
    const got = checkAdditive([{ status: "M", path: TESTS, diff: insertion(after, ["    3,"]) }], () => BASE, none);
    assert.equal(got.additive, false, String(after));
    assert.match(got.problems[0], /inside an item/);
  }
});

test("an item under a name the file or what its crate sees already holds does not only add", () => {
  const own = checkAdditive([{ status: "M", path: TESTS, diff: insertion(11, ["fn existing_twice() {}", "const TABLE_B: u32 = 1;"]) }], () => BASE + "// TABLE_B was here\n", none);
  assert.equal(own.additive, false);
  assert.deepEqual(own.problems, [`${TESTS}:11: an item named \`TABLE_B\`, a name the base already uses`]);
  const seen = checkAdditive([{ status: "M", path: TESTS, diff: insertion(11, ["fn run_arm() {}"]) }], () => BASE, (names) => new Set(names));
  assert.equal(seen.additive, false);
  assert.match(seen.problems[0], /`run_arm`/);
});

test("a use, or an impl for a type the base names, does not only add", () => {
  const used = checkAdditive([{ status: "M", path: TESTS, diff: insertion(1, ["use other::existing;"]) }], () => BASE, none);
  assert.match(used.problems.join(), /a `use`/);
  const implemented = checkAdditive([{ status: "M", path: TESTS, diff: insertion(11, ["impl Block {", "    fn twice(&self) {}", "}"]) }], () => BASE, (names) => new Set(names.filter((n) => n === "Block")));
  assert.deepEqual(implemented.problems, [`${TESTS}:11: an \`impl\` for \`Block\`, a type the base names`]);
});

test("a test file deleted, renamed, or a data file changed does not only add", () => {
  for (const [status, path, why] of [
    ["D", TESTS, /deleted/],
    ["R100", "runtime/cortex-runtime/tests/moved.rs", /renamed/],
    ["M", "runtime/cortex-runtime/tests/data/old.tables", /changed/],
  ]) {
    const got = checkAdditive([{ status, path, diff: "" }], () => "", none);
    assert.equal(got.additive, false, status);
    assert.match(got.problems[0], why);
  }
});
