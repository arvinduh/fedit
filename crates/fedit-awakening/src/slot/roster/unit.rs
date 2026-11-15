//! `UnitBlock` character slot views and iterators.
//!
//! Owns views over character entries in the TINU roster block.

use crate::error;
use crate::slot::convoy::item;

pub mod character;
pub mod class;

/// Operational combat and deployment status of a unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnitStatus {
  /// Unit is actively deployed and available for battle.
  Active,
  /// Unit was downed in battle and retreated from combat.
  Retreated,
  /// Unit fell in battle on Classic mode (permadeath).
  Dead,
}

/// Combat attributes and character statistics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UnitStats {
  /// Maximum hit points.
  pub hp: u8,
  /// Physical strength stat.
  pub strength: u8,
  /// Magic stat.
  pub magic: u8,
  /// Skill stat.
  pub skill: u8,
  /// Speed stat.
  pub speed: u8,
  /// Luck stat.
  pub luck: u8,
  /// Physical defense stat.
  pub defense: u8,
  /// Magical resistance stat.
  pub resistance: u8,
}

impl UnitStats {
  /// Sets hit points with bounds checking `[10..=99]`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::StatOutOfBounds`] if `val` is outside `[10, 99]`.
  pub fn set_hp(&mut self, val: u8) -> Result<(), error::Error> {
    if !(10..=99).contains(&val) {
      return Err(error::Error::StatOutOfBounds {
        stat: "hp",
        value: val,
        min: 10,
        max: 99,
      });
    }
    self.hp = val;
    Ok(())
  }

  /// Sets combat stat value with upper bound validation `(0..=80)`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::StatOutOfBounds`] if `val > 80`.
  pub fn set_combat_stat(
    &mut self,
    stat: &'static str,
    val: u8,
  ) -> Result<(), error::Error> {
    if val > 80 {
      return Err(error::Error::StatOutOfBounds {
        stat,
        value: val,
        min: 0,
        max: 80,
      });
    }
    match stat {
      "strength" => self.strength = val,
      "magic" => self.magic = val,
      "skill" => self.skill = val,
      "speed" => self.speed = val,
      "luck" => self.luck = val,
      "defense" => self.defense = val,
      "resistance" => self.resistance = val,
      _ => {}
    }
    Ok(())
  }
}

/// Owned character unit domain representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unit {
  /// Canonical character enumeration.
  character: character::Character,
  /// Unit class enumeration.
  class: class::Class,
  /// Character combat attributes.
  pub stats: UnitStats,
  /// Personal inventory equipment slots (up to 5 items).
  inventory: [Option<item::ItemSlot>; 5],
  /// Index of the currently equipped weapon slot (0..5), or None if unarmed.
  equipped_slot: Option<usize>,
  /// Operational deployment status.
  status: UnitStatus,
  /// Byte offset of this unit entry within the TINU block.
  offset: usize,
}

impl Unit {
  /// Constructs a new unit with default empty inventory and active status.
  #[must_use]
  pub fn new(
    character: character::Character,
    class: class::Class,
    stats: UnitStats,
    offset: usize,
  ) -> Self {
    Self {
      character,
      class,
      stats,
      inventory: [None, None, None, None, None],
      equipped_slot: None,
      status: UnitStatus::Active,
      offset,
    }
  }

  /// Constructs an owned unit from a borrowed view and block offset.
  #[must_use]
  pub(crate) fn from_ref(offset: usize, r: &UnitRef<'_>) -> Self {
    Self {
      character: r.character(),
      class: r.class(),
      stats: UnitStats {
        hp: r.hp(),
        strength: r.strength(),
        magic: r.magic(),
        skill: r.skill(),
        speed: r.speed(),
        luck: r.luck(),
        defense: r.defense(),
        resistance: r.resistance(),
      },
      inventory: [r.item(0), r.item(1), r.item(2), r.item(3), r.item(4)],
      equipped_slot: r.equipped_slot(),
      status: r.status(),
      offset,
    }
  }

  /// Returns the byte offset of this unit entry within the TINU block.
  #[must_use]
  pub(crate) const fn offset(&self) -> usize {
    self.offset
  }

  /// Returns the canonical character identity.
  #[must_use]
  pub const fn character(&self) -> character::Character {
    self.character
  }

