//! Avatar customization representation and serialization.
//!
//! Owns the `AvatarCustomization` model and read/write operations
//! for Robin's 392-byte `LogBlock` and preview header synchronizations.

use thiserror;

use crate::save::roster::unit::character;
use crate::save::utils::text;
use crate::save::{Error as SaveError, Save, roster};

pub(crate) mod rgb;

pub use rgb::Rgb;

/// Errors that can occur during avatar customization operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// Avatar block offset was out of bounds for the payload slice.
  #[error("avatar block offset out of bounds: {offset} > {len}")]
  OffsetOutOfBounds {
    /// Resolved offset.
    offset: usize,
    /// Slice length.
    len: usize,
  },
}

/// Asset or flaw stat focus for Robin's avatar customization.
#[derive(Clone, Copy, Debug)]
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
  pub(crate) const fn from_raw(raw: u8) -> Self {
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
  pub(crate) const fn raw_id(self) -> u8 {
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
#[derive(Clone, Debug)]
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
  pub hair_color: Rgb,
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
  /// Returns [`Error::OffsetOutOfBounds`] if slice is shorter than [`Self::LOG_BLOCK_LEN`].
  pub fn parse(slice: &[u8]) -> Result<Self, Error> {
    if slice.len() < Self::LOG_BLOCK_LEN {
      return Err(Error::OffsetOutOfBounds {
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
    let hair_color = Rgb::from_raw([main[6], main[7], main[8]]);
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
  /// Returns [`Error::OffsetOutOfBounds`] if `slice.len() < Self::LOG_BLOCK_LEN`.
  pub(crate) fn write_back(&self, slice: &mut [u8]) -> Result<(), Error> {
    if slice.len() < Self::LOG_BLOCK_LEN {
      return Err(Error::OffsetOutOfBounds {
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
    main[6..9].copy_from_slice(&self.hair_color.to_raw());
    main[10] = self.voice;
    main[11] = self.birthday_month;
    main[12] = self.birthday_day;

    Ok(())
  }
}

impl Save {
  /// Resolves the decompressed payload byte offset of Robin's `LogBlock`.
  fn find_avatar_log_offset(&self) -> Result<usize, SaveError> {
    let robin_name = self.preview().avatar_name();
    let mut name_bytes = [0u8; 26];
    text::write_utf16le(&mut name_bytes, &robin_name);
    let search_prefix = if robin_name.is_empty() {
      &b"R\x00o\x00"[..]
    } else {
      let len = (robin_name.chars().count() * 2).min(26);
      &name_bytes[..len]
    };

    let start = self.index.unit_offset;
    let end = self.index.ifer_offset.min(self.payload.len());
    let min_len = AvatarCustomization::LOG_BLOCK_LEN;
    if start >= end || end.saturating_sub(start) < min_len {
      return Err(SaveError::AvatarNotFound);
    }

    for i in start..=end - min_len {
      if self.payload[i..].starts_with(search_prefix) {
        let asset = self.payload[i + 26];
        let flaw = self.payload[i + 27];
        let gender = self.payload[i + 28];
        if asset <= 7 && flaw <= 7 && gender <= 1 {
          return Ok(i);
        }
      }
    }
    Err(SaveError::AvatarNotFound)
  }

  /// Returns Robin's avatar customization attributes.
  ///
  /// # Errors
  ///
  /// Returns [`SaveError::AvatarNotFound`] if the `LogBlock` is not found.
  pub fn avatar_customization(&self) -> Result<AvatarCustomization, SaveError> {
    let offset = self.find_avatar_log_offset()?;
    AvatarCustomization::parse(&self.payload[offset..]).map_err(SaveError::from)
  }

  /// Sets Robin's avatar customization attributes with strict invariant checks.
  ///
  /// # Side Effects
  ///
  /// - Synchronizes the avatar display name in the preview header.
  /// - Modifies the 392-byte `LogBlock` attributes in the save payload.
  /// - If the avatar gender changes, synchronizes Robin's PID ($0 \leftrightarrow 1$)
  ///   and Morgan's PID ($39 \leftrightarrow 40$) and verifies class compatibility.
  ///
  /// # Errors
  ///
  /// - Returns [`SaveError::Unit`] if Robin's or Morgan's current class is
  ///   ineligible for the new gender.
  /// - Returns [`SaveError::AvatarNotFound`] if the `LogBlock` is not found.
  pub fn set_avatar_customization(
    &mut self,
    custom: &AvatarCustomization,
  ) -> Result<(), SaveError> {
    let log_offset = self.find_avatar_log_offset()?;
    let current_custom =
      AvatarCustomization::parse(&self.payload[log_offset..])
        .map_err(SaveError::from)?;

    if custom.gender != current_custom.gender {
      if let Some(robin) = self.roster.iter().find(|u| u.character().is_robin())
        && !robin.class().is_eligible(custom.gender)
      {
        return Err(SaveError::from(
          roster::unit::Error::ClassGenderMismatch {
            class: robin.class(),
            gender: custom.gender,
          },
        ));
      }
      let morgan_gender = custom.gender.opposite();
      if let Some(morgan) =
        self.roster.iter().find(|u| u.character().is_morgan())
        && !morgan.class().is_eligible(morgan_gender)
      {
        return Err(SaveError::from(
          roster::unit::Error::ClassGenderMismatch {
            class: morgan.class(),
            gender: morgan_gender,
          },
        ));
      }

      if let Some(robin) =
        self.roster.iter_mut().find(|u| u.character().is_robin())
      {
        robin.set_gender(custom.gender).map_err(SaveError::from)?;
      }
      if let Some(morgan) =
        self.roster.iter_mut().find(|u| u.character().is_morgan())
      {
        morgan.set_gender(morgan_gender).map_err(SaveError::from)?;
      }
    }

    self.preview_mut().set_avatar_name(&custom.name);
    custom
      .write_back(&mut self.payload[log_offset..])
      .map_err(SaveError::from)?;
    Ok(())
  }
}
