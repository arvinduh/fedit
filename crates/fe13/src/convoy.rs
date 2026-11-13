//! `TranBlock` convoy storage views and mutators.
//!
//! Owns views over the NART convoy storage subsystem block.

use crate::types;

/// Owned convoy storage inventory item.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConvoyItem {
  /// Item identifier.
  pub item_id: types::ItemId,
  /// Remaining uses or stacked quantity.
  pub uses: u16,
  /// Slot index within convoy storage.
  pub slot: usize,
}

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

  /// Returns all occupied convoy storage items.
  #[must_use]
  pub fn items(&self) -> Vec<ConvoyItem> {
    let mut items = Vec::new();
    let max_slots = self.raw.len().saturating_sub(8) / 4;
    for slot in 0..max_slots {
      let off = 8 + slot * 4;
      let id = u16::from_be_bytes([self.raw[off], self.raw[off + 1]]);
      let uses = u16::from_be_bytes([self.raw[off + 2], self.raw[off + 3]]);
      if id != 0 {
        items.push(ConvoyItem {
          item_id: types::ItemId(id),
          uses,
          slot,
        });
      }
    }
    items
  }
}

/// Mutable view over the NART convoy storage block.
#[derive(Debug)]
pub struct ConvoyMut<'a> {
  /// Mutable backing NART byte slice.
  raw: &'a mut [u8],
}

impl<'a> ConvoyMut<'a> {
  /// Wraps a mutable NART block byte slice.
  pub fn new(raw: &'a mut [u8]) -> Self {
    Self { raw }
  }

  /// Sets stored item quantity for an entry slot.
  pub fn set_item_quantity(&mut self, slot: usize, quantity: u16) {
    let off = 8 + slot * 4 + 2;
    if off + 2 <= self.raw.len() {
      self.raw[off..off + 2].copy_from_slice(&quantity.to_le_bytes());
    }
  }

  /// Sets an item and quantity in a convoy slot.
  pub fn set_item(&mut self, slot: usize, item_id: types::ItemId, uses: u16) {
    let off = 8 + slot * 4;
    if off + 4 <= self.raw.len() {
      self.raw[off..off + 2].copy_from_slice(&item_id.0.to_be_bytes());
      self.raw[off + 2..off + 4].copy_from_slice(&uses.to_be_bytes());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convoy_views() {
    let mut buf = [0u8; 32];
    buf[4..6].copy_from_slice(&2u16.to_le_bytes());
    let view = ConvoyRef::new(&buf);
    assert_eq!(view.len(), 2);
    assert!(!view.is_empty());

    let mut mutator = ConvoyMut::new(&mut buf);
    mutator.set_item_quantity(0, 42);
    assert_eq!(u16::from_le_bytes([buf[10], buf[11]]), 42);
  }
}
