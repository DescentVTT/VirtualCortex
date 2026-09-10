//! Tool-invocation frames: the 64-byte unit through which the engine acts on a digital
//! environment (whitepaper §3.2, §5.2.21, §6.8, §8.10; admitted by ADR-0016).
//!
//! The engine never makes the call a tool needs. It writes a frame into a shared-memory ring
//! (cursor protocol of ADR-0015); a separate broker process, holding the only credentials and
//! its own allow-list, reads the frame, checks the `authorization_level` the ethics gate
//! (`cortex-ethics`) wrote into it, performs the action, and writes the result back into the
//! same frame. The worker's seccomp filter is unchanged. The frame's state machine is
//! Implemented; the broker and the ring mapping are the runtime's (Specified). Since ADR-0031
//! a third category names the amendment register: the channel for what the engine may not
//! change about itself and the audit trail of what it did (whitepaper §8.18).

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here, one of the crates that passed the lint when it was adopted (ADR-0029).
#![deny(clippy::arithmetic_side_effects)]

/// Written by the engine, not yet seen by the broker.
pub const STATUS_PENDING: u32 = 0;
/// Claimed by the broker.
pub const STATUS_RUNNING: u32 = 1;
/// Finished; `return_payload[..payload_len]` is the result.
pub const STATUS_COMPLETED: u32 = 2;
/// The broker could not perform the action.
pub const STATUS_FAILED: u32 = 3;
/// Refused before execution by the ethics gate or the broker's policy.
pub const STATUS_DENIED: u32 = 4;
/// Bytes of result a frame can carry in place.
pub const PAYLOAD_BYTES: usize = 32;

/// Tool category: a formal prover or symbolic solver co-processor the broker runs on a
/// conjecture (whitepaper §6.10). The opcodes name mathematical actions, never a product:
/// which prover or solver the broker runs is its configuration, judged under whitepaper §2.1
/// when it is chosen, and the engine stays agnostic and dependency-free. Categories 0x0001 to
/// 0x0003 are reserved for the broker's basic services (Specified).
pub const TOOL_CATEGORY_FORMAL_PROVER: u16 = 0x0004;
/// `TOOL_CATEGORY_FORMAL_PROVER`: check a formal proof term against axiomatic definitions.
pub const ACTION_VERIFY_PROOF: u16 = 0x0001;
/// `TOOL_CATEGORY_FORMAL_PROVER`: decide a formula by automated constraint or SMT solving.
pub const ACTION_SOLVE_CONSTRAINTS: u16 = 0x0002;
/// `TOOL_CATEGORY_FORMAL_PROVER`: algebraic symbolic simplification and term rewriting.
pub const ACTION_SYMBOLIC_EVAL: u16 = 0x0003;
/// Tool category: the document engine the broker runs over a structured text (whitepaper §6.11).
pub const TOOL_CATEGORY_DOC_ENGINE: u16 = 0x0005;
/// `TOOL_CATEGORY_DOC_ENGINE`: extract the hierarchy, section dependencies and cross-references.
pub const ACTION_PARSE_STRUCTURE: u16 = 0x0001;
/// `TOOL_CATEGORY_DOC_ENGINE`: extract tables, units, metrics and formal claims as triples.
pub const ACTION_EXTRACT_ENTITIES: u16 = 0x0002;
/// `TOOL_CATEGORY_DOC_ENGINE`: search for premise–conclusion contradictions and citation validity.
pub const ACTION_SEARCH_CROSS_REF: u16 = 0x0003;
/// Tool category: the amendment register outside the engine (whitepaper §8.18; ADR-0031). The
/// engine amends a parameter of its own policy by itself, through the gates of
/// `cortex-executive`'s `PolicyAmendment`; it never amends its own code. What it cannot commit
/// leaves through this category as a frame, and what it did commit is recorded outside it.
/// Whether the register is a file, an issue tracker or a pull request is the broker's
/// configuration, never named here.
pub const TOOL_CATEGORY_AMENDMENT_REGISTER: u16 = 0x0006;
/// `TOOL_CATEGORY_AMENDMENT_REGISTER`: file an amendment the engine may not commit itself, a
/// change to a rule rather than to a registered parameter, for the repository's gates
/// (ADR-0029, ADR-0030) and its maintainers; `param_hash` names the amendment record.
pub const ACTION_FILE_PROPOSAL: u16 = 0x0001;
/// `TOOL_CATEGORY_AMENDMENT_REGISTER`: record a committed parameter amendment in the
/// operator's register, an audit trail the engine cannot rewrite; `param_hash` names the record.
pub const ACTION_RECORD_COMMIT: u16 = 0x0002;

