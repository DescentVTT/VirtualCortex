//! The exit test of brief 020 (whitepaper §6.9; ADR-0039, ADR-0040) and, since brief 023, of
//! the lexicon (ADR-0046): a sentence, as token ids of a lexicon that exists in this file
//! alone, is read into a frame, the frame is realised back into token ids in the template's
//! order with its markers, sealed as a hypervector and read back through the codebook, role
//! by role, with every distance pinned so that the AArch64 job holds the arithmetic equal.
//! No word reaches a record: the table maps words to token ids on the way in and token ids to
//! words on the way out, for the assertions' messages only.

#![deny(clippy::arithmetic_side_effects)]

use cortex_linguistic::{
    GATE_PARTICLE_OPEN, LinguisticFrameSlot, POLITENESS_FORMAL, PROSODY_NONE, PROSODY_SUGGEST,
    Q16_ONE, ROLE_ACTION, ROLE_AFFECT, ROLE_OBJECT, ROLE_SUBJECT, SPEECH_ACT_ASSERTIVE,
    SPEECH_ACT_DIRECTIVE, TEMPLATE_CAUSAL, TEMPLATE_CAUSATIVE, TEMPLATE_EPISTEMIC, TEMPLATE_STATE,
};
use cortex_reasoning::{
    Binding, ParseError, ParseScratch, RULE_BACKWARD_APPLICATION, RULE_FORWARD_APPLICATION,
    RULE_FORWARD_COMPOSITION, Reduction, TermNode,
};
use cortex_runtime::{
    Entry, LanguageError, Lexicon, LexiconError, Reading, SHAPE_ADJECTIVE, SHAPE_DETERMINER,
    SHAPE_HEDGE, SHAPE_INTRANSITIVE, SHAPE_NOMINAL, SHAPE_NOUN, SHAPE_TAG, SHAPE_TRANSITIVE,
    comprehend_tokens, concept_in, decode_frame, encode_frame, read_role, realise,
};
use cortex_symbolic::{BODY_BITS, HypervectorBody, confidence_q16};

// Atomic category functors, above every concept and below the role band.
const S: u32 = 0x1000;
const NP: u32 = 0x1001;
const N: u32 = 0x1002;

/// The words, by token id: the table of §1.5, the only place a word exists. A token id is
/// the host's; here it is 1 000 plus the word's place. The concept is the codebook index. A
/// noun needs its determiner; a pronoun and a name are noun phrases on their own.
const WORDS: [(&str, u32, u8, u32); 13] = [
    ("the", 0, SHAPE_DETERMINER, 0),
    ("dog", 1, SHAPE_NOUN, 0),
    ("cat", 2, SHAPE_NOUN, 0),
    ("chased", 3, SHAPE_TRANSITIVE, 0),
    ("bird", 4, SHAPE_NOUN, 0),
    ("saw", 5, SHAPE_TRANSITIVE, 0),
    ("slept", 6, SHAPE_INTRANSITIVE, 0),
    ("maybe", 7, SHAPE_HEDGE, 0),
    ("sadly", 8, SHAPE_TAG, 0),
    ("big", 9, SHAPE_ADJECTIVE, 0),
    ("you", 10, SHAPE_NOMINAL, FORMAL_YOU),
    ("spot", 11, SHAPE_NOMINAL, 0),
    ("felix", 12, SHAPE_NOMINAL, 0),
];
/// The formal register's token for "you" (a variant the table realises, never comprehends).
const FORMAL_YOU: u32 = 1_999;
/// The markers: an opener for a directive, a closer for an expressive act, five particles.
const PLEASE: u32 = 2_001;
const BANG: u32 = 2_003;
const PARTICLES: [u32; 6] = [0, 3_001, 3_002, 3_003, 3_004, 3_005];

fn token(word: &str) -> u32 {
    let place = WORDS
        .iter()
        .position(|(w, _, _, _)| *w == word)
        .expect("in the lexicon");
    1_000u32.wrapping_add(place as u32)
}