  /// Returns the canonical character display name.
  #[must_use]
  pub fn name(&self) -> &'static str {
    self.character.name()
  }

  /// Returns true if this unit is an essential story anchor.
  #[must_use]
  pub fn is_story_anchor(&self) -> bool {
    self.character.is_story_anchor()
  }

  /// Returns the current class.
  #[must_use]
  pub const fn class(&self) -> class::Class {
    self.class
  }

  /// Changes the unit's class.
  ///
  /// Automatically unequips the currently active weapon if the new class
  /// is not proficient in using it, preventing battle animation crashes.
  pub fn set_class(&mut self, class: class::Class) {
    self.class = class;
    if let Some(eq_slot) = self.equipped_slot
      && let Some(item_slot) = &self.inventory[eq_slot]
      && !class.can_use(item_slot.item().kind())
    {
      self.equipped_slot = None;
    }
  }

  /// Returns a borrowed slice of the 5 inventory item slots.
  #[must_use]
  pub const fn inventory(&self) -> &[Option<item::ItemSlot>; 5] {
    &self.inventory
  }

  /// Returns a reference to the item slot at index `slot` (0..5).
  #[must_use]
  pub fn item(&self, slot: usize) -> Option<&item::ItemSlot> {
    if slot < 5 {
      self.inventory[slot].as_ref()
    } else {
      None
    }
  }

  /// Returns a mutable reference to the item slot at index `slot` (0..5).
  pub fn item_mut(&mut self, slot: usize) -> Option<&mut item::ItemSlot> {
    if slot < 5 {
      self.inventory[slot].as_mut()
    } else {
      None
    }
  }

  /// Returns the index of the currently equipped inventory slot, if any.
  #[must_use]
  pub const fn equipped_slot(&self) -> Option<usize> {
    self.equipped_slot
  }

  /// Returns true if the specified slot is the currently equipped weapon.
  #[must_use]
  pub fn is_slot_equipped(&self, slot: usize) -> bool {
    self.equipped_slot == Some(slot)
  }

  /// Equips the weapon in the specified inventory slot (0..5).
  ///
  /// Guarantees that at most one item is equipped at any time.
  ///
  /// # Errors
  ///
  /// - Returns [`error::Error::SlotOutOfBounds`] if `slot >= 5`.
  /// - Returns [`error::Error::EmptySlot`] if the slot has no item.
  /// - Returns [`error::Error::CannotEquipNonWeapon`] if the item is not equippable.
  /// - Returns [`error::Error::ClassCannotEquip`] if the unit's class cannot use it.
  pub fn equip(&mut self, slot: usize) -> Result<(), error::Error> {
    if slot >= 5 {
      return Err(error::Error::SlotOutOfBounds { slot });
    }
    let Some(item_slot) = &self.inventory[slot] else {
      return Err(error::Error::EmptySlot { slot });
    };
    if !item_slot.item().is_equippable() {
      return Err(error::Error::CannotEquipNonWeapon {
        item: item_slot.item(),
      });
    }
    if !self.class.can_use(item_slot.item().kind()) {
      return Err(error::Error::ClassCannotEquip {
        class: self.class,
        item: item_slot.item(),
      });
    }
    self.equipped_slot = Some(slot);
    Ok(())
  }

  /// Unequips the currently active weapon.
  pub fn unequip(&mut self) {
    self.equipped_slot = None;
  }

  /// Sets an item and its remaining uses in a specific inventory slot (0..5).
  ///
  /// # Errors
  ///
  /// - Returns [`error::Error::SlotOutOfBounds`] if `slot >= 5`.
  /// - Returns [`error::Error::DurabilityOverflow`] if `uses > 4095`.
  pub fn set_item(
    &mut self,
    slot: usize,
    item: item::Item,
    uses: u16,
  ) -> Result<(), error::Error> {
    if slot >= 5 {
      return Err(error::Error::SlotOutOfBounds { slot });
    }
    let slot_entry = item::ItemSlot::with_uses(item, uses)?;
    self.inventory[slot] = Some(slot_entry);
    if self.equipped_slot == Some(slot)
      && (!item.is_equippable() || !self.class.can_use(item.kind()))
    {
      self.equipped_slot = None;
    }
    Ok(())
  }

  /// Clears the item in the specified inventory slot (0..5).
  pub fn clear_item(&mut self, slot: usize) {
    if slot < 5 {
      self.inventory[slot] = None;
      if self.equipped_slot == Some(slot) {
        self.equipped_slot = None;
      }
    }
  }

  /// Returns the operational deployment status of the unit.
  #[must_use]
  pub const fn status(&self) -> UnitStatus {
    self.status
  }

  /// Sets the operational deployment status of the unit.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::AnchorUnitProtected`] if attempting to set a story
  /// anchor unit to [`UnitStatus::Dead`].
  pub fn set_status(&mut self, status: UnitStatus) -> Result<(), error::Error> {
    if self.is_story_anchor() && status == UnitStatus::Dead {
      return Err(error::Error::AnchorUnitProtected {
        character: self.character,
      });
    }
    self.status = status;
    if status == UnitStatus::Dead {
      self.stats.hp = 0;
    } else if self.stats.hp == 0 {
      self.stats.hp = 10;
    }
    Ok(())
  }

  /// Revives a dead or retreated unit to active status with restored hit points.
  pub fn revive(&mut self) {
    let _ = self.set_status(UnitStatus::Active);
  }

  /// Writes modified unit fields back into a raw unit slice.
  pub fn write_back(&self, slice: &mut [u8]) {
    if slice.len() < 32 {
      return;
    }
    slice[0..2].copy_from_slice(&self.character.raw_id().to_le_bytes());
    slice[2] = self.class.raw_id();
    slice[8] = self.stats.hp;
    slice[9] = self.stats.strength;
    slice[10] = self.stats.magic;
    slice[11] = self.stats.skill;
    slice[12] = self.stats.speed;
    slice[13] = self.stats.luck;
    slice[14] = self.stats.defense;
    slice[15] = self.stats.resistance;

    // Inventory records at offsets 25..50 (5 slots * 5 bytes)
    for (i, slot) in self.inventory.iter().enumerate() {
      let off = 25 + i * 5;
      if off + 5 <= slice.len() {
        if let Some(it) = slot {
          slice[off] = 0x04;
          slice[off + 1..off + 3]
            .copy_from_slice(&it.item().raw_id().to_le_bytes());
          let equipped_flag = if self.equipped_slot == Some(i) {
            0x1000
          } else {
            0
          };
          let raw_uses = (it.uses() & 0x0fff) | equipped_flag;
          slice[off + 3..off + 5].copy_from_slice(&raw_uses.to_le_bytes());
        } else {
          slice[off] = 0x04;
          slice[off + 1..off + 5].fill(0);
        }
      }
    }
  }
}

