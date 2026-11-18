//! EDNI master block index parser.
//!
//! Resolves block offsets for subsystem data within the save payload.

use thiserror;

/// Preview offset subtracted from table entries.
pub const PREVIEW_OFFSET: usize = 0xC0;

/// Errors that can occur when parsing the EDNI index table.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// EDNI master table magic did not match `b"EDNI"`.
  #[error("invalid EDNI master index magic")]
  InvalidMagic,
  /// Block offset was out of bounds for the decompressed payload.
  #[error("offset {offset} is out of bounds for payload length {len}")]
  OffsetOutOfBounds {
    /// Resolved offset.
    offset: usize,
    /// Payload length.
    len: usize,
  },
}

/// Block offsets within a decompressed save payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(
  clippy::struct_field_names,
  reason = "Explicit offset naming mirrors raw EDNI table offsets"
)]
pub struct EdniIndex {
  /// RESU user campaign state offset.
  pub user_offset: usize,
  /// PAMG game state offset.
  pub pamg_offset: usize,
  /// TINU unit roster offset.
  pub unit_offset: usize,
  /// IFER block offset.
  pub ifer_offset: usize,
  /// NART convoy storage offset.
  pub convoy_offset: usize,
  /// Convoy storage buffer end offset.
  pub convoy_end_offset: usize,
  /// 62UD support table offset.
  pub d62_offset: usize,
  /// TSVE event state offset.
  pub tsve_offset: usize,
}

impl EdniIndex {
  /// Parses the EDNI index table from a decompressed save payload.
  ///
  /// # Errors
  ///
  /// Returns an error if magic is invalid or offsets exceed bounds.
  pub fn from_bytes(payload: &[u8]) -> Result<Self, Error> {
    if payload.len() < 0x20 || &payload[0..4] != b"EDNI" {
      return Err(Error::InvalidMagic);
    }
    let (mut user, mut pamg, mut unit, mut ifer) = (None, None, None, None);
    let (mut convoy, mut convoy_end, mut d62, mut tsve) =
      (None, None, None, None);
    let mut idx = 1;
    while idx * 4 + 4 <= payload.len() {
      let b = &payload[idx * 4..idx * 4 + 4];
      let raw = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize;
      if raw == 0 {
        break;
      }
      let off =
        raw
          .checked_sub(PREVIEW_OFFSET)
          .ok_or(Error::OffsetOutOfBounds {
            offset: raw,
            len: payload.len(),
          })?;
      if off + 4 > payload.len() {
        return Err(Error::OffsetOutOfBounds {
          offset: off,
          len: payload.len(),
        });
      }
      if convoy.is_some() && convoy_end.is_none() {
        convoy_end = Some(off);
      }
      match &payload[off..off + 4] {
        b"RESU" => user = Some(off),
        b"PAMG" => pamg = Some(off),
        b"TINU" => unit = Some(off),
        b"IFER" => ifer = Some(off),
        b"NART" => convoy = Some(off),
        b"62UD" => d62 = Some(off),
        b"TSVE" => tsve = Some(off),
        _ => {}
      }
      idx += 1;
    }
    Ok(Self {
      user_offset: user.ok_or(Error::InvalidMagic)?,
      pamg_offset: pamg.ok_or(Error::InvalidMagic)?,
      unit_offset: unit.ok_or(Error::InvalidMagic)?,
      ifer_offset: ifer.ok_or(Error::InvalidMagic)?,
      convoy_offset: convoy.ok_or(Error::InvalidMagic)?,
      convoy_end_offset: convoy_end.unwrap_or(0),
      d62_offset: d62.ok_or(Error::InvalidMagic)?,
      tsve_offset: tsve.ok_or(Error::InvalidMagic)?,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_edni_index_parse() {
    let mut payload = vec![0u8; 300];
    payload[0..4].copy_from_slice(b"EDNI");
    let offsets: [(u32, &[u8; 4]); 7] = [
      (0x104, b"RESU"),
      (0x108, b"PAMG"),
      (0x10C, b"TINU"),
      (0x110, b"IFER"),
      (0x114, b"NART"),
      (0x118, b"62UD"),
      (0x11C, b"TSVE"),
    ];
    for (i, (off, tag)) in offsets.iter().enumerate() {
      let entry_off = (i + 1) * 4;
      payload[entry_off..entry_off + 4].copy_from_slice(&off.to_le_bytes());
      let body_off = (*off as usize) - PREVIEW_OFFSET;
      payload[body_off..body_off + 4].copy_from_slice(*tag);
    }
    let idx = EdniIndex::from_bytes(&payload).expect("parse edni index");
    assert_eq!(idx.user_offset, 0x104 - PREVIEW_OFFSET);
    assert_eq!(idx.pamg_offset, 0x108 - PREVIEW_OFFSET);
    assert_eq!(idx.unit_offset, 0x10C - PREVIEW_OFFSET);
    assert_eq!(idx.ifer_offset, 0x110 - PREVIEW_OFFSET);
    assert_eq!(idx.convoy_offset, 0x114 - PREVIEW_OFFSET);
    assert_eq!(idx.d62_offset, 0x118 - PREVIEW_OFFSET);
    assert_eq!(idx.tsve_offset, 0x11C - PREVIEW_OFFSET);
  }

  #[test]
  fn test_edni_invalid_magic() {
    let payload = vec![0u8; 300];
    let res = EdniIndex::from_bytes(&payload);
    assert_eq!(res, Err(Error::InvalidMagic));
  }
}
