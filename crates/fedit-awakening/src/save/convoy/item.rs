//! Item domain enumerations, weapon classifications, and inventory items.
//!
//! Owns canonical weapon, staff, consumable, and seal definitions for FE13,
//! as well as item instances with durability data.

use thiserror;

pub(crate) mod catalog;
pub(crate) mod kind;

pub use catalog::ItemInfo;
pub use catalog::ItemType;
pub use kind::ItemKind;

/// Errors that can occur when manipulating items.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// Durability value exceeded maximum allowable bound (4095).
  #[error("durability {uses} exceeds safe limit {max}")]
  DurabilityOverflow {
    /// Provided uses.
    uses: u16,
    /// Maximum allowed uses.
    max: u16,
  },
}

/// An individual item within a unit's inventory or convoy storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Item {
  /// Base item variant enumeration.
  item_type: ItemType,
  /// Current remaining durability uses or stack count.
  durability: u16,
}

impl Item {
  /// Creates a new item with default canonical durability.
  #[must_use]
  pub fn new(item_type: ItemType) -> Self {
    let durability = u16::from(item_type.max_durability().unwrap_or(1));
    Self {
      item_type,
      durability,
    }
  }

  /// Creates an item with a specified durability uses count.
  ///
  /// # Errors
  ///
  /// Returns [`Error::DurabilityOverflow`] if `durability > 4095`.
  pub fn with_durability(
    item_type: ItemType,
    durability: u16,
  ) -> Result<Self, Error> {
    if durability > 4095 {
      return Err(Error::DurabilityOverflow {
        uses: durability,
        max: 4095,
      });
    }
    Ok(Self {
      item_type,
      durability,
    })
  }

  /// Returns the canonical 16-bit item identifier.
  #[must_use]
  pub const fn id(&self) -> u16 {
    self.item_type.id()
  }

  /// Returns the base item variant.
  #[must_use]
  pub const fn item_type(&self) -> ItemType {
    self.item_type
  }

  /// Returns the remaining durability uses count.
  #[must_use]
  pub const fn durability(&self) -> u16 {
    self.durability
  }

  /// Sets the base item variant.
  pub fn set_item_type(&mut self, item_type: ItemType) {
    self.item_type = item_type;
  }

  /// Sets the remaining durability uses count with bounds validation `[0..=4095]`.
  ///
  /// # Errors
  ///
  /// Returns [`Error::DurabilityOverflow`] if `durability > 4095`.
  pub fn set_durability(&mut self, durability: u16) -> Result<(), Error> {
    if durability > 4095 {
      return Err(Error::DurabilityOverflow {
        uses: durability,
        max: 4095,
      });
    }
    self.durability = durability;
    Ok(())
  }

  /// Repairs the item back to its canonical maximum durability.
  pub fn repair(&mut self) {
    if let Some(max) = self.item_type.max_durability() {
      self.durability = u16::from(max);
    }
  }

  /// Returns true if the weapon is broken (0 uses and not unbreakable).
  #[must_use]
  pub const fn is_broken(&self) -> bool {
    self.durability == 0 && !self.item_type.is_unbreakable()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_item_roundtrip_and_properties() {
    let falchion = ItemType::Falchion;
    assert_eq!(falchion.name(), "Falchion");
    assert_eq!(falchion.kind(), ItemKind::Sword);
    assert!(falchion.is_unbreakable());
    assert!(falchion.is_equippable());
    assert!(!falchion.is_forgeable());
    assert_eq!(ItemType::from_raw(falchion.raw_id()), falchion);

    let vuln = ItemType::Vulnerary;
    assert_eq!(vuln.name(), "Vulnerary");
    assert_eq!(vuln.kind(), ItemKind::Consumable);
    assert!(!vuln.is_equippable());
    assert!(!vuln.is_forgeable());
    assert_eq!(vuln.max_durability(), Some(3));
  }

  #[test]
  fn test_unknown_item_fallback() {
    let unknown = ItemType::from_raw(9999);
    assert_eq!(unknown, ItemType::Unknown(9999));
    assert_eq!(unknown.raw_id(), 9999);
    assert_eq!(unknown.name(), "Unknown Item");
    assert_eq!(unknown.kind(), ItemKind::Unknown);
    assert!(!unknown.is_equippable());
  }

  #[test]
  fn test_item_mutators() {
    let mut item = Item::new(ItemType::IronSword);
    assert_eq!(item.durability(), 40);
    assert!(item.set_durability(4095).is_ok());
    assert!(item.set_durability(4096).is_err());
    item.set_item_type(ItemType::BronzeSword);
    assert_eq!(item.item_type(), ItemType::BronzeSword);
  }
}
