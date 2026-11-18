//! 3DS save set directory discovery and lifecycle management.
//!
//! Scans a folder containing Nintendo 3DS Fire Emblem save files (`Global`,
//! `Chapter0..2`, `Map0..1`), reads preview metadata for immediate slot
//! switching, and provides verified loading and saving with automated backups.

use std::fs;
use std::path;

use awakening::PlayTime;
use awakening::header;

use crate::backup;
use crate::error;

/// Canonical classification of a save slot file within a save set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SlotKind {
  /// Global archive containing system settings, renown, and extras.
  Global,
  /// Story campaign chapter save slot with 0-based index (e.g. `Chapter0`).
  Chapter(u8),
  /// In-progress battle or bookmark save slot with 0-based index (e.g. `Map0`).
  Map(u8),
  /// Other or unclassified save file.
  Other,
}

impl SlotKind {
  /// Resolves the slot kind from a filename string.
  #[must_use]
  pub fn from_filename(name: &str) -> Self {
    if name.eq_ignore_ascii_case("global") {
      Self::Global
    } else if let Some(suffix) = name.strip_prefix("Chapter") {
      suffix.parse::<u8>().map_or(Self::Other, Self::Chapter)
    } else if let Some(suffix) = name.strip_prefix("Map") {
      suffix.parse::<u8>().map_or(Self::Other, Self::Map)
    } else {
      Self::Other
    }
  }

  /// Returns a sort priority key for ordering slots in UI switcher bars.
  #[must_use]
  pub const fn sort_order(&self) -> u16 {
    match *self {
      Self::Global => 0,
      Self::Chapter(idx) => 100 + idx as u16,
      Self::Map(idx) => 200 + idx as u16,
      Self::Other => 999,
    }
  }
}

/// Metadata summary of an individual save slot discovered on disk.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SlotSummary {
  /// Structural category of the save slot.
  pub kind: SlotKind,
  /// Filename within the directory (e.g. "Chapter0").
  pub filename: String,
  /// Absolute or relative path to the file on disk.
  pub path: path::PathBuf,
  /// Human-readable title suitable for buttons and tabs.
  pub display_title: String,
  /// Avatar name parsed from preview header if available.
  pub avatar_name: Option<String>,
  /// Formatted chapter name (e.g. "Chapter 19: The Conqueror").
  pub chapter_title: Option<String>,
  /// Game difficulty setting if available.
  pub difficulty: Option<header::Difficulty>,
  /// Permadeath game mode (Classic vs Casual).
  pub mode: Option<header::GameMode>,
  /// Recorded campaign playtime duration.
  pub play_time: Option<PlayTime>,
  /// Raw 192-byte preview header buffer if parsed.
  pub raw_preview: Option<[u8; header::PREVIEW_LEN]>,
}

/// Multi-slot save directory container.
#[derive(Clone, Debug)]
pub struct SaveSet {
  /// Base directory containing the save files.
  dir: path::PathBuf,
  /// Discovered save slots sorted in canonical order.
  slots: Vec<SlotSummary>,
}

impl SaveSet {
  /// Opens a directory or specific save slot file path.
  ///
  /// If `path` is a file (e.g. `.../Chapter1`), scans its parent directory and
  /// returns the filename for immediate selection. If `path` is a directory,
  /// scans all slots within it.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::PathNotFound`] if the path does not exist,
  /// [`error::Error::NoSavesFound`] if no valid saves exist in the folder, or
  /// [`error::Error::Io`] on read failure.
  pub fn open_path(
    path: impl AsRef<path::Path>,
  ) -> Result<(Self, Option<String>), error::Error> {
    let p = path.as_ref().to_path_buf();
    if !p.exists() {
      return Err(error::Error::PathNotFound { path: p });
    }

    if p.is_file() {
      let filename = p
        .file_name()
        .and_then(|n| n.to_str())
        .map(ToString::to_string);
      let parent = p.parent().unwrap_or(&p);
      let set = Self::open_dir(parent)?;
      Ok((set, filename))
    } else {
      let set = Self::open_dir(&p)?;
      Ok((set, None))
    }
  }

