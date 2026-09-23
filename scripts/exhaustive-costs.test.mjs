// `scripts/exhaustive-costs.mjs`'s tests (node:test, no dependency): `npm run spec:costs:test`.
//
// The shard script's parsers have been wrong the first time twice (ADR-0073); these hold the
// plan's parser, its deal and the table's checks to what ADR-0092 says they do.

import { deepEqual, equal, ok, throws } from "node:assert/strict";
import { test } from "node:test";
import { DEFAULT_COST, fromArtifacts, parseCosts, parseList, plan, stale, stemOf } from "./exhaustive-costs.mjs";

const LIST = [
  "   Compiling cortex-runtime v0.1.0",
  "     Running unittests src/lib.rs (target/release/deps/cortex_core-026c8fc3cc426559)",
  "dynamics::plasticity::prop::exhaustive_the_efficacy_is_bounded_by_one_for_every_input: test",
  "",
  "1 test, 0 benchmarks",
  "     Running unittests src/lib.rs (target/release/deps/cortex_neuromod-9a83c9f77989b9bc)",
  "",
  "0 tests, 0 benchmarks",
  "     Running tests/inhibition.rs (target/release/deps/inhibition-1e70ed1bbc1ad67d)",
  "the_assignment_reversed_first_exhaustive: test",
  "the_mirrored_reversed_first_exhaustive: test",
  "",
  "2 tests, 0 benchmarks",
  "   Doc-tests cortex_core",
  "src/lib.rs - Thing (line 3): test",
  "",
  "1 test, 0 benchmarks",
].join("\n");

test("a binary's stem drops the directory, `.exe` and cargo's hash, on either separator", () => {
  equal(stemOf("target/release/deps/instrument-1e70ed1bbc1ad67d"), "instrument");
  equal(stemOf("target\\release\\deps\\instrument-1e70ed1bbc1ad67d.exe"), "instrument");
  equal(stemOf("target/release/deps/cortex_core-026c8fc3cc426559"), "cortex_core");
});

test("the listing's tests are read under their binary, and a doc-test section is dropped", () => {
  deepEqual(
    parseList(LIST).map((t) => `${t.stem} ${t.name}`),
    [
      "cortex_core dynamics::plasticity::prop::exhaustive_the_efficacy_is_bounded_by_one_for_every_input",
      "inhibition the_assignment_reversed_first_exhaustive",
      "inhibition the_mirrored_reversed_first_exhaustive",
    ],
  );
  equal(parseList(LIST)[1].path, "target/release/deps/inhibition-1e70ed1bbc1ad67d");
  deepEqual(parseList(LIST.replace(/\r?\n/g, "\r\n")), parseList(LIST), "a Windows listing reads the same");
  deepEqual(parseList("error: could not compile `cortex-runtime`"), [], "a failed build lists nothing");
});

test("the table reads its lines, skips comments and blanks, and refuses a malformed or repeated line", () => {
  const { costs, lines } = parseCosts("# a comment\n\ninstrument\ta_exhaustive\t120\nlearning\tb_exhaustive\t0\n");
  equal(costs.get("instrument\ta_exhaustive"), 120);
  equal(costs.get("learning\tb_exhaustive"), 0);
  equal(lines.get("instrument\ta_exhaustive"), 3);
  throws(() => parseCosts("instrument a_exhaustive 120\n"), /:1: expected/);
  throws(() => parseCosts("instrument\ta_exhaustive\t1.5\n"), /whole seconds/);
  throws(() => parseCosts("instrument\ta_exhaustive\t-1\n"), /:1:/);
  throws(() => parseCosts("x\ta\t1\nx\ta\t2\n"), /:2: x a is listed twice/);
});

const t = (stem, name) => ({ path: `deps/${stem}-0a`, stem, name });

test("the deal is longest first to the least-loaded shard, which balances what round robin stacks", () => {
  // Round robin by name puts both heavy tests in shard 0 of 2; the deal parts them.
  const tests = [t("a", "heavy1"), t("a", "light1"), t("b", "heavy2"), t("b", "light2")];
  const costs = new Map([["a\theavy1", 1500], ["b\theavy2", 1400], ["a\tlight1", 100], ["b\tlight2", 100]]);
  const { shards } = plan(tests, costs, 2);
  deepEqual(shards.map((s) => s.tests.map((x) => x.name)), [["heavy1", "light2"], ["light1", "heavy2"]]);
  deepEqual(shards.map((s) => s.load), [1600, 1500]);
});

