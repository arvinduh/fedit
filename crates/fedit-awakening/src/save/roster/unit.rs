//! `UnitBlock` character slot domain aggregate and mutators.
//!
//! Owns character entry models in the TINU roster block, coordinating combat stats,
//! equipment, skills, and biological gender resolution.

use crate::error;
use crate::save::convoy::item;
use crate::save::types;

pub mod character;
pub mod class;
pub mod iter;
pub mod skill;
pub mod stats;
pub mod status;

pub use character::Character;
pub use character::Gender;
pub use character::GenderRule;
pub use class::Class;
pub(crate) use iter::UnitIter;
pub use iter::UnitRef;
pub use skill::Skill;
pub use stats::UnitStats;
pub use status::Availability;
pub use status::UnitStatus;

/// Owned character unit domain representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Unit {
  /// Canonical character enumeration.
  character: Character,
  /// Biological gender of the unit.
  gender: Gender,
  /// Unit class enumeration.
  class: Class,
  /// Character experience level (1..=40).
  pub level: u8,
  /// Level experience progress (0..=99).
  pub exp: u8,
  /// Character combat attributes.
  pub stats: UnitStats,
  /// Level-up stat growth bonuses (HP, Str, Mag, Skl, Spd, Lck, Def, Res).
  pub growths: [u8; 8],
  /// Weapon proficiency ranks (Sword, Lance, Axe, Bow, Tome, Staff).
  pub weapon_ranks: [u8; 6],
  /// Personal inventory equipment slots (up to 5 items).
  inventory: [Option<item::ItemSlot>; 5],
  /// Index of the currently equipped weapon slot (0..5), or None if unarmed.
  equipped_slot: Option<usize>,
  /// Equipped skills (up to 5 skills).
  skills: [Skill; 5],
  /// Operational deployment status.
  status: UnitStatus,
  /// Byte offset of this unit entry within the TINU block.
  offset: usize,
}

impl Unit {
  /// Constructs a new unit with default empty inventory and active status.
  #[must_use]
  pub fn new(
    character: Character,
    class: Class,
    stats: UnitStats,
    offset: usize,
  ) -> Self {
    let gender = character.default_gender();
    Self {
      character,
      gender,
      class,
      level: 1,
      exp: 0,
      stats,
      growths: [0; 8],
      weapon_ranks: [0; 6],
      inventory: [None, None, None, None, None],
      equipped_slot: None,
      skills: [Skill::None; 5],
      status: UnitStatus::Active,
      offset,
    }
  }

  /// Constructs an owned unit from a borrowed view and block offset.
  #[must_use]
  pub(crate) fn from_ref(
    offset: usize,
    r: &UnitRef<'_>,
    mode: Option<types::GameMode>,
  ) -> Self {
    Self {
      character: r.character(),
      gender: r.gender(),
      class: r.class(),
      level: r.level().max(1),
      exp: r.exp(),
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
      growths: r.growths(),
      weapon_ranks: r.weapon_ranks(),
      inventory: [r.item(0), r.item(1), r.item(2), r.item(3), r.item(4)],
      equipped_slot: r.equipped_slot(),
      skills: [
        Skill::from_raw(r.skill_id(0)),
        Skill::from_raw(r.skill_id(1)),
        Skill::from_raw(r.skill_id(2)),
        Skill::from_raw(r.skill_id(3)),
        Skill::from_raw(r.skill_id(4)),
      ],
      status: r.status(mode),
      offset,
    }
  }

  /// Returns the byte offset of this unit entry within the TINU block.
  #[must_use]
  pub const fn offset(&self) -> usize {
    self.offset
  }

  /// Returns the canonical character identity.
  #[must_use]
  pub const fn character(&self) -> Character {
    self.character
  }

