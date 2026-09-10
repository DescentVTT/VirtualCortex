//! Linguistic frame slots: the record of the engine's native, three-layer language pipeline
//! (whitepaper §5.2.20, §6.9, §8.8; admitted by ADR-0016).
//!
//! - **Layer 1, semantic grounding.** `cortex-symbolic` unbinds a hypervector by role,
//!   $\text{Concept} \approx S \otimes \text{Role}^{-1}$, into a subject, an action, an object and
//!   an affect, each with a confidence (Specified there).
//! - **Layer 2, syntactic framing.** This crate: construction-grammar templates with strict role
//!   slots. A frame is complete when the roles its template requires are bound, and the template
//!   fixes the order in which a lexicon emits the roles (Implemented).
//! - **Layer 3, temporal flow and prosody.** A linear recurrent cell in saturating Q16.16,
//!   $s_{t+1} = \alpha\, s_t + k_t v_t$, with constant memory: the record carries the cell's
//!   scalar energy and a running hash of its trajectory, and the energy's band selects the
//!   prosody marker (a particle class) that fills the frame's particle slot (Implemented for the
//!   scalar; a fixed-dimension state vector in an arena is Specified).
//!
//! Self-contained by construction: no external language model, no transformer runtime, no heap.
//! The lexicon that turns concept ids, the politeness level and the marker into Chinese or
//! English tokens is the runtime's (Specified).

#![no_std]

/// 1.0 in Q16.16.
pub const Q16_ONE: u32 = 0x0001_0000;

/// Roles, as bits of `filled_roles` and as entries of a realisation order.
pub const ROLE_SUBJECT: u8 = 0x01;
pub const ROLE_ACTION: u8 = 0x02;
pub const ROLE_OBJECT: u8 = 0x04;
pub const ROLE_AFFECT: u8 = 0x08;

/// Templates. Each fixes the roles it requires and the order it is realised in.
/// "subject is in a state": subject + action.
pub const TEMPLATE_STATE: u16 = 0;
/// "do action to object", addressed to the listener: action + object.
pub const TEMPLATE_REQUEST: u16 = 1;
/// "subject needs object": subject + object.
pub const TEMPLATE_NEED: u16 = 2;
/// "subject makes object undergo action": subject + action + object.
pub const TEMPLATE_CAUSATIVE: u16 = 3;
/// "subject holds that object, with a hedge": subject + action + object + affect, hedge first.
pub const TEMPLATE_EPISTEMIC: u16 = 4;

/// Speech acts: the illocutionary force the lexicon marks the utterance with.
pub const SPEECH_ACT_ASSERTIVE: u8 = 0;
pub const SPEECH_ACT_DIRECTIVE: u8 = 1;
pub const SPEECH_ACT_COMMISSIVE: u8 = 2;
pub const SPEECH_ACT_EXPRESSIVE: u8 = 3;

/// Prosody markers: particle classes the lexicon maps to a particle in each language.
pub const PROSODY_NONE: u8 = 0;
/// A softening tag (for example 呢).
pub const PROSODY_SOFTEN: u8 = 1;
/// A suggestion tag (for example 吧).
pub const PROSODY_SUGGEST: u8 = 2;
/// A reflective opener (for example 其實).
pub const PROSODY_REFLECT: u8 = 3;
/// A topic-shift opener (for example 話說回來).
pub const PROSODY_TOPIC_SHIFT: u8 = 4;

/// `syntax_gate_flags` bit: the particle slot is open; the lexicon may emit the marker.
pub const GATE_PARTICLE_OPEN: u8 = 0x01;
/// `syntax_gate_flags` bit: the frame is sealed; no further binding is accepted.
pub const GATE_SEALED: u8 = 0x02;
/// `syntax_gate_flags` bits 4–7 hold the `ROLE_*` bits bound so far, shifted by this.
pub const GATE_ROLES_SHIFT: u32 = 4;

/// Roles a template requires, or `None` for an unknown template.
pub const fn required_roles(template: u16) -> Option<u8> {
    match template {
        TEMPLATE_STATE => Some(ROLE_SUBJECT | ROLE_ACTION),
        TEMPLATE_REQUEST => Some(ROLE_ACTION | ROLE_OBJECT),
        TEMPLATE_NEED => Some(ROLE_SUBJECT | ROLE_OBJECT),
        TEMPLATE_CAUSATIVE => Some(ROLE_SUBJECT | ROLE_ACTION | ROLE_OBJECT),
        TEMPLATE_EPISTEMIC => Some(ROLE_SUBJECT | ROLE_ACTION | ROLE_OBJECT | ROLE_AFFECT),
        _ => None,
    }
}