/// Borrowed view over an individual character unit slot.
#[derive(Debug, Clone, Copy)]
pub(crate) struct UnitRef<'a> {
  /// Backing unit entry byte slice.
  raw: &'a [u8],
}

impl<'a> UnitRef<'a> {
  /// Wraps a unit entry byte slice.
  #[must_use]
  pub(crate) fn new(raw: &'a [u8]) -> Self {
    Self { raw }
  }

  /// Character unit enumeration.
  #[must_use]
  pub(crate) fn character(&self) -> character::Character {
    character::Character::from_raw(u16::from_le_bytes([
      self.raw[0],
      self.raw[1],
    ]))
  }

  /// Current class enumeration.
  #[must_use]
  pub fn class(&self) -> class::Class {
    class::Class::from_raw(self.raw[2])
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

  /// Inventory item slot at index `slot` (0..5).
  #[must_use]
  pub fn item(&self, slot: usize) -> Option<item::ItemSlot> {
    if slot >= 5 || self.raw.len() < 25 + (slot + 1) * 5 {
      if self.raw.len() >= 16 + (slot + 1) * 2 {
        let id = u16::from_le_bytes([
          self.raw[16 + slot * 2],
          self.raw[17 + slot * 2],
        ]);
        if id != 0 && id != 0xffff {
          return item::ItemSlot::with_uses(item::Item::from_raw(id), 30).ok();
        }
      }
      return None;
    }
    let off = 25 + slot * 5;
    let id = u16::from_le_bytes([self.raw[off + 1], self.raw[off + 2]]);
    if id == 0 || id == 0xffff {
      None
    } else {
      let raw_uses = u16::from_le_bytes([self.raw[off + 3], self.raw[off + 4]]);
      let uses = raw_uses & 0x0fff;
      item::ItemSlot::with_uses(item::Item::from_raw(id), uses).ok()
    }
  }

  /// Index of the currently equipped weapon slot, if any.
  #[must_use]
  pub fn equipped_slot(&self) -> Option<usize> {
    for slot in 0..5 {
      if self.raw.len() >= 25 + (slot + 1) * 5 {
        let off = 25 + slot * 5;
        let id = u16::from_le_bytes([self.raw[off + 1], self.raw[off + 2]]);
        if id != 0 && id != 0xffff {
          let raw_uses =
            u16::from_le_bytes([self.raw[off + 3], self.raw[off + 4]]);
          if (raw_uses & 0x1000) != 0 {
            return Some(slot);
          }
        }
      }
    }
    None
  }

  /// Current operational status.
  #[must_use]
  pub fn status(&self) -> UnitStatus {
    if self.hp() == 0 {
      UnitStatus::Dead
    } else {
      UnitStatus::Active
    }
  }
}

/// Iterator over character units within a TINU block.
#[derive(Debug, Clone)]
pub(crate) struct UnitIter<'a> {
  /// Backing TINU byte slice.
  raw: &'a [u8],
  /// Current search byte offset.
  pos: usize,
}

