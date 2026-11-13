//! Error types for save archive management, backups, and persistence.
//!
//! Owns error variants for filesystem operations, archive validation, and codecs.

use std::io;
use std::path;

use fe_compress;
use fe13;
use thiserror;

/// Save archive operation error variants.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Underlying filesystem I/O failure.
  #[error(transparent)]
  Io(#[from] io::Error),

  /// Compression container decompression or compression failure.
  #[error(transparent)]
  Compress(#[from] fe_compress::Error),

  /// Save slot parsing or repacking failure.
  #[error(transparent)]
  Save(#[from] fe13::Error),

  /// The directory contains no recognizable save files.
  #[error("no valid save files discovered in directory '{path}'")]
  EmptyArchive {
    /// Path to the empty or invalid directory.
    path: path::PathBuf,
  },
}
