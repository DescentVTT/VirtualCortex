//! Brief 025's exit test for the engine's own term arena and clause store (ADR-0052;
//! whitepaper §5.2.30, §6.7, §6.10): the inputs between ticks build terms and assert clauses
//! into the executor's arena; the discovery loop between ticks (`Executor::discover`)
//! searches the store from its cursor, rewards the modulator and tags the coincidence before
//! the reward from the executor's own train, binding the episode to the first invented
//! predicate; the same loop runs inside the tick on its cadence while awake and counts its
//! refusals; a bounded budget resumes from the cursor; the image carries the arena, the
//! store, the induction record and the affect state, a loaded engine searches as the
//! un-loaded one does, and every clause of the loader's checks on the four sections refuses
//! on its own. Brief 026 adds the compaction (ADR-0056): between ticks it reclaims what the
//! commits left and the store reads the same node for node; the cursor stands; a loaded
//! engine searches alike; and inside the tick every entry into slow-wave sleep compacts
//! once.

#![deny(clippy::arithmetic_side_effects)]

use cortex_affect::InteroceptiveState;
use cortex_connectome::{
    CortexFileHeader, SECTION_AFFECT, SECTION_CLAUSE, SECTION_EPISODE, SECTION_HOMEOSTASIS,
    SECTION_INDUCTION, SECTION_TERM, SectionEntry, crc64,
};
use cortex_core::{
    MODULATION_ONE_Q16, STP_MAX, STP_U, THRESHOLD_BASE, spike_message, synaptic_efficacy_q16,
};
use cortex_hippocampus::Episode;
use cortex_homeostasis::{
    ACTIVITY_BIN_SHIFT, ACTIVITY_WINDOW_SHIFT, HomeostaticDrivePool, PRESSURE_MAX_Q16, STAGE_AWAKE,
    STAGE_REM, STAGE_SWS,
};
use cortex_reasoning::{
    Binding, Compaction, INVENTED_BASE, INVENTED_LIMIT, InductionState, MAX_BODY, TERM_COMPOUND,
    TERM_CONSTANT, TERM_VARIABLE, TermNode, is_clause, term_hash,
};
use cortex_runtime::{
    Config, DiscoverError, DiscoveryError, Executor, Image, ImageError, TagError, TermError,
};

type Engine = Executor<64>;

/// The vocabulary of the exit store, as `tests/discovery.rs` and `tests/reference.rs` have
/// it: two heads, eight literals, four constants, as concept ids.
const P: u32 = 0x100;
const R: u32 = 0x101;
const LIT: [u32; 8] = [0x200, 0x201, 0x202, 0x203, 0x204, 0x205, 0x206, 0x207];
const K: [u32; 4] = [0x300, 0x301, 0x302, 0x303];
const ONE: i32 = MODULATION_ONE_Q16;

/// Four armed units, a ledger of two, a train of 256, an arena and a store of the given
/// capacities; the loop's cadence, budget and tag as given.
fn engine(terms: usize, clauses: usize, shift: u8, budget: u32, tag: u8) -> Engine {
    let mut exec = Engine::new(Config {
        units: 4,
        nodes_per_worker: 256,
        injector_capacity: 64,
        episodes: 2,
        train_capacity: 256,
        terms,
        clauses,
        search_shift: shift,
        search_budget: budget,
        discovery_tag: tag,
        ..Config::default()
    })
    .unwrap();
    for unit in exec.units_mut() {
        unit.v_thresh = THRESHOLD_BASE;
        unit.stp_u_rel = STP_U;
        unit.stp_r_ves = STP_MAX;
    }
    exec
}

/// Fires units 0 and 1 by fourteen strong messages each and runs sixty-four ticks (the soma
/// takes a few to cross its threshold), so that the executor's train holds a coincidence
/// within the ripple before now.
fn fire(exec: &mut Engine) {
    let inject = exec.injector();
    let strong = spike_message(synaptic_efficacy_q16(i16::MAX, STP_U, STP_MAX), false);
    for unit in [0, 1] {
        for _ in 0..14 {
            inject.inject(unit, strong).unwrap();
        }
    }
    exec.run(64);
}

/// A fresh variable in the engine's arena, numbered after the record's.
fn var(exec: &mut Engine) -> u32 {
    let next = exec.induction().next_variable;
    exec.term(TermNode::variable(next)).unwrap()
}

/// `head(X) ← l1(X), ..., lk(X)` over a fresh variable, asserted.
fn rule(exec: &mut Engine, head: u32, literals: &[u32]) -> u32 {
    let x = var(exec);
    let h = exec.term(TermNode::compound(head, &[x]).unwrap()).unwrap();
    let mut body = [0u32; MAX_BODY];
    for (i, &l) in literals.iter().enumerate() {
        body[i] = exec.term(TermNode::compound(l, &[x]).unwrap()).unwrap();
    }
    exec.assert_clause(h, &body[..literals.len()]).unwrap()
}

/// The fact `l(k)`, asserted.
fn fact(exec: &mut Engine, literal: u32, k: u32) -> u32 {
    let c = exec.term(TermNode::constant(k)).unwrap();
    let h = exec
        .term(TermNode::compound(literal, &[c]).unwrap())
        .unwrap();
    exec.assert_clause(h, &[]).unwrap()
}

