//! `SaveSlot` container aggregate and codecs.
//!
//! Owns the uncompressed save slot aggregate, parsing and repacking archives.

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
