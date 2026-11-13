//! Save container unpacking command.
//!
//! Decompresses PMOC save containers to raw payload files and preview sidecars.

use std::fs;
use std::path;

use clap;
use fe_compress;
use serde;
use serde_json;

use crate::cli::error;

/// Unpack summary output structure for `--json` format.
#[derive(Debug, serde::Serialize)]
struct UnpackSummary {
  /// Container format kind ("global" or "slot").
  container_kind: &'static str,
  /// Decompressed payload length in bytes.
  payload_len: usize,
  /// Preview header length in bytes.
  preview_len: usize,
}

/// Arguments for the `unpack` subcommand.
#[derive(Debug, clap::Args)]
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
  pub fn run(self) -> Result<(), error::Error> {
    let bytes = fs::read(&self.file)?;
    let container = fe_compress::unpack(&bytes)?;
    let out_path = self
      .output
      .unwrap_or_else(|| self.file.with_extension("bin"));
    fs::write(out_path, &container.payload)?;

    if let Some(prev_path) = self.preview
      && !container.preview.is_empty()
    {
      fs::write(prev_path, &container.preview)?;
    }

    if self.json {
      let summary = UnpackSummary {
        container_kind: if container.preview.is_empty() {
          "global"
        } else {
          "slot"
        },
        payload_len: container.payload.len(),
        preview_len: container.preview.len(),
      };
      println!("{}", serde_json::to_string_pretty(&summary)?);
    }
    Ok(())
  }
}