/// The store's clauses as structural hashes through no binding, in the store's order: what
/// a compaction must leave as it was.
fn hashes(exec: &Engine) -> Vec<u32> {
    let terms = exec.terms();
    let (capacity, _) = exec.term_capacity();
    let bindings = vec![Binding::UNBOUND; capacity];
    let mut stack = vec![0u32; capacity.saturating_mul(2)];
    exec.clauses()
        .iter()
        .map(|&c| term_hash(c, terms, &bindings, &mut stack).unwrap())
        .collect()
}

/// A window of the tick, $2^{17}$ ticks (ADR-0035).
const WINDOW: u64 = 1 << (ACTIVITY_BIN_SHIFT + ACTIVITY_WINDOW_SHIFT);

/// The store of ADR-0045's exit test: three rules of `p` sharing `LIT[0..4]` and differing
/// in `LIT[4]`, `LIT[5]`, `LIT[6]`; one rule of `r`; twenty facts. 106 nodes.
fn exit_store(exec: &mut Engine) -> Vec<u32> {
    let mut out = vec![
        rule(exec, P, &[LIT[0], LIT[1], LIT[2], LIT[3], LIT[4]]),
        rule(exec, P, &[LIT[0], LIT[1], LIT[2], LIT[3], LIT[5]]),
        rule(exec, P, &[LIT[0], LIT[1], LIT[2], LIT[3], LIT[6]]),
        rule(exec, R, &[LIT[0], LIT[7]]),
    ];
    for (i, &k) in K.iter().enumerate() {
        for &l in &LIT[..4] {
            out.push(fact(exec, l, k));
        }
        out.push(fact(exec, [LIT[4], LIT[5], LIT[6], LIT[7]][i], k));
    }
    out
}

/// `p(X, Z) ← r(X, Y), own(X)` over fresh variables: an invention over two of these is one
/// node longer than its inputs, so the search rejects every pair (`tests/discovery.rs`).
fn narrow(exec: &mut Engine, own: u32) -> u32 {
    let (x, y, z) = (var(exec), var(exec), var(exec));
    let head = exec.term(TermNode::compound(P, &[x, z]).unwrap()).unwrap();
    let body = [
        exec.term(TermNode::compound(R, &[x, y]).unwrap()).unwrap(),
        exec.term(TermNode::compound(own, &[x]).unwrap()).unwrap(),
    ];
    exec.assert_clause(head, &body).unwrap()
}

/// Edits section `kind` in place and re-seals its checksum.
fn patch_section(img: &mut [u8], kind: u32, patch: impl Fn(&mut [u8])) {
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == kind {
            let (offset, length) = (entry.offset as usize, entry.length as usize);
            patch(&mut img[offset..][..length]);
            entry.crc64 = crc64(&img[offset..][..length]);
            img[at..][..64].copy_from_slice(&entry.encode());
            return;
        }
    }
    panic!("no section {kind}");
}

/// Edits directory entry `kind` in place.
fn patch_entry(img: &mut [u8], kind: u32, patch: impl Fn(&mut SectionEntry)) {
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    for at in (64..).step_by(64).take(header.section_count as usize) {
        let mut entry = SectionEntry::decode(img[at..][..64].try_into().unwrap());
        if entry.kind == kind {
            patch(&mut entry);
            img[at..][..64].copy_from_slice(&entry.encode());
            return;
        }
    }
    panic!("no section {kind}");
}

/// The image of `exec` with one 64-byte record section patched through its decoded form.
fn with_record<T>(
    exec: &Engine,
    kind: u32,
    decode: impl Fn(&[u8; 64]) -> T,
    encode: impl Fn(&T) -> [u8; 64],
    patch: impl Fn(&mut T),
) -> Vec<u8> {
    let mut img = Image::encode(exec).unwrap();
    patch_section(&mut img, kind, |s| {
        let mut record = decode((&s[..64]).try_into().unwrap());
        patch(&mut record);
        s[..64].copy_from_slice(&encode(&record));
    });
    img
}

#[test]
fn the_inputs_refuse_what_the_arena_and_the_store_cannot_hold() {
    let mut none = engine(0, 0, 0, 0, 0);
    assert_eq!(none.term(TermNode::constant(1)), Err(TermError::NoArena));
    assert_eq!(none.assert_clause(0, &[]), Err(TermError::NoArena));
    assert_eq!(none.term_capacity(), (0, 0));
    let report = none.discover().unwrap();
    assert_eq!(
        (report.search.attempts, report.search.commits, report.tagged),
        (0, 0, None),
        "a search over nothing"
    );
    let mut exec = engine(8, 1, 0, 8, 3);
    let a = exec.term(TermNode::constant(K[0])).unwrap();
    assert_eq!(
        exec.term(TermNode::compound(P, &[1]).unwrap()),
        Err(TermError::Malformed),
        "a child at the cursor"
    );
    assert_eq!(
        exec.term(TermNode::variable(8)),
        Err(TermError::Malformed),
        "a variable outside a table of eight"
    );
    assert_eq!(exec.term(TermNode::default()), Err(TermError::Malformed));
    let x = exec.term(TermNode::variable(3)).unwrap();
    assert_eq!(exec.induction().next_variable, 4);
    let pa = exec.term(TermNode::compound(P, &[a]).unwrap()).unwrap();
    let px = exec.term(TermNode::compound(P, &[x]).unwrap()).unwrap();
    assert_eq!(
        exec.assert_clause(px, &[pa; MAX_BODY + 1]),
        Err(TermError::Malformed)
    );
    let c = exec.assert_clause(px, &[pa]).unwrap();
    assert_eq!(exec.clauses(), &[c]);
    assert_eq!(
        exec.affect().free_energy_prev_q16,
        5 << 16,
        "primed to five nodes"
    );
    assert_eq!(exec.assert_clause(pa, &[]), Err(TermError::StoreFull));
    assert_eq!(exec.terms().len(), 5);
    for _ in 0..3 {
        exec.term(TermNode::constant(9)).unwrap();
    }
    assert_eq!(exec.term(TermNode::constant(9)), Err(TermError::ArenaFull));
    assert_eq!(exec.bind_episode(0, 1), Err(TagError::NoSuchEpisode));
}

