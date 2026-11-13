//! Slot preview header borrowed view and mutators.
//!
//! Owns accessors for the 192-byte uncompressed save slot preview header.

use crate::text;
use crate::time;
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
    types::Difficulty::from_raw((self.raw[8] >> 2) & 0x03)
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
  /// Mutable 192-byte preview header slice.
  raw: &'a mut [u8; PREVIEW_LEN],
}

impl<'a> PreviewMut<'a> {
  /// Wraps a mutable 192-byte preview header slice.
  pub fn new(raw: &'a mut [u8; PREVIEW_LEN]) -> Self {
    Self { raw }
  }

  /// Sets avatar character name.
  pub fn set_avatar_name(&mut self, name: &str) {
    text::write_utf16le(&mut self.raw[0x10..0x2A], name);
  }

  /// Sets game difficulty.
  pub fn set_difficulty(&mut self, difficulty: types::Difficulty) {
    let bits = (difficulty as u8) << 2;
    self.raw[8] = (self.raw[8] & !0x0C) | bits;
  }

  /// Sets permanent death mode.
  pub fn set_mode(&mut self, mode: types::GameMode) {
    self.raw[9] = mode as u8;
  }

  /// Sets total play time duration.
  pub fn set_play_time(&mut self, time: time::PlayTime) {
    let raw = u32::try_from(time.as_secs())
      .unwrap_or(u32::MAX)
      .to_le_bytes();
    self.raw[1..5].copy_from_slice(&raw);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const CHAPTER0: &[u8] =
    include_bytes!("../../../tests/fixtures/20200831-225628/Chapter0");

  #[test]
  fn test_preview_header_parsing() {
    let raw: &[u8; PREVIEW_LEN] =
      CHAPTER0[..PREVIEW_LEN].try_into().expect("192 bytes");
    let preview = PreviewRef::new(raw);
    assert_eq!(preview.avatar_name(), "Robin", "avatar name");
    assert_eq!(
      preview.chapter_title(),
      "Chapter 19: The Conqueror",
      "chapter title"
    );
    assert_eq!(preview.difficulty(), Some(types::Difficulty::Hard), "diff");
    assert_eq!(preview.mode(), Some(types::GameMode::Classic), "mode");
    assert_eq!(preview.play_time().to_string(), "203:03:15");
  }

  #[test]
  fn test_preview_mutators() {
    let mut buf = [0u8; PREVIEW_LEN];
    let mut mutator = PreviewMut::new(&mut buf);
    mutator.set_avatar_name("Chrom");
    mutator.set_difficulty(types::Difficulty::Lunatic);
    mutator.set_mode(types::GameMode::Casual);
    mutator.set_play_time(time::PlayTime::from_secs(3600));
    let view = PreviewRef::new(&buf);
    assert_eq!(view.avatar_name(), "Chrom", "mutated name");
    assert_eq!(
      view.difficulty(),
      Some(types::Difficulty::Lunatic),
      "mutated diff"
    );
    assert_eq!(view.mode(), Some(types::GameMode::Casual), "mutated mode");
    assert_eq!(view.play_time().to_string(), "01:00:00");
  }
}
