//! High-level forge orchestration, inventory integration, and lifecycle cleanup.
//!
//! Owns forged weapon creation, placement in unit inventory and convoy storage,
//! in-place forging mutations, reference tracking across roster and bookmark groups,
//! and pruning of unreferenced IFER entries.

use crate::save::Error;
use crate::save::Save;
use crate::save::convoy;
use crate::save::roster;

impl Save {
  /// Creates a custom forged weapon in the IFER refinement table.
  ///
  /// # Errors
  ///
  /// Returns an error if the base item is not forgeable, stat totals are out of bounds
  /// (might $0..=99$, hit $0..=150$, crit $0..=100$), name exceeds character limits,
  /// or table is full.
  pub fn create_forged_weapon(
    &mut self,
    base: convoy::ItemType,
    name: impl Into<String>,
    might: i8,
    hit: i8,
    crit: i8,
  ) -> Result<usize, Error> {
    Ok(self.forges.create_weapon(base, name, might, hit, crit)?)
  }

  /// Adds a custom forged weapon directly to convoy storage.
  ///
  /// # Errors
  ///
  /// Returns an error if the forged weapon position does not exist or durability overflows.
  pub fn add_forged_weapon_to_convoy(
    &mut self,
    position: usize,
    uses: u16,
  ) -> Result<usize, Error> {
    if self.forges.get(position).is_none() {
      return Err(convoy::forge::Error::ForgeNotFound { position }.into());
    }
    let item_id = self.convoy.forged_item_id(position)?;
    let item_idx = usize::from(item_id);
    self.convoy.set_durability(item_idx, uses)?;
    Ok(item_idx)
  }

  /// One-step forged weapon creation and addition to convoy storage.
  ///
  /// # Errors
  ///
  /// Returns an error if base item is not forgeable, stat deltas or durability
  /// are out of bounds, or tables are full.
  pub fn forge_and_store(
    &mut self,
    base: convoy::ItemType,
    name: impl Into<String>,
    might: i8,
    hit: i8,
    crit: i8,
    uses: Option<u16>,
  ) -> Result<usize, Error> {
    let position = self.create_forged_weapon(base, name, might, hit, crit)?;
    let durability =
      uses.unwrap_or_else(|| u16::from(base.max_durability().unwrap_or(1)));
    self.add_forged_weapon_to_convoy(position, durability)?;
    Ok(position)
  }

  /// Equips or places a forged weapon directly into a unit's inventory slot.
  ///
  /// # Errors
  ///
  /// Returns an error if the forge position does not exist, unit does not exist,
  /// or inventory slot is invalid.
  pub fn give_forged_weapon_to_unit(
    &mut self,
    unit_index: usize,
    inv_slot: usize,
    position: usize,
    uses: Option<u16>,
  ) -> Result<(), Error> {
    let forge = self
      .forges
      .get(position)
      .ok_or(convoy::forge::Error::ForgeNotFound { position })?;
    let base_item = convoy::ItemType::from_id(forge.weapon_id);
    let durability = uses
      .unwrap_or_else(|| u16::from(base_item.max_durability().unwrap_or(1)));
    let forged_id = self
      .convoy
      .forged_item_id(position)
      .map_err(|_| convoy::forge::Error::ForgeNotFound { position })?;
    let item = convoy::item::Item::with_durability(
      convoy::ItemType::from_id(forged_id),
      durability,
    )
    .map_err(convoy::Error::from)?;
    let unit = self
      .roster
      .get_mut(unit_index)
      .ok_or(roster::Error::UnitIndexOutOfBounds { index: unit_index })?;
    unit.set_item(inv_slot, item)?;
    Ok(())
  }

  /// Forges an existing weapon currently equipped in a unit's inventory slot in-place.
  ///
  /// # Errors
  ///
  /// Returns an error if the unit index or inventory slot is invalid, item is empty,
  /// or item is not forgeable.
  pub fn forge_unit_weapon(
    &mut self,
    unit_idx: usize,
    inv_slot: usize,
    name: impl Into<String>,
    might_delta: i8,
    hit_delta: i8,
    crit_delta: i8,
  ) -> Result<usize, Error> {
    let unit = self
      .roster
      .get(unit_idx)
      .ok_or(roster::Error::UnitIndexOutOfBounds { index: unit_idx })?;
    let item = unit
      .item(inv_slot)
      .copied()
      .ok_or(roster::unit::Error::EmptySlot { slot: inv_slot })?;
    let base = item.item_type();
    let durability = item.durability();
    let forge_pos = self.create_forged_weapon(
      base,
      name,
      might_delta,
      hit_delta,
      crit_delta,
    )?;
    self.give_forged_weapon_to_unit(
      unit_idx,
      inv_slot,
      forge_pos,
      Some(durability),
    )?;
    Ok(forge_pos)
  }

