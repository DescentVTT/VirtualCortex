#!/usr/bin/env node
// The shape of new text (ADR-0197). The documents grew past reading because each round appended
// its result in about seven places, and a rule about length that no check held would have grown
// back. This is a ratchet, not a sweep: a check over every line would fail on the whole history
// before it could block anything, so the limits hold only text the base does not already have.
//
// What it reads, against the base (`SPEC_SHAPE_BASE`, else `origin/main`, read through git as
// `spec:scope` reads it):
//
// 1. In the living documents -- the whitepaper, README.md, CLAUDE.md, CONTRIBUTING.md,
//    briefs/README.md, docs/STATUS.md, docs/research/**, docs/zh-TW/**, the ADR index, a new ADR
//    and a live brief -- each paragraph and list item is at most 1 200 characters and each table
//    cell at most 400, the index's at most 300. A block whose text the base already holds, in any
//    Markdown file, is not new text and passes whatever its length: a line left as it was, and a
//    sentence moved verbatim from one document to another. Links are compared by the file they
//    reach, so a moved sentence whose relative links were rewritten for its new place is the
//    sentence it was. So is a block whose only change is ADR-0221's rename: the new names are read
//    back to the old before the comparison (ADR-0222).
// 2. docs/STATUS.md is at most 60 lines.
// 3. A CHANGELOG.md entry, a top-level item under a version's heading, is at most 400 characters
//    when the base does not hold it.
// 4. An ADR accepted on the base keeps its body byte for byte, and its front matter changes only
//    in the keys a later ADR's lifecycle writes. The four keys ADR-0197 gave every ADR are added
//    once: to an accepted ADR whose base copy has no `title:` they may be added, and not changed
//    after.
// 5. A new ADR -- not on the base, or not accepted there -- carries `title:` of at most 80
//    characters, `summary:` of at most 300, `kind:` one of five, an H1 that is `# ADR-NNNN: ` and
//    its title, and a body that opens with "In brief".
// 6. Math renders on GitHub (ADR-0201), in every block of the living documents, in every ADR's
//    body and in every changelog entry, not only in new text: no `\operatorname`, which GitHub
//    refuses, and no backslash before ASCII punctuation in `$...$` or `$$...$$`, which Markdown
//    takes as an escape before the math is rendered. Such math is written `$`...`$` inline and as
//    a ```math fence on its own lines. A change that only moves math into those forms, or
//    `\operatorname` to `\mathrm`, is not new text, and an accepted ADR so changed keeps its body.
//
// Not read: archived briefs, code blocks, front matter outside the ADRs, and the directive
// comments themselves. A base git cannot read fails the check, since a comparison with nothing
// would pass everything. Zero dependencies. Exit 1 with each file, line and limit, 0 otherwise.
// The exports are for scripts/check-doc-shape.test.mjs.

import { execFileSync } from "node:child_process";
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { posix, resolve } from "node:path";
import { fileURLToPath } from "node:url";

export const LIMITS = {
  block: 1200,
  cell: 400,
  indexCell: 300,
  title: 80,
  summary: 300,
  statusLines: 60,
  changelogEntry: 400,
};

export const KINDS = ["decision", "protocol", "built", "measured", "process"];

// The keys a later ADR's lifecycle writes into an accepted ADR's front matter.
export const LIFECYCLE = ["status", "superseded-by", "amended-by"];

// The keys ADR-0197 added to every ADR's front matter once.
export const SHAPE_KEYS = ["title", "summary", "kind", "hypothesis"];

export const INDEX = "docs/adr/README.md";
export const STATUS = "docs/STATUS.md";
export const CHANGELOG = "CHANGELOG.md";
const ADR = /^docs\/adr\/(\d{4})-[^/]+\.md$/;
const LIVE_BRIEF = /^briefs\/\d{3}_[^/]+\.md$/;

