//! Forged weapons table (`IFER` refinement block) management and mutation.
//!
//! Owns custom named and stat-refined weapons stored in the campaign convoy
//! and character inventory.

use std::ops;
use std::slice;

use thiserror;

use super::item::ItemType;
use crate::save::utils::text;

/// Maximum number of forged weapons supported in the IFER block.
pub const MAX_FORGED_WEAPONS: usize = 150;

/// Western (US/EU) single refinement byte length: 2 + (18 * 2) + 6 = 44 bytes.
pub const SIZE_WEST: usize = 0x2C;

/// Japanese single refinement byte length: 2 + (10 * 2) + 6 = 28 bytes.
pub const SIZE_JP: usize = 0x1C;

/// Maximum character length for custom weapon name in Western saves.
pub const MAX_NAME_LEN_WEST: usize = 18;

/// Maximum character length for custom weapon name in Japanese saves.
pub const MAX_NAME_LEN_JP: usize = 10;

/// Errors that can occur during forge table parsing and mutation.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// IFER block buffer was too short to contain the header or weapon entries.
  #[error("IFER block length {len} too short (expected at least {expected})")]
  BufferTooShort {
    /// Provided buffer length.
    len: usize,
    /// Minimum expected length.
    expected: usize,
  },
  /// Target forged weapon position was not found in the table.
  #[error("forged weapon position {position} not found")]
  ForgeNotFound {
    /// Searched position index.
    position: usize,
  },
  /// Forge table has reached maximum capacity.
  #[error("forge table is full (max {max} weapons)")]
  ForgeTableFull {
    /// Maximum capacity.
    max: usize,
  },
  /// Might modification value was out of bounds.
  #[error("forge might {value} is out of bounds [{min}..={max}]")]
  MightOutOfBounds {
    /// Provided value.
    value: i16,
    /// Minimum allowed.
    min: i16,
    /// Maximum allowed.
    max: i16,
  },
  /// Hit modification value was out of bounds.
  #[error("forge hit {value} is out of bounds [{min}..={max}]")]
  HitOutOfBounds {
    /// Provided value.
    value: i16,
    /// Minimum allowed.
    min: i16,
    /// Maximum allowed.
    max: i16,
  },
  /// Critical modification value was out of bounds.
  #[error("forge crit {value} is out of bounds [{min}..={max}]")]
  CritOutOfBounds {
    /// Provided value.
    value: i16,
    /// Minimum allowed.
    min: i16,
    /// Maximum allowed.
    max: i16,
  },
  /// Custom weapon name exceeded maximum character length.
  #[error("forge custom name length {len} exceeds maximum {max}")]
  NameTooLong {
    /// Character count.
    len: usize,
    /// Maximum allowed characters.
    max: usize,
  },
  /// Target item type does not support weapon forging.
  #[error("item '{item}' is not forgeable")]
  CannotForgeItem {
    /// Provided base item.
    item: ItemType,
  },
}

/// A single forged weapon entry within the IFER refinement table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgedWeapon {
  /// Unique position identifier matching inventory equipment references (0..150).
  pub position: usize,
  /// Custom UTF-16LE weapon name.
  pub name: String,
  /// Base weapon item type ID.
  pub weapon_id: u16,
  /// Might modification delta (-5 to +5, safe up to +10).
  pub might: i8,
  /// Hit modification delta (-15 to +15, safe up to +50).
  pub hit: i8,
  /// Critical modification delta (-15 to +15, safe up to +50).
  pub crit: i8,
  /// True if assigned to enemy unit combatants.
  pub is_enemy: bool,
}

impl ForgedWeapon {
  /// Constructs a new forged weapon with the given attributes.
  #[must_use]
  pub fn new(
    position: usize,
    name: impl Into<String>,
    weapon_id: u16,
    might: i8,
    hit: i8,
    crit: i8,
  ) -> Self {
    Self {
      position,
      name: name.into(),
      weapon_id,
      might,
      hit,
      crit,
      is_enemy: false,
    }
  }

  /// Computes total weapon might (base + delta), clamped to `[0, 99]`.
  #[must_use]
  pub fn total_might(&self, base: ItemType) -> u8 {
    let total = i16::from(base.might()) + i16::from(self.might);
    u8::try_from(total.clamp(0, 99)).unwrap_or(0)
  }

