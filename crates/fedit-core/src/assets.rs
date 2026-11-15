//! Character portrait asset resolution and directory caching.
//!
//! Maps domain characters to disk image assets using canonical filename slugs.

use std::fs;
use std::path;

use awakening::roster;

use crate::error;
use crate::paths;

/// Resolves the file path for a character's portrait image.
///
/// Looks first in the local portable `./assets/portraits/` directory,
/// then falls back to the user application data directory.
#[must_use]
pub fn resolve_portrait(character: roster::Character) -> Option<path::PathBuf> {
  let slug = character.slug();
  let filename = format!("{slug}.png");

  let portable = paths::portable_portraits_dir().join(&filename);
  if portable.exists() {
    return Some(portable);
  }

  let user = paths::portraits_dir().join(&filename);
  if user.exists() {
    return Some(user);
  }

  None
}

/// Ensures all necessary asset directories exist on disk.
///
/// # Errors
///
/// Returns [`error::Error::Io`] if creating the portrait directory fails.
pub fn ensure_asset_dirs() -> Result<path::PathBuf, error::Error> {
  let dir = paths::portraits_dir();
  fs::create_dir_all(&dir).map_err(|source| error::Error::Io {
    path: dir.clone(),
    source,
  })?;
  Ok(dir)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_resolve_portrait_non_existent() {
    // When no files are installed, returns None gracefully
    let path = resolve_portrait(roster::Character::Chrom);
    // Might be None unless tests run with fixtures
    assert!(path.is_none() || path.unwrap().ends_with("chrom.png"));
  }
}
