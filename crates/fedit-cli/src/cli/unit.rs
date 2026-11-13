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
  /// Target chapter slot index (0..=2).
  #[arg(long)]
  pub chapter: Option<usize>,
  /// Target unit roster index.
  #[arg(long)]
  pub index: usize,
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
}

/// Clamps and assigns an optional stat value, returning whether it was set.
#[expect(dead_code, reason = "used by stat mutation")]
fn clamp_stat(opt: Option<u8>, min: u8, max: u8, target: &mut u8) -> bool {
  if let Some(v) = opt {
    *target = v.clamp(min, max);
    true
  } else {
    false
  }
}

impl UnitArgs {
  /// Executes the unit command.
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
    let u = slot.roster.get(self.index).ok_or_else(|| {
      fedit_core::Error::SlotNotFound {
        slot: format!("Unit {}", self.index),
      }
    })?;
    println!(
      "Idx: {}  Unit: {}  Class: {}",
      self.index, u.unit_id.0, u.class_id.0
    );
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