  /// Computes total weapon hit rate (base + delta), clamped to `[0, 150]`.
  #[must_use]
  pub fn total_hit(&self, base: ItemType) -> u8 {
    let total = i16::from(base.hit()) + i16::from(self.hit);
    u8::try_from(total.clamp(0, 150)).unwrap_or(0)
  }

  /// Computes total weapon critical rate (base + delta), clamped to `[0, 100]`.
  #[must_use]
  pub fn total_crit(&self, base: ItemType) -> u8 {
    let total = i16::from(base.crit()) + i16::from(self.crit);
    u8::try_from(total.clamp(0, 100)).unwrap_or(0)
  }
}

/// Encapsulates the IFER refined weapons table in the save payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeTable {
  /// Whether Western (US/EU) format is used (affects entry sizing).
  is_west: bool,
  /// Preserved 7-byte IFER block header.
  header: [u8; 7],
  /// Active forged weapon records.
  weapons: Vec<ForgedWeapon>,
}

impl ForgeTable {
  /// Parses the IFER refined weapons table from raw block bytes.
  ///
  /// # Errors
  ///
  /// Returns [`Error::BufferTooShort`] if `raw.len() < 7`.
  pub fn from_bytes(raw: &[u8], is_west: bool) -> Result<Self, Error> {
    if raw.len() < 7 {
      return Err(Error::BufferTooShort {
        len: raw.len(),
        expected: 7,
      });
    }

    let mut header = [0u8; 7];
    header.copy_from_slice(&raw[..7]);

    let count = usize::from(raw[5]);
    let entry_size = if is_west { SIZE_WEST } else { SIZE_JP };
    let name_chars = if is_west {
      MAX_NAME_LEN_WEST
    } else {
      MAX_NAME_LEN_JP
    };

    let mut weapons = Vec::with_capacity(count);
    for i in 0..count {
      let offset = 7 + i * entry_size;
      if offset + entry_size > raw.len() {
        break;
      }
      let entry = &raw[offset..offset + entry_size];
      let position = usize::from(entry[0]);

      // Decode UTF-16LE weapon name
      let name = text::read_utf16le(&entry[2..2 + name_chars * 2]);

      let stats_off = 2 + name_chars * 2;
      let weapon_id =
        u16::from_le_bytes([entry[stats_off], entry[stats_off + 1]]);
      let might = entry[stats_off + 2].cast_signed();
      let hit = entry[stats_off + 3].cast_signed();
      let crit = entry[stats_off + 4].cast_signed();
      let is_enemy = entry[stats_off + 5] == 1;

      weapons.push(ForgedWeapon {
        position,
        name,
        weapon_id,
        might,
        hit,
        crit,
        is_enemy,
      });
    }

    Ok(Self {
      is_west,
      header,
      weapons,
    })
  }

  /// Serializes the forge table into raw IFER block bytes.
  #[must_use]
  pub fn to_bytes(&self) -> Vec<u8> {
    let entry_size = if self.is_west { SIZE_WEST } else { SIZE_JP };
    let name_chars = if self.is_west {
      MAX_NAME_LEN_WEST
    } else {
      MAX_NAME_LEN_JP
    };

    let mut out = Vec::with_capacity(7 + self.weapons.len() * entry_size);
    let mut header = self.header;
    header[5] =
      u8::try_from(self.weapons.len().min(MAX_FORGED_WEAPONS)).unwrap_or(0);
    out.extend_from_slice(&header);

    for w in &self.weapons {
      let mut entry = vec![0u8; entry_size];
      entry[0] = u8::try_from(w.position & 0xFF).unwrap_or(0);

      text::write_utf16le(&mut entry[2..2 + name_chars * 2], &w.name);

      let stats_off = 2 + name_chars * 2;
      let wid = w.weapon_id.to_le_bytes();
      entry[stats_off] = wid[0];
      entry[stats_off + 1] = wid[1];
      entry[stats_off + 2] = w.might.cast_unsigned();
      entry[stats_off + 3] = w.hit.cast_unsigned();
      entry[stats_off + 4] = w.crit.cast_unsigned();
      entry[stats_off + 5] = u8::from(w.is_enemy);

      out.extend_from_slice(&entry);
    }

    out
  }

