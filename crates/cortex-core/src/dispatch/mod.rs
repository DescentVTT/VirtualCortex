//! Dispatch: the timing wheel that orders delayed delivery (ADR-0013). The delivery path from
//! the wheel into mailboxes is Specified (whitepaper §6.1).
pub mod wheel;