#[test]
fn the_loop_between_ticks_searches_the_store_rewards_the_modulator_tags_and_binds() {
    let mut exec = engine(1024, 32, 0, 64, 5);
    let clauses = exit_store(&mut exec);
    assert_eq!(clauses.len(), 24);
    assert_eq!(exec.clauses(), &clauses[..]);
    assert_eq!(exec.affect().free_energy_prev_q16, 106 << 16);
    fire(&mut exec);
    assert!(exec.train().len() >= 2, "both units fired");
    let report = exec.discover().unwrap();
    assert_eq!(
        (
            report.search.attempts,
            report.search.commits,
            report.search.length_before,
            report.search.length_after,
            report.search.reward_total_q16
        ),
        (2, 2, 106, 100, 3 * ONE / 2)
    );
    assert_eq!(report.signal_q16, 3 * ONE / 2, "the reward is the signal");
    assert_eq!(exec.modulator().dopamine_rpe, 3 * ONE / 2);
    let (episode, burst) = report.tagged.expect("a rewarded search tags");
    assert_eq!(episode, 0);
    assert!(burst.spikes >= 2);
    assert_eq!(exec.episodes().len(), 1);
    assert_eq!(exec.episodes()[0].symbol(), Some(INVENTED_BASE));
    assert_eq!(exec.episodes()[0].tag, 5);
    assert_eq!(
        exec.discoveries()
            .iter()
            .map(|d| d.invention.predicate)
            .collect::<Vec<_>>(),
        vec![INVENTED_BASE, INVENTED_BASE + 1]
    );
    assert_eq!(exec.clauses().len(), 26);
    assert_eq!(
        (
            exec.induction().clauses,
            exec.induction().next_invented,
            exec.induction().resume()
        ),
        (26, INVENTED_BASE + 2, None)
    );
    assert_eq!(exec.affect().free_energy_prev_q16, 100 << 16);
    assert_eq!((exec.searches(), exec.inventions()), (1, 2));
    // A second search: no pair left, nothing committed, nothing tagged, the signal as it
    // stood, the episode bound as it was.
    let again = exec.discover().unwrap();
    assert_eq!(
        (again.search.attempts, again.search.commits, again.tagged),
        (0, 0, None)
    );
    assert_eq!(again.signal_q16, 3 * ONE / 2);
    assert_eq!(exec.discoveries(), &[]);
    assert_eq!((exec.searches(), exec.inventions()), (2, 2));
    assert_eq!(exec.episodes().len(), 1);
    assert_eq!(
        exec.bind_episode(0, INVENTED_BASE + 1),
        Err(TagError::Bound),
        "an episode stands for one symbol"
    );
    assert_eq!(
        exec.bind_episode(1, INVENTED_BASE),
        Err(TagError::NoSuchEpisode)
    );
    // A rewarded search whose moment cannot be tagged: the reward stands, the search is
    // counted as untagged, and the caller sees the ledger's refusal.
    let mut full = engine(1024, 32, 0, 64, 5);
    exit_store(&mut full);
    fire(&mut full);
    assert_eq!(full.tag_episode(&[0], 1), Ok(0));
    assert_eq!(full.tag_episode(&[1], 1), Ok(1));
    assert_eq!(
        full.discover(),
        Err(DiscoverError::Tag(TagError::LedgerFull))
    );
    assert_eq!(
        full.modulator().dopamine_rpe,
        3 * ONE / 2,
        "the reward stood"
    );
    assert_eq!((full.inventions(), full.untagged()), (2, 1));
    assert_eq!(full.clauses().len(), 26, "the commits stood");
    // A tag of zero tags nothing either: the pattern is refused by the ledger's rule.
    let mut untagged = engine(1024, 32, 0, 64, 0);
    exit_store(&mut untagged);
    fire(&mut untagged);
    assert_eq!(
        untagged.discover(),
        Err(DiscoverError::Tag(TagError::InvalidPattern))
    );
    assert_eq!(untagged.untagged(), 1);
    // A store with no slot for a commit: the search ends in its own error, nothing is
    // committed, nothing rewarded, the failure counted.
    let mut tight = engine(1024, 24, 0, 64, 5);
    exit_store(&mut tight);
    fire(&mut tight);
    assert_eq!(
        tight.discover(),
        Err(DiscoverError::Discovery(DiscoveryError::StoreFull))
    );
    assert_eq!(tight.modulator().dopamine_rpe, 0);
    assert_eq!(
        (
            tight.inventions(),
            tight.search_failures(),
            tight.searches()
        ),
        (0, 1, 1)
    );
    assert_eq!(tight.clauses().len(), 24);
    assert_eq!(tight.discoveries(), &[]);
    // A search that is not rewarded tags nothing and leaves the signal as it stood.
    let mut quiet = engine(256, 8, 0, 64, 5);
    narrow(&mut quiet, LIT[4]);
    narrow(&mut quiet, LIT[5]);
    fire(&mut quiet);
    let report = quiet.discover().unwrap();
    assert_eq!(
        (
            report.search.attempts,
            report.search.rejections,
            report.search.commits,
            report.tagged,
            report.signal_q16
        ),
        (1, 1, 0, None, 0)
    );
    assert!(quiet.episodes().is_empty());
}

