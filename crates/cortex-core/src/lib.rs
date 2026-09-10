//! Neural state and dispatch: the `DendriticSuperNeuron` and `SynapseBlock` arena records, the
//! synaptic efficacy arithmetic and the two-tier timing wheel (whitepaper §5.2.1). Membrane
//! dynamics, the mailbox and the executor are Specified (§6.1, §8.8; briefs 009 to 011).

#![no_std]
pub mod dispatch;
pub mod dynamics;

pub use dispatch::wheel::{FlatTimingWheel, ScheduleError, WorkerWheel};
pub use dynamics::{DendriticSuperNeuron, SynapseBlock, synaptic_efficacy_q16};
