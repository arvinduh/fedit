//! Fire Emblem: Awakening save domain models and binary structures.
//!
//! Owns save slot campaign structures, unit roster, convoy storage, support
//! affinity tracking, and global system save state. Lower layers handle raw
//! container decompression; higher layers handle process and file I/O.

#![deny(clippy::print_stdout, clippy::print_stderr)]

pub mod error;
pub mod global;
pub mod save;

pub use error::Error;
pub use global::GlobalSave;
pub use save::SaveSlot;
pub use save::avatar;
pub use save::convoy;
pub use save::paralogue;
pub use save::roster;
pub use save::support;
pub use save::types;

pub use save as slot;
