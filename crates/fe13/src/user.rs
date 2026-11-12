//! `UserBlock` campaign state reader and mutator.
//!
//! Owns views over the RESU subsystem block in save payloads.

/// Maximum allowable campaign gold.
pub const MAX_GOLD: u32 = 999_999;

/// Byte offset of the campaign gold field relative to RESU start.
const GOLD_OFFSET: usize = 0x08;

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
