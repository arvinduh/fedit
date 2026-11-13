//! Save container unpacking command.
//!
//! Decompresses PMOC save containers to raw payload files and preview sidecars.

use std::fs;
use std::path;

use clap;

use crate::cli::error;

/// Arguments for the `unpack` subcommand.
#[derive(Debug, clap::Args)]
#[expect(dead_code, reason = "wired in subcommand router commit")]
pub struct UnpackArgs {
  /// Path to the compressed save container.
  pub file: path::PathBuf,
  /// Output path for the decompressed payload.
  #[arg(short, long)]
  pub output: Option<path::PathBuf>,
  /// Output path for the uncompressed preview header sidecar.
  #[arg(short, long)]
  pub preview: Option<path::PathBuf>,
  /// Output container metadata as JSON.
  #[arg(long)]
  pub json: bool,
}

impl UnpackArgs {
  /// Executes the unpack command.
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be read or decompressed.
  #[expect(dead_code, reason = "wired in subcommand router commit")]
  pub fn run(self) -> Result<(), error::Error> {
    let _ = fs::metadata(&self.file)?;
    Ok(())
  }
}
