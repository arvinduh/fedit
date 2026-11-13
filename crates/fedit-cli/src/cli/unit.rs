//! Character unit stat inspection and modification command.

use std::fs;
use std::path;

use clap;
use fe13;
use fedit_core;

use crate::cli::error;

/// Arguments for the `unit` subcommand.
#[derive(Debug, clap::Args)]
pub struct UnitArgs {
  /// Path to save file or archive directory.
  pub file: path::PathBuf,
  /// Target unit roster index.
  #[arg(long)]
  pub index: Option<usize>,
  /// Target unit character name (case-insensitive).
  #[arg(short, long)]
  pub name: Option<String>,
  /// New max hit points stat.
  #[arg(long)]
  pub hp: Option<u8>,
  /// New strength stat.
  #[arg(long)]
  pub strength: Option<u8>,
  /// New magic stat.
  #[arg(long)]
  pub magic: Option<u8>,
  /// New skill stat.
  #[arg(long)]
  pub skill: Option<u8>,
  /// New speed stat.
  #[arg(long)]
  pub speed: Option<u8>,
  /// New luck stat.
  #[arg(long)]
  pub luck: Option<u8>,
  /// New defense stat.
  #[arg(long)]
  pub defense: Option<u8>,
  /// New resistance stat.
  #[arg(long)]
  pub resistance: Option<u8>,
  /// Destination output path.
  #[arg(short, long)]
  pub output: Option<path::PathBuf>,
  /// Create a backup before mutating.
  #[arg(long)]
  pub backup: bool,
}



impl UnitArgs {
  /// Executes the unit command.
  pub fn run(self) -> Result<(), error::Error> {
    let (mut a, mut f_slot) = if self.file.is_dir() {
      (Some(fedit_core::SaveArchive::open(&self.file)?), None)
    } else {
      let s = fe13::SaveSlot::from_bytes(&fs::read(&self.file)?)?;
      (None, Some(s))
    };
    let c = 0;
    let slot = match a.as_mut() {
      Some(ar) => ar.chapters[c].as_mut().ok_or_else(|| {
        fedit_core::Error::SlotNotFound {
          slot: format!("Chapter{c}"),
        }
      })?,
      None => f_slot.as_mut().expect("file slot"),
    };
    let u = if let Some(idx) = self.index {
      slot.roster.get_mut(idx).ok_or_else(|| {
        fedit_core::Error::SlotNotFound {
          slot: format!("Unit index {idx}"),
        }
      })?
    } else if let Some(ref target) = self.name {
      slot.find_unit_by_name_mut(target).ok_or_else(|| {
        fedit_core::Error::SlotNotFound {
          slot: format!("Character '{target}'"),
        }
      })?
    } else {
      return Err(
        fedit_core::Error::SlotNotFound {
          slot: "Must specify --index or --name".to_string(),
        }
        .into(),
      );
    };
    let mut m = false;
    if let Some(v) = self.hp {
      u.stats.set_hp(v)?;
      m = true;
    }
    if let Some(v) = self.strength {
      u.stats.set_combat_stat("strength", v)?;
      m = true;
    }
    if let Some(v) = self.magic {
      u.stats.set_combat_stat("magic", v)?;
      m = true;
    }
    if let Some(v) = self.skill {
      u.stats.set_combat_stat("skill", v)?;
      m = true;
    }
    if let Some(v) = self.speed {
      u.stats.set_combat_stat("speed", v)?;
      m = true;
    }
    if let Some(v) = self.luck {
      u.stats.set_combat_stat("luck", v)?;
      m = true;
    }
    if let Some(v) = self.defense {
      u.stats.set_combat_stat("defense", v)?;
      m = true;
    }
    if let Some(v) = self.resistance {
      u.stats.set_combat_stat("resistance", v)?;
      m = true;
    }
    let uid = u.unit_id;
    if m {
      let dest = self.output.as_ref().unwrap_or(&self.file);
      crate::atomic::write_atomic(dest, &slot.to_bytes()?)?;
      println!("Updated {uid} stats");
      return Ok(());
    }
    println!("Unit: {}  Class: {}", u.unit_id, u.class_id.0);
    println!(
      "HP:  {:2}  Str: {:2}  Mag: {:2}  Skl: {:2}",
      u.stats.hp, u.stats.strength, u.stats.magic, u.stats.skill
    );
    println!(
      "Spd: {:2}  Lck: {:2}  Def: {:2}  Res: {:2}",
      u.stats.speed, u.stats.luck, u.stats.defense, u.stats.resistance
    );
    Ok(())
  }
}
