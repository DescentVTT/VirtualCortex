//! Dispatch: the timing wheel that orders delayed delivery (ADR-0013; the delivery phase that
//! drains it is the executor's, ADR-0023) and the cadence a rule slower than the tick runs on
//! (ADR-0035).
pub mod cadence;
pub mod wheel;
