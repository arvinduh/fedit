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
    Ok(Self {
      preview,
      payload: container.payload,
      index,
    })
  }

  /// Repacks the save slot into a compressed PMOC container.
  ///
  /// # Errors
  ///
  /// Fails on compression error.
  pub fn to_bytes(&self) -> Result<Vec<u8>, error::Error> {
    Ok(fe_compress::repack(&self.preview, &self.payload)?)
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

  /// Synchronously updates campaign gold across both preview and RESU blocks.
  pub fn set_gold(&mut self, amount: u32) {
    self.preview_mut().set_gold(amount);
    self.user_mut().set_gold(amount);
  }
}
