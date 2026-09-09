//! VirtualCortex Core Neural Biophysical Engine

#![no_std]
pub mod dispatch;
pub mod dynamics;

pub use dynamics::{synaptic_efficacy_q16, DendriticSuperNeuron, SynapseBlock};
