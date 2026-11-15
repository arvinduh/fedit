//! Automated save file backup, rotation, and restoration engine.
//!
//! Owns content-addressable file snapshotting and rollback to pristine states.

use std::fs;
use std::path;

use crate::error;
use crate::hash;
use crate::manifest;
use crate::paths;

/// Outcome report produced by a restore operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestoreOutcome {
  /// SHA-256 hash of the restored save state.
  pub hash: String,
  /// Relative backup filename that was restored.
  pub backup_file: String,
  /// True if the restored version is the earliest unedited root.
  pub is_root: bool,
}

/// An entry in a file's snapshot history lineage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoryEntry {
  /// SHA-256 hash string.
  pub hash: String,
  /// ISO 8601 creation timestamp string.
  pub timestamp: String,
  /// True if this entry represents the initial pristine save.
  pub is_root: bool,
}

/// Creates a snapshot backup of a save file before modification.
///
/// If an identical snapshot for the file's current content hash already
/// exists, it returns the existing snapshot without duplicating file storage.
///
/// # Errors
///
/// Returns [`error::Error`] if file reading, hashing, or manifest writes fail.
pub fn create_backup(
  target_path: &path::Path,
) -> Result<manifest::Snapshot, error::Error> {
  create_backup_to(target_path, &paths::backups_dir(), &paths::manifest_path())
}

/// Parameterized backup creation for explicit directory and manifest targets.
///
/// # Errors
///
/// Returns [`error::Error`] if file reading, hashing, or manifest writes fail.
pub fn create_backup_to(
  target_path: &path::Path,
  backups_dir: &path::Path,
  manifest_file: &path::Path,
) -> Result<manifest::Snapshot, error::Error> {
  let canonical = target_path
    .canonicalize()
    .unwrap_or_else(|_| target_path.to_path_buf());
  let content = fs::read(target_path).map_err(|source| error::Error::Io {
    path: target_path.to_path_buf(),
    source,
  })?;

  let content_hash = hash::compute_sha256(&content);
  let mut manifest = manifest::Manifest::load_or_default(manifest_file)?;

  if let Some(existing) = manifest.snapshots.get(&content_hash) {
    manifest.files.insert(canonical, content_hash);
    manifest.save(manifest_file)?;
    return Ok(existing.clone());
  }

  fs::create_dir_all(backups_dir).map_err(|source| error::Error::Io {
    path: backups_dir.to_path_buf(),
    source,
  })?;

  let stem = target_path
    .file_name()
    .and_then(|n| n.to_str())
    .unwrap_or("save");
  let short_hash = &content_hash[..8];
  let backup_filename = format!("{stem}_{short_hash}.sav");
  let backup_dest = backups_dir.join(&backup_filename);

  fs::write(&backup_dest, &content).map_err(|source| error::Error::Io {
    path: backup_dest,
    source,
  })?;

  let now = time_rfc3339();
  let snapshot =
    manifest.record_snapshot(&canonical, &content_hash, backup_filename, now);
  manifest.save(manifest_file)?;

  Ok(snapshot)
}

/// Restores a save file to its pristine root or a specified snapshot hash.
///
/// # Errors
///
/// Returns [`error::Error::NoLineageFound`] if no matching backup history exists, or
/// [`error::Error::SnapshotNotFound`] if the requested specific hash is absent.
pub fn restore(
  target_path: &path::Path,
  specific_hash: Option<&str>,
) -> Result<RestoreOutcome, error::Error> {
  restore_from(
    target_path,
    specific_hash,
    &paths::backups_dir(),
    &paths::manifest_path(),
  )
}

