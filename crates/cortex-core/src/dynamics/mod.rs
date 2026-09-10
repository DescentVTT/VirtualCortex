pub mod neuron;
pub use neuron::{
    DendriticSuperNeuron, GateState, MAILBOX_EMPTY, MAILBOX_NIL, MailboxDrain, MailboxNode,
    SynapseBlock, synaptic_efficacy_q16,
};
