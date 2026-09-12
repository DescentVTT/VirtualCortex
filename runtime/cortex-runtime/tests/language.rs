//! The exit test of brief 020 (whitepaper §6.9; ADR-0039, ADR-0040): a sentence, as concept ids
//! and lexical categories from a table that exists in this file alone, is reduced into a frame,
//! the frame is sealed as a hypervector and read back through the codebook, role by role, with
//! every distance pinned so that the AArch64 job holds the arithmetic equal. No word reaches a
//! record: the table maps words to ids on the way in and ids to words on the way out.

#![deny(clippy::arithmetic_side_effects)]

use cortex_linguistic::{
    LinguisticFrameSlot, Q16_ONE, ROLE_ACTION, ROLE_AFFECT, ROLE_OBJECT, ROLE_SUBJECT,
    SPEECH_ACT_ASSERTIVE, TEMPLATE_CAUSATIVE,
};
use cortex_reasoning::{
    Binding, ParseError, ParseScratch, RULE_BACKWARD_APPLICATION, RULE_FORWARD_APPLICATION,
    RULE_FORWARD_COMPOSITION, Reduction, TermNode, backward, forward,
};
use cortex_runtime::{
    LanguageError, comprehend, concept_in, decode_frame, encode_frame, read_role, role_concept,
};
use cortex_symbolic::{BODY_BITS, HypervectorBody, confidence_q16};

// Atomic category functors, above every concept and below the role band.
const S: u32 = 0x1000;
const NP: u32 = 0x1001;
const N: u32 = 0x1002;

/// The lexicon: a word, its concept (its index in the codebook) and its part of speech. This
/// table is the boundary of §1.5: the only place a word exists.
const WORDS: [(&str, u32, Part); 7] = [
    ("the", 0, Part::Determiner),
    ("dog", 1, Part::Noun),
    ("cat", 2, Part::Noun),
    ("chased", 3, Part::Transitive),
    ("bird", 4, Part::Noun),
    ("saw", 5, Part::Transitive),
    ("slept", 6, Part::Intransitive),
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Part {
    Determiner,
    Noun,
    Transitive,
    Intransitive,
}

fn lookup(word: &str) -> (u32, Part) {
    let (_, concept, part) = WORDS
        .iter()
        .find(|(w, _, _)| *w == word)
        .expect("in the lexicon");
    (*concept, *part)
}

fn word_of(concept: u32) -> &'static str {
    WORDS
        .iter()
        .find(|(_, c, _)| *c == concept)
        .expect("in the lexicon")
        .0
}

/// A term arena with a cursor and a fresh-variable counter: every lexical entry is instantiated
/// with variables of its own (standardising apart is the caller's, ADR-0025).
struct Arena {
    nodes: [TermNode; 256],
    len: usize,
    vars: u32,
}

impl Arena {
    fn new() -> Self {
        Self {
            nodes: [TermNode::default(); 256],
            len: 0,
            vars: 0,
        }
    }

    fn push(&mut self, node: TermNode) -> u32 {
        let i = self.len;
        self.nodes[i] = node;
        self.len = self.len.wrapping_add(1);
        i as u32
    }

    fn var(&mut self) -> u32 {
        let v = self.vars;
        self.vars = self.vars.wrapping_add(1);
        self.push(TermNode::variable(v))
    }

    fn constant(&mut self, c: u32) -> u32 {
        self.push(TermNode::constant(c))
    }

    fn atom(&mut self, functor: u32, feature: u32) -> u32 {
        self.push(TermNode::compound(functor, &[feature]).unwrap())
    }

    /// The category of a word, built fresh.
    fn category(&mut self, word: &str) -> u32 {
        let (concept, part) = lookup(word);
        match part {
            // `NP(X)/N(X)`: the head passes up; no role.
            Part::Determiner => {
                let x = self.var();
                let np = self.atom(NP, x);
                let n = self.atom(N, x);
                let none = self.constant(0xFFFF_FE00);
                self.push(forward(np, n, none).unwrap())
            }
            Part::Noun => {
                let head = self.constant(concept);
                self.atom(N, head)
            }
            // `(S(v)\NP(A):subject)/NP(P):object`.
            Part::Transitive => {
                let head = self.constant(concept);
                let s = self.atom(S, head);
                let a = self.var();
                let np_a = self.atom(NP, a);
                let subject = self.constant(role_concept(ROLE_SUBJECT).unwrap());
                let s_np = self.push(backward(s, np_a, subject).unwrap());
                let p = self.var();
                let np_p = self.atom(NP, p);
                let object = self.constant(role_concept(ROLE_OBJECT).unwrap());
                self.push(forward(s_np, np_p, object).unwrap())
            }
            // `S(v)\NP(A):subject`.
            Part::Intransitive => {
                let head = self.constant(concept);
                let s = self.atom(S, head);
                let a = self.var();
                let np_a = self.atom(NP, a);
                let subject = self.constant(role_concept(ROLE_SUBJECT).unwrap());
                self.push(backward(s, np_a, subject).unwrap())
            }
        }
    }

