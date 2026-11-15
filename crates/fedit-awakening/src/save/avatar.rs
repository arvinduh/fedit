//! Avatar customization representation and serialization.
//!
//! Owns the `AvatarCustomization` model and read/write operations
//! for Robin's 392-byte `LogBlock` and preview header synchronizations.

use crate::error;
use crate::slot::roster::unit::character;
use crate::slot::text;

/// Asset or flaw stat focus for Robin's avatar customization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum StatFocus {
  /// Hit Points focus.
  Hp,
  /// Strength combat focus.
  Strength,
  /// Magic combat focus.
  Magic,
  /// Skill combat focus.
  Skill,
  /// Speed combat focus.
  Speed,
  /// Luck combat focus.
  Luck,
  /// Defense combat focus.
  Defense,
  /// Resistance combat focus.
  Resistance,
}

impl StatFocus {
  /// Converts a raw byte to a [`StatFocus`].
  #[must_use]
  pub const fn from_raw(raw: u8) -> Self {
    match raw {
      1 => Self::Strength,
      2 => Self::Magic,
      3 => Self::Skill,
      4 => Self::Speed,
      5 => Self::Luck,
      6 => Self::Defense,
      7 => Self::Resistance,
      _ => Self::Hp,
    }
  }

  /// Converts a [`StatFocus`] to its raw byte ID.
  #[must_use]
  pub const fn raw_id(self) -> u8 {
    match self {
      Self::Hp => 0,
      Self::Strength => 1,
      Self::Magic => 2,
      Self::Skill => 3,
      Self::Speed => 4,
      Self::Luck => 5,
      Self::Defense => 6,
      Self::Resistance => 7,
    }
  }
}

/// Owned domain representation of Robin's avatar customization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AvatarCustomization {
  /// Avatar display name (up to 12 UTF-16 characters).
  pub name: String,
  /// Biological gender.
  pub gender: character::Gender,
  /// Asset attribute boost.
  pub asset: StatFocus,
  /// Flaw attribute penalty.
  pub flaw: StatFocus,
  /// Body build variation index (0..=2).
  pub build: u8,
  /// Face variation index (0..=4).
  pub face: u8,
  /// Hair style variation index (0..=4).
  pub hair: u8,
  /// Hair color RGB value.
  pub hair_color: [u8; 3],
  /// Voice variation index (0..=2).
  pub voice: u8,
  /// Birthday month (1..=12).
  pub birthday_month: u8,
  /// Birthday day (1..=31).
  pub birthday_day: u8,
}

impl AvatarCustomization {
  /// Minimum byte length for an avatar `LogBlock` slice.
  pub const LOG_BLOCK_LEN: usize = 392;

  /// Parses an [`AvatarCustomization`] from a 392-byte `LogBlock` slice.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::OffsetOutOfBounds`] if slice is shorter than [`Self::LOG_BLOCK_LEN`].
  pub fn parse(slice: &[u8]) -> Result<Self, error::Error> {
    if slice.len() < Self::LOG_BLOCK_LEN {
      return Err(error::Error::OffsetOutOfBounds {
        offset: Self::LOG_BLOCK_LEN,
        len: slice.len(),
      });
    }

    let name = text::read_utf16le(&slice[0..26]);
    let main = &slice[26..127];

    let asset = StatFocus::from_raw(main[0]);
    let flaw = StatFocus::from_raw(main[1]);
    let gender = if main[2] == 1 {
      character::Gender::Female
    } else {
      character::Gender::Male
    };
    let build = main[3];
    let face = main[4];
    let hair = main[5];
    let hair_color = [main[6], main[7], main[8]];
    let voice = main[10];
    let birthday_month = main[11].clamp(1, 12);
    let birthday_day = main[12].clamp(1, 31);

    Ok(Self {
      name,
      gender,
      asset,
      flaw,
      build,
      face,
      hair,
      hair_color,
      voice,
      birthday_month,
      birthday_day,
    })
  }

  /// Writes modified avatar customization attributes back into a `LogBlock` slice.
  ///
  /// # Side Effects
  ///
  /// Modifies the name, asset, flaw, gender, visual appearance, and birthday
  /// bytes in the given slice.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::OffsetOutOfBounds`] if `slice.len() < Self::LOG_BLOCK_LEN`.
  pub fn write_back(&self, slice: &mut [u8]) -> Result<(), error::Error> {
    if slice.len() < Self::LOG_BLOCK_LEN {
      return Err(error::Error::OffsetOutOfBounds {
        offset: Self::LOG_BLOCK_LEN,
        len: slice.len(),
      });
    }

    text::write_utf16le(&mut slice[0..26], &self.name);
    let main = &mut slice[26..127];

    main[0] = self.asset.raw_id();
    main[1] = self.flaw.raw_id();
    main[2] = match self.gender {
      character::Gender::Female => 1,
      character::Gender::Male => 0,
    };
    main[3] = self.build;
    main[4] = self.face;
    main[5] = self.hair;
    main[6..9].copy_from_slice(&self.hair_color);
    main[10] = self.voice;
    main[11] = self.birthday_month;
    main[12] = self.birthday_day;

    Ok(())
  }
}
