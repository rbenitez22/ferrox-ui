//! The build script's logic, apart from `build.rs` so the crate's unit tests
//! can include it (`src/lib.rs`).

pub mod discover;
pub mod emit;
pub mod markup;
pub mod naming;
pub mod sources;
pub mod validate;
