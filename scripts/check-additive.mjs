#!/usr/bin/env node
// Whether a round's change only adds (ADR-0215). A round's calibration runs every whole-domain
// test of the tree but its own, to show that its build moves no number an earlier round pinned.
// A change that edits no line a Rust build reads, and only inserts whole items into test files
// under names the tree does not use, cannot move such a number: the code every earlier test runs
// is the code it ran. Such a round's calibration is its own calibration tests alone; the schedule
// runs every test each week.
//
//   node scripts/check-additive.mjs <base> <head>
//
// Reads `git diff <base>...<head>`, from their merge base. A file a Rust build reads -- anything
// under runtime/, crates/, benches/, testkit/ or .cargo/, a `.rs` file anywhere, and the
// workspace's manifests and toolchain files -- is additive only when:
// 1. it lies under a package's `tests/` directory;
// 2. it is new, or every hunk of it inserts lines and removes none;
// 3. in a file that was there, each insertion goes in at the top level, after a line that ends an
//    item (`}` or `;`, or nothing but blank lines and comments before it), and is itself whole
//    items: its brackets balanced, its first line an item's, its last ending in `}` or `;`;
// 4. no item it inserts at the top level is a `use`, an `impl` of a type the base names, or an
//    item whose name the sources its crate can see already hold -- the file's own text, the
//    modules the test crates share under `tests/` directories, the library and the state crates --
//    which could shadow what a glob import gave the code that was there, or turn a binding in a
//    pattern into a constant. Another test crate's root is its own crate, and is not read.
// Brackets are counted outside comments, strings and character literals, all three kinds, since
// a table's rows sit inside `[`. Exit 0 when the change only adds, 1 with each file and reason
// when it does not, 2 on a usage or git error. The exports are for scripts/check-additive.test.mjs.

import { execFileSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** Whether a Rust build reads the path. */
export function readByRust(path) {
  return (
    /^(runtime|crates|benches|testkit|\.cargo)\//.test(path) ||
    path.endsWith(".rs") ||
    ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "clippy.toml", "rustfmt.toml", ".rustfmt.toml"].includes(path)
  );
}

/** Whether the path lies under a package's `tests/` directory. */
export const underTests = (path) => /^(runtime|crates|benches)\/[^/]+\/tests\//.test(path);

/**
 * The bracket depth at the end of each line of Rust text, `(`, `[` and `{` alike, outside line
 * and nested block comments, strings, raw strings and character literals; and, for each line,
 * its last character outside comments that is not white space, or "" for none.
 */
export function scan(text) {
  const depth = [];
  const last = [];
  let d = 0;
  let block = 0;
  let str = null; // null, or the closing delimiter of the string being read
  let lineLast = "";
  const n = text.length;
  for (let i = 0; i < n; i += 1) {
    const c = text[i];
    if (c === "\n") {
      depth.push(d);
      last.push(lineLast);
      lineLast = "";
      continue;
    }
    if (block > 0) {
      if (c === "*" && text[i + 1] === "/") (block -= 1), (i += 1);
      else if (c === "/" && text[i + 1] === "*") (block += 1), (i += 1);
      continue;
    }
    if (str !== null) {
      lineLast = '"';
      if (str === '"' && c === "\\") {
        i += 1;
        continue;
      }
      if (c === '"' && text.startsWith(str, i)) (i += str.length - 1), (str = null);
      continue;
    }
    if (c === "/" && text[i + 1] === "/") {
      const end = text.indexOf("\n", i);
      i = (end < 0 ? n : end) - 1;
      continue;
    }
    if (c === "/" && text[i + 1] === "*") {
      block = 1;
      i += 1;
      continue;
    }
    const raw = c === "r" || c === "b" ? /^b?r(#*)"/.exec(text.slice(i, i + 260)) : null;
    if (raw && !/[A-Za-z0-9_]/.test(text[i - 1] ?? "")) {
      str = `"${raw[1]}`;
      i += raw[0].length - 1;
      lineLast = '"';
      continue;
    }
    if (c === '"') {
      str = '"';
      lineLast = '"';
      continue;
    }
    if (c === "'") {
      const ch = /^'(\\(u\{[0-9A-Fa-f]+\}|x[0-9A-Fa-f]{2}|.)|[^\\'\n])'/.exec(text.slice(i, i + 16));
      if (ch) {
        i += ch[0].length - 1;
        lineLast = "'";
      }
      continue; // a lifetime otherwise
    }
    if (c === "(" || c === "[" || c === "{") d += 1;
    else if (c === ")" || c === "]" || c === "}") d -= 1;
    if (!/\s/.test(c)) lineLast = c;
  }
  if (n === 0 || text[n - 1] !== "\n") {
    depth.push(d);
    last.push(lineLast);
  }
  return { depth, last };
}