/// The order a template realises its roles in, or `None` for an unknown template. The affect
/// role is a tag at the end for every template except the epistemic one, where it is the hedge
/// that opens the utterance.
pub const fn role_order(template: u16) -> Option<[u8; 4]> {
    match template {
        TEMPLATE_STATE => Some([ROLE_SUBJECT, ROLE_ACTION, ROLE_AFFECT, 0]),
        TEMPLATE_REQUEST => Some([ROLE_ACTION, ROLE_OBJECT, ROLE_AFFECT, 0]),
        TEMPLATE_NEED => Some([ROLE_SUBJECT, ROLE_OBJECT, ROLE_AFFECT, 0]),
        TEMPLATE_CAUSATIVE => Some([ROLE_SUBJECT, ROLE_ACTION, ROLE_OBJECT, ROLE_AFFECT]),
        TEMPLATE_EPISTEMIC => Some([ROLE_AFFECT, ROLE_SUBJECT, ROLE_ACTION, ROLE_OBJECT]),
        _ => None,
    }
}

/// 64-byte linguistic frame slot (whitepaper §5.2.20).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct LinguisticFrameSlot {
    pub frame_template_id: u16,           // [0..2] TEMPLATE_*
    pub speech_act_type: u8,              // [2] SPEECH_ACT_*
    pub politeness_level: u8, // [3] 0 plain; higher is more polite; the lexicon reads it
    pub subject_concept_id: u32, // [4..8] Concept unbound into the subject role
    pub action_predicate_id: u32, // [8..12] Concept unbound into the action role
    pub object_concept_id: u32, // [12..16] Concept unbound into the object role
    pub affect_modifier_id: u16, // [16..18] Concept unbound into the affect role
    pub prosody_tone_marker: u8, // [18] PROSODY_*, selected by the recurrent cell's energy band
    pub syntax_gate_flags: u8, // [19] GATE_* bits
    pub confidence_q16: u32,  // [20..24] Frame confidence: the weakest binding (Q16.16)
    pub recurrent_state_hash: u32, // [24..28] Running hash of the recurrent cell's trajectory
    pub surface_token_id: u32, // [28..32] Surface token the lexicon last realised
    pub linear_attention_energy_q16: i32, // [32..36] The recurrent cell's scalar energy (Q16.16)
    pub _reserved: [u8; 28],  // [36..64] Reserved; MUST be zero
}

impl LinguisticFrameSlot {
    /// An empty frame for `template` with the given speech act and politeness.
    pub fn new(template: u16, speech_act: u8, politeness: u8) -> Self {
        Self {
            frame_template_id: template,
            speech_act_type: speech_act,
            politeness_level: politeness,
            ..Default::default()
        }
    }

    /// Layer 2: binds one role to the concept Layer 1 unbound, with that unbinding's
    /// confidence. The frame's confidence is the weakest binding so far, clamped to 1.0; the
    /// first binding sets it. Refused, with nothing changed, when the frame is sealed, when
    /// `role` is not exactly one `ROLE_*` bit, or when an affect concept does not fit sixteen
    /// bits.
    pub fn bind_role(&mut self, role: u8, concept_id: u32, confidence_q16: u32) -> bool {
        if self.is_sealed() {
            return false;
        }
        match role {
            ROLE_SUBJECT => self.subject_concept_id = concept_id,
            ROLE_ACTION => self.action_predicate_id = concept_id,
            ROLE_OBJECT => self.object_concept_id = concept_id,
            ROLE_AFFECT => {
                if concept_id > u16::MAX as u32 {
                    return false;
                }
                self.affect_modifier_id = concept_id as u16;
            }
            _ => return false,
        }
        let confidence = confidence_q16.min(Q16_ONE);
        self.confidence_q16 = if self.filled_roles() == 0 {
            confidence
        } else {
            self.confidence_q16.min(confidence)
        };
        self.syntax_gate_flags |= role << GATE_ROLES_SHIFT;
        true
    }