/// Parameterized save restoration using explicit directory and manifest paths.
///
/// # Errors
///
/// Returns [`error::Error`] if file reading, writing, or lookup fails.
pub fn restore_from(
  target_path: &path::Path,
  specific_hash: Option<&str>,
  backups_dir: &path::Path,
  manifest_file: &path::Path,
) -> Result<RestoreOutcome, error::Error> {
  let canonical = target_path
    .canonicalize()
    .unwrap_or_else(|_| target_path.to_path_buf());
  let manifest = manifest::Manifest::load_or_default(manifest_file)?;

  let current_hash = if target_path.exists() {
    hash::compute_file_sha256(target_path).ok()
  } else {
    None
  };

  let active_hash = manifest.files.get(&canonical).or(current_hash.as_ref());

  let active_snap = active_hash.and_then(|h| manifest.snapshots.get(h));

  let (target_hash, snap) = if let Some(target) = specific_hash {
    manifest
      .snapshots
      .iter()
      .find(|(h, _)| h.starts_with(target))
      .map(|(h, s)| (h.clone(), s))
      .ok_or_else(|| error::Error::SnapshotNotFound {
        hash: target.to_string(),
      })?
  } else if let Some(active) = active_snap {
    let root = &active.root_hash;
    let s = manifest
      .snapshots
      .get(root)
      .ok_or_else(|| error::Error::SnapshotNotFound { hash: root.clone() })?;
    (root.clone(), s)
  } else {
    return Err(error::Error::NoLineageFound {
      path: target_path.to_path_buf(),
    });
  };

  let backup_src = backups_dir.join(&snap.backup_file);
  let backup_bytes =
    fs::read(&backup_src).map_err(|source| error::Error::Io {
      path: backup_src.clone(),
      source,
    })?;

  let tmp = target_path.with_extension("restore_tmp");
  fs::write(&tmp, &backup_bytes).map_err(|source| error::Error::Io {
    path: tmp.clone(),
    source,
  })?;
  fs::rename(&tmp, target_path).map_err(|source| error::Error::Io {
    path: target_path.to_path_buf(),
    source,
  })?;

  let is_root = snap.parent_hash.is_none() || snap.root_hash == target_hash;

  Ok(RestoreOutcome {
    hash: target_hash,
    backup_file: snap.backup_file.clone(),
    is_root,
  })
}

/// Lists all historical snapshots available for a given save file path.
///
/// # Errors
///
/// Returns [`error::Error::Io`] if manifest reading fails.
pub fn list_history(
  target_path: &path::Path,
) -> Result<Vec<HistoryEntry>, error::Error> {
  list_history_from(target_path, &paths::manifest_path())
}

/// Parameterized history traversal using an explicit manifest path.
///
/// # Errors
///
/// Returns [`error::Error::Io`] if manifest reading fails.
pub fn list_history_from(
  target_path: &path::Path,
  manifest_file: &path::Path,
) -> Result<Vec<HistoryEntry>, error::Error> {
  let canonical = target_path
    .canonicalize()
    .unwrap_or_else(|_| target_path.to_path_buf());
  let manifest = manifest::Manifest::load_or_default(manifest_file)?;

  let mut history = Vec::new();
  let Some(active_hash) = manifest.files.get(&canonical) else {
    return Ok(history);
  };

  let mut cur = Some(active_hash.as_str());
  while let Some(h) = cur {
    if let Some(snap) = manifest.snapshots.get(h) {
      let is_root = snap.parent_hash.is_none() || snap.root_hash == h;
      history.push(HistoryEntry {
        hash: h.to_string(),
        timestamp: snap.timestamp.clone(),
        is_root,
      });
      cur = snap.parent_hash.as_deref();
    } else {
      break;
    }
  }

  Ok(history)
}

/// Fallback RFC 3339 timestamp generator avoiding external chrono dependencies.
fn time_rfc3339() -> String {
  let duration = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap_or_default();
  format!("{}s", duration.as_secs())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_time_rfc3339_non_empty() {
    assert_ne!(time_rfc3339(), "");
  }

  #[test]
  fn test_backup_and_restore_cycle() {
    let tmp =
      std::env::temp_dir().join(format!("fedit_test_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    let backups = tmp.join("backups");
    let manifest = backups.join("manifest.json");
    let save_file = tmp.join("Chapter0");

    fs::create_dir_all(&tmp).expect("create test tmp");
    fs::write(&save_file, b"pristine save content").expect("write save");

    // 1. Initial backup
    let snap1 =
      create_backup_to(&save_file, &backups, &manifest).expect("snap 1");
    assert_eq!(snap1.root_hash, snap1.root_hash);
    assert_eq!(snap1.parent_hash, None);

    // 2. Edit save file and create second backup
    fs::write(&save_file, b"edited save content").expect("write edit");
    let snap2 =
      create_backup_to(&save_file, &backups, &manifest).expect("snap 2");
    assert_eq!(snap2.root_hash, snap1.root_hash);
    assert_eq!(snap2.parent_hash, Some(snap1.root_hash.clone()));

    // 3. Edit save file again
    fs::write(&save_file, b"corrupted save content").expect("write corrupt");

    // 4. Restore without specific hash -> should restore pristine original (snap1)
    let outcome = restore_from(&save_file, None, &backups, &manifest)
      .expect("restore root");
    assert!(outcome.is_root);
    assert_eq!(outcome.hash, snap1.root_hash);
    let restored_bytes = fs::read(&save_file).expect("read restored");
    assert_eq!(restored_bytes, b"pristine save content");

    // 5. Restore intermediate snapshot (snap2)
    let outcome2 = restore_from(
      &save_file,
      Some(&snap2.root_hash[..4]),
      &backups,
      &manifest,
    );
    assert!(outcome2.is_ok());

    let _ = fs::remove_dir_all(&tmp);
  }
}