/** The hunks of a `git diff -U0` of one file: `{ oldStart, oldCount, added: [lines] }`. */
export function hunks(diff) {
  const out = [];
  let open = null;
  for (const line of diff.split("\n")) {
    const h = /^@@ -(\d+)(?:,(\d+))? \+\d+(?:,\d+)? @@/.exec(line);
    if (h) {
      open = { oldStart: Number(h[1]), oldCount: h[2] === undefined ? 1 : Number(h[2]), added: [] };
      out.push(open);
      continue;
    }
    if (open && line.startsWith("+") && !line.startsWith("+++")) open.added.push(line.slice(1));
  }
  return out;
}

const ITEM = /^\s*(?:pub(?:\([^)]*\))?\s+)?(?:(?:const|async|unsafe|extern\s+"[^"]*")\s+)*(fn|const|static|struct|enum|union|type|trait|mod)\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)/;
const MACRO = /^\s*macro_rules!\s*([A-Za-z_][A-Za-z0-9_]*)/;
const IMPL = /^\s*(?:unsafe\s+)?impl\b/;
const USE = /^\s*(?:pub(?:\([^)]*\))?\s+)?use\b/;
const STARTS_ITEM = /^\s*(?:#|pub\b|fn\b|const\b|static\b|struct\b|enum\b|union\b|type\b|trait\b|impl\b|mod\b|use\b|extern\b|async\b|unsafe\b|macro_rules!|[A-Za-z_][A-Za-z0-9_]*!)/;

/** The type an `impl` line implements for, its last path segment, or null. */
export function implType(line) {
  let rest = line.replace(/^\s*(?:unsafe\s+)?impl\b\s*/, "");
  if (rest.startsWith("<")) {
    let d = 0;
    let i = 0;
    for (; i < rest.length; i += 1) {
      if (rest[i] === "<") d += 1;
      else if (rest[i] === ">" && (d -= 1) === 0) break;
    }
    rest = rest.slice(i + 1);
  }
  const at = rest.lastIndexOf(" for ");
  if (at >= 0) rest = rest.slice(at + 5);
  const m = /^\s*&?\s*(?:'\w+\s+)?(?:mut\s+)?((?:[A-Za-z_]\w*::)*[A-Za-z_]\w*)/.exec(rest);
  return m ? m[1].split("::").pop() : null;
}

/**
 * What an inserted run of lines puts at the top level: `{ names, impls, uses, problems }`, the
 * problems those of whole items (rule 3's second half).
 */
export function inserted(lines) {
  const text = lines.join("\n") + "\n";
  const { depth, last } = scan(text);
  const names = [];
  const impls = [];
  const problems = [];
  let uses = 0;
  let before = 0;
  let firstSeen = false;
  let lastSig = "";
  for (let i = 0; i < lines.length; i += 1) {
    const line = lines[i];
    const sig = last[i];
    if (before === 0 && sig !== "") {
      if (!firstSeen && !STARTS_ITEM.test(line)) problems.push(`its first line is not an item's: '${line.trim()}'`);
      firstSeen = true;
      const item = ITEM.exec(line);
      const macro = MACRO.exec(line);
      // `const _` names nothing, so it shadows nothing.
      if (item && item[2] !== "_") names.push(item[2]);
      else if (macro) names.push(macro[1]);
      if (IMPL.test(line)) {
        const t = implType(line);
        if (t) impls.push(t);
      }
      if (USE.test(line)) uses += 1;
    }
    if (depth[i] < 0) {
      problems.push("its brackets close what it did not open");
      break;
    }
    before = depth[i];
    if (sig !== "") lastSig = sig;
  }
  if (before !== 0) problems.push("its brackets do not balance");
  if (firstSeen && lastSig !== "}" && lastSig !== ";") problems.push("its last line does not end an item");
  return { names, impls, uses, problems };
}

/** Whether an insertion after base line `after` (1-based; 0 is the file's start) is at the top level after a whole item. */
export function insertsAtTop(baseScan, after) {
  if (after === 0) return true;
  if ((baseScan.depth[after - 1] ?? 0) !== 0) return false;
  for (let i = after - 1; i >= 0; i -= 1) {
    const s = baseScan.last[i];
    if (s !== "") return s === "}" || s === ";";
  }
  return true;
}

/** The names of `names` that `text` holds as whole words. */
export function wordsIn(text, names) {
  const held = new Set();
  for (const n of names) {
    const re = new RegExp(`(^|[^A-Za-z0-9_])${n}($|[^A-Za-z0-9_])`);
    if (re.test(text)) held.add(n);
  }
  return held;
}

/**
 * The verdict on a change. `changes`: `{ status, path, diff }` for each changed file, `status`
 * git's letter and `diff` its `-U0` text; `baseText`: path to the base's text; `existing`: an array
 * of names to the set of them the base's Rust sources other than the test crates' roots hold. A
 * test crate's root sees its own text, the modules the crates share under `tests/*\/`, and the
 * library and the state crates; another crate's root it cannot see. Returns
 * `{ additive, problems, files }`.
 */
export function checkAdditive(changes, baseText, existing) {
  const problems = [];
  const names = new Map();
  const impls = new Map();
  const own = new Map();
  let files = 0;
  for (const { status, path, diff } of changes) {
    if (!readByRust(path)) continue;
    files += 1;
    if (!underTests(path)) {
      problems.push(`${path}: a file a Rust build reads, outside a package's tests/`);
      continue;
    }
    if (status === "A") continue;
    if (status !== "M") {
      problems.push(`${path}: ${status === "D" ? "deleted" : status.startsWith("R") ? "renamed" : `changed as ${status}`}`);
      continue;
    }
    if (!path.endsWith(".rs")) {
      problems.push(`${path}: a file that was there, changed`);
      continue;
    }
    const text = baseText(path);
    const base = scan(text);
    const mine = [];
    for (const h of hunks(diff)) {
      if (h.oldCount !== 0) {
        problems.push(`${path}:${h.oldStart}: ${h.oldCount} line(s) of the base removed or changed`);
        continue;
      }
      if (!insertsAtTop(base, h.oldStart)) {
        problems.push(`${path}:${h.oldStart}: an insertion inside an item, or after a line that does not end one`);
        continue;
      }
      const got = inserted(h.added);
      for (const p of got.problems) problems.push(`${path}:${h.oldStart}: an insertion whose ${p.replace(/^its /, "")}`);
      if (got.uses > 0) problems.push(`${path}:${h.oldStart}: an insertion with a \`use\`, which can shadow a glob import`);
      for (const n of got.names) names.set(n, `${path}:${h.oldStart}`), mine.push(n);
      for (const t of got.impls) impls.set(t, `${path}:${h.oldStart}`), mine.push(t);
    }
    for (const n of wordsIn(text, mine)) own.set(n, true);
  }
  const asked = [...new Set([...names.keys(), ...impls.keys()])];
  const held = asked.length > 0 ? existing(asked) : new Set();
  for (const n of own.keys()) held.add(n);
  for (const [n, at] of names) if (held.has(n)) problems.push(`${at}: an item named \`${n}\`, a name the base already uses`);
  for (const [t, at] of impls) if (held.has(t)) problems.push(`${at}: an \`impl\` for \`${t}\`, a type the base names`);
  return { additive: problems.length === 0, problems, files };
}

function git(args) {
  return execFileSync("git", args, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], maxBuffer: 1 << 30 });
}

