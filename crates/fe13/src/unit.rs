//! `UnitBlock` character slot views and iterators.
//!
//! Owns views over character entries in the TINU roster block.

use crate::types;

/// Borrowed view over an individual character unit slot.
#[derive(Debug, Clone, Copy)]
pub struct UnitRef<'a> {
  /// Backing unit entry byte slice.
  raw: &'a [u8],
}

impl<'a> UnitRef<'a> {
  /// Wraps a unit entry byte slice.
  #[must_use]
  pub fn new(raw: &'a [u8]) -> Self {
    Self { raw }
  }

  /// Character unit identifier.
  #[must_use]
  pub fn unit_id(&self) -> types::UnitId {
    types::UnitId(u16::from_le_bytes([self.raw[0], self.raw[1]]))
  }

  /// Current class identifier.
  #[must_use]
  pub fn class_id(&self) -> types::ClassId {
    types::ClassId(self.raw[2])
  }

  /// Maximum hit points stat.
  #[must_use]
  pub fn hp(&self) -> u8 {
    self.raw[8]
  }

  /// Strength stat.
  #[must_use]
  pub fn strength(&self) -> u8 {
    self.raw[9]
  }

  /// Magic stat.
  #[must_use]
  pub fn magic(&self) -> u8 {
    self.raw[10]
  }

  /// Skill stat.
  #[must_use]
  pub fn skill(&self) -> u8 {
    self.raw[11]
  }

  /// Speed stat.
  #[must_use]
  pub fn speed(&self) -> u8 {
    self.raw[12]
  }

  /// Luck stat.
  #[must_use]
  pub fn luck(&self) -> u8 {
    self.raw[13]
  }

  /// Defense stat.
  #[must_use]
  pub fn defense(&self) -> u8 {
    self.raw[14]
  }

  /// Resistance stat.
  #[must_use]
  pub fn resistance(&self) -> u8 {
    self.raw[15]
  }
}
