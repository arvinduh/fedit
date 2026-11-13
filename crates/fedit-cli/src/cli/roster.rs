//! Character unit roster inspection command.

use std::fs;
use std::io;
use std::path;

use clap;
use fe13;
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
    let slot = fe13::SaveSlot::from_bytes(&fs::read(&self.file)?)?;
    if self.json {
      let rows: Vec<serde_json::Value> = slot
        .roster
        .iter()
        .enumerate()
        .map(|(i, u)| {
          let s = u.stats;
          serde_json::json!({
            "index": i,
            "unit_id": u.unit_id.0,
            "name": u.unit_id.name(),
            "class_id": u.class_id.0,
            "hp": s.hp,
            "strength": s.strength,
            "magic": s.magic,
            "skill": s.skill,
            "speed": s.speed,
            "luck": s.luck,
            "defense": s.defense,
            "resistance": s.resistance,
          })
        })
        .collect();
      println!("{}", serde_json::to_string_pretty(&rows)?);
      return Ok(());
    }
    println!("Idx  Unit Name            Class  HP Str Mag Skl Spd Lck Def Res");
    println!("---------------------------------------------------------------");
    for (i, u) in slot.roster.iter().enumerate() {
      let s = u.stats;
      let name = u.unit_id.name().unwrap_or("Unknown");
      println!(
        "{i:3}  {:4} {:<15} {:5} {:3} {:3} {:3} {:3} {:3} {:3} {:3} {:3}",
        u.unit_id.0,
        name,
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