fn word_of(token: u32) -> &'static str {
    if token == FORMAL_YOU {
        return "you (formal)";
    }
    if token == PLEASE {
        return "please";
    }
    if token == BANG {
        return "!";
    }
    if let Some(p) = PARTICLES.iter().position(|&t| t == token) {
        return ["", "ne", "ba", "actually", "anyway", "haha"][p];
    }
    WORDS[token.wrapping_sub(1_000) as usize].0
}

fn entries() -> [Entry; 13] {
    core::array::from_fn(|i| Entry {
        token: 1_000u32.wrapping_add(i as u32),
        concept: WORDS[i].1,
        shape: WORDS[i].2,
        formal: WORDS[i].3,
    })
}

fn lexicon(entries: &[Entry]) -> Lexicon<'_> {
    Lexicon {
        entries,
        sentence: S,
        np: NP,
        n: N,
        act_open: [0, PLEASE, 0, 0],
        act_close: [0, 0, 0, BANG],
        particles: PARTICLES,
    }
}

/// The slices a reading runs over: the arena and the tables, sized once.
struct Slices {
    arena: [TermNode; 256],
    free: usize,
    vars: u32,
    bindings: [Binding; 64],
    trail: [u32; 64],
    stack: [u32; 64],
    parse: [u32; 8],
    steps: [Reduction; 8],
}

impl Slices {
    fn new() -> Self {
        Self {
            arena: [TermNode::default(); 256],
            free: 0,
            vars: 0,
            bindings: [Binding::UNBOUND; 64],
            trail: [0; 64],
            stack: [0; 64],
            parse: [0; 8],
            steps: [Reduction::default(); 8],
        }
    }

    /// Reads `words` into a frame of `template` at `politeness`, the reductions and their
    /// count returned beside it; the arena keeps every category and the counter advances.
    fn read(
        &mut self,
        words: &[&str],
        template: u16,
        politeness: u8,
    ) -> (
        Result<LinguisticFrameSlot, LexiconError>,
        [Reduction; 8],
        usize,
    ) {
        let entries = entries();
        let lex = lexicon(&entries);
        let mut tokens = [0u32; 8];
        for (slot, word) in words.iter().enumerate() {
            tokens[slot] = match *word {
                "please" => PLEASE,
                "!" => BANG,
                "ba" => PARTICLES[PROSODY_SUGGEST as usize],
                w => token(w),
            };
        }
        let mut categories = [0u32; 8];
        let mut reading = Reading {
            categories: &mut categories,
            next_variable: self.vars,
        };
        let mut scratch = ParseScratch {
            arena: &mut self.arena,
            free: self.free,
            bindings: &mut self.bindings,
            trail: &mut self.trail,
            trail_len: 0,
            stack: &mut self.stack,
            parse: &mut self.parse,
            steps: &mut self.steps,
            step_count: 0,
        };
        let outcome = comprehend_tokens(
            &tokens[..words.len()],
            &lex,
            &mut scratch,
            &mut reading,
            template,
            SPEECH_ACT_ASSERTIVE,
            politeness,
        );
        let count = scratch.step_count;
        self.free = scratch.free;
        self.vars = reading.next_variable;
        (outcome, self.steps, count)
    }
}

/// The codebook (one body per concept, seeded by its index) and the four role bodies.
fn books() -> ([HypervectorBody; 16], [HypervectorBody; 4]) {
    (
        core::array::from_fn(|i| HypervectorBody::from_seed(1_000u64.wrapping_add(i as u64))),
        core::array::from_fn(|i| HypervectorBody::from_seed(7_000u64.wrapping_add(i as u64))),
    )
}

/// The frames at `root` realised into token ids, and those as words.
fn render(frames: &mut [LinguisticFrameSlot], root: u16) -> (Vec<u32>, Vec<&'static str>) {
    let entries = entries();
    let lex = lexicon(&entries);
    let mut out = [0u32; 16];
    let n = realise(frames, root, &lex, &mut out).expect("a complete frame");
    let tokens = out[..n].to_vec();
    let words = tokens.iter().map(|&t| word_of(t)).collect();
    (tokens, words)
}