    fn sentence(&mut self, words: &[&str]) -> [u32; 8] {
        let mut out = [0u32; 8];
        for (slot, word) in words.iter().enumerate() {
            out[slot] = self.category(word);
        }
        out
    }
}

/// The codebook (one body per concept, seeded by its index) and the four role bodies.
fn books() -> ([HypervectorBody; 16], [HypervectorBody; 4]) {
    (
        core::array::from_fn(|i| HypervectorBody::from_seed(1_000u64.wrapping_add(i as u64))),
        core::array::from_fn(|i| HypervectorBody::from_seed(7_000u64.wrapping_add(i as u64))),
    )
}

/// Reduces `words` into a causative frame over fresh scratch, returning the frame, the log
/// and the count.
fn comprehend_words(
    arena: &mut Arena,
    words: &[&str],
) -> (
    Result<LinguisticFrameSlot, LanguageError>,
    [Reduction; 8],
    usize,
) {
    let categories = arena.sentence(words);
    let mut bindings = [Binding::UNBOUND; 64];
    let mut trail = [0u32; 64];
    let mut stack = [0u32; 64];
    let mut parse = [0u32; 8];
    let mut steps = [Reduction::default(); 8];
    let free = arena.len;
    let mut scratch = ParseScratch {
        arena: &mut arena.nodes,
        free,
        bindings: &mut bindings,
        trail: &mut trail,
        trail_len: 0,
        stack: &mut stack,
        parse: &mut parse,
        steps: &mut steps,
        step_count: 0,
    };
    let outcome = comprehend(
        &categories[..words.len()],
        &mut scratch,
        TEMPLATE_CAUSATIVE,
        SPEECH_ACT_ASSERTIVE,
        0,
    );
    let count = scratch.step_count;
    arena.len = scratch.free;
    (outcome, steps, count)
}

/// The frame's bound roles in its realisation order, as words of the lexicon.
fn render(frame: &LinguisticFrameSlot) -> [&'static str; 5] {
    let mut out = [""; 5];
    let order = frame.realisation_order().expect("a complete frame");
    for (slot, &role) in order.iter().enumerate() {
        if role == 0 {
            break;
        }
        out[slot] = word_of(concept_in(frame, role).expect("bound"));
    }
    out
}

#[test]
fn the_dog_chased_the_cat_is_read_into_a_frame_sealed_as_a_hypervector_and_read_back() {
    let mut arena = Arena::new();
    let (outcome, steps, count) =
        comprehend_words(&mut arena, &["the", "dog", "chased", "the", "cat"]);
    let mut frame = outcome.unwrap();
    assert_eq!(count, 4);
    assert_eq!(
        [steps[0].rule, steps[1].rule, steps[2].rule, steps[3].rule],
        [
            RULE_FORWARD_APPLICATION,
            RULE_FORWARD_COMPOSITION,
            RULE_FORWARD_APPLICATION,
            RULE_BACKWARD_APPLICATION
        ],
        "the dog | chased the | cat | subject"
    );
    assert_eq!(
        (
            concept_in(&frame, ROLE_SUBJECT),
            concept_in(&frame, ROLE_ACTION),
            concept_in(&frame, ROLE_OBJECT),
            concept_in(&frame, ROLE_AFFECT)
        ),
        (Some(1), Some(3), Some(2), None),
        "dog, chase, cat; no affect"
    );
    assert_eq!(frame.confidence_q16, Q16_ONE, "a parsed filler is certain");
    assert!(frame.is_complete(), "a causative frame has its three roles");
    assert!(frame.seal());
    assert_eq!(
        render(&frame),
        ["dog", "chased", "cat", "", ""],
        "subject, action, object"
    );

    // Sealed as a bundle of three bound pairs; every role read back through the codebook.
    let (book, roles) = books();
    let sealed = encode_frame(&frame, &roles, &book).unwrap();
    let expected = HypervectorBody::bundle(&[
        roles[0].bind(&book[1]),
        roles[1].bind(&book[3]),
        roles[2].bind(&book[2]),
    ])
    .unwrap();
    assert_eq!(sealed, expected, "in slot order");
    let readouts: [(usize, u32, u32); 4] =
        core::array::from_fn(|slot| read_role(&sealed, &roles[slot], &book).unwrap());
    assert_eq!(
        readouts,
        [
            (1, 2_574, 32_588),
            (3, 2_562, 32_742),
            (2, 2_482, 33_766),
            (10, 4_997, 1_574)
        ],
        "each bound role near a quarter of the width; the unbound affect near half"
    );
    for (index, distance, confidence) in readouts {
        assert_eq!(confidence, confidence_q16(distance));
        assert!(index < book.len());
    }
    assert!(
        readouts[3].1 > BODY_BITS * 4 / 10,
        "the affect role is noise"
    );
    let decoded = decode_frame(
        &sealed,
        &roles,
        &book,
        TEMPLATE_CAUSATIVE,
        SPEECH_ACT_ASSERTIVE,
        0,
    );
    assert_eq!(
        (
            concept_in(&decoded, ROLE_SUBJECT),
            concept_in(&decoded, ROLE_ACTION),
            concept_in(&decoded, ROLE_OBJECT),
            concept_in(&decoded, ROLE_AFFECT)
        ),
        (Some(1), Some(3), Some(2), None),
        "the same three concepts; the affect left unbound"
    );
    assert_eq!(
        decoded.filled_roles(),
        ROLE_SUBJECT | ROLE_ACTION | ROLE_OBJECT
    );
    assert_eq!(
        decoded.confidence_q16, 32_588,
        "the weakest readout: the subject's"
    );
    assert!(decoded.is_complete());
    assert_eq!(render(&decoded), render(&frame));

    // The whole path again is bit-identical.
    let mut again = Arena::new();
    let (outcome, _, _) = comprehend_words(&mut again, &["the", "dog", "chased", "the", "cat"]);
    let mut frame2 = outcome.unwrap();
    assert!(frame2.seal());
    assert_eq!(frame2, frame);
    assert_eq!(encode_frame(&frame2, &roles, &book).unwrap(), sealed);
    assert_eq!(arena.nodes, again.nodes);
}