    /// The `ROLE_*` bits bound so far (the upper nibble of `syntax_gate_flags`).
    #[inline]
    pub const fn filled_roles(&self) -> u8 {
        self.syntax_gate_flags >> GATE_ROLES_SHIFT
    }

    /// True when every role the template requires is bound. An unknown template is never
    /// complete.
    pub const fn is_complete(&self) -> bool {
        match required_roles(self.frame_template_id) {
            Some(required) => self.filled_roles() & required == required,
            None => false,
        }
    }

    /// True once `seal` has closed the frame to further binding.
    #[inline]
    pub const fn is_sealed(&self) -> bool {
        self.syntax_gate_flags & GATE_SEALED != 0
    }

    /// Seals a complete frame; refused for an incomplete one.
    pub fn seal(&mut self) -> bool {
        if !self.is_complete() {
            return false;
        }
        self.syntax_gate_flags |= GATE_SEALED;
        true
    }

    /// The order in which the lexicon emits the bound roles, zero-terminated: the template's
    /// order restricted to the roles that are bound. `None` for an incomplete frame.
    pub const fn realisation_order(&self) -> Option<[u8; 4]> {
        if !self.is_complete() {
            return None;
        }
        let Some(template_order) = role_order(self.frame_template_id) else {
            return None;
        };
        let mut order = [0u8; 4];
        let mut n = 0;
        let mut i = 0;
        while i < template_order.len() {
            let role = template_order[i];
            if role != 0 && self.filled_roles() & role != 0 {
                order[n] = role;
                n += 1;
            }
            i += 1;
        }
        Some(order)
    }

    /// Layer 3: one step of the linear recurrent cell, $s \leftarrow \alpha s + k v$, every
    /// product widened to `i64` and clamped to the `i32` range (whitepaper §8.1). The energy's
    /// band selects the prosody marker: below a quarter in magnitude none; a positive energy
    /// softens below 1.0 and suggests at or above it; a negative energy reflects above -1.0 and
    /// shifts the topic at or below it. The particle slot opens when a marker is selected. The
    /// trajectory hash mixes the new energy in. Returns the marker.
    pub fn advance_prosody(&mut self, alpha_q16: u32, key_q16: i32, value_q16: i32) -> u8 {
        let decayed = (self.linear_attention_energy_q16 as i64 * alpha_q16 as i64) >> 16;
        let input = (key_q16 as i64 * value_q16 as i64) >> 16;
        let energy = (decayed + input).clamp(i32::MIN as i64, i32::MAX as i64) as i32;
        self.linear_attention_energy_q16 = energy;
        self.recurrent_state_hash =
            (self.recurrent_state_hash ^ energy as u32).wrapping_mul(0x0100_0193);
        let one = Q16_ONE as i32;
        let quarter = one / 4;
        self.prosody_tone_marker = if energy >= one {
            PROSODY_SUGGEST
        } else if energy >= quarter {
            PROSODY_SOFTEN
        } else if energy > -quarter {
            PROSODY_NONE
        } else if energy > -one {
            PROSODY_REFLECT
        } else {
            PROSODY_TOPIC_SHIFT
        };
        if self.prosody_tone_marker == PROSODY_NONE {
            self.syntax_gate_flags &= !GATE_PARTICLE_OPEN;
        } else {
            self.syntax_gate_flags |= GATE_PARTICLE_OPEN;
        }
        self.prosody_tone_marker
    }
}

