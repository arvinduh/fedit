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
pub mod saveset;

pub use assets::ensure_asset_dirs;
pub use assets::initials;
pub use assets::resolve_item_icon;
pub use assets::resolve_map_sprite;
pub use assets::resolve_portrait;
pub use assets::resolve_skill_icon;
pub use assets::tint_pixels;
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
pub use saveset::SaveSet;
pub use saveset::SlotKind;
pub use saveset::SlotSummary;