#[test]
fn a_bounded_budget_resumes_after_the_last_pair_attempted_and_a_commit_restarts_the_walk() {
    // Three clauses whose every pair is rejected: with a budget of one, each search attempts
    // the next pair after the cursor; the third attempts the last pair, after which no pair
    // is left, so its cursor is the start and the fourth starts over.
    let mut exec = engine(256, 8, 0, 1, 5);
    for own in [LIT[4], LIT[5], LIT[6]] {
        narrow(&mut exec, own);
    }
    let mut cursors = Vec::new();
    for _ in 0..5 {
        let report = exec.discover().unwrap();
        cursors.push((report.search.attempts, exec.induction().resume()));
    }
    assert_eq!(
        cursors,
        vec![
            (1, Some((0, 1))),
            (1, Some((0, 2))),
            (1, None),
            (1, Some((0, 1))),
            (1, Some((0, 2))),
        ]
    );
    // Without a budget's end the pass is one search: three attempts, the cursor at the
    // start.
    let mut whole = engine(256, 8, 0, 64, 5);
    for own in [LIT[4], LIT[5], LIT[6]] {
        narrow(&mut whole, own);
    }
    let report = whole.discover().unwrap();
    assert_eq!(
        (
            report.search.attempts,
            report.search.rejections,
            whole.induction().resume()
        ),
        (3, 3, None)
    );
    assert_eq!(exec.inventions(), 0);
    // A budget of one over the exit store: the first search commits its first pair and
    // restarts, so the cursor is at the start; the second commits the next; the third
    // finds no pair.
    let mut one = engine(1024, 32, 0, 1, 5);
    exit_store(&mut one);
    fire(&mut one);
    let first = one.discover().unwrap();
    assert_eq!((first.search.commits, one.induction().resume()), (1, None));
    assert_eq!(one.episodes()[0].symbol(), Some(INVENTED_BASE));
    let second = one.discover().unwrap();
    assert_eq!((second.search.commits, one.induction().resume()), (1, None));
    assert_eq!(
        second.tagged,
        Some((1, second.tagged.unwrap().1)),
        "a second rewarded search tags a second episode"
    );
    assert_eq!(one.episodes()[1].symbol(), Some(INVENTED_BASE + 1));
    let third = one.discover().unwrap();
    assert_eq!((third.search.attempts, third.search.commits), (0, 0));
    assert_eq!(one.clauses().len(), 26);
    assert_eq!(one.affect().free_energy_prev_q16, 100 << 16);
}

#[test]
fn the_loop_inside_the_tick_runs_on_its_cadence_while_awake_and_never_asleep() {
    let mut exec = engine(1024, 32, 7, 64, 5);
    exit_store(&mut exec);
    fire(&mut exec);
    assert_eq!(exec.ticks(), 64);
    assert_eq!(exec.searches(), 0, "the cadence is every 128 ticks");
    exec.run(63);
    assert_eq!(exec.searches(), 0);
    exec.run(1);
    assert_eq!(
        (exec.searches(), exec.inventions(), exec.episodes().len()),
        (1, 2, 1),
        "the 128th tick"
    );
    assert_eq!(exec.episodes()[0].symbol(), Some(INVENTED_BASE));
    assert_eq!(exec.episodes()[0].tagged_tick, 128);
    assert!(
        exec.modulator().dopamine_rpe > 0,
        "rewarded inside the tick"
    );
    exec.run(256);
    assert_eq!(
        (exec.searches(), exec.inventions(), exec.episodes().len()),
        (3, 2, 1),
        "two more searches, no pair left"
    );
    assert_eq!((exec.untagged(), exec.search_failures()), (0, 0));
    // Asleep, the cadence runs nothing: the image's stage outranks everything, so the same
    // engine is put to slow-wave sleep through its image (with room to commit) and run for
    // the same ticks.
    let mut asleep = engine(1024, 32, 7, 64, 5);
    exit_store(&mut asleep);
    fire(&mut asleep);
    let img = with_record(
        &asleep,
        SECTION_HOMEOSTASIS,
        HomeostaticDrivePool::decode,
        HomeostaticDrivePool::encode,
        |h| {
            h.sleep_stage = STAGE_SWS;
            h.sleep_pressure_q16 = PRESSURE_MAX_Q16;
        },
    );
    let room = Config {
        terms: 256,
        clauses: 8,
        train_capacity: 256,
        episodes: 2,
        ..Config::default()
    };
    let mut asleep = Image::decode::<64>(&img, room).unwrap();
    assert_eq!(asleep.sleep_stage(), STAGE_SWS);
    asleep.run(384);
    assert_eq!((asleep.searches(), asleep.inventions()), (0, 0));
    assert!(asleep.wake());
    fire(&mut asleep);
    asleep.run(64);
    assert_eq!(
        (asleep.searches(), asleep.inventions(), asleep.untagged()),
        (1, 2, 0),
        "awake, it searches and tags"
    );
    // No cadence: nothing runs inside the tick however long it runs.
    let mut never = engine(1024, 32, 0, 64, 5);
    exit_store(&mut never);
    never.run(512);
    assert_eq!(never.searches(), 0);
}

