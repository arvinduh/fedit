//! Error types for Fire Emblem: Awakening save domain operations.
//!
//! Owns the crate-wide [`enum@Error`] type.

use fe_compress;
use thiserror;

/// Errors that can occur when parsing or mutating FE13 saves.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Preview buffer length was not 192 bytes.
  #[error("invalid preview length: expected {expected}, got {actual}")]
  InvalidPreviewLength {
    /// Expected byte length (192).
    expected: usize,
    /// Actual byte length.
    actual: usize,
  },
  /// EDNI master table magic did not match `b"EDNI"`.
  #[error("invalid EDNI master index magic")]
  InvalidEdniMagic,
  /// Target block magic tag did not match expected ASCII bytes.
  #[error("invalid block magic: expected {expected:?}, got {actual:?}")]
  InvalidBlockMagic {
    /// Expected 4-byte ASCII magic tag.
    expected: [u8; 4],
    /// Actual 4-byte tag found at offset.
    actual: [u8; 4],
  },
  /// Block offset was out of bounds for the decompressed payload.
  #[error("offset {offset} is out of bounds for payload length {len}")]
  OffsetOutOfBounds {
    /// Resolved offset.
    offset: usize,
    /// Payload length.
    len: usize,
  },
  /// Fixed-width UTF-16LE string decoding failed.
  #[error("failed to decode UTF-16LE string")]
  Utf16Decode,
  /// Lower-level container or compression error.
  #[error(transparent)]
  Compress(#[from] fe_compress::Error),
}
