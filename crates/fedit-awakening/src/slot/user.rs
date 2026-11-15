//! `UserBlock` campaign state reader and mutator.
//!
//! Owns views over the RESU subsystem block in save payloads.

/// Maximum allowable campaign gold.
pub const MAX_GOLD: u32 = 999_999;

/// Byte offset of the play time field relative to RESU start.
const TIME_OFFSET: usize = 0x05;

/// Byte offset of the campaign gold field relative to RESU start.
const GOLD_OFFSET: usize = 0x09;

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

  /// Total play time duration in seconds.
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

  /// Campaign gold amount.
  #[must_use]
  pub fn gold(&self) -> u32 {
    if self.raw.len() < GOLD_OFFSET + 4 {
      return 0;
    }
    u32::from_le_bytes([
      self.raw[GOLD_OFFSET],
      self.raw[GOLD_OFFSET + 1],
      self.raw[GOLD_OFFSET + 2],
      self.raw[GOLD_OFFSET + 3],
    ])
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

  /// Sets play time duration in seconds.
  pub fn set_play_time(&mut self, secs: u32) {
    if self.raw.len() >= TIME_OFFSET + 4 {
      self.raw[TIME_OFFSET..TIME_OFFSET + 4]
        .copy_from_slice(&secs.to_le_bytes());
    }
  }

  /// Sets campaign gold clamped to [`MAX_GOLD`].
  pub fn set_gold(&mut self, gold: u32) {
    if self.raw.len() >= GOLD_OFFSET + 4 {
      let clamped = gold.min(MAX_GOLD);
      self.raw[GOLD_OFFSET..GOLD_OFFSET + 4]
        .copy_from_slice(&clamped.to_le_bytes());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_gold_mutation() {
    let mut buf = [0u8; 16];
    let mut user = UserMut::new(&mut buf);
    user.set_gold(1_500_000);
    let view = UserRef::new(&buf);
    assert_eq!(view.gold(), MAX_GOLD);
    let mut user = UserMut::new(&mut buf);
    user.set_gold(42_000);
    let view = UserRef::new(&buf);
    assert_eq!(view.gold(), 42_000);
  }
}