/** Whether a path is a living document whose blocks the limits hold, its ADRs aside. */
export function isLiving(path) {
  return (
    [
      "docs/WHITEPAPER.md",
      "README.md",
      "CLAUDE.md",
      "CONTRIBUTING.md",
      "briefs/README.md",
      STATUS,
      INDEX,
    ].includes(path) ||
    path.startsWith("docs/research/") ||
    path.startsWith("docs/zh-TW/") ||
    LIVE_BRIEF.test(path)
  );
}

const lf = (text) => text.replace(/\r\n?/g, "\n");

/** A backslash before ASCII punctuation: an escape to Markdown, a macro to TeX. */
const ESCAPE = /\\[!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~]/;

/**
 * Text with GitHub's protected math read as the plain forms (ADR-0201): `$`...`$` as `$...$`, a
 * ```math fence and a line that is `$$...$$` as a `$$` block, a `$$` line without its
 * indentation, and `\operatorname{` as `\mathrm{`. Math moved into the forms GitHub renders
 * reads as the text it was.
 */
export function rendered(text) {
  return lf(text)
    .replace(/^[ \t]*```math[ \t]*\n([\s\S]*?)\n[ \t]*```[ \t]*$/gm, "$$$$\n$1\n$$$$")
    .replace(/^([ \t]*)\$\$([^$\n]+)\$\$[ \t]*$/gm, "$$$$\n$1$2\n$$$$")
    .replace(/^[ \t]*\$\$[ \t]*$/gm, "$$$$")
    .replace(/\$`([^`\n]*)`\$/g, "$$$1$$")
    .replace(/\\operatorname\{/g, "\\mathrm{");
}

/**
 * The math in a text GitHub renders wrongly, one description each (ADR-0201): `\operatorname`
 * anywhere in math, and a backslash before ASCII punctuation in `$...$` or `$$...$$` outside
 * the protected `$`...`$`. Code spans are not math.
 */
export function badMath(text) {
  const found = [];
  for (const m of text.matchAll(/\$`([^`]*)`\$/g)) {
    if (/\\operatorname\b/.test(m[1])) found.push("\\operatorname, which GitHub refuses");
  }
  const plain = text.replace(/\$`[^`]*`\$/g, "").replace(/`[^`]*`/g, "");
  for (const m of plain.matchAll(/\$\$([^$]+?)\$\$|\$([^$\s][^$]*?)\$/g)) {
    const tex = m[1] ?? m[2];
    if (/\\operatorname\b/.test(tex)) found.push("\\operatorname, which GitHub refuses");
    const esc = ESCAPE.exec(tex);
    if (esc) found.push(`${esc[0]} outside $\`...\`$, which Markdown takes before the math`);
  }
  return [...new Set(found)];
}

