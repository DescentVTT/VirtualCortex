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
//! (ADR-0041, ADR-0043); the lexicon of [`lexicon`] between a host's token ids and the frame,
//! in both directions (ADR-0046); and the episodes of [`episode`], tagged from a spike train
//! and bound to a rewarded invention (ADR-0048), from the executor's own train since ADR-0050
//! (every worker's spikes of a tick merged in unit order into a bounded ring after the tick).
//! Since ADR-0052 the executor owns a term arena and a clause store ([`store`]) that the
//! image carries, and the discovery loop runs inside the tick on a cadence while awake: the
//! search from its cursor, the reward into the modulator, the coincidence before the reward
//! tagged and bound to the invented predicate. Everything is allocated in [`Executor::new`];
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
pub mod episode;
pub mod executor;
pub mod image;
pub mod injector;
pub mod language;
pub mod lexicon;
pub mod pool;
pub mod store;
pub mod synthesis;
pub mod trial;

pub use branching::{
    Attribution, Cascade, ForkError, Perturbation, cascade, fork, run_driven, trace, train_of,
};
pub use discovery::{
    CERTIFICATE_BYTES, COMPRESSION_REWARD_SHIFT, CertifyError, Discovery, DiscoveryError,
    LENGTH_CEILING, SearchReport, certify_from_frame, conjecture_frame, description_length,
    free_energy_q16, invent, prime, reward_q16, search, search_from,
};
pub use episode::{
    Association, COINCIDENCE_TICKS, DISCOVERY_WINDOW, DiscoverError, DiscoverReport, tag_burst,
    tag_burst_in, tag_discovery, tag_discovery_recent, tag_from_trace, tag_recent,
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
pub use lexicon::{
    Entry, Lexicon, LexiconError, MAX_NESTING, Reading, SHAPE_ADJECTIVE, SHAPE_DETERMINER,
    SHAPE_HEDGE, SHAPE_INTRANSITIVE, SHAPE_NOMINAL, SHAPE_NOUN, SHAPE_TAG, SHAPE_TRANSITIVE,
    comprehend_tokens, realise,
};
pub use store::TermError;
pub use synthesis::{Drive, SynthesisError, blocks_for, blocks_per_unit, mix64, synthesize};
pub use trial::{ForkReport, Trial, TrialReport, run as run_trial};

/// The executor with the production wheel geometry (2 048 tokens per slot, ADR-0013).
pub type ProductionExecutor = Executor<2048>;
