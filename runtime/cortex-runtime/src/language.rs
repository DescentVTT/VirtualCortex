//! The language composition (whitepaper §5.2.20, §6.9; ADR-0039, ADR-0040; brief 020): the
//! one place the three language crates meet, between ticks, with no executor field. In the
//! comprehension direction a sequence of lexical categories is reduced on the term arena
//! (`cortex-reasoning`), the roles its derivation reports are bound into a
//! `LinguisticFrameSlot` with the head concept of each argument (`cortex-linguistic`, Layer
//! 2), and a sealed frame is encoded as the bundle of each bound role's body bound to its
//! concept's body (`cortex-symbolic`). In the reading direction a role is unbound from a sealed
//! body and resolved to the nearest codebook entry with a confidence (Layer 1), and a frame is
//! decoded role by role. Words never appear: a concept is a codebook index, a role a
//! `ROLE_*` bit, and the lexicon that turns either into a token is the caller's (§1.5).
//! Nothing here allocates.

use cortex_linguistic::{
    LinguisticFrameSlot, Q16_ONE, ROLE_ACTION, ROLE_AFFECT, ROLE_OBJECT, ROLE_SUBJECT,
};
use cortex_reasoning::{
    Binding, ParseError, ParseScratch, TERM_CONSTANT, TermNode, deref, head, reduce,
};
use cortex_symbolic::{HypervectorBody, confidence_q16};

/// The band of concept ids the grammar's role constants take: `ROLE_CONCEPT_BASE | role_bit`,
/// above every codebook index and below the slash functors of `cortex-reasoning`.
pub const ROLE_CONCEPT_BASE: u32 = 0xFFFF_FE00;
/// The confidence below which a readout binds nothing: 0.125, a distance above 0.4375 of the
/// width. A bound role in a bundle of three reads at about a quarter (0.5); an unbound one at
/// about half (0), forty standard deviations of the noise below the floor.
pub const DECODE_FLOOR_Q16: u32 = Q16_ONE / 8;
/// The four roles in slot order: the index of a role's body in a caller's `[HypervectorBody; 4]`.
pub const ROLES: [u8; 4] = [ROLE_SUBJECT, ROLE_ACTION, ROLE_OBJECT, ROLE_AFFECT];

/// Why a sequence did not become a frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanguageError {
    /// The reducer's reason.
    Parse(ParseError),
    /// The frame refused a role's binding (an affect concept wider than sixteen bits).
    RoleRefused(u8),
}

impl From<ParseError> for LanguageError {
    fn from(error: ParseError) -> Self {
        Self::Parse(error)
    }
}

/// The concept id of a role constant in a category, or `None` for a value that is not exactly
/// one `ROLE_*` bit.
pub const fn role_concept(role: u8) -> Option<u32> {
    if role_slot(role).is_some() {
        Some(ROLE_CONCEPT_BASE | role as u32)
    } else {
        None
    }
}

/// The `ROLE_*` bit a role constant's concept id names, or `None` outside the band.
pub const fn role_of_concept(id: u32) -> Option<u8> {
    if id & !0xF == ROLE_CONCEPT_BASE {
        let role = (id & 0xF) as u8;
        if role_slot(role).is_some() {
            return Some(role);
        }
    }
    None
}

/// The slot of a role in `ROLES`, or `None` for a value that is not exactly one role bit.
pub const fn role_slot(role: u8) -> Option<usize> {
    match role {
        ROLE_SUBJECT => Some(0),
        ROLE_ACTION => Some(1),
        ROLE_OBJECT => Some(2),
        ROLE_AFFECT => Some(3),
        _ => None,
    }
}

/// The role a reduction's role term names under the bindings: a constant in the band.
fn role_of_term(term: u32, arena: &[TermNode], bindings: &[Binding]) -> Option<u8> {
    let index = deref(term, arena, bindings)?;
    let node = arena.get(index as usize)?;
    if node.kind == TERM_CONSTANT {
        role_of_concept(node.functor)
    } else {
        None
    }
}

