//! Save slot modification command.
//!
//! Applies in-place mutations (gold, avatar name, difficulty) to save slots.

use std::fs;
use std::io;
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
  /// New play time duration ("hh:mm:ss" or total seconds).
  #[arg(long)]
  pub play_time: Option<String>,
  /// Destination output path (defaults to in-place overwrite).
  #[arg(short, long)]
  pub output: Option<path::PathBuf>,
  /// Create a backup before mutating.
  #[arg(long)]
  pub backup: bool,
}

impl EditArgs {
  /// Executes the edit command.
  ///
  /// # Errors
  ///
  /// Returns an error if the save file cannot be read or mutated.
  pub fn run(self) -> Result<(), error::Error> {
    if self.file.is_dir() {
      return Err(
        io::Error::new(
          io::ErrorKind::InvalidInput,
          "directories not supported; pass a single save slot file",
        )
        .into(),
      );
    }
    if self.backup
      && (self.output.is_none() || self.output.as_ref() == Some(&self.file))
    {
      let bak = self.file.with_extension("bak");
      fs::copy(&self.file, &bak)?;
      println!("Backup created at {}", bak.display());
    }

    let bytes = fs::read(&self.file)?;
    let mut slot = fe13::SaveSlot::from_bytes(&bytes)?;

    if let Some(gold) = self.gold {
      slot.set_gold(gold)?;
    }
    if let Some(ref name) = self.avatar_name {
      slot.preview_mut().set_avatar_name(name);
    }
    if let Some(diff) = self.difficulty {
      slot.preview_mut().set_difficulty(diff.into());
    }
    if let Some(t) = self.play_time.as_deref().and_then(parse_play_time) {
      slot.set_play_time(t);
    }

    let packed = slot.to_bytes()?;
    let out_path = self.output.as_ref().unwrap_or(&self.file);
    crate::atomic::write_atomic(out_path, &packed)?;
    Ok(())
  }
}

/// Parses duration string formatted as "hh:mm:ss" or integer seconds.
fn parse_play_time(s: &str) -> Option<fe13::PlayTime> {
  let parts: Vec<&str> = s.split(':').collect();
  if parts.len() == 3 {
    let h: u64 = parts[0].parse().ok()?;
    let m: u64 = parts[1].parse().ok()?;
    let s: u64 = parts[2].parse().ok()?;
    Some(fe13::PlayTime::from_secs(h * 3600 + m * 60 + s))
  } else {
    s.parse::<u64>().ok().map(fe13::PlayTime::from_secs)
  }
}
