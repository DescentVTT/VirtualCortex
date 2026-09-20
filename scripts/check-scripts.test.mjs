// `package.json`'s own tests (node:test, no dependency): `npm run spec:scripts:test`.
//
// CLAUDE.md: "a check here and not in CI is a gate nobody enforces". On 2026-09-20 a round added
// `spec:decisions` and `spec:decisions:test` to `npm run spec`, to CONTRIBUTING.md and to
// CLAUDE.md, and not to the workflow, so for two days a check the documents called blocking ran
// on a developer machine only (F-44). The workflow now runs `npm run spec`, one step, so the
// list lives in one file; these hold that list to containing what it must.

import { ok } from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

const { scripts } = JSON.parse(readFileSync("package.json", "utf8"));
const spec = scripts.spec;
const runs = (name) => new RegExp(`(^|&&)\\s*npm run ${name.replace(/:/g, "\\:")}\\s*($|&&)`).test(spec);

test("every documentation check runs under `npm run spec`, which is the one command CI runs", () => {
  for (const name of Object.keys(scripts)) {
    if (name === "spec" || !name.startsWith("spec:")) continue;
    ok(runs(name), `npm run spec does not run ${name}; a check outside it runs nowhere in CI`);
  }
});

test("every test script runs under `npm run spec`, whatever it is named", () => {
  for (const name of Object.keys(scripts)) {
    if (!name.endsWith(":test")) continue;
    ok(runs(name), `npm run spec does not run ${name}`);
  }
});

test("`npm run spec` runs nothing but the other scripts, so its list is readable", () => {
  for (const step of spec.split("&&").map((s) => s.trim())) {
    const name = /^npm run ([\w:-]+)$/.exec(step)?.[1];
    ok(name !== undefined, `'${step}' is not 'npm run <script>'`);
    ok(name in scripts, `npm run spec runs ${name}, which package.json does not define`);
  }
});

test("a script that takes an argument is not in `npm run spec`, because it cannot run without one", () => {
  // `mutants:timeouts` reads a sweep's output directory, which exists only after a weekly run.
  ok(!runs("mutants:timeouts"), "mutants:timeouts needs a directory and cannot be part of the gate");
  ok("mutants:timeouts:test" in scripts, "its tests, which need nothing, are");
});
