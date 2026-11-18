//! Fire Emblem: Awakening save domain models and binary structures.
//!
//! Owns save campaign structures, unit roster, convoy storage, support
//! affinity tracking, and global system save state. Lower layers handle raw
//! container decompression; higher layers handle process and file I/O.

#![deny(clippy::print_stdout, clippy::print_stderr)]

use thiserror;

pub mod global;
pub mod save;

pub use global::GlobalSave;
pub use save::PlayTime;
pub use save::Save;
pub use save::avatar;
pub use save::convoy;
pub use save::header;
pub use save::header::Difficulty;
pub use save::header::GameMode;
pub use save::roster;
pub use save::social;
pub use save::social::ParalogueError;
pub use save::social::ParalogueState;
pub use save::social::SupportError;
pub use save::social::SupportRank;

/// Root aggregate error type for Fire Emblem: Awakening domain operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Save campaign slot parsing or mutation error.
  #[error(transparent)]
  Save(#[from] save::Error),
  /// Global system save parsing or mutation error.
  #[error(transparent)]
  Global(#[from] global::Error),
}