  /// Returns the concrete 16-bit identifier of this unit in the save file.
  #[must_use]
  pub const fn raw_id(&self) -> u16 {
    match (self.character, self.gender) {
      (Character::Robin, Gender::Male) => 0,
      (Character::Robin, Gender::Female) => 1,
      (Character::Morgan, Gender::Male) => 39,
      (Character::Morgan, Gender::Female) => 40,
      _ => match self.character.raw_id() {
        Some(id) => id,
        None => 0,
      },
    }
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
  pub const fn class(&self) -> Class {
    self.class
  }

  /// Returns the biological gender of this unit.
  #[must_use]
  pub const fn gender(&self) -> Gender {
    self.gender
  }

  /// Sets the unit's biological gender with class compatibility checks.
  ///
  /// # Side Effects
  ///
  /// Automatically resolves gender-paired variants for the current class
  /// (e.g. Priest <-> Cleric). If the unit's class is gender-exclusive and
  /// cannot be adapted (e.g. Berserker or Bride), the mutation is rejected.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::GenderIncompatibleClass`] if the current class
  /// is incompatible with the requested gender.
  pub fn set_gender(&mut self, gender: Gender) -> Result<(), error::Error> {
    let resolved = self.class.resolve_for_gender(gender);
    if !resolved.is_eligible_for_gender(gender) {
      return Err(error::Error::GenderIncompatibleClass {
        character: self.character,
        class: resolved,
        gender,
      });
    }
    self.gender = gender;
    self.class = resolved;
    Ok(())
  }

  /// Returns character experience level (1..=40).
  #[must_use]
  pub const fn level(&self) -> u8 {
    self.level
  }

  /// Sets experience level with bounds checking `[1..=40]`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::StatOutOfBounds`] if `level` is outside `[1, 40]`.
  pub fn set_level(&mut self, level: u8) -> Result<(), error::Error> {
    if level == 0 || level > 40 {
      return Err(error::Error::StatOutOfBounds {
        stat: "level",
        value: level,
        min: 1,
        max: 40,
      });
    }
    self.level = level;
    Ok(())
  }

  /// Returns current level experience progress (0..=99).
  #[must_use]
  pub const fn exp(&self) -> u8 {
    self.exp
  }

  /// Sets level experience progress with bounds checking `[0..=99]`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::StatOutOfBounds`] if `exp > 99`.
  pub fn set_exp(&mut self, exp: u8) -> Result<(), error::Error> {
    if exp > 99 {
      return Err(error::Error::StatOutOfBounds {
        stat: "exp",
        value: exp,
        min: 0,
        max: 99,
      });
    }
    self.exp = exp;
    Ok(())
  }

  /// Returns equipped skills.
  #[must_use]
  pub const fn skills(&self) -> [Skill; 5] {
    self.skills
  }

  /// Sets the skill at the specified slot index (0..5).
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::SlotOutOfBounds`] if `slot >= 5`.
  pub fn set_skill(
    &mut self,
    slot: usize,
    skill: Skill,
  ) -> Result<(), error::Error> {
    if slot >= 5 {
      return Err(error::Error::SlotOutOfBounds { slot });
    }
    self.skills[slot] = skill;
    Ok(())
  }

  /// Changes the unit's class with biological gender eligibility enforcement.
  ///
  /// Automatically unequips the currently active weapon if the new class
  /// is not proficient in using it, preventing battle animation crashes.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::GenderIncompatibleClass`] if the unit's biological
  /// gender cannot reclass to the requested class.
  pub fn set_class(&mut self, class: Class) -> Result<(), error::Error> {
    let gender = self.gender;
    let resolved = class.resolve_for_gender(gender);
    if !resolved.is_eligible_for_gender(gender) {
      return Err(error::Error::GenderIncompatibleClass {
        character: self.character,
        class: resolved,
        gender,
      });
    }
    self.class = resolved;
    if let Some(eq_slot) = self.equipped_slot
      && let Some(item_slot) = &self.inventory[eq_slot]
      && !class.can_use(item_slot.item().kind())
    {
      self.equipped_slot = None;
    }
    Ok(())
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

  /// Sets all level-up stat growth bonuses to maximum (255).
  ///
  /// # Side Effects
  ///
  /// Modifies all 8 stat growth attributes and corresponding stats to 255.
  pub fn max_growths(&mut self) {
    self.growths.fill(255);
    for i in 0..8 {
      let base = i16::from(self.class.base_stats()[i])
        + i16::from(self.character.additions()[i]);
      let max = if i == 0 {
        99
      } else {
        (i16::from(self.class.max_stats()[i])
          + i16::from(self.character.modifiers()[i]))
        .max(0)
      };
      let total = u8::try_from((base + 255).clamp(0, max)).unwrap_or(0);
      match i {
        0 => self.stats.hp = total,
        1 => self.stats.strength = total,
        2 => self.stats.magic = total,
        3 => self.stats.skill = total,
        4 => self.stats.speed = total,
        5 => self.stats.luck = total,
        6 => self.stats.defense = total,
        7 => self.stats.resistance = total,
        _ => {}
      }
    }
  }

  /// Sets all weapon proficiencies to maximum Rank A (90).
  ///
  /// # Side Effects
  ///
  /// Modifies all 6 weapon proficiencies to 90.
  pub fn max_weapon_ranks(&mut self) {
    self.weapon_ranks.fill(90);
  }

  /// Sets a specific stat growth bonus attribute.
  ///
  /// # Side Effects
  ///
  /// Modifies the growth byte and corresponding stat at `stat_index`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::SlotOutOfBounds`] if `stat_index >= 8`.
  pub fn set_growth(
    &mut self,
    stat_index: usize,
    value: u8,
  ) -> Result<(), error::Error> {
    if stat_index >= 8 {
      return Err(error::Error::SlotOutOfBounds { slot: stat_index });
    }
    self.growths[stat_index] = value;
    let base = i16::from(self.class.base_stats()[stat_index])
      + i16::from(self.character.additions()[stat_index]);
    let max = if stat_index == 0 {
      99
    } else {
      (i16::from(self.class.max_stats()[stat_index])
        + i16::from(self.character.modifiers()[stat_index]))
      .max(0)
    };
    let total =
      u8::try_from((base + i16::from(value)).clamp(0, max)).unwrap_or(0);
    match stat_index {
      0 => self.stats.hp = total,
      1 => self.stats.strength = total,
      2 => self.stats.magic = total,
      3 => self.stats.skill = total,
      4 => self.stats.speed = total,
      5 => self.stats.luck = total,
      6 => self.stats.defense = total,
      7 => self.stats.resistance = total,
      _ => {}
    }
    Ok(())
  }

  /// Sets a specific weapon proficiency rank.
  ///
  /// # Side Effects
  ///
  /// Modifies the weapon exp byte at `rank_index`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::SlotOutOfBounds`] if `rank_index >= 6`.
  pub fn set_weapon_rank(
    &mut self,
    rank_index: usize,
    value: u8,
  ) -> Result<(), error::Error> {
    if rank_index >= 6 {
      return Err(error::Error::SlotOutOfBounds { slot: rank_index });
    }
    self.weapon_ranks[rank_index] = value;
    Ok(())
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

  /// Returns high-level army availability status.
  #[must_use]
  pub const fn availability(&self) -> Availability {
    match self.status {
      UnitStatus::Active => Availability::Active,
      UnitStatus::Retreated | UnitStatus::Dead => Availability::Inactive,
    }
  }

  /// Returns true if the unit is actively deployed and available.
  #[must_use]
  pub const fn is_active(&self) -> bool {
    matches!(self.status, UnitStatus::Active)
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
      self.stats.strength = 0;
      self.stats.magic = 0;
      self.stats.skill = 0;
      self.stats.speed = 0;
      self.stats.luck = 0;
      self.stats.defense = 0;
      self.stats.resistance = 0;
    } else if self.stats.hp == 0 {
      self.stats.hp = 10;
    }
    Ok(())
  }

  /// Revives a dead or retreated unit to active status with restored hit points.
  pub fn revive(&mut self) {
    self.status = UnitStatus::Active;
    if self.stats.hp == 0 {
      self.stats.hp = 10;
    }
    if self.stats.strength == 0
      && self.stats.magic == 0
      && self.stats.skill == 0
      && self.stats.speed == 0
      && self.stats.luck == 0
      && self.stats.defense == 0
      && self.stats.resistance == 0
    {
      self.stats.strength = 5;
      self.stats.magic = 5;
      self.stats.skill = 5;
      self.stats.speed = 5;
      self.stats.luck = 5;
      self.stats.defense = 5;
      self.stats.resistance = 5;
    }
    if self.level == 0 {
      self.level = 1;
    }
  }

  /// Writes modified unit fields back into a raw unit slice.
  pub fn write_back(&self, slice: &mut [u8]) {
    if slice.len() < 32 {
      return;
    }
    let pid = self.raw_id();
    slice[0..2].copy_from_slice(&pid.to_le_bytes());
    slice[2] = self.class.raw_id_for_gender(self.gender);
    for i in 0..8 {
      if slice.len() > 9 + i {
        let base = i16::from(self.class.base_stats()[i])
          + i16::from(self.character.additions()[i]);
        let max = if i == 0 {
          99
        } else {
          (i16::from(self.class.max_stats()[i])
            + i16::from(self.character.modifiers()[i]))
          .max(0)
        };
        let stat_val = match i {
          0 => self.stats.hp,
          1 => self.stats.strength,
          2 => self.stats.magic,
          3 => self.stats.skill,
          4 => self.stats.speed,
          5 => self.stats.luck,
          6 => self.stats.defense,
          7 => self.stats.resistance,
          _ => 0,
        };
        let calc_from_growth =
          u8::try_from((base + i16::from(self.growths[i])).clamp(0, max))
            .unwrap_or(0);
        let growth_to_write = if stat_val == calc_from_growth {
          self.growths[i]
        } else {
          u8::try_from((i16::from(stat_val) - base).clamp(0, 255)).unwrap_or(0)
        };
        slice[9 + i] = growth_to_write;
      }
    }
    if slice.len() >= 20 {
      slice[17] = self.level;
      slice[18] = self.exp;
      slice[19] = self.stats.hp;
    }

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

    // Skill records at offsets 50..60 (5 slots * 2 bytes)
    for (i, &sk) in self.skills.iter().enumerate() {
      let off = 50 + i * 2;
      if off + 2 <= slice.len() {
        slice[off..off + 2].copy_from_slice(&sk.raw_id().to_le_bytes());
      }
    }

    if slice.len() >= 66 {
      slice[60..66].copy_from_slice(&self.weapon_ranks);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_unit_equipment_and_single_equipped_invariant() {
    let mut unit = Unit::new(
      Character::Chrom,
      Class::Lord,
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
      Character::Chrom,
      Class::Lord,
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
    assert!(unit.set_class(Class::Archer).is_ok());
    assert_eq!(unit.equipped_slot(), None);
  }

  #[test]
  fn test_class_gender_restrictions_and_resolution() {
    let mut chrom = Unit::new(
      Character::Chrom,
      Class::Lord,
      UnitStats {
        hp: 45,
        ..Default::default()
      },
      0,
    );
    // Chrom cannot become a Bride or Pegasus Knight
    assert!(chrom.set_class(Class::Bride).is_err());
    assert!(chrom.set_class(Class::PegasusKnight).is_err());
    assert!(chrom.set_class(Class::DarkFlier).is_err());

    // Chrom can become a Dread Fighter or Berserker
    assert!(chrom.set_class(Class::DreadFighter).is_ok());
    assert!(chrom.set_class(Class::Berserker).is_ok());

    // Reverting to Lord gives ID 0 (Lord M)
    assert!(chrom.set_class(Class::Lord).is_ok());
    let mut slice = [0u8; 32];
    chrom.write_back(&mut slice);
    assert_eq!(slice[2], 0);

    // Lucina
    let mut lucina = Unit::new(
      Character::Lucina,
      Class::Lord,
      UnitStats {
        hp: 35,
        ..Default::default()
      },
      0,
    );
    // Lucina as Lord gives ID 1 (Lord F)
    let mut lucina_slice = [0u8; 32];
    lucina.write_back(&mut lucina_slice);
    assert_eq!(lucina_slice[2], 1);

    // Lucina can become a Bride or Dark Flier, but not a Berserker
    assert!(lucina.set_class(Class::Bride).is_ok());
    assert!(lucina.set_class(Class::DarkFlier).is_ok());
    assert!(lucina.set_class(Class::Berserker).is_err());
  }

  #[test]
  fn test_unit_availability_abstractions() {
    let mut cordelia = Unit::new(
      Character::Cordelia,
      Class::DarkFlier,
      UnitStats {
        hp: 40,
        strength: 20,
        ..Default::default()
      },
      0,
    );
    assert_eq!(cordelia.availability(), Availability::Active);
    assert!(cordelia.is_active());

    assert!(cordelia.set_status(UnitStatus::Dead).is_ok());
    assert_eq!(cordelia.availability(), Availability::Inactive);
    assert!(!cordelia.is_active());

    cordelia.revive();
    assert_eq!(cordelia.availability(), Availability::Active);
    assert!(cordelia.is_active());
  }

  #[test]
  fn test_story_anchor_unit_protection() {
    let mut chrom = Unit::new(
      Character::Chrom,
      Class::Lord,
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
      Character::Donnel,
      Class::Villager,
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

  #[test]
  fn test_unit_skills_and_raw_id() {
    let mut robin = Unit::new(
      Character::Robin,
      Class::Tactician,
      UnitStats {
        hp: 30,
        ..Default::default()
      },
      0,
    );
    assert_eq!(robin.raw_id(), 0);
    assert!(robin.set_gender(Gender::Female).is_ok());
    assert_eq!(robin.raw_id(), 1);

    assert_eq!(robin.skills()[0], Skill::None);
    assert!(robin.set_skill(0, Skill::Veteran).is_ok());
    assert!(robin.set_skill(1, Skill::Solidarity).is_ok());
    assert!(robin.set_skill(2, Skill::Ignis).is_ok());
    assert!(robin.set_skill(3, Skill::RallySpectrum).is_ok());
    assert!(robin.set_skill(4, Skill::LimitBreaker).is_ok());
    assert_eq!(robin.skills()[0], Skill::Veteran);
    assert_eq!(robin.skills()[4], Skill::LimitBreaker);

    let mut slice = [0u8; 60];
    robin.write_back(&mut slice);
    assert_eq!(
      u16::from_le_bytes([slice[50], slice[51]]),
      Skill::Veteran.raw_id()
    );
    assert_eq!(
      u16::from_le_bytes([slice[58], slice[59]]),
      Skill::LimitBreaker.raw_id()
    );
  }
}
