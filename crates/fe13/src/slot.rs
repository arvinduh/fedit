//! `SaveSlot` container aggregate and codecs.
//!
//! Owns the uncompressed save slot aggregate, parsing and repacking archives.

use fe_compress;

use crate::convoy;
use crate::error;
use crate::index;
use crate::preview;
use crate::unit;
use crate::user;

/// An uncompressed, editable Fire Emblem Awakening save slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SaveSlot {
  /// Raw 192-byte preview header buffer.
  preview: [u8; preview::PREVIEW_LEN],
  /// Decompressed EDNI save payload buffer.
  payload: Vec<u8>,
  /// Resolved subsystem block index table.
  index: index::EdniIndex,
  /// Owned unit roster.
  pub roster: Vec<unit::Unit>,
  /// Owned convoy storage items.
  pub convoy: Vec<convoy::ConvoyItem>,
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
    let roster =
      unit::UnitIter::collect_units(&container.payload[index.unit_offset..]);
    let convoy =
      convoy::ConvoyRef::new(&container.payload[index.convoy_offset..]).items();
    Ok(Self {
      preview,
      payload: container.payload,
      index,
      roster,
      convoy,
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
      let base = self.index.unit_offset + u.offset;
      if base + 32 <= payload.len() {
        u.write_back(&mut payload[base..]);
      }
    }
    for item in &self.convoy {
      let off = self.index.convoy_offset + 8 + item.slot * 4;
      if off + 4 <= payload.len() {
        payload[off..off + 2].copy_from_slice(&item.item_id.0.to_be_bytes());
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

  /// Returns an iterator over unit character slots in the TINU roster.
  #[must_use]
  pub fn units(&self) -> unit::UnitIter<'_> {
    unit::UnitIter::new(&self.payload[self.index.unit_offset..])
  }

  /// Returns a borrowed view over the NART convoy storage.
  #[must_use]
  pub fn convoy(&self) -> convoy::ConvoyRef<'_> {
    convoy::ConvoyRef::new(&self.payload[self.index.convoy_offset..])
  }

  /// Returns a mutable view over the NART convoy storage.
  pub fn convoy_mut(&mut self) -> convoy::ConvoyMut<'_> {
    convoy::ConvoyMut::new(&mut self.payload[self.index.convoy_offset..])
  }

  /// Returns campaign gold.
  #[must_use]
  pub fn gold(&self) -> u32 {
    self.user().gold()
  }

  /// Updates campaign gold clamped to maximum allowable gold.
  pub fn set_gold(&mut self, amount: u32) {
    self.user_mut().set_gold(amount);
  }
}
