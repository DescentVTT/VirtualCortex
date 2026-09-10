//! Neural state and dispatch: the `DendriticSuperNeuron` and `SynapseBlock` arena records, the
//! turn gate and mailbox (ADR-0017), membrane integration (ADR-0018), short-term plasticity
//! (ADR-0019), synaptic fan-out with STDP and the delivery encodings (ADR-0022), the Tier-2
//! plastic delta and the records' image bytes (ADR-0024), and the two-tier timing wheel
//! (ADR-0013); whitepaper §5.2.1. The executor that composes them is a
//! runtime crate (§6.1; brief 012).

#![no_std]
// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// here (ADR-0029; migrated under brief 016 on 2026-09-10).
#![deny(clippy::arithmetic_side_effects)]
pub mod dispatch;
pub mod dynamics;
pub mod serial;

pub use dispatch::wheel::{FlatTimingWheel, MAX_TOKEN, ScheduleError, WorkerWheel};
pub use dynamics::delta::{DELTA_END, DeltaChain, PlasticDelta};
pub use dynamics::membrane::{
    APICAL_LEAK_SHIFT, BAC_APICAL_THRESHOLD, BAC_PLATEAU_TICKS, BASAL_LEAK_SHIFT,
    BURST_REFRACTORY_TICKS, COUPLING_SHIFT, FLAG_BURST_MODE, FLAG_INHIBITORY,
    PLATEAU_COUPLING_SHIFT, REFRACTORY_TICKS, SOMA_LEAK_SHIFT, THRESHOLD_BASE,
    THRESHOLD_DECAY_SHIFT, THRESHOLD_STEP, V_RESET,
};
pub use dynamics::plasticity::{
    STP_MAX, STP_TAU_D_SHIFT, STP_TAU_F_SHIFT, STP_U, stp_decay_factor_q16,
};
pub use dynamics::synapse::{
    CHAIN_END, Chain, FanOut, MAX_TOKEN_BLOCK, MESSAGE_APICAL, NO_SPIKE_ON_RECORD, SLOT_EMPTY,
    STDP_A_MINUS_Q1_15, STDP_A_PLUS_Q1_15, STDP_TAU_SHIFT, SYNAPSES_PER_BLOCK, Synapse,
    message_efficacy_q16, message_is_apical, spike_message, synapse_token, token_block, token_slot,
};
pub use dynamics::{
    DendriticSuperNeuron, GateState, MAILBOX_EMPTY, MAILBOX_NIL, MailboxDrain, MailboxNode,
    SynapseBlock, synaptic_efficacy_q16,
};
