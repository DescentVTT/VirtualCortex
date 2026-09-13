//! The lexicon (whitepaper §5.2.20, §6.9; ADR-0046): the runtime's table between a host's
//! token ids and the language stream's concept ids, in both directions, on ids alone. A
//! token's entry carries its concept and its lexical shape, the category the reducer of
//! ADR-0040 takes, which [`comprehend_tokens`] instantiates on the term arena with fresh
//! variables and reads into a frame through [`comprehend`]; a token that is one of the
//! lexicon's speech-act markers sets the frame's act, one that is a prosody particle sets its
//! marker. [`realise`] emits a complete frame's roles in the template's order, each concept
//! as its token (the formal register's variant at or above `POLITENESS_FORMAL`), descending
//! into a nested frame at `ROLE_CHILD`, with the act's opener and closer and the particle
//! when the slot is open, and writes the last token into `surface_token_id`. No word is here:
//! a token is an id the host chose (§1.5), the table is the caller's slice, and nothing
//! allocates.

use crate::language::{LanguageError, ROLE_CONCEPT_BASE, comprehend, concept_in, role_concept};
use cortex_linguistic::{
    GATE_PARTICLE_OPEN, LinguisticFrameSlot, POLITENESS_FORMAL, PROSODY_NONE, ROLE_AFFECT,
    ROLE_CHILD, ROLE_OBJECT, ROLE_SUBJECT,
};
use cortex_reasoning::{ParseScratch, TermNode, backward, forward};

/// A noun: `N(c)`, its concept the head.
pub const SHAPE_NOUN: u8 = 1;
/// A determiner: `NP(X)/N(X)`, the head passing up, no role.
pub const SHAPE_DETERMINER: u8 = 2;
/// An adjective: `N(X)/N(X)`, the head passing up, no role; its concept reaches no slot.
pub const SHAPE_ADJECTIVE: u8 = 3;
/// An intransitive verb: `S(c)\NP(A):subject`.
pub const SHAPE_INTRANSITIVE: u8 = 4;
/// A transitive verb: `(S(c)\NP(A):subject)/NP(P):object`.
pub const SHAPE_TRANSITIVE: u8 = 5;
/// A hedge that opens an utterance: `S(V)/S(V)`, its role term `AFFECT(c)` naming its own
/// concept as the affect filler.
pub const SHAPE_HEDGE: u8 = 6;
/// A tag that closes an utterance: `S(V)\S(V)`, likewise.
pub const SHAPE_TAG: u8 = 7;
/// A bare noun phrase, `NP(c)`: a name, a pronoun, a mass noun, or every noun of a language
/// without articles; what a realised utterance holds where a noun stood, so that it can be
/// read back.
pub const SHAPE_NOMINAL: u8 = 8;

/// The most frames a realisation descends through: a child slot nested deeper is refused.
pub const MAX_NESTING: usize = 8;

/// The steps a frame costs the realisation's walk: its five positions and its return.
const STEPS_PER_FRAME: usize = 6;

/// One word of the table: a token id, its concept, its lexical shape, and the token the
/// formal register uses in its place (0 for none).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Entry {
    pub token: u32,
    pub concept: u32,
    pub shape: u8,
    pub formal: u32,
}

/// Why a sequence did not become a frame, or a frame a sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LexiconError {
    /// The table is not one the rules can use (`Lexicon::is_well_formed`).
    Malformed,
    /// The token at this position of the sequence has no entry and is no marker.
    UnknownToken(usize),
    /// The token at this position has an entry whose shape is none of the constants.
    UnknownShape(usize),
    /// The arena has no node for a category, or its index would not fit the encoding.
    ArenaFull,
    /// The category slice is too small for the sequence.
    CategoriesFull,
    /// The reducer's or the composition's reason.
    Language(LanguageError),
    /// The frame at this index is not complete, so it has no realisation order.
    Incomplete(u16),
    /// No frame at this index of the arena.
    NoSuchFrame(u16),
    /// A concept the frame holds has no entry.
    NoToken(u32),
    /// The child slots nest deeper than `MAX_NESTING`, or they form a cycle.
    TooDeep,
    /// The output is too small for the utterance.
    OutFull,
}

impl From<LanguageError> for LexiconError {
    fn from(error: LanguageError) -> Self {
        Self::Language(error)
    }
}

/// The table: entries sorted by token, the three atomic category functors (the sentence,
/// the noun phrase, the noun), the speech-act markers (an opener and a closer per act, 0
/// for none) and the prosody particles (one per `PROSODY_*` marker, 0 for none; the first is
/// always 0). A caller's slice: no allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lexicon<'a> {
    pub entries: &'a [Entry],
    pub sentence: u32,
    pub np: u32,
    pub n: u32,
    pub act_open: [u32; 4],
    pub act_close: [u32; 4],
    pub particles: [u32; 6],
}

/// The caller's slices for a reading: the categories a token sequence instantiates, one per
/// token that is not a marker, and the fresh-variable counter they are instantiated from
/// (standardising apart is the caller's, ADR-0025): every reading advances it, so that two
/// readings over one scratch never share a variable.
pub struct Reading<'a> {
    pub categories: &'a mut [u32],
    pub next_variable: u32,
}

