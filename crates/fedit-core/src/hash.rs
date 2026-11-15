//! SHA-256 cryptographic digest calculation.
//!
//! Provides hex-encoded checksums for save states and snapshot tracking.

use std::fmt::Write;
use std::fs;
use std::path;

use sha2;
use sha2::Digest;

use crate::error;

/// Computes a lowercase hex-encoded SHA-256 hash for a byte slice.
#[must_use]
pub fn compute_sha256(bytes: &[u8]) -> String {
  let mut hasher = sha2::Sha256::new();
  hasher.update(bytes);
  let result = hasher.finalize();
  let mut hex = String::with_capacity(64);
  for byte in result {
    let _ = write!(hex, "{byte:02x}");
  }
  hex
}

/// Computes a lowercase hex-encoded SHA-256 hash for a file on disk.
///
/// # Errors
///
/// Returns [`error::Error::Io`] if reading the file fails.
pub fn compute_file_sha256(path: &path::Path) -> Result<String, error::Error> {
  let bytes = fs::read(path).map_err(|source| error::Error::Io {
    path: path.to_path_buf(),
    source,
  })?;
  Ok(compute_sha256(&bytes))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compute_sha256_known_vector() {
    let empty = compute_sha256(b"");
    assert_eq!(
      empty,
      "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
  }
}
