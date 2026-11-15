//! Save file backup subcommand.
//!
//! Creates a snapshot backup of a save file in standard `AppData` storage.

use std::path::PathBuf;

use clap;
use fedit_core;

use crate::error::Error;

/// Arguments for creating a save backup.
#[derive(Debug, clap::Args)]
pub struct BackupArgs {
  /// Path to save file to back up.
  pub file: PathBuf,
}

impl BackupArgs {
  /// Executes the backup subcommand.
  ///
  /// # Errors
  ///
  /// Returns an error if reading or backing up the file fails.
  pub fn run(&self) -> Result<(), Error> {
    let snap = fedit_core::create_backup(&self.file)?;
    let short_hash = &snap.root_hash[..8.min(snap.root_hash.len())];
    println!(
      "Snapshot backup created: {} (hash: {short_hash})",
      snap.backup_file
    );
    Ok(())
  }
}