fn roles_of(frame: &LinguisticFrameSlot) -> [Option<u32>; 4] {
    [
        concept_in(frame, ROLE_SUBJECT),
        concept_in(frame, ROLE_ACTION),
        concept_in(frame, ROLE_OBJECT),
        concept_in(frame, ROLE_AFFECT),
    ]
}

#[test]
fn the_dog_chased_the_cat_is_read_into_a_frame_realised_sealed_as_a_hypervector_and_read_back() {
    let mut slices = Slices::new();
    let (outcome, steps, count) = slices.read(
        &["the", "dog", "chased", "the", "cat"],
        TEMPLATE_CAUSATIVE,
        0,
    );
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
        roles_of(&frame),
        [Some(1), Some(3), Some(2), None],
        "dog, chase, cat; no affect"
    );
    assert_eq!(frame.confidence_q16, Q16_ONE, "a parsed filler is certain");
    assert!(frame.is_complete(), "a causative frame has its three roles");
    assert!(frame.seal());
    // Realised: subject, action, object as token ids, the determiners gone; the last token
    // realised is the frame's surface token.
    let mut frames = [frame];
    let (tokens, words) = render(&mut frames, 0);
    assert_eq!(tokens, vec![token("dog"), token("chased"), token("cat")]);
    assert_eq!(words, vec!["dog", "chased", "cat"]);
    assert_eq!(frames[0].surface_token_id, token("cat"));

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
        roles_of(&decoded),
        [Some(1), Some(3), Some(2), None],
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
    let mut decoded = [decoded];
    assert_eq!(
        render(&mut decoded, 0).0,
        tokens,
        "the decoded frame realises the same tokens"
    );

    // The whole path again is bit-identical.
    let mut again = Slices::new();
    let (outcome, _, _) = again.read(
        &["the", "dog", "chased", "the", "cat"],
        TEMPLATE_CAUSATIVE,
        0,
    );
    let mut frame2 = outcome.unwrap();
    assert!(frame2.seal());
    assert_eq!(frame2, frame);
    assert_eq!(encode_frame(&frame2, &roles, &book).unwrap(), sealed);
    assert_eq!(slices.arena, again.arena);
}

#[test]
fn the_swapped_sentence_reads_back_swapped_and_an_intransitive_one_has_no_object() {
    let (book, roles) = books();
    let mut slices = Slices::new();
    let (outcome, _, _) = slices.read(
        &["the", "cat", "chased", "the", "dog"],
        TEMPLATE_CAUSATIVE,
        0,
    );
    let mut frame = outcome.unwrap();
    assert_eq!(
        (
            concept_in(&frame, ROLE_SUBJECT),
            concept_in(&frame, ROLE_OBJECT)
        ),
        (Some(2), Some(1))
    );
    assert!(frame.seal());
    let mut frames = [frame];
    assert_eq!(render(&mut frames, 0).1, vec!["cat", "chased", "dog"]);
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
    let mut decoded = [decoded];
    assert_eq!(render(&mut decoded, 0).1, vec!["cat", "chased", "dog"]);

    // A bird slept: subject and action, no object; the causative template is incomplete and
    // a state template would be complete.
    let (outcome, steps, count) = slices.read(&["the", "bird", "slept"], TEMPLATE_CAUSATIVE, 0);
    let bird = outcome.unwrap();
    assert_eq!(count, 2);
    assert_eq!(
        [steps[0].rule, steps[1].rule],
        [RULE_FORWARD_APPLICATION, RULE_BACKWARD_APPLICATION]
    );
    assert_eq!(roles_of(&bird), [Some(4), Some(6), None, None]);
    assert!(!bird.is_complete(), "a causative frame needs its object");
    let entries = entries();
    let mut out = [0u32; 4];
    assert_eq!(
        realise(&mut [bird], 0, &lexicon(&entries), &mut out),
        Err(LexiconError::Incomplete(0)),
        "and cannot be realised"
    );
    let sealed = encode_frame(&bird, &roles, &book).unwrap();
    let decoded = decode_frame(
        &sealed,
        &roles,
        &book,
        TEMPLATE_CAUSATIVE,
        SPEECH_ACT_ASSERTIVE,
        0,
    );
    assert_eq!(roles_of(&decoded), [Some(4), Some(6), None, None]);
    assert_eq!(
        decoded.filled_roles(),
        ROLE_SUBJECT | ROLE_ACTION,
        "two roles read back, two as noise"
    );
}

