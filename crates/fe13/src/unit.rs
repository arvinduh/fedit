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

  /// Inventory item identifier at slot (0..5).
  #[must_use]
  pub fn item(&self, slot: usize) -> Option<types::ItemId> {
    if slot >= 5 || self.raw.len() < 16 + (slot + 1) * 2 {
      return None;
    }
    let id =
      u16::from_le_bytes([self.raw[16 + slot * 2], self.raw[17 + slot * 2]]);
    if id == 0xffff {
      None
    } else {
      Some(types::ItemId(id))
    }
  }
}

/// Iterator over character units within a TINU block.
#[derive(Debug, Clone)]
pub struct UnitIter<'a> {
  /// Backing TINU byte slice.
  raw: &'a [u8],
  /// Current search byte offset.
  pos: usize,
}

impl<'a> UnitIter<'a> {
  /// Creates a new unit iterator over a TINU block slice.
  #[must_use]
  pub fn new(raw: &'a [u8]) -> Self {
    Self { raw, pos: 0 }
  }
}

impl<'a> Iterator for UnitIter<'a> {
  type Item = UnitRef<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    while self.pos + 32 <= self.raw.len() {
      let i = self.pos;
      let uid = u16::from_le_bytes([self.raw[i], self.raw[i + 1]]);
      let cid = self.raw[i + 2];
      let hp = self.raw[i + 8];
      let has_empty = (0..5).any(|s| {
        let it = u16::from_le_bytes([
          self.raw[i + 16 + s * 2],
          self.raw[i + 17 + s * 2],
        ]);
        it == 0xffff
      });
      if (1..=60).contains(&uid)
        && (1..=80).contains(&cid)
        && (10..=99).contains(&hp)
        && has_empty
      {
        self.pos += 180;
        return Some(UnitRef::new(&self.raw[i..]));
      }
      self.pos += 1;
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_unit_ref_accessors() {
    let mut buf = [0u8; 32];
    buf[0..2].copy_from_slice(&1u16.to_le_bytes());
    buf[2] = 7;
    buf[8] = 45;
    buf[9] = 28;
    buf[10] = 24;
    buf[16..18].copy_from_slice(&0x0244u16.to_le_bytes());
    buf[18..20].copy_from_slice(&0xffffu16.to_le_bytes());
    let unit = UnitRef::new(&buf);
    assert_eq!(unit.unit_id(), types::UnitId(1));
    assert_eq!(unit.class_id(), types::ClassId(7));
    assert_eq!(unit.hp(), 45);
    assert_eq!(unit.strength(), 28);
    assert_eq!(unit.magic(), 24);
    assert_eq!(unit.item(0), Some(types::ItemId(0x0244)));
    assert_eq!(unit.item(1), None);
  }

  #[test]
  fn test_unit_iter() {
    let mut buf = vec![0u8; 250];
    buf[0..2].copy_from_slice(&1u16.to_le_bytes());
    buf[2] = 7;
    buf[8] = 45;
    buf[16..18].copy_from_slice(&0xffffu16.to_le_bytes());
    let units: Vec<_> = UnitIter::new(&buf).collect();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].unit_id(), types::UnitId(1));
  }
}
