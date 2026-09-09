//! VirtualCortex Core Neural Biophysical Engine

#![no_std]
pub mod dispatch;
pub mod dynamics;

pub use dynamics::{DendriticSuperNeuron, SynapseBlock};
