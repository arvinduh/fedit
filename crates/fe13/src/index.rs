//! EDNI master block index parser.
//!
//! Resolves block offsets for subsystem data within the save payload.

/// Preview offset subtracted from table entries.
pub const PREVIEW_OFFSET: usize = 0xC0;

/// Block offsets within a decompressed save payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
  pub fn parse(payload: &[u8]) -> Result<Self, crate::error::Error> {
    if payload.len() < 0x20 || &payload[0..4] != b"EDNI" {
      return Err(crate::error::Error::InvalidEdniMagic);
    }
    let r = |idx: usize, exp: [u8; 4]| -> Result<usize, crate::error::Error> {
      let b = &payload[idx * 4..idx * 4 + 4];
      let raw = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize;
      let off = raw.checked_sub(PREVIEW_OFFSET).ok_or(
        crate::error::Error::OffsetOutOfBounds {
          offset: raw,
          len: payload.len(),
        },
      )?;
      if off + 4 > payload.len() {
        return Err(crate::error::Error::OffsetOutOfBounds {
          offset: off,
          len: payload.len(),
        });
      }
      let mut act = [0u8; 4];
      act.copy_from_slice(&payload[off..off + 4]);
      if act != exp {
        return Err(crate::error::Error::InvalidBlockMagic {
          expected: exp,
          actual: act,
        });
      }
      Ok(off)
    };
    Ok(Self {
      user_offset: r(1, *b"RESU")?,
      pamg_offset: r(2, *b"PAMG")?,
      unit_offset: r(3, *b"TINU")?,
      ifer_offset: r(4, *b"IFER")?,
      convoy_offset: r(5, *b"NART")?,
      d62_offset: r(6, *b"62UD")?,
      tsve_offset: r(7, *b"TSVE")?,
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
      (0x110, b"PAMG"),
      (0x120, b"TINU"),
      (0x130, b"IFER"),
      (0x140, b"NART"),
      (0x150, b"62UD"),
      (0x160, b"TSVE"),
    ];
    for (i, &(raw_off, magic)) in offsets.iter().enumerate() {
      let b = raw_off.to_le_bytes();
      payload[(i + 1) * 4..(i + 1) * 4 + 4].copy_from_slice(&b);
      let payload_off = (raw_off as usize) - PREVIEW_OFFSET;
      payload[payload_off..payload_off + 4].copy_from_slice(magic);
    }
    let index = EdniIndex::parse(&payload).expect("valid index");
    assert_eq!(index.user_offset, 0x104 - PREVIEW_OFFSET);
    assert_eq!(index.pamg_offset, 0x110 - PREVIEW_OFFSET);
    assert_eq!(index.unit_offset, 0x120 - PREVIEW_OFFSET);
    assert_eq!(index.ifer_offset, 0x130 - PREVIEW_OFFSET);
    assert_eq!(index.convoy_offset, 0x140 - PREVIEW_OFFSET);
    assert_eq!(index.d62_offset, 0x150 - PREVIEW_OFFSET);
    assert_eq!(index.tsve_offset, 0x160 - PREVIEW_OFFSET);
  }

  #[test]
  fn test_edni_invalid_magic() {
    let payload = b"BADM0000000000000000000000000000";
    assert!(matches!(
      EdniIndex::parse(payload),
      Err(crate::error::Error::InvalidEdniMagic)
    ));
  }
}
