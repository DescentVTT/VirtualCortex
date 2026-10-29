#!/usr/bin/env node
// A round's pinned tables (ADR-0227). The tables a round pins from its runs live in
// `tests/<binary>/pins/<round>.rs`, one file a round, and the tests that read them stay in
// `tests/<binary>.rs`. rustfmt lays none of the tables out. For every `tests/<binary>/pins/`
// under the runtime's tests, this holds:
//
// 1. `tests/<binary>.rs` declares the module as `#[rustfmt::skip]`, `#[path = "<binary>/pins/mod.rs"]`
//    and `mod pins;`, on three lines in that order, and imports it with `use pins::*;`.
// 2. `pins/mod.rs` declares every round's file, `h<N>.rs` or `brief<NNN>.rs`, as `mod <round>;`
//    followed by `pub(crate) use <round>::*;`, and no module without its file. `pins/` holds those
//    files alone: no directory, so no round's file has a module of its own.
// 3. A round's file holds no function, no macro and no test. The tests stay in `tests/<binary>.rs`,
//    whose names the weekly job, the cost table and the ADRs cite. At its top level it holds
//    nothing but `use crate::*;`, its `const` and `static` items, comments, attributes and blank
//    lines: no other `use`, which could re-export to the binary's root a name it does not declare,
//    no `mod` and no macro's invocation.
// 4. A round's file breaks no tuple: no line but a table's first ends in `(`.
// 5. `tests/<binary>.rs` holds no `const` or `static` longer than `LONGEST` lines. A table that
//    long is pinned, and goes in its round's file.
//
// That a shorter table is its round's, and that a row is one line, is held by review.
// Zero dependencies; runs the same way locally and in CI. Exit 1 with each file and the problem
// on failure, 0 otherwise. The checks are exported for scripts/check-pins.test.mjs.

import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { scan } from "./check-additive.mjs";

// Where the binaries are.
export const TESTS = "runtime/menteion-runtime/tests";

// A round's file: a hypothesis's, `h45.rs`, or a brief's that read none, `brief049.rs`.
export const ROUND = /^(h[0-9]+|brief[0-9]{3})\.rs$/;

// The longest `const` or `static` a binary's own file may hold, in lines.
export const LONGEST = 24;

const lines = (text) => text.replace(/^﻿/, "").split(/\r?\n/);

// Rule 1: the binary's file declares the module and imports it.
export function checkRoot(name, text, binary) {
  const ls = lines(text);
  const at = ls.indexOf("mod pins;");
  const problems = [];
  if (at < 2 || ls[at - 2] !== "#[rustfmt::skip]" || ls[at - 1] !== `#[path = "${binary}/pins/mod.rs"]`) {
    problems.push(
      `${name}: does not declare its pins as #[rustfmt::skip], #[path = "${binary}/pins/mod.rs"] and mod pins;, in that order`,
    );
  }
  if (!ls.includes("use pins::*;")) problems.push(`${name}: does not import its pins with use pins::*;`);
  return problems;
}

// Rule 2: `mod.rs` declares and re-exports each round's file, and nothing else.
export function checkMod(name, text, files) {
  const problems = [];
  const ls = lines(text);
  const declared = ls.map((l) => /^mod ([A-Za-z0-9_]+);$/.exec(l)?.[1]).filter(Boolean);
  for (const f of files) {
    const r = f.replace(/\.rs$/, "");
    const k = ls.indexOf(`mod ${r};`);
    if (k < 0 || ls[k + 1] !== `pub(crate) use ${r}::*;`) {
      problems.push(`${name}: does not declare ${f} as mod ${r}; and pub(crate) use ${r}::*;`);
    }
  }
  for (const d of declared) if (!files.includes(`${d}.rs`)) problems.push(`${name}: declares mod ${d};, which has no file`);
  return problems;
}

