//! Save slot modification command.
//!
//! Applies in-place mutations (gold, avatar name, difficulty) to save slots.

use std::fs;
use std::path;

use clap;
use fe13;
use fedit_core;

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
pub struct EditArgs {
  /// Path to the save slot file or archive directory to edit.
  pub file: path::PathBuf,
  /// Target chapter slot index when editing an archive directory (0..=2).
  #[arg(long)]
  pub chapter: Option<usize>,
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
  pub fn run(self) -> Result<(), error::Error> {
    if self.file.is_dir() {
      let mut archive = fedit_core::SaveArchive::open(&self.file)?;
      let ch_idx = self.chapter.unwrap_or(0);
      let slot = archive
        .chapters
        .get_mut(ch_idx)
        .and_then(Option::as_mut)
        .ok_or_else(|| fedit_core::Error::SlotNotFound {
          slot: format!("Chapter{ch_idx}"),
        })?;
      if let Some(gold) = self.gold {
        slot.set_gold(gold);
      }
      if let Some(ref name) = self.avatar_name {
        slot.preview_mut().set_avatar_name(name);
      }
      if let Some(diff) = self.difficulty {
        slot.preview_mut().set_difficulty(diff.into());
      }
      let backup = archive.save()?;
      if let Some(backup) = backup {
        println!("Backup created at {}", backup.display());
      }
      println!("Saved changes to Chapter{ch_idx}");
      return Ok(());
    }
    let bytes = fs::read(&self.file)?;
    let mut slot = fe13::SaveSlot::from_bytes(&bytes)?;

    if let Some(gold) = self.gold {
      slot.set_gold(gold);
    }
    if let Some(ref name) = self.avatar_name {
      slot.preview_mut().set_avatar_name(name);
    }
    if let Some(diff) = self.difficulty {
      slot.preview_mut().set_difficulty(diff.into());
    }

    let packed = slot.to_bytes()?;
    let out_path = self.output.as_ref().unwrap_or(&self.file);
    fs::write(out_path, packed)?;
    Ok(())
  }
}
