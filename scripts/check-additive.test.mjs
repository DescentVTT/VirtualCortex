// Tests of scripts/check-additive.mjs under node:test (stable since Node 20; this repository requires
// 22). The changes, the base's text and the names it holds are given in memory, so no case touches
// git or a file.

import assert from "node:assert/strict";
import { test } from "node:test";

import { ABSENT, PINS_FILE, PINS_MOD, PINS_NESTED, checkAdditive, hunks, implType, inserted, insertsAtTop, pinsPairs, readByRust, roundsFile, scan, underTests, wordsIn } from "./check-additive.mjs";

const TESTS = "runtime/menteion-runtime/tests/inhibition.rs";

// A `-U0` diff of one hunk inserting `lines` after base line `after`.
const insertion = (after, lines) => [`diff --git a/${TESTS} b/${TESTS}`, `@@ -${after},0 +${after + 1},${lines.length} @@`, ...lines.map((l) => `+${l}`)].join("\n");

const BASE = ["use super::*;", "", "fn existing() -> u32 {", "    let x = 1;", "    x", "}", "", "const TABLE: &[u32] = &[", "    1,", "    2,", "];", ""].join("\n");

const none = () => new Set();

test("a Rust build reads the crates, the runtime, the benches, the kit and the manifests, and not the documents", () => {
  for (const p of ["runtime/menteion-runtime/src/executor.rs", "crates/menteion-core/src/lib.rs", "benches/menteion-bench/benches/hot_path.rs", "testkit/prop.rs", ".cargo/mutants.toml", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "elsewhere/x.rs"]) {
    assert.ok(readByRust(p), p);
  }
  for (const p of ["docs/WHITEPAPER.md", "briefs/README.md", "scripts/exhaustive-costs.tsv", ".github/workflows/ci.yml", "package.json", "CHANGELOG.md"]) {
    assert.ok(!readByRust(p), p);
  }
  assert.ok(underTests(TESTS));
  assert.ok(underTests("crates/menteion-core/tests/x.rs"));
  assert.ok(!underTests("runtime/menteion-runtime/src/task.rs"));
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
  const got = checkAdditive([{ status: "A", path: "runtime/menteion-runtime/tests/data/new.tables", diff: "" }, { status: "A", path: "runtime/menteion-runtime/tests/fresh.rs", diff: "" }], () => "", none);
  assert.equal(got.additive, true);
  assert.equal(got.files, 2);
});

test("a change under src/, to a manifest or to the kit does not only add", () => {
  for (const path of ["runtime/menteion-runtime/src/executor.rs", "Cargo.lock", "testkit/prop.rs", "crates/menteion-core/src/lib.rs"]) {
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
    ["R100", "runtime/menteion-runtime/tests/moved.rs", /renamed/],
    ["M", "runtime/menteion-runtime/tests/data/old.tables", /changed/],
  ]) {
    const got = checkAdditive([{ status, path, diff: "" }], () => "", none);
    assert.equal(got.additive, false, status);
    assert.match(got.problems[0], why);
  }
});

// A round under ADR-0227's layout: its tables in a new `pins/<round>.rs`, a pair in `pins/mod.rs`.
const PINS = "runtime/menteion-runtime/tests/inhibition/pins";
const MOD_BASE = ["//! The tables.", "", "mod h45;", "pub(crate) use h45::*;", ""].join("\n");
const H46 = ["//! The tables H-46's round pinned from its runs.", "", "use crate::*;", "", "pub(crate) const H46_ROWS: &[u32] = &[1, 2];", ""].join("\n");
const pinsBase = (path) => {
  if (path === `${PINS}/mod.rs`) return MOD_BASE;
  if (path === TESTS) return BASE;
  throw new Error(`no base for ${path}`);
};
const modInsertion = (lines) => [`diff --git a/${PINS}/mod.rs b/${PINS}/mod.rs`, `@@ -4,0 +5,${lines.length} @@`, ...lines.map((l) => `+${l}`)].join("\n");
const round = (modLines, h46 = H46) => [
  { status: "A", path: `${PINS}/h46.rs`, diff: "", text: h46 },
  { status: "M", path: `${PINS}/mod.rs`, diff: modInsertion(modLines) },
  { status: "M", path: TESTS, diff: insertion(11, ["", "#[test]", "fn h46_rows_hold() {", "    assert_eq!(H46_ROWS[0], 1);", "}"]) },
];
const PAIR = ["mod h46;", "pub(crate) use h46::*;"];