#[test]
fn a_hedge_opens_and_a_tag_closes_an_utterance_and_each_fills_the_affect_role_with_its_own_concept()
{
    let (book, roles) = books();
    let mut slices = Slices::new();
    // "maybe the dog chased the cat": an epistemic frame, the hedge realised first.
    let (outcome, _, count) = slices.read(
        &["maybe", "the", "dog", "chased", "the", "cat"],
        TEMPLATE_EPISTEMIC,
        0,
    );
    let mut hedged = outcome.unwrap();
    assert_eq!(
        count, 5,
        "the four of the sentence, then the hedge's application"
    );
    assert_eq!(roles_of(&hedged), [Some(1), Some(3), Some(2), Some(7)]);
    assert!(hedged.is_complete() && hedged.seal());
    let mut frames = [hedged];
    let (tokens, words) = render(&mut frames, 0);
    assert_eq!(words, vec!["maybe", "dog", "chased", "cat"]);
    assert_eq!(
        tokens,
        vec![token("maybe"), token("dog"), token("chased"), token("cat")]
    );
    // Sealed with four roles and read back: the hedge's concept in the affect slot.
    let sealed = encode_frame(&hedged, &roles, &book).unwrap();
    let readouts: [(usize, u32, u32); 4] =
        core::array::from_fn(|slot| read_role(&sealed, &roles[slot], &book).unwrap());
    assert_eq!(
        readouts,
        [
            (1, 3_195, 24_640),
            (3, 3_157, 25_126),
            (2, 3_157, 25_126),
            (7, 3_227, 24_230)
        ],
        "four bound roles, each near three tenths of the width"
    );
    let decoded = decode_frame(
        &sealed,
        &roles,
        &book,
        TEMPLATE_EPISTEMIC,
        SPEECH_ACT_ASSERTIVE,
        0,
    );
    assert_eq!(roles_of(&decoded), [Some(1), Some(3), Some(2), Some(7)]);
    let mut decoded = [decoded];
    assert_eq!(render(&mut decoded, 0).0, tokens);

    // "the dog slept sadly": a state frame with its tag, realised last.
    let (outcome, _, count) = slices.read(&["the", "dog", "slept", "sadly"], TEMPLATE_STATE, 0);
    let mut tagged = outcome.unwrap();
    assert_eq!(count, 3);
    assert_eq!(roles_of(&tagged), [Some(1), Some(6), None, Some(8)]);
    assert!(tagged.seal());
    let mut frames = [tagged];
    assert_eq!(render(&mut frames, 0).1, vec!["dog", "slept", "sadly"]);
    // A tag before its sentence is no derivation: its shape is sentence-final.
    let (outcome, _, _) = slices.read(&["sadly", "the", "dog", "slept"], TEMPLATE_STATE, 0);
    assert!(matches!(
        outcome,
        Err(LexiconError::Language(LanguageError::Parse(
            ParseError::NoDerivation { remaining: 2 }
        )))
    ));
    // An adjective passes the head up and reaches no slot; without a determiner the phrase
    // is a noun, not a noun phrase, and the verb finds no subject.
    let (outcome, _, _) = slices.read(&["the", "big", "dog", "slept"], TEMPLATE_STATE, 0);
    assert_eq!(roles_of(&outcome.unwrap()), [Some(1), Some(6), None, None]);
    let (outcome, _, _) = slices.read(&["big", "dog", "slept"], TEMPLATE_STATE, 0);
    assert!(matches!(
        outcome,
        Err(LexiconError::Language(LanguageError::Parse(
            ParseError::NoDerivation { remaining: 2 }
        )))
    ));
}

