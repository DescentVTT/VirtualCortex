---
status: accepted
date: 2026-09-14
depends-on: ADR-0052
decision-makers: VirtualCortex maintainers
---

# ADR-0056: A compaction of the term arena — the nodes the clause store does not reach are reclaimed by one descending pass that marks and one ascending pass that moves, a rule of `cortex-reasoning` over its own bottom-up arena; the executor runs it between ticks on a call and by itself at every entry into slow-wave sleep; the image unchanged

## Context and Problem Statement

[ADR-0052](0052-the-term-arena-in-the-image.md) put the term arena in the image and the discovery loop inside the tick, and named "a compaction of the arena's garbage" as Specified. Every commit of `search_from` puts the common clause and the first definition where the two inputs were and the second definition at the store's end; the outputs are instantiated copies, so the inputs' nodes, the generalisation's intermediate nodes and the matching's stay in the arena below `free`, reached by no clause of the store. `InduceScratch::restore` reclaims a failed attempt's nodes only. So the arena's capacity bounded the commits an engine could make over its life: an arena full mid-search is `ArenaFull`, an error the loop counts as a failure, and nothing in the tree took the garbage back. Whitepaper §8.8's glymphatic row ("Reclamation, compaction and checksum audit during the slow-wave stage of ADR-0037") and §8.6 placed reclamation in slow-wave sleep under `cortex-immune`, Specified; TC-2 forbids a dependency between state crates, so the arena's compaction is the arena's crate's, and the executor composes it as it composes every rule.

Brief 026 asked where the rule lives, what it costs, when it runs, and what a compaction must leave unchanged.

## Decision Drivers

- The bottom-up invariant: a child's index is below its parent's, kept by the constructors ([ADR-0025](0025-term-arena-and-unification.md): `compound` takes its children's indices), the operators ([ADR-0041](0041-induction-on-the-term-arena.md): `lgg` and `Body::alloc` allocate children before their parent), the instantiation and the loader ([ADR-0052](0052-the-term-arena-in-the-image.md): `admits` refuses a child at or beyond the cursor). One descending pass marks completely, with no stack, no recursion and no bound but the cursor.
- What a compaction must leave: the store reads the same node for node (its description length, every proof, every structural hash), so the affect state primed to the length stands and nothing is primed again; the search's cursor names store positions, not indices, and stands.
- The bindings are the scratch [ADR-0052](0052-the-term-arena-in-the-image.md) made them, empty between searches; a binding names an index, so the rule takes no table and a caller compacts an empty one.
- TC-5: nothing allocates in the loop; the rule's scratch is the executor's work stack, twice the arena's size and empty between walks.
- Determinism (§8.3): the compaction runs on the coordinator between the window's regulation and the next tick, touching no worker's state, so it is bit-identical on every worker count.
- §8.8's glymphatic row: reclamation during the slow-wave stage.

## Considered Options

1. **Two passes at slow-wave onset and on a call**: the mark descending, the move ascending, the roots remapped in place; run by the executor once per entry into slow-wave sleep and by a caller between ticks.
2. A compaction on a cadence while awake.
3. A compaction when the arena is full mid-search.
4. A free list without a move (mark and sweep): the indices stable, a free list in the record, every allocation site changed from the cursor to the list.
5. Reference counts per node.
6. The `cortex-immune` record composed as the compaction's ledger.

## Decision Outcome

Option 1.

- **The rule** (`cortex-reasoning::compact(arena, free, roots, forward) -> Result<Compaction, CompactError>`). The roots are the store's indices, remapped in place; `forward` a scratch of at least `free` entries. Three passes over `[0, free)`: the check (every root below the cursor, every child below its parent; refused as `Root` or `NotBottomUp` before anything is written, `Scratch` for a short scratch or a cursor beyond the arena), the mark (the roots first, then descending, a marked node marks its children, which lie below it and are visited after it), and the move (ascending, a live node takes the next free slot, which is at or below its own, so nothing live is overwritten before it is read; its children, moved before it, are remapped through the table). The dead tail is zeroed, so an arena at rest above the cursor stays at rest (§8.7); the live nodes keep their order, so the compacted arena is bottom-up too. `Compaction { live, reclaimed }` sums to the cursor before. A shared subterm is kept once; a root named twice is remapped twice; no root empties the arena; an arena without garbage moves nothing and reports nothing reclaimed. Nothing allocates; every loop is a range over the cursor.
- **The executor.** `Induction::compact` runs the rule over the engine's arena with the store's indices as roots and the work stack as the table, moves the record's `free`, clears the last search's discoveries (their indices moved) and steps two counters; the affect state stands, since the store's description length is the same by construction, and the cursor stands, since it names store positions. `Executor::compact()` runs it between ticks (`TermError::NoArena` for an engine without an arena; `Malformed` if the rule refuses, which the engine's own arena cannot make it do); `Executor::{compactions, reclaimed}` read the counters. Inside the tick, `tally` reads the stage before and after `step_sleep` and, when the stage moved from awake or REM into slow-wave sleep, compacts once: every entry into the stage, so a night of two slow-wave bouts compacts twice, the second reclaiming what the REM bout between left, which is nothing unless a search ran. The onset is the trigger, not a cadence and not a full arena: awake, a search may hold indices in flight; asleep, the loop does not run (ADR-0052), so the arena is quiet.
- **What the runs read.** The exit store of [ADR-0045](0045-clause-search.md) in the engine's arena, searched once between ticks (two commits, 106 nodes of description length becoming 100): the cursor stood at 113 nodes, of which 83 are reached by the store's twenty-six clauses and 30 are reclaimed (the four replaced inputs, the generalisation's and the matching's intermediates, and the copies instantiation made only where a binding required one); the store's twenty-six hashes are the same in the same order, the description length is 100 before and after, a second compaction reclaims nothing, and a search after it finds no pair and reads the length through the moved indices. A loaded image after a compaction holds the same arena, and the two engines' next searches commit the same invention. In the capture nights of [ADR-0048](0048-episodes-tagged-from-the-train.md) the onset reclaims the same thirty nodes at 256 units and thirty at 1 024, and every other reading of the night stood.
- **The image.** Nothing: format 14 (ADR-0052). A loaded arena is smaller after a night, which the loader reads as it reads any arena.
- **What stays Specified.** Standardising apart; the checksum audit; the `cortex-immune` record's composition (nothing reads it, and a compaction of one arena is not a segment's scrub state).

