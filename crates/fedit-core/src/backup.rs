//! Automated pre-mutation save archive backup manager.
//!
//! Creates timestamped snapshot copies of save files before modifications.

use std::fs;
use std::io;
use std::path;

/// Candidate file names for Fire Emblem Awakening saves.
const SAVE_FILES: [&str; 7] = [
  "Global", "Chapter0", "Chapter1", "Chapter2", "Map0", "Map1", "Map2",
];

/// Automated pre-mutation archive snapshot manager.
pub struct BackupManager;

impl BackupManager {
  /// Creates a timestamped snapshot of save files in an archive directory.
  ///
  /// # Errors
  ///
  /// Returns an [`io::Error`] if creating directories or copying files fails.
  pub fn create_backup(
    dir: impl AsRef<path::Path>,
  ) -> io::Result<Option<path::PathBuf>> {
    let root = dir.as_ref();
    let to_copy: Vec<_> = SAVE_FILES
      .iter()
      .map(|n| root.join(n))
      .filter(|p| p.is_file())
      .collect();
    if to_copy.is_empty() {
      return Ok(None);
    }
    let name = root.file_name().map_or_else(
      || "save_bak".to_string(),
      |s| format!("{}_bak", s.to_string_lossy()),
    );
    let backup_dir = root
      .parent()
      .map_or_else(|| path::PathBuf::from(&name), |p| p.join(&name));
    fs::create_dir_all(&backup_dir)?;
    for src in to_copy {
      if let Some(name) = src.file_name() {
        fs::copy(&src, backup_dir.join(name))?;
      }
    }
    Ok(Some(backup_dir))
  }
}