  /// Forges an existing regular item in convoy storage into a custom forged weapon.
  ///
  /// # Errors
  ///
  /// Returns an error if the convoy item ID is invalid or item is unforgeable.
  pub fn forge_convoy_item(
    &mut self,
    convoy_item_id: usize,
    name: impl Into<String>,
    might_delta: i8,
    hit_delta: i8,
    crit_delta: i8,
  ) -> Result<usize, Error> {
    let item_u16 = u16::try_from(convoy_item_id).map_err(|_| {
      convoy::Error::SlotOutOfBounds {
        slot: convoy_item_id,
        max: self.convoy.total_count(),
      }
    })?;
    let item = self.convoy.find_by_id(item_u16).copied().ok_or(
      convoy::Error::SlotOutOfBounds {
        slot: convoy_item_id,
        max: self.convoy.total_count(),
      },
    )?;
    let base = item.item_type();
    let durability = item.durability();
    let forge_pos = self.create_forged_weapon(
      base,
      name,
      might_delta,
      hit_delta,
      crit_delta,
    )?;
    self.convoy.set_durability(convoy_item_id, 0)?;
    self.add_forged_weapon_to_convoy(forge_pos, durability)?;
    Ok(forge_pos)
  }

  /// Removes an unused forged weapon from the IFER table by position.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::ForgeInUse`] if the forged weapon is currently held in any
  ///   unit's inventory (across roster or active map groups) or in convoy storage.
  /// - Returns [`Error::ForgeNotFound`] if no forge exists at `position`.
  pub fn remove_forged_weapon(&mut self, position: usize) -> Result<(), Error> {
    if self.forges.get(position).is_none() {
      return Err(convoy::forge::Error::ForgeNotFound { position }.into());
    }
    if self.is_forge_in_use(position) {
      return Err(Error::ForgeInUse { position });
    }
    self.forges.remove_forge(position);
    Ok(())
  }

  /// Prunes all unreferenced forged weapons from the IFER refinement table.
  ///
  /// Scans all combatant inventories (campaign roster and active bookmark groups)
  /// and convoy storage. Removes any forge entry that has zero active references,
  /// freeing up its position ID for subsequent forges without disrupting the
  /// position IDs of remaining weapons.
  ///
  /// Returns the count of pruned forged weapons.
  pub fn prune_unused_forges(&mut self) -> usize {
    let unreferenced: Vec<usize> = self
      .forges
      .iter()
      .map(|w| w.position)
      .filter(|&pos| !self.is_forge_in_use(pos))
      .collect();

    let count = unreferenced.len();
    for pos in unreferenced {
      self.forges.remove_forge(pos);
    }
    count
  }

  /// Checks if a forged weapon position is actively referenced anywhere in the save.
  #[must_use]
  pub fn is_forge_in_use(&self, position: usize) -> bool {
    let Ok(forged_id) = self.convoy.forged_item_id(position) else {
      return false;
    };

    // 1. Check Convoy storage
    if self
      .convoy
      .find_by_id(forged_id)
      .map_or(0, convoy::item::Item::durability)
      > 0
    {
      return true;
    }

    // 2. Check campaign roster units
    for unit in &self.roster {
      for item in unit.inventory().iter().flatten() {
        if item.item_type().id() == forged_id {
          return true;
        }
      }
    }

    // 3. Check active battle groups (if bookmark)
    if self.is_bookmark() {
      let check_group = |units: &[roster::Unit]| {
        units.iter().any(|u| {
          u.inventory()
            .iter()
            .flatten()
            .any(|it| it.item_type().id() == forged_id)
        })
      };
      if check_group(self.roster.groups().blue())
        || check_group(self.roster.groups().red())
        || check_group(self.roster.groups().green())
        || check_group(self.roster.groups().other())
      {
        return true;
      }
    }

    false
  }
}
