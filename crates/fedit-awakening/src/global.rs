//! Global game-wide system save file domain representation.
//!
//! Owns the [`GlobalSave`] aggregate containing shared system records,
//! achievement clear flags, Lunatic+ unlocks, and Logbook profiles.

use fedit_compress;
use thiserror;

/// Errors that can occur when parsing or modifying the global save.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Renown points value exceeded maximum allowable amount (999,999).
  #[error("renown amount {value} exceeds maximum {max}")]
  RenownOutOfBounds {
    /// Provided value.
    value: u32,
    /// Maximum allowed.
    max: u32,
  },
  /// Block offset was out of bounds for the payload length.
  #[error("offset {offset} is out of bounds for payload length {len}")]
  OffsetOutOfBounds {
    /// Resolved offset.
    offset: usize,
    /// Payload length.
    len: usize,
  },
  /// Container decompression or packing error.
  #[error(transparent)]
  Compress(#[from] fedit_compress::Error),
}

/// Owned representation of the game-wide `Global` save file.
#[derive(Debug)]
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
  pub fn from_bytes(raw: &[u8]) -> Result<Self, Error> {
    let container = fedit_compress::unpack(raw)?;
    Ok(Self {
      payload: container.payload,
    })
  }

  /// Repacks the global save into a compressed container.
  ///
  /// # Errors
  ///
  /// Fails on compression error.
  pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
    Ok(fedit_compress::repack(&[], &self.payload)?)
  }

  /// Returns the decompressed payload length in bytes.
  #[must_use]
  pub fn payload_len(&self) -> usize {
    self.payload.len()
  }

  /// Returns the offset of the RESU block in the payload.
  fn resu_offset(&self) -> Option<usize> {
    self.payload.windows(4).position(|w| w == b"RESU")
  }

  /// Returns the offset of the DG18 Logbook block in the payload.
  fn dg18_offset(&self) -> Option<usize> {
    self.payload.windows(4).position(|w| w == b"DG18")
  }

  /// Returns the global Renown points from the RESU block.
  #[must_use]
  pub fn renown(&self) -> u32 {
    let Some(resu_pos) = self.resu_offset() else {
      return 0;
    };
    let off = resu_pos + 11;
    if off + 4 <= self.payload.len() {
      u32::from_le_bytes([
        self.payload[off],
        self.payload[off + 1],
        self.payload[off + 2],
        self.payload[off + 3],
      ])
    } else {
      0
    }
  }

  /// Sets global Renown points with bounds checking `[0..=999_999]`.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::RenownOutOfBounds`] if `renown > 999_999`.
  /// - Returns [`Error::OffsetOutOfBounds`] if RESU block is missing.
  pub fn set_renown(&mut self, renown: u32) -> Result<(), Error> {
    if renown > 999_999 {
      return Err(Error::RenownOutOfBounds {
        value: renown,
        max: 999_999,
      });
    }
    let Some(resu_pos) = self.resu_offset() else {
      return Err(Error::OffsetOutOfBounds {
        offset: 0,
        len: self.payload.len(),
      });
    };
    let off = resu_pos + 11;
    if off + 4 > self.payload.len() {
      return Err(Error::OffsetOutOfBounds {
        offset: off + 4,
        len: self.payload.len(),
      });
    }
    self.payload[off..off + 4].copy_from_slice(&renown.to_le_bytes());
    Ok(())
  }

  /// Returns whether the game clear completion flag (Theater, Gallery) is set.
  #[must_use]
  pub fn game_cleared(&self) -> bool {
    let Some(resu_pos) = self.resu_offset() else {
      return false;
    };
    if resu_pos + 5 < self.payload.len() {
      (self.payload[resu_pos + 5] & 0x02) != 0
    } else {
      false
    }
  }

  /// Sets whether the game clear completion flag is unlocked.
  ///
  /// # Errors
  ///
  /// Returns [`Error::OffsetOutOfBounds`] if the RESU block is missing.
  pub fn set_game_cleared(&mut self, cleared: bool) -> Result<(), Error> {
    let Some(resu_pos) = self.resu_offset() else {
      return Err(Error::OffsetOutOfBounds {
        offset: 0,
        len: self.payload.len(),
      });
    };
    if resu_pos + 5 >= self.payload.len() {
      return Err(Error::OffsetOutOfBounds {
        offset: resu_pos + 5,
        len: self.payload.len(),
      });
    }
    if cleared {
      self.payload[resu_pos + 5] |= 0x02;
    } else {
      self.payload[resu_pos + 5] &= !0x02;
    }
    Ok(())
  }

  /// Returns whether the Lunatic+ difficulty tier is unlocked.
  #[must_use]
  pub fn lunatic_plus_unlocked(&self) -> bool {
    let Some(resu_pos) = self.resu_offset() else {
      return false;
    };
    if resu_pos + 5 < self.payload.len() {
      (self.payload[resu_pos + 5] & 0x0C) != 0
    } else {
      false
    }
  }

  /// Sets whether the Lunatic+ difficulty tier is unlocked.
  ///
  /// # Errors
  ///
  /// Returns [`Error::OffsetOutOfBounds`] if the RESU block is missing.
  pub fn set_lunatic_plus_unlocked(
    &mut self,
    unlocked: bool,
  ) -> Result<(), Error> {
    let Some(resu_pos) = self.resu_offset() else {
      return Err(Error::OffsetOutOfBounds {
        offset: 0,
        len: self.payload.len(),
      });
    };
    if resu_pos + 5 >= self.payload.len() {
      return Err(Error::OffsetOutOfBounds {
        offset: resu_pos + 5,
        len: self.payload.len(),
      });
    }
    if unlocked {
      self.payload[resu_pos + 5] |= 0x0C;
    } else {
      self.payload[resu_pos + 5] &= !0x0C;
    }
    Ok(())
  }

  /// Returns whether Japanese voice acting is enabled in system settings.
  #[must_use]
  pub fn jp_voice_enabled(&self) -> bool {
    let Some(resu_pos) = self.resu_offset() else {
      return false;
    };
    if resu_pos + 6 < self.payload.len() {
      (self.payload[resu_pos + 6] & 0x08) != 0
    } else {
      false
    }
  }

  /// Sets whether Japanese voice acting is enabled in system settings.
  ///
  /// # Errors
  ///
  /// Returns [`Error::OffsetOutOfBounds`] if the RESU block is missing.
  pub fn set_jp_voice_enabled(&mut self, enabled: bool) -> Result<(), Error> {
    let Some(resu_pos) = self.resu_offset() else {
      return Err(Error::OffsetOutOfBounds {
        offset: 0,
        len: self.payload.len(),
      });
    };
    if resu_pos + 6 >= self.payload.len() {
      return Err(Error::OffsetOutOfBounds {
        offset: resu_pos + 6,
        len: self.payload.len(),
      });
    }
    if enabled {
      self.payload[resu_pos + 6] |= 0x08;
    } else {
      self.payload[resu_pos + 6] &= !0x08;
    }
    Ok(())
  }

  /// Returns the index of the most recently accessed save slot.
  #[must_use]
  pub fn last_save_slot(&self) -> u8 {
    let Some(resu_pos) = self.resu_offset() else {
      return 0;
    };
    if resu_pos + 9 < self.payload.len() {
      self.payload[resu_pos + 9]
    } else {
      0
    }
  }

  /// Sets the index of the most recently accessed save slot.
  ///
  /// # Errors
  ///
  /// Returns [`Error::OffsetOutOfBounds`] if the RESU block is missing.
  pub fn set_last_save_slot(&mut self, slot: u8) -> Result<(), Error> {
    let Some(resu_pos) = self.resu_offset() else {
      return Err(Error::OffsetOutOfBounds {
        offset: 0,
        len: self.payload.len(),
      });
    };
    if resu_pos + 9 >= self.payload.len() {
      return Err(Error::OffsetOutOfBounds {
        offset: resu_pos + 9,
        len: self.payload.len(),
      });
    }
    self.payload[resu_pos + 9] = slot;
    Ok(())
  }

  /// Returns the first 8 global bitflags matching the reference schema.
  #[must_use]
  pub fn global_flags(&self) -> [bool; 8] {
    let Some(resu_pos) = self.resu_offset() else {
      return [false; 8];
    };
    if resu_pos + 5 < self.payload.len() {
      let b = self.payload[resu_pos + 5];
      [
        (b & 0x01) != 0,
        (b & 0x02) != 0,
        (b & 0x04) != 0,
        (b & 0x08) != 0,
        (b & 0x10) != 0,
        (b & 0x20) != 0,
        (b & 0x40) != 0,
        (b & 0x80) != 0,
      ]
    } else {
      [false; 8]
    }
  }

  /// Returns the total count of registered Logbook avatar profiles.
  #[must_use]
  pub fn logbook_count(&self) -> usize {
    let Some(dg_pos) = self.dg18_offset() else {
      return 0;
    };
    if dg_pos + 0x0B <= self.payload.len() {
      self.payload[dg_pos + 0x0A] as usize
    } else {
      0
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_global_renown_mutations() {
    let mut payload = vec![0u8; 0x100];
    payload[0x44..0x48].copy_from_slice(b"RESU");
    let mut global = GlobalSave { payload };

    assert_eq!(global.renown(), 0);
    assert!(global.set_renown(50_000).is_ok());
    assert_eq!(global.renown(), 50_000);

    // Exceeds 999,999
    assert!(global.set_renown(1_000_000).is_err());
  }

  #[test]
  fn test_global_clear_and_lunatic_flags() {
    let mut payload = vec![0u8; 0x100];
    payload[0x44..0x48].copy_from_slice(b"RESU");
    let mut global = GlobalSave { payload };

    assert!(!global.game_cleared());
    assert!(global.set_game_cleared(true).is_ok());
    assert!(global.game_cleared());

    assert!(!global.lunatic_plus_unlocked());
    assert!(global.set_lunatic_plus_unlocked(true).is_ok());
    assert!(global.lunatic_plus_unlocked());

    assert!(!global.jp_voice_enabled());
    assert!(global.set_jp_voice_enabled(true).is_ok());
    assert!(global.jp_voice_enabled());

    assert_eq!(global.last_save_slot(), 0);
    assert!(global.set_last_save_slot(3).is_ok());
    assert_eq!(global.last_save_slot(), 3);
  }
}
