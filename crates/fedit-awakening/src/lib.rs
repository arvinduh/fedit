//! Fire Emblem: Awakening save domain models and binary structures.
//!
//! Owns save slot campaign structures, unit roster, convoy storage, support
//! affinity tracking, and global system save state. Lower layers handle raw
//! container decompression; higher layers handle process and file I/O.

#![deny(clippy::print_stdout, clippy::print_stderr)]

pub mod error;
pub mod global;
pub mod slot;

pub use error::Error;
pub use global::GlobalSave;
pub use slot::SaveSlot;
pub use slot::convoy;
pub use slot::roster;
pub use slot::support;
pub use slot::types;