#[test]
fn a_compaction_between_ticks_reclaims_what_the_commits_left_and_the_store_reads_the_same() {
    // An engine without an arena has nothing to compact.
    let mut none = engine(0, 0, 0, 0, 0);
    assert_eq!(none.compact(), Err(TermError::NoArena));
    assert_eq!((none.compactions(), none.reclaimed()), (0, 0));
    // The exit store, searched: two commits leave their four inputs and the operators'
    // intermediate nodes below the cursor, reached by no clause.
    let mut exec = engine(1024, 32, 0, 64, 5);
    exit_store(&mut exec);
    // A host builds no garbage: eight nodes per rule of five literals (the clause, the head,
    // the shared variable and the five literals), five for the rule of two, three per fact;
    // the description length of 106 counts the shared variable at every reach.
    assert_eq!(exec.terms().len(), 89);
    fire(&mut exec);
    assert_eq!(exec.discover().unwrap().search.commits, 2);
    let before = exec.terms().len();
    let hashes_before = hashes(&exec);
    let length = exec.affect().free_energy_prev_q16;
    assert_eq!((exec.discoveries().len(), length), (2, 100 << 16));
    let compaction = exec.compact().unwrap();
    // Two commits appended their instantiated outputs, twelve nodes each, to the host's 89:
    // the store's twenty-six clauses reach 83 of the 113, and the four replaced inputs with
    // the operators' intermediates are the thirty reclaimed (pinned from the run).
    assert_eq!(before, 113, "the cursor before");
    assert_eq!(
        compaction,
        Compaction {
            live: 83,
            reclaimed: 30
        }
    );
    assert_eq!(exec.terms().len(), compaction.live as usize);
    assert_eq!(exec.induction().free, compaction.live);
    assert_eq!(
        hashes(&exec),
        hashes_before,
        "every clause reads the same, in the store's order"
    );
    assert!(
        exec.clauses()
            .iter()
            .all(|&c| is_clause(&exec.terms()[c as usize])),
        "every store index names a clause"
    );
    assert_eq!(
        exec.affect().free_energy_prev_q16,
        length,
        "the length stands, so the affect state is not primed again"
    );
    assert_eq!(exec.discoveries(), &[], "the last search's indices moved");
    assert_eq!(
        (exec.compactions(), exec.reclaimed()),
        (1, u64::from(compaction.reclaimed))
    );
    // Nothing left: a second compaction reclaims nothing; a search finds no pair and reads
    // the store's length through the moved indices.
    assert_eq!(
        exec.compact().unwrap(),
        Compaction {
            live: compaction.live,
            reclaimed: 0
        }
    );
    let again = exec.discover().unwrap();
    assert_eq!(
        (
            again.search.attempts,
            again.search.commits,
            again.search.length_before
        ),
        (0, 0, 100)
    );
    assert_eq!(
        (exec.compactions(), exec.reclaimed()),
        (2, u64::from(compaction.reclaimed))
    );
}

#[test]
fn a_compaction_leaves_the_cursor_and_a_loaded_engine_searches_alike_after_one() {
    // Three narrow clauses under a budget of one: a host's store holds no garbage, and the
    // cursor names store positions, so a compaction between two searches changes neither.
    let mut exec = engine(256, 8, 0, 1, 5);
    for own in [LIT[4], LIT[5], LIT[6]] {
        narrow(&mut exec, own);
    }
    exec.discover().unwrap();
    assert_eq!(exec.induction().resume(), Some((0, 1)));
    assert_eq!(exec.compact().unwrap().reclaimed, 0);
    assert_eq!(exec.induction().resume(), Some((0, 1)));
    exec.discover().unwrap();
    assert_eq!(exec.induction().resume(), Some((0, 2)));
    // The exit store under a budget of one, with no spike on the train, so that every
    // rewarded search commits and then reports the tag it could not make (a loaded unit is
    // as adapted as the image left it, so firing both engines alike is not open): the first
    // search commits one invention and leaves its garbage; compacted and written, the image
    // loads to the same arena, and the two engines' next searches commit the same second
    // invention.
    let mut one = engine(1024, 32, 0, 1, 5);
    exit_store(&mut one);
    assert!(matches!(one.discover(), Err(DiscoverError::Tag(_))));
    assert_eq!((one.inventions(), one.clauses().len()), (1, 25));
    let compaction = one.compact().unwrap();
    assert!(compaction.reclaimed > 0, "{compaction:?}");
    let img = Image::encode(&one).unwrap();
    let room = Config {
        terms: 1024,
        clauses: 32,
        train_capacity: 256,
        episodes: 2,
        ..Config::default()
    };
    let mut loaded = Image::decode::<64>(&img, room).unwrap();
    assert_eq!(loaded.terms(), one.terms());
    assert_eq!(loaded.clauses(), one.clauses());
    assert_eq!(loaded.induction(), one.induction());
    assert_eq!(loaded.affect(), one.affect());
    assert!(matches!(one.discover(), Err(DiscoverError::Tag(_))));
    assert!(matches!(loaded.discover(), Err(DiscoverError::Tag(_))));
    // The counters are the executor's, not the image's: the loaded engine counts its own.
    assert_eq!((one.inventions(), loaded.inventions()), (2, 1));
    assert_eq!((one.clauses().len(), loaded.clauses().len()), (26, 26));
    assert_eq!(loaded.terms(), one.terms());
    assert_eq!(loaded.clauses(), one.clauses());
    assert_eq!(hashes(&loaded), hashes(&one));
}