### Consequences

- Good: the arena's use over a life is bounded by what the store reaches, not by what the searches tried; an `ArenaFull` failure of the loop is recoverable at the next night; an image written after a night holds no garbage; the store reads the same before and after, by construction and by test.
- Neutral: an index into the arena is valid until the next slow-wave onset; the discoveries of the last search are cleared at a compaction for that reason, and a caller that keeps an index across a night keeps garbage. The cost is three passes over the cursor at a window boundary: at the reference tests' arena (1 024 nodes) nothing; at Appendix A's arena it is the cost of a sweep, once per night, and a Target like every figure at that scale.
- Bad: none found. The rule refuses an arena that is not bottom-up, which no path in the tree can make and the loader refuses.

## Alternatives considered and why rejected

- **Option 2**, a cadence while awake: a search in flight holds indices in the scratch and the discovery buffer; between the loop's searches the arena is quiet, but a cadence would compact a live arena for nothing most of the time, and the night is when the store is quiet by construction.
- **Option 3**, on a full arena mid-search: a search holds the mark it will restore to and the bindings the matching made; compacting under it is the one case the rule refuses to think about (the table must be empty). The loop counts the failure, and the next night reclaims.
- **Option 4**, a free list: it needs a list in the record (the reserved bytes of `InductionState` would take it) and a change to every allocation site from a cursor to a list, and a walk over a fragmented arena costs the same as over a compact one only until the list is long; the bottom-up invariant, which every measure and the loader rely on, is a property of a cursor, not of a list.
- **Option 5**, reference counts: a count per node in the reserved bytes of `TermNode`, kept by every operator and undone by every `restore`; the marking pass is cheaper than the bookkeeping and cannot be wrong by omission.
- **Option 6**, the immune record: `ImmuneScrubNode` is a segment's scrub state with a checksum, a health score and a degenerate-synapse count, none of which a term arena has; composing it would give the image a section that says nothing.

## Confirmation

- `crates/cortex-reasoning/src/compact.rs`: the rule, three unit tests (garbage below, between and above the live nodes; a shared subterm and a store of no roots; every refusal leaving everything as it was) and a property test over 2 000 seeded bottom-up arenas of up to sixty-four nodes with seeded roots (every root's size and hash unchanged, the kept and the reclaimed summing to the cursor, the result bottom-up, a second compaction reclaiming nothing).
- `runtime/cortex-runtime/tests/store.rs`: the compaction between ticks after the loop committed, the cursor across a compaction and a loaded engine searching alike, every entry into slow-wave sleep compacting once inside the tick; `tests/no_alloc.rs`: a night's onset allocating nothing; `tests/reference.rs`: the capture nights' reclaimed counts and the determinism test through a night on one and four workers.
- The mutation gate on the changed lines (ADR-0030).
