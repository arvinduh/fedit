//! Convoy storage inspection command.

use std::fs;
use std::io;
use std::path;

use awakening;
use clap;
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
    if self.file.is_dir() {
      return Err(
        io::Error::new(
          io::ErrorKind::InvalidInput,
          "directories not supported; pass a single save slot file",
        )
        .into(),
      );
    }
    if let (Some(target), Some(uses)) = (self.slot, self.uses) {
      if self.backup {
        let bak = self.file.with_extension("bak");
        fs::copy(&self.file, &bak)?;
        println!("Backup created at {}", bak.display());
      }
      let mut s = awakening::SaveSlot::from_bytes(&fs::read(&self.file)?)?;
      let it = s
        .convoy
        .iter_mut()
        .find(|it| it.slot == target)
        .ok_or_else(|| {
          io::Error::new(
            io::ErrorKind::NotFound,
            format!("convoy slot {target} not found"),
          )
        })?;
      it.uses = uses;
      crate::atomic::write_atomic(&self.file, &s.to_bytes()?)?;
      println!("Updated convoy slot {target} uses to {uses}");
      return Ok(());
    }
    let slot = awakening::SaveSlot::from_bytes(&fs::read(&self.file)?)?;
    if self.json {
      let rows: Vec<serde_json::Value> = slot
        .convoy
        .iter()
        .map(|it| {
          serde_json::json!({
            "slot": it.slot,
            "item_id": it.item.raw_id(),
            "name": it.item.name(),
            "uses": it.uses,
          })
        })
        .collect();
      println!("{}", serde_json::to_string_pretty(&rows)?);
      return Ok(());
    }
    println!("Slot  Item ID  Uses  Name");
    println!("-----------------------------------");
    for it in &slot.convoy {
      println!(
        "{:4}    {:5}   {:4}  {}",
        it.slot,
        it.item.raw_id(),
        it.uses,
        it.item.name()
      );
    }
    Ok(())
  }
}
