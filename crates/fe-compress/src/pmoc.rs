//! PMOC archive header parsing and serialization.
//!
//! Owns the 16-byte PMOC layout. Higher layers handle container streams.

use crate::error;

/// PMOC archive magic signature.
pub const MAGIC: [u8; 4] = *b"PMOC";

/// A 16-byte PMOC container archive header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Header {
  /// Format version (must be 2).
  pub version: u32,
  /// Decompressed payload length.
  pub decompressed_len: u32,
  /// IEEE 802.3 CRC-32 checksum.
  pub crc32: u32,
}

impl Header {
  /// Parses a 16-byte PMOC header.
  ///
  /// # Errors
  ///
  /// Fails on short slice, bad magic, or version != 2.
  pub fn parse(b: &[u8]) -> Result<Self, error::Error> {
    if b.len() < 16 || b[..4] != MAGIC {
      return Err(error::Error::InvalidMagic);
    }
    let version = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);
    if version != 2 {
      return Err(error::Error::UnsupportedVersion { version });
    }
    Ok(Self {
      version,
      decompressed_len: u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
      crc32: u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
    })
  }
}
