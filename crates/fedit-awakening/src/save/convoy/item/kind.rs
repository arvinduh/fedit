//! Item category and weapon proficiency classification.
//!
//! Classifies items into weapon types, staves, stones, consumables, and seals
//! to determine unit equipment eligibility and combat proficiencies.

/// Category of an item determining combat proficiencies and equipment rules.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ItemKind {
  /// Swords and katanas.
  Sword,
  /// Lances and naginatas.
  Lance,
  /// Axes and clubs.
  Axe,
  /// Bows and yumis.
  Bow,
  /// Elemental, light, and dark magic tomes.
  Tome,
  /// Healing and utility staves.
  Staff,
  /// Dragonstones and beaststones.
  Stone,
  /// Healing and restorative consumables.
  Consumable,
  /// Class progression and reclassing seals.
  Seal,
  /// Permanent stat enhancement items.
  StatBooster,
  /// Valuables and bullion.
  Valuable,
  /// Unrecognized or generic item category.
  Unknown,
}

impl ItemKind {
  /// Returns true if this item kind can be equipped into a unit's active hand.
  #[must_use]
  pub const fn is_equippable(&self) -> bool {
    match self {
      Self::Sword
      | Self::Lance
      | Self::Axe
      | Self::Bow
      | Self::Tome
      | Self::Staff
      | Self::Stone => true,
      Self::Consumable
      | Self::Seal
      | Self::StatBooster
      | Self::Valuable
      | Self::Unknown => false,
    }
  }
}