/** The lines of the ```math fences of a body that name `\operatorname`, 1-based from `firstLine`. */
export function fencedOperatorname(body, firstLine = 1) {
  const out = [];
  let inMath = false;
  let fence = null;
  lf(body)
    .split("\n")
    .forEach((raw, i) => {
      const t = raw.trim();
      if (fence !== null) {
        if (t.startsWith(fence)) {
          fence = null;
          inMath = false;
        } else if (inMath && /\\operatorname\b/.test(raw)) out.push(firstLine + i);
        return;
      }
      const open = /^(`{3,}|~{3,})\s*(\S*)/.exec(t);
      if (open) {
        fence = open[1];
        inMath = open[2] === "math";
      }
    });
  return out;
}
const length = (text) => [...text].length;

/** The front matter's lines and the body, or no front matter and the whole text as the body. */
export function splitFrontMatter(text) {
  const t = lf(text);
  if (!t.startsWith("---\n")) return { front: null, body: t, bodyLine: 1 };
  const end = t.indexOf("\n---\n", 3);
  if (end < 0) return { front: null, body: t, bodyLine: 1 };
  const front = t.slice(4, end);
  return { front, body: t.slice(end + 5), bodyLine: front.split("\n").length + 3 };
}

/** The front matter's top-level keys and their raw values, as written. */
export function frontKeys(front) {
  const keys = new Map();
  if (front === null) return keys;
  let current = null;
  for (const line of front.split("\n")) {
    const m = /^([A-Za-z_][\w.-]*):(.*)$/.exec(line);
    if (m) {
      current = m[1].toLowerCase();
      keys.set(current, m[2].trim());
    } else if (current !== null && line.trim() !== "") {
      keys.set(current, `${keys.get(current)}\n${line}`);
    }
  }
  return keys;
}

/** A scalar's value as YAML reads it: plain, single-quoted or double-quoted on one line. */
export function scalar(raw) {
  if (raw === undefined) return undefined;
  const v = raw.trim();
  if (v.startsWith("'") && v.endsWith("'") && v.length >= 2) return v.slice(1, -1).replace(/''/g, "'");
  if (v.startsWith('"') && v.endsWith('"') && v.length >= 2) {
    return v.slice(1, -1).replace(/\\(["\\/])/g, "$1");
  }
  return v;
}

/** Inline comments taken out, so a directive inside a line is not text. */
const uncomment = (line) => line.replace(/<!--.*?-->/g, "");

/** A table row's cells, split at the pipes that are not escaped. */
export function cells(row) {
  const out = [];
  let cur = "";
  for (let i = 0; i < row.length; i += 1) {
    const ch = row[i];
    if (ch === "\\" && row[i + 1] === "|") {
      cur += "\\|";
      i += 1;
    } else if (ch === "|") {
      out.push(cur);
      cur = "";
    } else {
      cur += ch;
    }
  }
  out.push(cur);
  const trimmed = row.trim();
  return out.slice(trimmed.startsWith("|") ? 1 : 0, trimmed.endsWith("|") ? -1 : undefined).map((c) => c.trim());
}

const LIST = /^(\s*)([-*+]|\d+[.)])\s+(\[[ xX]\]\s+)?/;
const DELIMITER = /^\s*\|?\s*:?-{3,}:?\s*(\|\s*:?-{3,}:?\s*)*\|?\s*$/;

/**
 * The blocks of a Markdown body: `{ kind, line, text }`, kind "paragraph", "item" or "cell", line
 * 1-based from `firstLine`. A list item is its own line and the lines continuing it up to a blank
 * line, another item or another block; a paragraph indented under an item after a blank line is a
 * paragraph of its own. Code blocks, headings, comments and table delimiter rows are not blocks.
 */
export function blocks(body, firstLine = 1) {
  const lines = lf(body).split("\n");
  const out = [];
  let open = null;
  let fence = null;
  let comment = false;
  const close = () => {
    if (open !== null) out.push({ kind: open.kind, line: open.line, text: open.parts.join(" ").replace(/\s+/g, " ").trim() });
    open = null;
  };
  for (let i = 0; i < lines.length; i += 1) {
    const raw = lines[i];
    const line = firstLine + i;
    if (fence !== null) {
      if (raw.trim().startsWith(fence)) fence = null;
      continue;
    }
    if (comment) {
      if (raw.includes("-->")) comment = false;
      continue;
    }
    const trimmed = raw.trim();
    const fenceOpen = /^(`{3,}|~{3,})/.exec(trimmed);
    if (fenceOpen) {
      close();
      fence = fenceOpen[1];
      continue;
    }
    if (trimmed.startsWith("<!--")) {
      close();
      if (!trimmed.includes("-->")) comment = true;
      continue;
    }
    const text = uncomment(raw);
    if (text.trim() === "") {
      close();
      continue;
    }
    if (/^\s{0,3}#{1,6}(\s|$)/.test(text) || /^\s*(-{3,}|\*{3,}|_{3,})\s*$/.test(text)) {
      close();
      continue;
    }
    if (text.trim().startsWith("|")) {
      close();
      if (!DELIMITER.test(text)) for (const c of cells(text.trim())) out.push({ kind: "cell", line, text: c });
      continue;
    }
    const item = LIST.exec(text);
    if (item) {
      close();
      open = { kind: "item", line, parts: [text.slice(item[0].length)] };
      continue;
    }
    const quoted = text.replace(/^\s*>\s?/, "");
    if (open === null) open = { kind: "paragraph", line, parts: [] };
    open.parts.push(quoted.trim());
  }
  close();
  return out;
}

/**
 * Text with ADR-0221's names read back, each new name to the old one, the longer first: the
 * address, the npm package, the header's type, the project, the magic, the image's extension and
 * a crate's prefix (ADR-0222). A block whose only change is the project's rename reads as the
 * text it was, as math moved into GitHub's forms does (ADR-0201); a block changed otherwise is
 * still new text.
 */
export function renamedBack(text) {
  return text
    .replace(/Menteion\/Menteion/g, "DescentVTT/VirtualCortex")
    .replace(/menteion-specs/g, "virtualcortex-specs")
    .replace(/MenteionFileHeader/g, "CortexFileHeader")
    .replace(/Menteion/g, "VirtualCortex")
    .replace(/MENTEION/g, "VCORTEX1")
    .replace(/\.menteion\b/g, ".cortex")
    .replace(/\bmenteion-/g, "cortex-")
    .replace(/\bmenteion_/g, "cortex_");
}

/**
 * Text compared by what it says and where its links lead: ADR-0221's names read back
 * (`renamedBack`), every relative link resolved from the file it is in to a path from the
 * repository's root, an anchor alone to the file's own, and every run of whitespace one space.
 */
export function normalise(text, path) {
  const dir = posix.dirname(path);
  return renamedBack(rendered(text))
    .replace(/\]\(([^)\s]+)((?:\s+"[^"]*")?)\)/g, (all, target, title) => {
      if (/^[a-z][a-z0-9+.-]*:/i.test(target)) return all;
      if (target.startsWith("#")) return `](/${path}${target}${title})`;
      const [file, anchor] = target.split(/(?=#)/);
      const joined = posix.normalize(posix.join(dir, file));
      return `](/${joined}${anchor ?? ""}${title})`;
    })
    .replace(/\s+/g, " ");
}

/**
 * The base's Markdown as the blocks this check reads, each normalised, one a line: a block of the
 * tree is known when its text lies inside one of them, which a block left as it was and a sentence
 * moved out of one both do, and a block made by joining two does not.
 */
export function corpus(base) {
  const parts = [];
  for (const [path, text] of base) {
    if (!path.endsWith(".md")) continue;
    const { body, bodyLine } = splitFrontMatter(text);
    for (const block of blocks(body, bodyLine)) parts.push(normalise(block.text, path).trim());
  }
  return parts.join("\n");
}

const isAdr = (path) => ADR.test(path);

/** The changelog's entries: each top-level item under a `## ` heading, with what continues it. */
export function changelogEntries(text) {
  const lines = lf(text).split("\n");
  const out = [];
  let open = null;
  let fence = false;
  let underVersion = false;
  const close = () => {
    if (open !== null) out.push({ line: open.line, text: open.parts.join(" ").replace(/\s+/g, " ").trim() });
    open = null;
  };
  for (let i = 0; i < lines.length; i += 1) {
    const raw = lines[i];
    if (raw.trim().startsWith("```")) {
      fence = !fence;
      continue;
    }
    if (fence) continue;
    if (/^##\s/.test(raw)) {
      close();
      underVersion = true;
      continue;
    }
    if (/^#{3,}\s/.test(raw)) {
      close();
      continue;
    }
    if (!underVersion) continue;
    if (/^[-*+]\s/.test(raw)) {
      close();
      open = { line: i + 1, parts: [raw.replace(/^[-*+]\s+/, "")] };
      continue;
    }
    if (open !== null && (raw.trim() === "" || /^\s/.test(raw))) {
      if (raw.trim() !== "") open.parts.push(raw.trim().replace(/^[-*+]\s+/, ""));
      continue;
    }
    close();
  }
  close();
  return out;
}

/**
 * The problems of a tree against its base. `head` and `base` are Maps from a path, from the
 * repository's root with `/`, to its text; `head` holds the files this check reads, `base` every
 * Markdown file of the base.
 */
export function checkShape(head, base) {
  const problems = [];
  const known = corpus(base);
  const isNew = (text, path) => !known.includes(normalise(text, path).trim());

  for (const [path, text] of head) {
    if (path.startsWith("briefs/archive/")) continue;

    if (path === CHANGELOG) {
      const before = base.has(path)
        ? changelogEntries(base.get(path))
            .map((e) => normalise(e.text, path).trim())
            .join("\n")
        : "";
      for (const entry of changelogEntries(text)) {
        for (const bad of badMath(entry.text)) problems.push(`${path}:${entry.line}: math GitHub renders wrongly: ${bad}`);
        const n = length(entry.text);
        if (n > LIMITS.changelogEntry && !before.includes(normalise(entry.text, path).trim())) {
          problems.push(`${path}:${entry.line}: a new entry of ${n} characters, over ${LIMITS.changelogEntry}`);
        }
      }
      continue;
    }

    if (isAdr(path)) {
      problems.push(...checkAdr(path, text, base.get(path), isNew));
      continue;
    }

    if (!isLiving(path)) continue;
    if (path === STATUS) {
      const n = lf(text).replace(/\n$/, "").split("\n").length;
      if (n > LIMITS.statusLines) problems.push(`${path}: ${n} lines, over ${LIMITS.statusLines}`);
    }
    const { body, bodyLine } = splitFrontMatter(text);
    problems.push(...checkBlocks(path, body, bodyLine, isNew, path === INDEX ? LIMITS.indexCell : LIMITS.cell));
  }
  return problems;
}

function checkBlocks(path, body, bodyLine, isNew, cellLimit) {
  const problems = [];
  for (const line of fencedOperatorname(body, bodyLine)) {
    problems.push(`${path}:${line}: math GitHub renders wrongly: \\operatorname, which GitHub refuses`);
  }
  for (const block of blocks(body, bodyLine)) {
    for (const bad of badMath(block.text)) problems.push(`${path}:${block.line}: math GitHub renders wrongly: ${bad}`);
    const limit = block.kind === "cell" ? cellLimit : LIMITS.block;
    const n = length(block.text);
    if (n > limit && isNew(block.text, path)) {
      const what = block.kind === "cell" ? "a table cell" : block.kind === "item" ? "a list item" : "a paragraph";
      problems.push(`${path}:${block.line}: ${what} of ${n} characters, over ${limit}`);
    }
  }
  return problems;
}

/** An accepted ADR held to its base copy; a new one held to ADR-0197's shape. */
function checkAdr(path, text, before, isNew) {
  const problems = [];
  const here = splitFrontMatter(text);
  const keys = frontKeys(here.front);
  const there = before === undefined ? null : splitFrontMatter(before);
  const baseKeys = there === null ? null : frontKeys(there.front);
  const accepted = baseKeys !== null && scalar(baseKeys.get("status")) === "accepted";

  if (accepted) {
    if (here.body !== there.body && rendered(here.body) !== rendered(there.body)) {
      problems.push(`${path}: the body of an accepted ADR differs from its base copy`);
    }
    for (const line of fencedOperatorname(here.body, here.bodyLine)) {
      problems.push(`${path}:${line}: math GitHub renders wrongly: \\operatorname, which GitHub refuses`);
    }
    for (const block of blocks(here.body, here.bodyLine)) {
      for (const bad of badMath(block.text)) problems.push(`${path}:${block.line}: math GitHub renders wrongly: ${bad}`);
    }
    const completing = !baseKeys.has("title");
    for (const key of new Set([...keys.keys(), ...baseKeys.keys()])) {
      if (keys.get(key) === baseKeys.get(key) || LIFECYCLE.includes(key)) continue;
      if (completing && SHAPE_KEYS.includes(key) && !baseKeys.has(key)) continue;
      problems.push(`${path}: the front matter's ${key}: changed on an accepted ADR, which only its lifecycle may change`);
    }
    if (completing && keys.has("title")) problems.push(...checkShapeKeys(path, keys));
    return problems;
  }

  problems.push(...checkShapeKeys(path, keys));
  const number = ADR.exec(path)[1];
  const title = scalar(keys.get("title"));
  const h1 = /^# (.*)$/m.exec(here.body)?.[1];
  if (title !== undefined && h1 !== `ADR-${number}: ${title}`) {
    problems.push(`${path}: the H1 is not '# ADR-${number}: ' and the title`);
  }
  const afterH1 = here.body.slice(here.body.search(/^# .*$/m)).split("\n").slice(1).join("\n").trimStart();
  if (!/^\*\*In brief\*\*/.test(afterH1)) problems.push(`${path}: the body does not open with **In brief**`);
  problems.push(...checkBlocks(path, here.body, here.bodyLine, isNew, LIMITS.cell));
  return problems;
}

function checkShapeKeys(path, keys) {
  const problems = [];
  const title = scalar(keys.get("title"));
  const summary = scalar(keys.get("summary"));
  const kind = scalar(keys.get("kind"));
  if (title === undefined || title === "") problems.push(`${path}: no title: in the front matter`);
  else if (length(title) > LIMITS.title) problems.push(`${path}: the title is ${length(title)} characters, over ${LIMITS.title}`);
  if (summary === undefined || summary === "") problems.push(`${path}: no summary: in the front matter`);
  else if (length(summary) > LIMITS.summary) {
    problems.push(`${path}: the summary is ${length(summary)} characters, over ${LIMITS.summary}`);
  }
  if (!KINDS.includes(kind)) problems.push(`${path}: kind: ${kind ?? "(none)"} is not one of ${KINDS.join(", ")}`);
  return problems;
}

// --- reading the tree and the base ------------------------------------------------------------

function walk(dir, out) {
  if (!existsSync(dir)) return out;
  for (const name of readdirSync(dir)) {
    const path = dir === "." ? name : `${dir}/${name}`;
    if (statSync(path).isDirectory()) walk(path, out);
    else if (name.endsWith(".md")) out.push(path);
  }
  return out;
}

/** The files of the working tree this check reads. */
export function headFiles() {
  const paths = [
    "README.md",
    "CLAUDE.md",
    "CONTRIBUTING.md",
    CHANGELOG,
    ...walk("briefs", []).filter((p) => !p.startsWith("briefs/archive/")),
    ...walk("docs", []).filter((p) => isLiving(p) || isAdr(p)),
  ];
  const head = new Map();
  for (const path of paths) if (existsSync(path)) head.set(path, readFileSync(path, "utf8"));
  return head;
}

/** Every Markdown file of a commit, read through one `git cat-file --batch`. */
export function baseFiles(ref) {
  const names = execFileSync("git", ["ls-tree", "-r", "--name-only", ref], { encoding: "utf8", maxBuffer: 1 << 26 })
    .split("\n")
    .filter((p) => p.endsWith(".md"));
  const out = execFileSync("git", ["cat-file", "--batch"], {
    input: names.map((p) => `${ref}:${p}`).join("\n") + "\n",
    maxBuffer: 1 << 30,
  });
  const base = new Map();
  let at = 0;
  for (const path of names) {
    const eol = out.indexOf(10, at);
    const header = out.subarray(at, eol).toString("utf8");
    const size = Number(header.split(" ")[2]);
    base.set(path, out.subarray(eol + 1, eol + 1 + size).toString("utf8"));
    at = eol + 1 + size + 1;
  }
  return base;
}

const invoked =
  process.argv[1] !== undefined &&
  resolve(process.argv[1]).toLowerCase() === fileURLToPath(import.meta.url).toLowerCase();

if (invoked) {
  const ref = process.env.SPEC_SHAPE_BASE || "origin/main";
  try {
    execFileSync("git", ["rev-parse", "--verify", "--quiet", `${ref}^{commit}`], { stdio: "ignore" });
  } catch {
    console.error(`check-doc-shape: the base ${ref} is not a commit git can read, so nothing could be compared`);
    process.exit(1);
  }
  const head = headFiles();
  const problems = checkShape(head, baseFiles(ref));
  for (const p of problems) console.error(p);
  if (problems.length > 0) {
    console.error(`check-doc-shape: ${problems.length} problem(s) against ${ref}`);
    process.exit(1);
  }
  console.log(`check-doc-shape: ${head.size} file(s) read against ${ref}; new text within ADR-0197's limits, math as GitHub renders it`);
}
