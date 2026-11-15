//! Borrowed unit views and TINU block iterators.
//!
//! Provides zero-allocation scanning of character unit records within
//! the raw TINU subsystem block.

use crate::save::convoy::item;
use crate::save::types;

use super::Unit;
use super::character;
use super::class;
use super::status::UnitStatus;

/// Borrowed view over an individual character unit slot.
#[derive(Debug, Clone, Copy)]
pub struct UnitRef<'a> {
  /// Backing unit entry byte slice.
  raw: &'a [u8],
}

impl<'a> UnitRef<'a> {
  /// Wraps a unit entry byte slice.
  #[must_use]
  pub fn new(raw: &'a [u8]) -> Self {
    Self { raw }
  }

  /// Character unit enumeration.
  #[must_use]
  pub fn character(&self) -> character::Character {
    character::Character::from_raw(u16::from_le_bytes([
      self.raw[0],
      self.raw[1],
    ]))
  }

  /// Biological gender resolved from the unit's character ID.
  #[must_use]
  pub fn gender(&self) -> character::Gender {
    let raw = u16::from_le_bytes([self.raw[0], self.raw[1]]);
    character::Character::from_raw_with_gender(raw).1
  }

  /// Current class enumeration.
  #[must_use]
  pub fn class(&self) -> class::Class {
    class::Class::from_raw(self.raw[2])
  }

  /// Character experience level.
  #[must_use]
  pub fn level(&self) -> u8 {
    if self.raw.len() > 17 { self.raw[17] } else { 1 }
  }

  /// Character experience points progress.
  #[must_use]
  pub fn exp(&self) -> u8 {
    if self.raw.len() > 18 { self.raw[18] } else { 0 }
  }

  /// Level-up stat growth bonus attributes (HP, Str, Mag, Skl, Spd, Lck, Def, Res).
  #[must_use]
  pub fn growths(&self) -> [u8; 8] {
    if self.raw.len() >= 17 {
      let mut g = [0u8; 8];
      g.copy_from_slice(&self.raw[9..17]);
      g
    } else {
      [0u8; 8]
    }
  }

  /// Helper to calculate combat attributes from class base, additions, and growths.
  fn calc_stat(&self, idx: usize) -> u8 {
    let character = self.character();
    let class = self.class();
    let growths = self.growths();
    let base = i16::from(class.base_stats()[idx])
      + i16::from(character.additions()[idx]);
    let max = if idx == 0 {
      99
    } else {
      (i16::from(class.max_stats()[idx])
        + i16::from(character.modifiers()[idx]))
      .max(0)
    };
    let total = base + i16::from(growths[idx]);
    u8::try_from(total.clamp(0, max)).unwrap_or(0)
  }

  /// Maximum hit points stat.
  #[must_use]
  pub fn hp(&self) -> u8 {
    self.calc_stat(0)
  }

  /// Strength stat.
  #[must_use]
  pub fn strength(&self) -> u8 {
    self.calc_stat(1)
  }

  /// Magic stat.
  #[must_use]
  pub fn magic(&self) -> u8 {
    self.calc_stat(2)
  }

  /// Skill stat.
  #[must_use]
  pub fn skill(&self) -> u8 {
    self.calc_stat(3)
  }

  /// Speed stat.
  #[must_use]
  pub fn speed(&self) -> u8 {
    self.calc_stat(4)
  }

  /// Luck stat.
  #[must_use]
  pub fn luck(&self) -> u8 {
    self.calc_stat(5)
  }

  /// Defense stat.
  #[must_use]
  pub fn defense(&self) -> u8 {
    self.calc_stat(6)
  }

  /// Resistance stat.
  #[must_use]
  pub fn resistance(&self) -> u8 {
    self.calc_stat(7)
  }

  /// Weapon proficiency ranks (Sword, Lance, Axe, Bow, Tome, Staff).
  #[must_use]
  pub fn weapon_ranks(&self) -> [u8; 6] {
    if self.raw.len() >= 66 {
      let mut w = [0u8; 6];
      w.copy_from_slice(&self.raw[60..66]);
      w
    } else {
      [0u8; 6]
    }
  }

  /// Inventory item slot at index `slot` (0..5).
  #[must_use]
  pub fn item(&self, slot: usize) -> Option<item::ItemSlot> {
    if slot >= 5 || self.raw.len() < 25 + (slot + 1) * 5 {
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

  /// Equipped skill identifier at index `slot` (0..5).
  #[must_use]
  pub fn skill_id(&self, slot: usize) -> u16 {
    let off = 50 + slot * 2;
    if slot < 5 && self.raw.len() >= off + 2 {
      u16::from_le_bytes([self.raw[off], self.raw[off + 1]])
    } else {
      0
    }
  }

  /// Current operational status given the active game mode.
  #[must_use]
  pub fn status(&self, mode: Option<types::GameMode>) -> UnitStatus {
    if mode == Some(types::GameMode::Casual) {
      return UnitStatus::Active;
    }
    if self.hp() == 0 {
      if self.character().is_story_anchor() {
        UnitStatus::Retreated
      } else {
        UnitStatus::Dead
      }
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
  /// Save slot game mode context.
  mode: Option<types::GameMode>,
}

impl<'a> UnitIter<'a> {
  /// Creates a new unit iterator over a TINU block slice.
  #[must_use]
  pub(crate) fn new(raw: &'a [u8], mode: Option<types::GameMode>) -> Self {
    Self { raw, pos: 0, mode }
  }

  /// Collects all valid units from a TINU block slice.
  #[must_use]
  pub(crate) fn collect_units(
    raw: &'a [u8],
    mode: Option<types::GameMode>,
  ) -> Vec<Unit> {
    let iter = Self::new(raw, mode);
    let mode_val = iter.mode;
    iter
      .map(|(offset, r)| Unit::from_ref(offset, &r, mode_val))
      .collect()
  }
}

impl<'a> Iterator for UnitIter<'a> {
  type Item = (usize, UnitRef<'a>);

  fn next(&mut self) -> Option<Self::Item> {
    while self.pos + 60 <= self.raw.len() {
      let i = self.pos;
      let uid = u16::from_le_bytes([self.raw[i], self.raw[i + 1]]);
      let cid = self.raw[i + 2];
      let lvl = self.raw[i + 17];
      let exp = self.raw[i + 18];
      let has_items_pattern = self.raw[i + 25] == 0x04
        && self.raw[i + 30] == 0x04
        && self.raw[i + 35] == 0x04
        && self.raw[i + 40] == 0x04
        && self.raw[i + 45] == 0x04;
      if uid <= 65 && cid <= 82 && lvl <= 40 && exp <= 100 && has_items_pattern
      {
        self.pos += 50;
        return Some((i, UnitRef::new(&self.raw[i..])));
      }
      self.pos += 1;
    }
    None
  }
}
