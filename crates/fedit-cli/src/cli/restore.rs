//! Save file restoration subcommand.
//!
//! Restores a save file from `AppData` snapshot storage back to disk.

use std::path::PathBuf;

use clap;
use fedit_core;

use crate::error::Error;

/// Arguments for restoring a save file to an earlier snapshot.
#[derive(Debug, clap::Args)]
pub struct RestoreArgs {
  /// Path to save file to restore.
  pub file: PathBuf,

  /// Target snapshot hash to restore to (defaults to earliest unedited root).
  #[arg(long = "to")]
  pub to: Option<String>,

  /// List historical snapshots without restoring.
  #[arg(long = "list")]
  pub list: bool,
}

impl RestoreArgs {
  /// Executes the restore subcommand.
  ///
  /// # Errors
  ///
  /// Returns an error if snapshot resolution or file restoration fails.
  pub fn run(&self) -> Result<(), Error> {
    if self.list {
      let history = fedit_core::list_history(&self.file)?;
      if history.is_empty() {
        println!("No snapshot history found for {}", self.file.display());
        return Ok(());
      }
      println!("Snapshot history for {}:", self.file.display());
      println!("Snapshot Hash  Timestamp             Original?");
      println!("------------------------------------------------");
      for entry in history {
        let short_hash = &entry.hash[..8.min(entry.hash.len())];
        println!(
          "{:<14} {:<21} {}",
          short_hash,
          entry.timestamp,
          if entry.is_root {
            "Yes (Root)"
          } else {
            "No (Edit)"
          }
        );
      }
      return Ok(());
    }

    match fedit_core::restore(&self.file, self.to.as_deref()) {
      Ok(outcome) => {
        let short_hash = &outcome.hash[..8.min(outcome.hash.len())];
        if outcome.is_root {
          println!(
            "Restored {} to original pristine state ({short_hash}) from {}",
            self.file.display(),
            outcome.backup_file
          );
        } else {
          println!(
            "Restored {} to intermediate state ({short_hash}) from {}",
            self.file.display(),
            outcome.backup_file
          );
        }
        Ok(())
      }
      Err(fedit_core::Error::NoLineageFound { path }) => {
        eprintln!(
          "Original version could not be found for '{}'.",
          path.display()
        );
        eprintln!(
          "Please check backups directly in: {}",
          fedit_core::backups_dir().display()
        );
        Err(Error::Core(fedit_core::Error::NoLineageFound { path }))
      }
      Err(e) => Err(Error::Core(e)),
    }
  }
}
