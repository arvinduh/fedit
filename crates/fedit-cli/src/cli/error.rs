//! CLI error types aggregating I/O and domain library errors.
//!
//! Maps low-level I/O failures, compression errors, and Fire Emblem save
//! validation errors into a unified CLI diagnostic type.

use std::io;

use fe_compress;
use fe13;
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
  Compress(#[from] fe_compress::Error),
  /// Fire Emblem domain or save validation error occurred.
  #[error(transparent)]
  Fe13(#[from] fe13::Error),
  /// JSON serialization or deserialization error.
  #[error(transparent)]
  Json(#[from] serde_json::Error),
}
