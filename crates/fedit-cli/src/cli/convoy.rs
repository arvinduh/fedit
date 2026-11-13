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
  /// Path to save file or archive directory.
  pub file: path::PathBuf,
  /// Target chapter index when inspecting an archive directory (0..=2).
  #[arg(long)]
  pub chapter: Option<usize>,
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
