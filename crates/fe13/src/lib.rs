//! Fire Emblem: Awakening save domain models and binary structures.
//!
//! Owns save slot preview headers, EDNI block indexing, user state, unit
//! roster, and convoy storage. Lower layers handle raw container decompression;
//! higher layers handle process and file I/O.

#![deny(clippy::print_stdout, clippy::print_stderr)]

pub mod error;
pub mod text;
pub mod types;

pub use error::Error;
pub use types::ClassId;
pub use types::Difficulty;
pub use types::GameMode;
pub use types::ItemId;
pub use types::UnitId;
