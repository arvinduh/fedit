//! PMOC archive header parsing and serialization.
//!
//! Owns the 16-byte PMOC layout. Higher layers handle container streams.

use crc32fast;

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

  /// Serializes header to a 16-byte array.
  #[must_use]
  pub fn to_bytes(&self) -> [u8; 16] {
    let mut b = [0u8; 16];
    b[..4].copy_from_slice(&MAGIC);
    b[4..8].copy_from_slice(&self.version.to_le_bytes());
    b[8..12].copy_from_slice(&self.decompressed_len.to_le_bytes());
    b[12..16].copy_from_slice(&self.crc32.to_le_bytes());
    b
  }
}

/// Computes IEEE 802.3 CRC-32 across preview slice and payload.
#[must_use]
pub fn compute_crc32(preview: &[u8], payload: &[u8]) -> u32 {
  let mut hasher = crc32fast::Hasher::new();
  hasher.update(preview);
  hasher.update(payload);
  hasher.finalize()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pmoc_header_roundtrip() {
    let header = Header {
      version: 2,
      decompressed_len: 12_298,
      crc32: 0xd370_51d6,
    };
    let bytes = header.to_bytes();
    let parsed = Header::parse(&bytes).expect("valid header");
    assert_eq!(parsed, header, "header roundtrip must match");
  }

  #[test]
  fn test_pmoc_invalid_magic_and_version() {
    assert!(
      Header::parse(b"PMOX000000000000").is_err(),
      "bad magic rejected"
    );
    let bad_ver = [b'P', b'M', b'O', b'C', 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(Header::parse(&bad_ver).is_err(), "version != 2 rejected");
  }

  #[test]
  fn test_compute_crc32() {
    let crc = compute_crc32(b"prev", b"load");
    assert_ne!(crc, 0, "crc should be computed");
  }
}
