//! Convoy storage inspection command.

use std::fs;
use std::path;

use clap;
use fe13;
use fedit_core;
use serde_json;

use crate::cli::error;

/// Arguments for the `convoy` subcommand.
#[derive(Debug, clap::Args)]
pub struct ConvoyArgs {
  /// Path to save slot file.
  pub file: path::PathBuf,
  /// Target convoy slot index to mutate.
  #[arg(long)]
  pub slot: Option<usize>,
  /// New uses count for the target convoy item.
  #[arg(long)]
  pub uses: Option<u16>,
  /// Create a backup before mutating.
  #[arg(long)]
  pub backup: bool,
  /// Output convoy storage as JSON.
  #[arg(long)]
  pub json: bool,
}

impl ConvoyArgs {
  /// Executes the convoy command.
  ///
  /// # Errors
  ///
  /// Returns an error if the save file cannot be read or parsed.
  pub fn run(self) -> Result<(), error::Error> {
    if let (Some(target), Some(uses)) = (self.slot, self.uses) {
      if self.file.is_dir() {
        let mut a = fedit_core::SaveArchive::open(&self.file)?;
        let c = 0;
        let s = a.chapters[c].as_mut().ok_or_else(|| {
          fedit_core::Error::SlotNotFound {
            slot: format!("Chapter{c}"),
          }
        })?;
        let it = s
          .convoy
          .iter_mut()
          .find(|it| it.slot == target)
          .ok_or_else(|| fedit_core::Error::SlotNotFound {
            slot: format!("Convoy slot {target}"),
          })?;
        it.uses = uses;
        a.save_archive(self.backup)?;
      } else {
        let mut s = fe13::SaveSlot::from_bytes(&fs::read(&self.file)?)?;
        let it = s
          .convoy
          .iter_mut()
          .find(|it| it.slot == target)
          .ok_or_else(|| fedit_core::Error::SlotNotFound {
            slot: format!("Convoy slot {target}"),
          })?;
        it.uses = uses;
        if self.backup {
          let bak = self.file.with_extension("bak");
          fs::copy(&self.file, &bak)?;
        }
        crate::atomic::write_atomic(&self.file, &s.to_bytes()?)?;
      }
      println!("Updated convoy slot {target} uses to {uses}");
      return Ok(());
    }
    let slot = if self.file.is_dir() {
      let a = fedit_core::SaveArchive::open(&self.file)?;
      let c = 0;
      a.chapters[c]
        .clone()
        .ok_or_else(|| fedit_core::Error::SlotNotFound {
          slot: format!("Chapter{c}"),
        })?
    } else {
      fe13::SaveSlot::from_bytes(&fs::read(&self.file)?)?
    };
    if self.json {
      let rows: Vec<serde_json::Value> = slot
        .convoy
        .iter()
        .map(|it| {
          serde_json::json!({
            "slot": it.slot,
            "item_id": it.item_id.0,
            "uses": it.uses,
          })
        })
        .collect();
      println!("{}", serde_json::to_string_pretty(&rows)?);
      return Ok(());
    }
    println!("Slot  Item ID  Uses");
    println!("-------------------");
    for it in &slot.convoy {
      println!("{:4}    {:5}   {:4}", it.slot, it.item_id.0, it.uses);
    }
    Ok(())
  }
}
