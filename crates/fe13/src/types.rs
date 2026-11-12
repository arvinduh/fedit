//! Domain identifier newtypes and game configuration enumerations.
//!
//! Owns typed wrappers for unit, class, item IDs, difficulty, and mode.

/// Unique identifier for a character unit slot or prototype.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnitId(pub u16);

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
