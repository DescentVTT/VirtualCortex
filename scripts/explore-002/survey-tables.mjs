// Lists the top-level `const`/`static` items of a test file whose value is a pure literal
// (integers, true/false, None/Some, tuples, arrays, `&[...]`, `[x; N]` with a numeric N) and
// prints how many there are and the bytes they hold. Usage: node survey-tables.mjs <file.rs>
import fs from "node:fs";

export function items(text) {
  // Top-level items start at column 0 with `const NAME: TYPE = ` or `static NAME: TYPE = ` and end
  // at the first `;` at column 0 after a line that ends the expression (`];`, `);`, or `= x;`).
  const lines = text.split("\n");
  const out = [];
  for (let i = 0; i < lines.length; i += 1) {
    const m = /^(pub(?:\(crate\))? )?(const|static) ([A-Z0-9_]+): (.*?) = (.*)$/.exec(lines[i]);
    if (!m) continue;
    // The expression runs to the line that ends with `;` at depth zero.
    let depth = 0;
    let j = i;
    let expr = m[5];
    const scan = (s) => {
      for (const c of s) {
        if (c === "[" || c === "(" || c === "{") depth += 1;
        else if (c === "]" || c === ")" || c === "}") depth -= 1;
      }
    };
    scan(expr);
    while (!(depth === 0 && /;\s*$/.test(lines[j]))) {
      j += 1;
      if (j >= lines.length) throw new Error(`unterminated item at line ${i + 1}`);
      expr += "\n" + lines[j];
      scan(lines[j]);
    }
    expr = expr.replace(/;\s*$/, "");
    out.push({ start: i, end: j, vis: m[1] ?? "", kind: m[2], name: m[3], type: m[4], expr });
    i = j;
  }
  return out;
}

export function pureLiteral(expr) {
  // Strip line comments, then allow only the literal tokens.
  const s = expr.replace(/\/\/[^\n]*/g, "");
  const words = s.match(/[A-Za-z_][A-Za-z0-9_]*/g) ?? [];
  for (const w of words) {
    if (/^_+$/.test(w)) continue;
    if (w === "true" || w === "false" || w === "None" || w === "Some") continue;
    return false;
  }
  // Numbers: decimal only, with `_` separators, and an optional leading `-`.
  if (/0x|0b|0o|\d[a-z]/i.test(s)) return false;
  return /^[\s0-9_\-\[\](),&;A-Za-z]*$/.test(s);
}

if (process.argv[1] && process.argv[1].endsWith("survey-tables.mjs")) {
  const text = fs.readFileSync(process.argv[2], "utf8").replace(/\r\n/g, "\n");
  const all = items(text);
  let n = 0, bytes = 0, nAll = 0, bytesAll = 0;
  const big = [];
  for (const it of all) {
    nAll += 1; bytesAll += it.expr.length;
    if (pureLiteral(it.expr)) {
      n += 1; bytes += it.expr.length;
      if (it.expr.length >= 1024) big.push(it);
    }
  }
  console.log(`items ${nAll}, ${bytesAll} bytes; pure literals ${n}, ${bytes} bytes`);
  const bb = big.reduce((a, x) => a + x.expr.length, 0);
  console.log(`pure literals of at least 1 KiB: ${big.length}, ${bb} bytes`);
  const types = {};
  for (const it of big) types[it.type] = (types[it.type] ?? 0) + 1;
  for (const [t, c] of Object.entries(types).sort((a, b) => b[1] - a[1]).slice(0, 60)) console.log(c, t);
  const notPure = all.filter((x) => !pureLiteral(x.expr) && x.expr.length >= 4096).sort((a, b) => b.expr.length - a.expr.length).slice(0, 15);
  console.log("\nlargest not pure:");
  for (const x of notPure) console.log(x.expr.length, x.name, x.type, JSON.stringify((x.expr.match(/[A-Za-z_][A-Za-z0-9_]*/g) ?? []).filter((w) => !/^(true|false|None|Some)$/.test(w)).slice(0, 5)));
}
