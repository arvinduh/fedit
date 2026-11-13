//! Save archive directory discovery and management.
//!
//! Owns aggregate views over an entire 3DS save directory.

use std::path;

use fe13;

/// Save archive containing multi-slot campaign and battle states.
pub struct SaveArchive {
  /// Path to the archive root directory.
  pub root: path::PathBuf,
  /// Global system file payload if present.
  pub global: Option<Vec<u8>>,
  /// Discovered campaign save slots (Chapter0..=2).
  pub chapters: [Option<fe13::SaveSlot>; 3],
  /// Discovered battle save slots (Map0..=2).
  pub maps: [Option<fe13::SaveSlot>; 3],
}