#[test]
fn every_entry_into_slow_wave_sleep_compacts_the_arena_once_inside_the_tick() {
    // The exit store searched between ticks, so the arena holds garbage; then the engine
    // is put at the edge of sleep through its image (the pressure at its maximum, awake,
    // the shift 5), so that the first window's step is the onset.
    let mut exec = engine(1024, 32, 0, 64, 5);
    exit_store(&mut exec);
    fire(&mut exec);
    exec.discover().unwrap();
    let garbage = exec.terms().len();
    let hashes_before = hashes(&exec);
    let img = with_record(
        &exec,
        SECTION_HOMEOSTASIS,
        HomeostaticDrivePool::decode,
        HomeostaticDrivePool::encode,
        |h| {
            h.sleep_shift = 5;
            h.sleep_pressure_q16 = PRESSURE_MAX_Q16;
        },
    );
    let room = Config {
        terms: 1024,
        clauses: 32,
        train_capacity: 256,
        episodes: 2,
        ..Config::default()
    };
    let mut exec = Image::decode::<64>(&img, room).unwrap();
    assert_eq!(exec.sleep_stage(), STAGE_AWAKE);
    assert_eq!((exec.compactions(), exec.reclaimed()), (0, 0));
    assert_eq!(exec.terms().len(), garbage);
    // To the first window boundary: the onset, and one compaction.
    let to_boundary = WINDOW.wrapping_sub(exec.ticks().wrapping_rem(WINDOW));
    exec.run(to_boundary);
    assert_eq!(exec.sleep_stage(), STAGE_SWS);
    assert_eq!(exec.compactions(), 1);
    let reclaimed = exec.reclaimed();
    assert!(reclaimed > 0);
    assert_eq!(exec.terms().len(), garbage.wrapping_sub(reclaimed as usize));
    assert_eq!(hashes(&exec), hashes_before);
    // Four windows of slow-wave sleep, two of REM: no second compaction until the stage
    // comes back, which is a second entry and reclaims nothing.
    exec.run(WINDOW.wrapping_mul(4));
    assert_eq!(exec.sleep_stage(), STAGE_REM);
    assert_eq!(exec.compactions(), 1);
    exec.run(WINDOW.wrapping_mul(2));
    assert_eq!(exec.sleep_stage(), STAGE_SWS);
    assert_eq!((exec.compactions(), exec.reclaimed()), (2, reclaimed));
    // An engine without an arena passes the onset with nothing to reclaim.
    let mut none = engine(0, 0, 0, 0, 0);
    let img = with_record(
        &none,
        SECTION_HOMEOSTASIS,
        HomeostaticDrivePool::decode,
        HomeostaticDrivePool::encode,
        |h| {
            h.sleep_shift = 5;
            h.sleep_pressure_q16 = PRESSURE_MAX_Q16;
        },
    );
    none = Image::decode::<64>(&img, Config::default()).unwrap();
    none.run(WINDOW);
    assert_eq!(none.sleep_stage(), STAGE_SWS);
    assert_eq!((none.compactions(), none.reclaimed()), (0, 0));
}

