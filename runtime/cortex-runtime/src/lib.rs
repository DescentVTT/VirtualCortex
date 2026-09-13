//! The VirtualCortex executor: the runtime crate that composes the state crates (whitepaper
//! §4.3, §6.1; ADR-0023). A fixed pool of worker threads, one work-stealing deque and one
//! timing wheel per worker, mailbox delivery, synaptic fan-out and STDP, in three
//! barrier-separated phases per fine tick; the `.cortex` image writer and loader and the clock
//! sweep with its write-ahead log (ADR-0024); the policy amendment's trial in two forks of the
//! image and its commit into the live policy (ADR-0031); the modulator of three-factor
//! plasticity (ADR-0032); the population spike tally and the criticality controller's gain,
//! stepped on a cadence (ADR-0035, ADR-0036); the sleep stages on the window's cadence
//! (ADR-0037) and the episodic ledger, replayed on the ripple's during slow-wave sleep
//! (ADR-0038); and, between ticks with no executor field, the language composition of
//! [`language`]: a category sequence reduced on the term arena into a frame, the frame sealed
//! as a hypervector and read back through a codebook (ADR-0039, ADR-0040), and the discovery
//! path of [`discovery`]: an invention's drop in a clause store's description length as the
//! valence and the modulator's reward, and a prover frame's certificate into a theorem
//! (ADR-0041, ADR-0043). Everything is allocated in [`Executor::new`];
//! nothing allocates, blocks or (apart from the barrier's yield) makes a system call in the
//! loop. This crate is `std`, is never published, and is the one place in the workspace with
//! `unsafe`: the arena access of [`arena`], under the invariant ADR-0023 names.

// §8.1: an operation on a state field saturates or wraps by name; plain arithmetic is refused
// in the runtime too, the last crate to pass the lint (brief 016, 2026-09-10; ADR-0029).
#![deny(clippy::arithmetic_side_effects)]

pub mod arena;
pub mod barrier;
pub mod branching;
pub mod deque;
pub mod discovery;
pub mod executor;
pub mod image;
pub mod injector;
pub mod language;
pub mod pool;
pub mod synthesis;
pub mod trial;

pub use branching::{
    Attribution, Cascade, ForkError, Perturbation, cascade, fork, run_driven, trace,
};
pub use discovery::{
    CERTIFICATE_BYTES, COMPRESSION_REWARD_SHIFT, CertifyError, Discovery, DiscoveryError,
    LENGTH_CEILING, SearchReport, certify_from_frame, conjecture_frame, description_length,
    free_energy_q16, invent, prime, reward_q16, search,
};
pub use executor::{
    ACTIVATE, AmendError, Config, ConfigError, Executor, Inject, InjectError, Policy, TagError,
    WorkerReport,
};
pub use image::{Image, ImageError, WriteAheadLog};
pub use language::{
    DECODE_FLOOR_Q16, LanguageError, ROLE_CONCEPT_BASE, ROLES, comprehend, concept_in,
    decode_frame, encode_frame, read_role, role_concept, role_of_concept, role_slot,
};
pub use synthesis::{Drive, SynthesisError, blocks_for, blocks_per_unit, mix64, synthesize};
pub use trial::{ForkReport, Trial, TrialReport, run as run_trial};

/// The executor with the production wheel geometry (2 048 tokens per slot, ADR-0013).
pub type ProductionExecutor = Executor<2048>;
