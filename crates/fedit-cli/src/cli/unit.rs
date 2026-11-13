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
    let (mut a, mut f_slot) = if self.file.is_dir() {
      (Some(fedit_core::SaveArchive::open(&self.file)?), None)
    } else {
      let s = fe13::SaveSlot::from_bytes(&fs::read(&self.file)?)?;
      (None, Some(s))
    };
    let c = self.chapter.unwrap_or(0);
    let slot = match a.as_mut() {
      Some(ar) => ar.chapters[c].as_mut().ok_or_else(|| {
        fedit_core::Error::SlotNotFound {
          slot: format!("Chapter{c}"),
        }
      })?,
      None => f_slot.as_mut().expect("file slot"),
    };
    let u = slot.roster.get_mut(self.index).ok_or_else(|| {
      fedit_core::Error::SlotNotFound {
        slot: format!("Unit {}", self.index),
      }
    })?;
    let mut m = clamp_stat(self.hp, 10, 99, &mut u.stats.hp);
    m |= clamp_stat(self.strength, 0, 80, &mut u.stats.strength);
    m |= clamp_stat(self.magic, 0, 80, &mut u.stats.magic);
    m |= clamp_stat(self.skill, 0, 80, &mut u.stats.skill);
    m |= clamp_stat(self.speed, 0, 80, &mut u.stats.speed);
    m |= clamp_stat(self.luck, 0, 80, &mut u.stats.luck);
    m |= clamp_stat(self.defense, 0, 80, &mut u.stats.defense);
    m |= clamp_stat(self.resistance, 0, 80, &mut u.stats.resistance);
    if m {
      if let Some(ar) = a {
        ar.save()?;
      } else {
        let dest = self.output.as_ref().unwrap_or(&self.file);
        if self.output.is_none() {
          fs::copy(&self.file, self.file.with_extension("bak"))?;
        }
        fedit_core::atomic::write_atomic(
          dest,
          &f_slot.expect("f_slot").to_bytes()?,
        )?;
      }
      println!("Updated unit {} stats", self.index);
      return Ok(());
    }
    println!(
      "Idx: {}  Unit: {}  Class: {}",
      self.index, u.unit_id, u.class_id.0
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
