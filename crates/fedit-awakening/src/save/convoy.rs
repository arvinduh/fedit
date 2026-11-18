//! `TranBlock` convoy storage views and mutators.
//!
//! Owns views over the NART convoy storage subsystem block.

use std::ops;
use std::slice;

use thiserror;

pub(crate) mod forge;
pub(crate) mod item;

pub use forge::ForgeTable;
pub use forge::ForgedWeapon;
pub use item::Error as ItemError;
pub use item::Item;
pub use item::ItemInfo;
pub use item::ItemKind;
pub use item::ItemType;

/// Maximum number of forged weapons tracked in the refinement array.
pub const MAX_FORGED_COUNT: usize = 150;

/// Header length of the NART block: 5 bytes magic + 2 bytes item count.
pub const NART_HEADER_LEN: usize = 7;

/// Errors that can occur during convoy storage operations.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// Slot was out of valid convoy bounds.
  #[error("convoy slot {slot} is out of bounds (max {max})")]
  SlotOutOfBounds {
    /// Provided slot index.
    slot: usize,
    /// Maximum allowed slot index.
    max: usize,
  },
  /// Item error.
  #[error(transparent)]
  Item(#[from] item::Error),
}

/// Encapsulated convoy storage inventory.
#[derive(Debug)]
pub struct Convoy {
  /// Active convoy items.
  items: Vec<Item>,
  /// Total regular item capacity in NART block before forges.
  regular_count: usize,
  /// Total item count recorded in NART header.
  total_count: usize,
}

impl Convoy {
  /// Parses occupied convoy storage items from a raw NART block byte slice.
  #[must_use]
  pub(crate) fn from_bytes(raw: &[u8]) -> Self {
    let mut items = Vec::new();
    if raw.len() < NART_HEADER_LEN {
      return Self {
        items,
        regular_count: 0,
        total_count: 0,
      };
    }
    let total_count = u16::from_le_bytes([raw[5], raw[6]]) as usize;
    let regular_count = total_count.saturating_sub(MAX_FORGED_COUNT);

    for id in 1..total_count {
      let off = NART_HEADER_LEN + id * 2;
      if off + 2 > raw.len() {
        break;
      }
      let uses = u16::from_le_bytes([raw[off], raw[off + 1]]);
      if uses > 0
        && let Ok(id_u16) = u16::try_from(id)
      {
        let item_type = ItemType::from_raw(id_u16);
        if let Ok(it) = Item::with_durability(item_type, uses) {
          items.push(it);
        }
      }
    }
    Self {
      items,
      regular_count,
      total_count,
    }
  }

  /// Serializes active convoy items into the provided raw NART block buffer.
  pub(crate) fn write_back(&self, buf: &mut [u8]) {
    for item in &self.items {
      let slot = usize::from(item.item_type().raw_id());
      let off = NART_HEADER_LEN + slot * 2;
      if off + 2 <= buf.len() {
        buf[off..off + 2].copy_from_slice(&item.durability().to_le_bytes());
      }
    }
  }

  /// Returns the total item capacity in the NART block header.
  #[must_use]
  pub const fn total_count(&self) -> usize {
    self.total_count
  }

  /// Returns the regular item capacity in the NART block.
  #[must_use]
  pub const fn regular_count(&self) -> usize {
    self.regular_count
  }

  /// Calculates the canonical 16-bit item identifier corresponding to a forged weapon position.
  ///
  /// # Errors
  ///
  /// Returns [`Error::SlotOutOfBounds`] if the position cannot fit in a `u16`.
  pub fn forged_item_id(&self, position: usize) -> Result<u16, Error> {
    let id = self.regular_count + position;
    u16::try_from(id).map_err(|_| Error::SlotOutOfBounds {
      slot: id,
      max: self.total_count,
    })
  }

  /// Returns true if a forged weapon at `position` is currently stored in convoy.
  #[must_use]
  pub fn contains_forged_weapon(&self, position: usize) -> bool {
    let Ok(id) = self.forged_item_id(position) else {
      return false;
    };
    self.find_by_id(id).is_some()
  }