test("every test is dealt exactly once, and the deal is the same whatever order the listing gives", () => {
  const tests = Array.from({ length: 23 }, (_, i) => t(i % 3 === 0 ? "x" : "y", `t${String(i).padStart(2, "0")}`));
  const costs = new Map(tests.map((x, i) => [`${x.stem}\t${x.name}`, (i * 37) % 11]));
  const a = plan(tests, costs, 4);
  const b = plan([...tests].reverse(), costs, 4);
  deepEqual(a, b);
  const dealt = a.shards.flatMap((s) => s.tests.map((x) => `${x.stem} ${x.name}`)).sort();
  deepEqual(dealt, tests.map((x) => `${x.stem} ${x.name}`).sort());
});

test("with an empty table every test costs the default, and the deal cycles through the shards", () => {
  const tests = [t("a", "p"), t("a", "q"), t("a", "r"), t("a", "s"), t("a", "u")];
  const { shards } = plan(tests, new Map(), 2);
  deepEqual(shards.map((s) => s.tests.map((x) => x.name)), [["p", "r", "u"], ["q", "s"]]);
  equal(shards[0].load, 3 * DEFAULT_COST);
});

test("a test the table does not know is costed as a heavy one, so it is not stacked on the heaviest shard", () => {
  const tests = [t("a", "old_heavy"), t("a", "old_light"), t("a", "new_test")];
  const costs = new Map([["a\told_heavy", 1200], ["a\told_light", 50]]);
  const { shards } = plan(tests, costs, 2);
  ok(!shards.some((s) => s.tests.some((x) => x.name === "old_heavy") && s.tests.some((x) => x.name === "new_test")));
});

test("more shards than tests leaves shards empty, and a shard count below one is refused", () => {
  const { shards } = plan([t("a", "only")], new Map(), 3);
  deepEqual(shards.map((s) => s.tests.length), [1, 0, 0]);
  throws(() => plan([t("a", "only")], new Map(), 0), /positive integer/);
});

test("a line of the table that names no test function in the tree is stale, and says where it is", () => {
  const sources = [
    "fn the_assignment_reversed_first_exhaustive() {}",
    "mod prop { fn exhaustive_the_efficacy_is_bounded_by_one_for_every_input() {} }",
  ];
  const table = [
    "# header",
    "inhibition\tthe_assignment_reversed_first_exhaustive\t860",
    "cortex_core\tdynamics::plasticity::prop::exhaustive_the_efficacy_is_bounded_by_one_for_every_input\t2",
    "instrument\ta_test_that_was_renamed_exhaustive\t400",
  ].join("\n");
  deepEqual(stale(table, sources), [
    "scripts/exhaustive-costs.tsv:4: instrument a_test_that_was_renamed_exhaustive names no test function in the tree",
  ]);
  deepEqual(stale("inhibition\tthe_assignment_reversed_first\t1\n", sources).length, 1, "a prefix of a name is not the name");
});

test("a table is written from a run's per-test files, the passing tests only, sorted and headed", () => {
  const shard0 = "p/inhibition-0a\tinhibition\tb_exhaustive\t860\t0\t1\np/learning-0a\tlearning\tfailed_exhaustive\t30\t101\t1\n";
  const shard1 = "p/instrument-0a\tinstrument\ta_exhaustive\t1500\t0\t1\np/x-0a\tx\tran_nothing\t0\t0\t0\n";
  const out = fromArtifacts([shard0, shard1], "12345");
  const rows = out.split("\n").filter((l) => l !== "" && !l.startsWith("#"));
  deepEqual(rows, ["inhibition\tb_exhaustive\t860", "instrument\ta_exhaustive\t1500"]);
  ok(out.includes("# Source: run 12345."));
  deepEqual(parseCosts(out).costs.size, 2, "the table it writes reads back");
});
