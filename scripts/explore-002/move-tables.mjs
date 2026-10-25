// Exploration 002: moves a test file's pure-literal tables into a data file.
//
// node move-tables.mjs <tests dir> <stem> <mode> [--keep NAME,NAME]
//   mode v0: the consts stay; each moved table gets a `NAME__READ` static read from the data
//            file, and the test `the_moved_tables_read_as_written_in_<stem>_explore` holds every
//            one to its const.
//   mode v1: each moved const becomes `static NAME: LazyLock<TYPE>` read from the data file at run
//            time (`DataFile::from_file`); its uses that are not an index or a method call read
//            `(*NAME)`; the test `the_moved_tables_parse_in_<stem>_explore` times the parse.
//   mode v2: as v1, with the data file in the binary by `include_str!`.
//   --keep: tables not to move (their const is read in a const context, say).
// Writes <tests>/<stem>.rs and <tests>/data/<stem>.tables; prints what it moved.
import fs from "node:fs";
import path from "node:path";
import { items, pureLiteral } from "./survey-tables.mjs";

const [dir, stem, mode, ...rest] = process.argv.slice(2);
const keep = new Set();
for (let i = 0; i < rest.length; i += 1) {
  if (rest[i] === "--keep") for (const n of rest[i + 1].split(",").filter(Boolean)) keep.add(n);
}
const file = path.join(dir, `${stem}.rs`);
const text = fs.readFileSync(file, "utf8");
if (text.includes("\r")) throw new Error("CRLF in the source");
const lines = text.split("\n");
const all = items(text);

// The tables moved: plain literals of at least a kilobyte. An item whose value names a moved table
// (or such an item) becomes lazy itself: the same expression, run once at the first read, each
// name in it read through its `LazyLock`. A name given to --keep stays a const, and so does every
// table it names.
const byName = new Map(all.map((it) => [it.name, it]));
const namesIn = (it) => new Set(it.expr.match(/\b[A-Z][A-Z0-9_]+\b/g) ?? []);
const pinnedBy = new Set();
for (const k of keep) for (const n of byName.has(k) ? namesIn(byName.get(k)) : []) pinnedBy.add(n);
const moved = all.filter((it) => pureLiteral(it.expr) && it.expr.length >= 1024 && !keep.has(it.name) && !pinnedBy.has(it.name));
const movedNames = new Set(moved.map((x) => x.name));
const movedBytes = moved.reduce((a, x) => a + x.expr.length, 0);
const lazy = new Set();
for (let changed = true; changed; ) {
  changed = false;
  for (const it of all) {
    if (movedNames.has(it.name) || lazy.has(it.name) || keep.has(it.name)) continue;
    for (const n of namesIn(it)) {
      if (movedNames.has(n) || lazy.has(n)) {
        lazy.add(it.name);
        changed = true;
        break;
      }
    }
  }
}
const lazyItems = all.filter((it) => lazy.has(it.name));
const readNames = new Set([...movedNames, ...lazy]);

// The data file: a header, then `@ NAME` and the value as written.
const data = [
  `// Exploration 002 (ADR-0193), branch explore/002-what-the-pinned-tables-cost: the pinned tables`,
  `// moved out of tests/${stem}.rs, each \`@ NAME\` followed by its value as the source wrote it.`,
  `// Read by tests/instrument/data_file.rs. Not in main.`,
];
for (const it of moved) data.push(`@ ${it.name}`, it.expr);
fs.mkdirSync(path.join(dir, "data"), { recursive: true });
fs.writeFileSync(path.join(dir, "data", `${stem}.tables`), data.join("\n") + "\n");

const loader =
  mode === "v2"
    ? `LazyLock::new(|| DataFile::from_text(include_str!("data/${stem}.tables")))`
    : `LazyLock::new(|| {\n    DataFile::from_file(concat!(\n        env!("CARGO_MANIFEST_DIR"),\n        "/tests/data/${stem}.tables"\n    ))\n})`;
const preamble = [
  "",
  "// Exploration 002 (ADR-0193): the pinned tables of at least a kilobyte whose values are plain",
  `// literals live in tests/data/${stem}.tables and are read from it at run time${mode === "v2" ? ", the file in the binary by `include_str!`" : ""}`,
  "// (tests/instrument/data_file.rs). Not in main.",
  "#[allow(dead_code)]",
  '#[path = "instrument/data_file.rs"]',
  "mod data_file;",
  "use std::sync::LazyLock;",
  "use data_file::DataFile;",
  `static DATA_FILE: LazyLock<DataFile> = ${loader};`,
];