  /// Scans a directory and discovers all available Fire Emblem save files.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::PathNotFound`] if `dir` does not exist,
  /// [`error::Error::NotADirectory`] if `dir` is not a directory,
  /// [`error::Error::NoSavesFound`] if the directory contains no save files,
  /// or [`error::Error::Io`] on filesystem read failure.
  pub fn open_dir(dir: impl AsRef<path::Path>) -> Result<Self, error::Error> {
    let dir = dir.as_ref().to_path_buf();
    if !dir.exists() {
      return Err(error::Error::PathNotFound { path: dir });
    }
    if !dir.is_dir() {
      return Err(error::Error::NotADirectory { path: dir });
    }

    let entries = fs::read_dir(&dir).map_err(|source| error::Error::Io {
      path: dir.clone(),
      source,
    })?;

    let mut slots = Vec::new();

    for entry in entries {
      let entry = entry.map_err(|source| error::Error::Io {
        path: dir.clone(),
        source,
      })?;
      let path = entry.path();
      if !path.is_file() {
        continue;
      }

      let filename = match path.file_name().and_then(|n| n.to_str()) {
        Some(name) => name.to_string(),
        None => continue,
      };

      let kind = SlotKind::from_filename(&filename);
      if kind == SlotKind::Other {
        // Skip non-save files (e.g. backups, logs, temp files)
        continue;
      }

      let summary = Self::read_slot_summary(&path, kind, filename)?;
      slots.push(summary);
    }

    if slots.is_empty() {
      return Err(error::Error::NoSavesFound { path: dir });
    }

    slots.sort_by_key(|s| s.kind.sort_order());

    Ok(Self { dir, slots })
  }

  /// Returns a human-readable validation summary of discovered saves.
  #[must_use]
  pub fn summary_reason(&self) -> String {
    let count = self.slots.len();
    let names: Vec<&str> =
      self.slots.iter().map(|s| s.filename.as_str()).collect();
    format!(
      "Valid save set ({} file{} found: {})",
      count,
      if count == 1 { "" } else { "s" },
      names.join(", ")
    )
  }

  /// Returns the base directory path of the save set.
  #[must_use]
  pub fn dir(&self) -> &path::Path {
    &self.dir
  }

  /// Returns the slice of all discovered save slots.
  #[must_use]
  pub fn slots(&self) -> &[SlotSummary] {
    &self.slots
  }

  /// Finds a slot summary by filename.
  #[must_use]
  pub fn find_slot(&self, filename: &str) -> Option<&SlotSummary> {
    self.slots.iter().find(|s| s.filename == filename)
  }

  /// Loads and decompresses a campaign or battle save slot by filename.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::SlotNotFound`] if the file does not exist in this
  /// save set, [`error::Error::Io`] on read failure, or
  /// [`error::Error::Awakening`] if decompressing or parsing fails.
  pub fn load_slot(
    &self,
    filename: &str,
  ) -> Result<awakening::Save, error::Error> {
    let summary =
      self
        .find_slot(filename)
        .ok_or_else(|| error::Error::SlotNotFound {
          dir: self.dir.clone(),
          filename: filename.to_string(),
        })?;

    let bytes = fs::read(&summary.path).map_err(|source| error::Error::Io {
      path: summary.path.clone(),
      source,
    })?;

    let slot = awakening::Save::from_bytes(&bytes)?;
    Ok(slot)
  }

  /// Loads and parses the `Global` save file from this save set.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::SlotNotFound`] if `Global` is not in this save set,
  /// [`error::Error::Io`] on read failure, or [`error::Error::Awakening`] if
  /// parsing fails.
  pub fn load_global(&self) -> Result<awakening::GlobalSave, error::Error> {
    let summary =
      self
        .find_slot("Global")
        .ok_or_else(|| error::Error::SlotNotFound {
          dir: self.dir.clone(),
          filename: "Global".to_string(),
        })?;

    let bytes = fs::read(&summary.path).map_err(|source| error::Error::Io {
      path: summary.path.clone(),
      source,
    })?;

    let global = awakening::GlobalSave::from_bytes(&bytes)?;
    Ok(global)
  }

  /// Repacks and saves a save slot to disk, optionally creating a backup.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::SlotNotFound`] if the filename is not in this save
  /// set, [`error::Error::Io`] on write failure, or [`error::Error::Awakening`]
  /// if repacking fails.
  pub fn save_slot(
    &self,
    filename: &str,
    slot: &awakening::Save,
    create_backup: bool,
  ) -> Result<(), error::Error> {
    let summary =
      self
        .find_slot(filename)
        .ok_or_else(|| error::Error::SlotNotFound {
          dir: self.dir.clone(),
          filename: filename.to_string(),
        })?;

    if create_backup {
      let _ = backup::create_backup(&summary.path)?;
    }

    let packed = slot.to_bytes()?;
    fs::write(&summary.path, packed).map_err(|source| error::Error::Io {
      path: summary.path.clone(),
      source,
    })?;

    Ok(())
  }

