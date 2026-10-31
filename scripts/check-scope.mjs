#!/usr/bin/env node
// A brief's scope (ADR-0191). From brief 075 on, a brief declares what its round may write,
// `affectedFiles`, and what it may not, `protectedFiles`; `spec-brief archive --strict` refuses a
// round that wrote outside the one or into the other. Two things neither spec-brief rule holds:
//
// 1. A brief from 075 on, live or archived, declares both, each with at least one pattern. A
//    brief with no scope would pass the archive's check over nothing.
// 2. A brief's protected patterns never shrink against the base branch: every pattern the live
//    brief protects there, the brief here protects still, live or archived. A round may widen
//    what it writes (each widening is printed, for the review) but may not unprotect what its
//    brief protected, since the archive's refusal would then have nothing to refuse.
//
// Reads `.spec-brief.json` and the briefs with spec-brief's own parser, and the base's copies with
// `git show`; the base is `SPEC_SCOPE_BASE` or `origin/main`. A base git cannot read fails the
// check: a comparison with nothing reads as a clean one. Exit 1 with each brief and the problem,
// 0 otherwise. `checkScope` is exported for scripts/check-scope.test.mjs.

import { execFileSync } from "node:child_process";
import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { parseBrief, parseConfig } from "@spec-family/spec-brief";

// The first brief held to a declared scope.
export const SCOPE_FROM = 75;

// A brief's file name, as `.spec-brief.json`'s `files` has it.
export const BRIEF = /^([0-9]{3})_.*\.md$/;

/**
 * The problems and the notes of a set of briefs.
 *
 * `briefs`: `{ path, name, phase, text }` for each brief here, `phase` "active" or "archived";
 * `base`: a function from a live brief's file name to its text on the base, or `null` where the
 * base has no such live brief; `config`: spec-brief's configuration.
 */
export function checkScope(briefs, base, config) {
  const problems = [];
  const notes = [];
  for (const { path, name, phase, text } of briefs) {
    const match = BRIEF.exec(name);
    if (match === null || Number(match[1]) < SCOPE_FROM) continue;
    const here = parseBrief(path, text, config, phase);
    for (const field of ["affectedFiles", "protectedFiles"]) {
      if (here[field].length === 0) problems.push(`${path}: declares no ${field}`);
    }
    const before = base(name);
    if (before === null) continue;
    const there = parseBrief(`briefs/${name}`, before, config, "active");
    const dropped = there.protectedFiles.filter((p) => !here.protectedFiles.includes(p));
    for (const p of dropped) {
      problems.push(`${path}: no longer protects ${p}, which the base's brief protects`);
    }
    const widened = here.affectedFiles.filter((p) => !there.affectedFiles.includes(p));
    for (const p of widened) notes.push(`${path}: affectedFiles widened by ${p} since the base`);
  }
  return { problems, notes };
}

function briefsIn(dir, phase) {
  return readdirSync(dir)
    .filter((name) => BRIEF.test(name))
    .sort()
    .map((name) => ({ path: `${dir}/${name}`, name, phase, text: readFileSync(join(dir, name), "utf8") }));
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const ref = process.env.SPEC_SCOPE_BASE || "origin/main";
  try {
    execFileSync("git", ["rev-parse", "--verify", "--quiet", `${ref}^{commit}`], { stdio: "ignore" });
  } catch {
    console.error(`check-scope: the base ${ref} is not a commit git can read, so nothing could be compared`);
    process.exit(1);
  }
  const base = (name) => {
    try {
      return execFileSync("git", ["show", `${ref}:briefs/${name}`], { encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] });
    } catch {
      return null;
    }
  };
  const config = parseConfig(readFileSync(".spec-brief.json", "utf8"), ".spec-brief.json");
  const briefs = [...briefsIn("briefs", "active"), ...briefsIn("briefs/archive", "archived")];
  const { problems, notes } = checkScope(briefs, base, config);
  for (const n of notes) console.log(`note: ${n}`);
  if (problems.length > 0) {
    for (const p of problems) console.error(p);
    console.error(`check-scope: ${problems.length} problem(s)`);
    process.exit(1);
  }
  const held = briefs.filter((b) => Number(BRIEF.exec(b.name)[1]) >= SCOPE_FROM).length;
  console.log(`check-scope: ${held} brief(s) from ${String(SCOPE_FROM).padStart(3, "0")} declare their scope, none unprotects against ${ref}`);
}
