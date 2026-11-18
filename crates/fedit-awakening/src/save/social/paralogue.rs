//! World map paralogue progression and child parentage associations.
//!
//! Tracks paralogue completion states and maps child units to their canonical
//! paralogue chapters and fixed mothers. Caller validates roster state before
//! altering stage progression.

use thiserror;

use crate::save::roster::unit::character;

/// Errors that can occur when updating paralogue progression.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// Child paralogues cannot be unlocked before Chapter 13 is cleared.
  #[error("child paralogue {paralogue} requires chapter 13 to be cleared")]
  ChildParalogueGatedByChapter13 {
    /// Target paralogue index.
    paralogue: u8,
  },
  /// Child unit parents are not married (support rank S not achieved).
  #[error(
    "cannot unlock child paralogue: parents of '{child}' are not married"
  )]
  ParentsNotMarried {
    /// Child character.
    child: character::Character,
  },
  /// Cannot relock or reset completed paralogue while child is in roster.
  #[error(
    "cannot rollback paralogue {paralogue}: child unit '{child}' is already in roster"
  )]
  ChildInRosterParalogueConflict {
    /// Recruited child character.
    child: character::Character,
    /// Target paralogue index.
    paralogue: u8,
  },
  /// Character is not a canonical child unit with variable parentage.
  #[error("character '{child}' does not have variable parentage")]
  NotAChildCharacter {
    /// Non-child character.
    child: character::Character,
  },
}

/// Progression state of a world map stage or paralogue node.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
  pub(crate) const fn from_raw(raw: u8) -> Self {
    match raw {
      1 => Self::Completed,
      2 => Self::Unlocked,
      _ => Self::Locked,
    }
  }

  /// Converts a [`ParalogueState`] variant into its raw byte representation.
  #[must_use]
  pub(crate) const fn raw_id(self) -> u8 {
    match self {
      Self::Locked => 0,
      Self::Completed => 1,
      Self::Unlocked => 2,
    }
  }
}
