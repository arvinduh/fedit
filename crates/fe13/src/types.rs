//! Domain identifier newtypes and game configuration enumerations.
//!
//! Owns typed wrappers for unit, class, item IDs, difficulty, and mode.

use std::fmt;

/// Unique identifier for a character unit slot or prototype.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnitId(pub u16);

impl UnitId {
  /// Canonical character name for the unit identifier.
  #[must_use]
  pub const fn name(&self) -> Option<&'static str> {
    crate::names::character_name(self.0)
  }
}

impl fmt::Display for UnitId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.name() {
      Some(name) => write!(f, "{name}"),
      None => write!(f, "Unit({})", self.0),
    }
  }
}

/// Unique identifier for a unit class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClassId(pub u8);

/// Unique identifier for an inventory or convoy item.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ItemId(pub u16);

/// Game difficulty setting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Difficulty {
  /// Normal difficulty.
  Normal = 0,
  /// Hard difficulty.
  Hard = 1,
  /// Lunatic difficulty.
  Lunatic = 2,
  /// Lunatic+ difficulty.
  LunaticPlus = 3,
}

impl Difficulty {
  /// Converts a raw byte to a difficulty variant.
  #[must_use]
  pub const fn from_raw(raw: u8) -> Option<Self> {
    match raw {
      0 => Some(Self::Normal),
      1 => Some(Self::Hard),
      2 => Some(Self::Lunatic),
      3 => Some(Self::LunaticPlus),
      _ => None,
    }
  }
}

/// Permanent death gameplay mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameMode {
  /// Classic mode with permanent unit death.
  Classic = 0,
  /// Casual mode with unit revival after chapter victory.
  Casual = 1,
}

impl GameMode {
  /// Converts a raw byte to a game mode variant.
  #[must_use]
  pub const fn from_raw(raw: u8) -> Option<Self> {
    match raw {
      0 => Some(Self::Classic),
      1 => Some(Self::Casual),
      _ => None,
    }
  }
}

impl fmt::Display for Difficulty {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Normal => write!(f, "Normal"),
      Self::Hard => write!(f, "Hard"),
      Self::Lunatic => write!(f, "Lunatic"),
      Self::LunaticPlus => write!(f, "Lunatic+"),
    }
  }
}

impl fmt::Display for GameMode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Classic => write!(f, "Classic"),
      Self::Casual => write!(f, "Casual"),
    }
  }
}
