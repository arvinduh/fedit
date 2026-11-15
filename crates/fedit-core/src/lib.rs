//! Application core services for Fire Emblem save editing.
//!
//! Owns configuration paths, content-addressable save backups, manifest
//! tracking, and portrait asset resolution.

#![deny(clippy::print_stdout, clippy::print_stderr)]

pub use awakening;

pub mod assets;
pub mod backup;
pub mod error;
pub mod hash;
pub mod manifest;
pub mod paths;

pub use backup::HistoryEntry;
pub use backup::RestoreOutcome;
pub use backup::create_backup;
pub use backup::list_history;
pub use backup::restore;
pub use error::Error;
pub use manifest::Manifest;
pub use manifest::Snapshot;
pub use paths::backups_dir;
pub use paths::data_dir;
pub use paths::manifest_path;
pub use paths::portraits_dir;