  /// Number of active forged weapons.
  #[must_use]
  pub fn len(&self) -> usize {
    self.weapons.len()
  }

  /// Returns true if there are no forged weapons.
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.weapons.is_empty()
  }

  /// Returns a reference to the forged weapon at the given position.
  #[must_use]
  pub fn get(&self, position: usize) -> Option<&ForgedWeapon> {
    self.weapons.iter().find(|w| w.position == position)
  }

  /// Returns a mutable reference to the forged weapon at the given position.
  fn get_mut(&mut self, position: usize) -> Option<&mut ForgedWeapon> {
    self.weapons.iter_mut().find(|w| w.position == position)
  }

  /// Returns an iterator over all forged weapons.
  pub fn iter(&self) -> slice::Iter<'_, ForgedWeapon> {
    self.weapons.iter()
  }

  /// Returns a mutable iterator over all forged weapons.
  pub fn iter_mut(&mut self) -> slice::IterMut<'_, ForgedWeapon> {
    self.weapons.iter_mut()
  }
}

impl ops::Index<usize> for ForgeTable {
  type Output = ForgedWeapon;

  fn index(&self, index: usize) -> &Self::Output {
    &self.weapons[index]
  }
}

impl<'a> IntoIterator for &'a ForgeTable {
  type Item = &'a ForgedWeapon;
  type IntoIter = slice::Iter<'a, ForgedWeapon>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut ForgeTable {
  type Item = &'a mut ForgedWeapon;
  type IntoIter = slice::IterMut<'a, ForgedWeapon>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl ForgeTable {
  /// Creates and adds a new forged weapon to the table, validating bounds and allocating a position.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::CannotForgeItem`] if `base` is not forgeable.
  /// - Returns [`Error::MightOutOfBounds`] if `might` is outside `[-10, 10]`.
  /// - Returns [`Error::HitOutOfBounds`] if `hit` is outside `[-50, 50]`.
  /// - Returns [`Error::CritOutOfBounds`] if `crit` is outside `[-50, 50]`.
  /// - Returns [`Error::NameTooLong`] if the custom name exceeds character limits.
  /// - Returns [`Error::ForgeTableFull`] if 150 weapons already exist.
  pub fn create_weapon(
    &mut self,
    base: ItemType,
    name: impl Into<String>,
    might: i8,
    hit: i8,
    crit: i8,
  ) -> Result<usize, Error> {
    if !base.is_forgeable() {
      return Err(Error::CannotForgeItem { item: base });
    }
    let total_might = i16::from(base.might()) + i16::from(might);
    if !(0..=99).contains(&total_might) {
      return Err(Error::MightOutOfBounds {
        value: total_might,
        min: 0,
        max: 99,
      });
    }
    let total_hit = i16::from(base.hit()) + i16::from(hit);
    if !(0..=150).contains(&total_hit) {
      return Err(Error::HitOutOfBounds {
        value: total_hit,
        min: 0,
        max: 150,
      });
    }
    let total_crit = i16::from(base.crit()) + i16::from(crit);
    if !(0..=100).contains(&total_crit) {
      return Err(Error::CritOutOfBounds {
        value: total_crit,
        min: 0,
        max: 100,
      });
    }
    let name = name.into();
    let max_len = if self.is_west {
      MAX_NAME_LEN_WEST
    } else {
      MAX_NAME_LEN_JP
    };
    let char_count = name.chars().count();
    if char_count > max_len {
      return Err(Error::NameTooLong {
        len: char_count,
        max: max_len,
      });
    }
    if self.weapons.len() >= MAX_FORGED_WEAPONS {
      return Err(Error::ForgeTableFull {
        max: MAX_FORGED_WEAPONS,
      });
    }
    let position = (0..MAX_FORGED_WEAPONS)
      .find(|&id| !self.weapons.iter().any(|w| w.position == id))
      .unwrap_or(self.weapons.len());
    self.weapons.push(ForgedWeapon {
      position,
      name,
      weapon_id: base.raw_id(),
      might,
      hit,
      crit,
      is_enemy: false,
    });
    Ok(position)
  }

