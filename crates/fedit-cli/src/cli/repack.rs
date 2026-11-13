//! Save container repacking command.
//!
//! Compresses raw payload files and optional preview headers into PMOC containers.

use std::fs;
use std::path;

use clap;
use fe_compress;

use crate::cli::error;

/// Arguments for the `repack` subcommand.
#[derive(Debug, clap::Args)]
pub struct RepackArgs {
  /// Path to the decompressed payload file.
  pub payload: path::PathBuf,
  /// Path to optional 192-byte preview header sidecar.
  #[arg(short, long)]
  pub preview: Option<path::PathBuf>,
  /// Destination output path for the repacked save container.
  #[arg(short, long)]
  pub output: path::PathBuf,
}

impl RepackArgs {
  /// Executes the repack command.
  ///
  /// # Errors
  ///
  /// Returns an error if inputs cannot be read or compressed.
  pub fn run(self) -> Result<(), error::Error> {
    let payload = fs::read(&self.payload)?;
    let preview = match self.preview {
      Some(ref p) => fs::read(p)?,
      None => Vec::new(),
    };
    let packed = fe_compress::repack(&preview, &payload)?;
    fs::write(&self.output, packed)?;
    Ok(())
  }
}