#[test]
fn markers_the_register_and_a_nested_clause_are_realised_and_the_markers_are_read_back() {
    let mut slices = Slices::new();
    // "please spot chased felix ba": the opener sets the act, the particle the marker, and
    // the names are noun phrases on their own, so the realised utterance is the input.
    let (outcome, _, count) = slices.read(
        &["please", "spot", "chased", "felix", "ba"],
        TEMPLATE_CAUSATIVE,
        1,
    );
    let mut asked = outcome.unwrap();
    assert_eq!(count, 2, "the markers are not categories; two applications");
    assert_eq!(asked.speech_act_type, SPEECH_ACT_DIRECTIVE);
    assert_eq!(asked.prosody_tone_marker, PROSODY_SUGGEST);
    assert_ne!(asked.syntax_gate_flags & GATE_PARTICLE_OPEN, 0);
    assert_eq!(asked.politeness_level, 1);
    assert!(asked.seal());
    let mut frames = [asked];
    let (tokens, words) = render(&mut frames, 0);
    assert_eq!(words, vec!["please", "spot", "chased", "felix", "ba"]);
    assert_eq!(
        tokens,
        vec![
            PLEASE,
            token("spot"),
            token("chased"),
            token("felix"),
            PARTICLES[2]
        ]
    );
    assert_eq!(frames[0].surface_token_id, PARTICLES[2]);
    // Read back: the same act, the same marker, the same roles, the same tokens realised.
    let mut tokens_in = [0u32; 8];
    tokens_in[..5].copy_from_slice(&tokens);
    let entries = entries();
    let lex = lexicon(&entries);
    let mut categories = [0u32; 8];
    let mut reading = Reading {
        categories: &mut categories,
        next_variable: slices.vars,
    };
    let mut scratch = ParseScratch {
        arena: &mut slices.arena,
        free: slices.free,
        bindings: &mut slices.bindings,
        trail: &mut slices.trail,
        trail_len: 0,
        stack: &mut slices.stack,
        parse: &mut slices.parse,
        steps: &mut slices.steps,
        step_count: 0,
    };
    let back = comprehend_tokens(
        &tokens_in[..5],
        &lex,
        &mut scratch,
        &mut reading,
        TEMPLATE_CAUSATIVE,
        SPEECH_ACT_ASSERTIVE,
        1,
    )
    .unwrap();
    assert_eq!(
        (
            back.speech_act_type,
            back.prosody_tone_marker,
            roles_of(&back)
        ),
        (SPEECH_ACT_DIRECTIVE, PROSODY_SUGGEST, roles_of(&asked))
    );
    slices.free = scratch.free;
    slices.vars = reading.next_variable;
    let mut back = back;
    assert!(back.seal());
    let mut frames = [back];
    assert_eq!(render(&mut frames, 0).0, tokens, "the round trip is closed");

    // A causal frame "you chased the cat [because] the dog slept", the child a state frame
    // comprehended on its own and bound into the child slot; the formal register realises
    // "you" by its variant.
    let (outcome, _, _) = slices.read(&["the", "dog", "slept"], TEMPLATE_STATE, POLITENESS_FORMAL);
    let mut child = outcome.unwrap();
    let mut root =
        LinguisticFrameSlot::new(TEMPLATE_CAUSAL, SPEECH_ACT_ASSERTIVE, POLITENESS_FORMAL);
    assert!(root.bind_role(ROLE_SUBJECT, 10, Q16_ONE));
    assert!(root.bind_role(ROLE_ACTION, 3, Q16_ONE));
    assert!(root.bind_role(ROLE_OBJECT, 2, Q16_ONE));
    assert!(!root.is_complete(), "a causal frame needs its clause");
    assert!(root.bind_child(1, 0));
    assert!(child.set_parent(0));
    assert!(root.is_complete() && root.seal() && child.seal());
    let mut frames = [root, child];
    let (tokens, words) = render(&mut frames, 0);
    assert_eq!(words, vec!["you (formal)", "chased", "cat", "dog", "slept"]);
    assert_eq!(tokens[0], FORMAL_YOU);
    assert_eq!(frames[0].surface_token_id, token("slept"));
    assert_eq!(
        frames[1].surface_token_id, 0,
        "the child is realised, not the root"
    );
    // At a plain register the same frame realises the plain token.
    frames[0].politeness_level = 0;
    assert_eq!(render(&mut frames, 0).1[0], "you");
    assert_eq!(PROSODY_NONE, 0);
}

