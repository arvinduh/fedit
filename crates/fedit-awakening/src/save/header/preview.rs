//! Slot preview header borrowed view and mutators.
//!
//! Owns accessors for the 192-byte uncompressed save slot preview header.

use crate::save::header::config;
use crate::save::utils::text;
use crate::save::utils::time;

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
  pub const fn from_bytes(raw: &'a [u8; PREVIEW_LEN]) -> Self {
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
  pub fn difficulty(self) -> Option<config::Difficulty> {
    config::Difficulty::from_raw((self.raw[8] >> 2) & 0x03)
  }

  /// Permanent death mode.
  #[must_use]
  pub fn mode(self) -> Option<config::GameMode> {
    config::GameMode::from_raw(self.raw[9])
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

  /// Chapter identifier string (e.g. "Chapter 19").
  #[must_use]
  pub fn chapter_number(self) -> String {
    text::read_utf16le(&self.raw[0x2A..0x4A])
  }

  /// Chapter subtitle string (e.g. "The Conqueror").
  #[must_use]
  pub fn chapter_subtitle(self) -> String {
    text::read_utf16le(&self.raw[0x4A..0x6A])
  }

  /// Full formatted chapter title string.
  #[must_use]
  pub fn chapter_title(self) -> String {
    let num = self.chapter_number();
    let sub = self.chapter_subtitle();
    if num.is_empty() {
      sub
    } else if sub.is_empty() {
      num
    } else {
      format!("{num}: {sub}")
    }
  }

  /// Total play time duration.
  #[must_use]
  pub fn play_time(self) -> time::PlayTime {
    let secs = u64::from(u32::from_le_bytes([
      self.raw[1],
      self.raw[2],
      self.raw[3],
      self.raw[4],
    ]));
    time::PlayTime::from_secs(secs)
  }
}

/// Mutable view over a 192-byte slot preview header.
#[derive(Debug)]
pub struct PreviewMut<'a> {
  /// Mutable reference to the 192-byte preview header buffer.
  raw: &'a mut [u8; PREVIEW_LEN],
}

impl<'a> PreviewMut<'a> {
  /// Wraps a mutable reference to a 192-byte preview header.
  pub fn from_bytes(raw: &'a mut [u8; PREVIEW_LEN]) -> Self {
    Self { raw }
  }

  /// Updates game difficulty setting.
  pub fn set_difficulty(&mut self, difficulty: config::Difficulty) {
    self.raw[8] = (self.raw[8] & !0x0C) | ((difficulty as u8) << 2);
  }

  /// Updates permanent death mode.
  pub fn set_mode(&mut self, mode: config::GameMode) {
    self.raw[9] = mode as u8;
  }

  /// Updates current chapter ID.
  pub fn set_chapter_id(&mut self, chapter: u16) {
    let bytes = chapter.to_le_bytes();
    self.raw[12] = bytes[0];
    self.raw[13] = bytes[1];
  }

  /// Updates avatar character name.
  pub fn set_avatar_name(&mut self, name: &str) {
    text::write_utf16le(&mut self.raw[0x10..0x2A], name);
  }

  /// Updates chapter identifier string.
  pub fn set_chapter_number(&mut self, number: &str) {
    text::write_utf16le(&mut self.raw[0x2A..0x4A], number);
  }

  /// Updates chapter subtitle string.
  pub fn set_chapter_subtitle(&mut self, subtitle: &str) {
    text::write_utf16le(&mut self.raw[0x4A..0x6A], subtitle);
  }

  /// Updates total play time duration.
  pub fn set_play_time(&mut self, play_time: time::PlayTime) {
    let secs = u32::try_from(play_time.as_secs()).unwrap_or(u32::MAX);
    let bytes = secs.to_le_bytes();
    self.raw[1] = bytes[0];
    self.raw[2] = bytes[1];
    self.raw[3] = bytes[2];
    self.raw[4] = bytes[3];
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_preview_header_parsing() {
    let mut raw = [0u8; PREVIEW_LEN];
    raw[8] = 0x04; // Hard difficulty
    raw[9] = 0x00; // Classic mode
    raw[12] = 0x13; // Chapter 19
    text::write_utf16le(&mut raw[0x10..0x2A], "Robin");
    text::write_utf16le(&mut raw[0x2A..0x4A], "Chapter 19");
    text::write_utf16le(&mut raw[0x4A..0x6A], "The Conqueror");

    let p = PreviewRef::from_bytes(&raw);
    assert_eq!(p.difficulty(), Some(config::Difficulty::Hard));
    assert_eq!(p.mode(), Some(config::GameMode::Classic));
    assert_eq!(p.chapter_id(), 19);
    assert_eq!(p.avatar_name(), "Robin");
    assert_eq!(p.chapter_title(), "Chapter 19: The Conqueror");
  }

  #[test]
  fn test_preview_mutators() {
    let mut raw = [0u8; PREVIEW_LEN];
    let mut pm = PreviewMut::from_bytes(&mut raw);
    pm.set_difficulty(config::Difficulty::Lunatic);
    pm.set_mode(config::GameMode::Casual);
    pm.set_avatar_name("Chrom");

    let p = PreviewRef::from_bytes(&raw);
    assert_eq!(p.difficulty(), Some(config::Difficulty::Lunatic));
    assert_eq!(p.mode(), Some(config::GameMode::Casual));
    assert_eq!(p.avatar_name(), "Chrom");
  }
}