impl Lexicon<'_> {
    /// True for a table the rules can use: the entries strictly sorted by token with no token
    /// zero and every shape one of the eight; the three atoms distinct and below the role
    /// band; every marker that is not zero no entry's token and no other marker's; the
    /// particle of `PROSODY_NONE` zero.
    pub fn is_well_formed(&self) -> bool {
        self.entries.windows(2).all(|w| w[0].token < w[1].token)
            && self
                .entries
                .iter()
                .all(|e| e.token != 0 && (SHAPE_NOUN..=SHAPE_NOMINAL).contains(&e.shape))
            && self.sentence != self.np
            && self.np != self.n
            && self.sentence != self.n
            && [self.sentence, self.np, self.n]
                .iter()
                .all(|&atom| atom < ROLE_CONCEPT_BASE)
            && self.particles[0] == 0
            && self.markers().enumerate().all(|(i, marker)| {
                marker == 0
                    || (self.lookup(marker).is_none()
                        && self
                            .markers()
                            .skip(i.wrapping_add(1))
                            .all(|other| other != marker))
            })
    }

    /// Every marker, the openers then the closers then the particles.
    fn markers(&self) -> impl Iterator<Item = u32> + '_ {
        self.act_open
            .iter()
            .chain(self.act_close.iter())
            .chain(self.particles.iter())
            .copied()
    }

    /// The entry of `token`, by binary search over the sorted entries.
    pub fn lookup(&self, token: u32) -> Option<&Entry> {
        self.entries
            .binary_search_by_key(&token, |e| e.token)
            .ok()
            .and_then(|i| self.entries.get(i))
    }

    /// The token that realises `concept` at `politeness`: the first entry (by token) with
    /// that concept, its formal variant at or above `POLITENESS_FORMAL` when it has one.
    pub fn token_of(&self, concept: u32, politeness: u8) -> Option<u32> {
        let entry = self.entries.iter().find(|e| e.concept == concept)?;
        Some(if politeness >= POLITENESS_FORMAL && entry.formal != 0 {
            entry.formal
        } else {
            entry.token
        })
    }

    /// The act a marker token names, opener or closer.
    fn act_of(&self, token: u32) -> Option<u8> {
        marker_at(&self.act_open, token)
            .or_else(|| marker_at(&self.act_close, token))
            .map(|act| act as u8)
    }

    /// The `PROSODY_*` marker a particle token names.
    fn particle_of(&self, token: u32) -> Option<u8> {
        marker_at(&self.particles, token).map(|marker| marker as u8)
    }
}

/// The position of `token` in a marker table; a token of zero is never a marker.
fn marker_at(table: &[u32], token: u32) -> Option<usize> {
    if token == 0 {
        return None;
    }
    table.iter().position(|&t| t == token)
}

/// The concept id of a role constant, at compile time.
const fn role_constant(role: u8) -> u32 {
    match role_concept(role) {
        Some(concept) => concept,
        None => panic!("not a role bit"),
    }
}

const SUBJECT: u32 = role_constant(ROLE_SUBJECT);
const OBJECT: u32 = role_constant(ROLE_OBJECT);
const AFFECT: u32 = role_constant(ROLE_AFFECT);

/// Writes `node` at the scratch's cursor and returns its index; `ArenaFull` past the arena or
/// past the index the encoding holds.
fn push(scratch: &mut ParseScratch, node: TermNode) -> Result<u32, LexiconError> {
    let index = u32::try_from(scratch.free).map_err(|_| LexiconError::ArenaFull)?;
    if index == u32::MAX {
        return Err(LexiconError::ArenaFull);
    }
    let slot = scratch
        .arena
        .get_mut(scratch.free)
        .ok_or(LexiconError::ArenaFull)?;
    *slot = node;
    // Below the arena's length, checked above.
    scratch.free = scratch.free.wrapping_add(1);
    Ok(index)
}

/// A fresh variable.
fn variable(scratch: &mut ParseScratch, next: &mut u32) -> Result<u32, LexiconError> {
    let number = *next;
    *next = number.wrapping_add(1);
    push(scratch, TermNode::variable(number))
}

/// An atomic category `functor(feature)`.
fn atom(scratch: &mut ParseScratch, functor: u32, feature: u32) -> Result<u32, LexiconError> {
    let node = TermNode::compound(functor, &[feature]).ok_or(LexiconError::ArenaFull)?;
    push(scratch, node)
}

/// A functor category, forward or backward.
fn slash(
    scratch: &mut ParseScratch,
    is_forward: bool,
    result: u32,
    argument: u32,
    role: u32,
) -> Result<u32, LexiconError> {
    let node = if is_forward {
        forward(result, argument, role)
    } else {
        backward(result, argument, role)
    };
    push(scratch, node.ok_or(LexiconError::ArenaFull)?)
}

/// The category of `entry`, built at the scratch's cursor with variables from `next`: the
/// index of its root. On a refusal the cursor and the counter are put back, so nothing an
/// earlier category reaches has moved.
fn instantiate(
    entry: &Entry,
    lexicon: &Lexicon,
    scratch: &mut ParseScratch,
    next: &mut u32,
    position: usize,
) -> Result<u32, LexiconError> {
    let (free, counter) = (scratch.free, *next);
    let built = build(entry, lexicon, scratch, next, position);
    if built.is_err() {
        scratch.free = free;
        *next = counter;
    }
    built
}