  /// Adds a new forged weapon to the table, allocating an unused position ID.
  ///
  /// # Errors
  ///
  /// Returns [`Error::ForgeTableFull`] if 150 weapons already exist.
  pub fn add_forge(
    &mut self,
    mut weapon: ForgedWeapon,
  ) -> Result<usize, Error> {
    if self.weapons.len() >= MAX_FORGED_WEAPONS {
      return Err(Error::ForgeTableFull {
        max: MAX_FORGED_WEAPONS,
      });
    }

    let next_id = (0..MAX_FORGED_WEAPONS)
      .find(|&id| !self.weapons.iter().any(|w| w.position == id))
      .unwrap_or(self.weapons.len());

    weapon.position = next_id;
    self.weapons.push(weapon);
    Ok(next_id)
  }

  /// Removes the forged weapon at the specified position.
  pub fn remove_forge(&mut self, position: usize) -> Option<ForgedWeapon> {
    if let Some(idx) = self.weapons.iter().position(|w| w.position == position)
    {
      Some(self.weapons.remove(idx))
    } else {
      None
    }
  }

  /// Sets the might delta for the forged weapon at `position`.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::ForgeNotFound`] if no weapon exists at `position`.
  /// - Returns [`Error::MightOutOfBounds`] if `might` is outside `[-10, 10]`.
  pub fn set_might(&mut self, position: usize, might: i8) -> Result<(), Error> {
    let w = self
      .get_mut(position)
      .ok_or(Error::ForgeNotFound { position })?;
    let base = ItemType::from_raw(w.weapon_id);
    let total_might = i16::from(base.might()) + i16::from(might);
    if !(0..=99).contains(&total_might) {
      return Err(Error::MightOutOfBounds {
        value: total_might,
        min: 0,
        max: 99,
      });
    }
    w.might = might;
    Ok(())
  }

  /// Sets the hit delta for the forged weapon at `position`.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::ForgeNotFound`] if no weapon exists at `position`.
  /// - Returns [`Error::HitOutOfBounds`] if `hit` is outside `[-50, 50]`.
  pub fn set_hit(&mut self, position: usize, hit: i8) -> Result<(), Error> {
    let w = self
      .get_mut(position)
      .ok_or(Error::ForgeNotFound { position })?;
    let base = ItemType::from_raw(w.weapon_id);
    let total_hit = i16::from(base.hit()) + i16::from(hit);
    if !(0..=150).contains(&total_hit) {
      return Err(Error::HitOutOfBounds {
        value: total_hit,
        min: 0,
        max: 150,
      });
    }
    w.hit = hit;
    Ok(())
  }

  /// Sets the crit delta for the forged weapon at `position`.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::ForgeNotFound`] if no weapon exists at `position`.
  /// - Returns [`Error::CritOutOfBounds`] if `crit` is outside `[-50, 50]`.
  pub fn set_crit(&mut self, position: usize, crit: i8) -> Result<(), Error> {
    let w = self
      .get_mut(position)
      .ok_or(Error::ForgeNotFound { position })?;
    let base = ItemType::from_raw(w.weapon_id);
    let total_crit = i16::from(base.crit()) + i16::from(crit);
    if !(0..=100).contains(&total_crit) {
      return Err(Error::CritOutOfBounds {
        value: total_crit,
        min: 0,
        max: 100,
      });
    }
    w.crit = crit;
    Ok(())
  }