/// Comprehension: reduces `categories` (the term indices of the lexical categories, in order,
/// each instantiated with fresh variables by the caller) and binds into a new frame of
/// `template` every role a reduction reports, with the head concept of the category that filled
/// it, and the head of the root category as the action, each at a confidence of 1.0. The frame
/// is returned unsealed; a reduction whose role is not a role constant, or whose argument has
/// no head, binds nothing. The reductions and the substitution are left in `scratch`.
pub fn comprehend(
    categories: &[u32],
    scratch: &mut ParseScratch,
    template: u16,
    speech_act: u8,
    politeness: u8,
) -> Result<LinguisticFrameSlot, LanguageError> {
    let root = reduce(categories, scratch)?;
    let mut frame = LinguisticFrameSlot::new(template, speech_act, politeness);
    let arena = &*scratch.arena;
    let bindings = &*scratch.bindings;
    for step in scratch.steps.iter().take(scratch.step_count) {
        let Some(role) = role_of_term(step.role, arena, bindings) else {
            continue;
        };
        let Some(concept) = head(step.argument, arena, bindings) else {
            continue;
        };
        if !frame.bind_role(role, concept, Q16_ONE) {
            return Err(LanguageError::RoleRefused(role));
        }
    }
    // Nested rather than a `let` chain: the chain is stable from Rust 1.88, the floor is 1.85.
    if let Some(action) = head(root, arena, bindings) {
        if !frame.bind_role(ROLE_ACTION, action, Q16_ONE) {
            return Err(LanguageError::RoleRefused(ROLE_ACTION));
        }
    }
    Ok(frame)
}

/// The concept bound in a role of the frame, or `None` when the role is not bound.
pub const fn concept_in(frame: &LinguisticFrameSlot, role: u8) -> Option<u32> {
    if frame.filled_roles() & role == 0 {
        return None;
    }
    match role {
        ROLE_SUBJECT => Some(frame.subject_concept_id),
        ROLE_ACTION => Some(frame.action_predicate_id),
        ROLE_OBJECT => Some(frame.object_concept_id),
        ROLE_AFFECT => Some(frame.affect_modifier_id as u32),
        _ => None,
    }
}

/// Encoding: the bundle, over the frame's bound roles in slot order, of each role's body bound
/// to its concept's body in `book` (the concept id is the codebook index). `None` when no role
/// is bound or a concept is outside the book.
pub fn encode_frame(
    frame: &LinguisticFrameSlot,
    roles: &[HypervectorBody; 4],
    book: &[HypervectorBody],
) -> Option<HypervectorBody> {
    let mut items = [HypervectorBody::ZERO; 4];
    let mut count = 0usize;
    for (slot, &role) in ROLES.iter().enumerate() {
        let Some(concept) = concept_in(frame, role) else {
            continue;
        };
        let body = book.get(concept as usize)?;
        items[count] = roles[slot].bind(body);
        // At most four roles.
        count = count.wrapping_add(1);
    }
    HypervectorBody::bundle(&items[..count])
}

/// Layer 1: the concept a role was bound with in `sealed`, read as the nearest codebook entry
/// of the unbound body: the index, the distance and the confidence. `None` for an empty book.
pub fn read_role(
    sealed: &HypervectorBody,
    role: &HypervectorBody,
    book: &[HypervectorBody],
) -> Option<(usize, u32, u32)> {
    let (index, distance) = sealed.bind(role).nearest(book)?;
    Some((index, distance, confidence_q16(distance)))
}

/// Decoding: a new frame of `template` with every role whose readout reaches
/// `DECODE_FLOOR_Q16` bound to the concept read, at that confidence, in slot order; a role
/// below the floor, or whose concept the frame cannot hold, is left unbound. The frame's
/// confidence is the weakest readout bound (`bind_role`'s rule).
pub fn decode_frame(
    sealed: &HypervectorBody,
    roles: &[HypervectorBody; 4],
    book: &[HypervectorBody],
    template: u16,
    speech_act: u8,
    politeness: u8,
) -> LinguisticFrameSlot {
    let mut frame = LinguisticFrameSlot::new(template, speech_act, politeness);
    for (slot, &role) in ROLES.iter().enumerate() {
        if let Some((index, _, confidence)) = read_role(sealed, &roles[slot], book) {
            if let Ok(concept) = u32::try_from(index) {
                if confidence >= DECODE_FLOOR_Q16 {
                    // Refused only for an affect concept wider than sixteen bits, which is
                    // left unbound, as the doc says.
                    let _ = frame.bind_role(role, concept, confidence);
                }
            }
        }
    }
    frame
}

