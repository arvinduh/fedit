//! Backup catalog manifest and snapshot metadata models.
//!
//! Owns serializable ledger tracking original and intermediate save snapshots.

use std::collections;
use std::fs;
use std::path;

use serde;
use serde_json;

use crate::error;

/// Current schema version for manifest files.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Metadata for an individual save file snapshot.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
  /// Pristine non-edited root ancestor hash for this lineage.
  pub root_hash: String,
  /// Direct predecessor snapshot hash if modified in sequence.
  pub parent_hash: Option<String>,
  /// Relative backup filename within the backups directory.
  pub backup_file: String,
  /// ISO 8601 creation timestamp string.
  pub timestamp: String,
}

/// Persistent backup manifest catalog.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Manifest {
  /// Schema specification version.
  pub schema_version: u32,
  /// Semantic application version that wrote this manifest.
  pub app_version: String,
  /// Canonical absolute save file paths mapped to active snapshot hashes.
  pub files: collections::HashMap<path::PathBuf, String>,
  /// Content-addressable snapshot entries keyed by SHA-256 hash.
  pub snapshots: collections::HashMap<String, Snapshot>,
}

impl Default for Manifest {
  fn default() -> Self {
    Self {
      schema_version: CURRENT_SCHEMA_VERSION,
      app_version: env!("CARGO_PKG_VERSION").to_string(),
      files: collections::HashMap::new(),
      snapshots: collections::HashMap::new(),
    }
  }
}

impl Manifest {
  /// Loads an existing manifest or constructs an empty default.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::Json`] if the manifest file is corrupted.
  pub fn load_or_default(path: &path::Path) -> Result<Self, error::Error> {
    if !path.exists() {
      return Ok(Self::default());
    }
    let data = fs::read(path).map_err(|source| error::Error::Io {
      path: path.to_path_buf(),
      source,
    })?;
    Ok(serde_json::from_slice(&data)?)
  }

  /// Atomically writes the manifest to disk.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::Io`] if directory creation or file writing fails.
  pub fn save(&self, path: &path::Path) -> Result<(), error::Error> {
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent).map_err(|source| error::Error::Io {
        path: parent.to_path_buf(),
        source,
      })?;
    }
    let json = serde_json::to_vec_pretty(self)?;
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, json).map_err(|source| error::Error::Io {
      path: tmp.clone(),
      source,
    })?;
    fs::rename(&tmp, path).map_err(|source| error::Error::Io {
      path: path.to_path_buf(),
      source,
    })?;
    Ok(())
  }

  /// Registers a newly created snapshot in the catalog.
  pub fn record_snapshot(
    &mut self,
    canonical_path: &path::Path,
    content_hash: &str,
    backup_file: String,
    timestamp: String,
  ) -> Snapshot {
    let (root_hash, parent_hash) =
      if let Some(prev_hash) = self.files.get(canonical_path) {
        if let Some(prev_snap) = self.snapshots.get(prev_hash) {
          (prev_snap.root_hash.clone(), Some(prev_hash.clone()))
        } else {
          (content_hash.to_string(), None)
        }
      } else {
        (content_hash.to_string(), None)
      };

    let snapshot = Snapshot {
      root_hash,
      parent_hash,
      backup_file,
      timestamp,
    };

    self
      .files
      .insert(canonical_path.to_path_buf(), content_hash.to_string());
    self
      .snapshots
      .insert(content_hash.to_string(), snapshot.clone());

    snapshot
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_manifest_record_and_roundtrip() {
    let mut manifest = Manifest::default();
    let file = path::PathBuf::from("test/Chapter0");
    let snap1 = manifest.record_snapshot(
      &file,
      "hash1",
      "Chapter0_hash1.sav".to_string(),
      "2026-11-14T18:00:00Z".to_string(),
    );
    assert_eq!(snap1.root_hash, "hash1");
    assert_eq!(snap1.parent_hash, None);

    let snap2 = manifest.record_snapshot(
      &file,
      "hash2",
      "Chapter0_hash2.sav".to_string(),
      "2026-11-14T18:10:00Z".to_string(),
    );
    assert_eq!(snap2.root_hash, "hash1");
    assert_eq!(snap2.parent_hash, Some("hash1".to_string()));
  }
}
