//! World map paralogue progression and child parentage associations.
//!
//! Tracks paralogue completion states and maps child units to their canonical
//! paralogue chapters and fixed mothers. Caller validates roster state before
//! altering stage progression.

use super::roster::Character;

/// Progression state of a world map stage or paralogue node.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ParalogueState {
  /// Stage is locked and hidden on the world map.
  Locked,
  /// Stage has been completed and cleared.
  Completed,
  /// Stage is unlocked and available for battle.
  Unlocked,
}

impl ParalogueState {
  /// Converts a raw byte value into a [`ParalogueState`].
  #[must_use]
  pub const fn from_raw(raw: u8) -> Self {
    match raw {
      1 => Self::Completed,
      2 => Self::Unlocked,
      _ => Self::Locked,
    }
  }

  /// Converts a [`ParalogueState`] variant into its raw byte representation.
  #[must_use]
  pub const fn raw_id(self) -> u8 {
    match self {
      Self::Locked => 0,
      Self::Completed => 1,
      Self::Unlocked => 2,
    }
  }
}

/// Returns the canonical child character associated with a child paralogue.
///
/// Child paralogues span chapters 5 through 16. Returns `None` for unmapped
/// stages.
#[must_use]
pub const fn child_for_paralogue(paralogue: u8) -> Option<Character> {
  match paralogue {
    5 => Some(Character::Owain),
    6 => Some(Character::Inigo),
    7 => Some(Character::Brady),
    8 => Some(Character::Kjelle),
    9 => Some(Character::Cynthia),
    10 => Some(Character::Severa),
    11 => Some(Character::Gerome),
    12 => Some(Character::Morgan),
    13 => Some(Character::Yarne),
    14 => Some(Character::Laurent),
    15 => Some(Character::Noire),
    16 => Some(Character::Nah),
    _ => None,
  }
}

/// Returns the immutable mother/primary parent of a child character.
///
/// Returns `None` for units that do not have fixed parentage in FE13.
#[must_use]
pub const fn child_primary_parent(child: Character) -> Option<Character> {
  match child {
    Character::Lucina => Some(Character::Chrom),
    Character::Owain => Some(Character::Lissa),
    Character::Inigo => Some(Character::Olivia),
    Character::Brady => Some(Character::Maribelle),
    Character::Kjelle => Some(Character::Sully),
    Character::Cynthia => Some(Character::Sumia),
    Character::Severa => Some(Character::Cordelia),
    Character::Gerome => Some(Character::Cherche),
    Character::Morgan => Some(Character::Robin),
    Character::Yarne => Some(Character::Panne),
    Character::Laurent => Some(Character::Miriel),
    Character::Noire => Some(Character::Tharja),
    Character::Nah => Some(Character::Nowi),
    _ => None,
  }
}