function main(argv) {
  const [base, head] = argv;
  if (!base || !head) {
    console.error("usage: node scripts/check-additive.mjs <base> <head>");
    return 2;
  }
  let mergeBase;
  try {
    mergeBase = git(["merge-base", base, head]).trim();
  } catch (e) {
    console.error(`check-additive: git cannot read ${base} and ${head}: ${e.message.split("\n")[0]}`);
    return 2;
  }
  const changes = git(["diff", "--name-status", "-M", mergeBase, head])
    .split("\n")
    .filter((l) => l.trim() !== "")
    .map((l) => {
      const cells = l.split("\t");
      return { status: cells[0], path: cells[cells.length - 1] };
    })
    .map((c) => ({ ...c, diff: c.status === "M" && readByRust(c.path) ? git(["diff", "-U0", mergeBase, head, "--", c.path]) : "" }));
  const baseText = (path) => git(["show", `${mergeBase}:${path}`]);
  const existing = (names) => {
    const dir = mkdtempSync(join(tmpdir(), "check-additive-"));
    try {
      const file = join(dir, "names.txt");
      writeFileSync(file, names.join("\n") + "\n");
      let out = "";
      try {
        // Every Rust source but the test crates' roots, `<package>/tests/*.rs`, each its own crate.
        out = git(["grep", "-o", "-h", "-w", "-F", "-f", file, mergeBase, "--", ":(glob)**/*.rs", ":(exclude,glob)*/*/tests/*.rs"]);
      } catch (e) {
        if (e.status !== 1) throw e; // 1 is git grep's "nothing found"
      }
      return new Set(out.split("\n").map((s) => s.trim()).filter(Boolean));
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  };
  const { additive, problems, files } = checkAdditive(changes, baseText, existing);
  const at = `${mergeBase.slice(0, 7)}...${git(["rev-parse", "--short", head]).trim()}`;
  if (additive) {
    console.log(`check-additive: ${at} only adds: ${files} file(s) a Rust build reads, each whole items inserted into a package's tests/ (ADR-0215)`);
    return 0;
  }
  for (const p of problems) console.error(p);
  console.error(`check-additive: ${at} does not only add, ${problems.length} reason(s): the calibration runs every test (ADR-0215)`);
  return 1;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) process.exitCode = main(process.argv.slice(2));
