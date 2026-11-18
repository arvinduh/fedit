//! Character unit roster inspection command.

use std::fs;
use std::io;
use std::path;

use awakening;
use clap;
use serde_json;

use crate::cli::error;

/// Arguments for the `roster` subcommand.
#[derive(Debug, clap::Args)]
pub struct RosterArgs {
  /// Path to save slot file.
  pub file: path::PathBuf,
  /// Output roster as JSON.
  #[arg(long)]
  pub json: bool,
}

impl RosterArgs {
  /// Executes the roster command.
  ///
  /// # Errors
  ///
  /// Returns an error if the save file cannot be read or parsed.
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
    let slot = awakening::Save::from_bytes(&fs::read(&self.file)?)?;
    if self.json {
      let rows: Vec<serde_json::Value> = slot
        .roster
        .iter()
        .enumerate()
        .map(|(i, u)| {
          serde_json::json!({
            "index": i,
            "name": u.name(),
            "class": u.class().name(),
            "hp": u.hp.total(),
            "strength": u.strength.total(),
            "magic": u.magic.total(),
            "skill": u.skill.total(),
            "speed": u.speed.total(),
            "luck": u.luck.total(),
            "defense": u.defense.total(),
            "resistance": u.resistance.total(),
          })
        })
        .collect();
      println!("{}", serde_json::to_string_pretty(&rows)?);
      return Ok(());
    }
    println!(
      "Idx  Unit Name       Class          HP Str Mag Skl Spd Lck Def Res"
    );
    println!(
      "------------------------------------------------------------------"
    );
    for (i, u) in slot.roster.iter().enumerate() {
      let name = u.name();
      let class_name = u.class().name();
      println!(
        "{i:3}  {:<15} {:<14} {:3} {:3} {:3} {:3} {:3} {:3} {:3} {:3}",
        name,
        class_name,
        u.hp.total(),
        u.strength.total(),
        u.magic.total(),
        u.skill.total(),
        u.speed.total(),
        u.luck.total(),
        u.defense.total(),
        u.resistance.total()
      );
    }
    Ok(())
  }
}