#[test]
fn a_non_sentence_is_refused_with_the_reducer_s_reason_and_nothing_is_bound() {
    let mut slices = Slices::new();
    let (outcome, _, count) = slices.read(&["dog", "cat"], TEMPLATE_CAUSATIVE, 0);
    assert_eq!(
        outcome,
        Err(LexiconError::Language(LanguageError::Parse(
            ParseError::NoDerivation { remaining: 2 }
        )))
    );
    assert_eq!(count, 0);
    let (outcome, _, _) = slices.read(&[], TEMPLATE_CAUSATIVE, 0);
    assert_eq!(
        outcome,
        Err(LexiconError::Language(LanguageError::Parse(
            ParseError::Empty
        )))
    );
    let (outcome, _, count) = slices.read(
        &["the", "dog", "the", "cat", "chased"],
        TEMPLATE_CAUSATIVE,
        0,
    );
    assert_eq!(
        outcome,
        Err(LexiconError::Language(LanguageError::Parse(
            ParseError::NoDerivation { remaining: 3 }
        ))),
        "an object relative: the greedy reducer never type-raises"
    );
    assert_eq!(count, 2, "the two noun phrases were built");
    // A phrase reduces to one category that is not the sentence's: no frame, no action.
    let (outcome, _, count) = slices.read(&["the", "dog"], TEMPLATE_CAUSATIVE, 0);
    assert!(
        matches!(
            outcome,
            Err(LexiconError::Language(LanguageError::NotASentence(_)))
        ),
        "a noun phrase is not an utterance: {outcome:?}"
    );
    assert_eq!(count, 1, "the phrase was built");
    let (outcome, _, _) = slices.read(&["dog"], TEMPLATE_CAUSATIVE, 0);
    assert!(matches!(
        outcome,
        Err(LexiconError::Language(LanguageError::NotASentence(_)))
    ));
    // A token the table does not hold is refused at its position, before the reducer runs.
    let entries = entries();
    let lex = lexicon(&entries);
    let mut categories = [0u32; 8];
    let mut reading = Reading {
        categories: &mut categories,
        next_variable: slices.vars,
    };
    let mut scratch = ParseScratch {
        arena: &mut slices.arena,
        free: slices.free,
        bindings: &mut slices.bindings,
        trail: &mut slices.trail,
        trail_len: 0,
        stack: &mut slices.stack,
        parse: &mut slices.parse,
        steps: &mut slices.steps,
        step_count: 0,
    };
    assert_eq!(
        comprehend_tokens(
            &[token("the"), token("dog"), 4_242],
            &lex,
            &mut scratch,
            &mut reading,
            TEMPLATE_STATE,
            SPEECH_ACT_ASSERTIVE,
            0
        ),
        Err(LexiconError::UnknownToken(2))
    );
    assert_eq!(scratch.step_count, 0);
    assert_eq!(
        comprehend_tokens(
            &[FORMAL_YOU, token("slept")],
            &lex,
            &mut scratch,
            &mut reading,
            TEMPLATE_STATE,
            SPEECH_ACT_ASSERTIVE,
            0
        ),
        Err(LexiconError::UnknownToken(0)),
        "a formal variant is realised, never comprehended"
    );
}
