//! Container error definitions.
//!
//! Owns [`enum@Error`] for archive decompression and verification.

use thiserror;

/// Archive decompression and verification errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Magic bytes did not match the expected container signature.
  #[error("invalid container magic signature")]
  InvalidMagic,
  /// Container version is not supported by this codec.
  #[error("unsupported container version: {version}")]
  UnsupportedVersion {
    /// Encountered version.
    version: u32,
  },
  /// Leading byte was not 0x28.
  #[error("invalid huffman stream header byte: {byte:#04x}")]
  InvalidHuffmanHeader {
    /// Encountered byte.
    byte: u8,
  },
  /// Huffman tree table navigation exceeded bounds.
  #[error("corrupt huffman tree structure")]
  CorruptHuffmanTree,
  /// Uncompressed payload length did not match header.
  #[error("decompressed length mismatch: expected {expected}, got {actual}")]
  LengthMismatch {
    /// Expected length.
    expected: usize,
    /// Actual length.
    actual: usize,
  },
  /// CRC-32 checksum did not match header.
  #[error("crc32 mismatch: expected {expected:#010x}, got {actual:#010x}")]
  CrcMismatch {
    /// Expected CRC-32 checksum.
    expected: u32,
    /// Actual CRC-32 checksum.
    actual: u32,
  },
}
