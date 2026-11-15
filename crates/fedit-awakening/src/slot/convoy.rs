//! `TranBlock` convoy storage views and mutators.
//!
//! Owns views over the NART convoy storage subsystem block.

pub mod item;

pub use item::Item;
pub use item::ItemKind;
pub use item::ItemSlot;
pub use item::forge::Forge;

/// Owned convoy storage inventory item.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConvoyItem {
  /// Base item variant.
  pub item: Item,
  /// Remaining uses or stacked quantity.
  pub uses: u16,
  /// Slot index within convoy storage.
  pub slot: usize,
}

/// Borrowed view over the NART convoy storage block.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ConvoyRef<'a> {
  /// Backing NART byte slice.
  raw: &'a [u8],
}

impl<'a> ConvoyRef<'a> {
  /// Wraps a NART block byte slice.
  #[must_use]
  pub(crate) fn new(raw: &'a [u8]) -> Self {
    Self { raw }
  }

  /// Returns all occupied convoy storage items.
  #[must_use]
  pub(crate) fn items(&self) -> Vec<ConvoyItem> {
    let mut items = Vec::new();
    let max_slots = self.raw.len().saturating_sub(8) / 4;
    for slot in 0..max_slots {
      let off = 8 + slot * 4;
      let id = u16::from_be_bytes([self.raw[off], self.raw[off + 1]]);
      let uses = u16::from_be_bytes([self.raw[off + 2], self.raw[off + 3]]);
      if id != 0 {
        items.push(ConvoyItem {
          item: Item::from_raw(id),
          uses,
          slot,
        });
      }
    }
    items
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convoy_views() {
    let mut buf = [0u8; 16];
    // Write 1 item at slot 0: ID 1 (Bronze Sword), uses 50
    buf[8..10].copy_from_slice(&1u16.to_be_bytes());
    buf[10..12].copy_from_slice(&50u16.to_be_bytes());
    let view = ConvoyRef::new(&buf);
    let items = view.items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].item, Item::BronzeSword);
    assert_eq!(items[0].uses, 50);
  }
}