#[test]
fn the_image_carries_the_arena_the_store_and_the_records_and_a_loaded_engine_searches_alike() {
    // An engine without an arena: the two records are written, no arena or store section.
    let none = engine(0, 0, 0, 0, 0);
    let img = Image::encode(&none).unwrap();
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    assert_eq!(
        header.section_count, 7,
        "neuron, synapse, modulator, homeostasis, hippocampus, affect, induction"
    );
    let loaded = Image::decode::<64>(&img, Config::default()).unwrap();
    assert_eq!(loaded.term_capacity(), (0, 0));
    assert_eq!(*loaded.induction(), InductionState::new());
    // A searched engine round-trips byte for byte, and the loaded one holds the same arena,
    // store, records and ledger.
    let mut exec = engine(1024, 32, 0, 64, 5);
    exit_store(&mut exec);
    fire(&mut exec);
    assert!(exec.discover().unwrap().tagged.is_some());
    exec.run(4);
    assert!(exec.is_quiescent());
    let img = Image::encode(&exec).unwrap();
    let header = CortexFileHeader::decode(img[0..64].try_into().unwrap());
    assert_eq!(
        header.section_count, 10,
        "with the ledger, the arena and the store"
    );
    let loaded = Image::decode::<64>(&img, Config::default()).unwrap();
    assert_eq!(loaded.terms(), exec.terms());
    assert_eq!(loaded.clauses(), exec.clauses());
    assert_eq!(loaded.induction(), exec.induction());
    assert_eq!(loaded.affect(), exec.affect());
    assert_eq!(loaded.episodes(), exec.episodes());
    assert_eq!(loaded.episodes()[0].symbol(), Some(INVENTED_BASE));
    assert_eq!(
        loaded.term_capacity(),
        (exec.terms().len(), exec.clauses().len()),
        "sized as the image's arena and store plus the configuration's room"
    );
    assert_eq!(Image::encode(&loaded).unwrap(), img, "byte for byte");
    let roomy = Image::decode::<64>(
        &img,
        Config {
            terms: 10,
            clauses: 3,
            search_shift: 9,
            search_budget: 1,
            discovery_tag: 1,
            ..Config::default()
        },
    )
    .unwrap();
    assert_eq!(
        roomy.term_capacity(),
        (exec.terms().len() + 10, exec.clauses().len() + 3)
    );
    assert_eq!(
        (
            roomy.induction().search_shift,
            roomy.induction().search_budget,
            roomy.induction().tag
        ),
        (0, 64, 5),
        "the image's cadence, budget and tag outrank the configuration's"
    );
    // An engine with the store asserted and not yet searched: the loaded one's search
    // commits what the un-loaded one's does, to the same store and the same episode.
    let mut fresh = engine(1024, 32, 0, 64, 5);
    exit_store(&mut fresh);
    let img = Image::encode(&fresh).unwrap();
    let room = Config {
        terms: 256,
        clauses: 8,
        train_capacity: 256,
        episodes: 2,
        ..Config::default()
    };
    let mut loaded = Image::decode::<64>(&img, room).unwrap();
    // The image carries no train (the ring is the executor's, ADR-0050): both engines are
    // fired on the same clock, so that both have the same coincidence to tag.
    fire(&mut fresh);
    fire(&mut loaded);
    assert_eq!(fresh.ticks(), loaded.ticks());
    let (a, b) = (fresh.discover().unwrap(), loaded.discover().unwrap());
    assert_eq!(a, b);
    assert_eq!(a.search.commits, 2);
    assert!(a.tagged.is_some());
    assert_eq!(fresh.terms(), loaded.terms());
    assert_eq!(fresh.clauses(), loaded.clauses());
    assert_eq!(fresh.episodes(), loaded.episodes());
    assert_eq!(fresh.induction(), loaded.induction());
}

