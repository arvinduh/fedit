//! Item domain enumerations, weapon classifications, and inventory slots.
//!
//! Owns canonical weapon, staff, consumable, and seal definitions for FE13,
//! as well as item slot instances with durability and forging data.

pub mod catalog;
pub mod forge;
pub mod kind;
pub mod slot;

pub use catalog::Item;
pub use forge::Forge;
pub use kind::ItemKind;
pub use slot::ItemSlot;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_item_roundtrip_and_properties() {
    let falchion = Item::Falchion;
    assert_eq!(falchion.name(), "Falchion");
    assert_eq!(falchion.kind(), ItemKind::Sword);
    assert!(falchion.is_unbreakable());
    assert!(falchion.is_equippable());
    assert_eq!(Item::from_raw(falchion.raw_id()), falchion);

    let vuln = Item::Vulnerary;
    assert_eq!(vuln.name(), "Vulnerary");
    assert_eq!(vuln.kind(), ItemKind::Consumable);
    assert!(!vuln.is_equippable());
    assert_eq!(vuln.max_durability(), Some(3));
  }

  #[test]
  fn test_unknown_item_fallback() {
    let unknown = Item::from_raw(9999);
    assert_eq!(unknown, Item::Unknown(9999));
    assert_eq!(unknown.raw_id(), 9999);
    assert_eq!(unknown.name(), "Unknown Item");
    assert_eq!(unknown.kind(), ItemKind::Unknown);
    assert!(!unknown.is_equippable());
  }
}
