//! `Roster` domain aggregate and army character management.
//!
//! Owns the collection of story units, enforcing army uniqueness invariants,
//! anchor unit protections, and status transitions (revivals and retirements).

use std::ops;
use std::slice;

use thiserror;

use crate::save::header::config;

pub(crate) mod groups;
pub(crate) mod unit;

pub use groups::Error as GroupError;
pub use groups::UnitGroups;
pub use unit::Availability;
pub use unit::BattleState;
pub use unit::Character;
pub use unit::Class;
pub use unit::Error as UnitError;
pub use unit::Gender;
pub use unit::Skill;
pub use unit::Stat;
pub use unit::StatError;
pub use unit::StatKind;
pub use unit::Unit;
pub use unit::UnitStatus;

/// Maximum number of units supported in the TINU roster block.
pub const MAX_ROSTER_CAPACITY: usize = 50;

/// Errors that can occur during roster management.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Critical story anchor character cannot be removed.
  #[error("story anchor unit '{character}' cannot be removed")]
  CannotRemoveStoryAnchor {
    /// Protected character.
    character: Character,
  },
  /// Unit with the specified character is already present in the roster.
  #[error("character '{character}' is already present in the roster")]
  DuplicateCharacter {
    /// Existing character.
    character: Character,
  },
  /// Roster capacity exceeded maximum allowable units.
  #[error("roster capacity exceeded: {count} >= {max}")]
  RosterCapacityExceeded {
    /// Current unit count.
    count: usize,
    /// Maximum allowed count.
    max: usize,
  },
  /// Unit with the specified name was not found in the roster.
  #[error("unit '{name}' not found")]
  UnitNotFound {
    /// Target name.
    name: String,
  },
  /// Unit index was out of bounds.
  #[error("unit index {index} out of bounds")]
  UnitIndexOutOfBounds {
    /// Target index.
    index: usize,
  },
  /// Unit error.
  #[error(transparent)]
  Unit(#[from] unit::Error),
}

/// Encapsulated army roster of story and recruited units.
#[derive(Debug)]
pub struct Roster {
  /// Partitioned unit groups.
  groups: UnitGroups,
}

impl Roster {
  /// Returns a borrowed reference to the unit groups.
  #[must_use]
  pub(crate) const fn groups(&self) -> &UnitGroups {
    &self.groups
  }

  /// Returns a mutable reference to the unit groups.
  pub(crate) fn groups_mut(&mut self) -> &mut UnitGroups {
    &mut self.groups
  }
  /// Parses the roster and all unit groups from a TINU byte slice.
  ///
  /// # Errors
  ///
  /// Returns [`Error::Unit`] if the TINU slice is malformed.
  pub fn from_bytes(
    raw: &[u8],
    is_west: bool,
    mode: Option<config::GameMode>,
  ) -> Result<Self, Error> {
    let groups = UnitGroups::from_bytes(raw, is_west, mode)?;
    Ok(Self { groups })
  }

  /// Serializes all unit groups back into a contiguous binary TINU block.
  #[must_use]
  pub fn to_bytes(&self) -> Vec<u8> {
    self.groups.to_bytes()
  }

  /// Adds a new unit to the main camp roster with capacity, duplicate, and gender validation.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::RosterCapacityExceeded`] if `self.len() >= MAX_ROSTER_CAPACITY`.
  /// - Returns [`Error::DuplicateCharacter`] if a unit with the same character already exists.
  /// - Returns [`unit::Error::ClassGenderMismatch`] if the class is ineligible for the character's gender.
  pub fn add_unit(
    &mut self,
    character: Character,
    class: Class,
  ) -> Result<&mut Unit, Error> {
    if self.groups.main.len() >= MAX_ROSTER_CAPACITY {
      return Err(Error::RosterCapacityExceeded {
        count: self.groups.main.len(),
        max: MAX_ROSTER_CAPACITY,
      });
    }
    if self.groups.main.iter().any(|u| u.character() == character) {
      return Err(Error::DuplicateCharacter { character });
    }
    if !class.is_eligible(character.gender()) {
      return Err(Error::Unit(unit::Error::ClassGenderMismatch {
        class,
        gender: character.gender(),
      }));
    }
    let unit = Unit::new(character, class, 0);
    self.groups.main.push(unit);
    let last_idx = self.groups.main.len() - 1;
    Ok(&mut self.groups.main[last_idx])
  }

  /// Removes a unit from the main roster at `index` with story anchor protections.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::CannotRemoveStoryAnchor`] if the unit is a story anchor.
  /// - Returns [`unit::Error::SlotOutOfBounds`] if `index >= self.len()`.
  pub fn remove_unit(&mut self, index: usize) -> Result<Unit, Error> {
    if index >= self.groups.main.len() {
      return Err(Error::Unit(unit::Error::SlotOutOfBounds { slot: index }));
    }
    if self.groups.main[index].is_story_anchor() {
      return Err(Error::CannotRemoveStoryAnchor {
        character: self.groups.main[index].character(),
      });
    }
    Ok(self.groups.main.remove(index))
  }

  /// Number of characters currently in the main camp roster.
  #[must_use]
  pub fn len(&self) -> usize {
    self.groups.main.len()
  }