  /// Sets the custom weapon name for the forged weapon at `position`.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::ForgeNotFound`] if no weapon exists at `position`.
  /// - Returns [`Error::NameTooLong`] if the name exceeds maximum characters.
  pub fn set_name(&mut self, position: usize, name: &str) -> Result<(), Error> {
    let max_len = if self.is_west {
      MAX_NAME_LEN_WEST
    } else {
      MAX_NAME_LEN_JP
    };
    let char_count = name.chars().count();
    if char_count > max_len {
      return Err(Error::NameTooLong {
        len: char_count,
        max: max_len,
      });
    }
    let w = self
      .get_mut(position)
      .ok_or(Error::ForgeNotFound { position })?;
    w.name = name.to_string();
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_forge_table_roundtrip() {
    let mut header = [0u8; 7];
    header[0..4].copy_from_slice(b"IFER");
    header[5] = 1;

    let mut raw = Vec::new();
    raw.extend_from_slice(&header);

    let mut entry = vec![0u8; SIZE_WEST];
    entry[0] = 3; // position
    // Name: "Excalibur" in UTF-16LE
    let name_units: Vec<u16> = "Excalibur".encode_utf16().collect();
    for (i, &u) in name_units.iter().enumerate() {
      let b = u.to_le_bytes();
      entry[2 + i * 2] = b[0];
      entry[2 + i * 2 + 1] = b[1];
    }
    let stats_off = 2 + MAX_NAME_LEN_WEST * 2;
    entry[stats_off] = 4; // weapon_id = 4
    entry[stats_off + 2] = 5; // might = 5
    entry[stats_off + 3] = 10; // hit = 10
    entry[stats_off + 4] = 15; // crit = 15

    raw.extend_from_slice(&entry);

    let mut table =
      ForgeTable::from_bytes(&raw, true).expect("parse forge table");
    assert_eq!(table.len(), 1);

    let weapon = table.get(3).expect("find weapon 3");
    assert_eq!(weapon.name, "Excalibur");
    assert_eq!(weapon.weapon_id, 4);
    assert_eq!(weapon.might, 5);
    assert_eq!(weapon.hit, 10);
    assert_eq!(weapon.crit, 15);

    // Mutate might and name
    assert!(table.set_might(3, 8).is_ok());
    assert!(table.set_name(3, "Holy Sword").is_ok());

    let repacked = table.to_bytes();
    let reloaded =
      ForgeTable::from_bytes(&repacked, true).expect("reload forge table");
    let reloaded_w = reloaded.get(3).expect("find reloaded weapon 3");
    assert_eq!(reloaded_w.name, "Holy Sword");
    assert_eq!(reloaded_w.might, 8);
  }

  #[test]
  fn test_forge_table_add_remove() {
    let mut header = [0u8; 7];
    header[0..4].copy_from_slice(b"IFER");
    let mut table = ForgeTable::from_bytes(&header, true).expect("empty table");
    assert!(table.is_empty());

    let w = ForgedWeapon::new(0, "Thunder", 108, 3, 5, 0);
    let pos = table.add_forge(w).expect("add forge");
    assert_eq!(pos, 0);
    assert_eq!(table.len(), 1);

    let removed = table.remove_forge(0);
    assert!(removed.is_some());
    assert!(table.is_empty());
  }

  #[test]
  fn test_create_weapon_validation() {
    let mut header = [0u8; 7];
    header[0..4].copy_from_slice(b"IFER");
    let mut table = ForgeTable::from_bytes(&header, true).expect("empty table");

    // Unforgeable item rejected
    assert!(
      table
        .create_weapon(ItemType::Vulnerary, "Bad", 0, 0, 0)
        .is_err()
    );
    assert!(
      table
        .create_weapon(ItemType::Falchion, "Bad", 0, 0, 0)
        .is_err()
    );

    // Out of bounds might/hit/crit rejected (total might > 99, hit > 150, crit < 0)
    assert!(
      table
        .create_weapon(ItemType::IronSword, "Good", 96, 0, 0)
        .is_err()
    );
    assert!(
      table
        .create_weapon(ItemType::IronSword, "Good", 0, 70, 0)
        .is_err()
    );
    assert!(
      table
        .create_weapon(ItemType::IronSword, "Good", 0, 0, -10)
        .is_err()
    );

    // Name too long rejected
    assert!(
      table
        .create_weapon(ItemType::IronSword, "A".repeat(19), 0, 0, 0)
        .is_err()
    );

    // Valid forge created
    let pos = table
      .create_weapon(ItemType::IronSword, "Blade+", 5, 10, 5)
      .expect("created");
    assert_eq!(pos, 0);
    assert_eq!(table.len(), 1);
    assert_eq!(table[0].name, "Blade+");
    assert_eq!(table[0].might, 5);
  }
}
