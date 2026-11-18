//! `UserBlock` campaign state reader and mutator.
//!
//! Owns views over the RESU subsystem block in save payloads.

/// Maximum allowable campaign gold.
pub const MAX_GOLD: u32 = 999_999;

/// Byte offset of the play time field relative to RESU start.
const TIME_OFFSET: usize = 0x05;

/// West block baseline size.
const SIZE_BLOCK_WEST: usize = 0xEE;

/// Japanese block baseline size.
const SIZE_BLOCK_JP: usize = 0xEF;

/// Size per chapter in progress table.
const SIZE_MAP: usize = 0x10;

/// Calculates the offset of `rawBlock2` within the RESU block.
fn raw_block2_offset(raw: &[u8]) -> Option<usize> {
  if raw.len() < SIZE_BLOCK_WEST {
    return None;
  }
  let is_jp = raw.len() > 0x16
    && raw[0x15] as usize != (raw.len() - SIZE_BLOCK_JP) / SIZE_MAP
    && raw[0x16] as usize == (raw.len() - SIZE_BLOCK_JP) / SIZE_MAP;
  let (size_block, extra_byte) = if is_jp {
    (SIZE_BLOCK_JP, 1)
  } else {
    (SIZE_BLOCK_WEST, 0)
  };
  if raw.len() < size_block {
    return None;
  }
  let chapters = (raw.len() - size_block) / SIZE_MAP;
  let raw_block1_len = 0x10 + extra_byte;
  let chapters_len = chapters * SIZE_MAP + 1;
  let offset = 5 + raw_block1_len + chapters_len;
  if offset + 0x71 <= raw.len() {
    Some(offset)
  } else {
    None
  }
}

/// Borrowed view over the RESU campaign state block.
#[derive(Debug, Clone, Copy)]
pub struct UserRef<'a> {
  /// Backing RESU byte slice.
  raw: &'a [u8],
}

impl<'a> UserRef<'a> {
  /// Wraps a RESU block byte slice.
  #[must_use]
  pub fn new(raw: &'a [u8]) -> Self {
    Self { raw }
  }

  /// Total play time duration in frames.
  #[must_use]
  pub fn play_time(&self) -> u32 {
    if self.raw.len() < TIME_OFFSET + 4 {
      return 0;
    }
    u32::from_le_bytes([
      self.raw[TIME_OFFSET],
      self.raw[TIME_OFFSET + 1],
      self.raw[TIME_OFFSET + 2],
      self.raw[TIME_OFFSET + 3],
    ])
  }

  /// Campaign gold (money) amount.
  #[must_use]
  pub fn gold(&self) -> u32 {
    let Some(b2) = raw_block2_offset(self.raw) else {
      return 0;
    };
    let off = b2 + 0x69;
    if off + 4 <= self.raw.len() {
      u32::from_le_bytes([
        self.raw[off],
        self.raw[off + 1],
        self.raw[off + 2],
        self.raw[off + 3],
      ])
    } else {
      0
    }
  }

  /// Campaign renown points.
  #[must_use]
  pub fn renown(&self) -> u32 {
    let Some(b2) = raw_block2_offset(self.raw) else {
      return 0;
    };
    let off = b2 + 0x6D;
    if off + 4 <= self.raw.len() {
      u32::from_le_bytes([
        self.raw[off],
        self.raw[off + 1],
        self.raw[off + 2],
        self.raw[off + 3],
      ])
    } else {
      0
    }
  }

  /// Campaign difficulty level (0: Normal, 1: Hard, 2: Lunatic).
  #[must_use]
  pub fn difficulty(&self) -> u8 {
    let Some(b2) = raw_block2_offset(self.raw) else {
      return 0;
    };
    if b2 + 0x68 < self.raw.len() {
      self.raw[b2 + 0x68]
    } else {
      0
    }
  }

  /// Whether Lunatic+ mode is enabled.
  #[must_use]
  pub fn is_lunatic_plus(&self) -> bool {
    let Some(b2) = raw_block2_offset(self.raw) else {
      return false;
    };
    if b2 + 0x64 < self.raw.len() {
      (self.raw[b2 + 0x64] & 0x01) != 0
    } else {
      false
    }
  }

  /// Current campaign chapter index.
  #[must_use]
  pub fn current_chapter(&self) -> u8 {
    if self.raw.len() > 15 { self.raw[15] } else { 0 }
  }

  /// Count of total chapters beaten/unlocked.
  #[must_use]
  pub fn count_total_chapters(&self) -> u8 {
    if self.raw.len() > 13 { self.raw[13] } else { 0 }
  }

  /// Count of last story chapter beaten.
  #[must_use]
  pub fn count_last_chapter(&self) -> u8 {
    if self.raw.len() > 14 { self.raw[14] } else { 0 }
  }
}

/// Mutable view over the RESU campaign state block.
#[derive(Debug)]
pub struct UserMut<'a> {
  /// Mutable backing RESU byte slice.
  raw: &'a mut [u8],
}

impl<'a> UserMut<'a> {
  /// Wraps a mutable RESU block byte slice.
  pub fn new(raw: &'a mut [u8]) -> Self {
    Self { raw }
  }

  /// Sets current chapter index.
  pub fn set_current_chapter(&mut self, chapter: u8) {
    if self.raw.len() > 15 {
      self.raw[15] = chapter;
    }
  }

  /// Sets play time duration in frames.
  pub fn set_play_time(&mut self, frames: u32) {
    if self.raw.len() >= TIME_OFFSET + 4 {
      self.raw[TIME_OFFSET..TIME_OFFSET + 4]
        .copy_from_slice(&frames.to_le_bytes());
    }
  }

  /// Sets campaign gold clamped to [`MAX_GOLD`].
  pub fn set_gold(&mut self, gold: u32) {
    let Some(b2) = raw_block2_offset(self.raw) else {
      return;
    };
    let off = b2 + 0x69;
    if off + 4 <= self.raw.len() {
      let clamped = gold.min(MAX_GOLD);
      self.raw[off..off + 4].copy_from_slice(&clamped.to_le_bytes());
    }
  }

  /// Sets campaign renown points clamped to [`MAX_GOLD`].
  pub fn set_renown(&mut self, renown: u32) {
    let Some(b2) = raw_block2_offset(self.raw) else {
      return;
    };
    let off = b2 + 0x6D;
    if off + 4 <= self.raw.len() {
      let clamped = renown.min(MAX_GOLD);
      self.raw[off..off + 4].copy_from_slice(&clamped.to_le_bytes());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_gold_mutation() {
    let mut buf = vec![0u8; 0x250];
    {
      let mut u = UserMut::new(&mut buf);
      u.set_gold(50_000);
    }
    assert_eq!(UserRef::new(&buf).gold(), 50_000);
    {
      let mut u = UserMut::new(&mut buf);
      u.set_gold(2_000_000);
    }
    assert_eq!(UserRef::new(&buf).gold(), MAX_GOLD);
  }
}
