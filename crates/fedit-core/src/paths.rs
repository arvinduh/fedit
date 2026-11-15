//! Operating system standard directories and path resolvers.
//!
//! Owns path resolution for application data, backups, and cached assets.

use std::env;
use std::path;

/// Application root data directory name.
const APP_DIR_NAME: &str = "fedit";

/// Resolves the operating system's standard local application data directory.
///
/// On Windows: `%LOCALAPPDATA%\fedit`
/// On Unix/Linux: `$XDG_DATA_HOME/fedit` or `~/.local/share/fedit`
/// On macOS: `~/Library/Application Support/fedit`
#[must_use]
pub fn data_dir() -> path::PathBuf {
  #[cfg(target_os = "windows")]
  {
    if let Some(local_app_data) = env::var_os("LOCALAPPDATA") {
      return path::PathBuf::from(local_app_data).join(APP_DIR_NAME);
    }
    if let Some(user_profile) = env::var_os("USERPROFILE") {
      return path::PathBuf::from(user_profile)
        .join("AppData")
        .join("Local")
        .join(APP_DIR_NAME);
    }
  }

  #[cfg(not(target_os = "windows"))]
  {
    if let Some(xdg) = env::var_os("XDG_DATA_HOME") {
      return path::PathBuf::from(xdg).join(APP_DIR_NAME);
    }
    if let Some(home) = env::var_os("HOME") {
      return path::PathBuf::from(home)
        .join(".local")
        .join("share")
        .join(APP_DIR_NAME);
    }
  }

  path::PathBuf::from(".").join(APP_DIR_NAME)
}

/// Directory path where save file backups are stored.
#[must_use]
pub fn backups_dir() -> path::PathBuf {
  data_dir().join("backups")
}

/// File path to the persistent backup manifest JSON.
#[must_use]
pub fn manifest_path() -> path::PathBuf {
  backups_dir().join("manifest.json")
}

/// Directory path for cached character portraits and graphics.
#[must_use]
pub fn portraits_dir() -> path::PathBuf {
  data_dir().join("assets").join("portraits")
}

/// Portable local portraits directory adjacent to executable or workspace.
#[must_use]
pub fn portable_portraits_dir() -> path::PathBuf {
  path::PathBuf::from("assets").join("portraits")
}

/// Directory path for cached skill icons.
#[must_use]
pub fn skills_dir() -> path::PathBuf {
  data_dir().join("assets").join("skills")
}

/// Portable local skill icons directory adjacent to executable or workspace.
#[must_use]
pub fn portable_skills_dir() -> path::PathBuf {
  path::PathBuf::from("assets").join("skills")
}

/// Directory path for cached item and weapon icons.
#[must_use]
pub fn items_dir() -> path::PathBuf {
  data_dir().join("assets").join("items")
}

/// Portable local item icons directory adjacent to executable or workspace.
#[must_use]
pub fn portable_items_dir() -> path::PathBuf {
  path::PathBuf::from("assets").join("items")
}

/// Directory path for cached animated class map sprites.
#[must_use]
pub fn sprites_dir() -> path::PathBuf {
  data_dir().join("assets").join("sprites")
}

/// Portable local map sprites directory adjacent to executable or workspace.
#[must_use]
pub fn portable_sprites_dir() -> path::PathBuf {
  path::PathBuf::from("assets").join("sprites")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_data_paths_non_empty() {
    assert_ne!(data_dir().as_os_str(), "");
    assert_ne!(backups_dir().as_os_str(), "");
    assert!(manifest_path().ends_with("manifest.json"));
  }
}