const _: () = {
    assert!(core::mem::size_of::<LinguisticFrameSlot>() == 64);
    assert!(core::mem::align_of::<LinguisticFrameSlot>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: i32 = Q16_ONE as i32;
    const HALF: u32 = Q16_ONE / 2;

    #[test]
    fn record_is_one_cache_line_and_default_is_an_empty_state_frame() {
        assert_eq!(core::mem::size_of::<LinguisticFrameSlot>(), 64);
        assert_eq!(core::mem::align_of::<LinguisticFrameSlot>(), 64);
        let d = LinguisticFrameSlot::default();
        assert_eq!(d.frame_template_id, TEMPLATE_STATE);
        assert!(!d.is_complete());
        assert!(!d.is_sealed());
        assert_eq!(d.realisation_order(), None);
        assert_eq!(d.prosody_tone_marker, PROSODY_NONE);
    }

    #[test]
    fn every_template_requires_its_roles_and_unknown_templates_require_the_impossible() {
        assert_eq!(
            required_roles(TEMPLATE_STATE),
            Some(ROLE_SUBJECT | ROLE_ACTION)
        );
        assert_eq!(
            required_roles(TEMPLATE_REQUEST),
            Some(ROLE_ACTION | ROLE_OBJECT)
        );
        assert_eq!(
            required_roles(TEMPLATE_NEED),
            Some(ROLE_SUBJECT | ROLE_OBJECT)
        );
        assert_eq!(
            required_roles(TEMPLATE_CAUSATIVE),
            Some(ROLE_SUBJECT | ROLE_ACTION | ROLE_OBJECT)
        );
        assert_eq!(
            required_roles(TEMPLATE_EPISTEMIC),
            Some(ROLE_SUBJECT | ROLE_ACTION | ROLE_OBJECT | ROLE_AFFECT)
        );
        assert_eq!(required_roles(5), None);
        assert_eq!(role_order(5), None);
        let mut f = LinguisticFrameSlot::new(5, SPEECH_ACT_ASSERTIVE, 0);
        for role in [ROLE_SUBJECT, ROLE_ACTION, ROLE_OBJECT, ROLE_AFFECT] {
            assert!(f.bind_role(role, 1, Q16_ONE));
        }
        assert!(!f.is_complete(), "an unknown template never realises");
        assert!(!f.seal());
    }

    #[test]
    fn a_causative_frame_completes_with_three_roles_in_subject_action_object_order() {
        let mut f = LinguisticFrameSlot::new(TEMPLATE_CAUSATIVE, SPEECH_ACT_ASSERTIVE, 0);
        assert!(f.bind_role(ROLE_OBJECT, 30, Q16_ONE));
        assert!(f.bind_role(ROLE_SUBJECT, 10, Q16_ONE));
        assert!(!f.is_complete());
        assert!(f.bind_role(ROLE_ACTION, 20, Q16_ONE));
        assert!(f.is_complete());
        assert_eq!(
            f.realisation_order(),
            Some([ROLE_SUBJECT, ROLE_ACTION, ROLE_OBJECT, 0])
        );
        assert_eq!(
            (
                f.subject_concept_id,
                f.action_predicate_id,
                f.object_concept_id
            ),
            (10, 20, 30)
        );
    }

    #[test]
    fn a_request_puts_its_particle_last_and_an_epistemic_frame_puts_its_hedge_first() {
        let mut r = LinguisticFrameSlot::new(TEMPLATE_REQUEST, SPEECH_ACT_DIRECTIVE, 2);
        assert!(r.bind_role(ROLE_ACTION, 20, Q16_ONE));
        assert!(r.bind_role(ROLE_AFFECT, 7, Q16_ONE));
        assert!(!r.is_complete(), "a request needs its object");
        assert!(r.bind_role(ROLE_OBJECT, 30, Q16_ONE));
        assert_eq!(
            r.realisation_order(),
            Some([ROLE_ACTION, ROLE_OBJECT, ROLE_AFFECT, 0])
        );
        assert_eq!(r.affect_modifier_id, 7);

        let mut e = LinguisticFrameSlot::new(TEMPLATE_EPISTEMIC, SPEECH_ACT_ASSERTIVE, 1);
        for role in [ROLE_SUBJECT, ROLE_ACTION, ROLE_OBJECT] {
            assert!(e.bind_role(role, 1, Q16_ONE));
        }
        assert!(!e.is_complete(), "an epistemic frame needs its hedge");
        assert!(e.bind_role(ROLE_AFFECT, 9, Q16_ONE));
        assert_eq!(
            e.realisation_order(),
            Some([ROLE_AFFECT, ROLE_SUBJECT, ROLE_ACTION, ROLE_OBJECT])
        );
    }

    #[test]
    fn frame_confidence_is_the_weakest_binding_clamped_to_one() {
        let mut f = LinguisticFrameSlot::new(TEMPLATE_STATE, SPEECH_ACT_ASSERTIVE, 0);
        assert!(f.bind_role(ROLE_SUBJECT, 1, u32::MAX));
        assert_eq!(
            f.confidence_q16, Q16_ONE,
            "the first binding sets it, clamped"
        );
        assert!(f.bind_role(ROLE_ACTION, 2, Q16_ONE / 4));
        assert_eq!(f.confidence_q16, Q16_ONE / 4);
        assert!(f.bind_role(ROLE_OBJECT, 3, HALF));
        assert_eq!(
            f.confidence_q16,
            Q16_ONE / 4,
            "a stronger later binding does not raise it"
        );
    }

    #[test]
    fn bad_roles_oversized_affects_and_sealed_frames_refuse_bindings_unchanged() {
        let mut f = LinguisticFrameSlot::new(TEMPLATE_STATE, SPEECH_ACT_ASSERTIVE, 0);
        let before = f;
        assert!(
            !f.bind_role(ROLE_SUBJECT | ROLE_ACTION, 1, Q16_ONE),
            "one role at a time"
        );
        assert!(!f.bind_role(0x10, 1, Q16_ONE), "not a role");
        assert!(
            !f.bind_role(ROLE_AFFECT, 0x1_0000, Q16_ONE),
            "the affect field is sixteen bits"
        );
        assert_eq!(f, before);
        assert!(!f.seal(), "an incomplete frame cannot be sealed");
        assert!(f.bind_role(ROLE_SUBJECT, 1, Q16_ONE));
        assert!(f.bind_role(ROLE_ACTION, 2, Q16_ONE));
        assert!(f.seal());
        let sealed = f;
        assert!(!f.bind_role(ROLE_OBJECT, 3, Q16_ONE));
        assert_eq!(f, sealed);
    }

    #[test]
    fn the_recurrent_cell_decays_by_alpha_and_integrates_key_times_value() {
        let mut f = LinguisticFrameSlot::default();
        assert_eq!(f.advance_prosody(HALF, ONE, ONE), PROSODY_SUGGEST);
        assert_eq!(f.linear_attention_energy_q16, ONE, "0 × 0.5 + 1.0 × 1.0");
        assert_eq!(f.advance_prosody(HALF, 0, 0), PROSODY_SOFTEN);
        assert_eq!(f.linear_attention_energy_q16, ONE / 2, "1.0 × 0.5 + 0");
        assert_eq!(f.advance_prosody(HALF, 0, 0), PROSODY_SOFTEN);
        assert_eq!(f.linear_attention_energy_q16, ONE / 4);
        assert_eq!(
            f.advance_prosody(HALF, 0, 0),
            PROSODY_NONE,
            "an eighth is below the band"
        );
        assert!(f.syntax_gate_flags & GATE_PARTICLE_OPEN == 0);
    }

    #[test]
    fn negative_energy_selects_the_openers_and_the_gate_follows_the_marker() {
        let mut f = LinguisticFrameSlot::default();
        assert_eq!(f.advance_prosody(0, -ONE / 2, ONE), PROSODY_REFLECT);
        assert!(f.syntax_gate_flags & GATE_PARTICLE_OPEN != 0);
        assert_eq!(f.advance_prosody(0, -ONE, ONE), PROSODY_TOPIC_SHIFT);
        assert_eq!(f.advance_prosody(0, 0, 0), PROSODY_NONE);
        assert!(f.syntax_gate_flags & GATE_PARTICLE_OPEN == 0);
    }

    #[test]
    fn the_cell_saturates_and_the_trajectory_hash_changes_with_the_energy() {
        let mut f = LinguisticFrameSlot {
            linear_attention_energy_q16: i32::MAX,
            ..Default::default()
        };
        let h0 = f.recurrent_state_hash;
        f.advance_prosody(Q16_ONE, i32::MAX, ONE);
        assert_eq!(
            f.linear_attention_energy_q16,
            i32::MAX,
            "clamps rather than wraps"
        );
        let h1 = f.recurrent_state_hash;
        assert_ne!(h0, h1);
        f.linear_attention_energy_q16 = i32::MIN;
        f.advance_prosody(Q16_ONE, i32::MIN, ONE);
        assert_eq!(f.linear_attention_energy_q16, i32::MIN);
        assert_ne!(f.recurrent_state_hash, h1);
        let mut g = LinguisticFrameSlot::default();
        let mut h = LinguisticFrameSlot::default();
        g.advance_prosody(HALF, ONE, ONE);
        h.advance_prosody(HALF, ONE, ONE);
        assert_eq!(
            g.recurrent_state_hash, h.recurrent_state_hash,
            "the hash is deterministic"
        );
    }
}