const out = [];
const byStart = new Map(moved.map((x) => [x.start, x]));
const lazyByStart = new Map(lazyItems.map((x) => [x.start, x]));
const inMoved = new Set();
for (const it of moved) for (let k = it.start; k <= it.end; k += 1) inMoved.add(k);

// Replace a use of a moved or lazy table that is not an index or a method call by `(*NAME)`,
// outside comments and strings.
const useRe = new RegExp(`\\b(${[...readNames].join("|")})\\b(?!\\s*[\\[.])`, "g");
function rewriteUses(line) {
  if (readNames.size === 0) return line;
  const t = line.trimStart();
  if (t.startsWith("//")) return line;
  // Split off a trailing comment and walk the string literals.
  let code = line;
  let comment = "";
  let inStr = false;
  for (let i = 0; i < line.length; i += 1) {
    const c = line[i];
    if (c === '"' && line[i - 1] !== "\\") inStr = !inStr;
    if (!inStr && c === "/" && line[i + 1] === "/") {
      code = line.slice(0, i);
      comment = line.slice(i);
      break;
    }
  }
  const parts = code.split('"');
  for (let i = 0; i < parts.length; i += 2) parts[i] = parts[i].replace(useRe, "(*$1)");
  return parts.join('"') + comment;
}

let inserted = false;
let rewritten = 0;
for (let i = 0; i < lines.length; i += 1) {
  const line = lines[i];
  if (!inserted && line === "use harness::*;") {
    out.push(line, ...preamble);
    inserted = true;
    continue;
  }
  if (byStart.has(i)) {
    const it = byStart.get(i);
    if (mode === "v0") {
      for (let k = it.start; k <= it.end; k += 1) out.push(lines[k]);
      out.push(`static ${it.name}__READ: LazyLock<${it.type}> = LazyLock::new(|| DATA_FILE.read("${it.name}"));`);
    } else {
      out.push(`${it.vis}static ${it.name}: LazyLock<${it.type}> = LazyLock::new(|| DATA_FILE.read("${it.name}"));`);
    }
    i = it.end;
    continue;
  }
  if (mode !== "v0" && lazyByStart.has(i)) {
    const it = lazyByStart.get(i);
    const expr = it.expr.split("\n").map(rewriteUses).join("\n");
    const decl = `${it.vis}static ${it.name}: LazyLock<${it.type}> = LazyLock::new(|| ${expr});`;
    out.push(...decl.split("\n"));
    rewritten += 1;
    i = it.end;
    continue;
  }
  if (mode !== "v0" && !inMoved.has(i)) {
    const r = rewriteUses(line);
    if (r !== line) rewritten += 1;
    out.push(r);
  } else {
    out.push(line);
  }
}
if (!inserted) throw new Error("no `use harness::*;` line");

while (out.length && out[out.length - 1] === "") out.pop();
const names = [...movedNames];
if (mode === "v0") {
  out.push(
    "",
    "/// Exploration 002: every table moved to the data file reads from it as the `const` it was moved",
    "/// from, value for value.",
    "#[test]",
    '#[ignore = "exploration 002 (ADR-0193): a run on its branch, never in the weekly job"]',
    `fn the_moved_tables_read_as_written_in_${stem}_explore() {`,
    `    assert_eq!(DATA_FILE.len(), ${names.length});`,
    ...names.map((n) => `    assert_eq!(*${n}__READ, ${n}, "${n}");`),
    `    println!("EXPLORE ${stem}: ${names.length} tables read as written");`,
    "}",
  );
} else {
  out.push(
    "",
    "/// Exploration 002: what reading every moved table costs a test binary, once.",
    "#[test]",
    '#[ignore = "exploration 002 (ADR-0193): a run on its branch, never in the weekly job"]',
    `fn the_moved_tables_parse_in_${stem}_explore() {`,
    "    let started = std::time::Instant::now();",
    `    assert_eq!(DATA_FILE.len(), ${names.length});`,
    "    let indexed = started.elapsed();",
    ...names.map((n) => `    LazyLock::force(&${n});`),
    `    println!(`,
    `        "EXPLORE ${stem}: ${names.length} tables, the file indexed in {} us, all parsed in {} us",`,
    "        indexed.as_micros(),",
    "        started.elapsed().as_micros()",
    "    );",
    "}",
  );
}
fs.writeFileSync(file, out.join("\n") + "\n");
console.log(`${stem}: ${mode}, moved ${moved.length} tables, ${movedBytes} bytes; ${lazy.size} items made lazy; uses rewritten on ${rewritten} lines; kept by --keep ${keep.size}`);
