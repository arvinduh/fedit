//! `Roster` domain aggregate and army character management.
//!
//! Owns the collection of story units, enforcing army uniqueness invariants,
//! anchor unit protections, and status transitions (revivals and retirements).

use std::ops;

use crate::error;

pub mod unit;

pub use unit::Availability;
pub use unit::Character;
pub use unit::Class;
pub use unit::Gender;
pub use unit::Skill;
pub use unit::Unit;
pub use unit::UnitStats;
pub use unit::UnitStatus;

/// Encapsulated army roster of story and recruited units.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Roster {
  /// Internal unit storage.
  units: Vec<Unit>,
}

impl Roster {
  /// Constructs a new roster from a collection of units.
  #[must_use]
  pub fn new(units: Vec<Unit>) -> Self {
    Self { units }
  }

  /// Converts the roster into an owned vector of units.
  #[must_use]
  pub fn into_vec(self) -> Vec<Unit> {
    self.units
  }

  /// Clones the roster into an owned vector of units.
  #[must_use]
  pub fn to_vec(&self) -> Vec<Unit> {
    self.units.clone()
  }

  /// Appends a new unit to the roster.
  pub fn push(&mut self, unit: Unit) {
    self.units.push(unit);
  }

  /// Number of characters currently in the roster.
  #[must_use]
  pub fn len(&self) -> usize {
    self.units.len()
  }

  /// Returns true if the roster contains no units.
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.units.is_empty()
  }

  /// Returns a borrowed reference to the unit at the specified index.
  #[must_use]
  pub fn get(&self, index: usize) -> Option<&Unit> {
    self.units.get(index)
  }

  /// Returns a mutable reference to the unit at the specified index.
  pub fn get_mut(&mut self, index: usize) -> Option<&mut Unit> {
    self.units.get_mut(index)
  }

  /// Returns an iterator over immutable unit references.
  pub fn iter(&self) -> std::slice::Iter<'_, Unit> {
    self.units.iter()
  }

  /// Returns an iterator over mutable unit references.
  pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Unit> {
    self.units.iter_mut()
  }

  /// Finds a unit in the roster by character name (case-insensitive).
  #[must_use]
  pub fn find_by_name(&self, name: &str) -> Option<&Unit> {
    self
      .units
      .iter()
      .find(|u| u.name().eq_ignore_ascii_case(name))
  }

  /// Finds a mutable unit in the roster by character name (case-insensitive).
  pub fn find_by_name_mut(&mut self, name: &str) -> Option<&mut Unit> {
    self
      .units
      .iter_mut()
      .find(|u| u.name().eq_ignore_ascii_case(name))
  }

  /// Revives a dead or retreated unit at the given index to active status.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::SlotOutOfBounds`] if `index >= self.len()`.
  pub fn revive_unit(&mut self, index: usize) -> Result<(), error::Error> {
    let Some(unit) = self.units.get_mut(index) else {
      return Err(error::Error::SlotOutOfBounds { slot: index });
    };
    unit.revive();
    Ok(())
  }

  /// Sets the operational deployment status of the unit at the given index.
  ///
  /// # Errors
  ///
  /// - Returns [`error::Error::SlotOutOfBounds`] if `index >= self.len()`.
  /// - Returns [`error::Error::AnchorUnitProtected`] if attempting to set any story
  ///   anchor character (Chrom, Robin, Lucina) to [`UnitStatus::Dead`].
  pub fn set_unit_status(
    &mut self,
    index: usize,
    status: UnitStatus,
  ) -> Result<(), error::Error> {
    let Some(unit) = self.units.get_mut(index) else {
      return Err(error::Error::SlotOutOfBounds { slot: index });
    };
    unit.set_status(status)
  }
}

impl ops::Index<usize> for Roster {
  type Output = Unit;

  fn index(&self, index: usize) -> &Self::Output {
    &self.units[index]
  }
}

impl ops::IndexMut<usize> for Roster {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.units[index]
  }
}

impl<'a> IntoIterator for &'a Roster {
  type Item = &'a Unit;
  type IntoIter = std::slice::Iter<'a, Unit>;

  fn into_iter(self) -> Self::IntoIter {
    self.units.iter()
  }
}

impl<'a> IntoIterator for &'a mut Roster {
  type Item = &'a mut Unit;
  type IntoIter = std::slice::IterMut<'a, Unit>;

  fn into_iter(self) -> Self::IntoIter {
    self.units.iter_mut()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_roster_anchor_protections() {
    let chrom = Unit::new(
      Character::Chrom,
      Class::Lord,
      UnitStats {
        hp: 40,
        ..Default::default()
      },
      0,
    );
    let donnel = Unit::new(
      Character::Donnel,
      Class::Villager,
      UnitStats {
        hp: 20,
        ..Default::default()
      },
      180,
    );
    let mut roster = Roster::new(vec![chrom, donnel]);

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
  }

  #[test]
  fn test_roster_find_by_name() {
    let chrom = Unit::new(
      Character::Chrom,
      Class::Lord,
      UnitStats {
        hp: 40,
        ..Default::default()
      },
      0,
    );
    let roster = Roster::new(vec![chrom]);
    assert!(roster.find_by_name("chrom").is_some());
    assert!(roster.find_by_name("CHROM").is_some());
    assert!(roster.find_by_name("Robin").is_none());
  }
}
