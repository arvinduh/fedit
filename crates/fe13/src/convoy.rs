//! `TranBlock` convoy storage views and mutators.
//!
//! Owns views over the NART convoy storage subsystem block.

/// Borrowed view over the NART convoy storage block.
#[derive(Debug, Clone, Copy)]
pub struct ConvoyRef<'a> {
  /// Backing NART byte slice.
  raw: &'a [u8],
}

impl<'a> ConvoyRef<'a> {
  /// Wraps a NART block byte slice.
  #[must_use]
  pub fn new(raw: &'a [u8]) -> Self {
    Self { raw }
  }

  /// Number of stored convoy items.
  #[must_use]
  pub fn len(&self) -> usize {
    if self.raw.len() < 8 {
      0
    } else {
      u16::from_le_bytes([self.raw[4], self.raw[5]]) as usize
    }
  }

  /// Returns true if the convoy storage is empty.
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }
}
