//! Save file inspection command.
//!
//! Parses and validates Nintendo 3DS Fire Emblem save files, printing
//! header metadata, play statistics, and CRC-32 integrity status.

use std::fs;
use std::path;

use clap;

use crate::cli::error;

/// Arguments for the `inspect` subcommand.
#[derive(Debug, clap::Args)]
#[expect(dead_code, reason = "wired in subcommand router commit")]
pub struct InspectArgs {
  /// Path to the save file to inspect.
  pub file: path::PathBuf,
}

impl InspectArgs {
  /// Executes the inspect command.
  ///
  /// # Errors
  ///
  /// Returns an error if the save file cannot be read or validated.
  #[expect(dead_code, reason = "wired in subcommand router commit")]
  pub fn run(self) -> Result<(), error::Error> {
    let _ = fs::metadata(&self.file)?;
    Ok(())
  }
}
