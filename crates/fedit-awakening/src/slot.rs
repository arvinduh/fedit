//! `SaveSlot` container aggregate and codecs.
//!
//! Owns the uncompressed save slot aggregate, parsing and repacking archives.

use fe_compress;

use crate::error;

pub mod convoy;
pub mod roster;
pub mod support;
pub mod types;

mod index;
mod preview;
mod text;
mod time;
mod user;

/// An uncompressed, editable Fire Emblem Awakening save slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SaveSlot {
  /// Raw 192-byte preview header buffer.
  preview: [u8; preview::PREVIEW_LEN],
  /// Decompressed EDNI save payload buffer.
  payload: Vec<u8>,
  /// Resolved subsystem block index table.
  index: index::EdniIndex,
  /// Encapsulated unit roster.
  pub roster: roster::Roster,
  /// Owned convoy storage items.
  pub convoy: Vec<convoy::ConvoyItem>,
  /// Army-wide support tracking table.
  pub supports: support::SupportTable,
}

impl SaveSlot {
  /// Parses a save slot from raw compressed container bytes.
  ///
  /// # Errors
  ///
  /// Fails on invalid container, corrupt stream, or bad EDNI index.
  pub fn from_bytes(raw: &[u8]) -> Result<Self, error::Error> {
    let container = fe_compress::unpack(raw)?;
    let preview: [u8; preview::PREVIEW_LEN] =
      container.preview.as_slice().try_into().map_err(|_| {
        error::Error::InvalidPreviewLength {
          expected: preview::PREVIEW_LEN,
          actual: container.preview.len(),
        }
      })?;
    let index = index::EdniIndex::parse(&container.payload)?;
    let units = roster::unit::UnitIter::collect_units(
      &container.payload[index.unit_offset..index.ifer_offset],
    );
    let roster = roster::Roster::new(units);
    let convoy = convoy::ConvoyRef::new(
      &container.payload[index.convoy_offset..index.convoy_end_offset],
    )
    .items();
    let supports = support::SupportTable::new();
    Ok(Self {
      preview,
      payload: container.payload,
      index,
      roster,
      convoy,
      supports,
    })
  }

  /// Repacks the save slot into a compressed PMOC container.
  ///
  /// # Errors
  ///
  /// Fails on compression error.
  pub fn to_bytes(&self) -> Result<Vec<u8>, error::Error> {
    let mut payload = self.payload.clone();
    for u in &self.roster {
      let base = self.index.unit_offset + u.offset();
      if base + 32 <= payload.len() {
        u.write_back(&mut payload[base..]);
      }
    }
    for item in &self.convoy {
      let off = self.index.convoy_offset + 8 + item.slot * 4;
      if off + 4 <= payload.len() {
        payload[off..off + 2]
          .copy_from_slice(&item.item.raw_id().to_be_bytes());
        payload[off + 2..off + 4].copy_from_slice(&item.uses.to_be_bytes());
      }
    }
    Ok(fe_compress::repack(&self.preview, &payload)?)
  }

  /// Returns a borrowed view over the preview header.
  #[must_use]
  pub fn preview(&self) -> preview::PreviewRef<'_> {
    preview::PreviewRef::new(&self.preview)
  }

  /// Returns a mutable view over the preview header.
  pub fn preview_mut(&mut self) -> preview::PreviewMut<'_> {
    preview::PreviewMut::new(&mut self.preview)
  }

  /// Returns a borrowed view over the RESU campaign user block.
  #[must_use]
  pub fn user(&self) -> user::UserRef<'_> {
    user::UserRef::new(&self.payload[self.index.user_offset..])
  }

  /// Returns a mutable view over the RESU campaign user block.
  pub fn user_mut(&mut self) -> user::UserMut<'_> {
    user::UserMut::new(&mut self.payload[self.index.user_offset..])
  }

  /// Finds a unit in the roster by character name (case-insensitive).
  pub fn find_unit_by_name_mut(
    &mut self,
    name: &str,
  ) -> Option<&mut roster::Unit> {
    self.roster.find_by_name_mut(name)
  }

  /// Finds a unit in the roster by character name (case-insensitive).
  #[must_use]
  pub fn find_unit_by_name(&self, name: &str) -> Option<&roster::Unit> {
    self.roster.find_by_name(name)
  }

  /// Returns campaign gold.
  #[must_use]
  pub fn gold(&self) -> u32 {
    self.user().gold()
  }

  /// Sets campaign gold amount.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::GoldOutOfBounds`] if `amount > MAX_GOLD`.
  pub fn set_gold(&mut self, amount: u32) -> Result<(), error::Error> {
    if amount > user::MAX_GOLD {
      return Err(error::Error::GoldOutOfBounds {
        value: amount,
        max: user::MAX_GOLD,
      });
    }
    self.user_mut().set_gold(amount);
    Ok(())
  }

  /// Returns total play time duration.
  #[must_use]
  pub fn play_time(&self) -> time::PlayTime {
    self.preview().play_time()
  }

  /// Updates total play time across preview header and campaign user block.
  pub fn set_play_time(&mut self, time: time::PlayTime) {
    self.preview_mut().set_play_time(time);
    let secs = u32::try_from(time.as_secs()).unwrap_or(u32::MAX);
    self.user_mut().set_play_time(secs);
  }
}