// A visibility, as `pub`, `pub(crate)`, `pub(super)` or `pub(in crate::a)`, and the space after it.
const VIS = String.raw`(?:pub(?:\([^)]*\))?\s+)?`;
// A function, whatever its visibility and qualifiers.
export const FN = new RegExp(String.raw`^\s*${VIS}(?:(?:const|async|unsafe|extern\s+"[^"]*")\s+)*fn\b`);
// A macro, by `macro_rules!` or a `#[macro_export]`.
export const MACRO = /^\s*(?:macro_rules!|#\[macro_export\])/;
// A `const` or `static` item's first line, `static mut` too: its name captured.
export const TABLE = new RegExp(String.raw`^${VIS}(?:const|static)\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:`);

// What may begin an item at a round's file's top level, beside a table: an attribute, the crate's glob.
const ATTRIBUTE = /^\s*#!?\[/;
const CRATE_GLOB = /^\s*use crate::\*;\s*$/;

// Rules 3 and 4: a round's file holds tables, with no function, no macro, no test, nothing else at
// its top level but `use crate::*;`, and no tuple broken.
export function checkPin(name, text) {
  const problems = [];
  const { depth, last } = scan(text.endsWith("\n") ? text : `${text}\n`);
  lines(text).forEach((l, i) => {
    if (/^\s*\/\//.test(l)) return;
    const top = (i === 0 ? 0 : depth[i - 1]) === 0 && (last[i] ?? "") !== "";
    if (/^\s*#\[test\]/.test(l) || FN.test(l) || MACRO.test(l)) {
      problems.push(`${name}:${i + 1}: a function, a macro or a test, which belongs in the binary's file`);
    } else if (top && !TABLE.test(l) && !ATTRIBUTE.test(l) && !CRATE_GLOB.test(l)) {
      problems.push(`${name}:${i + 1}: '${l.trim()}' at the top level, where a round's file holds only \`use crate::*;\` and its tables`);
    }
    if (/\($/.test(l.trimEnd()) && !TABLE.test(l)) {
      problems.push(`${name}:${i + 1}: a tuple broken over lines, where a row is one line`);
    }
  });
  return problems;
}

// Rule 2's second half: `pins/` holds `mod.rs` and rounds' files, and no directory.
// `entries`: `{ name, directory }` for each entry but `mod.rs`.
export function checkEntries(dir, entries) {
  const problems = [];
  for (const { name, directory } of entries) {
    if (directory) problems.push(`${join(dir, name)}: a directory under pins/, where a round's tables are one file`);
    else if (!ROUND.test(name)) problems.push(`${join(dir, name)}: not a round's file, h<N>.rs or brief<NNN>.rs`);
  }
  return problems;
}

// Rule 5: the binary's own `const` and `static` items are each at most `LONGEST` lines.
export function checkLength(name, text, longest = LONGEST) {
  const problems = [];
  const ls = lines(text);
  const { depth, last } = scan(text);
  for (let i = 0; i < ls.length; i += 1) {
    const before = i === 0 ? 0 : depth[i - 1];
    const head = TABLE.exec(ls[i]);
    if (before !== 0 || !head) continue;
    // the item ends on the first line from its head that closes it, or at the file's end
    let end = ls.length - 1;
    for (let e = i; e < ls.length; e += 1) {
      if (depth[e] === 0 && last[e] === ";") {
        end = e;
        break;
      }
    }
    const span = end - i + 1;
    if (span > longest) {
      problems.push(`${name}:${i + 1}: ${head[1]} is ${span} lines, past ${longest}: a pinned table goes in its round's file`);
    }
    i = end;
  }
  return problems;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const problems = [];
  let binaries = 0;
  let rounds = 0;
  for (const binary of readdirSync(TESTS).sort()) {
    const dir = join(TESTS, binary, "pins");
    if (!existsSync(join(dir, "mod.rs"))) continue;
    binaries += 1;
    const root = join(TESTS, `${binary}.rs`);
    if (!existsSync(root)) {
      problems.push(`${dir}: no ${root} declares it`);
      continue;
    }
    const rootText = readFileSync(root, "utf8");
    problems.push(...checkRoot(root, rootText, binary), ...checkLength(root, rootText));
    const entries = readdirSync(dir, { withFileTypes: true })
      .filter((e) => e.name !== "mod.rs")
      .map((e) => ({ name: e.name, directory: e.isDirectory() }))
      .sort((a, b) => a.name.localeCompare(b.name));
    problems.push(...checkEntries(dir, entries));
    const files = entries.filter((e) => !e.directory && ROUND.test(e.name)).map((e) => e.name);
    problems.push(...checkMod(join(dir, "mod.rs"), readFileSync(join(dir, "mod.rs"), "utf8"), files));
    for (const f of files) {
      rounds += 1;
      problems.push(...checkPin(join(dir, f), readFileSync(join(dir, f), "utf8")));
    }
  }
  if (binaries === 0) problems.push(`no ${TESTS}/<binary>/pins/mod.rs, so nothing was checked`);
  if (problems.length > 0) {
    for (const p of problems) console.error(p);
    console.error(`check-pins: ${problems.length} problem(s)`);
    process.exit(1);
  }
  console.log(`check-pins: ${binaries} binaries, ${rounds} rounds' files, each declared and holding tables alone; no const of a binary's own past ${LONGEST} lines`);
}
