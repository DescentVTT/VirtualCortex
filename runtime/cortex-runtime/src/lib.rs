//! The VirtualCortex executor: the runtime crate that composes the state crates (whitepaper
//! §4.3, §6.1; ADR-0023). A fixed pool of worker threads, one work-stealing deque and one
//! timing wheel per worker, mailbox delivery, synaptic fan-out and STDP, in three
//! barrier-separated phases per fine tick. Everything is allocated in [`Executor::new`];
//! nothing allocates, blocks or (apart from the barrier's yield) makes a system call in the
//! loop. This crate is `std`, is never published, and is the one place in the workspace with
//! `unsafe`: the arena access of [`arena`], under the invariant ADR-0023 names.

pub mod arena;
pub mod barrier;
pub mod deque;
pub mod executor;
pub mod injector;
pub mod pool;

pub use executor::{ACTIVATE, Config, ConfigError, Executor, Inject, InjectError, WorkerReport};

/// The executor with the production wheel geometry (2 048 tokens per slot, ADR-0013).
pub type ProductionExecutor = Executor<2048>;
