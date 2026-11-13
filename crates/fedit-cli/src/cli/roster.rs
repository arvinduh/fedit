//! Character unit roster inspection command.

use std::fs;
use std::path;

use clap;
use fe13;
use fedit_core;

use crate::cli::error;

/// Arguments for the `roster` subcommand.
#[derive(Debug, clap::Args)]
pub struct RosterArgs {
  /// Path to save file or archive directory.
  pub file: path::PathBuf,
  /// Target chapter index when inspecting an archive directory (0..=2).
  #[arg(long)]
  pub chapter: Option<usize>,
}

impl RosterArgs {
  /// Executes the roster command.
  ///
  /// # Errors
  ///
  /// Returns an error if the save file cannot be read or parsed.
  pub fn run(self) -> Result<(), error::Error> {
    let slot = if self.file.is_dir() {
      let a = fedit_core::SaveArchive::open(&self.file)?;
      let c = self.chapter.unwrap_or(0);
      a.chapters[c]
        .clone()
        .ok_or_else(|| fedit_core::Error::SlotNotFound {
          slot: format!("Chapter{c}"),
        })?
    } else {
      fe13::SaveSlot::from_bytes(&fs::read(&self.file)?)?
    };
    println!("Idx  Unit Class  HP Str Mag Skl Spd Lck Def Res");
    println!("----------------------------------------------");
    for (i, u) in slot.roster.iter().enumerate() {
      let s = u.stats;
      println!(
        "{i:3}  {:4} {:5} {:3} {:3} {:3} {:3} {:3} {:3} {:3} {:3}",
        u.unit_id.0,
        u.class_id.0,
        s.hp,
        s.strength,
        s.magic,
        s.skill,
        s.speed,
        s.luck,
        s.defense,
        s.resistance
      );
    }
    Ok(())
  }
}
