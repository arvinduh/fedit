//! Crash-resilient atomic file writing.
//!
//! Writes data to a sibling temporary file before atomically renaming into place.

use std::fs;
use std::io;
use std::io::Write;
use std::path;
use std::time;

/// Atomically writes content to a target file via temporary file replacement.
///
/// # Errors
///
/// Returns an [`io::Error`] if creating, writing, syncing, or renaming fails.
pub fn write_atomic(
  target: impl AsRef<path::Path>,
  bytes: &[u8],
) -> io::Result<()> {
  let target = target.as_ref();
  let parent = target.parent().unwrap_or_else(|| path::Path::new("."));
  let file_name = target
    .file_name()
    .and_then(|n| n.to_str())
    .unwrap_or("target");
  let nanos = time::SystemTime::now()
    .duration_since(time::UNIX_EPOCH)
    .unwrap_or_default()
    .as_nanos();
  let tmp_path = parent.join(format!(".{file_name}.tmp.{nanos}"));

  let mut file = fs::File::create(&tmp_path)?;
  let write_res = file.write_all(bytes).and_then(|()| file.sync_all());
  if let Err(err) = write_res {
    let _ = fs::remove_file(&tmp_path);
    return Err(err);
  }
  drop(file);

  if let Err(err) = fs::rename(&tmp_path, target) {
    let _ = fs::remove_file(&tmp_path);
    return Err(err);
  }
  Ok(())
}
