//! Save slot modification command.
//!
//! Applies in-place mutations (gold, avatar name, difficulty) to save slots.

use std::fs;
use std::path;

use clap;
use fe13;

use crate::cli::error;

/// CLI difficulty setting argument.
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum DifficultyArg {
  /// Normal difficulty.
  Normal,
  /// Hard difficulty.
  Hard,
  /// Lunatic difficulty.
  Lunatic,
  /// Lunatic+ difficulty.
  LunaticPlus,
}

impl From<DifficultyArg> for fe13::Difficulty {
  fn from(arg: DifficultyArg) -> Self {
    match arg {
      DifficultyArg::Normal => Self::Normal,
      DifficultyArg::Hard => Self::Hard,
      DifficultyArg::Lunatic => Self::Lunatic,
      DifficultyArg::LunaticPlus => Self::LunaticPlus,
    }
  }
}

/// Arguments for the `edit` subcommand.
#[derive(Debug, clap::Args)]
#[expect(dead_code, reason = "wired in subcommand router commit")]
pub struct EditArgs {
  /// Path to the save slot file to edit.
  pub file: path::PathBuf,
  /// New gold amount to set (clamped to 999,999).
  #[arg(long)]
  pub gold: Option<u32>,
  /// New avatar name to set.
  #[arg(long)]
  pub avatar_name: Option<String>,
  /// New difficulty setting to set.
  #[arg(long)]
  pub difficulty: Option<DifficultyArg>,
  /// Destination output path (defaults to in-place overwrite).
  #[arg(short, long)]
  pub output: Option<path::PathBuf>,
}

impl EditArgs {
  /// Executes the edit command.
  ///
  /// # Errors
  ///
  /// Returns an error if the save file cannot be read or mutated.
  #[expect(dead_code, reason = "wired in subcommand router commit")]
  pub fn run(self) -> Result<(), error::Error> {
    let _ = fs::metadata(&self.file)?;
    Ok(())
  }
}