impl<'a> UnitIter<'a> {
  /// Creates a new unit iterator over a TINU block slice.
  #[must_use]
  pub(crate) fn new(raw: &'a [u8]) -> Self {
    Self { raw, pos: 0 }
  }

  /// Collects all valid units from a TINU block slice.
  #[must_use]
  pub(crate) fn collect_units(raw: &'a [u8]) -> Vec<Unit> {
    Self::new(raw)
      .map(|(offset, r)| Unit::from_ref(offset, &r))
      .collect()
  }
}

impl<'a> Iterator for UnitIter<'a> {
  type Item = (usize, UnitRef<'a>);

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
        return Some((i, UnitRef::new(&self.raw[i..])));
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
  fn test_unit_stats_combat_stat_bounds() {
    let mut stats = UnitStats::default();
    assert!(stats.set_combat_stat("strength", 81).is_err());
    assert!(stats.set_combat_stat("strength", 40).is_ok());
    assert_eq!(stats.strength, 40);
  }

  #[test]
  fn test_unit_stats_hp_bounds() {
    let mut stats = UnitStats::default();
    assert!(stats.set_hp(9).is_err());
    assert!(stats.set_hp(100).is_err());
    assert!(stats.set_hp(45).is_ok());
    assert_eq!(stats.hp, 45);
  }

  #[test]
  fn test_unit_equipment_and_single_equipped_invariant() {
    let mut unit = Unit::new(
      character::Character::Chrom,
      class::Class::Lord,
      UnitStats {
        hp: 45,
        ..Default::default()
      },
      0,
    );

    // Lord can use swords, not axes
    assert!(unit.set_item(0, item::Item::Falchion, 0).is_ok());
    assert!(unit.set_item(1, item::Item::SteelAxe, 35).is_ok());
    assert!(unit.set_item(2, item::Item::Vulnerary, 3).is_ok());

    // Equip Falchion
    assert!(unit.equip(0).is_ok());
    assert_eq!(unit.equipped_slot(), Some(0));
    assert!(unit.is_slot_equipped(0));

    // Equipping steel axe on Lord fails
    assert!(unit.equip(1).is_err());
    // Falchion remains equipped
    assert_eq!(unit.equipped_slot(), Some(0));

    // Equipping non-weapon fails
    assert!(unit.equip(2).is_err());

    // Unequip
    unit.unequip();
    assert_eq!(unit.equipped_slot(), None);
  }

  #[test]
  fn test_class_change_unequips_incompatible_weapon() {
    let mut unit = Unit::new(
      character::Character::Chrom,
      class::Class::Lord,
      UnitStats {
        hp: 45,
        ..Default::default()
      },
      0,
    );
    assert!(unit.set_item(0, item::Item::Falchion, 0).is_ok());
    assert!(unit.equip(0).is_ok());
    assert_eq!(unit.equipped_slot(), Some(0));

    // Reclass to Archer (which cannot use Swords)
    unit.set_class(class::Class::Archer);
    assert_eq!(unit.equipped_slot(), None);
  }

  #[test]
  fn test_story_anchor_unit_protection() {
    let mut chrom = Unit::new(
      character::Character::Chrom,
      class::Class::Lord,
      UnitStats {
        hp: 40,
        ..Default::default()
      },
      0,
    );
    assert!(chrom.is_story_anchor());
    assert!(chrom.set_status(UnitStatus::Dead).is_err());
    assert!(chrom.set_status(UnitStatus::Retreated).is_ok());
    assert_eq!(chrom.status(), UnitStatus::Retreated);

    let mut donnel = Unit::new(
      character::Character::Donnel,
      class::Class::Villager,
      UnitStats {
        hp: 20,
        ..Default::default()
      },
      180,
    );
    assert!(!donnel.is_story_anchor());
    assert!(donnel.set_status(UnitStatus::Dead).is_ok());
    assert_eq!(donnel.status(), UnitStatus::Dead);
    assert_eq!(donnel.stats.hp, 0);

    donnel.revive();
    assert_eq!(donnel.status(), UnitStatus::Active);
    assert_eq!(donnel.stats.hp, 10);
  }
}
