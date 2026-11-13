//! Save archive directory discovery and management.
//!
//! Owns aggregate views over an entire 3DS save directory.

use std::fs;
use std::path;

use fe13;

use crate::atomic;
use crate::backup;
use crate::error;

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

impl SaveArchive {
  /// Opens and discovers save slots in a directory.
  ///
  /// # Errors
  ///
  /// Returns an error if reading files fails or no saves are discovered.
  pub fn open(dir: impl AsRef<path::Path>) -> Result<Self, error::Error> {
    let root = dir.as_ref().to_path_buf();
    let read_slot =
      |name: &str| -> Result<Option<fe13::SaveSlot>, error::Error> {
        let p = root.join(name);
        if p.is_file() {
          let b = fs::read(&p)?;
          Ok(Some(fe13::SaveSlot::from_bytes(&b)?))
        } else {
          Ok(None)
        }
      };
    let chapters = [
      read_slot("Chapter0")?,
      read_slot("Chapter1")?,
      read_slot("Chapter2")?,
    ];
    let maps = [read_slot("Map0")?, read_slot("Map1")?, read_slot("Map2")?];
    let gpath = root.join("Global");
    let global = if gpath.is_file() {
      Some(fs::read(&gpath)?)
    } else {
      None
    };
    if global.is_none()
      && chapters.iter().all(Option::is_none)
      && maps.iter().all(Option::is_none)
    {
      return Err(error::Error::EmptyArchive { path: root });
    }
    Ok(Self {
      root,
      global,
      chapters,
      maps,
    })
  }

  /// Persists modified slots to disk, creating an automated backup snapshot first.
  ///
  /// # Errors
  ///
  /// Returns an error if backup creation or file serialization fails.
  pub fn save(&self) -> Result<Option<path::PathBuf>, error::Error> {
    let backup_dir = backup::BackupManager::create_backup(&self.root)?;
    for (i, slot) in self.chapters.iter().enumerate() {
      if let Some(slot) = slot {
        let repacked = slot.to_bytes()?;
        atomic::write_atomic(self.root.join(format!("Chapter{i}")), &repacked)?;
      }
    }
    for (i, slot) in self.maps.iter().enumerate() {
      if let Some(slot) = slot {
        let repacked = slot.to_bytes()?;
        atomic::write_atomic(self.root.join(format!("Map{i}")), &repacked)?;
      }
    }
    Ok(backup_dir)
  }
}
