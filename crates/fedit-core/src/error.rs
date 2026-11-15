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
