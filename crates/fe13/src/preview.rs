//! Slot preview header borrowed view and mutators.
//!
//! Owns accessors for the 192-byte uncompressed save slot preview header.

use crate::text;
use crate::types;

/// Expected byte length of the uncompressed slot preview header.
pub const PREVIEW_LEN: usize = 0xC0;

/// Borrowed zero-allocation view over a 192-byte slot preview header.
#[derive(Clone, Copy, Debug)]
pub struct PreviewRef<'a> {
  /// Raw 192-byte preview header slice.
  raw: &'a [u8; PREVIEW_LEN],
}

impl<'a> PreviewRef<'a> {
  /// Wraps a 192-byte preview header slice.
  #[must_use]
  pub const fn new(raw: &'a [u8; PREVIEW_LEN]) -> Self {
    Self { raw }
  }

  /// Slot version marker.
  #[must_use]
  pub fn version(self) -> u16 {
    u16::from_le_bytes([self.raw[0], self.raw[1]])
  }

  /// Save slot counter.
  #[must_use]
  pub fn slot_counter(self) -> u16 {
    u16::from_le_bytes([self.raw[2], self.raw[3]])
  }

  /// Game difficulty setting.
  #[must_use]
  pub fn difficulty(self) -> Option<types::Difficulty> {
    types::Difficulty::from_raw(self.raw[8])
  }

  /// Permanent death mode.
  #[must_use]
  pub fn mode(self) -> Option<types::GameMode> {
    types::GameMode::from_raw(self.raw[9])
  }

  /// Current chapter index.
  #[must_use]
  pub fn chapter_id(self) -> u16 {
    u16::from_le_bytes([self.raw[12], self.raw[13]])
  }

  /// Avatar character name.
  #[must_use]
  pub fn avatar_name(self) -> String {
    text::read_utf16le(&self.raw[0x10..0x2A])
  }

  /// Display string for the current chapter.
  #[must_use]
  pub fn chapter_title(self) -> String {
    let title = text::read_utf16le(&self.raw[0x2A..0x4A]);
    if title.is_empty() {
      text::read_utf16le(&self.raw[0x4A..0x6A])
    } else {
      title
    }
  }

  /// Displayed gold amount on save slot card.
  #[must_use]
  pub fn gold(self) -> u32 {
    u32::from_le_bytes([
      self.raw[0x50],
      self.raw[0x51],
      self.raw[0x52],
      self.raw[0x53],
    ])
  }

  /// Total play time in seconds.
  #[must_use]
  pub fn play_time_secs(self) -> u32 {
    u32::from_le_bytes([
      self.raw[0x54],
      self.raw[0x55],
      self.raw[0x56],
      self.raw[0x57],
    ])
  }
}

/// Mutable view over a 192-byte slot preview header.
#[derive(Debug)]
pub struct PreviewMut<'a> {
  /// Mutable 192-byte preview header slice.
  raw: &'a mut [u8; PREVIEW_LEN],
}

impl<'a> PreviewMut<'a> {
  /// Wraps a mutable 192-byte preview header slice.
  pub fn new(raw: &'a mut [u8; PREVIEW_LEN]) -> Self {
    Self { raw }
  }

  /// Sets campaign gold displayed on the save slot card (clamped to 999,999).
  pub fn set_gold(&mut self, gold: u32) {
    let clamped = gold.min(999_999);
    self.raw[0x50..0x54].copy_from_slice(&clamped.to_le_bytes());
  }

  /// Sets avatar character name.
  pub fn set_avatar_name(&mut self, name: &str) {
    text::write_utf16le(&mut self.raw[0x10..0x2A], name);
  }

  /// Sets game difficulty.
  pub fn set_difficulty(&mut self, difficulty: types::Difficulty) {
    self.raw[8] = difficulty as u8;
  }

  /// Sets permanent death mode.
  pub fn set_mode(&mut self, mode: types::GameMode) {
    self.raw[9] = mode as u8;
  }
}