  /// Number of active items in convoy storage.
  #[must_use]
  pub fn len(&self) -> usize {
    self.items.len()
  }

  /// Returns true if convoy storage is empty.
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }

  /// Returns a slice of all active items in convoy storage.
  #[must_use]
  pub fn items(&self) -> &[Item] {
    &self.items
  }

  /// Finds an active item in convoy by its 16-bit item identifier.
  #[must_use]
  pub fn find_by_id(&self, id: u16) -> Option<&Item> {
    self.items.iter().find(|it| it.item_type().raw_id() == id)
  }

  /// Finds the first item matching the given item type.
  #[must_use]
  pub fn find_by_type(&self, item_type: ItemType) -> Option<&Item> {
    self.items.iter().find(|it| it.item_type() == item_type)
  }

  /// Sets durability uses for an item with the given item ID.
  ///
  /// Setting `uses == 0` clears the item from active convoy storage.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::SlotOutOfBounds`] if `item_id >= self.total_count`.
  /// - Returns [`Error::Item`] if durability exceeds maximum capacity.
  pub fn set_durability(
    &mut self,
    item_id: usize,
    uses: u16,
  ) -> Result<(), Error> {
    if item_id >= self.total_count {
      return Err(Error::SlotOutOfBounds {
        slot: item_id,
        max: self.total_count,
      });
    }
    if let Some(pos) = self
      .items
      .iter()
      .position(|it| usize::from(it.item_type().raw_id()) == item_id)
    {
      if uses == 0 {
        self.items.remove(pos);
      } else {
        self.items[pos].set_durability(uses)?;
      }
    } else if uses > 0
      && let Ok(id_u16) = u16::try_from(item_id)
    {
      let item_type = ItemType::from_raw(id_u16);
      let item = Item::with_durability(item_type, uses)?;
      self.items.push(item);
      self.items.sort_by_key(|it| it.item_type().raw_id());
    }
    Ok(())
  }

  /// Adds a regular item to the convoy with specified uses.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::SlotOutOfBounds`] if `item_type.raw_id() >= self.regular_count`.
  /// - Returns [`Error::Item`] if durability overflows.
  pub fn add_item(
    &mut self,
    item_type: ItemType,
    uses: u16,
  ) -> Result<usize, Error> {
    let slot = usize::from(item_type.raw_id());
    self.set_durability(slot, uses)?;
    Ok(slot)
  }

  /// Returns an iterator over active items in convoy storage.
  pub fn iter(&self) -> slice::Iter<'_, Item> {
    self.items.iter()
  }
}

impl ops::Deref for Convoy {
  type Target = [Item];

  fn deref(&self) -> &Self::Target {
    &self.items
  }
}

impl ops::Index<usize> for Convoy {
  type Output = Item;

  fn index(&self, index: usize) -> &Self::Output {
    &self.items[index]
  }
}

impl<'a> IntoIterator for &'a Convoy {
  type Item = &'a Item;
  type IntoIter = slice::Iter<'a, Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convoy_views() {
    let mut buf = vec![0u8; NART_HEADER_LEN + 200 * 2];
    buf[0..4].copy_from_slice(b"NART");
    // item_count = 200 (50 regular + 150 forged)
    buf[5..7].copy_from_slice(&200u16.to_le_bytes());
    // Write 1 item at ID 1 (Bronze Sword), uses 50
    let off = NART_HEADER_LEN + 2;
    buf[off..off + 2].copy_from_slice(&50u16.to_le_bytes());
    let mut convoy = Convoy::from_bytes(&buf);
    assert_eq!(convoy.len(), 1);
    assert_eq!(convoy[0].item_type(), ItemType::BronzeSword);
    assert_eq!(convoy[0].durability(), 50);
    assert_eq!(convoy[0].id(), 1);
    assert!(convoy.find_by_id(1).is_some());

    // Modify durability
    assert!(convoy.set_durability(1, 40).is_ok());
    assert_eq!(convoy[0].durability(), 40);

    // Add another item
    assert!(convoy.add_item(ItemType::IronSword, 45).is_ok());
    assert_eq!(convoy.len(), 2);
    assert!(convoy.find_by_id(2).is_some());
  }
}
