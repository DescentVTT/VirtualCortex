//! VirtualCortex Core Neural Biophysical Engine

#![no_std]
pub mod dispatch;
pub mod dynamics;

pub use dispatch::wheel::{FlatTimingWheel, ScheduleError, WorkerWheel};
pub use dynamics::{DendriticSuperNeuron, SynapseBlock, synaptic_efficacy_q16};