/// True for a (category, opcode) pair this crate defines. Opcodes are per category: 0x0001 is
/// a proof check under the prover and a structure parse under the document engine. The
/// broker keeps its own allow-list; this is the engine's mirror of it, so that a frame the
/// engine cannot name is never written.
pub const fn is_known_action(category: u16, opcode: u16) -> bool {
    match category {
        TOOL_CATEGORY_FORMAL_PROVER => matches!(
            opcode,
            ACTION_VERIFY_PROOF | ACTION_SOLVE_CONSTRAINTS | ACTION_SYMBOLIC_EVAL
        ),
        TOOL_CATEGORY_DOC_ENGINE => matches!(
            opcode,
            ACTION_PARSE_STRUCTURE | ACTION_EXTRACT_ENTITIES | ACTION_SEARCH_CROSS_REF
        ),
        TOOL_CATEGORY_AMENDMENT_REGISTER => {
            matches!(opcode, ACTION_FILE_PROPOSAL | ACTION_RECORD_COMMIT)
        }
        _ => false,
    }
}

/// 64-byte tool-invocation frame (whitepaper §5.2.21).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct ToolInvocationFrame {
    pub call_id: u64,                        // [0..8] Unique call identifier
    pub return_payload: [u8; PAYLOAD_BYTES], // [8..40] Result bytes, valid up to payload_len
    pub param_hash: u32,                     // [40..44] Hash of the parameters, held elsewhere
    pub execution_status: u32,               // [44..48] STATUS_*
    pub tool_category: u16,                  // [48..50] Broker-defined category
    pub action_opcode: u16,                  // [50..52] Broker-defined action
    pub authorization_level: u8, // [52] Written by the ethics gate; checked by the broker
    pub payload_len: u8,         // [53] Valid bytes of return_payload
    pub _reserved: [u8; 10],     // [54..64] Reserved; MUST be zero
}

impl ToolInvocationFrame {
    /// Engine: a pending frame for a known action, carrying the authorization level the veto
    /// gate assigned. `None` for a (category, opcode) pair this crate does not define, so that
    /// an unknown action never reaches the ring.
    pub fn new_call(
        call_id: u64,
        category: u16,
        opcode: u16,
        param_hash: u32,
        authorization_level: u8,
    ) -> Option<Self> {
        if !is_known_action(category, opcode) {
            return None;
        }
        Some(Self {
            call_id,
            tool_category: category,
            action_opcode: opcode,
            param_hash,
            authorization_level,
            ..Default::default()
        })
    }

    /// Broker: claims a pending frame. Refused from any other state.
    pub fn start(&mut self) -> bool {
        self.transition(STATUS_PENDING, STATUS_RUNNING)
    }

    /// Broker: finishes a running frame with `payload`, at most [`PAYLOAD_BYTES`] long.
    /// Refused, with the frame unchanged, when the frame is not running or the payload is too
    /// long.
    pub fn complete(&mut self, payload: &[u8]) -> bool {
        if self.execution_status != STATUS_RUNNING || payload.len() > PAYLOAD_BYTES {
            return false;
        }
        self.return_payload = [0; PAYLOAD_BYTES];
        self.return_payload[..payload.len()].copy_from_slice(payload);
        self.payload_len = payload.len() as u8;
        self.execution_status = STATUS_COMPLETED;
        true
    }

    /// Broker: marks a running frame failed. Refused from any other state.
    pub fn fail(&mut self) -> bool {
        self.transition(STATUS_RUNNING, STATUS_FAILED)
    }

    /// Ethics gate or broker policy: refuses a pending frame before execution.
    pub fn deny(&mut self) -> bool {
        self.transition(STATUS_PENDING, STATUS_DENIED)
    }

    /// True once the frame will not change again.
    #[inline]
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self.execution_status,
            STATUS_COMPLETED | STATUS_FAILED | STATUS_DENIED
        )
    }

    /// The valid result bytes.
    #[inline]
    pub fn payload(&self) -> &[u8] {
        let n = (self.payload_len as usize).min(PAYLOAD_BYTES);
        &self.return_payload[..n]
    }

    fn transition(&mut self, from: u32, to: u32) -> bool {
        if self.execution_status != from {
            return false;
        }
        self.execution_status = to;
        if to != STATUS_COMPLETED {
            self.payload_len = 0;
        }
        true
    }
}