  /// Returns true if the main camp roster contains no units.
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.groups.main.is_empty()
  }

  /// Returns a borrowed reference to the unit at the specified index.
  #[must_use]
  pub fn get(&self, index: usize) -> Option<&Unit> {
    self.groups.main.get(index)
  }

  /// Returns a mutable reference to the unit at the specified index.
  pub fn get_mut(&mut self, index: usize) -> Option<&mut Unit> {
    self.groups.main.get_mut(index)
  }

  /// Returns an iterator over immutable unit references in the main camp roster.
  pub fn iter(&self) -> slice::Iter<'_, Unit> {
    self.groups.main.iter()
  }

  /// Returns an iterator over mutable unit references in the main camp roster.
  pub fn iter_mut(&mut self) -> slice::IterMut<'_, Unit> {
    self.groups.main.iter_mut()
  }

  /// Finds a unit in the roster by character name (case-insensitive).
  #[must_use]
  pub fn find_by_name(&self, name: &str) -> Option<&Unit> {
    self
      .groups
      .main
      .iter()
      .find(|u| u.name().eq_ignore_ascii_case(name))
  }

  /// Finds a mutable unit in the roster by character name (case-insensitive).
  pub fn find_by_name_mut(&mut self, name: &str) -> Option<&mut Unit> {
    self
      .groups
      .main
      .iter_mut()
      .find(|u| u.name().eq_ignore_ascii_case(name))
  }

  /// Revives a dead or retreated unit at the given index to active status.
  ///
  /// # Errors
  ///
  /// Returns [`unit::Error::SlotOutOfBounds`] if `index >= self.len()`.
  pub fn revive_unit(&mut self, index: usize) -> Result<(), Error> {
    let Some(unit) = self.groups.main.get_mut(index) else {
      return Err(Error::Unit(unit::Error::SlotOutOfBounds { slot: index }));
    };
    unit.revive();
    Ok(())
  }

  /// Sets the operational deployment status of the unit at the given index.
  ///
  /// # Errors
  ///
  /// - Returns [`unit::Error::SlotOutOfBounds`] if `index >= self.len()`.
  /// - Returns [`Error::CannotRemoveStoryAnchor`] if attempting to set any story
  ///   anchor character (Chrom, Robin, Lucina) to [`UnitStatus::Dead`].
  pub fn set_unit_status(
    &mut self,
    index: usize,
    status: UnitStatus,
  ) -> Result<(), Error> {
    let Some(unit) = self.groups.main.get_mut(index) else {
      return Err(Error::Unit(unit::Error::SlotOutOfBounds { slot: index }));
    };
    if unit.is_story_anchor() && status == UnitStatus::Dead {
      return Err(Error::CannotRemoveStoryAnchor {
        character: unit.character(),
      });
    }
    unit.set_status(status)?;
    Ok(())
  }
}

impl ops::Index<usize> for Roster {
  type Output = Unit;

  fn index(&self, index: usize) -> &Self::Output {
    &self.groups.main[index]
  }
}

impl ops::IndexMut<usize> for Roster {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.groups.main[index]
  }
}

impl<'a> IntoIterator for &'a Roster {
  type Item = &'a Unit;
  type IntoIter = slice::Iter<'a, Unit>;

  fn into_iter(self) -> Self::IntoIter {
    self.groups.main.iter()
  }
}

impl<'a> IntoIterator for &'a mut Roster {
  type Item = &'a mut Unit;
  type IntoIter = slice::IterMut<'a, Unit>;

  fn into_iter(self) -> Self::IntoIter {
    self.groups.main.iter_mut()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_test_roster(units: Vec<Unit>) -> Roster {
    Roster {
      groups: UnitGroups {
        blue: Vec::new(),
        red: Vec::new(),
        green: Vec::new(),
        main: units,
        dead: Vec::new(),
        other: Vec::new(),
        header: b"TINU\x01".to_vec(),
        footer: Vec::new(),
      },
    }
  }

  #[test]
  fn test_roster_anchor_protections() {
    let chrom = Unit::new(Character::Chrom, Class::Lord, 0);
    let donnel = Unit::new(Character::Donnel, Class::Villager, 180);
    let mut roster = make_test_roster(vec![chrom, donnel]);

    // Chrom (index 0) cannot be killed
    assert!(roster.set_unit_status(0, UnitStatus::Dead).is_err());

    // Chrom can be retreated
    assert!(roster.set_unit_status(0, UnitStatus::Retreated).is_ok());

    // Donnel (index 1) can be killed
    assert!(roster.set_unit_status(1, UnitStatus::Dead).is_ok());

    // Reviving Donnel
    assert!(roster.revive_unit(1).is_ok());
    assert_eq!(roster[1].status(), UnitStatus::Active);

    // Out of bounds index
    assert!(roster.set_unit_status(99, UnitStatus::Active).is_err());

    // Chrom cannot be removed
    assert!(roster.remove_unit(0).is_err());
    // Donnel can be removed
    assert!(roster.remove_unit(1).is_ok());
    assert_eq!(roster.len(), 1);
  }

  #[test]
  fn test_roster_find_by_name() {
    let chrom = Unit::new(Character::Chrom, Class::Lord, 0);
    let roster = make_test_roster(vec![chrom]);
    assert!(roster.find_by_name("chrom").is_some());
    assert!(roster.find_by_name("CHROM").is_some());
    assert!(roster.find_by_name("Robin").is_none());
  }

  #[test]
  fn test_roster_add_unit() {
    let mut roster = make_test_roster(Vec::new());

    // Successful addition
    let unit = roster.add_unit(Character::Chrom, Class::Lord);
    assert!(unit.is_ok());
    assert_eq!(roster.len(), 1);

    // Duplicate character prevention
    let dup = roster.add_unit(Character::Chrom, Class::GreatLord);
    assert!(matches!(dup, Err(Error::DuplicateCharacter { .. })));

    // Ineligible class gender mismatch (Male Chrom cannot be PegasusKnight)
    let bad_gender = roster.add_unit(Character::Donnel, Class::PegasusKnight);
    assert!(matches!(
      bad_gender,
      Err(Error::Unit(unit::Error::ClassGenderMismatch { .. }))
    ));
  }
}
