// Tests of scripts/check-pins.mjs under node:test (stable since Node 20; this repository requires
// 22). Every case is text in memory.

import assert from "node:assert/strict";
import { test } from "node:test";

import { LONGEST, ROUND, checkLength, checkMod, checkPin, checkRoot } from "./check-pins.mjs";

const ROOT = [
  "#![deny(clippy::arithmetic_side_effects)]",
  "",
  '#[path = "instrument/harness.rs"]',
  "mod harness;",
  "use harness::*;",
  "",
  "// The tables the rounds pinned from their runs, a file a round and a row a line (ADR-0227).",
  "#[rustfmt::skip]",
  '#[path = "inhibition/pins/mod.rs"]',
  "mod pins;",
  "use pins::*;",
  "",
  "const ARMS: [Arm; 2] = [Arm::A, Arm::B];",
  "",
].join("\n");

test("a round's file is a hypothesis's or a brief's", () => {
  assert.ok(ROUND.test("h45.rs"));
  assert.ok(ROUND.test("brief049.rs"));
  assert.ok(!ROUND.test("mod.rs"));
  assert.ok(!ROUND.test("h45.txt"));
  assert.ok(!ROUND.test("brief49.rs"));
  assert.ok(!ROUND.test("H45.rs"));
});

test("a binary that declares its pins as the record says passes", () => {
  assert.deepEqual(checkRoot("inhibition.rs", ROOT, "inhibition"), []);
});

test("a binary whose pins rustfmt would lay out fails, and one with the attributes reordered", () => {
  assert.equal(checkRoot("inhibition.rs", ROOT.replace("#[rustfmt::skip]\n", ""), "inhibition").length, 1);
  const swapped = ROOT.replace(
    '#[rustfmt::skip]\n#[path = "inhibition/pins/mod.rs"]',
    '#[path = "inhibition/pins/mod.rs"]\n#[rustfmt::skip]',
  );
  assert.equal(checkRoot("inhibition.rs", swapped, "inhibition").length, 1);
});

test("a binary that names another's pins, or does not import its own, fails", () => {
  assert.equal(checkRoot("assembly.rs", ROOT, "assembly").length, 1);
  assert.deepEqual(checkRoot("inhibition.rs", ROOT.replace("use pins::*;\n", ""), "inhibition"), [
    "inhibition.rs: does not import its pins with use pins::*;",
  ]);
});

const MOD = ["//! The tables.", "", "mod h44;", "pub(crate) use h44::*;", "mod h45;", "pub(crate) use h45::*;", ""].join("\n");

test("a mod.rs that declares and re-exports every round's file passes", () => {
  assert.deepEqual(checkMod("mod.rs", MOD, ["h44.rs", "h45.rs"]), []);
});

test("a round's file mod.rs leaves out, or does not re-export, fails", () => {
  assert.deepEqual(checkMod("mod.rs", MOD, ["h44.rs", "h45.rs", "h46.rs"]), [
    "mod.rs: does not declare h46.rs as mod h46; and pub(crate) use h46::*;",
  ]);
  assert.equal(checkMod("mod.rs", MOD.replace("pub(crate) use h45::*;\n", ""), ["h44.rs", "h45.rs"]).length, 1);
});

test("a module mod.rs declares with no file fails", () => {
  assert.deepEqual(checkMod("mod.rs", MOD, ["h44.rs"]), ["mod.rs: declares mod h45;, which has no file"]);
});

const PIN = [
  "//! The tables H-45's round pinned from its runs, a row a line (ADR-0227).",
  "",
  "use crate::*;",
  "",
  "/// The arms' blocks, pinned from their one run.",
  "pub(crate) const BLOCKS: [&[Block]; 2] = [",
  "    &[",
  "        (31, 34, [[335, 321], [315, 321]], 8),",
  "        (32, 31, [[303, 314], [331, 347]], 5),",
  "    ],",
  "    &[(27, 31, [[301, 330], [343, 370]], 9)],",
  "];",
  "",
  "pub(crate) const HELD: Option<(Quieted, u64)> = Some((",
  "    [[689, 463, 130], [1737, 1793, 474]],",
  "    0xc9e7_9352_f7b3_2027,",
  "));",
  "",
].join("\n");

