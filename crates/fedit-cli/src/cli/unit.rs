//! Character unit stat inspection and modification command.

use std::path;

use clap;

use crate::cli::error;

/// Arguments for the `unit` subcommand.
#[derive(Debug, clap::Args)]
pub struct UnitArgs {
  /// Path to save file or archive directory.
  pub file: path::PathBuf,
  /// Target chapter slot index (0..=2).
  #[arg(long)]
  pub chapter: Option<usize>,
  /// Target unit roster index.
  #[arg(long)]
  pub index: usize,
  #[arg(long)]
  pub hp: Option<u8>,
  #[arg(long)]
  pub strength: Option<u8>,
  #[arg(long)]
  pub magic: Option<u8>,
  #[arg(long)]
  pub skill: Option<u8>,
  #[arg(long)]
  pub speed: Option<u8>,
  #[arg(long)]
  pub luck: Option<u8>,
  #[arg(long)]
  pub defense: Option<u8>,
  #[arg(long)]
  pub resistance: Option<u8>,
  /// Destination output path.
  #[arg(short, long)]
  pub output: Option<path::PathBuf>,
}

impl UnitArgs {
  /// Executes the unit command.
  pub fn run(self) -> Result<(), error::Error> {
    Ok(())
  }
}
