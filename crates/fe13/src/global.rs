//! Global game-wide system save file domain representation.
//!
//! Owns the [`GlobalSave`] aggregate containing shared system records.

use fe_compress;

use crate::error;

/// Owned representation of the game-wide `Global` save file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalSave {
  /// Decompressed payload buffer.
  payload: Vec<u8>,
}

impl GlobalSave {
  /// Parses a global save file from raw compressed container bytes.
  ///
  /// # Errors
  ///
  /// Fails if container is invalid or decompression fails.
  pub fn from_bytes(raw: &[u8]) -> Result<Self, error::Error> {
    let container = fe_compress::unpack(raw)?;
    Ok(Self {
      payload: container.payload,
    })
  }

  /// Repacks the global save into a compressed container.
  ///
  /// # Errors
  ///
  /// Fails on compression error.
  pub fn to_bytes(&self) -> Result<Vec<u8>, error::Error> {
    Ok(fe_compress::repack(&[], &self.payload)?)
  }

  /// Returns the decompressed payload length in bytes.
  #[must_use]
  pub fn payload_len(&self) -> usize {
    self.payload.len()
  }

  /// Returns a borrowed slice of the decompressed payload.
  #[must_use]
  pub fn payload(&self) -> &[u8] {
    &self.payload
  }
}