  /// Repacks and saves the `Global` save file to disk, optionally creating a
  /// backup.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::SlotNotFound`] if `Global` is not in this save set,
  /// [`error::Error::Io`] on write failure, or [`error::Error::Awakening`] if
  /// repacking fails.
  pub fn save_global(
    &self,
    global: &awakening::GlobalSave,
    create_backup: bool,
  ) -> Result<(), error::Error> {
    let summary =
      self
        .find_slot("Global")
        .ok_or_else(|| error::Error::SlotNotFound {
          dir: self.dir.clone(),
          filename: "Global".to_string(),
        })?;

    if create_backup {
      let _ = backup::create_backup(&summary.path)?;
    }

    let packed = global.to_bytes()?;
    fs::write(&summary.path, packed).map_err(|source| error::Error::Io {
      path: summary.path.clone(),
      source,
    })?;

    Ok(())
  }

  /// Reads preview metadata for a slot without decompressing the full payload.
  fn read_slot_summary(
    path: &path::Path,
    kind: SlotKind,
    filename: String,
  ) -> Result<SlotSummary, error::Error> {
    if kind == SlotKind::Global {
      return Ok(SlotSummary {
        kind,
        filename,
        path: path.to_path_buf(),
        display_title: "Global Archive".to_string(),
        avatar_name: None,
        chapter_title: None,
        difficulty: None,
        mode: None,
        play_time: None,
        raw_preview: None,
      });
    }

    let bytes = fs::read(path).map_err(|source| error::Error::Io {
      path: path.to_path_buf(),
      source,
    })?;

    if bytes.len() >= header::PREVIEW_LEN {
      let mut raw = [0u8; header::PREVIEW_LEN];
      raw.copy_from_slice(&bytes[..header::PREVIEW_LEN]);
      let view = header::PreviewRef::from_bytes(&raw);

      let avatar_name = view.avatar_name();
      let chapter_title = view.chapter_title();
      let difficulty = view.difficulty();
      let mode = view.mode();
      let play_time = view.play_time();

      let prefix = match kind {
        SlotKind::Chapter(idx) => format!("Slot {}", idx + 1),
        SlotKind::Map(idx) => format!("Battle Save {}", idx + 1),
        _ => filename.clone(),
      };

      let display_title = if avatar_name.is_empty() {
        format!("{prefix}: {chapter_title}")
      } else {
        format!("{prefix}: {avatar_name} ({chapter_title})")
      };

      Ok(SlotSummary {
        kind,
        filename,
        path: path.to_path_buf(),
        display_title,
        avatar_name: Some(avatar_name),
        chapter_title: Some(chapter_title),
        difficulty,
        mode,
        play_time: Some(play_time),
        raw_preview: Some(raw),
      })
    } else {
      Ok(SlotSummary {
        kind,
        filename: filename.clone(),
        path: path.to_path_buf(),
        display_title: filename,
        avatar_name: None,
        chapter_title: None,
        difficulty: None,
        mode: None,
        play_time: None,
        raw_preview: None,
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_open_saveset_fixtures() {
    let fixture_dir = path::Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("../fedit-awakening/tests/fixtures/20200831-225628");
    let set =
      SaveSet::open_dir(&fixture_dir).expect("failed to open fixtures saveset");

    assert_eq!(set.slots().len(), 6);
    assert!(set.summary_reason().contains("6 files found"));

    let global = set.find_slot("Global").expect("missing Global");
    assert_eq!(global.kind, SlotKind::Global);

    let ch0 = set.find_slot("Chapter0").expect("missing Chapter0");
    assert_eq!(ch0.kind, SlotKind::Chapter(0));
    assert!(ch0.avatar_name.is_some());
    assert!(ch0.chapter_title.is_some());

    let ch1 = set.find_slot("Chapter1").expect("missing Chapter1");
    assert_eq!(ch1.kind, SlotKind::Chapter(1));

    let map0 = set.find_slot("Map0").expect("missing Map0");
    assert_eq!(map0.kind, SlotKind::Map(0));

    // Verify verified loading works
    let loaded_slot =
      set.load_slot("Chapter0").expect("failed to load Chapter0");
    assert!(!loaded_slot.roster.is_empty());

    let loaded_global = set.load_global().expect("failed to load Global");
    assert!(loaded_global.payload_len() > 0);
  }

  #[test]
  fn test_open_path_with_individual_file() {
    let file = path::Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("../fedit-awakening/tests/fixtures/20200831-225628/Chapter1");
    let (set, selected_file) =
      SaveSet::open_path(&file).expect("open Chapter1 path");
    assert_eq!(selected_file.as_deref(), Some("Chapter1"));
    assert_eq!(set.slots().len(), 6);
  }

  #[test]
  fn test_open_non_directory_fails() {
    let file = path::Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("../fedit-awakening/tests/fixtures/20200831-225628/Chapter0");
    let res = SaveSet::open_dir(&file);
    assert!(matches!(res, Err(error::Error::NotADirectory { .. })));
  }
}
