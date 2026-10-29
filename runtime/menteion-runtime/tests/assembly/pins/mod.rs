//! The tables `assembly.rs`'s rounds pinned from their runs, one file a round (ADR-0227). Each
//! table is a typed `const` with a row a line, in Debug's form as the round's DUMP printed it,
//! and `mod pins` is `#[rustfmt::skip]`, so rustfmt lays none of them out.

mod brief048;
pub(crate) use brief048::*;
mod brief049;
pub(crate) use brief049::*;
mod brief050;
pub(crate) use brief050::*;
mod brief051;
pub(crate) use brief051::*;
mod brief052;
pub(crate) use brief052::*;
mod brief053;
pub(crate) use brief053::*;