const _: () = {
    // The role band sits above any codebook index a frame holds and below the slash functors.
    assert!(ROLE_CONCEPT_BASE > u16::MAX as u32);
    assert!(ROLE_CONCEPT_BASE < cortex_reasoning::CATEGORY_RESERVED);
    assert!(DECODE_FLOOR_Q16 < Q16_ONE);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_concepts_are_one_bit_in_the_band_and_nothing_else() {
        for (slot, &role) in ROLES.iter().enumerate() {
            let id = role_concept(role).unwrap();
            assert_eq!(id & !0xF, ROLE_CONCEPT_BASE);
            assert_eq!(role_of_concept(id), Some(role));
            assert_eq!(role_slot(role), Some(slot));
        }
        assert_eq!(role_concept(0), None);
        assert_eq!(
            role_concept(ROLE_SUBJECT | ROLE_OBJECT),
            None,
            "one role at a time"
        );
        assert_eq!(role_concept(0x10), None, "not a role bit");
        assert_eq!(role_of_concept(ROLE_CONCEPT_BASE), None, "no bit");
        assert_eq!(role_of_concept(ROLE_CONCEPT_BASE | 3), None, "two bits");
        assert_eq!(
            role_of_concept(ROLE_CONCEPT_BASE | 0x10),
            None,
            "outside the nibble"
        );
        assert_eq!(role_of_concept(1), None, "a codebook index");
        assert_eq!(
            role_of_concept(cortex_reasoning::CATEGORY_FORWARD),
            None,
            "a slash"
        );
        assert_eq!(role_slot(0), None);
        assert_eq!(role_slot(ROLE_AFFECT | ROLE_ACTION), None);
        assert_eq!(DECODE_FLOOR_Q16, 0x2000);
    }

    #[test]
    fn a_frame_encodes_over_its_bound_roles_only_and_refuses_a_concept_outside_the_book() {
        let book: [HypervectorBody; 4] =
            core::array::from_fn(|i| HypervectorBody::from_seed(50u64.wrapping_add(i as u64)));
        let roles: [HypervectorBody; 4] =
            core::array::from_fn(|i| HypervectorBody::from_seed(60u64.wrapping_add(i as u64)));
        let empty = LinguisticFrameSlot::new(0, 0, 0);
        assert_eq!(encode_frame(&empty, &roles, &book), None, "no role bound");
        let mut one = LinguisticFrameSlot::new(0, 0, 0);
        assert!(one.bind_role(ROLE_OBJECT, 2, Q16_ONE));
        assert_eq!(
            encode_frame(&one, &roles, &book),
            Some(roles[2].bind(&book[2])),
            "one role: the bound pair itself"
        );
        assert_eq!(concept_in(&one, ROLE_OBJECT), Some(2));
        assert_eq!(concept_in(&one, ROLE_SUBJECT), None);
        assert_eq!(concept_in(&one, 0x10), None);
        let mut outside = LinguisticFrameSlot::new(0, 0, 0);
        assert!(outside.bind_role(ROLE_SUBJECT, 4, Q16_ONE));
        assert_eq!(
            encode_frame(&outside, &roles, &book),
            None,
            "index 4 in a book of four"
        );
        let mut two = LinguisticFrameSlot::new(0, 0, 0);
        assert!(two.bind_role(ROLE_ACTION, 1, Q16_ONE) && two.bind_role(ROLE_SUBJECT, 0, Q16_ONE));
        let expected =
            HypervectorBody::bundle(&[roles[0].bind(&book[0]), roles[1].bind(&book[1])]).unwrap();
        assert_eq!(
            encode_frame(&two, &roles, &book),
            Some(expected),
            "in slot order"
        );
        assert_eq!(read_role(&expected, &roles[0], &[]), None, "an empty book");
        let (index, distance, confidence) = read_role(&expected, &roles[0], &book).unwrap();
        assert_eq!(index, 0);
        assert_eq!(confidence, confidence_q16(distance));
        let decoded = decode_frame(&expected, &roles, &book, 0, 0, 0);
        assert_eq!(
            (
                concept_in(&decoded, ROLE_SUBJECT),
                concept_in(&decoded, ROLE_ACTION)
            ),
            (Some(0), Some(1))
        );
        assert_eq!(
            decoded.filled_roles(),
            ROLE_SUBJECT | ROLE_ACTION,
            "the others read as noise"
        );
        assert_eq!(
            decoded.confidence_q16,
            confidence.min(read_role(&expected, &roles[1], &book).unwrap().2)
        );
    }
}