fn build(
    entry: &Entry,
    lexicon: &Lexicon,
    scratch: &mut ParseScratch,
    next: &mut u32,
    position: usize,
) -> Result<u32, LexiconError> {
    // A role term that names no role: a constant of the band with no bit.
    let none = TermNode::constant(ROLE_CONCEPT_BASE);
    match entry.shape {
        SHAPE_NOUN => {
            let head = push(scratch, TermNode::constant(entry.concept))?;
            atom(scratch, lexicon.n, head)
        }
        SHAPE_NOMINAL => {
            let head = push(scratch, TermNode::constant(entry.concept))?;
            atom(scratch, lexicon.np, head)
        }
        SHAPE_DETERMINER => {
            let x = variable(scratch, next)?;
            let np = atom(scratch, lexicon.np, x)?;
            let n = atom(scratch, lexicon.n, x)?;
            let role = push(scratch, none)?;
            slash(scratch, true, np, n, role)
        }
        SHAPE_ADJECTIVE => {
            let x = variable(scratch, next)?;
            let result = atom(scratch, lexicon.n, x)?;
            let argument = atom(scratch, lexicon.n, x)?;
            let role = push(scratch, none)?;
            slash(scratch, true, result, argument, role)
        }
        SHAPE_INTRANSITIVE => {
            let head = push(scratch, TermNode::constant(entry.concept))?;
            let s = atom(scratch, lexicon.sentence, head)?;
            let a = variable(scratch, next)?;
            let np_a = atom(scratch, lexicon.np, a)?;
            let subject = push(scratch, TermNode::constant(SUBJECT))?;
            slash(scratch, false, s, np_a, subject)
        }
        SHAPE_TRANSITIVE => {
            let head = push(scratch, TermNode::constant(entry.concept))?;
            let s = atom(scratch, lexicon.sentence, head)?;
            let a = variable(scratch, next)?;
            let np_a = atom(scratch, lexicon.np, a)?;
            let subject = push(scratch, TermNode::constant(SUBJECT))?;
            let s_np = slash(scratch, false, s, np_a, subject)?;
            let p = variable(scratch, next)?;
            let np_p = atom(scratch, lexicon.np, p)?;
            let object = push(scratch, TermNode::constant(OBJECT))?;
            slash(scratch, true, s_np, np_p, object)
        }
        SHAPE_HEDGE | SHAPE_TAG => {
            let v = variable(scratch, next)?;
            let result = atom(scratch, lexicon.sentence, v)?;
            let argument = atom(scratch, lexicon.sentence, v)?;
            let filler = push(scratch, TermNode::constant(entry.concept))?;
            let role = atom(scratch, AFFECT, filler)?;
            slash(scratch, entry.shape == SHAPE_HEDGE, result, argument, role)
        }
        _ => Err(LexiconError::UnknownShape(position)),
    }
}

/// Comprehension from tokens: a token that is one of the lexicon's act markers sets the
/// frame's act (the last one wins; `speech_act` is the act when there is none), one that is
/// a particle sets the frame's marker and opens the particle slot, and every other token is
/// looked up and its category instantiated into `reading.categories` in order; the
/// categories are then read into a frame of `template` at `politeness` by [`comprehend`],
/// with the lexicon's sentence atom as the root's category. `UnknownToken` and
/// `UnknownShape` carry the token's position; `CategoriesFull` when the slice is too small;
/// the reducer's and the composition's reasons pass through. On a refusal the categories
/// instantiated before it stay on the arena and the counter has advanced past them.
pub fn comprehend_tokens(
    tokens: &[u32],
    lexicon: &Lexicon,
    scratch: &mut ParseScratch,
    reading: &mut Reading,
    template: u16,
    speech_act: u8,
    politeness: u8,
) -> Result<LinguisticFrameSlot, LexiconError> {
    if !lexicon.is_well_formed() {
        return Err(LexiconError::Malformed);
    }
    let mut act = speech_act;
    let mut marker = PROSODY_NONE;
    let mut count = 0usize;
    for (position, &token) in tokens.iter().enumerate() {
        if let Some(found) = lexicon.act_of(token) {
            act = found;
            continue;
        }
        if let Some(found) = lexicon.particle_of(token) {
            marker = found;
            continue;
        }
        let entry = lexicon
            .lookup(token)
            .ok_or(LexiconError::UnknownToken(position))?;
        let category = instantiate(
            entry,
            lexicon,
            scratch,
            &mut reading.next_variable,
            position,
        )?;
        let slot = reading
            .categories
            .get_mut(count)
            .ok_or(LexiconError::CategoriesFull)?;
        *slot = category;
        // Below the slice's length, checked above.
        count = count.wrapping_add(1);
    }
    let categories = reading
        .categories
        .get(..count)
        .ok_or(LexiconError::CategoriesFull)?;
    let mut frame = comprehend(
        categories,
        scratch,
        lexicon.sentence,
        template,
        act,
        politeness,
    )?;
    if marker != PROSODY_NONE {
        frame.prosody_tone_marker = marker;
        frame.syntax_gate_flags |= GATE_PARTICLE_OPEN;
    }
    Ok(frame)
}

/// One frame on the realisation's stack: the frame, its realisation order, the position
/// the walk is at.
#[derive(Clone, Copy, Default)]
struct Level {
    frame: u16,
    order: [u8; 5],
    at: usize,
}

/// One token into the output; a token of zero is nothing.
fn emit(
    out: &mut [u32],
    written: &mut usize,
    last: &mut u32,
    token: u32,
) -> Result<(), LexiconError> {
    if token == 0 {
        return Ok(());
    }
    let slot = out.get_mut(*written).ok_or(LexiconError::OutFull)?;
    *slot = token;
    *last = token;
    // Below the output's length, checked above.
    *written = written.wrapping_add(1);
    Ok(())
}