test("the pins' paths: a binary's mod.rs and its rounds' files, nothing else", () => {
  assert.equal(PINS_MOD.exec(`${PINS}/mod.rs`)[1], PINS);
  assert.equal(PINS_MOD.exec(TESTS), null);
  assert.deepEqual(PINS_FILE.exec(`${PINS}/h46.rs`).slice(1), ["runtime/menteion-runtime/tests/inhibition", "h46"]);
  assert.equal(PINS_FILE.exec("runtime/menteion-runtime/tests/instrument/harness.rs"), null);
});

test("a round's pairs are read, and an insertion with anything else is not pairs", () => {
  assert.deepEqual(pinsPairs([...PAIR, "", "mod h47;", "pub(crate) use h47::*;"]), ["h46", "h47"]);
  assert.equal(pinsPairs(["mod h46;", "pub(crate) use h45::*;"]), null);
  assert.equal(pinsPairs(["mod h46;", "pub(crate) use h46::H46_ROWS;"]), null);
  assert.equal(pinsPairs(["mod h46;", "use h46::*;"]), null);
  assert.equal(pinsPairs(["mod h46;"]), null);
  assert.equal(pinsPairs(["// a note", ...PAIR]), null);
  assert.equal(pinsPairs([]), null);
});

test("a round that pins its tables in its own file, its pair in mod.rs and its tests in the root, only adds", () => {
  const asked = [];
  const got = checkAdditive(round(PAIR), pinsBase, (names) => (asked.push(...names), new Set()));
  assert.deepEqual(got, { additive: true, problems: [], files: 3 });
  assert.deepEqual(asked.sort(), ["H46_ROWS", "h46", "h46_rows_hold"]);
});

test("a pair for a round's file the change does not add, or a use that is not a pair, does not only add", () => {
  const notAdded = checkAdditive(round(["mod h47;", "pub(crate) use h47::*;"]), pinsBase, none);
  assert.deepEqual(notAdded.problems, [`${PINS}/mod.rs:4: an insertion with a \`use\`, which can shadow a glob import`]);
  assert.equal(checkAdditive(round(["mod h46;", "pub(crate) use h46::H46_ROWS;"]), pinsBase, none).additive, false);
  const inRoot = checkAdditive([round(PAIR)[0], { status: "M", path: TESTS, diff: insertion(11, PAIR) }], pinsBase, none);
  assert.match(inRoot.problems.join(), /a `use`/);
});

test("a round's file whose names the base already uses, in its root or anywhere it sees, does not only add", () => {
  const inRoot = checkAdditive(round(PAIR, H46.replace("H46_ROWS: &[u32]", "TABLE: &[u32]")), pinsBase, none);
  assert.deepEqual(inRoot.problems, [`${PINS}/h46.rs: an item named \`TABLE\`, a name the base already uses`]);
  const seen = checkAdditive(round(PAIR), pinsBase, (names) => new Set(names.filter((n) => n === "H46_ROWS")));
  assert.match(seen.problems.join(), /`H46_ROWS`/);
  const implemented = checkAdditive(round(PAIR, H46 + "impl Block {}\n"), pinsBase, (names) => new Set(names.filter((n) => n === "Block")));
  assert.match(implemented.problems.join(), /an `impl` for `Block`/);
});

test("a round's file added with no text read does not only add; its binary's new mod.rs does", () => {
  const got = checkAdditive([{ status: "A", path: `${PINS}/h46.rs`, diff: "" }], pinsBase, none);
  assert.deepEqual(got.problems, [`${PINS}/h46.rs: a round's file of pinned tables whose text was not read`]);
  assert.equal(checkAdditive([{ status: "A", path: `${PINS}/mod.rs`, diff: "" }], pinsBase, none).additive, true);
});

