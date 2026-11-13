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
  pub fn parse(payload: &[u8]) -> Result<Self, crate::error::Error> {
    if payload.len() < 0x20 || &payload[0..4] != b"EDNI" {
      return Err(crate::error::Error::InvalidEdniMagic);
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
      user_offset: user.ok_or(crate::error::Error::InvalidEdniMagic)?,
      pamg_offset: pamg.ok_or(crate::error::Error::InvalidEdniMagic)?,
      unit_offset: unit.ok_or(crate::error::Error::InvalidEdniMagic)?,
      ifer_offset: ifer.ok_or(crate::error::Error::InvalidEdniMagic)?,
      convoy_offset: convoy.ok_or(crate::error::Error::InvalidEdniMagic)?,
      convoy_end_offset: convoy_end.unwrap_or(0),
      d62_offset: d62.ok_or(crate::error::Error::InvalidEdniMagic)?,
      tsve_offset: tsve.ok_or(crate::error::Error::InvalidEdniMagic)?,
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