test("a round's file of tables, one row a line, passes", () => {
  assert.deepEqual(checkPin("h45.rs", PIN), []);
});

test("a test or a function in a round's file fails, by line", () => {
  const withTest = PIN + "#[test]\nfn the_arms() {}\n";
  assert.deepEqual(checkPin("h45.rs", withTest), [
    "h45.rs:18: a function, a macro or a test, which belongs in the binary's file",
    "h45.rs:19: a function, a macro or a test, which belongs in the binary's file",
  ]);
  assert.equal(checkPin("h45.rs", PIN + "pub(crate) const fn at(k: usize) -> u32 { 0 }\n").length, 1);
});

test("a function under any visibility or qualifier, and a macro, fail in a round's file", () => {
  for (const item of [
    "pub(in crate) fn at() -> u32 { 0 }",
    "pub(super) unsafe fn at() {}",
    "pub async fn at() {}",
    'extern "C" fn at() {}',
    "macro_rules! row { () => {}; }",
    "#[macro_export]",
  ]) {
    assert.deepEqual(checkPin("h45.rs", PIN + item + "\n"), [
      "h45.rs:18: a function, a macro or a test, which belongs in the binary's file",
    ], item);
  }
  assert.deepEqual(checkPin("h45.rs", PIN + "pub(crate) const FNORD: u32 = 1;\n"), []);
});

test("a row broken over lines, as rustfmt lays one out, fails; a comment does not", () => {
  const broken = PIN.replace("        (31, 34, [[335, 321], [315, 321]], 8),", "        (\n            31,\n            34,\n        ),");
  assert.deepEqual(checkPin("h45.rs", broken), ["h45.rs:8: a tuple broken over lines, where a row is one line"]);
  assert.deepEqual(checkPin("h45.rs", PIN.replace("/// The arms' blocks, pinned from their one run.", "/// The arms' blocks (")), []);
});

test("a binary's const of LONGEST lines passes, one line more fails, and a fn's body is not read", () => {
  const table = (n) => ["const T: [u32; 2] = [", ...Array.from({ length: n - 2 }, () => "    1,"), "];"].join("\n");
  assert.deepEqual(checkLength("b.rs", table(LONGEST) + "\n"), []);
  assert.deepEqual(checkLength("b.rs", "use x;\n\n" + table(LONGEST + 1) + "\n"), [
    `b.rs:3: T is ${LONGEST + 1} lines, past ${LONGEST}: a pinned table goes in its round's file`,
  ]);
  const inFn = ["fn f() {", "    " + table(LONGEST + 5).split("\n").join("\n    "), "}"].join("\n");
  assert.deepEqual(checkLength("b.rs", inFn + "\n"), []);
});

test("a const's length is read past strings and comments that hold brackets", () => {
  const text = ['const NAME: &str = "a ( [ {";', "// ] ) }", "const N: u32 = 1;", ""].join("\n");
  assert.deepEqual(checkLength("b.rs", text, 1), []);
  assert.deepEqual(checkLength("b.rs", "static S: [u8; 2] =\n    [1, 2];\n", 1), [
    "b.rs:1: S is 2 lines, past 1: a pinned table goes in its round's file",
  ]);
});

test("a static mut, and an item under any visibility, is held to LONGEST too", () => {
  const table = (head, n) => [`${head}: [u32; 2] = [`, ...Array.from({ length: n - 2 }, () => "    1,"), "];"].join("\n");
  assert.deepEqual(checkLength("b.rs", table("static mut M", LONGEST + 1) + "\n"), [
    `b.rs:1: M is ${LONGEST + 1} lines, past ${LONGEST}: a pinned table goes in its round's file`,
  ]);
  assert.equal(checkLength("b.rs", table("pub(in crate) const P", LONGEST + 1) + "\n").length, 1);
  assert.equal(checkLength("b.rs", table("pub(crate) static mut Q", LONGEST) + "\n").length, 0);
});

test("an item never closed is read to the file's end", () => {
  const open = ["const T: [u32; 2] = [", ...Array.from({ length: LONGEST }, () => "    1,")].join("\n");
  assert.deepEqual(checkLength("b.rs", open), [`b.rs:1: T is ${LONGEST + 1} lines, past ${LONGEST}: a pinned table goes in its round's file`]);
});