// What a round's file may not hold at its top level: anything that brings a name it does not declare.
const REEXPORTS = {
  E: "pub(crate) use crate::BOTH_SEEDS as SPANS;",
  E2: "pub(crate) use crate::harness::BACKGROUND_ROWS_1024;",
  glob: "pub(crate) use menteion_core::*;",
  macro: 'include!("rows.rs");',
  fn: "pub(crate) fn helper() -> u32 { 0 }",
};

test("a round's file of `use crate::*;`, its tables, comments and attributes holds nothing else", () => {
  const text = [
    "//! The tables.",
    "",
    "use crate::*;",
    "",
    "/// A table.",
    "#[allow(clippy::type_complexity)]",
    "pub(crate) const ROWS: &[(u32, u32)] = &[",
    "    (1, 2),",
    '    // a row: use x; mod y; include!("z");',
    "];",
    "pub(crate) static WHOLE: [u32; 2] = [1, 2];",
    "const _: () = assert!(ROWS.len() == 1);",
    "",
  ].join("\n");
  assert.deepEqual(roundsFile(text), []);
  assert.deepEqual(roundsFile(H46), []);
});

test("a re-export, another glob, a mod, a macro's invocation or a function in a round's file is named by line", () => {
  for (const [what, line] of Object.entries(REEXPORTS)) {
    assert.deepEqual(roundsFile(H46 + line + "\n"), [`6: '${line}', where a round's file holds only \`use crate::*;\` and its tables`], what);
  }
  assert.equal(roundsFile(H46 + "mod zq;\npub(crate) use zq::*;\n").length, 2, "K's mod and its re-export");
  assert.equal(roundsFile(H46.replace("use crate::*;", "pub(crate) use crate::*;")).length, 1, "the crate's glob re-exported");
});

test("a round whose file re-exports a name it does not declare does not only add: E, E2, a glob, a macro", () => {
  for (const [what, line] of Object.entries(REEXPORTS)) {
    const got = checkAdditive(round(PAIR, H46 + line + "\n"), pinsBase, none);
    assert.equal(got.additive, false, what);
    assert.match(got.problems.join(" | "), /h46\.rs:6: '/, what);
  }
});

test("a round whose file declares a module, or that adds a file under pins/<round>/, does not only add: K", () => {
  const nested = `${PINS}/h46/zq.rs`;
  assert.ok(PINS_NESTED.test(nested));
  assert.ok(!PINS_NESTED.test(`${PINS}/h46.rs`));
  const got = checkAdditive(
    [...round(PAIR, H46 + "mod zq;\npub(crate) use zq::*;\n"), { status: "A", path: nested, diff: "", text: "pub(crate) const BOTH_SEEDS: [u64; 3] = [1, 2, 3];\n" }],
    pinsBase,
    none,
  );
  assert.equal(got.additive, false);
  assert.ok(got.problems.includes(`${nested}: a file under a round's directory, where a round's tables are one file`));
  assert.ok(got.problems.some((p) => p.startsWith(`${PINS}/h46.rs:6: 'mod zq;'`)));
});

test("a binary with no root at the base is new; any other failure to read the base is thrown", () => {
  const fresh = "runtime/menteion-runtime/tests/fresh/pins";
  const change = [
    { status: "A", path: `${fresh}/h46.rs`, diff: "", text: H46 },
    { status: "A", path: `${fresh}/mod.rs`, diff: "", text: undefined },
  ];
  const absent = () => {
    throw Object.assign(new Error("Command failed: git show abc1234:runtime/menteion-runtime/tests/fresh.rs"), {
      stderr: "fatal: path 'runtime/menteion-runtime/tests/fresh.rs' does not exist in 'abc1234'\n",
    });
  };
  assert.deepEqual(checkAdditive(change, absent, none), { additive: true, problems: [], files: 2 });
  assert.ok(ABSENT.test("fatal: path 'x.rs' exists on disk, but not in 'abc1234'"));
  const broken = () => {
    throw Object.assign(new Error("Command failed: git show"), { stderr: "fatal: bad object abc1234\n" });
  };
  assert.throws(() => checkAdditive(change, broken, none), /Command failed/);
});