const _: () = {
    assert!(core::mem::size_of::<ToolInvocationFrame>() == 64);
    assert!(core::mem::align_of::<ToolInvocationFrame>() == 64);
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_is_one_cache_line_and_default_is_pending() {
        assert_eq!(core::mem::size_of::<ToolInvocationFrame>(), 64);
        assert_eq!(core::mem::align_of::<ToolInvocationFrame>(), 64);
        let f = ToolInvocationFrame::default();
        assert_eq!(f.execution_status, STATUS_PENDING);
        assert!(!f.is_terminal());
        assert!(f.payload().is_empty());
    }

    #[test]
    fn the_happy_path_is_pending_running_completed() {
        let mut f = ToolInvocationFrame::default();
        assert!(f.start());
        assert!(f.complete(b"ok"));
        assert_eq!(f.payload(), b"ok");
        assert!(f.is_terminal());
        assert!(!f.start(), "a terminal frame never restarts");
    }

    #[test]
    fn transitions_from_the_wrong_state_are_refused_unchanged() {
        let mut f = ToolInvocationFrame::default();
        let before = f;
        assert!(!f.complete(b"x"), "not running");
        assert!(!f.fail(), "not running");
        assert_eq!(f, before);
        assert!(f.start());
        assert!(!f.deny(), "deny applies to pending frames only");
        assert_eq!(f.execution_status, STATUS_RUNNING);
    }

    #[test]
    fn deny_and_fail_clear_the_payload_length() {
        let mut f = ToolInvocationFrame {
            payload_len: 5,
            ..Default::default()
        };
        assert!(f.deny());
        assert_eq!((f.execution_status, f.payload_len), (STATUS_DENIED, 0));
        let mut g = ToolInvocationFrame::default();
        g.start();
        g.payload_len = 3;
        assert!(g.fail());
        assert_eq!((g.execution_status, g.payload_len), (STATUS_FAILED, 0));
    }

    #[test]
    fn the_defined_actions_are_known_per_category_and_nothing_else_is() {
        assert!(is_known_action(
            TOOL_CATEGORY_FORMAL_PROVER,
            ACTION_VERIFY_PROOF
        ));
        assert!(is_known_action(
            TOOL_CATEGORY_FORMAL_PROVER,
            ACTION_SOLVE_CONSTRAINTS
        ));
        assert!(is_known_action(
            TOOL_CATEGORY_FORMAL_PROVER,
            ACTION_SYMBOLIC_EVAL
        ));
        assert!(
            !is_known_action(TOOL_CATEGORY_FORMAL_PROVER, 0x0004),
            "the prover has three actions"
        );
        assert!(is_known_action(
            TOOL_CATEGORY_DOC_ENGINE,
            ACTION_PARSE_STRUCTURE
        ));
        assert!(is_known_action(
            TOOL_CATEGORY_DOC_ENGINE,
            ACTION_EXTRACT_ENTITIES
        ));
        assert!(is_known_action(
            TOOL_CATEGORY_DOC_ENGINE,
            ACTION_SEARCH_CROSS_REF
        ));
        assert!(
            !is_known_action(0x0003, ACTION_VERIFY_PROOF),
            "an opcode means nothing outside its category"
        );
        assert!(!is_known_action(TOOL_CATEGORY_DOC_ENGINE, 0));
        assert!(is_known_action(
            TOOL_CATEGORY_AMENDMENT_REGISTER,
            ACTION_FILE_PROPOSAL
        ));
        assert!(is_known_action(
            TOOL_CATEGORY_AMENDMENT_REGISTER,
            ACTION_RECORD_COMMIT
        ));
        assert!(
            !is_known_action(TOOL_CATEGORY_AMENDMENT_REGISTER, 0x0003),
            "the register has two actions"
        );
        assert!(!is_known_action(TOOL_CATEGORY_AMENDMENT_REGISTER, 0));
        assert_ne!(TOOL_CATEGORY_AMENDMENT_REGISTER, TOOL_CATEGORY_DOC_ENGINE);
        assert_eq!(TOOL_CATEGORY_AMENDMENT_REGISTER, 0x0006);
        assert!(
            !is_known_action(0x0001, 0x0001),
            "reserved categories name nothing yet"
        );
        assert_ne!(TOOL_CATEGORY_FORMAL_PROVER, TOOL_CATEGORY_DOC_ENGINE);
    }

    #[test]
    fn new_call_builds_a_pending_frame_for_a_known_action_only() {
        let f = ToolInvocationFrame::new_call(
            9,
            TOOL_CATEGORY_FORMAL_PROVER,
            ACTION_SOLVE_CONSTRAINTS,
            0xABCD,
            2,
        )
        .expect("known");
        assert_eq!(
            (f.call_id, f.tool_category, f.action_opcode),
            (9, TOOL_CATEGORY_FORMAL_PROVER, ACTION_SOLVE_CONSTRAINTS)
        );
        assert_eq!((f.param_hash, f.authorization_level), (0xABCD, 2));
        assert_eq!(f.execution_status, STATUS_PENDING);
        assert!(f.payload().is_empty());
        assert!(ToolInvocationFrame::new_call(9, TOOL_CATEGORY_DOC_ENGINE, 0x0009, 0, 0).is_none());
    }

    #[test]
    fn a_payload_longer_than_the_frame_is_refused_and_a_full_one_fits() {
        let mut f = ToolInvocationFrame::default();
        f.start();
        assert!(!f.complete(&[7u8; PAYLOAD_BYTES + 1]));
        assert_eq!(f.execution_status, STATUS_RUNNING);
        assert!(f.complete(&[7u8; PAYLOAD_BYTES]));
        assert_eq!(f.payload().len(), PAYLOAD_BYTES);
        assert!(f.payload().iter().all(|&b| b == 7));
    }
}