/// Realisation: the frame at `root` of the caller's arena `frames` into token ids in `out`,
/// and the number written. The act's opener first (`act_open[act]`, none for zero or an act
/// the table does not name); then the root's `realisation_order`, each role's concept as its
/// token at the frame's own politeness (`Lexicon::token_of`), and at `ROLE_CHILD` the frame
/// at `child_frame_idx` realised in place the same way, by an explicit stack of at most
/// [`MAX_NESTING`] frames; then the closer; then the particle of the root's marker when its
/// particle slot is open. The last token realised is written into the root's
/// `surface_token_id`. Refused, with the output partly written, for a frame outside the
/// arena (`NoSuchFrame`), a frame without a realisation order (`Incomplete`), a concept
/// without an entry (`NoToken`), a nesting past the bound or a child slot that returns to a
/// frame already realised (`TooDeep`), and an output too small (`OutFull`). A child clause's
/// markers and particle are not realised: they are the utterance's.
pub fn realise(
    frames: &mut [LinguisticFrameSlot],
    root: u16,
    lexicon: &Lexicon,
    out: &mut [u32],
) -> Result<usize, LexiconError> {
    if !lexicon.is_well_formed() {
        return Err(LexiconError::Malformed);
    }
    let top = *frames
        .get(root as usize)
        .ok_or(LexiconError::NoSuchFrame(root))?;
    let mut written = 0usize;
    let mut last = 0u32;
    if let Some(&opener) = lexicon.act_open.get(top.speech_act_type as usize) {
        emit(out, &mut written, &mut last, opener)?;
    }
    let mut stack = [Level::default(); MAX_NESTING];
    stack[0] = Level {
        frame: root,
        order: top
            .realisation_order()
            .ok_or(LexiconError::Incomplete(root))?,
        at: 0,
    };
    let mut depth = 1usize;
    let mut visited = 1usize;
    // Every step emits a token, descends or returns, and a frame is visited once unless the
    // arena holds a cycle, which the visit count refuses: the steps per frame bound the walk.
    let budget = frames.len().saturating_mul(STEPS_PER_FRAME);
    for _ in 0..budget {
        let Some(below) = depth.checked_sub(1) else {
            break;
        };
        let (frame_index, role) = {
            let Some(level) = stack.get_mut(below) else {
                break;
            };
            let role = level.order.get(level.at).copied().unwrap_or(0);
            if role != 0 {
                // At most five positions.
                level.at = level.at.wrapping_add(1);
            }
            (level.frame, role)
        };
        if role == 0 {
            depth = below;
            continue;
        }
        let frame = *frames
            .get(frame_index as usize)
            .ok_or(LexiconError::NoSuchFrame(frame_index))?;
        if role == ROLE_CHILD {
            let child = frame.child_frame_idx;
            let order = frames
                .get(child as usize)
                .ok_or(LexiconError::NoSuchFrame(child))?
                .realisation_order()
                .ok_or(LexiconError::Incomplete(child))?;
            visited = visited.wrapping_add(1);
            if visited > frames.len() || depth >= MAX_NESTING {
                return Err(LexiconError::TooDeep);
            }
            let Some(slot) = stack.get_mut(depth) else {
                return Err(LexiconError::TooDeep);
            };
            *slot = Level {
                frame: child,
                order,
                at: 0,
            };
            // Below the bound, checked above.
            depth = depth.wrapping_add(1);
        } else {
            let concept = concept_in(&frame, role).ok_or(LexiconError::Incomplete(frame_index))?;
            let token = lexicon
                .token_of(concept, frame.politeness_level)
                .ok_or(LexiconError::NoToken(concept))?;
            emit(out, &mut written, &mut last, token)?;
        }
    }
    if depth != 0 {
        return Err(LexiconError::TooDeep);
    }
    if let Some(&closer) = lexicon.act_close.get(top.speech_act_type as usize) {
        emit(out, &mut written, &mut last, closer)?;
    }
    if top.syntax_gate_flags & GATE_PARTICLE_OPEN != 0 {
        if let Some(&particle) = lexicon.particles.get(top.prosody_tone_marker as usize) {
            emit(out, &mut written, &mut last, particle)?;
        }
    }
    if let Some(frame) = frames.get_mut(root as usize) {
        frame.surface_token_id = last;
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cortex_linguistic::{
        PROSODY_SOFTEN, Q16_ONE, ROLE_ACTION, SPEECH_ACT_ASSERTIVE, SPEECH_ACT_DIRECTIVE,
        SPEECH_ACT_EXPRESSIVE, TEMPLATE_CAUSAL, TEMPLATE_CAUSATIVE, TEMPLATE_EPISTEMIC,
        TEMPLATE_STATE,
    };
    use cortex_reasoning::{Binding, ParseError, Reduction};

    const S: u32 = 0x1000;
    const NP: u32 = 0x1001;
    const N: u32 = 0x1002;
    /// Tokens 10 to 18: the, dog, cat, chased, slept, maybe, sadly, big, you (a pronoun, a
    /// bare noun phrase, formal 19); concepts 100 to 108, the determiner's and the
    /// adjective's included.
    const ENTRIES: [Entry; 9] = [
        entry(10, 100, SHAPE_DETERMINER, 0),
        entry(11, 101, SHAPE_NOUN, 0),
        entry(12, 102, SHAPE_NOUN, 0),
        entry(13, 103, SHAPE_TRANSITIVE, 0),
        entry(14, 104, SHAPE_INTRANSITIVE, 0),
        entry(15, 105, SHAPE_HEDGE, 0),
        entry(16, 106, SHAPE_TAG, 0),
        entry(17, 107, SHAPE_ADJECTIVE, 0),
        entry(18, 108, SHAPE_NOMINAL, 19),
    ];
    /// An opener for a directive (20), a closer for an expressive (21), particles 31 to 35.
    const OPEN: [u32; 4] = [0, 20, 0, 0];
    const CLOSE: [u32; 4] = [0, 0, 0, 21];
    const PARTICLES: [u32; 6] = [0, 31, 32, 33, 34, 35];

    const fn entry(token: u32, concept: u32, shape: u8, formal: u32) -> Entry {
        Entry {
            token,
            concept,
            shape,
            formal,
        }
    }

    fn lexicon() -> Lexicon<'static> {
        Lexicon {
            entries: &ENTRIES,
            sentence: S,
            np: NP,
            n: N,
            act_open: OPEN,
            act_close: CLOSE,
            particles: PARTICLES,
        }
    }

    struct Slices {
        arena: [TermNode; 128],
        bindings: [Binding; 64],
        trail: [u32; 64],
        stack: [u32; 64],
        parse: [u32; 8],
        steps: [Reduction; 8],
    }

    impl Slices {
        fn new() -> Self {
            Self {
                arena: [TermNode::default(); 128],
                bindings: [Binding::UNBOUND; 64],
                trail: [0; 64],
                stack: [0; 64],
                parse: [0; 8],
                steps: [Reduction::default(); 8],
            }
        }

        fn scratch(&mut self) -> ParseScratch<'_> {
            ParseScratch {
                arena: &mut self.arena,
                free: 0,
                bindings: &mut self.bindings,
                trail: &mut self.trail,
                trail_len: 0,
                stack: &mut self.stack,
                parse: &mut self.parse,
                steps: &mut self.steps,
                step_count: 0,
            }
        }
    }

    /// Reads `tokens` into a frame of `template` over fresh slices.
    fn read(tokens: &[u32], template: u16) -> Result<LinguisticFrameSlot, LexiconError> {
        let mut slices = Slices::new();
        let mut categories = [0u32; 8];
        let mut reading = Reading {
            categories: &mut categories,
            next_variable: 0,
        };
        let mut scratch = slices.scratch();
        comprehend_tokens(
            tokens,
            &lexicon(),
            &mut scratch,
            &mut reading,
            template,
            SPEECH_ACT_ASSERTIVE,
            0,
        )
    }

    fn roles(frame: &LinguisticFrameSlot) -> [Option<u32>; 4] {
        [
            concept_in(frame, ROLE_SUBJECT),
            concept_in(frame, ROLE_ACTION),
            concept_in(frame, ROLE_OBJECT),
            concept_in(frame, ROLE_AFFECT),
        ]
    }

    #[test]
    fn the_table_is_well_formed_only_when_sorted_shaped_and_its_markers_are_apart() {
        let lex = lexicon();
        assert!(lex.is_well_formed());
        let mut unsorted = ENTRIES;
        unsorted.swap(0, 1);
        assert!(
            !Lexicon {
                entries: &unsorted,
                ..lex
            }
            .is_well_formed()
        );
        let mut twice = ENTRIES;
        twice[1].token = 10;
        assert!(
            !Lexicon {
                entries: &twice,
                ..lex
            }
            .is_well_formed(),
            "a token twice"
        );
        let mut zero = ENTRIES;
        zero[0].token = 0;
        assert!(
            !Lexicon {
                entries: &zero,
                ..lex
            }
            .is_well_formed(),
            "token zero"
        );
        for shape in [0u8, 9, u8::MAX] {
            let mut shaped = ENTRIES;
            shaped[3].shape = shape;
            assert!(
                !Lexicon {
                    entries: &shaped,
                    ..lex
                }
                .is_well_formed(),
                "shape {shape}"
            );
        }
        assert!(
            !Lexicon { np: S, ..lex }.is_well_formed(),
            "two atoms alike"
        );
        assert!(!Lexicon { n: NP, ..lex }.is_well_formed());
        assert!(!Lexicon { n: S, ..lex }.is_well_formed());
        assert!(
            !Lexicon {
                sentence: ROLE_CONCEPT_BASE,
                ..lex
            }
            .is_well_formed(),
            "an atom in the band"
        );
        assert!(
            !Lexicon {
                act_open: [0, 11, 0, 0],
                ..lex
            }
            .is_well_formed(),
            "a marker that is a word"
        );
        assert!(
            !Lexicon {
                act_close: [0, 20, 0, 0],
                ..lex
            }
            .is_well_formed(),
            "a marker twice"
        );
        assert!(
            !Lexicon {
                particles: [0, 31, 31, 0, 0, 0],
                ..lex
            }
            .is_well_formed()
        );
        assert!(
            !Lexicon {
                particles: [7, 0, 0, 0, 0, 0],
                ..lex
            }
            .is_well_formed(),
            "no particle for no marker"
        );
        assert!(
            Lexicon {
                act_open: [0; 4],
                act_close: [0; 4],
                particles: [0; 6],
                ..lex
            }
            .is_well_formed(),
            "no markers at all"
        );
        assert!(
            Lexicon {
                entries: &[],
                ..lex
            }
            .is_well_formed(),
            "an empty table"
        );
    }

    #[test]
    fn lookup_and_the_reverse_lookup_with_the_formal_variant() {
        let lex = lexicon();
        assert_eq!(lex.lookup(13).map(|e| e.concept), Some(103));
        assert_eq!(lex.lookup(10).map(|e| e.shape), Some(SHAPE_DETERMINER));
        assert_eq!(lex.lookup(9), None);
        assert_eq!(lex.lookup(19), None, "a formal variant is no entry");
        assert_eq!(lex.lookup(0), None);
        assert_eq!(lex.token_of(101, 0), Some(11));
        assert_eq!(lex.token_of(108, 1), Some(18), "below the formal register");
        assert_eq!(lex.token_of(108, POLITENESS_FORMAL), Some(19), "at it");
        assert_eq!(lex.token_of(108, u8::MAX), Some(19));
        assert_eq!(
            lex.token_of(101, u8::MAX),
            Some(11),
            "no variant: the token"
        );
        assert_eq!(lex.token_of(99, 0), None);
        let twice = [entry(5, 7, SHAPE_NOUN, 0), entry(6, 7, SHAPE_NOUN, 0)];
        let lex = Lexicon {
            entries: &twice,
            ..lex
        };
        assert_eq!(lex.token_of(7, 0), Some(5), "the first by token");
        assert_eq!(lex.act_of(20), Some(SPEECH_ACT_DIRECTIVE));
        assert_eq!(lex.act_of(21), Some(SPEECH_ACT_EXPRESSIVE));
        assert_eq!(lex.act_of(0), None);
        assert_eq!(lex.act_of(31), None);
        assert_eq!(lex.particle_of(31), Some(PROSODY_SOFTEN));
        assert_eq!(lex.particle_of(35), Some(5));
        assert_eq!(lex.particle_of(0), None, "zero is never a marker");
        assert_eq!(lex.particle_of(20), None);
    }

    #[test]
    fn every_shape_instantiates_its_category_and_a_full_arena_puts_the_cursor_back() {
        let lex = lexicon();
        let mut slices = Slices::new();
        let mut scratch = slices.scratch();
        let mut next = 0u32;
        // In token order: the determiner, two nouns, the transitive and the intransitive
        // verb, the hedge, the tag, the adjective, the pronoun; the nodes and the variables
        // each takes.
        let nodes = [5usize, 2, 2, 10, 6, 6, 6, 5, 2];
        let vars = [1u32, 0, 0, 2, 1, 1, 1, 1, 0];
        let mut roots = [0u32; 9];
        for (i, entry) in ENTRIES.iter().enumerate() {
            let free = scratch.free;
            let counter = next;
            roots[i] = instantiate(entry, &lex, &mut scratch, &mut next, i).unwrap();
            assert_eq!(scratch.free.wrapping_sub(free), nodes[i], "{i}");
            assert_eq!(next.wrapping_sub(counter), vars[i], "{i}");
            assert_eq!(
                roots[i] as usize,
                scratch.free.wrapping_sub(1),
                "the root is last"
            );
        }
        // The transitive verb's root: a forward slash whose result is a backward slash.
        let root = scratch.arena[roots[3] as usize];
        assert_eq!(root.functor, cortex_reasoning::CATEGORY_FORWARD);
        let inner = scratch.arena[root.child(0).unwrap() as usize];
        assert_eq!(inner.functor, cortex_reasoning::CATEGORY_BACKWARD);
        assert_eq!(
            scratch.arena[inner.child(2).unwrap() as usize].functor,
            SUBJECT
        );
        assert_eq!(
            scratch.arena[root.child(2).unwrap() as usize].functor,
            OBJECT
        );
        // The hedge's role term: `AFFECT(105)`; the tag's slash is backward.
        let hedge = scratch.arena[roots[5] as usize];
        assert_eq!(hedge.functor, cortex_reasoning::CATEGORY_FORWARD);
        let role = scratch.arena[hedge.child(2).unwrap() as usize];
        assert_eq!((role.functor, role.arity), (AFFECT, 1));
        assert_eq!(scratch.arena[role.child(0).unwrap() as usize].functor, 105);
        let tag = scratch.arena[roots[6] as usize];
        assert_eq!(tag.functor, cortex_reasoning::CATEGORY_BACKWARD);
        assert_eq!(
            scratch.arena[tag.child(2).unwrap() as usize].functor,
            AFFECT
        );
        // The determiner's and the adjective's role terms name no role.
        for i in [0usize, 7] {
            let node = scratch.arena[roots[i] as usize];
            let role = scratch.arena[node.child(2).unwrap() as usize];
            assert_eq!(role.functor, ROLE_CONCEPT_BASE);
            assert_eq!(crate::language::role_of_concept(role.functor), None);
        }
        // A noun is `N(c)`, a nominal `NP(c)`.
        assert_eq!(scratch.arena[roots[1] as usize].functor, N);
        assert_eq!(scratch.arena[roots[8] as usize].functor, NP);
        // An unknown shape names the position; a full arena puts the cursor and the counter
        // back.
        let odd = entry(1, 1, 9, 0);
        assert_eq!(
            instantiate(&odd, &lex, &mut scratch, &mut next, 4),
            Err(LexiconError::UnknownShape(4))
        );
        let (free, counter) = (scratch.free, next);
        scratch.free = 125;
        assert_eq!(
            instantiate(&ENTRIES[3], &lex, &mut scratch, &mut next, 0),
            Err(LexiconError::ArenaFull),
            "three nodes for ten"
        );
        assert_eq!((scratch.free, next), (125, counter));
        scratch.free = free;
        assert!(instantiate(&ENTRIES[3], &lex, &mut scratch, &mut next, 0).is_ok());
    }

    #[test]
    fn tokens_are_read_into_a_frame_with_the_markers_taken_out() {
        // "the dog chased the cat": dog, chase, cat.
        let frame = read(&[10, 11, 13, 10, 12], TEMPLATE_CAUSATIVE).unwrap();
        assert_eq!(roles(&frame), [Some(101), Some(103), Some(102), None]);
        assert_eq!(frame.speech_act_type, SPEECH_ACT_ASSERTIVE);
        assert_eq!(frame.prosody_tone_marker, PROSODY_NONE);
        assert!(frame.is_complete());
        // The opener sets the act, the particle the marker, and neither is a category.
        let frame = read(&[20, 10, 11, 14, 31], TEMPLATE_STATE).unwrap();
        assert_eq!(roles(&frame), [Some(101), Some(104), None, None]);
        assert_eq!(frame.speech_act_type, SPEECH_ACT_DIRECTIVE);
        assert_eq!(frame.prosody_tone_marker, PROSODY_SOFTEN);
        assert_ne!(frame.syntax_gate_flags & GATE_PARTICLE_OPEN, 0);
        let frame = read(&[10, 11, 14, 21], TEMPLATE_STATE).unwrap();
        assert_eq!(frame.speech_act_type, SPEECH_ACT_EXPRESSIVE, "a closer too");
        assert_eq!(frame.syntax_gate_flags & GATE_PARTICLE_OPEN, 0);
        // A hedge fills the affect role with its own concept; a tag likewise.
        let frame = read(&[15, 10, 11, 13, 10, 12], TEMPLATE_EPISTEMIC).unwrap();
        assert_eq!(roles(&frame), [Some(101), Some(103), Some(102), Some(105)]);
        assert!(frame.is_complete());
        let frame = read(&[10, 11, 14, 16], TEMPLATE_STATE).unwrap();
        assert_eq!(roles(&frame), [Some(101), Some(104), None, Some(106)]);
        // An adjective passes the head up and reaches no slot.
        let frame = read(&[10, 17, 11, 14], TEMPLATE_STATE).unwrap();
        assert_eq!(roles(&frame), [Some(101), Some(104), None, None]);
        // A nominal is a noun phrase on its own; a noun is not.
        let frame = read(&[18, 14], TEMPLATE_STATE).unwrap();
        assert_eq!(roles(&frame), [Some(108), Some(104), None, None]);
        assert!(matches!(
            read(&[11, 14], TEMPLATE_STATE),
            Err(LexiconError::Language(LanguageError::Parse(
                ParseError::NoDerivation { remaining: 2 }
            )))
        ));
        // The refusals: an unknown token at its position, an unknown shape, a slice too
        // small, the reducer's reason, markers alone.
        assert_eq!(
            read(&[10, 11, 99], TEMPLATE_STATE),
            Err(LexiconError::UnknownToken(2))
        );
        assert_eq!(
            read(&[0], TEMPLATE_STATE),
            Err(LexiconError::UnknownToken(0))
        );
        assert_eq!(
            read(&[11, 12], TEMPLATE_STATE),
            Err(LexiconError::Language(LanguageError::Parse(
                ParseError::NoDerivation { remaining: 2 }
            ))),
            "two nouns"
        );
        assert_eq!(
            read(&[20, 31], TEMPLATE_STATE),
            Err(LexiconError::Language(LanguageError::Parse(
                ParseError::Empty
            )))
        );
        assert!(matches!(
            read(&[10, 11], TEMPLATE_STATE),
            Err(LexiconError::Language(LanguageError::NotASentence(_)))
        ));
        let mut slices = Slices::new();
        let mut one = [0u32; 1];
        let mut reading = Reading {
            categories: &mut one,
            next_variable: 0,
        };
        let mut scratch = slices.scratch();
        assert_eq!(
            comprehend_tokens(
                &[11, 14],
                &lexicon(),
                &mut scratch,
                &mut reading,
                TEMPLATE_STATE,
                0,
                0
            ),
            Err(LexiconError::CategoriesFull)
        );
        assert_eq!(reading.next_variable, 1, "the verb's variable was taken");
        let mut odd = ENTRIES;
        odd[4].shape = 9;
        let lex = Lexicon {
            entries: &odd,
            ..lexicon()
        };
        assert!(!lex.is_well_formed());
        let mut categories = [0u32; 8];
        let mut reading = Reading {
            categories: &mut categories,
            next_variable: 0,
        };
        assert_eq!(
            comprehend_tokens(
                &[11, 14],
                &lex,
                &mut scratch,
                &mut reading,
                TEMPLATE_STATE,
                0,
                0
            ),
            Err(LexiconError::Malformed)
        );
        let mut frames = [LinguisticFrameSlot::default(); 1];
        let mut out = [0u32; 4];
        assert_eq!(
            realise(&mut frames, 0, &lex, &mut out),
            Err(LexiconError::Malformed)
        );
        assert_eq!(
            LexiconError::from(LanguageError::RoleRefused(1)),
            LexiconError::Language(LanguageError::RoleRefused(1))
        );
    }

    /// A causal frame `dog chased cat [because] the child`, the child a state frame `cat
    /// slept`, at the frames' own politeness.
    fn causal(politeness: u8) -> [LinguisticFrameSlot; 3] {
        let mut root = LinguisticFrameSlot::new(TEMPLATE_CAUSAL, SPEECH_ACT_ASSERTIVE, politeness);
        assert!(root.bind_role(ROLE_SUBJECT, 101, Q16_ONE));
        assert!(root.bind_role(ROLE_ACTION, 103, Q16_ONE));
        assert!(root.bind_role(ROLE_OBJECT, 102, Q16_ONE));
        let mut child = LinguisticFrameSlot::new(TEMPLATE_STATE, SPEECH_ACT_ASSERTIVE, politeness);
        assert!(child.bind_role(ROLE_SUBJECT, 108, Q16_ONE));
        assert!(child.bind_role(ROLE_ACTION, 104, Q16_ONE));
        assert!(root.bind_child(2, 0));
        assert!(child.set_parent(0));
        [root, LinguisticFrameSlot::default(), child]
    }

    #[test]
    fn a_frame_is_realised_in_its_order_with_its_child_its_markers_and_its_particle() {
        let lex = lexicon();
        let mut frames = causal(0);
        let mut out = [0u32; 8];
        assert_eq!(realise(&mut frames, 0, &lex, &mut out), Ok(5));
        assert_eq!(
            &out[..5],
            &[11, 13, 12, 18, 14],
            "dog chased cat, you slept"
        );
        assert_eq!(frames[0].surface_token_id, 14, "the last token realised");
        assert_eq!(frames[2].surface_token_id, 0, "the child is not the root");
        // The formal register: the child's own politeness picks the variant.
        let mut formal = causal(POLITENESS_FORMAL);
        assert_eq!(realise(&mut formal, 0, &lex, &mut out), Ok(5));
        assert_eq!(&out[..5], &[11, 13, 12, 19, 14]);
        // A directive with the affect tag and a particle: opener, roles, tag, particle.
        let mut asked = LinguisticFrameSlot::new(TEMPLATE_STATE, SPEECH_ACT_DIRECTIVE, 1);
        assert!(asked.bind_role(ROLE_SUBJECT, 108, Q16_ONE));
        assert!(asked.bind_role(ROLE_ACTION, 104, Q16_ONE));
        assert!(asked.bind_role(ROLE_AFFECT, 106, Q16_ONE));
        asked.prosody_tone_marker = PROSODY_SOFTEN;
        asked.syntax_gate_flags |= GATE_PARTICLE_OPEN;
        let mut frames = [asked];
        assert_eq!(realise(&mut frames, 0, &lex, &mut out), Ok(5));
        assert_eq!(&out[..5], &[20, 18, 14, 16, 31]);
        assert_eq!(frames[0].surface_token_id, 31);
        // The slot closed: no particle; a marker without a particle: none; an act the table
        // does not name: no marker.
        frames[0].syntax_gate_flags &= !GATE_PARTICLE_OPEN;
        assert_eq!(realise(&mut frames, 0, &lex, &mut out), Ok(4));
        assert_eq!(frames[0].surface_token_id, 16);
        frames[0].syntax_gate_flags |= GATE_PARTICLE_OPEN;
        frames[0].prosody_tone_marker = 9;
        assert_eq!(realise(&mut frames, 0, &lex, &mut out), Ok(4));
        frames[0].prosody_tone_marker = PROSODY_SOFTEN;
        frames[0].speech_act_type = 7;
        assert_eq!(realise(&mut frames, 0, &lex, &mut out), Ok(4));
        assert_eq!(&out[..4], &[18, 14, 16, 31]);
        // The expressive closer.
        frames[0].speech_act_type = SPEECH_ACT_EXPRESSIVE;
        assert_eq!(realise(&mut frames, 0, &lex, &mut out), Ok(5));
        assert_eq!(&out[..5], &[18, 14, 16, 21, 31]);
        // An epistemic frame realises its hedge first.
        let mut hedged = LinguisticFrameSlot::new(TEMPLATE_EPISTEMIC, SPEECH_ACT_ASSERTIVE, 0);
        for (role, concept) in [
            (ROLE_SUBJECT, 101),
            (ROLE_ACTION, 103),
            (ROLE_OBJECT, 102),
            (ROLE_AFFECT, 105),
        ] {
            assert!(hedged.bind_role(role, concept, Q16_ONE));
        }
        let mut frames = [hedged];
        assert_eq!(realise(&mut frames, 0, &lex, &mut out), Ok(4));
        assert_eq!(&out[..4], &[15, 11, 13, 12]);
    }

    #[test]
    fn realisation_refuses_a_missing_frame_an_incomplete_one_a_concept_without_a_word_a_small_output_and_a_deep_or_cyclic_nesting()
     {
        let lex = lexicon();
        let mut out = [0u32; 8];
        let mut frames = causal(0);
        assert_eq!(
            realise(&mut frames, 3, &lex, &mut out),
            Err(LexiconError::NoSuchFrame(3))
        );
        assert_eq!(
            realise(&mut frames, 1, &lex, &mut out),
            Err(LexiconError::Incomplete(1)),
            "the empty frame"
        );
        frames[0].child_frame_idx = 9;
        assert_eq!(
            realise(&mut frames, 0, &lex, &mut out),
            Err(LexiconError::NoSuchFrame(9))
        );
        frames[0].child_frame_idx = 1;
        assert_eq!(
            realise(&mut frames, 0, &lex, &mut out),
            Err(LexiconError::Incomplete(1)),
            "a child with no order"
        );
        let mut frames = causal(0);
        frames[2].subject_concept_id = 999;
        assert_eq!(
            realise(&mut frames, 0, &lex, &mut out),
            Err(LexiconError::NoToken(999))
        );
        assert_eq!(&out[..3], &[11, 13, 12], "written up to the refusal");
        let mut frames = causal(0);
        let mut four = [0u32; 4];
        assert_eq!(
            realise(&mut frames, 0, &lex, &mut four),
            Err(LexiconError::OutFull)
        );
        let mut five = [0u32; 5];
        assert_eq!(
            realise(&mut frames, 0, &lex, &mut five),
            Ok(5),
            "exactly enough"
        );
        // A cycle of three relative frames: each nests the next, the last the first.
        let mut cycle: [LinguisticFrameSlot; 3] = core::array::from_fn(|i| {
            let mut f = LinguisticFrameSlot::new(
                cortex_linguistic::TEMPLATE_RELATIVE,
                SPEECH_ACT_ASSERTIVE,
                0,
            );
            assert!(f.bind_role(ROLE_SUBJECT, 101, Q16_ONE));
            assert!(f.bind_role(ROLE_ACTION, 104, Q16_ONE));
            let next = (i as u16).wrapping_add(1).wrapping_rem(3);
            assert!(f.bind_child(next, i as u16));
            f
        });
        let mut wide = [0u32; 32];
        assert_eq!(
            realise(&mut cycle, 0, &lex, &mut wide),
            Err(LexiconError::TooDeep)
        );
        // A chain of nine relative frames, the last a state frame: eight levels is the bound.
        let chain = |length: usize| -> [LinguisticFrameSlot; 10] {
            core::array::from_fn(|i| {
                let last = i.wrapping_add(1) >= length;
                let template = if last {
                    TEMPLATE_STATE
                } else {
                    cortex_linguistic::TEMPLATE_RELATIVE
                };
                let mut f = LinguisticFrameSlot::new(template, SPEECH_ACT_ASSERTIVE, 0);
                assert!(f.bind_role(ROLE_SUBJECT, 101, Q16_ONE));
                assert!(f.bind_role(ROLE_ACTION, 104, Q16_ONE));
                if !last {
                    assert!(f.bind_child((i as u16).wrapping_add(1), i as u16));
                }
                f
            })
        };
        let mut eight = chain(8);
        assert_eq!(
            realise(&mut eight, 0, &lex, &mut wide),
            Ok(16),
            "eight frames nest"
        );
        let mut nine = chain(9);
        assert_eq!(
            realise(&mut nine, 0, &lex, &mut wide),
            Err(LexiconError::TooDeep),
            "the ninth is past the bound"
        );
        assert_eq!(MAX_NESTING, 8);
    }
}
