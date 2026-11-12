//! PMOC archive container data structures.
//!
//! Owns container kind definitions and container payload structures.

use crate::error;
use crate::huffman;
use crate::pmoc;

/// Structural variant of a PMOC save container.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContainerKind {
  /// Slot save with 192-byte preview header.
  Slot,
  /// Global save without preview header.
  Global,
}

/// An unpacked PMOC archive container.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Container {
  /// The container kind.
  pub kind: ContainerKind,
  /// Preview header bytes (192 bytes for Slot, empty for Global).
  pub preview: Vec<u8>,
  /// Decompressed payload bytes.
  pub payload: Vec<u8>,
}

/// Unpacks a raw PMOC archive container.
///
/// # Errors
///
/// Fails on bad magic, unsupported version, or CRC mismatch.
pub fn unpack(raw: &[u8]) -> Result<Container, error::Error> {
  let (kind, p_off) = if raw.starts_with(&pmoc::MAGIC) {
    (ContainerKind::Global, 0)
  } else if raw.len() >= 0xD0 && raw[0xC0..0xC4] == pmoc::MAGIC {
    (ContainerKind::Slot, 0xC0)
  } else {
    return Err(error::Error::InvalidMagic);
  };
  let preview = &raw[..p_off];
  let header = pmoc::Header::parse(raw.get(p_off..p_off + 16).unwrap_or(&[]))?;
  let payload = huffman::decompress(raw.get(p_off + 16..).unwrap_or(&[]))?;
  let (expected, actual) =
    (header.crc32, pmoc::compute_crc32(preview, &payload));
  if expected != actual {
    return Err(error::Error::CrcMismatch { expected, actual });
  }
  Ok(Container {
    kind,
    preview: preview.to_vec(),
    payload,
  })
}