#[test]
fn the_swapped_sentence_reads_back_swapped_and_an_intransitive_one_has_no_object() {
    let (book, roles) = books();
    let mut arena = Arena::new();
    let (outcome, _, _) = comprehend_words(&mut arena, &["the", "cat", "chased", "the", "dog"]);
    let mut frame = outcome.unwrap();
    assert_eq!(
        (
            concept_in(&frame, ROLE_SUBJECT),
            concept_in(&frame, ROLE_OBJECT)
        ),
        (Some(2), Some(1))
    );
    assert!(frame.seal());
    assert_eq!(render(&frame), ["cat", "chased", "dog", "", ""]);
    let sealed = encode_frame(&frame, &roles, &book).unwrap();
    let readouts: [(usize, u32, u32); 4] =
        core::array::from_fn(|slot| read_role(&sealed, &roles[slot], &book).unwrap());
    assert_eq!(
        readouts,
        [
            (2, 2_532, 33_126),
            (3, 2_635, 31_808),
            (1, 2_524, 33_228),
            (5, 5_037, 1_062)
        ]
    );
    let decoded = decode_frame(
        &sealed,
        &roles,
        &book,
        TEMPLATE_CAUSATIVE,
        SPEECH_ACT_ASSERTIVE,
        0,
    );
    assert_eq!(render(&decoded), ["cat", "chased", "dog", "", ""]);

    // A bird slept: subject and action, no object; the causative template is incomplete and
    // a state template would be complete.
    let (outcome, steps, count) = comprehend_words(&mut arena, &["the", "bird", "slept"]);
    let bird = outcome.unwrap();
    assert_eq!(count, 2);
    assert_eq!(
        [steps[0].rule, steps[1].rule],
        [RULE_FORWARD_APPLICATION, RULE_BACKWARD_APPLICATION]
    );
    assert_eq!(
        (
            concept_in(&bird, ROLE_SUBJECT),
            concept_in(&bird, ROLE_ACTION),
            concept_in(&bird, ROLE_OBJECT)
        ),
        (Some(4), Some(6), None)
    );
    assert!(!bird.is_complete(), "a causative frame needs its object");
    let sealed = encode_frame(&bird, &roles, &book).unwrap();
    let decoded = decode_frame(
        &sealed,
        &roles,
        &book,
        TEMPLATE_CAUSATIVE,
        SPEECH_ACT_ASSERTIVE,
        0,
    );
    assert_eq!(
        (
            concept_in(&decoded, ROLE_SUBJECT),
            concept_in(&decoded, ROLE_ACTION)
        ),
        (Some(4), Some(6))
    );
    assert_eq!(
        decoded.filled_roles(),
        ROLE_SUBJECT | ROLE_ACTION,
        "two roles read back, two as noise"
    );
}

#[test]
fn a_non_sentence_is_refused_with_the_reducer_s_reason_and_nothing_is_bound() {
    let mut arena = Arena::new();
    let (outcome, _, count) = comprehend_words(&mut arena, &["dog", "cat"]);
    assert_eq!(
        outcome,
        Err(LanguageError::Parse(ParseError::NoDerivation {
            remaining: 2
        }))
    );
    assert_eq!(count, 0);
    let (outcome, _, _) = comprehend_words(&mut arena, &[]);
    assert_eq!(outcome, Err(LanguageError::Parse(ParseError::Empty)));
    let (outcome, _, count) = comprehend_words(&mut arena, &["the", "dog", "the", "cat", "chased"]);
    assert_eq!(
        outcome,
        Err(LanguageError::Parse(ParseError::NoDerivation {
            remaining: 3
        })),
        "an object relative: the greedy reducer never type-raises"
    );
    assert_eq!(count, 2, "the two noun phrases were built");
    assert_eq!(
        LanguageError::from(ParseError::Empty),
        LanguageError::Parse(ParseError::Empty)
    );
}
