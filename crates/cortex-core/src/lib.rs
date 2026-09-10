//! Neural state and dispatch: the `DendriticSuperNeuron` and `SynapseBlock` arena records, the
//! synaptic efficacy arithmetic and the two-tier timing wheel (whitepaper §5.2.1). Membrane
//! dynamics, the mailbox and the executor are Specified (§6.1, §8.8; briefs 009 to 011).

#![no_std]
pub mod dispatch;
pub mod dynamics;

pub use dispatch::wheel::{FlatTimingWheel, ScheduleError, WorkerWheel};
pub use dynamics::membrane::{
    APICAL_LEAK_SHIFT, BAC_APICAL_THRESHOLD, BAC_PLATEAU_TICKS, BASAL_LEAK_SHIFT,
    BURST_REFRACTORY_TICKS, COUPLING_SHIFT, FLAG_BURST_MODE, FLAG_INHIBITORY,
    PLATEAU_COUPLING_SHIFT, REFRACTORY_TICKS, SOMA_LEAK_SHIFT, THRESHOLD_BASE,
    THRESHOLD_DECAY_SHIFT, THRESHOLD_STEP, V_RESET,
};
pub use dynamics::{
    DendriticSuperNeuron, GateState, MAILBOX_EMPTY, MAILBOX_NIL, MailboxDrain, MailboxNode,
    SynapseBlock, synaptic_efficacy_q16,
};
