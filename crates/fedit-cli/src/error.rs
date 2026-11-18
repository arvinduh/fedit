//! CLI error types aggregating I/O and domain library errors.
//!
//! Maps low-level I/O failures, compression errors, and Fire Emblem save
//! validation errors into a unified CLI diagnostic type.

use std::io;

use awakening;
use fedit_compress;
use fedit_core;
use serde_json;
use thiserror;

/// Top-level error type for `fedit` CLI operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Standard I/O error occurred while reading or writing files.
  #[error(transparent)]
  Io(#[from] io::Error),
  /// Compression or container error occurred.
  #[error(transparent)]
  Compress(#[from] fedit_compress::Error),
  /// Fire Emblem domain or save validation error occurred.
  #[error(transparent)]
  Awakening(#[from] awakening::Error),
  /// Fire Emblem save campaign error occurred.
  #[error(transparent)]
  AwakeningSave(#[from] awakening::save::Error),
  /// Fire Emblem global save error occurred.
  #[error(transparent)]
  AwakeningGlobal(#[from] awakening::global::Error),
  /// Fire Emblem unit mutation error occurred.
  #[error(transparent)]
  AwakeningUnit(#[from] awakening::roster::UnitError),
  /// Fire Emblem stat mutation error occurred.
  #[error(transparent)]
  AwakeningStat(#[from] awakening::roster::StatError),
  /// Fire Emblem convoy item error occurred.
  #[error(transparent)]
  AwakeningItem(#[from] awakening::convoy::ItemError),
  /// Application core services error.
  #[error(transparent)]
  Core(#[from] fedit_core::Error),
  /// JSON serialization or deserialization error.
  #[error(transparent)]
  Json(#[from] serde_json::Error),
}
