//! `SaveSlot` container aggregate and codecs.
//!
//! Owns the uncompressed save slot aggregate, parsing and repacking archives.

use fe_compress;

use crate::error;
use crate::index;
use crate::preview;

/// An uncompressed, editable Fire Emblem Awakening save slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SaveSlot {
  /// Raw 192-byte preview header buffer.
  pub(crate) preview: [u8; preview::PREVIEW_LEN],
  /// Decompressed EDNI save payload buffer.
  pub(crate) payload: Vec<u8>,
  /// Resolved subsystem block index table.
  pub(crate) index: index::EdniIndex,
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
}
