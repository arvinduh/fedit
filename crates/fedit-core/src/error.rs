//! Error types for application-level operations and manifest management.
//!
//! Defines errors encountered during backup, restoration, and file resolution.

use std::io;
use std::path;

use thiserror;

/// Application core subsystem errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// File system input/output error.
  #[error("I/O error at {path}: {source}")]
  Io {
    /// Context file path where I/O failed.
    path: path::PathBuf,
    /// Underlying standard I/O error.
    #[source]
    source: io::Error,
  },

  /// JSON serialization or deserialization failure.
  #[error("Manifest format error: {0}")]
  Json(#[from] serde_json::Error),

  /// Fire Emblem save domain or format error.
  #[error(transparent)]
  Awakening(#[from] awakening::Error),

  /// Save set directory not found or not a directory.
  #[error("Path '{path}' is not a valid directory")]
  NotADirectory {
    /// Given path.
    path: path::PathBuf,
  },

  /// Path does not exist on disk.
  #[error("Path '{path}' does not exist on disk")]
  PathNotFound {
    /// Non-existent path.
    path: path::PathBuf,
  },

  /// Directory contains no valid Fire Emblem: Awakening save files.
  #[error(
    "No valid Fire Emblem save files found in '{path}'. Expected at least one of Global, Chapter*, or Map*."
  )]
  NoSavesFound {
    /// Scanned directory path.
    path: path::PathBuf,
  },

  /// Requested save slot not found in save set directory.
  #[error("Slot '{filename}' not found in save set at {dir}")]
  SlotNotFound {
    /// Save set directory path.
    dir: path::PathBuf,
    /// Slot filename.
    filename: String,
  },

  /// File has no tracked backup lineage.
  #[error("No backup lineage found for {path}")]
  NoLineageFound {
    /// Save file path lacking history.
    path: path::PathBuf,
  },

  /// Requested snapshot hash does not exist in manifest.
  #[error("Target snapshot {hash} not found in manifest")]
  SnapshotNotFound {
    /// Unrecognized snapshot SHA-256 hash.
    hash: String,
  },
}