#[test]
fn every_clause_of_the_loader_s_checks_on_the_arena_the_store_and_the_records_refuses_on_its_own() {
    let mut exec = engine(64, 4, 2, 8, 5);
    let a = exec.term(TermNode::constant(K[0])).unwrap();
    let x = exec.term(TermNode::variable(0)).unwrap();
    let pa = exec.term(TermNode::compound(P, &[a]).unwrap()).unwrap();
    let px = exec.term(TermNode::compound(P, &[x]).unwrap()).unwrap();
    let c0 = exec.assert_clause(pa, &[]).unwrap();
    let c1 = exec.assert_clause(px, &[pa]).unwrap();
    assert_eq!((c0, c1), (4, 5));
    fire(&mut exec);
    assert_eq!(exec.tag_episode(&[0, 1], 3), Ok(0));
    assert!(exec.bind_episode(0, INVENTED_BASE).is_ok());
    exec.run(4);
    assert!(exec.is_quiescent());
    let base = Image::encode(&exec).unwrap();
    let under = |img: &[u8]| Image::decode::<64>(img, Config::default()).err();
    assert!(under(&base).is_none(), "the untouched image opens");
    // The two records are required, one each, 64 bytes each; the store's records are 4.
    let mut img = base.clone();
    patch_entry(&mut img, SECTION_AFFECT, |e| e.kind = SECTION_HOMEOSTASIS);
    assert!(matches!(
        under(&img),
        Some(ImageError::MissingSection(SECTION_AFFECT))
    ));
    let mut img = base.clone();
    patch_entry(&mut img, SECTION_INDUCTION, |e| {
        e.kind = SECTION_HOMEOSTASIS
    });
    assert!(matches!(
        under(&img),
        Some(ImageError::MissingSection(SECTION_INDUCTION))
    ));
    let mut img = base.clone();
    patch_entry(&mut img, SECTION_AFFECT, |e| e.record_size = 16);
    assert!(matches!(
        under(&img),
        Some(ImageError::Directory(SECTION_AFFECT))
    ));
    let mut img = base.clone();
    patch_entry(&mut img, SECTION_INDUCTION, |e| {
        e.length = 0;
        e.crc64 = 0;
    });
    assert!(matches!(
        under(&img),
        Some(ImageError::Directory(SECTION_INDUCTION))
    ));
    let mut img = base.clone();
    patch_entry(&mut img, SECTION_CLAUSE, |e| e.record_size = 64);
    assert!(matches!(
        under(&img),
        Some(ImageError::Directory(SECTION_CLAUSE))
    ));
    let mut img = base.clone();
    patch_entry(&mut img, SECTION_TERM, |e| e.record_size = 4);
    assert!(matches!(
        under(&img),
        Some(ImageError::Directory(SECTION_TERM))
    ));
    // A term node: each refusal on its own, at its index.
    let with_term = |index: usize, patch: fn(&mut TermNode)| {
        let mut img = base.clone();
        patch_section(&mut img, SECTION_TERM, |s| {
            let at = index * 64;
            let mut node = TermNode::decode((&s[at..at + 64]).try_into().unwrap());
            patch(&mut node);
            s[at..at + 64].copy_from_slice(&node.encode());
        });
        img
    };
    for (index, patch) in [
        (0usize, (|n| n.kind = 4) as fn(&mut TermNode)),
        (0, |n| *n = TermNode::default()),
        (2, |n| n.children[0] = 3),
        (2, |n| n.children[0] = 4),
        (1, |n| n.functor = 64),
        (3, |n| n._pad = 1),
        (5, |n| n._reserved[23] = 1),
        (5, |n| n.arity = 9),
    ] {
        let err = under(&with_term(index, patch));
        assert!(
            matches!(err, Some(ImageError::MalformedTerm(i)) if i as usize == index),
            "term {index}: {err:?}"
        );
    }
    assert_eq!(
        (TERM_CONSTANT, TERM_VARIABLE, TERM_COMPOUND),
        (1, 2, 3),
        "the kinds the patches assume"
    );
    // A clause index: at the cursor, not a clause, twice.
    let with_clause = |index: usize, value: u32| {
        let mut img = base.clone();
        patch_section(&mut img, SECTION_CLAUSE, |s| {
            let at = index * 4;
            s[at..at + 4].copy_from_slice(&value.to_le_bytes());
        });
        img
    };
    for (index, value) in [(0usize, 6u32), (0, 2), (1, 4)] {
        let err = under(&with_clause(index, value));
        assert!(
            matches!(err, Some(ImageError::MalformedClause(i)) if i as usize == index),
            "clause {index}: {err:?}"
        );
    }
    // The induction record: not well formed, or not what was loaded.
    let with_induction = |patch: fn(&mut InductionState)| {
        with_record(
            &exec,
            SECTION_INDUCTION,
            InductionState::decode,
            InductionState::encode,
            patch,
        )
    };
    for patch in [
        (|r| r.free = 5) as fn(&mut InductionState),
        |r| r.free = 7,
        |r| r.clauses = 1,
        |r| r.clauses = 3,
        |r| r.next_variable = 0,
        |r| r.next_invented = INVENTED_LIMIT + 1,
        |r| r.next_invented = INVENTED_BASE - 1,
        |r| r._pad = 1,
        |r| r._reserved[0] = 1,
        |r| r.resume_i = 1,
        |r| r.search_shift = 64,
    ] {
        assert!(matches!(
            under(&with_induction(patch)),
            Some(ImageError::MalformedInduction)
        ));
    }
    let bigger = Image::decode::<64>(
        &with_induction(|r| {
            r.next_variable = 9;
            r.search_budget = 2;
            r.search_shift = 7;
            r.tag = 1;
        }),
        Config::default(),
    )
    .unwrap();
    assert_eq!(
        (
            bigger.induction().next_variable,
            bigger.induction().search_budget,
            bigger.induction().search_shift,
            bigger.induction().tag
        ),
        (9, 2, 7, 1),
        "the image's record is the engine's"
    );
    // The affect record: not well formed, or not primed to the store's length.
    let with_affect = |patch: fn(&mut InteroceptiveState)| {
        with_record(
            &exec,
            SECTION_AFFECT,
            InteroceptiveState::decode,
            InteroceptiveState::encode,
            patch,
        )
    };
    for patch in [
        (|a| a._reserved[19] = 1) as fn(&mut InteroceptiveState),
        |a| a.somatic_comfort_q16 = 0x0001_0001,
        |a| a.free_energy_prev_q16 = 0,
        |a| a.free_energy_prev_q16 += 1 << 16,
    ] {
        assert!(matches!(
            under(&with_affect(patch)),
            Some(ImageError::MalformedAffect)
        ));
    }
    let moody = Image::decode::<64>(
        &with_affect(|a| a.mood_baseline_q16 = -0x8000),
        Config::default(),
    )
    .unwrap();
    assert_eq!(moody.affect().mood_baseline_q16, -0x8000);
    // An episode's symbol outside the invented band.
    let with_symbol = |symbol: u32| {
        with_record(
            &exec,
            SECTION_EPISODE,
            Episode::decode,
            Episode::encode,
            |e| e.symbol = symbol,
        )
    };
    assert!(matches!(
        under(&with_symbol(1)),
        Some(ImageError::MalformedEpisode(0))
    ));
    assert!(matches!(
        under(&with_symbol(INVENTED_LIMIT)),
        Some(ImageError::MalformedEpisode(0))
    ));
    assert!(under(&with_symbol(INVENTED_LIMIT - 1)).is_none());
    assert!(under(&with_symbol(0)).is_none(), "unbound");
}
